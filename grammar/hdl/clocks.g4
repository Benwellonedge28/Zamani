/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/clocks.g4
 *
 * Status:
 *     CANONICAL HDL CLOCK SYNTAX COMPONENT
 *
 * Purpose:
 *     Defines source-level clock declarations, clock domains, clock
 *     relationships, generated-clock intent, and clock references.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust, no actions, no semantic
 *     predicates, no I/O, no hardware access, and no unsafe implementation.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical Zamani lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +--> HDL composition
 *          |       |
 *          |       +--> clocks.g4
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type/quantity analysis
 *          +--> clock-domain analysis
 *          +--> timing analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> CDC analysis
 *          |
 *          v
 *     canonical hardware semantic representation
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
 * SINGLE OWNER
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - source-level HDL clock declarations;
 *   - clock declaration specifications;
 *   - clock-domain declarations;
 *   - generated-clock intent;
 *   - clock source relationships;
 *   - clock parent relationships;
 *   - clock phase/frequency/period intent;
 *   - clock duty-cycle intent;
 *   - clock jitter/uncertainty intent;
 *   - clock enable/gating intent;
 *   - clock multiplication/division intent;
 *   - logical clock groups;
 *   - logical clock references;
 *   - clock-specific attributes.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical definitions;
 *   - identifiers;
 *   - general expressions;
 *   - general types;
 *   - timing analysis;
 *   - clock-domain-crossing analysis;
 *   - physical clock trees;
 *   - oscillators;
 *   - PLLs;
 *   - DLLs;
 *   - clock buffers;
 *   - physical clock pins;
 *   - vendor primitives;
 *   - FPGA-specific resources;
 *   - ASIC-specific resources;
 *   - synthesis;
 *   - placement;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - runtime clock control;
 *   - target selection;
 *   - hardware discovery.
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * The parent HDL grammar is the composition owner.
 *
 * It MUST import this grammar and delegate clock declarations to:
 *
 *     hdlClockDeclaration
 *
 * It MUST NOT retain an independent duplicate implementation of:
 *
 *     hdlClockDeclaration
 *     hdlClockSpecification
 *     hdlClockProperty
 *     hdlClockDomainDeclaration
 *     hdlGeneratedClockDeclaration
 *
 * Once this file is integrated, those duplicate rules MUST be removed from
 * grammar/hdl/hdl.g4.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical Zamani lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * `clock` is a declaration introducer and therefore MUST have one canonical
 * lexical token in the production lexer.
 *
 * The required token name is:
 *
 *     K_CLOCK
 *
 * The canonical lexical owner of K_CLOCK is:
 *
 *     grammar/lexer/keywords.g4
 *
 * and its spelling is:
 *
 *     K_CLOCK : 'clock' ;
 *
 * `clock` MUST NOT be defined again anywhere else.
 *
 * Clock property names are deliberately NOT reserved globally.
 *
 * Therefore names such as:
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
 *     domain
 *     relation
 *
 * remain identifiers.
 *
 * ============================================================================
 * SHARED PARSER CONTRACT
 * ============================================================================
 *
 * This grammar consumes shared parser rules supplied by the canonical HDL
 * parser composition:
 *
 *     identifier
 *     hdlExpression
 *     hdlTypeExpression
 *     hdlAttribute
 *     hdlAttributeList
 *
 * This file MUST NOT redefine those rules.
 *
 * ============================================================================
 * WHY CLOCK IS A KEYWORD
 * ============================================================================
 *
 * Unlike a property such as `period`, `clock` introduces a declaration.
 *
 * If `clock` remained an ordinary identifier, a module member beginning with:
 *
 *     clock clk
 *
 * could become ambiguous with ordinary expression/member syntax.
 *
 * Therefore:
 *
 *     clock
 *
 * is a genuine syntactic introducer and is lexically reserved.
 *
 * Property vocabulary remains contextual and open-world.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A clock declaration expresses portable CLOCK INTENT.
 *
 * It does NOT select:
 *
 *   - a physical oscillator;
 *   - a PLL;
 *   - a DLL;
 *   - a clock buffer;
 *   - a clock pin;
 *   - a vendor primitive;
 *   - a particular FPGA;
 *   - a particular ASIC;
 *   - a process node;
 *   - a physical clock tree;
 *   - a physical routing path.
 *
 * Example:
 *
 *     clock system {
 *         frequency = target_frequency;
 *         edge = rising;
 *     }
 *
 * expresses semantic intent.
 *
 * It does not mean:
 *
 *     "use device X's oscillator".
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no finite language-level limits on:
 *
 *   - clocks;
 *   - clock domains;
 *   - clock groups;
 *   - generated clocks;
 *   - clock properties;
 *   - nested clock specifications;
 *   - clock relationships;
 *   - expression complexity;
 *   - design hierarchy.
 *
 * There are deliberately no:
 *
 *     MAX_CLOCKS
 *     MAX_CLOCK_DOMAINS
 *     MAX_CLOCK_SOURCES
 *     MAX_GENERATED_CLOCKS
 *     MAX_CLOCK_GROUPS
 *     MAX_CLOCK_TREE_DEPTH
 *     MAX_FREQUENCY
 *     MAX_PERIOD
 *     MAX_JITTER
 *     MAX_PHASE
 *     MAX_DUTY_CYCLE
 *
 * Any operational parser/compiler/resource limits are implementation or
 * deployment limits and MUST NOT become language semantics.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE / REALIZATION
 * ============================================================================
 *
 * Clock syntax must preserve the distinction between:
 *
 *     requirement
 *     capability
 *     preference
 *     hint
 *     realization
 *
 * For example:
 *
 *     frequency = required_frequency
 *
 * expresses clock intent.
 *
 * A target capability check may later determine whether a target can realize
 * that intent.
 *
 * This grammar never performs that check.
 *
 * ============================================================================
 * QUANTITY CONTRACT
 * ============================================================================
 *
 * Clock values use the shared expression grammar.
 *
 * Examples:
 *
 *     period = 10ns;
 *     period = base_period;
 *     period = base_period / divisor;
 *
 *     frequency = 100MHz;
 *     frequency = target_frequency;
 *     frequency = base_frequency * multiplier;
 *
 * The grammar does not define the dimensional semantics of these values.
 *
 * Quantity/type analysis belongs downstream.
 *
 * If the canonical lexer emits DURATION_LITERAL, it may appear naturally
 * through hdlExpression.
 *
 * If a compatible source form represents a quantity as separate numeric and
 * identifier tokens, that representation is likewise handled by the shared
 * expression/quantity layer.
 *
 * clocks.g4 MUST NOT create a second duration grammar.
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * Every public clock construct must remain recoverable with its source span.
 *
 * At minimum, the AST/semantic layer must be able to locate:
 *
 *   - declaration;
 *   - clock name;
 *   - type;
 *   - each property;
 *   - each property value;
 *   - each relationship;
 *   - each domain;
 *   - each attribute.
 *
 * This grammar does not manufacture spans; ANTLR token/rule locations provide
 * the source information consumed by the frontend.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 1. CLOCK DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     clock clk;
 *
 *     clock clk: Clock;
 *
 *     clock clk {
 *         period = 10ns;
 *         edge = rising;
 *     }
 *
 * The declaration is intentionally property-oriented.
 *
 * This allows future clock properties to be represented without continually
 * expanding the global keyword vocabulary.
 * ============================================================================
 */

hdlClockDeclaration
    : K_CLOCK
      identifier
      hdlClockType?
      hdlClockSpecification?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. OPTIONAL CLOCK TYPE
 * ============================================================================
 *
 * Type semantics belong to the shared type system.
 * ============================================================================
 */

hdlClockType
    : COLON
      hdlTypeExpression
    ;


/*
 * ============================================================================
 * 3. CLOCK SPECIFICATION
 * ============================================================================
 *
 * An empty specification is legal syntactically.
 *
 * Semantic analysis may impose whatever requirements the active clock model
 * requires.
 * ============================================================================
 */

hdlClockSpecification
    : LBRACE
      hdlClockItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. CLOCK ITEM
 * ============================================================================
 *
 * Attributes and property assignments are the fundamental forms.
 *
 * Nested named blocks provide an extensibility boundary for relationships
 * such as:
 *
 *     generated { ... }
 *     source { ... }
 *     gating { ... }
 * ============================================================================
 */

hdlClockItem
    : hdlAttribute
    | hdlClockProperty
    | hdlClockNamedBlock
    ;


/*
 * ============================================================================
 * 5. CLOCK PROPERTY
 * ============================================================================
 *
 * Property names are identifiers rather than global keywords.
 *
 * Examples:
 *
 *     period = 10ns;
 *     frequency = 100MHz;
 *     edge = rising;
 *     polarity = active_high;
 *     phase = 90deg;
 *     jitter = 5ps;
 *     uncertainty = 1ps;
 *     enable = enable_signal;
 * ============================================================================
 */

hdlClockProperty
    : identifier
      ASSIGN
      hdlExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 6. NAMED CLOCK BLOCK
 * ============================================================================
 *
 * This provides an open-world extension mechanism while retaining structural
 * determinism.
 *
 * Examples:
 *
 *     generated {
 *         parent = source_clock;
 *         divide = 2;
 *     }
 *
 *     gating {
 *         enable = enable_signal;
 *     }
 *
 *     source {
 *         reference = oscillator;
 *     }
 *
 * The semantic layer determines whether a named block is known and what it
 * means.
 * ============================================================================
 */

hdlClockNamedBlock
    : identifier
      LBRACE
      hdlClockItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 7. CLOCK DOMAIN DECLARATION
 * ============================================================================
 *
 * A clock domain is a logical semantic domain, not a physical clock-tree
 * object.
 *
 * Canonical form:
 *
 *     clock_domain control {
 *         clock = control_clock;
 *     }
 *
 * `clock_domain` is represented as a structural introducer through the
 * canonical K_CLOCK token followed by the contextual identifier `domain`.
 *
 * This avoids adding another global keyword while retaining an unambiguous
 * declaration form.
 * ============================================================================
 */

hdlClockDomainDeclaration
    : K_CLOCK
      identifier
      identifier
      hdlClockDomainSpecification
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 8. CLOCK DOMAIN SPECIFICATION
 * ============================================================================
 */

hdlClockDomainSpecification
    : LBRACE
      hdlClockDomainItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 9. CLOCK DOMAIN ITEM
 * ============================================================================
 */

hdlClockDomainItem
    : hdlAttribute
    | hdlClockDomainProperty
    | hdlClockNamedBlock
    ;


/*
 * ============================================================================
 * 10. CLOCK DOMAIN PROPERTY
 * ============================================================================
 */

hdlClockDomainProperty
    : identifier
      ASSIGN
      hdlExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 11. GENERATED CLOCK DECLARATION
 * ============================================================================
 *
 * Generated clocks describe a semantic relationship.
 *
 * Example:
 *
 *     generated_clock derived {
 *         source = input_clock;
 *         divide = 2;
 *         multiply = 1;
 *     }
 *
 * No PLL/DLL/divider/multiplier primitive is selected here.
 *
 * IMPORTANT:
 *
 * `generated_clock` is represented by:
 *
 *     K_CLOCK + contextual identifier `generated_clock`
 *
 * only if the surrounding HDL composition explicitly chooses this form.
 *
 * To avoid accidental ambiguity with an ordinary clock declaration, the
 * canonical form below uses the reserved `clock` introducer and an explicit
 * `generated` property block.
 *
 * Preferred source form:
 *
 *     clock derived {
 *         generated {
 *             source = input_clock;
 *             divide = 2;
 *             multiply = 1;
 *         }
 *     }
 *
 * Therefore no separate generated-clock keyword is required.
 * ============================================================================
 */


/*
 * ============================================================================
 * 12. CLOCK RELATIONSHIP
 * ============================================================================
 *
 * A relationship is represented inside a clock specification:
 *
 *     clock derived {
 *         relationship {
 *             parent = source;
 *             divide = ratio;
 *         }
 *     }
 *
 * This avoids introducing a second top-level declaration family for what is
 * semantically still a property of a clock.
 * ============================================================================
 */

hdlClockRelationship
    : identifier
      LBRACE
      hdlClockRelationshipItem*
      RBRACE
    ;


hdlClockRelationshipItem
    : hdlAttribute
    | hdlClockProperty
    | hdlClockNamedBlock
    ;


/*
 * ============================================================================
 * 13. GENERATED-CLOCK INTENT
 * ============================================================================
 *
 * Named rule for semantic tooling.
 *
 * It is intentionally composed from ordinary clock properties.
 * ============================================================================
 */

hdlGeneratedClockIntent
    : identifier
      LBRACE
      hdlGeneratedClockItem*
      RBRACE
    ;


hdlGeneratedClockItem
    : hdlAttribute
    | hdlClockProperty
    | hdlClockNamedBlock
    ;


/*
 * ============================================================================
 * 14. CLOCK REFERENCE
 * ============================================================================
 *
 * A clock reference is a semantic name/path.
 *
 * It does not identify a physical clock pin.
 *
 * Examples:
 *
 *     clk
 *     clocks.system
 *     domain.control
 *     generated_clock
 *
 * Name resolution belongs to semantic analysis.
 * ============================================================================
 */

hdlClockReference
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


/*
 * ============================================================================
 * 15. CLOCK REFERENCE LIST
 * ============================================================================
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
 * 16. CLOCK GROUP
 * ============================================================================
 *
 * Groups are logical relationships used by timing/CDC analysis.
 *
 * They do not describe physical clock-tree grouping.
 * ============================================================================
 */

hdlClockGroupDeclaration
    : K_CLOCK
      identifier
      identifier
      LBRACE
      hdlClockGroupItem*
      RBRACE
      SEMICOLON?
    ;


hdlClockGroupItem
    : hdlAttribute
    | hdlClockGroupProperty
    ;


hdlClockGroupProperty
    : identifier
      ASSIGN
      hdlExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 17. CLOCK GROUP REFERENCES
 * ============================================================================
 */

hdlClockGroupReferences
    : hdlClockReferenceList
    ;


/*
 * ============================================================================
 * 18. CLOCK ATTRIBUTE LIST
 * ============================================================================
 *
 * Kept as a named integration point for AST/tooling consumers.
 * ============================================================================
 */

hdlClockAttributes
    : hdlAttribute+
    ;


/*
 * ============================================================================
 * 19. CLOCK PROPERTY LIST
 * ============================================================================
 */

hdlClockProperties
    : hdlClockProperty+
    ;


/*
 * ============================================================================
 * 20. CLOCK ITEM LIST
 * ============================================================================
 */

hdlClockItemList
    : hdlClockItem+
    ;


/*
 * ============================================================================
 * 21. CLOCK DECLARATION LIST
 * ============================================================================
 *
 * No finite number of declarations is imposed.
 * ============================================================================
 */

hdlClockDeclarationList
    : hdlClockDeclaration+
    ;


/*
 * ============================================================================
 * 22. CLOCK SECTION
 * ============================================================================
 *
 * Optional structural section for an HDL composition that wants to group
 * clock declarations.
 *
 * Example:
 *
 *     clocks {
 *         clock system { ... }
 *         clock control { ... }
 *     }
 *
 * The contextual word `clocks` remains an identifier.
 * ============================================================================
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
 * 23. CLOCK VALUE
 * ============================================================================
 *
 * Stable semantic-tooling boundary.
 *
 * The value remains the canonical HDL expression.
 * ============================================================================
 */

hdlClockValue
    : hdlExpression
    ;


/*
 * ============================================================================
 * 24. CLOCK TIMING VALUE
 * ============================================================================
 *
 * Timing dimensionality is validated semantically.
 *
 * The parser does not distinguish:
 *
 *     duration
 *     frequency
 *     phase
 *     ratio
 *
 * merely from syntax.
 * ============================================================================
 */

hdlClockTimingValue
    : hdlExpression
    ;


/*
 * ============================================================================
 * 25. CLOCK SOURCE VALUE
 * ============================================================================
 */

hdlClockSourceValue
    : hdlExpression
    ;


/*
 * ============================================================================
 * 26. CLOCK CONSTRAINT VALUE
 * ============================================================================
 */

hdlClockConstraintValue
    : hdlExpression
    ;


/*
 * ============================================================================
 * 27. CLOCK REQUIREMENT
 * ============================================================================
 *
 * Requirement syntax is deliberately expression-based.
 *
 * The semantic/resource system determines whether the expression represents:
 *
 *     timing
 *     capability
 *     capacity
 *     availability
 *     reliability
 *     another supported requirement.
 *
 * This prevents clocks.g4 from becoming a second resource language.
 * ============================================================================
 */

hdlClockRequirement
    : K_REQUIRES
      hdlExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 28. CLOCK PREFERENCE
 * ============================================================================
 *
 * Preferences are non-binding implementation guidance.
 * ============================================================================
 */

hdlClockPreference
    : K_PREFER
      hdlExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 29. CLOCK HINT
 * ============================================================================
 *
 * Hints are non-semantic guidance.
 * ============================================================================
 */

hdlClockHint
    : K_HINT
      hdlExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 30. CLOCK CONTRACT
 * ============================================================================
 *
 * Provides a stable integration boundary for clock-related semantic contracts
 * without defining backend realization.
 * ============================================================================
 */

hdlClockContract
    : K_CONTRACT
      identifier
      LBRACE
      hdlClockContractItem*
      RBRACE
      SEMICOLON?
    ;


hdlClockContractItem
    : hdlAttribute
    | hdlClockProperty
    | hdlClockRequirement
    | hdlClockPreference
    | hdlClockHint
    | hdlClockNamedBlock
    ;


/*
 * ============================================================================
 * 31. CLOCK DOMAIN REFERENCE
 * ============================================================================
 */

hdlClockDomainReference
    : hdlClockReference
    ;


/*
 * ============================================================================
 * 32. CLOCK RELATIONSHIP REFERENCE
 * ============================================================================
 */

hdlClockRelationshipReference
    : hdlClockReference
    ;


/*
 * ============================================================================
 * 33. CLOCK SOURCE REFERENCE
 * ============================================================================
 */

hdlClockSourceReference
    : hdlClockReference
    ;


/*
 * ============================================================================
 * 34. CLOCK PROPERTY VALUE
 * ============================================================================
 *
 * Shared expression boundary.
 * ============================================================================
 */

hdlClockPropertyValue
    : hdlExpression
    ;


/*
 * ============================================================================
 * 35. CLOCK GENERATION BLOCK
 * ============================================================================
 *
 * This is the canonical open-world representation of generated-clock intent.
 *
 * Example:
 *
 *     clock derived {
 *         generated {
 *             source = source_clock;
 *             divide = divisor;
 *             multiply = multiplier;
 *             phase = phase_offset;
 *         }
 *     }
 *
 * Semantic analysis decides whether the requested relationship is realizable.
 * ============================================================================
 */

hdlClockGenerationBlock
    : identifier
      LBRACE
      hdlClockGenerationItem*
      RBRACE
    ;


hdlClockGenerationItem
    : hdlAttribute
    | hdlClockProperty
    | hdlClockNamedBlock
    ;


/*
 * ============================================================================
 * 36. CLOCK ENABLE / GATING INTENT
 * ============================================================================
 *
 * No implementation primitive is selected.
 *
 * Example:
 *
 *     clock gated {
 *         enable = enable_signal;
 *     }
 *
 * or:
 *
 *     clock gated {
 *         gating {
 *             enable = enable_signal;
 *         }
 *     }
 * ============================================================================
 */

hdlClockGatingBlock
    : identifier
      LBRACE
      hdlClockGatingItem*
      RBRACE
    ;


hdlClockGatingItem
    : hdlAttribute
    | hdlClockProperty
    | hdlClockNamedBlock
    ;


/*
 * ============================================================================
 * 37. CLOCK SYNCHRONIZATION INTENT
 * ============================================================================
 *
 * This expresses logical synchronization requirements.
 *
 * It does not prescribe synchronizer implementation.
 * ============================================================================
 */

hdlClockSynchronizationBlock
    : identifier
      LBRACE
      hdlClockSynchronizationItem*
      RBRACE
    ;


hdlClockSynchronizationItem
    : hdlAttribute
    | hdlClockProperty
    | hdlClockNamedBlock
    ;


/*
 * ============================================================================
 * 38. CLOCK PHASE RELATIONSHIP
 * ============================================================================
 */

hdlClockPhaseRelationship
    : identifier
      hdlClockReference
      hdlClockReference
      LBRACE
      hdlClockProperty*
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 39. CLOCK DOMAIN RELATIONSHIP
 * ============================================================================
 */

hdlClockDomainRelationship
    : identifier
      hdlClockReference
      hdlClockReference
      LBRACE
      hdlClockProperty*
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 40. CLOCK SOURCE RELATIONSHIP
 * ============================================================================
 */

hdlClockSourceRelationship
    : identifier
      hdlClockReference
      hdlClockReference
      LBRACE
      hdlClockProperty*
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 41. INTEGRATION CONTRACT
 * ============================================================================
 *
 * PARENT HDL GRAMMAR
 * ------------------
 *
 * grammar/hdl/hdl.g4 MUST:
 *
 *     1. import HdlClocks;
 *
 *     2. retain hdlClockDeclaration as the public clock entry rule;
 *
 *     3. remove its duplicate inline hdlClockDeclaration implementation;
 *
 *     4. remove its duplicate hdlClockReference implementation;
 *
 *     5. use the imported hdlClockDeclaration in hdlModuleMemberCore.
 *
 * Existing composition:
 *
 *     hdlModuleMemberCore
 *         ...
 *         | hdlClockDeclaration
 *         ...
 *
 * remains correct.
 *
 *
 * HARDWARE MODULES
 * ---------------
 *
 * grammar/hdl/hardware-modules.g4 may reference:
 *
 *     hdlClockDeclaration
 *
 * but must not redefine clock syntax.
 *
 *
 * REGISTERS / SEQUENTIAL LOGIC
 * ----------------------------
 *
 * grammar/hdl/registers.g4
 * grammar/hdl/sequential.g4
 *
 * may consume clock references/associations through:
 *
 *     hdlClockReference
 *
 * but clock declaration semantics remain owned here.
 *
 *
 * PROCESSES
 * ---------
 *
 * grammar/hdl/processes.g4 may refer to a clock by:
 *
 *     hdlClockReference
 *
 * but MUST NOT create another clock declaration grammar.
 *
 *
 * TIMING
 * ------
 *
 * grammar/hdl/timing.g4 owns timing constraints.
 *
 * It may refer semantically to:
 *
 *     hdlClockReference
 *     hdlClockDomainReference
 *
 * but MUST NOT redefine:
 *
 *     hdlClockDeclaration
 *     hdlClockProperty
 *
 *
 * HARDWARE
 * --------
 *
 * grammar/hardware/ owns target capability/resource realization.
 *
 * clocks.g4 MUST NOT import hardware target grammars.
 *
 *
 * QUANTUM
 * -------
 *
 * Quantum timing may consume semantic clock information downstream.
 *
 * clocks.g4 MUST NOT import quantum::ir or define quantum operations.
 *
 *
 * SCHEDULING
 * ----------
 *
 * Scheduling consumes the semantic clock model after parsing.
 *
 * No scheduler dependency may be introduced into this grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must expose enough structure for the domain-neutral AST to
 * represent, at minimum:
 *
 *     ClockDeclaration
 *         name
 *         optional type
 *         properties
 *         nested specifications
 *         attributes
 *         source span
 *
 *     ClockDomain
 *         name
 *         properties
 *         members
 *         source span
 *
 *     ClockReference
 *         path
 *         source span
 *
 *     ClockRelationship
 *         source
 *         target
 *         properties
 *         source span
 *
 * No AST node may contain:
 *
 *     FPGA primitive
 *     ASIC cell
 *     physical pin
 *     physical oscillator
 *     vendor clock primitive
 *
 * unless introduced later by downstream target-specific lowering.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - clock name resolution;
 *     - duplicate declaration detection;
 *     - property recognition;
 *     - property type validation;
 *     - quantity dimensionality;
 *     - period/frequency consistency;
 *     - phase validity;
 *     - duty-cycle validity;
 *     - jitter validity;
 *     - uncertainty validity;
 *     - divide/multiply validity;
 *     - source/parent resolution;
 *     - relationship-cycle detection;
 *     - domain consistency;
 *     - clock-domain-crossing analysis;
 *     - capability checking;
 *     - resource checking;
 *     - target feasibility;
 *     - conflict detection.
 *
 * The parser must not perform these checks.
 *
 * ============================================================================
 * OPEN-WORLD PROPERTY MODEL
 * ============================================================================
 *
 * Syntactic acceptance of a property does not imply semantic support.
 *
 * For example:
 *
 *     clock clk {
 *         future_clock_property = value;
 *     }
 *
 * may be syntactically represented.
 *
 * Semantic analysis determines whether:
 *
 *     future_clock_property
 *
 * is recognized by the active Zamani language version/dialect.
 *
 * Unknown properties MUST NOT silently acquire implementation behavior.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_CLOCKS
 *     MAX_CLOCK_DOMAINS
 *     MAX_CLOCK_SOURCES
 *     MAX_GENERATED_CLOCKS
 *     MAX_CLOCK_GROUPS
 *     MAX_CLOCK_FREQUENCY
 *     MAX_CLOCK_PERIOD
 *     MAX_JITTER
 *     MAX_PHASE
 *     MAX_DUTY_CYCLE
 *     MAX_CLOCK_TREE_DEPTH
 *     MAX_PLLS
 *     MAX_DLLS
 *     MAX_CLOCK_PINS
 *
 * Also forbidden:
 *
 *     FPGA-specific clock counts;
 *     ASIC-specific clock counts;
 *     vendor names;
 *     physical pin numbers;
 *     physical oscillator identifiers;
 *     fixed clock-tree structures;
 *     fixed machine sizes.
 *
 * Valid program data includes:
 *
 *     frequency = 100MHz;
 *     period = 10ns;
 *     divide = 2;
 *     multiply = 5;
 *
 * provided those values are program semantics rather than universal compiler
 * restrictions.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions;
 *     no semantic predicates;
 *     no random behavior;
 *     no hardware queries;
 *     no filesystem access;
 *     no network access;
 *     no wall-clock dependency.
 *
 * Parsing identical source with identical grammar/lexer versions must produce
 * equivalent parse structures.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Clock syntax cannot itself:
 *
 *     execute code;
 *     access hardware;
 *     access files;
 *     access networks;
 *     select credentials;
 *     select devices;
 *     alter runtime state.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Stable token:
 *
 *     K_CLOCK
 *
 * Stable public rule:
 *
 *     hdlClockDeclaration
 *
 * Existing source forms:
 *
 *     clock name;
 *     clock name { ... }
 *
 * must remain compatible.
 *
 * Adding a property should normally not require a new lexer keyword.
 *
 * Renaming K_CLOCK or changing the meaning of `clock` is a language-version
 * compatibility change and must be recorded under:
 *
 *     grammar/compatibility/
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     clock clk;
 *
 *     clock clk {
 *         period = 10ns;
 *     }
 *
 *     clock clk {
 *         frequency = 100MHz;
 *         edge = rising;
 *         polarity = active_high;
 *     }
 *
 *     clock clk {
 *         generated {
 *             source = source_clk;
 *             divide = 2;
 *             multiply = 1;
 *         }
 *     }
 *
 *     clock clk {
 *         gating {
 *             enable = enable_signal;
 *         }
 *     }
 *
 *     clock_domain control {
 *         clock = control_clk;
 *     }
 *
 *
 * NEGATIVE
 * --------
 *
 *     clock;
 *
 *     clock { }
 *
 *     clock 123;
 *
 *     clock clk {
 *         = 10ns;
 *     }
 *
 *     clock clk {
 *         period
 *     }
 *
 *
 * SCALABILITY
 * -----------
 *
 * Test:
 *
 *     many clock declarations;
 *     many properties;
 *     deep logical hierarchy;
 *     generated clock relationships;
 *     large clock-domain sets;
 *     symbolic timing expressions.
 *
 * No test may define a language-level maximum.
 *
 *
 * DETERMINISM
 * -----------
 *
 * Parse identical source repeatedly and verify equivalent:
 *
 *     token stream;
 *     parse structure;
 *     source spans;
 *     diagnostics.
 *
 *
 * CROSS-DOMAIN
 * ------------
 *
 * Verify integration with:
 *
 *     HDL modules;
 *     ports;
 *     signals;
 *     registers;
 *     sequential logic;
 *     processes;
 *     timing;
 *     hardware requirements;
 *     resources;
 *     distributed execution;
 *     quantum timing metadata.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *   [ ] K_CLOCK exists exactly once in the canonical lexer.
 *   [ ] K_CLOCK is owned by grammar/lexer/keywords.g4.
 *   [ ] This grammar uses tokenVocab = ZamaniLexer.
 *   [ ] Shared HDL rules use canonical names.
 *   [ ] No duplicate clock declaration exists in hdl.g4.
 *   [ ] No duplicate clock reference exists in hdl.g4.
 *   [ ] Timing grammar does not redefine clock syntax.
 *   [ ] Register/process grammars consume clock references rather than
 *       redefining clock declarations.
 *   [ ] Clock properties remain contextual identifiers.
 *   [ ] No vendor-specific clock syntax is embedded.
 *   [ ] No physical clock implementation is embedded.
 *   [ ] No finite clock/resource limits exist.
 *   [ ] No hardware topology is encoded.
 *   [ ] No Rust actions exist.
 *   [ ] No unsafe implementation is required.
 *   [ ] Source spans remain recoverable.
 *   [ ] AST mapping is defined.
 *   [ ] Semantic mapping is defined.
 *   [ ] Hardware/resource integration is downstream.
 *   [ ] Quantum integration remains downstream.
 *   [ ] Positive tests exist.
 *   [ ] Negative tests exist.
 *   [ ] Scalability tests exist.
 *   [ ] Determinism tests exist.
 *   [ ] Compatibility tests exist.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * A Zamani clock describes:
 *
 *     WHAT timing relationship is intended.
 *
 * It does not prescribe:
 *
 *     HOW a particular machine must implement it.
 *
 * Therefore:
 *
 *     clock syntax
 *         ->
 *     domain-neutral AST
 *         ->
 *     semantic clock model
 *         ->
 *     capability/resource analysis
 *         ->
 *     optimization
 *         ->
 *     scheduling
 *         ->
 *     synthesis/routing
 *         ->
 *     target realization
 *
 * This preserves POCO-REAF:
 *
 *     Program Once
 *         ->
 *     Compile Once
 *         ->
 *     Run Everywhere
 *         ->
 *     Run Anywhere
 *         ->
 *     Run Forever
 *
 * subject to the actual semantics and resources available at realization time.
 *
 * ============================================================================
 */