parser grammar combinational;

options {
    tokenVocab = ZamaniTokens;
}

/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/combinational.g4
 *
 * Role:
 *     Production HDL combinational-logic grammar.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust code.
 *     No unsafe Rust is required or permitted.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE SYNTAX for combinational hardware behavior.
 *
 * It describes hardware whose outputs are determined by its current inputs
 * and explicitly referenced state-free values.
 *
 * The grammar describes:
 *
 *     - combinational declarations;
 *     - combinational blocks;
 *     - combinational assignments;
 *     - conditional combinational logic;
 *     - case/selection logic;
 *     - combinational local declarations;
 *     - nested combinational regions;
 *     - logical assertions;
 *     - combinational expression statements;
 *     - generate constructs when supported by the surrounding HDL grammar;
 *     - attributes and source-level metadata.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - hdlCombinationalDeclaration;
 *     - hdlCombinationalBody;
 *     - hdlCombinationalStatement;
 *     - combinational-context assignment syntax;
 *     - combinational-context control-flow composition;
 *     - combinational-context local declarations;
 *     - combinational-context nested blocks;
 *     - combinational-context expression statements;
 *     - combinational-context assertions;
 *     - combinational source annotations.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - general expressions;
 *     - general types;
 *     - ordinary software control flow;
 *     - sequential logic;
 *     - clocks;
 *     - timing;
 *     - registers;
 *     - memories;
 *     - state machines;
 *     - processes;
 *     - wires;
 *     - ports;
 *     - hardware modules;
 *     - synthesis;
 *     - optimization;
 *     - scheduling;
 *     - placement;
 *     - routing;
 *     - physical resources;
 *     - target devices;
 *     - FPGA resources;
 *     - ASIC resources;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     HDL parser
 *          |
 *          +--> hardware-modules.g4
 *          +--> signals.g4
 *          +--> wires.g4
 *          +--> registers.g4
 *          +--> clocks.g4
 *          +--> timing.g4
 *          +--> THIS FILE
 *          +--> sequential.g4
 *          +--> processes.g4
 *          +--> state-machines.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> width/shape checking
 *          +--> driver analysis
 *          +--> combinational completeness
 *          +--> latch detection
 *          +--> dependency analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          |
 *          v
 *     canonical hardware semantic representation
 *          |
 *          +--> optimization
 *          +--> synthesis
 *          +--> scheduling
 *          +--> placement
 *          +--> routing
 *          |
 *          v
 *     target lowering
 *
 * Grammar establishes syntax.
 * Semantic analysis establishes meaning.
 * Hardware compilation establishes realization.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A combinational declaration expresses LOGICAL HARDWARE BEHAVIOR.
 *
 * It MUST NOT permanently encode:
 *
 *     - FPGA family;
 *     - ASIC family;
 *     - process node;
 *     - LUT count;
 *     - gate count;
 *     - DSP count;
 *     - BRAM count;
 *     - physical routing;
 *     - physical placement;
 *     - pin numbers;
 *     - physical addresses;
 *     - device identifiers;
 *     - fixed machine topology;
 *     - fixed hardware capacity.
 *
 * The same combinational semantic program must be capable of being lowered
 * into different implementation technologies when semantically valid.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately NO grammar-level finite limits.
 *
 * There is no:
 *
 *     MAX_INPUTS
 *     MAX_OUTPUTS
 *     MAX_ASSIGNMENTS
 *     MAX_BRANCHES
 *     MAX_CASE_ITEMS
 *     MAX_NESTING
 *     MAX_SIGNALS
 *     MAX_WIDTH
 *     MAX_BITS
 *     MAX_MODULES
 *     MAX_BLOCKS
 *
 * Repetition is represented through ANTLR *, + and ? operators.
 *
 * Resource limits belong to:
 *
 *     - compiler resource policies;
 *     - semantic analysis;
 *     - synthesis;
 *     - target capability analysis;
 *     - scheduling;
 *     - deployment;
 *     - runtime.
 *
 * ============================================================================
 * IMPORTANT SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The grammar MUST NOT decide whether a combinational design:
 *
 *     - fits a particular FPGA;
 *     - fits an ASIC;
 *     - has sufficient routing;
 *     - meets timing;
 *     - consumes a particular number of LUTs;
 *     - consumes a particular number of gates;
 *     - has a particular propagation delay.
 *
 * Those are downstream properties.
 *
 * The semantic layer MUST, however, detect source-level semantic violations
 * such as:
 *
 *     - invalid assignment targets;
 *     - incompatible types;
 *     - incompatible widths;
 *     - conflicting drivers;
 *     - incomplete conditional assignment where completeness is required;
 *     - unintended inferred storage;
 *     - illegal sequential constructs inside a combinational region;
 *     - invalid recursive combinational dependencies;
 *     - unsupported semantic constructs.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This grammar uses the canonical ZamaniTokens vocabulary.
 *
 * A dedicated:
 *
 *     K_COMBINATIONAL
 *
 * token MUST be added to the canonical lexer/token vocabulary.
 *
 * The parser must NOT use:
 *
 *     'combinational'
 *
 * as a parser literal because this is a separate parser grammar using
 * tokenVocab. ANTLR requires parser-visible literals/tokens to exist in the
 * imported vocabulary. 3
 *
 * ============================================================================
 */


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * A named combinational hardware region.
 *
 * Canonical form:
 *
 *     combinational {
 *         ...
 *     }
 *
 * Named form:
 *
 *     combinational logic_unit {
 *         ...
 *     }
 *
 * The optional name is a logical source-level identifier.
 *
 * It is NOT a physical block/device identifier.
 */
hdlCombinationalDeclaration
    : hdlCombinationalModifiers*
      K_COMBINATIONAL
      identifier?
      hdlCombinationalAttributes?
      hdlCombinationalBody
    ;


/* ============================================================================
 * DECLARATION MODIFIERS
 * ========================================================================== */

/*
 * Modifiers are deliberately represented through the existing HDL contextual
 * keyword mechanism rather than duplicating the language-wide lexer.
 *
 * Semantic validation determines which modifiers are legal.
 */
hdlCombinationalModifiers
    : hdlKeyword
    ;


/* ============================================================================
 * ATTRIBUTES
 * ========================================================================== */

hdlCombinationalAttributes
    : hdlAttribute+
    ;


/*
 * Combinational attributes use the repository's existing HDL attribute
 * contract.
 *
 * Examples:
 *
 *     @pure
 *     @parallel
 *     @dont_merge
 *     @keep
 *     @vendor(...)
 *
 * The grammar does NOT assign implementation meaning to these attributes.
 */
hdlCombinationalAttribute
    : AT
      identifier
      (
          LPAREN
          hdlCombinationalArgumentList?
          RPAREN
      )?
    ;


hdlCombinationalArgumentList
    : hdlCombinationalArgument
      (
          COMMA
          hdlCombinationalArgument
      )*
      COMMA?
    ;


hdlCombinationalArgument
    : identifier
      (
          ASSIGN
          hdlExpression
      )?
    | hdlExpression
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
 * A body may be empty syntactically so tooling can support incremental editing.
 *
 * Semantic validation MUST reject an empty combinational implementation where
 * the surrounding language construct requires actual behavior.
 */
hdlCombinationalStatement
    : hdlCombinationalDeclaration
    | hdlCombinationalAssignment
    | hdlCombinationalIf
    | hdlCombinationalCase
    | hdlCombinationalFor
    | hdlCombinationalLocalDeclaration
    | hdlCombinationalExpressionStatement
    | hdlCombinationalAssertion
    | hdlCombinationalBlock
    | hdlCombinationalGenerate
    ;


/* ============================================================================
 * ASSIGNMENT
 * ========================================================================== */

/*
 * Combinational assignment is intentionally restricted to ordinary assignment.
 *
 * Sequential/non-blocking assignment semantics do not belong here.
 *
 * If Zamani later introduces additional hardware assignment operators, they
 * must be added to the shared operator/token specification first and then
 * explicitly classified semantically.
 */
hdlCombinationalAssignment
    : hdlLValue
      ASSIGN
      hdlExpression
      SEMICOLON
    ;


/*
 * Explicit assignment statement with an optional annotation.
 */
hdlCombinationalAnnotatedAssignment
    : hdlCombinationalAttributes
      hdlCombinationalAssignment
    ;


/* ============================================================================
 * CONDITIONAL LOGIC
 * ========================================================================== */

/*
 * Conditional combinational logic.
 *
 * Canonical form:
 *
 *     if (condition) {
 *         ...
 *     } else {
 *         ...
 *     }
 *
 * The condition is a normal Zamani expression.
 *
 * Whether all outputs receive assignments on every control path is a semantic
 * question, not a parser question.
 */
hdlCombinationalIf
    : K_IF
      LPAREN
      hdlExpression
      RPAREN
      hdlCombinationalBody
      hdlCombinationalElseClause?
    ;


hdlCombinationalElseClause
    : K_ELSE
      (
          hdlCombinationalIf
        | hdlCombinationalBody
      )
    ;


/* ============================================================================
 * CASE / SELECTION LOGIC
 * ========================================================================== */

/*
 * Case-style combinational selection.
 *
 * The semantic layer determines:
 *
 *     - exhaustiveness;
 *     - duplicate patterns;
 *     - overlap;
 *     - default coverage;
 *     - constant-foldability;
 *     - implementation strategy.
 */
hdlCombinationalCase
    : K_CASE
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
    | K_DEFAULT
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
 * COMBINATIONAL ITERATION
 * ========================================================================== */

/*
 * Compile-time/static elaboration iteration may be legal inside an HDL
 * combinational region.
 *
 * Runtime-dependent unbounded iteration is NOT silently assumed to be
 * synthesizable.
 *
 * Semantic analysis decides whether the loop can represent finite hardware
 * elaboration or whether it violates the selected hardware semantic model.
 */
hdlCombinationalFor
    : K_FOR
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
 * LOCAL DECLARATIONS
 * ========================================================================== */

/*
 * Local combinational values are logical intermediate values.
 *
 * They do not imply:
 *
 *     - physical registers;
 *     - CPU registers;
 *     - FPGA registers;
 *     - memory;
 *     - persistent state.
 */
hdlCombinationalLocalDeclaration
    : hdlCombinationalLocalDeclarationNoTerminator
      SEMICOLON
    ;


hdlCombinationalLocalDeclarationNoTerminator
    : hdlKeyword
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


/* ============================================================================
 * EXPRESSION STATEMENTS
 * ========================================================================== */

/*
 * Expression statements are allowed for constructs whose semantic model
 * explicitly permits them.
 *
 * Semantic validation must reject expressions with side effects that are
 * incompatible with combinational semantics.
 */
hdlCombinationalExpressionStatement
    : hdlExpression
      SEMICOLON
    ;


/* ============================================================================
 * NESTED BLOCKS
 * ========================================================================== */

hdlCombinationalBlock
    : LBRACE
      hdlCombinationalStatement*
      RBRACE
    ;


/* ============================================================================
 * GENERATION
 * ========================================================================== */

/*
 * Generation is syntax-level composition.
 *
 * It does not mean a fixed number of generated hardware instances.
 *
 * Cardinality may be derived from:
 *
 *     - generics;
 *     - parameters;
 *     - compile-time expressions;
 *     - type-level values;
 *     - resource-independent semantic descriptions.
 *
 * Actual realizability is determined during elaboration/compilation.
 */
hdlCombinationalGenerate
    : hdlKeyword
      (
          hdlCombinationalGenerateBody
        | hdlExpression
        hdlCombinationalGenerateBody
      )
    ;


hdlCombinationalGenerateBody
    : hdlCombinationalBody
    ;


/* ============================================================================
 * ASSERTIONS
 * ========================================================================== */

/*
 * Assertions are retained as source-level semantic statements.
 *
 * They may later feed:
 *
 *     - formal verification;
 *     - simulation;
 *     - synthesis-time validation;
 *     - equivalence checking.
 *
 * They do not directly prescribe a hardware implementation.
 */
hdlCombinationalAssertion
    : hdlKeyword
      hdlExpression
      SEMICOLON
    ;


/* ============================================================================
 * EXPLICIT OUTPUT COVERAGE / DEFAULTING
 * ========================================================================== */

/*
 * A combinational construct may explicitly express a default assignment.
 *
 * This is represented through the ordinary assignment grammar rather than
 * introducing a special machine-dependent defaulting operation.
 *
 * Example:
 *
 *     y = default_value;
 *     if (condition) {
 *         y = alternate_value;
 *     }
 *
 * Semantic analysis determines whether every relevant output is assigned on
 * every control path.
 */


/* ============================================================================
 * FORBIDDEN-IN-COMBINATION SEMANTIC CONTRACT
 * ========================================================================== */

/*
 * The following constructs MUST NOT become valid merely because they happen
 * to be syntactically expressible through shared HDL rules:
 *
 *     - clock declarations as executable statements;
 *     - edge-triggered events;
 *     - non-blocking/sequential assignment;
 *     - explicit state storage;
 *     - reset operations;
 *     - latch declarations;
 *     - sequential process declarations;
 *     - physical placement;
 *     - physical routing;
 *     - target-specific timing directives.
 *
 * These are checked by semantic analysis after parsing.
 *
 * The parser intentionally remains syntax-oriented and does not duplicate
 * semantic ownership from sequential.g4, clocks.g4, timing.g4, or hardware/.
 */


/* ============================================================================
 * DEPENDENCY CONTRACT
 * ========================================================================== */

/*
 * This delegate grammar intentionally consumes shared rules supplied by the
 * canonical HDL grammar/import graph:
 *
 *     hdlKeyword
 *     identifier
 *     hdlExpression
 *     hdlTypeExpression
 *     hdlLValue
 *     hdlAttribute
 *
 * It MUST NOT redefine them here.
 *
 * This prevents:
 *
 *     - duplicate expression grammars;
 *     - duplicate identifier grammars;
 *     - duplicate type systems;
 *     - duplicate attribute systems;
 *     - incompatible AST interpretations.
 */


/* ============================================================================
 * END OF FILE
 * ============================================================================
 */