/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/timing.g4
 *
 * Purpose:
 *     Production parser grammar for source-level HDL timing intent.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, process execution, or unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/lexer/tokens.g4
 *          |
 *          |  lexer grammar ZamaniTokens
 *          v
 *     canonical HDL parser
 *          |
 *          +--> hdl.g4
 *          |
 *          +--> clocks.g4
 *          |
 *          +--> timing.g4  <--- THIS FILE
 *          |
 *          +--> combinational.g4
 *          +--> sequential.g4
 *          +--> processes.g4
 *          +--> state-machines.g4
 *          +--> pipelines.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> timing analysis
 *          +--> clock-domain analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> constraint validation
 *          |
 *          v
 *     canonical HDL / hardware semantic representation
 *          |
 *          +--> optimization
 *          +--> scheduling
 *          +--> synthesis
 *          +--> placement
 *          +--> routing
 *          +--> verification
 *          +--> target lowering
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - HDL timing declaration syntax;
 *   - timing intent;
 *   - timing constraints;
 *   - timing requirements;
 *   - timing preferences;
 *   - timing windows;
 *   - latency declarations;
 *   - setup/hold intent;
 *   - recovery/removal intent;
 *   - skew/jitter/uncertainty intent;
 *   - propagation/transition intent;
 *   - timing relationships;
 *   - timing exceptions;
 *   - timing metadata;
 *   - source-level timing assertions.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - clock declaration syntax;
 *   - clock generation;
 *   - physical clock-tree construction;
 *   - PLL/DLL selection;
 *   - oscillator selection;
 *   - clock routing;
 *   - physical pins;
 *   - physical addresses;
 *   - FPGA resources;
 *   - ASIC cells;
 *   - synthesis;
 *   - placement;
 *   - routing;
 *   - scheduling algorithms;
 *   - hardware discovery;
 *   - target selection;
 *   - calibration;
 *   - runtime clock control;
 *   - simulation implementation;
 *   - vendor APIs;
 *   - canonical IR construction.
 *
 * Clock semantics remain owned by clocks.g4 / the HDL semantic layer.
 * This file may REFER to clocks but must not redefine clock declarations.
 *
 * ============================================================================
 * POCO-REAF PRINCIPLE
 * ============================================================================
 *
 * Timing syntax describes requirements and intent.
 *
 * It MUST NOT encode:
 *
 *   - a fixed FPGA;
 *   - a fixed ASIC;
 *   - a fixed process node;
 *   - a fixed clock-tree topology;
 *   - a fixed routing fabric;
 *   - a fixed number of timing domains;
 *   - a fixed number of modules;
 *   - a fixed number of paths;
 *   - a fixed machine size;
 *   - a fixed device;
 *   - a fixed vendor;
 *   - a fixed timing-analysis engine.
 *
 * A source program may say:
 *
 *     period = 10ns;
 *     latency <= 20ns;
 *     setup >= 2ns;
 *
 * These are semantic requirements.
 *
 * They do not select a physical implementation.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No finite grammar-level limits are imposed.
 *
 * There is deliberately no:
 *
 *     MAX_TIMING_CONSTRAINTS
 *     MAX_TIMING_PATHS
 *     MAX_CLOCK_DOMAINS
 *     MAX_CLOCKS
 *     MAX_LATENCY
 *     MAX_FREQUENCY
 *     MAX_PERIOD
 *     MAX_SKEW
 *     MAX_JITTER
 *     MAX_MODULES
 *     MAX_PATHS
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * Practical limits belong to:
 *
 *     parser resource policy
 *     compiler resource policy
 *     semantic analysis
 *     timing-analysis engines
 *     target capabilities
 *     synthesis
 *     scheduling
 *     deployment
 *     runtime
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The authoritative lexical grammar is:
 *
 *     grammar/lexer/tokens.g4
 *
 * whose grammar name is:
 *
 *     ZamaniTokens
 *
 * This file therefore uses:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * Timing words such as:
 *
 *     timing
 *     period
 *     frequency
 *     latency
 *     setup
 *     hold
 *     skew
 *     jitter
 *     uncertainty
 *     deadline
 *     window
 *     path
 *     false_path
 *     multicycle
 *
 * remain contextual HDL identifiers.
 *
 * This is intentional.
 *
 * A timing concept must not become a language-wide reserved keyword merely
 * because it is meaningful inside an HDL timing declaration.
 *
 * If a future language revision promotes a timing word to a reserved keyword,
 * the compatibility process must update this parser and its tests together.
 *
 * ============================================================================
 * SHARED HDL CONTRACT
 * ============================================================================
 *
 * This parser component is intended to be composed with the HDL parser layer.
 *
 * It consumes shared parser rules supplied by the HDL parser composition:
 *
 *     identifier
 *     hdlKeyword
 *     hdlExpression
 *     hdlAttribute
 *
 * It MUST NOT redefine them.
 *
 * This prevents:
 *
 *     timing -> expression -> timing
 *
 * dependency cycles and prevents timing.g4 from becoming a second expression
 * grammar.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This grammar answers:
 *
 *     "What timing intent did the programmer write?"
 *
 * It does NOT answer:
 *
 *     "Can the selected device realize that timing?"
 *
 * The latter belongs downstream.
 *
 * ============================================================================
 */

parser grammar HdlTiming;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The canonical HDL composition already expects:
 *
 *     hdlTimingDeclaration
 *
 * This is therefore the stable integration rule.
 *
 * Canonical structural form:
 *
 *     timing name {
 *         ...
 *     }
 *
 * The word `timing` is contextual through hdlKeyword.
 *
 * Semantic analysis MUST verify that the declaration's leading contextual
 * keyword denotes a timing declaration.
 */
hdlTimingDeclaration
    : hdlKeyword
      identifier
      hdlTimingTargetClause?
      hdlTimingBody
      SEMICOLON?
    ;


/* ============================================================================
 * 2. OPTIONAL TIMING TARGET
 * ============================================================================
 *
 * A timing declaration may identify the semantic object to which the timing
 * requirements apply.
 *
 * Examples:
 *
 *     timing datapath {
 *         ...
 *     }
 *
 *     timing datapath for module_name {
 *         ...
 *     }
 *
 * `for` remains contextual and is represented by hdlKeyword.
 */
hdlTimingTargetClause
    : hdlKeyword
      identifier
    ;


/* ============================================================================
 * 3. TIMING BODY
 * ============================================================================
 *
 * An arbitrary number of timing items is allowed.
 */
hdlTimingBody
    : LBRACE
      hdlTimingItem*
      RBRACE
    ;


/* ============================================================================
 * 4. TIMING ITEM
 * ============================================================================
 *
 * Timing declarations intentionally use a property-oriented model.
 *
 * This keeps the grammar extensible without requiring every future timing
 * concept to become a new language-wide keyword.
 */
hdlTimingItem
    : hdlTimingAttribute
    | hdlTimingAssignment
    | hdlTimingRelation
    | hdlTimingWindow
    | hdlTimingException
    | hdlTimingAssertion
    | hdlTimingNestedBlock
    ;


/* ============================================================================
 * 5. TIMING ATTRIBUTES
 * ============================================================================
 *
 * Attributes are metadata.
 *
 * They do not themselves select a device, clock, implementation technology,
 * synthesis primitive, or backend.
 */
hdlTimingAttribute
    : hdlAttribute
    ;


/* ============================================================================
 * 6. GENERIC TIMING ASSIGNMENT
 * ============================================================================
 *
 * Examples:
 *
 *     period = 10ns;
 *     frequency = 100MHz;
 *     latency = 20ns;
 *     setup = 2ns;
 *     hold = 1ns;
 *     jitter = 5ps;
 *     uncertainty = 1ps;
 *     skew = 100ps;
 *
 * The property name is contextual.
 *
 * The value can be:
 *
 *     - an ordinary HDL expression;
 *     - a symbolic expression;
 *     - a numeric quantity followed by a unit identifier.
 *
 * This avoids requiring a fixed list of timing quantities.
 */
hdlTimingAssignment
    : hdlKeyword
      ASSIGN
      hdlTimingValue
      SEMICOLON?
    ;


/* ============================================================================
 * 7. TIMING VALUE
 * ============================================================================
 *
 * The parser accepts both semantic expressions and compact physical-quantity
 * spellings.
 *
 * Examples:
 *
 *     period = 10 ns;
 *     period = 10ns;
 *     period = base_period;
 *     period = base_period / divisor;
 *     frequency = target_frequency;
 *
 * The lexical layer currently represents `10ns` as a numeric token followed
 * by an identifier. This grammar therefore deliberately does not require a
 * dedicated DURATION_LITERAL token.
 *
 * Quantity dimensionality belongs to semantic analysis.
 */
hdlTimingValue
    : hdlTimingQuantity
    | hdlExpression
    ;


/* ============================================================================
 * 8. TIMING QUANTITY
 * ============================================================================
 *
 * Numeric magnitude plus contextual unit.
 *
 * This is deliberately not limited to a predefined unit list.
 *
 * Therefore future units can be introduced without changing the parser.
 *
 * Semantic analysis validates whether the unit is recognized and whether it
 * is dimensionally appropriate for the property being assigned.
 *
 * Examples:
 *
 *     10 ns
 *     10ns
 *     2 ps
 *     100 MHz
 *     1 GHz
 *
 * A unit is an identifier, not a hardware/device name.
 */
hdlTimingQuantity
    : INTEGER_LITERAL
      identifier
    | FLOAT_LITERAL
      identifier
    | HEX_INTEGER
      identifier
    | BINARY_INTEGER
      identifier
    | OCTAL_INTEGER
      identifier
    ;


/* ============================================================================
 * 9. TIMING RELATION
 * ============================================================================
 *
 * Relations express timing between named semantic objects.
 *
 * Examples:
 *
 *     relation source to destination {
 *         latency <= 20ns;
 *     }
 *
 *     relation producer consumer {
 *         setup >= 2ns;
 *     }
 *
 * The actual meaning of the names is resolved semantically.
 */
hdlTimingRelation
    : hdlKeyword
      identifier
      hdlKeyword
      identifier
      hdlTimingBody
      SEMICOLON?
    ;


/* ============================================================================
 * 10. TIMING WINDOW
 * ============================================================================
 *
 * Timing windows allow a constraint to describe an interval rather than a
 * single scalar.
 *
 * Examples:
 *
 *     window transaction {
 *         minimum = 5ns;
 *         maximum = 20ns;
 *     }
 *
 *     window acquisition {
 *         start = lower_bound;
 *         end = upper_bound;
 *     }
 */
hdlTimingWindow
    : hdlKeyword
      identifier
      hdlTimingBody
      SEMICOLON?
    ;


/* ============================================================================
 * 11. TIMING EXCEPTIONS
 * ============================================================================
 *
 * Timing exceptions express semantic exceptions to ordinary path analysis.
 *
 * Examples:
 *
 *     exception false_path {
 *         from = source;
 *         to = destination;
 *     }
 *
 *     exception multicycle {
 *         factor = cycles;
 *     }
 *
 * The grammar does not prescribe how the backend realizes the exception.
 */
hdlTimingException
    : hdlKeyword
      identifier
      hdlTimingBody
      SEMICOLON?
    ;


/* ============================================================================
 * 12. TIMING ASSERTIONS
 * ============================================================================
 *
 * Assertions express source-level timing expectations.
 *
 * Examples:
 *
 *     assert_timing {
 *         latency <= 20ns;
 *     }
 *
 *     assert_timing {
 *         setup >= 2ns;
 *     }
 *
 * The semantic layer decides whether the assertion is:
 *
 *     - statically provable;
 *     - target-dependent;
 *     - simulation-only;
 *     - formal-verification-only;
 *     - unsatisfied.
 */
hdlTimingAssertion
    : hdlKeyword
      hdlTimingBody
      SEMICOLON?
    ;


/* ============================================================================
 * 13. NESTED TIMING BLOCK
 * ============================================================================
 *
 * A nested block is useful for namespacing related timing properties without
 * requiring the grammar to know every future timing-analysis concept.
 *
 * Example:
 *
 *     clock_domain {
 *         setup = 2ns;
 *         hold = 1ns;
 *     }
 */
hdlTimingNestedBlock
    : hdlKeyword
      hdlTimingBody
    ;


/* ============================================================================
 * 14. TIMING CONSTRAINT SET
 * ============================================================================
 *
 * This rule provides a stable parser boundary for consumers that need to
 * recognize a sequence of timing constraints independently of a declaration.
 *
 * It does not impose a finite number of constraints.
 */
hdlTimingConstraintSet
    : hdlTimingConstraint+
    ;


hdlTimingConstraint
    : hdlKeyword
      (
          ASSIGN
          hdlTimingValue
      )?
      SEMICOLON?
    ;


/* ============================================================================
 * 15. TIMING PROPERTY VALUE
 * ============================================================================
 *
 * Named timing values are intentionally generic.
 *
 * The semantic layer maps property names to canonical timing concepts.
 *
 * This supports future timing concepts without requiring a grammar rewrite.
 */
hdlTimingPropertyName
    : hdlKeyword
    ;


/* ============================================================================
 * 16. COMMON SEMANTIC TIMING FIELDS
 * ============================================================================
 *
 * These rules are parser-facing named boundaries.
 *
 * They do not reserve the names as lexer keywords.
 *
 * They provide stable locations for semantic tooling and AST mapping.
 * ============================================================================
 */

hdlTimingPeriod
    : hdlKeyword
      ASSIGN
      hdlTimingValue
      SEMICOLON?
    ;


hdlTimingFrequency
    : hdlKeyword
      ASSIGN
      hdlTimingValue
      SEMICOLON?
    ;


hdlTimingLatency
    : hdlKeyword
      ASSIGN
      hdlTimingValue
      SEMICOLON?
    ;


hdlTimingSetup
    : hdlKeyword
      ASSIGN
      hdlTimingValue
      SEMICOLON?
    ;


hdlTimingHold
    : hdlKeyword
      ASSIGN
      hdlTimingValue
      SEMICOLON?
    ;


hdlTimingSkew
    : hdlKeyword
      ASSIGN
      hdlTimingValue
      SEMICOLON?
    ;


hdlTimingJitter
    : hdlKeyword
      ASSIGN
      hdlTimingValue
      SEMICOLON?
    ;


hdlTimingUncertainty
    : hdlKeyword
      ASSIGN
      hdlTimingValue
      SEMICOLON?
    ;


/* ============================================================================
 * 17. MINIMUM / MAXIMUM TIMING VALUES
 * ============================================================================
 *
 * No fixed numeric range is encoded here.
 */
hdlTimingMinimum
    : hdlKeyword
      ASSIGN
      hdlTimingValue
      SEMICOLON?
    ;


hdlTimingMaximum
    : hdlKeyword
      ASSIGN
      hdlTimingValue
      SEMICOLON?
    ;


/* ============================================================================
 * 18. DEADLINE
 * ============================================================================
 */
hdlTimingDeadline
    : hdlKeyword
      ASSIGN
      hdlTimingValue
      SEMICOLON?
    ;


/* ============================================================================
 * 19. TIMING EDGE / EVENT RELATIONSHIP
 * ============================================================================
 *
 * Timing may refer to semantic events without defining their hardware
 * implementation.
 *
 * Example:
 *
 *     edge source to destination {
 *         latency <= 10ns;
 *     }
 */
hdlTimingEventRelation
    : hdlKeyword
      identifier
      hdlKeyword
      identifier
      hdlTimingBody
      SEMICOLON?
    ;


/* ============================================================================
 * 20. TIMING PATH
 * ============================================================================
 *
 * Timing paths are semantic paths.
 *
 * They are not physical routing paths.
 *
 * This distinction is critical for POCO-REAF.
 */
hdlTimingPath
    : hdlKeyword
      identifier
      hdlTimingPathEndpointClause?
      hdlTimingBody
      SEMICOLON?
    ;


hdlTimingPathEndpointClause
    : hdlKeyword
      identifier
      (
          hdlKeyword
          identifier
      )?
    ;


/* ============================================================================
 * 21. TIMING DOMAIN
 * ============================================================================
 *
 * A timing domain is a semantic analysis domain.
 *
 * It is not a physical clock-tree domain.
 */
hdlTimingDomain
    : hdlKeyword
      identifier
      hdlTimingBody
      SEMICOLON?
    ;


/* ============================================================================
 * 22. TIMING MODE
 * ============================================================================
 *
 * Timing modes permit source-level intent such as:
 *
 *     functional
 *     test
 *     debug
 *     low_power
 *     verification
 *
 * without reserving these names globally.
 */
hdlTimingMode
    : hdlKeyword
      identifier
      hdlTimingBody
      SEMICOLON?
    ;


/* ============================================================================
 * 23. TIMING PARAMETERIZATION
 * ============================================================================
 *
 * Timing values may depend on symbolic program parameters.
 *
 * Examples:
 *
 *     period = base_period;
 *     latency = stages * stage_latency;
 *
 * This grammar does not limit parameter count or expression complexity.
 */
hdlTimingParameter
    : hdlKeyword
      ASSIGN
      hdlExpression
      SEMICOLON?
    ;


/* ============================================================================
 * 24. TIMING RELATIONSHIP BODY
 * ============================================================================
 *
 * Kept as a separate stable rule so future semantic tooling can distinguish
 * timing relationships from ordinary property blocks.
 */
hdlTimingRelationshipBody
    : hdlTimingBody
    ;


/* ============================================================================
 * 25. TIMING METADATA
 * ============================================================================
 *
 * Metadata is deliberately syntax-only.
 */
hdlTimingMetadata
    : hdlAttribute
    ;


/* ============================================================================
 * 26. TIMING RESOURCE / CAPABILITY REFERENCES
 * ============================================================================
 *
 * A timing declaration may refer to semantic capabilities or resources.
 *
 * It must not directly select physical hardware.
 *
 * Example:
 *
 *     requires timing_capability;
 *
 * Interpretation belongs to capability/resource analysis.
 */
hdlTimingCapabilityReference
    : hdlKeyword
      identifier
      SEMICOLON?
    ;


/* ============================================================================
 * 27. TIMING CONSTRAINT EXPRESSION
 * ============================================================================
 *
 * This rule provides a stable semantic boundary for downstream timing
 * constraint extraction.
 */
hdlTimingConstraintExpression
    : hdlTimingValue
    ;


/* ============================================================================
 * 28. TIMING COMPARISON
 * ============================================================================
 *
 * Comparison operators are already owned by the canonical lexer.
 *
 * Examples:
 *
 *     latency <= 20ns
 *     setup >= 2ns
 *     skew < maximum_skew
 */
hdlTimingComparison
    : hdlTimingValue
      hdlTimingComparisonOperator
      hdlTimingValue
    ;


hdlTimingComparisonOperator
    : LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    | EQUAL_EQUAL
    | NOT_EQUAL
    ;


/* ============================================================================
 * 29. EXPLICIT TIMING CONDITION
 * ============================================================================
 *
 * Conditions remain expressions and are not interpreted by this grammar.
 */
hdlTimingCondition
    : hdlExpression
    ;


/* ============================================================================
 * 30. TIMING CONDITIONAL
 * ============================================================================
 *
 * Example:
 *
 *     when condition {
 *         latency <= 20ns;
 *     }
 *
 * The condition remains semantic HDL expression syntax.
 */
hdlTimingConditional
    : hdlKeyword
      hdlTimingCondition
      hdlTimingBody
    ;


/* ============================================================================
 * 31. TIMING SCENARIO
 * ============================================================================
 *
 * A scenario groups timing requirements for a named semantic situation.
 */
hdlTimingScenario
    : hdlKeyword
      identifier
      hdlTimingBody
      SEMICOLON?
    ;


/* ============================================================================
 * 32. TIMING REQUIREMENT
 * ============================================================================
 *
 * Requirements express conditions that must be satisfied by a realization.
 *
 * They do not directly select an implementation.
 */
hdlTimingRequirement
    : hdlKeyword
      hdlTimingComparison
      SEMICOLON?
    ;


/* ============================================================================
 * 33. TIMING PREFERENCE
 * ============================================================================
 *
 * Preferences differ semantically from hard requirements.
 *
 * A backend may trade a preference against other constraints.
 */
hdlTimingPreference
    : hdlKeyword
      hdlTimingComparison
      SEMICOLON?
    ;


/* ============================================================================
 * 34. TIMING HINT
 * ============================================================================
 *
 * Hints are advisory and must never silently become hard constraints.
 */
hdlTimingHint
    : hdlKeyword
      hdlTimingValue
      SEMICOLON?
    ;


/* ============================================================================
 * 35. TIMING RESOURCE CONSTRAINT
 * ============================================================================
 *
 * Timing/resource coupling is represented symbolically.
 *
 * Physical resource availability is evaluated downstream.
 */
hdlTimingResourceConstraint
    : hdlKeyword
      identifier
      hdlTimingBody
      SEMICOLON?
    ;


/* ============================================================================
 * 36. END-TO-END TIMING CONTRACT
 * ============================================================================
 *
 * This rule represents a semantic contract over a source-to-destination
 * relationship.
 *
 * Example:
 *
 *     contract source destination {
 *         latency <= 20ns;
 *         setup >= 2ns;
 *     }
 */
hdlTimingContract
    : hdlKeyword
      identifier
      hdlKeyword
      identifier
      hdlTimingBody
      SEMICOLON?
    ;


/* ============================================================================
 * 37. TIMING ITEM GROUP
 * ============================================================================
 *
 * This stable composition rule is useful for AST consumers that need to
 * process timing items without depending on the concrete declaration form.
 */
hdlTimingItemGroup
    : LBRACE
      hdlTimingItem*
      RBRACE
    ;


/* ============================================================================
 * 38. SEMANTICALLY EMPTY / MARKER TIMING DECLARATION
 * ============================================================================
 *
 * An empty timing body is syntactically legal:
 *
 *     timing example {}
 *
 * It carries no timing requirement and therefore cannot impose target
 * constraints by itself.
 *
 * This is useful for generated/intermediate source and forward-compatible
 * tooling.
 *
 * Semantic analysis may warn when an empty declaration has no observable
 * purpose, but the grammar must not reject it.
 */
hdlEmptyTimingDeclaration
    : hdlKeyword
      identifier
      LBRACE
      RBRACE
      SEMICOLON?
    ;