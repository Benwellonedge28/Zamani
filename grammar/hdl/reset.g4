/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/reset.g4
 *
 * Status:
 *     Canonical HDL reset parser delegate.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *     Safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the source-level syntax for HDL reset intent.
 *
 * It is deliberately a parser delegate rather than an independent HDL root.
 *
 * Ownership:
 *
 *     reset declarations
 *     reset properties
 *     reset clauses attached to sequential/storage constructs
 *     reset references
 *     reset expressions
 *     reset synchronization intent
 *     reset polarity intent
 *     reset deassertion/activation intent
 *     reset relationship metadata
 *
 * This file does NOT own:
 *
 *     clocks
 *     clock-domain definitions
 *     timing analysis
 *     physical reset pins
 *     reset trees
 *     reset buffers
 *     reset synchronizer implementation
 *     FPGA primitives
 *     ASIC cells
 *     vendor reset resources
 *     physical placement
 *     routing
 *     synthesis
 *     verification implementation
 *     runtime implementation
 *     hardware discovery
 *     resource allocation
 *
 * Those concerns belong to their respective downstream contracts.
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
 *     grammar/hdl/reset.g4
 *          |
 *          v
 *     grammar/hdl/hdl.g4
 *          |
 *          v
 *     grammar/Zamani.g4
 *          |
 *          v
 *     canonical lexer/parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical hardware semantic model / IR
 *          |
 *          v
 *     synthesis / verification / scheduling / routing / target lowering
 *
 * This file is not a competing grammar authority.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Reset syntax describes WHAT reset behavior is required.
 *
 * It must not prescribe WHICH physical resource implements that behavior.
 *
 * Valid source-level intent may include:
 *
 *     reset system_reset;
 *
 *     reset system_reset
 *         synchronous;
 *
 *     reset system_reset
 *         asynchronous;
 *
 *     reset system_reset
 *         active_high;
 *
 *     reset system_reset
 *         active_low;
 *
 *     reset system_reset
 *         synchronous
 *         active_low
 *         clock = clk;
 *
 * The actual implementation may use:
 *
 *     a reset pin;
 *     a generated reset;
 *     a clock-domain synchronizer;
 *     a reset tree;
 *     a vendor primitive;
 *     a synthesized network;
 *     a software-controlled reset;
 *     another target-specific mechanism.
 *
 * The grammar does not choose among them.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No language-level maximum is imposed on:
 *
 *     reset declarations
 *     reset domains
 *     reset consumers
 *     reset relationships
 *     reset groups
 *     reset properties
 *     modules
 *     hierarchy depth
 *     generated instances
 *     signal widths
 *     array dimensions
 *     hardware targets
 *
 * Repetition is represented structurally by ANTLR repetition operators.
 *
 * Resource exhaustion is an implementation concern, not a language rule.
 *
 * The grammar MUST NOT define:
 *
 *     MAX_RESETS
 *     MAX_RESET_DOMAINS
 *     MAX_RESET_FANOUT
 *     MAX_RESET_DEPTH
 *     MAX_MODULES
 *     MAX_SIGNALS
 *     MAX_WIDTH
 *     MAX_DEVICES
 *
 * ============================================================================
 * NO HARD-CODED HARDWARE ASSUMPTIONS
 * ============================================================================
 *
 * This file MUST NOT encode assumptions such as:
 *
 *     reset_width = 1
 *     reset_fanout <= 1024
 *     reset_delay <= fixed_value
 *     reset_clock <= fixed_frequency
 *     physical_reset_pin = 0
 *
 * A program MAY explicitly require such a value when it is part of program
 * semantics.
 *
 * For example:
 *
 *     requires fanout <= required_fanout;
 *
 * is semantic intent.
 *
 * It is NOT equivalent to making that number a language-wide limit.
 *
 * ============================================================================
 * CANONICAL TOKEN VOCABULARY
 * ============================================================================
 *
 * This grammar consumes the canonical Zamani lexer vocabulary.
 *
 * RESET is the canonical reset keyword already present in the repository's
 * lexical keyword contract.
 *
 * Other reset properties intentionally use identifiers instead of requiring
 * a growing collection of lexer keywords.
 *
 * Therefore:
 *
 *     synchronous
 *     asynchronous
 *     active_high
 *     active_low
 *     deassert
 *     synchronize
 *
 * remain semantic property names unless and until the language specification
 * promotes one of them to a reserved keyword.
 *
 * This avoids another lexer-vocabulary fork.
 *
 * ============================================================================
 * OPEN-WORLD PROPERTY MODEL
 * ============================================================================
 *
 * Reset semantics are extensible.
 *
 * A reset property consists of:
 *
 *     identifier
 *
 * or:
 *
 *     identifier = expression
 *
 * This permits future target-independent properties without changing the
 * parser for every new hardware technology.
 *
 * Known properties receive semantic validation downstream.
 *
 * Unknown properties are NOT automatically physical hardware features.
 *
 * A semantic validator may:
 *
 *     accept a standardized property;
 *     reject an unsupported property;
 *     route an explicitly enabled dialect property;
 *     report a version/compatibility diagnostic.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar maps to domain-neutral frontend AST structures.
 *
 * Conceptually:
 *
 *     hdlResetDeclaration
 *          |
 *          v
 *     declaration / hardware-intent node
 *          |
 *          v
 *     reset semantic model
 *
 * The grammar MUST NOT construct:
 *
 *     PhysicalResetPin
 *     ResetTree
 *     FPGAResetPrimitive
 *     ASICResetCell
 *     VendorResetController
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     reset-name uniqueness
 *     scope
 *     reset-domain identity
 *     polarity validation
 *     synchronous/asynchronous semantics
 *     clock relationship validation
 *     deassertion rules
 *     synchronization requirements
 *     reset fanout analysis
 *     reset sequencing
 *     reset dependency analysis
 *     CDC/reset-domain crossing validation
 *     target capability validation
 *
 * The parser only establishes structure.
 *
 * ============================================================================
 * CLOCK INTEGRATION
 * ============================================================================
 *
 * A reset may refer to a logical clock through a property:
 *
 *     clock = clk;
 *
 * The reset grammar does NOT define the clock grammar.
 *
 * Clock declarations remain owned by:
 *
 *     grammar/hdl/clocks.g4
 *
 * Clock-domain semantics remain downstream.
 *
 * This creates:
 *
 *     reset
 *       |
 *       +---- logical clock reference
 *                    |
 *                    v
 *                clock semantics
 *
 * rather than duplicating clock definitions inside reset.g4.
 *
 * ============================================================================
 * REGISTER / STORAGE INTEGRATION
 * ============================================================================
 *
 * Existing HDL constructs may attach reset intent through:
 *
 *     hdlResetClause
 *
 * For example, a register grammar may consume:
 *
 *     reset
 *
 * or:
 *
 *     reset(expression)
 *
 * The reset clause does not allocate or implement the reset.
 *
 * ============================================================================
 * SYNCHRONIZATION
 * ============================================================================
 *
 * Reset synchronization is represented as intent.
 *
 * Examples of semantic property names include:
 *
 *     synchronous
 *     asynchronous
 *     synchronize
 *     deassert_synchronously
 *
 * The grammar does not impose a fixed synchronizer depth.
 *
 * A program MAY explicitly express a parameter:
 *
 *     stages = n
 *
 * where n is program semantics.
 *
 * The compiler may later determine whether the requested behavior is
 * realizable on a target.
 *
 * ============================================================================
 * POLARITY
 * ============================================================================
 *
 * Polarity is source-level semantic intent.
 *
 * Examples:
 *
 *     active_high
 *     active_low
 *
 * No physical voltage level, logic-cell implementation, pin assignment, or
 * electrical standard is selected by this grammar.
 *
 * ============================================================================
 * RESET VALUE / INITIALIZATION
 * ============================================================================
 *
 * Reset behavior may specify an initialization expression:
 *
 *     value = expression
 *
 * The expression remains owned by the common HDL expression grammar.
 *
 * The reset grammar does not define the data type or width of the value.
 *
 * Semantic analysis determines whether the value is compatible with the
 * reset consumer.
 *
 * ============================================================================
 * RESET RELATIONSHIPS
 * ============================================================================
 *
 * Reset properties may describe relationships such as:
 *
 *     parent = system_reset;
 *     depends_on = power_good;
 *     clock = clk;
 *
 * These remain logical relationships.
 *
 * Physical dependency graphs are downstream semantic/implementation data.
 *
 * ============================================================================
 * REQUIREMENTS / CAPABILITIES
 * ============================================================================
 *
 * Reset declarations may carry ordinary repository attributes.
 *
 * Capability/resource requirements remain owned by the resource/hardware
 * semantic systems rather than becoming reset-specific parser logic.
 *
 * For example:
 *
 *     @requires(...)
 *
 * may be represented through the existing HDL attribute mechanism where
 * supported by the enclosing HDL grammar.
 *
 * ============================================================================
 * DIALECTS
 * ============================================================================
 *
 * Vendor- or technology-specific reset properties MUST NOT be added to the
 * core grammar merely because a particular target supports them.
 *
 * Such extensions belong in:
 *
 *     grammar/dialects/
 *
 * or the appropriate HDL dialect grammar.
 *
 * Core reset syntax remains target-independent.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics should identify the precise source span of:
 *
 *     reset declaration
 *     reset identifier
 *     reset property
 *     property value
 *     reset clause
 *     reset expression
 *
 * Semantic diagnostics may additionally identify:
 *
 *     invalid polarity
 *     conflicting reset modes
 *     invalid clock relationship
 *     unsupported synchronization requirement
 *     reset-domain crossing
 *     incompatible initialization value
 *     unsupported target capability
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar must parse a given token sequence deterministically according to
 * the canonical Zamani grammar contract.
 *
 * Property syntax is deliberately unambiguous:
 *
 *     identifier
 *
 * or:
 *
 *     identifier ASSIGN expression
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no actions;
 *     no semantic predicates;
 *     no unsafe code;
 *     no target-specific runtime calls.
 *
 * Rust implementations consuming this grammar must target:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * and must use safe Rust.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * Primary:
 *
 *     hdlResetDeclaration
 *
 * Supporting:
 *
 *     hdlResetProperty
 *     hdlResetClause
 *     hdlResetReference
 *     hdlResetPropertyName
 *     hdlResetPropertyValue
 *
 * ============================================================================
 */

parser grammar ZamaniHDLReset;

options {
    tokenVocab = ZamaniLexer;
}

import ZamaniHDLBase;

/*
 * ============================================================================
 * RESET DECLARATION
 *
 * Canonical form:
 *
 *     reset <identifier>
 *         [ : <type> ]
 *         <property>*
 *         ;
 *
 * Examples:
 *
 *     reset system_reset;
 *
 *     reset system_reset
 *         synchronous;
 *
 *     reset system_reset
 *         asynchronous
 *         active_low;
 *
 *     reset system_reset
 *         synchronous
 *         active_high
 *         clock = clk;
 *
 * No fixed reset count or hardware size is encoded.
 * ============================================================================
 */

hdlResetDeclaration
    : RESET
      identifier
      (
          COLON hdlTypeExpression
      )?
      hdlResetProperty*
      SEMICOLON
    ;

/*
 * ============================================================================
 * RESET PROPERTY
 *
 * A property is either:
 *
 *     name
 *
 * or:
 *
 *     name = value
 *
 * This is intentionally open-world.
 *
 * Known properties are validated semantically.
 * ============================================================================
 */

hdlResetProperty
    : hdlResetPropertyName
      (
          ASSIGN hdlResetPropertyValue
      )?
    ;

/*
 * ============================================================================
 * PROPERTY NAME
 * ============================================================================
 *
 * Property names remain identifiers instead of becoming an ever-growing list
 * of lexer keywords.
 *
 * Standard semantic property names include, but are not limited to:
 *
 *     synchronous
 *     asynchronous
 *     active_high
 *     active_low
 *     clock
 *     value
 *     synchronize
 *     deassert_synchronously
 *     parent
 *     depends_on
 *     domain
 *
 * The grammar intentionally does not enumerate those names.
 * ============================================================================
 */

hdlResetPropertyName
    : identifier
    ;

/*
 * ============================================================================
 * PROPERTY VALUE
 *
 * Values use the canonical HDL expression system.
 *
 * This permits:
 *
 *     clock = clk
 *     value = 0
 *     value = initial_state
 *     stages = n
 *     delay = reset_delay
 *
 * without imposing machine-dependent limits.
 * ============================================================================
 */

hdlResetPropertyValue
    : hdlExpression
    ;

/*
 * ============================================================================
 * RESET CLAUSE
 *
 * Used by other HDL constructs such as registers or sequential declarations.
 *
 * Forms:
 *
 *     reset
 *
 *     reset(expression)
 *
 * The expression identifies logical reset behavior.
 *
 * It does not identify a physical pin.
 * ============================================================================
 */

hdlResetClause
    : RESET
      (
          LPAREN hdlExpression RPAREN
      )?
    ;

/*
 * ============================================================================
 * RESET REFERENCE
 *
 * Explicit reusable reference form.
 *
 * Example:
 *
 *     reset(system_reset)
 *
 * This remains a logical reference.
 * ============================================================================
 */

hdlResetReference
    : RESET
      LPAREN
      hdlExpression
      RPAREN
    ;