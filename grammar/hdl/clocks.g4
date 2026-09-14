/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/clocks.g4
 *
 * Purpose:
 *     Production parser grammar for source-level HDL clock declarations.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, predicates, filesystem
 *     access, network access, process execution, or unsafe implementation.
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
 *          +--> HDL clock syntax
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> clock-domain analysis
 *          +--> type analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> timing legality
 *          |
 *          v
 *     canonical hardware/HDL semantic representation
 *          |
 *          +--> scheduling
 *          +--> optimization
 *          +--> synthesis
 *          +--> placement
 *          +--> routing
 *          +--> target lowering
 *          |
 *          v
 *     CPU / FPGA / ASIC / accelerator / other target
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - HDL clock declaration syntax;
 *   - clock identity syntax;
 *   - clock-domain declaration syntax;
 *   - source/parent clock references;
 *   - clock relationship syntax;
 *   - clock-edge syntax;
 *   - clock polarity syntax;
 *   - period/frequency/duty-cycle/phase syntax;
 *   - jitter and uncertainty syntax;
 *   - enable/gating intent syntax;
 *   - clock attributes;
 *   - source-level clock constraints;
 *   - source-level clock generation relationships;
 *   - parser composition of clock properties.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical token definitions;
 *   - identifier spelling;
 *   - numeric literal spelling;
 *   - duration literal spelling;
 *   - generic expression syntax;
 *   - general type syntax;
 *   - AST construction;
 *   - semantic clock analysis;
 *   - clock-domain crossing analysis;
 *   - clock-tree synthesis;
 *   - clock routing;
 *   - physical oscillator implementation;
 *   - PLL/DLL implementation;
 *   - FPGA primitive selection;
 *   - ASIC cell selection;
 *   - physical pin assignment;
 *   - hardware discovery;
 *   - target selection;
 *   - scheduling;
 *   - runtime clock management;
 *   - power management;
 *   - calibration;
 *   - simulation semantics;
 *   - vendor-specific APIs.
 *
 * ============================================================================
 * POCO-REAF PRINCIPLE
 * ============================================================================
 *
 * A clock declaration describes CLOCK INTENT and SEMANTICS.
 *
 * It MUST NOT permanently encode:
 *
 *   - a particular FPGA;
 *   - a particular ASIC;
 *   - a particular oscillator;
 *   - a physical pin;
 *   - a vendor primitive;
 *   - a fixed PLL;
 *   - a fixed DLL;
 *   - a fixed process node;
 *   - a fixed clock-tree topology;
 *   - a fixed routing resource;
 *   - a fixed machine size;
 *   - a fixed number of clock domains.
 *
 * A source program may express requirements such as:
 *
 *     period = 10ns
 *     frequency = 100MHz
 *     edge = rising
 *
 * but those are semantic constraints or requirements.
 *
 * They do NOT mean:
 *
 *     "the target must contain a 100 MHz oscillator"
 *
 * or:
 *
 *     "use vendor device X".
 *
 * Target realization belongs downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately NO grammar-level finite limits.
 *
 * No:
 *
 *     MAX_CLOCKS
 *     MAX_CLOCK_DOMAINS
 *     MAX_CLOCK_SOURCES
 *     MAX_GENERATED_CLOCKS
 *     MAX_EDGES
 *     MAX_FREQUENCY
 *     MAX_PERIOD
 *     MAX_JITTER
 *     MAX_PHASE
 *     MAX_CHILD_CLOCKS
 *     MAX_CLOCK_TREE_DEPTH
 *
 * Arbitrarily many clock declarations are represented by parser repetition.
 *
 * Practical limits belong to:
 *
 *     parser resource policy
 *     compiler resource policy
 *     semantic analysis
 *     target capability analysis
 *     synthesis
 *     scheduling
 *     deployment
 *     runtime
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical Zamani lexer.
 *
 * The canonical lexer MUST provide:
 *
 *     CLOCK
 *     IDENTIFIER
 *     INTEGER
 *     FLOAT
 *     DURATION_LITERAL
 *
 * together with the normal punctuation/operator vocabulary.
 *
 * CLOCK is the only new lexical reservation required by this grammar.
 *
 * The following MUST NOT become global keywords merely because they are
 * meaningful clock properties:
 *
 *     source
 *     parent
 *     edge
 *     polarity
 *     period
 *     frequency
 *     duty_cycle
 *     phase
 *     jitter
 *     uncertainty
 *     enable
 *     gate
 *     generated
 *     divide
 *     multiply
 *     relation
 *     domain
 *     virtual
 *
 * They remain identifiers and are interpreted contextually by this grammar
 * and semantic analysis.
 *
 * This preserves the identifier namespace and keeps future clock properties
 * extensible.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Clock property values consume the canonical `expression` rule.
 *
 * Consequently clock syntax can use:
 *
 *     literals
 *     constants
 *     generic parameters
 *     compile-time expressions
 *     symbolic expressions
 *     resource expressions
 *     duration literals
 *
 * without duplicating expression grammar.
 *
 * ============================================================================
 * DURATION CONTRACT
 * ============================================================================
 *
 * `DURATION_LITERAL` is owned by the canonical lexer.
 *
 * Examples:
 *
 *     1fs
 *     10ps
 *     100ns
 *     1us
 *     10ms
 *     1s
 *
 * The clock grammar does not redefine duration syntax.
 *
 * ============================================================================
 * FREQUENCY CONTRACT
 * ============================================================================
 *
 * Frequency is intentionally represented as an expression.
 *
 * Examples:
 *
 *     100MHz
 *     1GHz
 *     target_frequency
 *     base_frequency / 2
 *
 * Frequency-unit lexical/semantic handling belongs to the quantity/type
 * subsystem.
 *
 * This grammar must not introduce a fixed frequency range.
 *
 * ============================================================================
 * CLOCK SEMANTIC MODEL
 * ============================================================================
 *
 * A clock declaration can describe:
 *
 *     identity
 *     domain
 *     source
 *     parent
 *     edge
 *     polarity
 *     period
 *     frequency
 *     duty cycle
 *     phase
 *     jitter
 *     uncertainty
 *     enable
 *     gating intent
 *     multiplication
 *     division
 *     generated relationship
 *     attributes
 *     constraints
 *
 * The grammar only establishes syntax.
 *
 * Semantic analysis determines whether combinations are meaningful.
 *
 * ============================================================================
 * NO PHYSICAL ASSUMPTIONS
 * ============================================================================
 *
 * This grammar must remain valid for:
 *
 *     tiny embedded hardware
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     heterogeneous machines
 *     simulators
 *     future hardware
 *
 * The grammar therefore contains no:
 *
 *     device IDs
 *     pin numbers
 *     oscillator IDs
 *     vendor names
 *     fixed frequencies
 *     fixed periods
 *     fixed clock counts
 *     topology assumptions
 *
 * ============================================================================
 */

parser grammar HdlClocks;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC CLOCK DECLARATION
 * ============================================================================
 *
 * Canonical simple form:
 *
 *     clock clk;
 *
 * Typed form:
 *
 *     clock clk: Clock;
 *
 * Property form:
 *
 *     clock clk {
 *         period = 10ns;
 *         edge = rising;
 *     }
 *
 * No fixed number of declarations is imposed.
 */
hdlClockDeclaration
    : CLOCK
      identifier
      hdlClockType?
      hdlClockSpecification?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. CLOCK TYPE
 * ============================================================================
 *
 * The type remains optional because the semantic clock model may infer a
 * canonical clock type from the declaration context.
 */
hdlClockType
    : COLON
      typeExpr
    ;


/*
 * ============================================================================
 * 3. CLOCK SPECIFICATION
 * ============================================================================
 *
 * A clock specification is a block of independent properties.
 *
 * Block form avoids an ever-growing positional grammar and permits future
 * clock properties without changing the meaning of existing declarations.
 */
hdlClockSpecification
    : LBRACE
      hdlClockProperty*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. CLOCK PROPERTY
 * ============================================================================
 *
 * Every property has an identifier key.
 *
 * This deliberately avoids reserving every clock concept globally.
 *
 * Examples:
 *
 *     period = 10ns;
 *     frequency = 100MHz;
 *     edge = rising;
 *     phase = 0deg;
 *     jitter = 5ps;
 *     uncertainty = 1ps;
 *     source = oscillator;
 *     parent = system_clock;
 */
hdlClockProperty
    : identifier
      hdlClockPropertyOperator
      expression
      SEMICOLON?
    | identifier
      hdlClockPropertyBlock
    | hdlClockAttribute
    ;


/*
 * ============================================================================
 * 5. PROPERTY OPERATORS
 * ============================================================================
 *
 * Assignment is the canonical form.
 *
 * A property declaration is not an imperative hardware operation.
 */
hdlClockPropertyOperator
    : ASSIGN
    ;


/*
 * ============================================================================
 * 6. NESTED PROPERTY BLOCK
 * ============================================================================
 *
 * Nested blocks allow extensible clock relationships without requiring new
 * global grammar rules for every future clock-domain feature.
 *
 * Example:
 *
 *     generated {
 *         parent = input_clock;
 *         divide = 2;
 *         multiply = 1;
 *     }
 */
hdlClockPropertyBlock
    : LBRACE
      hdlClockProperty*
      RBRACE
    ;


/*
 * ============================================================================
 * 7. CLOCK ATTRIBUTES
 * ============================================================================
 *
 * Attributes remain metadata.
 *
 * They must not silently trigger target selection or runtime behavior.
 *
 * Examples:
 *
 *     @virtual
 *     @synthesizable
 *     @simulation
 *     @formal
 */
hdlClockAttribute
    : AT
      identifier
      (
          LPAREN
          expressionList?
          RPAREN
      )?
    ;


/*
 * ============================================================================
 * 8. CLOCK DOMAIN DECLARATION
 * ============================================================================
 *
 * A clock domain is a semantic grouping.
 *
 * It is NOT a physical clock-tree object.
 *
 * Example:
 *
 *     clock_domain control {
 *         clock = clk;
 *     }
 *
 * The contextual spelling `clock_domain` remains an identifier rather than
 * becoming another global keyword.
 */
hdlClockDomainDeclaration
    : identifier
      identifier
      LBRACE
      hdlClockDomainProperty*
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 9. CLOCK DOMAIN PROPERTIES
 * ============================================================================
 */
hdlClockDomainProperty
    : identifier
      ASSIGN
      expression
      SEMICOLON?
    | hdlClockAttribute
    ;


/*
 * ============================================================================
 * 10. CLOCK RELATIONSHIP
 * ============================================================================
 *
 * Explicit relationships are source-level declarations.
 *
 * Examples:
 *
 *     clock_relation generated_clock {
 *         parent = source_clock;
 *         divide = 2;
 *     }
 *
 *     clock_relation derived_clock {
 *         parent = input_clock;
 *         multiply = ratio;
 *     }
 *
 * The grammar does not prescribe how the relationship is physically
 * implemented.
 */
hdlClockRelationDeclaration
    : identifier
      identifier
      hdlClockRelationBody
      SEMICOLON?
    ;


hdlClockRelationBody
    : LBRACE
      hdlClockRelationProperty*
      RBRACE
    ;


hdlClockRelationProperty
    : identifier
      ASSIGN
      expression
      SEMICOLON?
    | hdlClockAttribute
    ;


/*
 * ============================================================================
 * 11. GENERATED CLOCK
 * ============================================================================
 *
 * Generated-clock intent is represented structurally.
 *
 * Example:
 *
 *     generated_clock derived {
 *         source = input_clock;
 *         divide = 2;
 *         multiply = 1;
 *     }
 *
 * This grammar does not require a PLL, DLL, divider, multiplier, or any
 * particular hardware primitive.
 */
hdlGeneratedClockDeclaration
    : identifier
      identifier
      LBRACE
      hdlGeneratedClockProperty*
      RBRACE
      SEMICOLON?
    ;


hdlGeneratedClockProperty
    : identifier
      ASSIGN
      expression
      SEMICOLON?
    | hdlClockAttribute
    ;


/*
 * ============================================================================
 * 12. CLOCK EDGE
 * ============================================================================
 *
 * Edge values remain expressions.
 *
 * Canonical semantic values may include:
 *
 *     rising
 *     falling
 *     both
 *
 * but this grammar does not reserve those words.
 *
 * This permits future edge models without lexer changes.
 */
hdlClockEdge
    : expression
    ;


/*
 * ============================================================================
 * 13. CLOCK POLARITY
 * ============================================================================
 *
 * Polarity is similarly semantic rather than lexical.
 *
 * Examples:
 *
 *     active_high
 *     active_low
 *     normal
 *     inverted
 */
hdlClockPolarity
    : expression
    ;


/*
 * ============================================================================
 * 14. CLOCK SOURCE
 * ============================================================================
 *
 * A source is a logical source expression.
 *
 * It is not a physical device identifier.
 */
hdlClockSource
    : expression
    ;


/*
 * ============================================================================
 * 15. CLOCK PERIOD
 * ============================================================================
 *
 * A period is an expression because the source may specify:
 *
 *     10ns
 *     base_period
 *     base_period / 2
 *
 * Semantic validation determines dimensional correctness.
 */
hdlClockPeriod
    : expression
    ;


/*
 * ============================================================================
 * 16. CLOCK FREQUENCY
 * ============================================================================
 */
hdlClockFrequency
    : expression
    ;


/*
 * ============================================================================
 * 17. DUTY CYCLE
 * ============================================================================
 *
 * No numeric range is imposed by the grammar.
 *
 * Semantic validation determines whether the value represents a valid duty
 * cycle under the selected clock model.
 */
hdlClockDutyCycle
    : expression
    ;


/*
 * ============================================================================
 * 18. PHASE
 * ============================================================================
 *
 * Phase may be represented as:
 *
 *     an angle
 *     a duration
 *     a symbolic quantity
 *
 * The semantic quantity/type system decides which forms are legal.
 */
hdlClockPhase
    : expression
    ;


/*
 * ============================================================================
 * 19. JITTER
 * ============================================================================
 */
hdlClockJitter
    : expression
    ;


/*
 * ============================================================================
 * 20. UNCERTAINTY
 * ============================================================================
 */
hdlClockUncertainty
    : expression
    ;


/*
 * ============================================================================
 * 21. CLOCK ENABLE
 * ============================================================================
 *
 * This describes enable intent.
 *
 * It does NOT require a particular gating-cell implementation.
 */
hdlClockEnable
    : expression
    ;


/*
 * ============================================================================
 * 22. CLOCK DIVISION
 * ============================================================================
 */
hdlClockDivision
    : expression
    ;


/*
 * ============================================================================
 * 23. CLOCK MULTIPLICATION
 * ============================================================================
 */
hdlClockMultiplication
    : expression
    ;


/*
 * ============================================================================
 * 24. CLOCK CONSTRAINT
 * ============================================================================
 *
 * Constraints are source-level requirements.
 *
 * They do not perform synthesis or target selection.
 *
 * Example:
 *
 *     constraint {
 *         maximum_jitter = 10ps;
 *         maximum_uncertainty = 2ps;
 *     }
 */
hdlClockConstraint
    : identifier
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 25. CLOCK CONSTRAINT BLOCK
 * ============================================================================
 */
hdlClockConstraintBlock
    : LBRACE
      hdlClockConstraint*
      RBRACE
    ;


/*
 * ============================================================================
 * 26. CLOCK REQUIREMENT
 * ============================================================================
 *
 * Requirements express what must be true of a realizable clock.
 *
 * They do not select the implementation.
 */
hdlClockRequirement
    : identifier
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 27. CLOCK REQUIREMENT BLOCK
 * ============================================================================
 */
hdlClockRequirementBlock
    : LBRACE
      hdlClockRequirement*
      RBRACE
    ;


/*
 * ============================================================================
 * 28. CLOCK LIST
 * ============================================================================
 *
 * Useful for declarations or integration points that accept multiple clock
 * references.
 *
 * No finite clock-count limit exists.
 */
hdlClockList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 29. CLOCK REFERENCE
 * ============================================================================
 *
 * A clock reference is intentionally an expression rather than a special
 * physical identifier.
 *
 * This allows:
 *
 *     clk
 *     clocks.primary
 *     generated_clk
 *     domain.clock
 *
 * while leaving name resolution to the semantic layer.
 */
hdlClockReference
    : expression
    ;


/*
 * ============================================================================
 * 30. CLOCK ASSERTION
 * ============================================================================
 *
 * Source-level clock properties can be asserted without embedding a timing
 * engine in the parser.
 *
 * Example:
 *
 *     clock_assert clk {
 *         period = expected_period;
 *     }
 */
hdlClockAssertion
    : identifier
      identifier
      hdlClockConstraintBlock
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 31. CLOCK GROUP
 * ============================================================================
 *
 * Clock groups are semantic relationships.
 *
 * The parser imposes no limit on group size.
 */
hdlClockGroupDeclaration
    : identifier
      identifier
      LBRACE
      hdlClockGroupProperty*
      RBRACE
      SEMICOLON?
    ;


hdlClockGroupProperty
    : identifier
      ASSIGN
      expression
      SEMICOLON?
    | hdlClockAttribute
    ;


/*
 * ============================================================================
 * 32. CLOCK ATTRIBUTE LIST
 * ============================================================================
 */
hdlClockAttributeList
    : hdlClockAttribute+
    ;


/*
 * ============================================================================
 * 33. CLOCK PROPERTY LIST
 * ============================================================================
 */
hdlClockPropertyList
    : hdlClockProperty+
    ;


/*
 * ============================================================================
 * 34. CLOCK DECLARATION LIST
 * ============================================================================
 */
hdlClockDeclarationList
    : hdlClockDeclaration+
    ;


/*
 * ============================================================================
 * 35. CANONICAL CLOCK SECTION
 * ============================================================================
 *
 * This rule is useful when the parent HDL grammar wants to collect clock
 * declarations without allowing arbitrary non-clock declarations inside the
 * section.
 */
hdlClockSection
    : identifier
      LBRACE
      hdlClockDeclaration*
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 36. CLOCK EXPRESSION VALUE HELPERS
 * ============================================================================
 *
 * These rules provide named integration points for semantic visitors and
 * AST builders while retaining the canonical expression grammar.
 */
hdlClockValue
    : expression
    ;


hdlClockSourceValue
    : expression
    ;


hdlClockTimingValue
    : expression
    ;


hdlClockConstraintValue
    : expression
    ;


/*
 * ============================================================================
 * 37. INTEGRATION NOTES
 * ============================================================================
 *
 * The canonical HDL parser should import this grammar:
 *
 *     import HdlClocks;
 *
 * and consume:
 *
 *     hdlClockDeclaration
 *
 * as one of its module members.
 *
 * The old inline implementation of:
 *
 *     hdlClockDeclaration
 *     hdlClockProperty
 *     hdlClockPropertyBlock
 *
 * MUST be removed from hdl.g4 once this grammar is imported.
 *
 * hdl.g4 should retain composition responsibility only.
 *
 * ============================================================================
 * 38. AST CONTRACT
 * ============================================================================
 *
 * The parser must expose enough structure for the frontend AST to represent:
 *
 *     ClockDeclaration {
 *         name
 *         type?
 *         specification?
 *         source_span
 *         attributes
 *     }
 *
 * The grammar must NOT construct that AST itself.
 *
 * The AST layer owns:
 *
 *     identifiers
 *     source spans
 *     normalized property representation
 *     declaration identity
 *
 * ============================================================================
 * 39. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must validate at least:
 *
 *     - clock name resolution;
 *     - duplicate clock declarations;
 *     - property-name validity;
 *     - property type compatibility;
 *     - duration dimensions;
 *     - frequency dimensions;
 *     - phase dimensions;
 *     - jitter dimensions;
 *     - uncertainty dimensions;
 *     - duty-cycle domain;
 *     - divide/multiply validity;
 *     - parent/source relationships;
 *     - cyclic clock relationships;
 *     - clock-domain consistency;
 *     - conflicting clock properties;
 *     - target capability requirements.
 *
 * None of those checks belong in this grammar.
 *
 * ============================================================================
 * 40. HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware abstraction consumes the semantic clock model.
 *
 * It may determine:
 *
 *     - available clock sources;
 *     - realizable frequency;
 *     - supported relationships;
 *     - timing capability;
 *     - clock-generation mechanisms;
 *     - physical constraints.
 *
 * This grammar must never query hardware.
 *
 * ============================================================================
 * 41. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Scheduling may consume semantic clock information for:
 *
 *     timing;
 *     ordering;
 *     alignment;
 *     resource availability;
 *     synchronization;
 *     execution constraints.
 *
 * The clock grammar has NO dependency on the scheduler.
 *
 * Direction:
 *
 *     grammar
 *        ->
 *     AST
 *        ->
 *     semantic clock model
 *        ->
 *     scheduling
 *
 * ============================================================================
 * 42. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum timing may consume the same canonical duration/clock concepts for:
 *
 *     pulse timing;
 *     gate timing;
 *     measurement windows;
 *     synchronization;
 *     dynamic-circuit timing.
 *
 * This file does not create quantum operations and does not access
 * `quantum::ir`.
 *
 * If a clock declaration affects quantum execution, semantic lowering is
 * responsible for translating the meaning into the appropriate canonical
 * representation.
 *
 * ============================================================================
 * 43. QEC / ZQN INTEGRATION
 * ============================================================================
 *
 * This grammar has no direct dependency on:
 *
 *     QEC
 *     ZQN
 *
 * QEC or ZQN may consume downstream timing/resource metadata where relevant.
 *
 * The dependency direction MUST remain:
 *
 *     grammar
 *       ->
 *     semantic model
 *       ->
 *     QEC/ZQN-aware compilation
 *
 * and never:
 *
 *     clocks.g4 -> QEC
 *     clocks.g4 -> ZQN
 *
 * ============================================================================
 * 44. OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * Optimization may transform clock realization where semantics permit.
 *
 * Examples:
 *
 *     clock-source substitution;
 *     clock-divider realization;
 *     redundant-clock elimination;
 *     clock-domain normalization.
 *
 * Such transformations must preserve semantic clock requirements.
 *
 * clocks.g4 has no dependency on optimization.
 *
 * ============================================================================
 * 45. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime may consume the lowered clock model where the execution target
 * exposes runtime clock control.
 *
 * The grammar must not imply that runtime clock control exists.
 *
 * ============================================================================
 * 46. DETERMINISM
 * ============================================================================
 *
 * Parsing this grammar must be deterministic.
 *
 * There are:
 *
 *     no semantic predicates;
 *     no target-specific actions;
 *     no embedded Rust;
 *     no runtime state;
 *     no external I/O.
 *
 * ============================================================================
 * 47. SECURITY
 * ============================================================================
 *
 * Clock syntax cannot:
 *
 *     execute commands;
 *     access files;
 *     access networks;
 *     discover hardware;
 *     mutate compiler state;
 *     select credentials;
 *     select physical devices.
 *
 * ============================================================================
 * 48. COMPATIBILITY
 * ============================================================================
 *
 * Adding a new clock property should normally NOT require a new lexer keyword.
 *
 * Example:
 *
 *     clock clk {
 *         future_property = value;
 *     }
 *
 * can remain syntactically representable while semantic validation determines
 * whether the property is known for the active language version/dialect.
 *
 * This is deliberate forward-compatibility behavior.
 *
 * ============================================================================
 * 49. SCALABILITY
 * ============================================================================
 *
 * The grammar has no finite limits on:
 *
 *     clock declarations;
 *     clock properties;
 *     nested property blocks;
 *     clock relationships;
 *     clock groups;
 *     expression complexity.
 *
 * Actual parser/compilation limits are implementation resource limits rather
 * than language semantics.
 *
 * ============================================================================
 * 50. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *   [ ] It compiles as an ANTLR parser grammar against ZamaniLexer.
 *   [ ] CLOCK exists in the canonical lexer.
 *   [ ] identifier resolves to the canonical identifier rule.
 *   [ ] typeExpr resolves to the canonical type grammar.
 *   [ ] expression resolves to the canonical expression grammar.
 *   [ ] expressionList resolves to the canonical expression grammar.
 *   [ ] The parent HDL grammar imports HdlClocks.
 *   [ ] Duplicate clock rules are removed from hdl.g4.
 *   [ ] No duplicate clock token exists in another lexer.
 *   [ ] No fixed clock/resource limits exist.
 *   [ ] No Rust actions or unsafe code exist.
 *   [ ] Positive clock tests pass.
 *   [ ] Negative clock tests pass.
 *   [ ] Boundary/scalability tests pass.
 *   [ ] Round-trip tests preserve clock syntax.
 *   [ ] Cross-domain HDL tests pass.
 *   [ ] Semantic validation rejects invalid clock properties.
 *   [ ] Hardware lowering consumes the semantic representation rather than
 *       this grammar directly.
 *   [ ] Quantum timing consumes canonical semantic data rather than this
 *       grammar directly.
 *
 * ============================================================================
 */