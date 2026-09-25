/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/timing.g4
 *
 * Status:
 *     CANONICAL HARDWARE-CONTRACT TIMING SYNTAX COMPONENT
 *
 * Purpose:
 *     Defines target-independent hardware timing intent.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     Action-free ANTLR parser grammar.
 *     No embedded Rust.
 *     No semantic predicates.
 *     No I/O.
 *     No hardware discovery.
 *     No runtime execution.
 *     No unsafe implementation.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * This file belongs to:
 *
 *     grammar/hardware/
 *
 * It is intentionally DISTINCT from:
 *
 *     grammar/hdl/timing.g4
 *
 * The ownership boundary is:
 *
 *     grammar/hardware/timing.g4
 *         ->
 *     machine-independent hardware timing CONTRACTS
 *
 *     grammar/hdl/timing.g4
 *         ->
 *     HDL timing SYNTAX / behavioral timing intent
 *
 *     grammar/hdl/clocks.g4
 *         ->
 *     HDL clock DECLARATIONS
 *
 *     grammar/hdl/clocking.g4
 *         ->
 *     HDL clocking / CDC CONTEXT
 *
 *     grammar/execution/scheduling.g4
 *         ->
 *     execution scheduling intent
 *
 *     compiler / semantic analysis
 *         ->
 *     timing feasibility and interpretation
 *
 *     scheduling
 *         ->
 *     concrete temporal realization
 *
 *     hardware HAL
 *         ->
 *     target timing capabilities
 *
 * This file MUST NOT become a second HDL timing grammar, a scheduler,
 * a timing-analysis engine, or a physical timing model.
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
 *     grammar/spec/hardware.md
 *          |
 *          v
 *     grammar/hardware/timing.g4
 *          |
 *          v
 *     grammar/hardware/hardware.g4
 *          |
 *          v
 *     grammar/antlr/ZamaniParser.g4
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical hardware semantic representation
 *          |
 *          +--> scheduling
 *          +--> optimization
 *          +--> synthesis
 *          +--> placement
 *          +--> routing
 *          +--> verification
 *          +--> HAL / target realization
 *
 * ============================================================================
 * SINGLE OWNER
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - hardware timing declarations;
 *   - hardware timing properties;
 *   - timing requirements;
 *   - timing constraints;
 *   - timing preferences;
 *   - timing hints;
 *   - timing budgets;
 *   - timing windows;
 *   - timing paths as LOGICAL timing paths;
 *   - timing relations;
 *   - timing exceptions;
 *   - timing assertions;
 *   - timing analysis intent;
 *   - timing metadata;
 *   - logical timing references;
 *   - source-level timing contracts.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - clock declarations;
 *   - clock generation;
 *   - clock domains as physical objects;
 *   - clock-tree construction;
 *   - PLL/DLL/oscillator selection;
 *   - clock pins;
 *   - clock routing;
 *   - CDC implementation;
 *   - synchronizer implementation;
 *   - HDL sequential semantics;
 *   - HDL combinational semantics;
 *   - physical timing closure;
 *   - placement;
 *   - routing;
 *   - scheduling algorithms;
 *   - resource allocation;
 *   - target selection;
 *   - hardware discovery;
 *   - calibration;
 *   - vendor primitives;
 *   - runtime implementation;
 *   - canonical IR construction.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Timing describes WHAT must be true.
 *
 * It does not prescribe HOW a target achieves it.
 *
 * The grammar therefore MUST NOT contain:
 *
 *     MAX_CLOCKS
 *     MAX_TIMING_PATHS
 *     MAX_TIMING_CONSTRAINTS
 *     MAX_FREQUENCY
 *     MAX_PERIOD
 *     MAX_LATENCY
 *     MAX_JITTER
 *     MAX_SKEW
 *     MAX_DOMAINS
 *     MAX_DEVICES
 *     MAX_MODULES
 *
 * It also MUST NOT encode:
 *
 *     fixed CPU frequencies;
 *     fixed GPU frequencies;
 *     fixed FPGA clock limits;
 *     fixed ASIC process limits;
 *     fixed QPU timing grids;
 *     fixed pulse clocks;
 *     fixed device timing resolution;
 *     fixed machine sizes.
 *
 * All quantities are source expressions.
 *
 * Actual representability and feasibility are downstream concerns.
 *
 * ============================================================================
 * IMPORTANT SCALABILITY RULE
 * ============================================================================
 *
 * A finite numeric value appearing in a program is PROGRAM SEMANTICS.
 *
 * For example:
 *
 *     latency <= 20ns;
 *
 * is legal.
 *
 * This does NOT mean:
 *
 *     Zamani has a universal 20ns latency limit.
 *
 * Likewise:
 *
 *     frequency >= required_frequency;
 *
 * is a requirement.
 *
 * It does NOT mean:
 *
 *     Zamani supports only a particular frequency.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This parser grammar consumes the canonical Zamani lexical vocabulary.
 *
 * It MUST NOT define lexer rules.
 *
 * Canonical parser integration is:
 *
 *     options {
 *         tokenVocab = ZamaniTokens;
 *     }
 *
 * The canonical production lexer remains responsible for turning source
 * text into the token vocabulary.
 *
 * In particular:
 *
 *     grammar/lexer/duration-literals.g4
 *
 * owns DURATION_LITERAL.
 *
 * Timing syntax consumes duration quantities through the shared expression
 * boundary instead of defining another duration grammar.
 *
 * ============================================================================
 * KEYWORD POLICY
 * ============================================================================
 *
 * Only the timing declaration introducer is structurally reserved here:
 *
 *     TIMING
 *
 * Existing generic resource/capability vocabulary may be used:
 *
 *     REQUIRES
 *     PREFER
 *     HINT
 *     ASSERT
 *
 * Timing property names remain contextual where possible.
 *
 * Examples:
 *
 *     latency
 *     period
 *     frequency
 *     setup
 *     hold
 *     jitter
 *     skew
 *     uncertainty
 *     phase
 *     duty_cycle
 *     throughput
 *
 * are semantic property names.
 *
 * This avoids requiring a new global lexer keyword every time a new timing
 * property is standardized.
 *
 * Existing reserved tokens such as:
 *
 *     LATENCY
 *     THROUGHPUT
 *     FREQUENCY
 *     POWER
 *
 * are accepted where they exist in the canonical lexical vocabulary.
 *
 * ============================================================================
 * SHARED PARSER CONTRACT
 * ============================================================================
 *
 * This grammar consumes shared hardware parser rules supplied by its
 * composition parent:
 *
 *     hardwareExpression
 *     hardwareQualifiedName
 *     hardwarePropertyStatement
 *
 * It MUST NOT redefine those rules.
 *
 * `hardwareExpression` is the temporary hardware expression bridge already
 * exposed by grammar/hardware/hardware.g4. When the repository completes the
 * migration to the universal expression grammar, the parent integration can
 * redirect that bridge without changing this file.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parser responsibility:
 *
 *     recognize timing intent.
 *
 * Semantic-analysis responsibility:
 *
 *     resolve timing references;
 *     validate dimensions;
 *     validate timing units;
 *     validate relational meaning;
 *     validate timing-property compatibility;
 *     distinguish requirements/preferences/hints;
 *     detect contradictory constraints;
 *     determine whether a target can satisfy requirements;
 *     produce diagnostics;
 *     construct the canonical timing semantic model.
 *
 * Scheduling responsibility:
 *
 *     construct concrete execution schedules.
 *
 * Hardware-analysis responsibility:
 *
 *     evaluate target timing capability.
 *
 * Synthesis/implementation responsibility:
 *
 *     realize timing constraints physically.
 *
 * ============================================================================
 * CANONICAL TIMING SEMANTICS
 * ============================================================================
 *
 * Timing quantities MAY be:
 *
 *     - literal;
 *     - symbolic;
 *     - generic;
 *     - parameterized;
 *     - computed;
 *     - target-dependent through explicit semantic references.
 *
 * Examples:
 *
 *     10ns
 *     1us
 *     base_period
 *     2 * base_period
 *     1 / target_frequency
 *     required_latency
 *
 * Dimensional correctness is NOT decided by this grammar.
 *
 * ============================================================================
 * CLOCK BOUNDARY
 * ============================================================================
 *
 * A timing declaration may refer to a logical clock:
 *
 *     clock::core
 *
 * or another logical timing reference.
 *
 * This file does NOT declare the clock.
 *
 * Clock declaration remains owned by:
 *
 *     grammar/hdl/clocks.g4
 *
 * Hardware-specific clock contracts remain subject to the hardware
 * composition layer.
 *
 * ============================================================================
 * HARDWARE VS HDL BOUNDARY
 * ============================================================================
 *
 * Hardware timing:
 *
 *     describes timing requirements/capabilities of a hardware contract.
 *
 * HDL timing:
 *
 *     describes timing in HDL behavioral/structural source.
 *
 * Therefore this file MUST NOT introduce rules such as:
 *
 *     always
 *     posedge
 *     negedge
 *     always_ff
 *     always_comb
 *     assign
 *
 * Those belong to HDL.
 *
 * ============================================================================
 * TIMING PROPERTY MODEL
 * ============================================================================
 *
 * A property has:
 *
 *     subject
 *     property name
 *     relation
 *     value
 *
 * Conceptually:
 *
 *     <property> <relation> <expression>
 *
 * Examples:
 *
 *     latency <= 20ns;
 *     frequency >= target_frequency;
 *     setup >= setup_requirement;
 *     jitter <= jitter_budget;
 *
 * Property names remain extensible.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Canonical hardware timing declaration.
 *
 * Examples:
 *
 *     timing datapath {
 *         latency <= 20ns;
 *         throughput >= required_throughput;
 *     }
 *
 *     timing interface_timing for interface::compute {
 *         setup >= setup_requirement;
 *         hold >= hold_requirement;
 *     }
 *
 * The declaration count is unbounded by language semantics.
 * ============================================================================
 */

hardwareTimingDeclaration
    : TIMING
      identifier?
      hardwareTimingTargetClause?
      hardwareTimingBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * TARGET CLAUSE
 * ============================================================================
 *
 * The target is a logical source-level timing subject.
 *
 * It is NOT:
 *
 *     a physical device;
 *     a device identifier;
 *     a hardware address;
 *     a placement decision.
 * ============================================================================
 */

hardwareTimingTargetClause
    : FOR
      hardwareQualifiedName
    ;


/*
 * ============================================================================
 * TIMING BODY
 * ============================================================================
 */

hardwareTimingBody
    : LBRACE
      hardwareTimingItem*
      RBRACE
    ;


/*
 * ============================================================================
 * TIMING ITEM DISPATCH
 * ============================================================================
 *
 * The grammar deliberately separates structurally distinct constructs.
 *
 * This prevents all timing constructs from becoming one ambiguous:
 *
 *     identifier expression
 *
 * production.
 * ============================================================================
 */

hardwareTimingItem
    : hardwareTimingProperty
    | hardwareTimingConstraint
    | hardwareTimingRequirement
    | hardwareTimingPreference
    | hardwareTimingHint
    | hardwareTimingRelation
    | hardwareTimingWindow
    | hardwareTimingPath
    | hardwareTimingException
    | hardwareTimingAssertion
    | hardwareTimingBudget
    | hardwareTimingAnalysis
    | hardwareTimingBlock
    | hardwarePropertyStatement
    ;


/*
 * ============================================================================
 * PROPERTY NAME
 * ============================================================================
 *
 * Existing reserved timing vocabulary is accepted explicitly.
 *
 * Ordinary identifiers remain valid so future timing properties do not require
 * immediate changes to this grammar.
 * ============================================================================
 */

hardwareTimingPropertyName
    : identifier
    | LATENCY
    | THROUGHPUT
    | FREQUENCY
    | POWER
    ;


/*
 * ============================================================================
 * OPTIONAL PROPERTY SUBJECT
 * ============================================================================
 *
 * Allows:
 *
 *     latency datapath <= 20ns;
 *
 * or:
 *
 *     datapath.latency <= 20ns;
 *
 * The semantic layer decides whether the selected property/subject
 * combination is meaningful.
 * ============================================================================
 */

hardwareTimingPropertyTarget
    : hardwareQualifiedName
    ;


/*
 * ============================================================================
 * GENERIC PROPERTY
 * ============================================================================
 *
 * Examples:
 *
 *     period = base_period;
 *     frequency = target_frequency;
 *     jitter = jitter_budget;
 *     skew = skew_budget;
 *     uncertainty = timing_uncertainty;
 *
 * Assignment here means property specification, not hardware allocation.
 * ============================================================================
 */

hardwareTimingProperty
    : hardwareTimingPropertyName
      ASSIGN
      hardwareTimingExpression
      SEMICOLON
    | hardwareTimingPropertyTarget
      DOT
      hardwareTimingPropertyName
      ASSIGN
      hardwareTimingExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * RELATIONAL CONSTRAINT
 * ============================================================================
 *
 * Examples:
 *
 *     latency <= 20ns;
 *     frequency >= minimum_frequency;
 *     setup >= setup_requirement;
 *     hold >= hold_requirement;
 *
 * The grammar does not evaluate the comparison.
 * ============================================================================
 */

hardwareTimingConstraint
    : hardwareTimingPropertyName
      hardwareTimingComparisonOperator
      hardwareTimingExpression
      SEMICOLON
    | hardwareTimingPropertyTarget
      DOT
      hardwareTimingPropertyName
      hardwareTimingComparisonOperator
      hardwareTimingExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * COMPARISON OPERATOR
 * ============================================================================
 */

hardwareTimingComparisonOperator
    : ASSIGN
    | EQUAL_EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


/*
 * ============================================================================
 * REQUIRED TIMING
 * ============================================================================
 *
 * `requires` means the condition is binding semantic intent.
 *
 * The compiler/semantic layer determines whether the selected target can
 * satisfy the requirement.
 * ============================================================================
 */

hardwareTimingRequirement
    : REQUIRES
      hardwareTimingRequirementExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * REQUIREMENT EXPRESSION
 * ============================================================================
 *
 * Supports:
 *
 *     requires latency <= 20ns;
 *
 *     requires frequency >= minimum_frequency;
 *
 *     requires timing::interface::valid;
 *
 * The final semantic interpretation belongs downstream.
 * ============================================================================
 */

hardwareTimingRequirementExpression
    : hardwareTimingPropertyName
      hardwareTimingComparisonOperator
      hardwareTimingExpression
    | hardwareTimingPropertyTarget
      DOT
      hardwareTimingPropertyName
      hardwareTimingComparisonOperator
      hardwareTimingExpression
    | hardwareTimingExpression
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 *
 * Preferences are non-binding implementation guidance.
 * ============================================================================
 */

hardwareTimingPreference
    : PREFER
      hardwareTimingRequirementExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * HINT
 * ============================================================================
 *
 * A hint is advisory.
 *
 * It MUST NOT silently become a semantic requirement.
 * ============================================================================
 */

hardwareTimingHint
    : HINT
      hardwareTimingExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * TIMING RELATION
 * ============================================================================
 *
 * A logical timing relation connects two semantic timing subjects.
 *
 * Example:
 *
 *     relation source to destination {
 *         latency <= 20ns;
 *     }
 *
 * `relation` remains contextual syntax rather than another global keyword.
 * ============================================================================
 */

hardwareTimingRelation
    : hardwareTimingNamedKeywordBlock
      hardwareQualifiedName
      TO
      hardwareQualifiedName
      hardwareTimingBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * TIMING WINDOW
 * ============================================================================
 *
 * A timing window describes an allowed interval.
 *
 * Example:
 *
 *     window transaction {
 *         minimum = 5ns;
 *         maximum = 20ns;
 *     }
 *
 * The semantic model determines whether the window represents:
 *
 *     availability;
 *     execution;
 *     response;
 *     sampling;
 *     synchronization;
 *     another timing concept.
 * ============================================================================
 */

hardwareTimingWindow
    : hardwareTimingNamedKeywordBlock
      identifier
      hardwareTimingBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * TIMING PATH
 * ============================================================================
 *
 * A path is a LOGICAL timing path.
 *
 * It is NOT a physical routing path.
 *
 * Example:
 *
 *     path datapath {
 *         from = producer;
 *         to = consumer;
 *         latency <= 20ns;
 *     }
 * ============================================================================
 */

hardwareTimingPath
    : hardwareTimingNamedKeywordBlock
      identifier
      hardwareTimingBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * TIMING EXCEPTION
 * ============================================================================
 *
 * Timing exceptions modify the semantic timing analysis contract.
 *
 * Examples may include:
 *
 *     false-path intent;
 *     multicycle intent;
 *     asynchronous relationship;
 *     ignored path;
 *     bounded exception.
 *
 * The actual analysis semantics belong downstream.
 * ============================================================================
 */

hardwareTimingException
    : hardwareTimingNamedKeywordBlock
      identifier?
      hardwareTimingBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * TIMING ASSERTION
 * ============================================================================
 *
 * Uses the canonical assertion keyword.
 *
 * Example:
 *
 *     assert latency <= 20ns;
 *
 * Verification interpretation belongs downstream.
 * ============================================================================
 */

hardwareTimingAssertion
    : ASSERT
      hardwareTimingRequirementExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * TIMING BUDGET
 * ============================================================================
 *
 * A budget represents an allowed timing envelope.
 *
 * Example:
 *
 *     budget datapath {
 *         latency <= latency_budget;
 *         jitter <= jitter_budget;
 *     }
 *
 * A budget does NOT allocate a hardware resource.
 * ============================================================================
 */

hardwareTimingBudget
    : hardwareTimingNamedKeywordBlock
      identifier
      hardwareTimingBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * TIMING ANALYSIS INTENT
 * ============================================================================
 *
 * Example:
 *
 *     analysis timing {
 *         ...
 *     }
 *
 * Analysis implementation remains outside the grammar.
 * ============================================================================
 */

hardwareTimingAnalysis
    : hardwareTimingNamedKeywordBlock
      TIMING
      hardwareTimingBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * EXTENSIBLE TIMING BLOCK
 * ============================================================================
 *
 * This rule deliberately permits future timing categories without making every
 * new timing concept a globally reserved keyword.
 *
 * The semantic layer determines whether a named timing block is standardized,
 * dialect-provided, experimental, or invalid.
 *
 * ============================================================================
 */

hardwareTimingBlock
    : hardwareTimingNamedKeywordBlock
      identifier
      hardwareTimingBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * CONTEXTUAL TIMING BLOCK NAME
 * ============================================================================
 *
 * The first identifier is intentionally contextual.
 *
 * Recognized standard names may include:
 *
 *     relation
 *     window
 *     path
 *     exception
 *     budget
 *     analysis
 *     contract
 *
 * Future names may be supplied through dialects/extensions without requiring
 * the core lexer to reserve every possible timing concept.
 *
 * The semantic layer is responsible for classifying the block.
 * ============================================================================
 */

hardwareTimingNamedKeywordBlock
    : identifier
    ;


/*
 * ============================================================================
 * TIMING EXPRESSION BRIDGE
 * ============================================================================
 *
 * The parent hardware grammar currently exposes:
 *
 *     hardwareExpression
 *
 * This adapter gives timing a stable ownership boundary.
 *
 * When the repository completes the universal expression migration, only the
 * parent composition layer needs to redirect the adapter.
 *
 * This file therefore does not need to be rewritten merely because the
 * expression implementation evolves.
 * ============================================================================
 */

hardwareTimingExpression
    : hardwareExpression
    ;


/*
 * ============================================================================
 * LOGICAL TIMING REFERENCE
 * ============================================================================
 *
 * References are semantic names.
 *
 * They do not identify:
 *
 *     physical clock pins;
 *     physical devices;
 *     physical wires;
 *     physical routing paths;
 *     physical FPGA regions;
 *     physical ASIC cells.
 * ============================================================================
 */

hardwareTimingReference
    : hardwareQualifiedName
    ;


/*
 * ============================================================================
 * TIMING PROPERTY REFERENCE
 * ============================================================================
 */

hardwareTimingPropertyReference
    : hardwareTimingReference
    ;


/*
 * ============================================================================
 * TIMING PATH REFERENCE
 * ============================================================================
 */

hardwareTimingPathReference
    : hardwareTimingReference
    ;


/*
 * ============================================================================
 * TIMING WINDOW REFERENCE
 * ============================================================================
 */

hardwareTimingWindowReference
    : hardwareTimingReference
    ;


/*
 * ============================================================================
 * TIMING RELATION REFERENCE
 * ============================================================================
 */

hardwareTimingRelationReference
    : hardwareTimingReference
    ;


/*
 * ============================================================================
 * TIMING DOMAIN REFERENCE
 * ============================================================================
 *
 * This is a logical reference.
 *
 * Clock-domain semantics remain outside this file.
 * ============================================================================
 */

hardwareTimingDomainReference
    : hardwareTimingReference
    ;


/*
 * ============================================================================
 * TIMING CLOCK REFERENCE
 * ============================================================================
 *
 * References a logical clock declared elsewhere.
 *
 * Clock declaration itself remains outside this file.
 * ============================================================================
 */

hardwareTimingClockReference
    : hardwareQualifiedName
    ;


/*
 * ============================================================================
 * TIMING RESOURCE REFERENCE
 * ============================================================================
 *
 * Resource semantics remain owned by the resource/hardware subsystems.
 *
 * The timing grammar merely provides the syntactic reference.
 * ============================================================================
 */

hardwareTimingResourceReference
    : hardwareQualifiedName
    ;


/*
 * ============================================================================
 * TIMING CAPABILITY REFERENCE
 * ============================================================================
 *
 * Capability discovery and validation remain downstream.
 * ============================================================================
 */

hardwareTimingCapabilityReference
    : hardwareQualifiedName
    ;


/*
 * ============================================================================
 * TIMING VALUE
 * ============================================================================
 *
 * A timing value is simply an expression.
 *
 * Examples:
 *
 *     10ns
 *     base_period
 *     2 * base_period
 *     1 / frequency
 *     required_latency
 *
 * Unit and dimensional analysis are semantic responsibilities.
 * ============================================================================
 */

hardwareTimingValue
    : hardwareTimingExpression
    ;


/*
 * ============================================================================
 * TIMING VALUE LIST
 * ============================================================================
 */

hardwareTimingValueList
    : hardwareTimingValue
      (
          COMMA
          hardwareTimingValue
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * TIMING PROPERTY LIST
 * ============================================================================
 */

hardwareTimingPropertyList
    : hardwareTimingProperty+
    ;


/*
 * ============================================================================
 * TIMING REQUIREMENT LIST
 * ============================================================================
 */

hardwareTimingRequirementList
    : hardwareTimingRequirement+
    ;


/*
 * ============================================================================
 * TIMING PREFERENCE LIST
 * ============================================================================
 */

hardwareTimingPreferenceList
    : hardwareTimingPreference+
    ;


/*
 * ============================================================================
 * TIMING ASSERTION LIST
 * ============================================================================
 */

hardwareTimingAssertionList
    : hardwareTimingAssertion+
    ;


/*
 * ============================================================================
 * TIMING DECLARATION LIST
 * ============================================================================
 *
 * No fixed cardinality is encoded.
 * ============================================================================
 */

hardwareTimingDeclarationList
    : hardwareTimingDeclaration+
    ;


/*
 * ============================================================================
 * TIMING ITEM LIST
 * ============================================================================
 */

hardwareTimingItemList
    : hardwareTimingItem*
    ;


/*
 * ============================================================================
 * TIMING CONTRACT
 * ============================================================================
 *
 * A contract is a source-level grouping of timing intent.
 *
 * It remains a logical contract and does not become a scheduler object.
 * ============================================================================
 */

hardwareTimingContract
    : identifier
      hardwareTimingBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * STANDARD PROPERTY REFERENCES
 * ============================================================================
 *
 * These rules provide stable semantic anchors for tooling without requiring
 * each property to become a dedicated parser production.
 * ============================================================================
 */

hardwareTimingLatency
    : LATENCY
    ;


hardwareTimingThroughput
    : THROUGHPUT
    ;


hardwareTimingFrequency
    : FREQUENCY
    ;


/*
 * ============================================================================
 * COMMON TIMING PROPERTY NAMES
 * ============================================================================
 *
 * These remain parser-level identifiers rather than additional global
 * keywords.
 *
 * The semantic layer may recognize:
 *
 *     period
 *     phase
 *     duty_cycle
 *     setup
 *     hold
 *     jitter
 *     skew
 *     uncertainty
 *     delay
 *     arrival
 *     required
 *     release
 *     deadline
 *     duration
 *     minimum
 *     maximum
 *     rise_time
 *     fall_time
 *     slew
 *
 * without requiring new lexical keywords.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Every parsed timing construct must be representable downstream as a
 * domain-neutral timing contract containing, conceptually:
 *
 *     source span
 *     declaration identity
 *     optional target
 *     property/constraint kind
 *     logical subjects
 *     relational operator
 *     timing expression
 *     requirement/preference/hint classification
 *     attributes
 *
 * The grammar does not prescribe the Rust type used for that representation.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser layer MUST preserve:
 *
 *     - declaration source span;
 *     - property source span;
 *     - target/reference source span;
 *     - operator source span;
 *     - timing expression source span;
 *     - ordering of timing items;
 *     - attributes.
 *
 * The domain-neutral AST must NOT require:
 *
 *     physical clocks;
 *     physical devices;
 *     physical timing cells;
 *     vendor primitives;
 *     FPGA resources;
 *     ASIC process information.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT define a new timing IR.
 *
 * Timing semantics must lower into the repository's canonical semantic/IR
 * boundary.
 *
 * For quantum-connected computations:
 *
 *     timing intent
 *          |
 *          v
 *     domain-neutral semantic model
 *          |
 *          v
 *     quantum::ir timing structures where quantum timing is involved
 *
 * The grammar MUST NOT introduce:
 *
 *     HardwareTimingIR
 *     HdlTimingIR
 *     QuantumTimingIR
 *
 * merely as parser-side convenience representations.
 *
 * Existing canonical timing semantics remain authoritative downstream.
 *
 * ============================================================================
 * SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Scheduling consumes semantic timing requirements.
 *
 * Examples:
 *
 *     latency <= 20ns
 *     deadline <= deadline_expression
 *     throughput >= throughput_requirement
 *     period <= period_requirement
 *
 * The scheduler determines:
 *
 *     ordering;
 *     placement;
 *     resource use;
 *     concrete start/end times;
 *     synchronization.
 *
 * This grammar does not construct schedules.
 *
 * ============================================================================
 * CLOCK INTEGRATION
 * ============================================================================
 *
 * This grammar may refer to logical clocks.
 *
 * It does not declare clocks.
 *
 * Clock declarations belong to the clock owner.
 *
 * Timing properties such as:
 *
 *     period
 *     frequency
 *     phase
 *     duty_cycle
 *     jitter
 *     uncertainty
 *
 * are timing requirements/properties when appearing in this file.
 *
 * Whether a target can physically realize them is downstream.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Timing requirements may interact with:
 *
 *     compute capacity;
 *     memory;
 *     bandwidth;
 *     accelerator capability;
 *     communication resources;
 *     quantum resources;
 *     hardware resources.
 *
 * This file does not allocate any resource.
 *
 * Example:
 *
 *     latency <= required_latency;
 *
 * may cause resource analysis to reject an incapable target.
 *
 * That rejection is semantic/resource analysis, not parsing.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * A target may expose timing-related capabilities such as:
 *
 *     hardware::timing::deterministic
 *     hardware::timing::high_resolution
 *     hardware::timing::synchronous
 *
 * Such capabilities are referenced semantically.
 *
 * The grammar does not discover whether a target provides them.
 *
 * ============================================================================
 * HARDWARE HAL INTEGRATION
 * ============================================================================
 *
 * The HAL may expose discovered timing facts:
 *
 *     available frequency;
 *     timing resolution;
 *     latency characteristics;
 *     synchronization capabilities;
 *     hardware-specific timing constraints.
 *
 * This grammar does not query the HAL.
 *
 * The semantic/compiler layer compares source intent against HAL facts.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * HDL timing remains owned by:
 *
 *     grammar/hdl/timing.g4
 *
 * Hardware-contract timing may refer to the same logical concepts but must
 * remain a separate ownership layer.
 *
 * No parser rule in this file may duplicate:
 *
 *     posedge
 *     negedge
 *     always_ff
 *     always_comb
 *     sequential
 *     combinational
 *     HDL clocking blocks
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum timing may involve:
 *
 *     operation duration;
 *     measurement timing;
 *     delay;
 *     synchronization;
 *     pulse timing;
 *     logical timing windows;
 *     QEC rounds;
 *     fault-tolerance timing requirements.
 *
 * This grammar does not implement those semantics.
 *
 * Quantum timing that reaches quantum compilation ultimately follows:
 *
 *     source
 *       |
 *       v
 *     semantic timing
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling
 *       |
 *       v
 *     QEC / resilience / ZQN
 *       |
 *       v
 *     HAL
 *
 * No second quantum IR is introduced.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical timing may describe:
 *
 *     latency;
 *     throughput;
 *     deadlines;
 *     response windows;
 *     deterministic execution;
 *     accelerator interaction.
 *
 * Classical implementation remains downstream.
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * Timing contracts may cross:
 *
 *     classical
 *         ->
 *     accelerator
 *         ->
 *     quantum
 *         ->
 *     classical
 *
 * without requiring separate timing languages.
 *
 * The semantic model records the timing relationship.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed timing may include:
 *
 *     communication latency;
 *     response deadlines;
 *     synchronization windows;
 *     service timing;
 *     throughput;
 *     ordering constraints.
 *
 * Network and distributed semantics remain owned by their respective domains.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * AI/data pipelines may use timing properties for:
 *
 *     inference latency;
 *     throughput;
 *     batch timing;
 *     stream windows;
 *     accelerator response.
 *
 * The grammar does not introduce framework-specific timing constructs.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Security-sensitive timing properties may be semantically relevant.
 *
 * For example:
 *
 *     deterministic timing;
 *     timing isolation;
 *     timing side-channel constraints.
 *
 * Security semantics remain owned by the security subsystem.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * External timing representations may be imported or exported through the
 * interoperability subsystem.
 *
 * This grammar remains the canonical Zamani source representation.
 *
 * External formats MUST NOT silently become a second Zamani timing grammar.
 *
 * ============================================================================
 * DIALECT INTEGRATION
 * ============================================================================
 *
 * Vendor- or technology-specific timing properties should use:
 *
 *     qualified names;
 *     attributes;
 *     dialect extensions.
 *
 * Example:
 *
 *     vendor::timing::some_property = value;
 *
 * Adding a new vendor must not require changing this core grammar.
 *
 * ============================================================================
 * ERROR MODEL
 * ============================================================================
 *
 * Syntax errors:
 *
 *     malformed timing syntax
 *
 * are parser errors.
 *
 * Semantic errors include:
 *
 *     unknown timing property;
 *     incompatible dimensions;
 *     impossible relation;
 *     contradictory requirements;
 *     invalid reference;
 *     unsupported timing capability.
 *
 * Resource errors include:
 *
 *     target cannot satisfy timing requirement.
 *
 * Scheduling errors include:
 *
 *     no valid schedule satisfies the timing contract.
 *
 * These failure classes MUST remain distinguishable.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic predicates;
 *     no actions;
 *     no randomness;
 *     no I/O;
 *     no network access;
 *     no hardware access;
 *     no runtime calls.
 *
 * Equal token streams produce equal parse structures.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains no Rust.
 *
 * Zamani-owned Rust consuming this grammar MUST:
 *
 *     use Rust 1.97 / Rust 1.97.1;
 *     use Rust 2021;
 *     remain safe Rust;
 *     contain no `unsafe` blocks;
 *     contain no unsafe implementation requirement.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * FORBIDDEN:
 *
 *     MAX_CLOCKS
 *     MAX_TIMING_PATHS
 *     MAX_TIMING_CONSTRAINTS
 *     MAX_LATENCY
 *     MAX_FREQUENCY
 *     MAX_PERIOD
 *     MAX_JITTER
 *     MAX_SKEW
 *     MAX_DOMAINS
 *     MAX_DEVICES
 *     CLOCK_0
 *     CLOCK_1
 *     DEVICE_0
 *     DEVICE_1
 *
 * ALSO FORBIDDEN:
 *
 *     fixed clock counts;
 *     fixed timing-path counts;
 *     fixed timing-domain counts;
 *     fixed timing-resolution assumptions;
 *     fixed physical clock IDs;
 *     fixed device IDs;
 *     fixed topology;
 *     fixed register widths;
 *     fixed memory sizes.
 *
 * LEGAL:
 *
 *     latency <= required_latency;
 *     frequency >= target_frequency;
 *     requires capability::timing::deterministic;
 *     requires resource::bandwidth >= required_bandwidth;
 *     requires timing::resolution <= required_resolution;
 *
 * Numeric literals remain ordinary program values.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar intentionally allows arbitrary source-level cardinality for:
 *
 *     timing declarations;
 *     timing items;
 *     timing properties;
 *     paths;
 *     windows;
 *     relations;
 *     constraints;
 *     requirements;
 *     preferences;
 *     assertions.
 *
 * No grammar-level finite machine capacity is assumed.
 *
 * Practical limits may arise from:
 *
 *     source size;
 *     parser memory;
 *     compiler resources;
 *     target resources;
 *     runtime resources;
 *
 * Such limits are implementation/resource limits and MUST NOT be represented
 * as language semantics.
 *
 * ============================================================================
 * INCREMENTAL COMPILATION
 * ============================================================================
 *
 * Timing declarations are independently identifiable by:
 *
 *     declaration name;
 *     optional target;
 *     source span;
 *
 * This permits downstream implementations to cache timing analysis without
 * requiring the grammar file itself to know anything about the cache.
 *
 * ============================================================================
 * SOURCE PROVENANCE
 * ============================================================================
 *
 * Every timing construct must remain source-traceable.
 *
 * Diagnostics should be able to identify:
 *
 *     timing declaration;
 *     timing property;
 *     timing relation;
 *     timing path;
 *     timing window;
 *     requirement;
 *     preference;
 *     hint;
 *     assertion;
 *
 * by source span.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include:
 *
 *     timing datapath {
 *         latency <= 20ns;
 *     }
 *
 *     timing interface_timing for interface::compute {
 *         setup >= setup_requirement;
 *         hold >= hold_requirement;
 *     }
 *
 *     timing throughput_contract {
 *         throughput >= required_throughput;
 *     }
 *
 *     timing symbolic {
 *         period = base_period;
 *         frequency >= target_frequency;
 *         jitter <= jitter_budget;
 *         skew <= skew_budget;
 *     }
 *
 *     timing parameterized {
 *         latency <= 2 * base_latency;
 *     }
 *
 *     timing qualified {
 *         hardware::timing::latency <= required_latency;
 *     }
 *
 *     timing requirements {
 *         requires latency <= required_latency;
 *         prefer throughput >= preferred_throughput;
 *         hint timing::optimization;
 *     }
 *
 *     timing path_contract {
 *         path datapath {
 *             from = producer;
 *             to = consumer;
 *             latency <= latency_budget;
 *         }
 *     }
 *
 *     timing window_contract {
 *         window transaction {
 *             minimum = minimum_window;
 *             maximum = maximum_window;
 *         }
 *     }
 *
 *     timing budget_contract {
 *         budget datapath {
 *             latency <= latency_budget;
 *             jitter <= jitter_budget;
 *         }
 *     }
 *
 * Negative tests MUST include:
 *
 *     timing {
 *     }
 *
 * only if the language policy requires a declaration name;
 *
 * malformed comparison operators;
 * missing expressions;
 * missing braces;
 * missing semicolons where required;
 * malformed qualified names;
 * malformed target references;
 * malformed relation bodies.
 *
 * Boundary tests MUST include:
 *
 *     zero timing items;
 *     one timing item;
 *     many timing items;
 *     deeply qualified names;
 *     symbolic timing values;
 *     very large numeric timing values;
 *     very small duration values;
 *     arbitrarily many timing properties;
 *     nested timing blocks;
 *     generated timing declarations.
 *
 * Scalability tests MUST prove that the grammar contains no timing-capacity
 * constant.
 *
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * Before marking this file integrated:
 *
 * [ ] `TIMING` exists exactly once in the canonical lexical vocabulary.
 *
 * [ ] The parser consumes the canonical token vocabulary.
 *
 * [ ] `hardwareExpression` remains owned by the hardware composition layer.
 *
 * [ ] `hardwareQualifiedName` remains owned by the hardware composition layer.
 *
 * [ ] `hardwarePropertyStatement` remains owned by the hardware composition
 *     layer.
 *
 * [ ] `hardwareTimingDeclaration` is added to `hardwareItem`.
 *
 * [ ] The old inline `hardwareTimingContract` implementation in
 *     `hardware/hardware.g4` is removed as an independent timing owner.
 *
 * [ ] No second hardware timing grammar exists.
 *
 * [ ] `grammar/hdl/timing.g4` remains the HDL timing owner.
 *
 * [ ] `grammar/hdl/clocks.g4` remains the clock declaration owner.
 *
 * [ ] `grammar/hdl/clocking.g4` remains the clocking/CDC owner.
 *
 * [ ] Scheduling consumes semantic timing contracts rather than reparsing
 *     source.
 *
 * [ ] Hardware capability analysis consumes semantic timing facts.
 *
 * [ ] Timing semantics remain target-independent.
 *
 * [ ] Quantum timing reaches canonical `quantum::ir` where applicable.
 *
 * [ ] No second quantum timing IR is introduced.
 *
 * [ ] No physical timing realization is encoded in the grammar.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Hard-coding audit passes.
 *
 * [ ] Rust 1.97 / 1.97.1 integration is verified.
 *
 * [ ] Zamani-owned Rust remains `unsafe`-free.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is considered complete when:
 *
 *     1. It owns only hardware-contract timing syntax.
 *
 *     2. It does not duplicate HDL timing ownership.
 *
 *     3. It does not duplicate clock declaration ownership.
 *
 *     4. It consumes canonical lexical tokens.
 *
 *     5. It delegates general expressions.
 *
 *     6. It delegates semantic timing analysis.
 *
 *     7. It delegates scheduling.
 *
 *     8. It delegates physical realization.
 *
 *     9. It supports symbolic timing quantities.
 *
 *    10. It supports parameterized timing quantities.
 *
 *    11. It supports requirements, constraints, preferences and hints.
 *
 *    12. It supports timing paths, windows, relations, exceptions and budgets.
 *
 *    13. It supports extensible timing property names.
 *
 *    14. It introduces no machine-size limits.
 *
 *    15. It introduces no timing-resolution limits.
 *
 *    16. It introduces no physical-device assumptions.
 *
 *    17. It introduces no vendor dependency.
 *
 *    18. It introduces no second IR.
 *
 *    19. It preserves source provenance.
 *
 *    20. It remains deterministic.
 *
 *    21. It requires no unsafe Rust.
 *
 *    22. It remains compatible with POCO-REAF.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 *     Zamani timing syntax
 *             |
 *             v
 *     domain-neutral AST
 *             |
 *             v
 *     semantic timing contract
 *             |
 *             +-----------------------+
 *             |                       |
 *             v                       v
 *       resource/capability       timing analysis
 *             |                       |
 *             +-----------+-----------+
 *                         |
 *                         v
 *                    scheduling
 *                         |
 *              +----------+----------+
 *              |          |          |
 *              v          v          v
 *          optimization  routing   synthesis
 *              |          |          |
 *              +----------+----------+
 *                         |
 *                         v
 *                     HAL / target
 *
 * The source describes timing intent.
 *
 * The compiler determines a valid realization.
 *
 * The target supplies actual timing capabilities.
 *
 * This separation is mandatory for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * and for Zamani's:
 *
 *     From Atom to Everywhere
 *
 * ============================================================================
 */