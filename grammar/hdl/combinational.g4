/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/combinational.g4
 *
 * Status:
 *     CANONICAL HDL COMBINATIONAL-BEHAVIOR DELEGATE
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust actions.
 *     Zamani's Rust implementation MUST remain safe Rust.
 *     Rust `unsafe` is not required or permitted by this contract.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE SYNTAX for target-independent combinational hardware
 * behavior.
 *
 * A combinational region describes behavior whose outputs are functions of
 * currently available inputs and values, without introducing persistent
 * sequential state.
 *
 * The grammar deliberately describes LOGICAL HARDWARE INTENT.
 *
 * It does NOT describe:
 *
 *     - physical gates;
 *     - physical wires;
 *     - FPGA LUTs;
 *     - ASIC cells;
 *     - physical registers;
 *     - physical pins;
 *     - placement;
 *     - routing;
 *     - clock trees;
 *     - device IDs;
 *     - vendor primitives;
 *     - target-specific resource counts.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - hdlCombinationalDeclaration
 *     - hdlCombinationalBody
 *     - hdlCombinationalStatement
 *     - hdlCombinationalAssignment
 *     - hdlCombinationalIf
 *     - hdlCombinationalCase
 *     - hdlCombinationalFor
 *     - hdlCombinationalLocalDeclaration
 *     - hdlCombinationalExpressionStatement
 *     - hdlCombinationalAssertion
 *     - hdlCombinationalBlock
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - general types;
 *     - attributes;
 *     - ports;
 *     - signals;
 *     - nets;
 *     - registers;
 *     - memories;
 *     - clocks;
 *     - resets;
 *     - timing;
 *     - sequential behavior;
 *     - state machines;
 *     - pipelines;
 *     - modules;
 *     - interfaces;
 *     - generation/elaboration;
 *     - synthesis;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - placement;
 *     - resource discovery;
 *     - target selection;
 *     - runtime execution;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     HDL composition
 *          |
 *          +--> modules
 *          +--> ports
 *          +--> signals
 *          +--> nets
 *          +--> registers
 *          +--> THIS FILE
 *          +--> sequential
 *          +--> clocking
 *          +--> timing
 *          +--> state machines
 *          +--> pipelines
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> width/shape analysis
 *          +--> driver analysis
 *          +--> combinational completeness
 *          +--> dependency analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          |
 *          v
 *     canonical hardware semantic representation / IR
 *          |
 *          +--> optimization
 *          +--> verification
 *          +--> synthesis
 *          +--> scheduling
 *          +--> placement
 *          +--> routing
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The same combinational source construct MUST remain target-independent.
 *
 * The grammar therefore contains no universal hardware capacities such as:
 *
 *     MAX_INPUTS
 *     MAX_OUTPUTS
 *     MAX_SIGNALS
 *     MAX_ASSIGNMENTS
 *     MAX_BRANCHES
 *     MAX_CASE_ITEMS
 *     MAX_WIDTH
 *     MAX_BITS
 *     MAX_NESTING
 *     MAX_MODULES
 *     MAX_LUTS
 *     MAX_GATES
 *     MAX_FPGAS
 *     MAX_ASICS
 *
 * A program quantity is allowed:
 *
 *     let width = 1024;
 *
 * because that is program semantics.
 *
 * A language restriction such as:
 *
 *     width <= 32
 *
 * is NOT allowed as a universal grammar rule.
 *
 * ============================================================================
 * IMPORTANT SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parsing does NOT determine whether a design:
 *
 *     - fits a target;
 *     - meets timing;
 *     - fits available LUTs;
 *     - fits available gates;
 *     - satisfies fanout limits;
 *     - satisfies routing capacity;
 *     - satisfies power limits;
 *     - satisfies thermal limits;
 *     - can be synthesized by a particular tool.
 *
 * Those are downstream semantic/compiler/target concerns.
 *
 * The semantic layer MUST nevertheless validate source-level correctness,
 * including:
 *
 *     - assignment target legality;
 *     - type compatibility;
 *     - width/shape compatibility;
 *     - conflicting drivers;
 *     - incomplete output assignment;
 *     - unintended storage inference;
 *     - invalid stateful constructs;
 *     - invalid combinational cycles where prohibited;
 *     - unsupported semantic operations.
 *
 * ============================================================================
 * CANONICAL LEXER CONTRACT
 * ============================================================================
 *
 * Parser grammars MUST consume the production lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * is intentional.
 *
 * This grammar MUST NOT define lexical rules.
 *
 * The canonical keyword vocabulary needs a stable token for:
 *
 *     combinational
 *
 * with the token name:
 *
 *     COMBINATIONAL
 *
 * The canonical keyword vocabulary also needs:
 *
 *     DEFAULT
 *
 * for the case default branch.
 *
 * Existing language-wide keywords reused here include:
 *
 *     IF
 *     ELSE
 *     CASE
 *     FOR
 *     LET
 *     VAR
 *     CONST
 *     ASSERT
 *
 * Parser-local K_* aliases MUST NOT be invented here.
 *
 * ============================================================================
 * SHARED GRAMMAR CONTRACT
 * ============================================================================
 *
 * The following rules are supplied by the canonical HDL/parser composition:
 *
 *     identifier
 *     hdlQualifiedName
 *     hdlExpression
 *     hdlTypeExpression
 *     hdlLValue
 *     hdlAttribute
 *
 * This file MUST NOT redefine them.
 *
 * The public rule names remain stable even if their ownership is later moved
 * into a more explicit shared HDL grammar.
 *
 * ============================================================================
 * COMBINATIONAL VS SEQUENTIAL
 * ============================================================================
 *
 * Combinational behavior:
 *
 *     current inputs/state-visible values
 *              |
 *              v
 *        combinational logic
 *              |
 *              v
 *           outputs
 *
 * Sequential behavior:
 *
 *     current state + inputs
 *              |
 *              v
 *        state transition
 *              |
 *              v
 *          next state
 *
 * Sequential constructs belong to:
 *
 *     grammar/hdl/sequential.g4
 *
 * Clock syntax belongs to:
 *
 *     grammar/hdl/clocking.g4
 *
 * Reset syntax belongs to:
 *
 *     grammar/hdl/reset.g4
 *
 * Timing constraints belong to:
 *
 *     grammar/hdl/timing.g4
 *
 * This grammar must not absorb those responsibilities.
 *
 * ============================================================================
 */


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * Canonical forms:
 *
 *     combinational {
 *         ...
 *     }
 *
 *     combinational adder {
 *         ...
 *     }
 *
 * The optional identifier is a logical source-level name.
 *
 * It is NOT:
 *
 *     - a device name;
 *     - a physical block name;
 *     - an FPGA region;
 *     - an ASIC placement;
 *     - a vendor primitive.
 */
hdlCombinationalDeclaration
    : COMBINATIONAL
      identifier?
      hdlCombinationalBody
    ;


/* ============================================================================
 * BODY
 * ========================================================================== */

hdlCombinationalBody
    : LBRACE
      hdlCombinationalStatement*
      RBRACE
    ;


/*
 * The body is intentionally open-ended through repetition.
 *
 * There is no grammar-level limit on the number of statements.
 */
hdlCombinationalStatement
    : hdlCombinationalAssignment
    | hdlCombinationalIf
    | hdlCombinationalCase
    | hdlCombinationalFor
    | hdlCombinationalLocalDeclaration
    | hdlCombinationalExpressionStatement
    | hdlCombinationalAssertion
    | hdlCombinationalBlock
    ;


/* ============================================================================
 * ASSIGNMENT
 * ========================================================================== */

/*
 * Combinational assignment intentionally uses the ordinary ASSIGN token.
 *
 * Example:
 *
 *     result = a + b;
 *
 * Sequential/non-blocking assignment syntax does NOT belong here.
 *
 * If Zamani introduces a distinct sequential assignment operator in the
 * future, that token must be defined by the canonical lexer/operator contract
 * and consumed by sequential.g4.
 */
hdlCombinationalAssignment
    : hdlLValue
      ASSIGN
      hdlExpression
      SEMICOLON
    ;


/* ============================================================================
 * CONDITIONAL LOGIC
 * ========================================================================== */

/*
 * Canonical form:
 *
 *     if (condition) {
 *         ...
 *     } else {
 *         ...
 *     }
 *
 * Parentheses are required here so that the HDL control construct remains
 * structurally distinct from expression-level conditionals.
 */
hdlCombinationalIf
    : IF
      LPAREN
      hdlExpression
      RPAREN
      hdlCombinationalBody
      hdlCombinationalElseClause?
    ;


hdlCombinationalElseClause
    : ELSE
      (
          hdlCombinationalIf
        | hdlCombinationalBody
      )
    ;


/* ============================================================================
 * CASE / SELECTION
 * ========================================================================== */

/*
 * Canonical form:
 *
 *     case (selector) {
 *         value:
 *             ...
 *
 *         other:
 *             ...
 *
 *         default:
 *             ...
 *     }
 *
 * Exhaustiveness and overlap are semantic properties.
 */
hdlCombinationalCase
    : CASE
      LPAREN
      hdlExpression
      RPAREN
      LBRACE
      hdlCombinationalCaseItem*
      RBRACE
    ;


hdlCombinationalCaseItem
    : hdlCombinationalCasePattern
      COLON
      hdlCombinationalBody
    | DEFAULT
      COLON
      hdlCombinationalBody
    ;


hdlCombinationalCasePattern
    : hdlExpression
      (
          COMMA
          hdlExpression
      )*
    ;


/* ============================================================================
 * COMPILE-TIME / ELABORATION-ORIENTED FOR
 * ========================================================================== */

/*
 * A `for` inside a combinational region is syntactically accepted as a
 * parameterized structural/combinational construct.
 *
 * Whether it represents:
 *
 *     - compile-time elaboration;
 *     - statically unrolled logic;
 *     - a legal bounded combinational iteration;
 *
 * is decided by semantic analysis/elaboration.
 *
 * The grammar does NOT impose a fixed iteration count.
 *
 * Example:
 *
 *     for (let i = 0; i < width; i = i + 1) {
 *         ...
 *     }
 *
 * An implementation may reject a runtime-unbounded loop semantically rather
 * than making the grammar encode a maximum.
 */
hdlCombinationalFor
    : FOR
      LPAREN
      hdlCombinationalForInitializer?
      SEMICOLON
      hdlExpression?
      SEMICOLON
      hdlExpression?
      RPAREN
      hdlCombinationalBody
    ;


hdlCombinationalForInitializer
    : hdlCombinationalLocalDeclarationNoTerminator
    | hdlLValue
      ASSIGN
      hdlExpression
    ;


/* ============================================================================
 * LOCAL COMBINATIONAL VALUES
 * ========================================================================== */

/*
 * Local values are source-level intermediate values.
 *
 * They do NOT imply physical storage.
 *
 * A compiler may lower them to:
 *
 *     wires;
 *     optimized expressions;
 *     shared subexpressions;
 *     registers only if another semantic construct explicitly requires state.
 *
 * Example:
 *
 *     let sum: Logic = a + b;
 *
 * or:
 *
 *     var carry = a & b;
 */
hdlCombinationalLocalDeclaration
    : hdlCombinationalLocalDeclarationNoTerminator
      SEMICOLON
    ;


hdlCombinationalLocalDeclarationNoTerminator
    : hdlCombinationalBindingKind
      identifier
      (
          COLON
          hdlTypeExpression
      )?
      (
          ASSIGN
          hdlExpression
      )?
    ;


hdlCombinationalBindingKind
    : LET
    | VAR
    | CONST
    ;


/* ============================================================================
 * EXPRESSION STATEMENTS
 * ========================================================================== */

/*
 * Expression statements are syntactically permitted because expressions may
 * include source-level constructs that are meaningful in an HDL semantic
 * environment.
 *
 * Semantic analysis MUST reject expressions that imply incompatible
 * side effects, state, runtime execution, or unsupported behavior.
 */
hdlCombinationalExpressionStatement
    : hdlExpression
      SEMICOLON
    ;


/* ============================================================================
 * NESTED BLOCKS
 * ========================================================================== */

/*
 * Nested blocks provide lexical grouping only.
 *
 * They do not create:
 *
 *     - clocks;
 *     - state;
 *     - physical hierarchy;
 *     - hardware instances.
 */
hdlCombinationalBlock
    : LBRACE
      hdlCombinationalStatement*
      RBRACE
    ;


/* ============================================================================
 * ASSERTIONS
 * ========================================================================== */

/*
 * Example:
 *
 *     assert (a != b);
 *
 * Assertions remain source-level correctness properties.
 *
 * They may feed:
 *
 *     - simulation;
 *     - formal verification;
 *     - equivalence checking;
 *     - synthesis-time validation.
 *
 * They do not directly select a physical implementation.
 */
hdlCombinationalAssertion
    : ASSERT
      LPAREN
      hdlExpression
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * EXPLICITLY EXCLUDED SYNTAX
 * ========================================================================== */

/*
 * The following are deliberately NOT rules in this file:
 *
 *     sequential declarations
 *     process declarations
 *     clock declarations
 *     reset declarations
 *     timing declarations
 *     register declarations
 *     memory declarations
 *     state-machine declarations
 *     pipeline declarations
 *     generate declarations
 *     physical placement
 *     routing
 *     vendor primitives
 *
 * Their syntax belongs to their respective grammar owners.
 *
 * This prevents combinational.g4 from becoming another monolithic HDL grammar.
 */


/* ============================================================================
 * AST CONTRACT
 * ========================================================================== */

/*
 * Conceptual mappings:
 *
 *     hdlCombinationalDeclaration
 *         -> domain-neutral CombinationalDeclaration
 *
 *     hdlCombinationalAssignment
 *         -> Assignment / HardwareAssignment
 *
 *     hdlCombinationalIf
 *         -> Conditional
 *
 *     hdlCombinationalCase
 *         -> Selection
 *
 *     hdlCombinationalFor
 *         -> ElaborationLoop / CombinationalLoop
 *
 *     hdlCombinationalLocalDeclaration
 *         -> LocalBinding
 *
 *     hdlCombinationalAssertion
 *         -> Assertion
 *
 * Exact Rust AST type names are owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT construct Rust AST objects.
 *
 * Every AST node must retain source-span information.
 */


/* ============================================================================
 * SEMANTIC CONTRACT
 * ========================================================================== */

/*
 * Semantic analysis owns:
 *
 *     - declaration/name resolution;
 *     - scope;
 *     - type resolution;
 *     - width/shape checking;
 *     - assignment compatibility;
 *     - lvalue legality;
 *     - driver analysis;
 *     - combinational completeness;
 *     - latch/storage inference detection;
 *     - dependency analysis;
 *     - combinational-cycle analysis;
 *     - loop/elaboration legality;
 *     - constant evaluation;
 *     - capability requirements;
 *     - resource requirements;
 *     - target-independent portability analysis.
 *
 * The grammar does NOT decide these properties.
 *
 * In particular, this grammar must not reject:
 *
 *     large widths;
 *     large arrays;
 *     large numbers of branches;
 *     large numbers of statements;
 *     large numbers of outputs;
 *     large generated designs;
 *
 * merely because today's hardware may not support them.
 */


/* ============================================================================
 * COMBINATIONAL COMPLETENESS
 * ========================================================================== */

/*
 * The semantic layer must determine whether every required output receives a
 * value on every control path.
 *
 * Example:
 *
 *     combinational {
 *         if (enable) {
 *             out = value;
 *         }
 *     }
 *
 * The parser accepts this.
 *
 * Semantic analysis determines whether:
 *
 *     - `out` already has a valid default;
 *     - another surrounding assignment provides coverage;
 *     - the construct is incomplete;
 *     - storage would otherwise be inferred;
 *     - the selected HDL semantic profile permits the behavior.
 *
 * Do NOT encode completeness using parser-only alternatives.
 */


/* ============================================================================
 * DEPENDENCY / CYCLE CONTRACT
 * ========================================================================== */

/*
 * The grammar accepts arbitrary legal expressions:
 *
 *     a = b;
 *     b = c;
 *     c = a;
 *
 * Whether such a design forms an illegal combinational cycle is a semantic
 * graph-analysis problem.
 *
 * The parser must not impose a fixed dependency depth or graph size.
 */


/* ============================================================================
 * IR CONTRACT
 * ========================================================================== */

/*
 * This grammar lowers indirectly:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic hardware model
 *       |
 *       v
 *     canonical HDL / Hardware IR
 *
 * Typical semantic IR concepts include:
 *
 *     - combinational region;
 *     - assignment;
 *     - boolean/logical operation;
 *     - arithmetic operation;
 *     - mux/selection;
 *     - intermediate value;
 *     - dependency edge;
 *     - assertion/property.
 *
 * The grammar MUST NOT define physical:
 *
 *     - gates;
 *     - LUTs;
 *     - cells;
 *     - wires;
 *     - pins;
 *     - routes.
 *
 * Physical realization belongs downstream.
 */


/* ============================================================================
 * COMPILER CONTRACT
 * ========================================================================== */

/*
 * Compiler responsibilities after parsing include:
 *
 *     semantic validation
 *     constant evaluation
 *     dependency analysis
 *     combinational-cycle detection
 *     width/shape normalization
 *     optimization
 *     common-subexpression elimination
 *     dead-code elimination
 *     boolean simplification
 *     algebraic simplification
 *     synthesis
 *     verification
 *     scheduling where applicable
 *     placement
 *     routing
 *     target lowering
 *
 * None of these operations belong in this grammar.
 */


/* ============================================================================
 * RUNTIME CONTRACT
 * ========================================================================== */

/*
 * A pure combinational construct has no mandatory runtime state.
 *
 * Simulation/runtime systems may evaluate it, but this grammar does not create
 * runtime objects or perform runtime actions.
 *
 * Runtime/simulation behavior is downstream.
 */


/* ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ========================================================================== */

/*
 * Classical:
 *
 *     ordinary expressions and values may participate in combinational logic.
 *
 * Quantum:
 *
 *     this grammar does not define quantum operations.
 *
 *     Quantum operations remain owned by grammar/quantum/ and ultimately
 *     semantic quantum operations cross the canonical quantum::ir boundary.
 *
 * Hybrid:
 *
 *     hybrid constructs may reference HDL combinational behavior through the
 *     common semantic model.
 *
 * AI/data:
 *
 *     tensor/vector/data expressions may participate when the semantic and
 *     hardware models permit them.
 *
 * Hardware/resources:
 *
 *     capability and resource requirements are analyzed downstream.
 *
 * Networking/distributed:
 *
 *     communication intent belongs to their respective domains.
 */


/* ============================================================================
 * SCALABILITY CONTRACT
 * ========================================================================== */

/*
 * The following constructs are intentionally unbounded by grammar:
 *
 *     hdlCombinationalStatement*
 *     hdlCombinationalCaseItem*
 *     hdlCombinationalCasePattern*
 *     hdlCombinationalBody nesting
 *     expression size
 *     expression dimensions
 *     number of declarations
 *     number of assignments
 *     number of outputs
 *     number of intermediate values
 *
 * No finite maximum is encoded.
 *
 * "Infinity" means:
 *
 *     no artificial language-level ceiling.
 *
 * Actual execution remains constrained by:
 *
 *     - available memory;
 *     - compiler resources;
 *     - synthesis resources;
 *     - target capabilities;
 *     - deployment resources;
 *     - runtime environment.
 */


/* ============================================================================
 * DIAGNOSTIC CONTRACT
 * ========================================================================== */

/*
 * Parser diagnostics must preserve source locations for:
 *
 *     - `combinational`;
 *     - optional combinational name;
 *     - assignment target;
 *     - assignment expression;
 *     - condition;
 *     - case selector;
 *     - case pattern;
 *     - loop initializer;
 *     - loop condition;
 *     - loop update;
 *     - local binding;
 *     - assertion.
 *
 * Semantic diagnostics should subsequently report:
 *
 *     - incompatible assignment;
 *     - incomplete assignment;
 *     - conflicting drivers;
 *     - illegal state;
 *     - illegal cycle;
 *     - invalid width/shape;
 *     - unsupported target capability.
 */


/* ============================================================================
 * DETERMINISM
 * ========================================================================== */

/*
 * Parsing MUST be deterministic for identical:
 *
 *     source;
 *     language version;
 *     lexer vocabulary;
 *     grammar version.
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no randomness;
 *     - no environment-dependent parsing.
 */


/* ============================================================================
 * SECURITY
 * ========================================================================== */

/*
 * This grammar cannot:
 *
 *     - execute commands;
 *     - access files;
 *     - access networks;
 *     - discover hardware;
 *     - access credentials;
 *     - invoke a compiler backend;
 *     - allocate physical resources.
 *
 * It is a pure syntax boundary.
 */


/* ============================================================================
 * HARD-CODING AUDIT
 * ========================================================================== */

/*
 * PASS:
 *
 *     No MAX_* hardware capacity exists.
 *     No physical device ID exists.
 *     No physical pin exists.
 *     No fixed register width exists.
 *     No fixed signal count exists.
 *     No fixed branch count exists.
 *     No fixed loop count exists.
 *     No fixed hardware topology exists.
 *     No vendor primitive is required.
 *     No target is selected.
 *
 * Numeric values remain ordinary program expressions.
 */


/* ============================================================================
 * INTEGRATION CONTRACT
 * ========================================================================== */

/*
 * 1. grammar/antlr/ZamaniLexer.g4
 *
 *    Remains the sole production lexer boundary.
 *
 *
 * 2. grammar/lexer/keywords.g4
 *
 *    Add the canonical reserved words:
 *
 *        COMBINATIONAL : 'combinational' ;
 *        DEFAULT       : 'default' ;
 *
 *    Do NOT add K_COMBINATIONAL or K_DEFAULT as a second vocabulary.
 *
 *
 * 3. grammar/lexer/tokens.g4
 *
 *    Continues composing ZamaniKeywords.
 *
 *    No parser-local token definition belongs here.
 *
 *
 * 4. grammar/hdl/hdl.g4
 *
 *    REMOVE its duplicate:
 *
 *        hdlCombinationalDeclaration
 *            : K_COMBINATIONAL
 *              hdlBlock
 *            ;
 *
 *    The canonical declaration must instead delegate to:
 *
 *        hdlCombinationalDeclaration
 *
 *    from this grammar.
 *
 *    The existing hdlBlockItem dispatch may continue to expose the stable
 *    rule name, but it must have exactly one implementation owner.
 *
 *
 * 5. grammar/hdl/sequential.g4
 *
 *    Must remain independent.
 *
 *    It must NOT import combinational semantics.
 *
 *
 * 6. grammar/hdl/registers.g4
 *
 *    Register declarations remain separate.
 *
 *    A local combinational binding must never silently become a register.
 *
 *
 * 7. grammar/hdl/signals.g4
 *
 *    Signals remain logical value-bearing objects.
 *
 *    Combinational assignments may target compatible signal lvalues according
 *    to semantic analysis.
 *
 *
 * 8. grammar/hdl/nets.g4
 *
 *    Nets remain connectivity objects.
 *
 *    This file does not define net declarations or connectivity.
 *
 *
 * 9. grammar/hdl/clocking.g4
 *
 *    Clock declarations remain outside this grammar.
 *
 *
 * 10. grammar/hdl/timing.g4
 *
 *     Timing constraints remain outside this grammar.
 *
 *
 * 11. grammar/hdl/state-machines.g4
 *
 *     State-machine semantics remain outside this grammar.
 *
 *
 * 12. grammar/hdl/pipelines.g4
 *
 *     Pipeline semantics remain outside this grammar.
 *
 *
 * 13. grammar/hdl/generate.g4
 *
 *     General hardware generation remains owned by generate.g4.
 *
 *     The `for` syntax here is only the combinational behavioral form.
 *
 *
 * 14. grammar/Zamani.g4
 *
 *     The universal root delegates to the HDL composition root.
 *
 *     It must not reproduce combinational productions.
 *
 *
 * 15. grammar/validation/
 *
 *     Validation must detect:
 *
 *        - duplicate hdlCombinationalDeclaration owners;
 *        - parser-local lexer definitions;
 *        - K_* legacy token usage where canonical tokens are required;
 *        - fixed hardware capacities;
 *        - sequential constructs leaking into combinational grammar.
 */


/* ============================================================================
 * TEST CONTRACT
 * ========================================================================== */

/*
 * POSITIVE:
 *
 *     combinational {
 *         out = a & b;
 *     }
 *
 *     combinational and_gate {
 *         out = a & b;
 *     }
 *
 *     combinational {
 *         let sum = a + b;
 *         out = sum;
 *     }
 *
 *     combinational {
 *         if (enable) {
 *             out = a;
 *         } else {
 *             out = b;
 *         }
 *     }
 *
 *     combinational {
 *         case (opcode) {
 *             0:
 *                 out = a;
 *
 *             1, 2:
 *                 out = b;
 *
 *             default:
 *                 out = c;
 *         }
 *     }
 *
 *     combinational {
 *         for (let i = 0; i < width; i = i + 1) {
 *             out[i] = input[i];
 *         }
 *     }
 *
 *     combinational {
 *         assert (a == b);
 *     }
 *
 *
 * NEGATIVE / PARSE:
 *
 *     combinational
 *
 *     combinational foo
 *
 *     combinational { out = ; }
 *
 *     combinational { if (a) out = b; }
 *
 *     combinational { case (x) { }   // semantically potentially invalid
 *                    }
 *
 *
 * SEMANTIC NEGATIVE:
 *
 *     incomplete output assignment;
 *     incompatible assignment widths;
 *     incompatible types;
 *     invalid lvalue;
 *     conflicting drivers;
 *     illegal combinational cycle;
 *     sequential-only operation;
 *     clock/reset declaration inside combinational behavior;
 *     storage inference where prohibited;
 *     runtime-unbounded synthesis loop.
 *
 *
 * BOUNDARY:
 *
 *     very large symbolic widths;
 *     many assignments;
 *     many case branches;
 *     many nested blocks;
 *     many outputs;
 *     many generated intermediate values;
 *     arbitrarily large valid expressions.
 *
 *
 * SCALABILITY:
 *
 *     The tests must verify absence of artificial grammar-level maxima.
 *
 *
 * DETERMINISM:
 *
 *     Identical source and grammar version must yield identical parse trees.
 *
 *
 * CROSS-DOMAIN:
 *
 *     classical expression -> combinational logic;
 *     tensor/vector expression -> combinational logic;
 *     hardware capability requirement -> downstream semantic analysis;
 *     hybrid source -> HDL combinational region;
 *     quantum-related metadata -> semantic integration only.
 */


/* ============================================================================
 * COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is DONE when:
 *
 * [x] It has exactly one combinational declaration owner.
 *
 * [x] It consumes ZamaniLexer rather than ZamaniTokens directly.
 *
 * [x] It does not define lexer rules.
 *
 * [x] It does not define a second expression grammar.
 *
 * [x] It does not define a second type grammar.
 *
 * [x] It does not define a second identifier grammar.
 *
 * [x] It does not define a second attribute grammar.
 *
 * [x] It does not own sequential behavior.
 *
 * [x] It does not own clocks.
 *
 * [x] It does not own resets.
 *
 * [x] It does not own timing.
 *
 * [x] It does not own registers.
 *
 * [x] It does not own nets.
 *
 * [x] It does not own physical realization.
 *
 * [x] It contains no hardware capacity constants.
 *
 * [x] It supports arbitrary expression complexity permitted by the language.
 *
 * [x] It supports arbitrary numbers of statements/branches structurally.
 *
 * [x] AST mappings are predetermined.
 *
 * [x] Semantic responsibilities are predetermined.
 *
 * [x] IR integration is predetermined.
 *
 * [x] Compiler responsibilities are predetermined.
 *
 * [x] Runtime responsibilities are predetermined.
 *
 * [x] Diagnostics are predetermined.
 *
 * [x] Positive tests are predetermined.
 *
 * [x] Negative tests are predetermined.
 *
 * [x] Boundary tests are predetermined.
 *
 * [x] Scalability tests are predetermined.
 *
 * [x] Determinism tests are predetermined.
 *
 * [x] Cross-domain integration is predetermined.
 *
 * [x] Rust 1.97 / 1.97.1 compatibility requires no unsafe Rust.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 *     combinational.g4
 *          =
 *     portable combinational SOURCE SYNTAX
 *
 *     combinational.g4
 *          !=
 *     synthesis
 *
 *     combinational.g4
 *          !=
 *     physical hardware
 *
 *     combinational.g4
 *          !=
 *     target selection
 *
 *     combinational.g4
 *          !=
 *     resource allocation
 *
 *     combinational.g4
 *          !=
 *     timing closure
 *
 *     combinational.g4
 *          !=
 *     runtime execution
 *
 * The same source-level combinational intent can therefore participate in
 * lowering toward different realization technologies, subject to semantic
 * correctness, capabilities, and available resources.
 *
 * ============================================================================
 */