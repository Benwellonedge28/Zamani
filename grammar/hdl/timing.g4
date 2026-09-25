/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/timing.g4
 *
 * Status:
 *     CANONICAL HDL TIMING SYNTAX COMPONENT
 *
 * Purpose:
 *     Defines portable, source-level timing intent for HDL and
 *     hardware/software co-design.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     No embedded Rust.
 *     No actions.
 *     No semantic predicates.
 *     No I/O.
 *     No hardware access.
 *     No unsafe implementation.
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
 *     grammar/hdl/timing.g4
 *          |
 *          v
 *     grammar/hdl/hdl.g4
 *          |
 *          v
 *     grammar/Zamani.g4
 *
 * Canonical lexical authority:
 *
 *     grammar/lexer/tokens.g4
 *
 * Canonical HDL composition root:
 *
 *     grammar/hdl/hdl.g4
 *
 * This file is a parser component, not a lexer and not an independent
 * language root.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - timing declarations;
 *   - timing properties;
 *   - timing requirements;
 *   - timing constraints;
 *   - timing relations;
 *   - timing windows;
 *   - timing paths;
 *   - timing exceptions;
 *   - timing assertions;
 *   - timing budgets;
 *   - timing requirements/preferences/hints;
 *   - timing analysis intent;
 *   - source-level timing metadata.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - clock declarations;
 *   - clock generation;
 *   - clock domains as physical objects;
 *   - clock-tree construction;
 *   - PLL/DLL selection;
 *   - oscillators;
 *   - clock routing;
 *   - clock pins;
 *   - CDC implementation;
 *   - synchronizer implementation;
 *   - synthesis;
 *   - placement;
 *   - routing;
 *   - scheduling algorithms;
 *   - physical timing closure;
 *   - target selection;
 *   - hardware discovery;
 *   - calibration;
 *   - vendor primitives;
 *   - runtime implementation;
 *   - canonical IR construction.
 *
 * Clock declarations belong to clocks.g4.
 *
 * Clocking/CDC context belongs to clocking.g4.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Timing syntax expresses WHAT timing properties are required.
 *
 * It must not encode WHICH physical implementation satisfies them.
 *
 * Therefore this grammar contains no:
 *
 *     MAX_CLOCKS
 *     MAX_TIMING_PATHS
 *     MAX_TIMING_CONSTRAINTS
 *     MAX_FREQUENCY
 *     MAX_PERIOD
 *     MAX_LATENCY
 *     MAX_MODULES
 *     MAX_DOMAINS
 *     MAX_DEVICES
 *     MAX_RESOURCES
 *
 * Nor does it encode:
 *
 *     FPGA family limits
 *     ASIC process limits
 *     physical clock-tree limits
 *     vendor timing-engine limits
 *     CPU/GPU/QPU limits
 *     machine-size limits
 *
 * A timing value is program intent.
 *
 * A target's ability to satisfy that intent is a downstream semantic/resource
 * question.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * All tokens come from the canonical Zamani lexer vocabulary.
 *
 * The timing component MUST NOT define lexer rules.
 *
 * Structural timing syntax uses the canonical TIMING token.
 *
 * Timing property names remain contextual where possible so that the language
 * does not require a new lexer keyword for every future timing concept.
 *
 * Existing canonical lexical concepts such as:
 *
 *     LATENCY
 *     THROUGHPUT
 *
 * may therefore be accepted as timing property names in addition to ordinary
 * identifiers.
 *
 * If a future timing concept becomes a globally reserved keyword, that change
 * belongs in grammar/lexer/keywords.g4 and grammar/compatibility/.
 *
 * ============================================================================
 * SHARED PARSER CONTRACT
 * ============================================================================
 *
 * This file consumes shared parser rules supplied by the HDL composition root,
 * including:
 *
 *     identifier
 *     hdlQualifiedName
 *     hdlExpression
 *     hdlAttribute
 *     hdlAttributeList
 *
 * This file MUST NOT redefine those rules.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parser responsibility:
 *
 *     recognize valid timing intent syntax.
 *
 * Semantic responsibility:
 *
 *     resolve names;
 *     validate dimensions;
 *     validate units;
 *     validate relationships;
 *     determine whether constraints are compatible;
 *     determine whether requirements are satisfiable;
 *     distinguish requirements from preferences and hints;
 *     construct the domain-neutral semantic timing model.
 *
 * Compiler/backend responsibility:
 *
 *     timing analysis;
 *     optimization;
 *     scheduling;
 *     synthesis;
 *     placement;
 *     routing;
 *     timing closure;
 *     target realization.
 *
 * ============================================================================
 * RESOURCE MODEL
 * ============================================================================
 *
 * Timing values are expressions.
 *
 * Examples:
 *
 *     period = 10ns;
 *     frequency = 100MHz;
 *     latency <= 20ns;
 *     setup >= 2ns;
 *
 * Dimensional correctness belongs to semantic analysis.
 *
 * The grammar does not impose a finite numeric range.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * PARSER DECLARATION
 * ============================================================================
 */

parser grammar HdlTiming;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC TIMING DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     timing name {
 *         ...
 *     }
 *
 * Anonymous timing declarations are deliberately not accepted here.
 *
 * Giving timing intent a logical name improves:
 *
 *     diagnostics
 *     references
 *     tooling
 *     semantic identity
 *     reproducibility
 *
 * It does NOT impose any limit on the number of timing declarations.
 * ============================================================================
 */

hdlTimingDeclaration
    : TIMING
      identifier
      hdlTimingTargetClause?
      hdlTimingBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. OPTIONAL TARGET
 * ============================================================================
 *
 * Examples:
 *
 *     timing datapath for module_name {
 *         ...
 *     }
 *
 *     timing interface_timing for interface_name {
 *         ...
 *     }
 *
 * The target is a logical source-level name.
 *
 * It is not a physical device.
 * ============================================================================
 */

hdlTimingTargetClause
    : FOR
      hdlQualifiedName
    ;


/*
 * ============================================================================
 * 3. TIMING BODY
 * ============================================================================
 */

hdlTimingBody
    : LBRACE
      hdlTimingItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. TIMING ITEM
 * ============================================================================
 *
 * The alternatives are intentionally structurally distinct.
 *
 * This avoids the previous design in which multiple constructs were simply:
 *
 *     hdlKeyword ...
 *
 * causing substantial ambiguity.
 * ============================================================================
 */

hdlTimingItem
    : hdlTimingProperty
    | hdlTimingConstraint
    | hdlTimingRequirement
    | hdlTimingPreference
    | hdlTimingHint
    | hdlTimingRelation
    | hdlTimingWindow
    | hdlTimingPath
    | hdlTimingException
    | hdlTimingAssertion
    | hdlTimingBudget
    | hdlTimingAnalysis
    | hdlTimingBlock
    | hdlAttribute
    ;


/*
 * ============================================================================
 * 5. PROPERTY NAME
 * ============================================================================
 *
 * Most timing properties remain contextual identifiers.
 *
 * LATENCY and THROUGHPUT already exist in the canonical lexical vocabulary,
 * so they are accepted explicitly.
 *
 * Future timing properties can remain identifiers without expanding the
 * global lexer vocabulary.
 * ============================================================================
 */

hdlTimingPropertyName
    : identifier
    | LATENCY
    | THROUGHPUT
    ;


/*
 * ============================================================================
 * 6. GENERIC TIMING PROPERTY
 * ============================================================================
 *
 * Examples:
 *
 *     period = 10ns;
 *     frequency = 100MHz;
 *     phase = phase_offset;
 *     jitter = allowed_jitter;
 *     skew = allowed_skew;
 *     uncertainty = timing_uncertainty;
 *
 * The property name is semantic data.
 * ============================================================================
 */

hdlTimingProperty
    : hdlTimingPropertyName
      ASSIGN
      hdlExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. TIMING CONSTRAINT
 * ============================================================================
 *
 * Constraints can use relational operators rather than only assignment.
 *
 * Examples:
 *
 *     latency <= 20ns;
 *     setup >= 2ns;
 *     hold >= 1ns;
 *     frequency >= minimum_frequency;
 *     period <= maximum_period;
 *
 * This is critical because timing intent is fundamentally relational.
 * ============================================================================
 */

hdlTimingConstraint
    : hdlTimingPropertyName
      hdlTimingComparisonOperator
      hdlExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. COMPARISON OPERATORS
 * ============================================================================
 */

hdlTimingComparisonOperator
    : ASSIGN
    | EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    ;


/*
 * ============================================================================
 * 9. REQUIREMENT
 * ============================================================================
 *
 * A requirement is binding semantic intent.
 *
 * It is distinct from a preference or implementation hint.
 *
 * Example:
 *
 *     require latency <= 20ns;
 *
 * The semantic layer determines whether the target can satisfy it.
 * ============================================================================
 */

hdlTimingRequirement
    : REQUIRE
      hdlTimingRequirementExpression
      SEMICOLON
    ;


hdlTimingRequirementExpression
    : hdlTimingPropertyName
      hdlTimingComparisonOperator
      hdlExpression
    | hdlExpression
    ;


/*
 * ============================================================================
 * 10. PREFERENCE
 * ============================================================================
 *
 * A preference is non-binding optimization guidance.
 * ============================================================================
 */

hdlTimingPreference
    : PREFER
      hdlTimingRequirementExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. HINT
 * ============================================================================
 *
 * A hint is implementation guidance and must not silently become a semantic
 * requirement.
 * ============================================================================
 */

hdlTimingHint
    : HINT
      hdlExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. TIMING RELATION
 * ============================================================================
 *
 * Relates two logical timing objects.
 *
 * Examples:
 *
 *     relation source to destination {
 *         latency <= 20ns;
 *     }
 *
 *     relation producer to consumer {
 *         setup >= 2ns;
 *     }
 *
 * These are logical relationships, not physical routing paths.
 * ============================================================================
 */

hdlTimingRelation
    : RELATION
      hdlQualifiedName
      TO
      hdlQualifiedName
      hdlTimingRelationBody
      SEMICOLON?
    ;


hdlTimingRelationBody
    : LBRACE
      hdlTimingItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 13. TIMING WINDOW
 * ============================================================================
 *
 * Represents a semantic interval.
 *
 * Example:
 *
 *     window transaction {
 *         minimum = 5ns;
 *         maximum = 20ns;
 *     }
 * ============================================================================
 */

hdlTimingWindow
    : WINDOW
      identifier
      hdlTimingBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 14. TIMING PATH
 * ============================================================================
 *
 * A timing path is a semantic path.
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

hdlTimingPath
    : PATH
      identifier
      hdlTimingBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 15. TIMING EXCEPTION
 * ============================================================================
 *
 * Timing exceptions alter semantic timing analysis.
 *
 * Examples:
 *
 *     exception false_path {
 *         from = source;
 *         to = destination;
 *     }
 *
 *     exception multicycle_path {
 *         factor = cycles;
 *     }
 *
 * The exception does not directly control a timing-analysis engine.
 * ============================================================================
 */

hdlTimingException
    : EXCEPTION
      identifier?
      hdlTimingBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 16. TIMING ASSERTION
 * ============================================================================
 *
 * Assertions are source-level verification intent.
 *
 * Example:
 *
 *     assert_timing latency <= 20ns;
 *
 * The exact verification mode is determined downstream.
 * ============================================================================
 */

hdlTimingAssertion
    : ASSERT_TIMING
      hdlTimingAssertionExpression
      SEMICOLON
    ;


hdlTimingAssertionExpression
    : hdlTimingPropertyName
      hdlTimingComparisonOperator
      hdlExpression
    | hdlExpression
    ;


/*
 * ============================================================================
 * 17. TIMING BUDGET
 * ============================================================================
 *
 * A budget expresses an allowed timing resource envelope.
 *
 * Example:
 *
 *     budget datapath {
 *         latency <= 20ns;
 *         skew <= 100ps;
 *     }
 *
 * A budget is not a hardware allocation.
 * ============================================================================
 */

hdlTimingBudget
    : BUDGET
      identifier
      hdlTimingBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 18. TIMING ANALYSIS INTENT
 * ============================================================================
 *
 * This declares analysis intent, not an analysis implementation.
 *
 * Example:
 *
 *     analysis timing {
 *         ...
 *     }
 *
 * The compiler may map this to:
 *
 *     static analysis
 *     dynamic analysis
 *     formal analysis
 *     statistical analysis
 *
 * without making those engines part of the source grammar.
 * ============================================================================
 */

hdlTimingAnalysis
    : ANALYSIS
      TIMING
      hdlTimingBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 19. TIMING NAMESPACE/BLOCK
 * ============================================================================
 *
 * Allows future timing concepts to be grouped without creating another
 * top-level timing grammar.
 *
 * Example:
 *
 *     timing_group constraints {
 *         ...
 *     }
 *
 * `identifier` remains open-ended.
 * ============================================================================
 */

hdlTimingBlock
    : identifier
      LBRACE
      hdlTimingItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 20. STANDALONE TIMING CONSTRAINT SET
 * ============================================================================
 *
 * This rule is available to HDL composition and tooling that need to parse a
 * sequence of constraints without a complete timing declaration.
 * ============================================================================
 */

hdlTimingConstraintSet
    : hdlTimingConstraint+
    ;


/*
 * ============================================================================
 * 21. TIMING PROPERTY LIST
 * ============================================================================
 *
 * Reusable list boundary for semantic tooling.
 * ============================================================================
 */

hdlTimingPropertyList
    : hdlTimingProperty+
    ;


/*
 * ============================================================================
 * 22. TIMING VALUE
 * ============================================================================
 *
 * Timing values deliberately delegate completely to hdlExpression.
 *
 * Therefore the grammar can support:
 *
 *     10ns
 *     10 ns
 *     base_period
 *     2 * base_period
 *     1 / frequency
 *     required_latency
 *     symbolic expressions
 *     parameterized values
 *     compile-time values
 *
 * without creating a second numeric-expression grammar.
 *
 * Dimensional/unit checking belongs to semantic analysis.
 * ============================================================================
 */

hdlTimingValue
    : hdlExpression
    ;


/*
 * ============================================================================
 * 23. TIMING REQUIREMENT LIST
 * ============================================================================
 */

hdlTimingRequirementList
    : hdlTimingRequirement+
    ;


/*
 * ============================================================================
 * 24. TIMING PREFERENCE LIST
 * ============================================================================
 */

hdlTimingPreferenceList
    : hdlTimingPreference+
    ;


/*
 * ============================================================================
 * 25. TIMING ASSERTION LIST
 * ============================================================================
 */

hdlTimingAssertionList
    : hdlTimingAssertion+
    ;


/*
 * ============================================================================
 * 26. TIMING DECLARATION LIST
 * ============================================================================
 *
 * No fixed number of timing declarations exists.
 * ============================================================================
 */

hdlTimingDeclarationList
    : hdlTimingDeclaration+
    ;


/*
 * ============================================================================
 * 27. TIMING REFERENCE
 * ============================================================================
 *
 * Timing references are logical names.
 *
 * They do not imply physical clock pins, cells, resources, or routes.
 * ============================================================================
 */

hdlTimingReference
    : hdlQualifiedName
    ;


/*
 * ============================================================================
 * 28. TIMING RELATION REFERENCE
 * ============================================================================
 */

hdlTimingRelationReference
    : hdlTimingReference
    ;


/*
 * ============================================================================
 * 29. TIMING PATH REFERENCE
 * ============================================================================
 */

hdlTimingPathReference
    : hdlTimingReference
    ;


/*
 * ============================================================================
 * 30. TIMING DOMAIN REFERENCE
 * ============================================================================
 *
 * A timing-domain reference is semantic.
 *
 * Physical clock-domain implementation remains downstream.
 * ============================================================================
 */

hdlTimingDomainReference
    : hdlTimingReference
    ;


/*
 * ============================================================================
 * 31. TIMING PROPERTY TARGET
 * ============================================================================
 *
 * Allows a property to identify the logical object to which it applies.
 *
 * Example:
 *
 *     latency datapath <= 20ns;
 *
 * The semantic layer determines whether this form is meaningful.
 * ============================================================================
 */

hdlTimingPropertyTarget
    : hdlTimingReference
    ;


/*
 * ============================================================================
 * 32. TIMING METADATA
 * ============================================================================
 *
 * Metadata remains attached to source-level intent.
 * ============================================================================
 */

hdlTimingMetadata
    : hdlAttribute
    ;


/*
 * ============================================================================
 * 33. TIMING METADATA LIST
 * ============================================================================
 */

hdlTimingMetadataList
    : hdlAttribute+
    ;


/*
 * ============================================================================
 * 34. TIMING CONTRACT
 * ============================================================================
 *
 * A contract groups requirements and optional preferences.
 *
 * Example:
 *
 *     contract datapath_timing {
 *         require latency <= 20ns;
 *         prefer frequency >= target_frequency;
 *     }
 *
 * Contract semantics are downstream.
 * ============================================================================
 */

hdlTimingContract
    : CONTRACT
      identifier
      hdlTimingBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 35. TIMING CONTRACT LIST
 * ============================================================================
 */

hdlTimingContractList
    : hdlTimingContract+
    ;


/*
 * ============================================================================
 * 36. TIMING SCALABILITY CONTRACT
 * ============================================================================
 *
 * This rule intentionally contains no numeric bounds.
 *
 * Any number of timing items is legal subject to parser/compiler resources.
 * ============================================================================
 */

hdlTimingItemList
    : hdlTimingItem*
    ;


/*
 * ============================================================================
 * END OF TIMING GRAMMAR
 * ============================================================================
 *
 * Architectural invariant:
 *
 *     timing syntax
 *          ->
 *     domain-neutral AST
 *          ->
 *     semantic timing model
 *          ->
 *     canonical hardware semantic representation / IR
 *          ->
 *     timing analysis / optimization / scheduling / synthesis / realization
 *
 * This grammar never selects a physical implementation.
 *
 * ============================================================================
 */