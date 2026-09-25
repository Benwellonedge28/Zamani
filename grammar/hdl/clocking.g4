/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/clocking.g4
 *
 * Status:
 *     PRODUCTION CLOCKING-CONTEXT GRAMMAR CONTRACT
 *
 * Responsibility:
 *     Defines source-level HDL clocking context, clock-domain relationships,
 *     clock-edge events, synchronization intent, crossing intent, sampling
 *     intent, and clocking constraints.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust.
 *     No unsafe Rust is required.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Normative architecture:
 *
 *     grammar/DESIGN.md
 *          |
 *          v
 *     grammar/spec/hdl.md
 *          |
 *          v
 *     grammar/hdl/clocking.g4
 *          |
 *          +--> grammar/hdl/clocks.g4
 *          |
 *          +--> grammar/hdl/timing.g4
 *          |
 *          v
 *     canonical HDL parser composition
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic clock model
 *          |
 *          +--> clock-domain analysis
 *          +--> CDC analysis
 *          +--> timing analysis
 *          +--> resource analysis
 *          +--> capability analysis
 *          |
 *          v
 *     canonical hardware semantic representation
 *          |
 *          +--> scheduling
 *          +--> optimization
 *          +--> synthesis
 *          +--> placement
 *          +--> routing
 *          +--> target lowering
 *
 * This file owns SOURCE-LEVEL CLOCKING CONTEXT.
 *
 * It does NOT own physical clock implementation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - clocking contexts;
 *   - clock-domain association;
 *   - clock-domain relationships;
 *   - source/destination clock relationships;
 *   - edge-selection intent;
 *   - sampling intent;
 *   - capture intent;
 *   - synchronization intent;
 *   - clock-domain crossing intent;
 *   - source-level synchronization requirements;
 *   - source-level clocking constraints;
 *   - clocking assertions;
 *   - clocking metadata;
 *   - composition of those constructs.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - identifiers;
 *   - literals;
 *   - general expressions;
 *   - general types;
 *   - ordinary clock declarations;
 *   - reset declarations;
 *   - generic timing declarations;
 *   - physical clocks;
 *   - oscillators;
 *   - PLLs;
 *   - DLLs;
 *   - clock trees;
 *   - clock buffers;
 *   - physical pins;
 *   - clock routing;
 *   - CDC implementation;
 *   - synchronizer implementation;
 *   - synthesis;
 *   - placement;
 *   - routing;
 *   - target selection;
 *   - hardware discovery;
 *   - runtime clock control;
 *   - vendor primitives.
 *
 * Ordinary clock declarations remain owned by:
 *
 *     grammar/hdl/clocks.g4
 *
 * General timing declarations remain owned by:
 *
 *     grammar/hdl/timing.g4
 *
 * ============================================================================
 * CRITICAL ARCHITECTURAL DISTINCTION
 * ============================================================================
 *
 * A CLOCK declaration describes a clock.
 *
 * A CLOCKING declaration describes how source-level hardware behavior relates
 * to one or more clocks.
 *
 * Therefore:
 *
 *     clocks.g4
 *         -> clock identity/properties
 *
 *     clocking.g4
 *         -> clocking relationships/context
 *
 *     timing.g4
 *         -> general timing intent
 *
 * These files MUST NOT redefine each other's primary declarations.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Clocking syntax describes semantic intent.
 *
 * It MUST NOT encode:
 *
 *     - a fixed oscillator;
 *     - a fixed PLL;
 *     - a fixed DLL;
 *     - a fixed clock-tree architecture;
 *     - a fixed FPGA;
 *     - a fixed ASIC process;
 *     - a fixed board;
 *     - a physical clock pin;
 *     - a fixed number of clock domains;
 *     - a fixed number of clocks;
 *     - a fixed number of synchronizer stages;
 *     - a fixed frequency;
 *     - a fixed timing grid.
 *
 * The same source-level clocking intent may therefore be considered for:
 *
 *     simulation
 *     emulation
 *     FPGA
 *     ASIC
 *     CPU
 *     GPU
 *     accelerator
 *     heterogeneous hardware
 *     future hardware
 *
 * Target realization is downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level limits on:
 *
 *     clocks;
 *     clock domains;
 *     clock relationships;
 *     crossing declarations;
 *     synchronization declarations;
 *     sampling declarations;
 *     capture declarations;
 *     generated relationships;
 *     edge events;
 *     constraints;
 *     assertions;
 *     module hierarchy;
 *     pipeline stages;
 *     hardware instances.
 *
 * The following concepts MUST NOT occur as language limits:
 *
 *     MAX_CLOCKS
 *     MAX_CLOCK_DOMAINS
 *     MAX_CLOCK_RELATIONSHIPS
 *     MAX_CDC
 *     MAX_SYNCHRONIZER_STAGES
 *     MAX_CLOCK_EDGES
 *     MAX_FREQUENCY
 *     MAX_PHASE
 *     MAX_JITTER
 *     MAX_SKEW
 *     MAX_TIMING_DOMAINS
 *
 * Practical implementation limits remain implementation/resource-policy
 * concerns and MUST NOT be promoted to grammar semantics.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar MUST consume the repository's canonical lexer.
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The parser must therefore use:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * No lexer rules belong here.
 *
 * IMPORTANT:
 *
 * The repository currently contains inconsistent historical HDL token names
 * such as K_CLOCK/K_MODULE in some HDL sources while the canonical keyword
 * vocabulary uses names such as MODULE.
 *
 * This file deliberately does NOT reproduce those historical K_* aliases.
 *
 * The lexical compatibility migration must establish one canonical token
 * vocabulary before this grammar is promoted to generated-parser production.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Clocking values are semantic expressions.
 *
 * The grammar does not duplicate arithmetic, comparison, logical, or
 * type-level expression grammar.
 *
 * Shared expressions may represent:
 *
 *     clock references;
 *     symbolic periods;
 *     frequency requirements;
 *     phase;
 *     duty cycle;
 *     skew;
 *     jitter;
 *     uncertainty;
 *     synchronization parameters;
 *     latency;
 *     sampling windows;
 *     resource requirements.
 *
 * Semantic analysis determines dimensional correctness.
 *
 * ============================================================================
 * IDENTIFIER CONTRACT
 * ============================================================================
 *
 * Clock names and domain names remain ordinary Zamani identifiers.
 *
 * They are logical names.
 *
 * They do not identify:
 *
 *     physical pins;
 *     physical oscillators;
 *     vendor resources;
 *     clock-tree nodes;
 *     FPGA primitives;
 *     ASIC cells.
 *
 * ============================================================================
 * DOMAIN-NEUTRAL HARDWARE MODEL
 * ============================================================================
 *
 * A clocking declaration may refer to:
 *
 *     a clock;
 *     a clock domain;
 *     a source;
 *     a destination;
 *     an edge;
 *     a synchronization policy;
 *     a sampling policy.
 *
 * It must not directly refer to physical realization.
 *
 * ============================================================================
 */

parser grammar HdlClocking;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A clocking context groups source-level relationships without redefining
 * ordinary clock declarations.
 *
 * Canonical conceptual form:
 *
 *     clocking name {
 *         ...
 *     }
 *
 * The exact lexical spelling of the contextual keyword is part of the
 * repository's canonical lexer migration described above.
 *
 * This grammar uses the canonical CLOCKING token once that token is promoted
 * by the lexical authority.
 */
hdlClockingDeclaration
    : CLOCKING
      identifier
      hdlClockingTarget?
      hdlClockingBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. OPTIONAL CLOCKING TARGET
 * ============================================================================
 *
 * A clocking context may apply to a module, interface, process, or another
 * semantic hardware object.
 *
 * Example:
 *
 *     clocking interface_timing for interface_name {
 *         ...
 *     }
 *
 * The target remains a logical source-level name.
 */
hdlClockingTarget
    : FOR
      hdlQualifiedName
    ;


/*
 * ============================================================================
 * 3. CLOCKING BODY
 * ============================================================================
 *
 * Clocking bodies are open-ended.
 *
 * New clocking constructs can therefore be added through dedicated rules
 * without changing the meaning of existing clock declarations.
 */
hdlClockingBody
    : LBRACE
      hdlClockingItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. CLOCKING ITEM
 * ============================================================================
 */

hdlClockingItem
    : hdlClockingAttribute
    | hdlClockingClockBinding
    | hdlClockingDomainBinding
    | hdlClockingRelation
    | hdlClockingEdgeSpecification
    | hdlClockingSampleSpecification
    | hdlClockingCaptureSpecification
    | hdlClockingSynchronization
    | hdlClockDomainCrossing
    | hdlClockingConstraint
    | hdlClockingAssertion
    | hdlClockingProperty
    ;


/*
 * ============================================================================
 * 5. CLOCK BINDING
 * ============================================================================
 *
 * Associates a logical clocking context with a clock.
 *
 * This does not select a physical clock source.
 */
hdlClockingClockBinding
    : CLOCK
      identifier
      ASSIGN
      hdlClockReference
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 6. CLOCK DOMAIN BINDING
 * ============================================================================
 *
 * Associates a logical clock domain with a clock reference.
 *
 * The semantic layer validates:
 *
 *     - existence;
 *     - uniqueness;
 *     - consistency;
 *     - relationship legality.
 */
hdlClockingDomainBinding
    : CLOCK_DOMAIN
      identifier
      ASSIGN
      hdlClockReference
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 7. CLOCK RELATION
 * ============================================================================
 *
 * Describes a semantic relationship between two clocks.
 *
 * Examples:
 *
 *     relation source -> destination;
 *
 *     relation source -> destination {
 *         phase = phase_offset;
 *         ratio = frequency_ratio;
 *     }
 *
 * Physical implementation is downstream.
 */
hdlClockingRelation
    : RELATION
      hdlClockReference
      THIN_ARROW
      hdlClockReference
      hdlClockingRelationBody?
      SEMICOLON?
    ;


hdlClockingRelationBody
    : LBRACE
      hdlClockingRelationProperty*
      RBRACE
    ;


hdlClockingRelationProperty
    : identifier
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 8. EDGE SPECIFICATION
 * ============================================================================
 *
 * An edge specification describes the semantic event on which a clocked
 * operation is evaluated.
 *
 * Examples:
 *
 *     edge rising;
 *
 *     edge falling;
 *
 *     edge both;
 *
 * Edge names remain semantic values rather than a permanently closed gate
 * enumeration.
 */
hdlClockingEdgeSpecification
    : EDGE
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 9. SAMPLE SPECIFICATION
 * ============================================================================
 *
 * Sampling describes when a value is observed relative to a clock.
 *
 * Example:
 *
 *     sample input_signal on clock_name rising;
 *
 * Additional semantic properties may be represented in a block.
 */
hdlClockingSampleSpecification
    : SAMPLE
      hdlExpressionList
      ON
      hdlClockReference
      hdlClockingEdgeClause?
      hdlClockingSampleBody?
      SEMICOLON?
    ;


hdlClockingSampleBody
    : LBRACE
      hdlClockingProperty*
      RBRACE
    ;


/*
 * ============================================================================
 * 10. CAPTURE SPECIFICATION
 * ============================================================================
 *
 * Capture describes source-level state/data capture intent.
 *
 * It does not imply a particular flip-flop, latch, synchronizer, memory,
 * clock-gating cell, or vendor primitive.
 */
hdlClockingCaptureSpecification
    : CAPTURE
      hdlExpressionList
      ON
      hdlClockReference
      hdlClockingEdgeClause?
      hdlClockingSampleBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 11. EDGE CLAUSE
 * ============================================================================
 */

hdlClockingEdgeClause
    : EDGE
      expression
    ;


/*
 * ============================================================================
 * 12. SYNCHRONIZATION
 * ============================================================================
 *
 * Synchronization describes semantic intent.
 *
 * Example:
 *
 *     synchronize signal_name
 *         from source_clock
 *         to destination_clock;
 *
 * The number and implementation of synchronizer stages is NOT selected by
 * this grammar.
 */
hdlClockingSynchronization
    : SYNCHRONIZE
      hdlExpressionList
      hdlClockingFromClause?
      hdlClockingToClause?
      hdlClockingSynchronizationBody?
      SEMICOLON?
    ;


hdlClockingFromClause
    : FROM
      hdlClockReference
    ;


hdlClockingToClause
    : TO
      hdlClockReference
    ;


hdlClockingSynchronizationBody
    : LBRACE
      hdlClockingProperty*
      RBRACE
    ;


/*
 * ============================================================================
 * 13. CLOCK-DOMAIN CROSSING
 * ============================================================================
 *
 * CDC is represented as source-level intent.
 *
 * It does not implement a synchronizer.
 *
 * Example:
 *
 *     crossing signal_name {
 *         from = source_clock;
 *         to = destination_clock;
 *         mode = handshake;
 *     }
 */
hdlClockDomainCrossing
    : CROSSING
      hdlExpressionList
      hdlClockingCdcBody?
      SEMICOLON?
    ;


hdlClockingCdcBody
    : LBRACE
      hdlClockingCdcProperty*
      RBRACE
    ;


hdlClockingCdcProperty
    : identifier
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 14. CLOCKING CONSTRAINT
 * ============================================================================
 *
 * Constraints describe requirements.
 *
 * They do not select a physical implementation.
 */
hdlClockingConstraint
    : CONSTRAINT
      hdlClockingConstraintBody
      SEMICOLON?
    ;


hdlClockingConstraintBody
    : LBRACE
      hdlClockingProperty*
      RBRACE
    ;


/*
 * ============================================================================
 * 15. CLOCKING ASSERTION
 * ============================================================================
 *
 * Assertions remain source-level verification intent.
 *
 * Verification engines consume their semantic representation downstream.
 */
hdlClockingAssertion
    : ASSERT
      hdlExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 16. CLOCKING PROPERTY
 * ============================================================================
 *
 * Property names remain extensible.
 *
 * Examples:
 *
 *     period = base_period;
 *     frequency = target_frequency;
 *     phase = phase_offset;
 *     skew = allowed_skew;
 *     jitter = allowed_jitter;
 *     uncertainty = allowed_uncertainty;
 *     latency = latency_budget;
 *     mode = handshake;
 *
 * The grammar intentionally does not enumerate every future clocking
 * property.
 */
hdlClockingProperty
    : identifier
      ASSIGN
      expression
      SEMICOLON?
    | identifier
      hdlClockingPropertyBlock
    ;


hdlClockingPropertyBlock
    : LBRACE
      hdlClockingProperty*
      RBRACE
    ;


/*
 * ============================================================================
 * 17. ATTRIBUTE
 * ============================================================================
 *
 * Clocking attributes are source metadata.
 *
 * They do not themselves select hardware.
 */
hdlClockingAttribute
    : AT
      identifier
      (
          LPAREN
          hdlArgumentList?
          RPAREN
      )?
    ;


/*
 * ============================================================================
 * 18. CLOCK REFERENCE
 * ============================================================================
 *
 * A clock reference is intentionally an expression.
 *
 * This permits:
 *
 *     clk
 *     clocks.primary
 *     generated.clock
 *     domain.clock
 *
 * without introducing a second identifier namespace.
 */
hdlClockReference
    : expression
    ;


/*
 * ============================================================================
 * 19. CLOCK LIST
 * ============================================================================
 *
 * No finite number of clocks is encoded.
 */
hdlClockReferenceList
    : hdlClockReference
      (
          COMMA
          hdlClockReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 20. CLOCK DOMAIN LIST
 * ============================================================================
 */

hdlClockDomainReferenceList
    : hdlQualifiedName
      (
          COMMA
          hdlQualifiedName
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 21. EDGE LIST
 * ============================================================================
 *
 * The semantic model determines whether a given edge expression is valid.
 *
 * This grammar intentionally does not create a finite enum such as:
 *
 *     RISING
 *     FALLING
 *     BOTH
 *
 * as universal parser tokens.
 */
hdlClockingEdgeList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 22. CLOCKING PROPERTY VALUE
 * ============================================================================
 *
 * Explicit semantic boundary for downstream AST builders and semantic
 * visitors.
 */
hdlClockingValue
    : expression
    ;


/*
 * ============================================================================
 * 23. SOURCE-LEVEL SYNCHRONIZATION POLICY
 * ============================================================================
 *
 * Synchronization policy is data.
 *
 * Examples may include:
 *
 *     handshake
 *     pulse
 *     toggle
 *     fifo
 *     protocol
 *
 * These names remain expressions/identifiers.
 */
hdlClockingSynchronizationPolicy
    : expression
    ;


/*
 * ============================================================================
 * 24. CLOCKING SAMPLE MODE
 * ============================================================================
 */

hdlClockingSampleMode
    : expression
    ;


/*
 * ============================================================================
 * 25. CLOCKING CAPTURE MODE
 * ============================================================================
 */

hdlClockingCaptureMode
    : expression
    ;


/*
 * ============================================================================
 * 26. CLOCKING RELATIONSHIP VALUE
 * ============================================================================
 */

hdlClockingRelationValue
    : expression
    ;


/*
 * ============================================================================
 * 27. CLOCKING CONSTRAINT VALUE
 * ============================================================================
 */

hdlClockingConstraintValue
    : expression
    ;


/*
 * ============================================================================
 * 28. INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar is a subordinate parser grammar.
 *
 * It must be composed through the canonical HDL parser dispatcher.
 *
 * Intended dependency direction:
 *
 *     HDL dispatcher
 *          |
 *          +--> HdlClocks
 *          |
 *          +--> HdlClocking
 *          |
 *          +--> HdlTiming
 *          |
 *          +--> remaining HDL domains
 *
 * This file must NOT become a second HDL root.
 *
 * ============================================================================
 * 29. CLOCK DECLARATION INTEGRATION
 * ============================================================================
 *
 * Ordinary clock declarations remain owned by:
 *
 *     grammar/hdl/clocks.g4
 *
 * This file consumes the resulting logical clock references through:
 *
 *     hdlClockReference
 *
 * It MUST NOT redefine:
 *
 *     hdlClockDeclaration
 *     hdlClockProperty
 *     hdlClockSpecification
 *
 * Doing so would create competing ownership.
 *
 * ============================================================================
 * 30. TIMING INTEGRATION
 * ============================================================================
 *
 * General timing constructs remain owned by:
 *
 *     grammar/hdl/timing.g4
 *
 * Clocking properties may reference timing expressions, but this grammar does
 * not duplicate timing declarations.
 *
 * Semantic integration:
 *
 *     clocking intent
 *          +
 *     timing intent
 *          |
 *          v
 *     canonical semantic timing/clock model
 *
 * ============================================================================
 * 31. RESET INTEGRATION
 * ============================================================================
 *
 * Reset syntax remains owned by the reset portion of the HDL architecture.
 *
 * Clocking does not redefine:
 *
 *     synchronous reset;
 *     asynchronous reset;
 *     active-high reset;
 *     active-low reset.
 *
 * A semantic clocking model may reference reset relationships downstream.
 *
 * ============================================================================
 * 32. REGISTER / SEQUENTIAL INTEGRATION
 * ============================================================================
 *
 * Registers and sequential behavior may consume clocking information.
 *
 * Dependency direction:
 *
 *     clocking syntax
 *          ->
 *     AST
 *          ->
 *     semantic clock context
 *          ->
 *     sequential analysis
 *
 * This file must not create a register IR or sequential IR.
 *
 * ============================================================================
 * 33. CDC INTEGRATION
 * ============================================================================
 *
 * CDC analysis belongs downstream.
 *
 * The grammar records intent such as:
 *
 *     source clock;
 *     destination clock;
 *     crossing object;
 *     synchronization policy.
 *
 * Semantic analysis determines:
 *
 *     - whether source and destination exist;
 *     - whether the crossing is legal;
 *     - whether synchronization is required;
 *     - whether the selected policy is sufficient;
 *     - whether the target can realize the requested behavior.
 *
 * The parser does not make those decisions.
 *
 * ============================================================================
 * 34. HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware capability analysis may determine whether a target can realize:
 *
 *     requested clock relationship;
 *     requested frequency;
 *     requested phase;
 *     requested synchronization policy;
 *     requested timing;
 *     requested crossing behavior.
 *
 * No hardware discovery occurs during parsing.
 *
 * ============================================================================
 * 35. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Clocking requirements may ultimately consume the shared resource/capability
 * architecture.
 *
 * Examples:
 *
 *     requires capability("clocking");
 *     requires capability("clock.domain.crossing");
 *     requires capability("clock.generation");
 *
 * The parser only recognizes source syntax.
 *
 * Capability satisfaction is downstream.
 *
 * ============================================================================
 * 36. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum systems may require timing and synchronization metadata for:
 *
 *     pulse control;
 *     measurement;
 *     dynamic circuits;
 *     classical feed-forward;
 *     hardware control;
 *     synchronization.
 *
 * This grammar does NOT create quantum timing IR.
 *
 * The downstream relationship remains:
 *
 *     Zamani source
 *          ->
 *     domain-neutral AST
 *          ->
 *     semantic analysis
 *          ->
 *     quantum::ir
 *          ->
 *     scheduling / QEC / ZQN / HAL
 *
 * where appropriate.
 *
 * ============================================================================
 * 37. QEC / ZQN
 * ============================================================================
 *
 * No direct grammar dependency exists on QEC or ZQN.
 *
 * QEC/ZQN may consume semantic timing metadata after parsing.
 *
 * The grammar must never implement:
 *
 *     error correction;
 *     noise modeling;
 *     physical qubit routing;
 *     calibration.
 *
 * ============================================================================
 * 38. AST CONTRACT
 * ============================================================================
 *
 * Each accepted construct should map to the domain-neutral frontend AST.
 *
 * Conceptual AST information includes:
 *
 *     ClockingDeclaration
 *         name
 *         target
 *         items
 *         source_span
 *
 *     ClockBinding
 *         name
 *         reference
 *         source_span
 *
 *     ClockDomainBinding
 *         domain
 *         clock
 *         source_span
 *
 *     ClockRelation
 *         source
 *         destination
 *         properties
 *         source_span
 *
 *     ClockEdgeSpecification
 *         edge
 *         source_span
 *
 *     ClockSampling
 *         values
 *         clock
 *         edge
 *         properties
 *         source_span
 *
 *     ClockCapture
 *         values
 *         clock
 *         edge
 *         properties
 *         source_span
 *
 *     ClockSynchronization
 *         values
 *         source
 *         destination
 *         properties
 *         source_span
 *
 *     ClockDomainCrossing
 *         values
 *         properties
 *         source_span
 *
 * The grammar does not construct these nodes.
 *
 * ============================================================================
 * 39. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must validate at minimum:
 *
 *     - clock references;
 *     - domain references;
 *     - duplicate bindings;
 *     - illegal self-relations;
 *     - cyclic relationships where prohibited;
 *     - incompatible domains;
 *     - edge validity;
 *     - timing dimensionality;
 *     - sampling legality;
 *     - capture legality;
 *     - synchronization policy;
 *     - CDC requirements;
 *     - source/destination compatibility;
 *     - conflicting properties;
 *     - target capability requirements.
 *
 * None of these checks belong in this grammar.
 *
 * ============================================================================
 * 40. IR CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT introduce:
 *
 *     ClockingIR
 *     ClockDomainIR
 *     CDCIR
 *     SynchronizerIR
 *     PhysicalClockIR
 *
 * as competing universal IRs.
 *
 * Semantic lowering must use the repository's canonical hardware semantic
 * representation/IR.
 *
 * Quantum-related constructs eventually continue through the existing
 * `quantum::ir` boundary where their meaning is quantum-specific.
 *
 * ============================================================================
 * 41. DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source;
 *     grammar version;
 *     lexer version;
 *     parser configuration explicitly supplied by the compilation request.
 *
 * It MUST NOT depend on:
 *
 *     hardware;
 *     network;
 *     filesystem;
 *     wall-clock time;
 *     randomness;
 *     environment variables;
 *     runtime state;
 *     target availability.
 *
 * ============================================================================
 * 42. SECURITY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no filesystem operations;
 *     no network operations;
 *     no command execution;
 *     no hardware discovery;
 *     no credential access;
 *     no runtime execution;
 *     no embedded Rust actions.
 *
 * ============================================================================
 * 43. SAFE RUST
 * ============================================================================
 *
 * This grammar itself contains no Rust.
 *
 * Generated frontend integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must use safe Rust only.
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * ============================================================================
 * 44. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden as universal language limits:
 *
 *     MAX_CLOCKS
 *     MAX_CLOCK_DOMAINS
 *     MAX_SYNCHRONIZERS
 *     MAX_CDC
 *     MAX_FREQUENCY
 *     MAX_PERIOD
 *     MAX_PHASE
 *     MAX_JITTER
 *     MAX_SKEW
 *     MAX_CLOCK_TREE_DEPTH
 *     MAX_DEVICES
 *
 * No such limits are represented by this grammar.
 *
 * Numeric values remain program semantics.
 *
 * ============================================================================
 * 45. COMPATIBILITY
 * ============================================================================
 *
 * This file introduces a new ownership boundary:
 *
 *     clocking.g4
 *
 * It does not rename:
 *
 *     clocks.g4
 *     timing.g4
 *     hdl.g4
 *
 * Existing clock declarations remain owned by clocks.g4.
 *
 * Existing timing declarations remain owned by timing.g4.
 *
 * ============================================================================
 * 46. REQUIRED REPOSITORY INTEGRATION
 * ============================================================================
 *
 * This file is intentionally complete as an independent grammar contract,
 * but the repository cannot generate it successfully until the existing
 * lexical inconsistency is resolved.
 *
 * Required integration sequence:
 *
 *     1. grammar/lexer/keywords.g4
 *        add the canonical CLOCKING token.
 *
 *     2. grammar/lexer/keywords.g4
 *        establish canonical CLOCK and CLOCK_DOMAIN tokens if ordinary
 *        clock declarations are to remain reserved lexical constructs.
 *
 *     3. grammar/antlr/ZamaniLexer.g4
 *        regenerate through the existing ZamaniTokens composition.
 *
 *     4. grammar/hdl/hdl.g4
 *        import/compose HdlClocking.
 *
 *     5. grammar/hdl/hdl.g4
 *        dispatch hdlClockingDeclaration from the HDL member composition.
 *
 *     6. grammar/antlr/ZamaniParser.g4
 *        retain HDL as the single domain dispatcher; do not import this leaf
 *        directly into the universal parser.
 *
 *     7. grammar/spec/hdl.md
 *        record clocking.g4 as the normative clocking syntax component.
 *
 *     8. grammar/grammar.md
 *        report clocking status independently:
 *
 *            SPECIFIED
 *            GRAMMAR_IMPLEMENTED
 *            AST_IMPLEMENTED
 *            SEMANTICALLY_IMPLEMENTED
 *            IR_IMPLEMENTED
 *            TESTED
 *            STABLE
 *
 * ============================================================================
 * 47. IMPORTANT INTEGRATION RULE
 * ============================================================================
 *
 * Do NOT solve the existing K_* versus canonical lexer-token discrepancy by
 * adding a second lexer.
 *
 * Do NOT create:
 *
 *     grammar/hdl/ClockLexer.g4
 *
 * Do NOT create:
 *
 *     grammar/hdl/ClockingLexer.g4
 *
 * Do NOT create:
 *
 *     grammar/antlr/ClockLexer.g4
 *
 * There must remain exactly one canonical Zamani lexer.
 *
 * ============================================================================
 * 48. TEST CONTRACT
 * ============================================================================
 *
 * Required tests belong under:
 *
 *     grammar/tests/hdl/clocking/
 *
 * At minimum:
 *
 *     minimal/
 *     positive/
 *     negative/
 *     boundary/
 *     scalability/
 *     determinism/
 *     compatibility/
 *     integration/
 *
 * Positive examples must cover:
 *
 *     clocking context;
 *     clock binding;
 *     domain binding;
 *     clock relationship;
 *     edge selection;
 *     sampling;
 *     capture;
 *     synchronization;
 *     CDC;
 *     constraints;
 *     assertions;
 *     symbolic values;
 *     qualified clock names;
 *     multiple domains;
 *     multiple relationships.
 *
 * Negative examples must cover:
 *
 *     malformed clock references;
 *     malformed domain references;
 *     malformed relations;
 *     malformed sampling;
 *     malformed synchronization;
 *     malformed CDC;
 *     malformed property blocks.
 *
 * Semantic-negative tests must separately cover:
 *
 *     unknown clock;
 *     unknown domain;
 *     cyclic clock relationship;
 *     invalid dimensions;
 *     invalid synchronization policy;
 *     incompatible source/destination domains.
 *
 * ============================================================================
 * 49. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST demonstrate that the grammar does not encode finite limits on:
 *
 *     clock declarations;
 *     domains;
 *     relationships;
 *     crossings;
 *     synchronization declarations;
 *     properties.
 *
 * Large test inputs are implementation stress tests.
 *
 * They MUST NOT be converted into language-level maxima.
 *
 * ============================================================================
 * 50. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It owns clocking context rather than ordinary clock declarations.
 * [x] clocks.g4 remains the clock declaration owner.
 * [x] timing.g4 remains the general timing owner.
 * [x] No physical clock implementation is encoded.
 * [x] No vendor is encoded.
 * [x] No FPGA/ASIC assumptions are encoded.
 * [x] No fixed frequency is encoded.
 * [x] No fixed clock count is encoded.
 * [x] No fixed synchronization-stage count is encoded.
 * [x] No fixed topology is encoded.
 * [x] Clock references remain logical.
 * [x] Clock domains remain logical.
 * [x] CDC remains semantic intent.
 * [x] Synchronization remains semantic intent.
 * [x] Sampling remains semantic intent.
 * [x] Capture remains semantic intent.
 * [x] Timing remains downstream.
 * [x] AST ownership is explicit.
 * [x] Semantic ownership is explicit.
 * [x] IR ownership is explicit.
 * [x] Resource/capability ownership is explicit.
 * [x] Quantum integration is explicit.
 * [x] QEC/ZQN remain downstream.
 * [x] Safe Rust requirements are explicit.
 * [x] Rust 1.97 / 1.97.1 requirements are explicit.
 * [x] Hard-coding audit is explicit.
 * [x] Test contract is explicit.
 * [x] Repository integration is specified in advance.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "How does Zamani express portable clocking relationships and
 *      synchronization intent?"
 *
 * It does NOT answer:
 *
 *     "How is the clock physically generated?"
 *
 *     "Which PLL is used?"
 *
 *     "Which FPGA clock resource is used?"
 *
 *     "Which ASIC cell is used?"
 *
 *     "How many synchronizer stages does the target require?"
 *
 *     "Which physical clock pin is selected?"
 *
 * Those questions belong downstream.
 *
 * The architectural direction remains:
 *
 *     source
 *       ->
 *     canonical lexer
 *       ->
 *     canonical parser
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic clock model
 *       ->
 *     timing / CDC / resource / capability analysis
 *       ->
 *     hardware semantic representation
 *       ->
 *     optimization / scheduling / synthesis / routing
 *       ->
 *     target realization
 *
 * ============================================================================
 */