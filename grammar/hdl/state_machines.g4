/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/state_machines.g4
 *
 * Grammar:
 *     HdlStateMachines
 *
 * Status:
 *     CANONICAL HDL STATE-MACHINE SYNTAX DELEGATE
 *
 * Purpose:
 *     Define target-independent, parameterizable HDL state-machine syntax.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
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
 *     grammar/hdl/state_machines.g4
 *          |
 *          v
 *     grammar/hdl/hdl.g4
 *          |
 *          v
 *     grammar/Zamani.g4
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical HDL / hardware semantic model
 *          |
 *          v
 *     canonical IR
 *          |
 *          +--> optimization
 *          +--> scheduling
 *          +--> synthesis
 *          +--> verification
 *          +--> routing
 *          +--> target lowering
 *
 * This file owns STATE-MACHINE SOURCE SYNTAX ONLY.
 *
 * It MUST NOT become an independent language root.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - state-machine declaration syntax;
 *     - state-machine identity;
 *     - state-machine generic parameters;
 *     - state-machine header contracts;
 *     - state declarations;
 *     - state metadata;
 *     - state behavior blocks;
 *     - transition declarations;
 *     - transition guards;
 *     - transition actions;
 *     - default transitions;
 *     - state-machine-level properties;
 *     - state-machine-level requirements;
 *     - state-machine-level constraints;
 *     - state-machine-level preferences;
 *     - state-machine-level hints;
 *     - symbolic state references;
 *     - state-machine extensibility blocks.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical definitions;
 *     - identifiers;
 *     - names;
 *     - expressions;
 *     - expression precedence;
 *     - types;
 *     - attributes;
 *     - universal blocks;
 *     - statements;
 *     - registers;
 *     - clocks;
 *     - resets;
 *     - timing;
 *     - processes;
 *     - pipelines;
 *     - memories;
 *     - interfaces;
 *     - protocols;
 *     - synthesis;
 *     - scheduling;
 *     - routing;
 *     - placement;
 *     - physical state encoding;
 *     - hardware discovery;
 *     - target selection;
 *     - vendor implementations;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A state machine describes LOGICAL CONTROL BEHAVIOR.
 *
 * It does not describe a fixed implementation.
 *
 * The same source-level state machine may be realized as:
 *
 *     - combinational control;
 *     - sequential logic;
 *     - software control flow;
 *     - FPGA logic;
 *     - ASIC logic;
 *     - accelerator control;
 *     - CPU/GPU control structures;
 *     - hybrid classical/quantum control;
 *     - distributed control;
 *     - future hardware.
 *
 * The grammar MUST NOT encode:
 *
 *     MAX_STATES
 *     MAX_TRANSITIONS
 *     MAX_ACTIONS
 *     MAX_STATE_WIDTH
 *     MAX_STATE_MACHINES
 *     MAX_CLOCKS
 *     MAX_PIPELINE_STAGES
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * State machines are structurally unbounded.
 *
 * The grammar uses:
 *
 *     *
 *     +
 *
 * for arbitrary collections.
 *
 * There is no language-level maximum for:
 *
 *     state machines;
 *     generic parameters;
 *     states;
 *     transitions;
 *     guards;
 *     actions;
 *     properties;
 *     attributes;
 *     nested blocks;
 *     generated state machines;
 *     state-machine instances;
 *     transition metadata.
 *
 * "Infinity" means:
 *
 *     no artificial semantic ceiling is encoded by this grammar.
 *
 * Actual limits are determined by:
 *
 *     available resources;
 *     compiler resource policies;
 *     synthesis capacity;
 *     target capabilities;
 *     deployment constraints;
 *     runtime resources.
 *
 * Those limits MUST NOT be transformed into language-level syntax limits.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * The following are NOT source-level state-machine semantics:
 *
 *     binary state encoding;
 *     one-hot encoding;
 *     Gray encoding;
 *     physical register allocation;
 *     register width selected by a target;
 *     FPGA LUT assignment;
 *     FPGA region;
 *     ASIC cell;
 *     physical clock;
 *     physical reset network;
 *     physical placement;
 *     routing path.
 *
 * Such implementation decisions belong downstream.
 *
 * A source expression such as:
 *
 *     state idle;
 *
 * declares a logical state.
 *
 * It does NOT select a physical encoding.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * State-machine headers may express semantic intent using the repository's
 * canonical expression language.
 *
 * Examples:
 *
 *     requires capability("state-machine")
 *     requires capability("deterministic-control")
 *     constraint latency <= budget
 *     prefer encoding
 *     hint optimization(...)
 *
 * These constructs are source-level contracts.
 *
 * They do not select a physical device.
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * This parser consumes:
 *
 *     ZamaniLexer
 *
 * It MUST NOT consume:
 *
 *     ZamaniTokens
 *
 * directly.
 *
 * The repository's canonical lexical composition is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     ZamaniTokens
 *
 * Parser grammars consume the resulting:
 *
 *     ZamaniLexer
 *
 * token vocabulary.
 *
 * ============================================================================
 * SHARED GRAMMAR IMPORTS
 * ============================================================================
 *
 * This file reuses the canonical grammar authorities for:
 *
 *     Names
 *     Types
 *     Expressions
 *     Attributes
 *     Blocks
 *
 * It MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     argumentList
 *     typeExpression
 *     attribute
 *     block
 *     blockExpression
 *     statement
 *     expression precedence
 *
 * ============================================================================
 * CONTEXTUAL KEYWORDS
 * ============================================================================
 *
 * The current Zamani keyword vocabulary does not define dedicated reserved
 * tokens for:
 *
 *     state_machine
 *     state
 *     transition
 *     initial
 *     terminal
 *     default
 *     entry
 *     exit
 *
 * This file therefore deliberately does NOT invent lexer tokens for those
 * words.
 *
 * They are represented syntactically by identifiers and interpreted by
 * semantic validation at the state-machine boundary.
 *
 * This keeps the lexical vocabulary open and prevents state-machine concepts
 * from becoming globally reserved words unnecessarily.
 *
 * IMPORTANT:
 *
 * The canonical HDL composition layer must invoke these rules only where the
 * state-machine construct is expected.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar does not construct AST nodes.
 *
 * The frontend must preserve enough structure to represent at least:
 *
 *     HdlStateMachineDecl
 *     HdlStateMachineParameter
 *     HdlStateMachineProperty
 *     HdlStateDecl
 *     HdlStateBehavior
 *     HdlTransitionDecl
 *     HdlTransitionGuard
 *     HdlTransitionAction
 *     HdlDefaultTransition
 *     HdlStateReference
 *     HdlStateMachineBlock
 *
 * Every node must retain source-span information through the normal frontend
 * AST mechanism.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - state-machine name resolution;
 *     - state uniqueness;
 *     - transition source validation;
 *     - transition target validation;
 *     - initial-state validation;
 *     - terminal-state validation;
 *     - reachability;
 *     - dead-state analysis;
 *     - duplicate-transition analysis;
 *     - guard type checking;
 *     - action legality;
 *     - state behavior legality;
 *     - reset compatibility;
 *     - clock compatibility;
 *     - determinism analysis;
 *     - completeness analysis;
 *     - resource requirements;
 *     - capability requirements;
 *     - target realizability.
 *
 * The parser MUST NOT perform those checks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO state-machine-specific IR.
 *
 * Source:
 *
 *     state machine
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic state-machine model
 *          |
 *          v
 *     canonical HDL / hardware IR
 *
 * Physical state encoding and implementation are selected downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A state-machine action or guard may participate in hybrid control involving
 * quantum computation.
 *
 * This grammar does NOT define:
 *
 *     quantum gates;
 *     qubits;
 *     QPU topology;
 *     measurement implementation;
 *     QEC;
 *     ZQN;
 *     quantum routing;
 *     quantum scheduling.
 *
 * If a state-machine action invokes quantum semantics, the existing semantic
 * quantum pipeline remains authoritative:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization / routing / scheduling / QEC / ZQN / HAL
 *
 * No competing quantum IR is introduced here.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     language version;
 *     lexer vocabulary;
 *     imported grammar contracts;
 *
 * parsing must produce the same structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware;
 *     target availability;
 *     network state;
 *     runtime state;
 *     random values;
 *     wall-clock time;
 *     environment variables.
 *
 * ============================================================================
 * SAFE RUST
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic actions;
 *     - no predicates requiring unsafe behavior;
 *     - no filesystem access;
 *     - no network access;
 *     - no process execution.
 *
 * The compiler/frontend consuming it MUST use:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust
 *
 * and MUST NOT use `unsafe`.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PARSER GRAMMAR
 * ============================================================================
 */

parser grammar HdlStateMachines;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Types, Expressions, Attributes, Blocks;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Canonical state-machine declaration:
 *
 *     state_machine Controller {
 *         ...
 *     }
 *
 * The spelling `state_machine` remains contextual because it is represented
 * by the canonical identifier vocabulary rather than a dedicated global
 * keyword.
 *
 * ============================================================================
 */

hdlStateMachineDeclaration
    : hdlStateMachineKeyword
      identifier
      hdlStateMachineGenericParameters?
      hdlStateMachineHeaderClause*
      hdlStateMachineBody
    ;


/*
 * ============================================================================
 * 2. CONTEXTUAL STATE-MACHINE KEYWORD
 * ============================================================================
 *
 * This rule intentionally consumes an identifier.
 *
 * Semantic validation MUST require the identifier spelling associated with the
 * state-machine declaration in the active Zamani language version.
 *
 * This keeps the lexer open-world and avoids introducing a duplicate/global
 * keyword merely for this HDL domain.
 * ============================================================================
 */

hdlStateMachineKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 3. GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic parameters make state machines parameterizable without introducing
 * hardware-specific limits.
 *
 * Examples:
 *
 *     state_machine Controller<Width, Mode> { ... }
 *
 *     state_machine Controller<
 *         Width: UInt,
 *         Mode: ControlMode = DefaultMode
 *     > {
 *         ...
 *     }
 * ============================================================================
 */

hdlStateMachineGenericParameters
    : LT
      hdlStateMachineGenericParameterList
      GT
    ;

hdlStateMachineGenericParameterList
    : hdlStateMachineGenericParameter
      (
          COMMA
          hdlStateMachineGenericParameter
      )*
      COMMA?
    ;

hdlStateMachineGenericParameter
    : identifier
      hdlStateMachineGenericParameterType?
      hdlStateMachineGenericParameterDefault?
    ;

hdlStateMachineGenericParameterType
    : COLON
      typeExpression
    ;

hdlStateMachineGenericParameterDefault
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 4. HEADER CONTRACTS
 * ============================================================================
 *
 * These clauses describe semantic intent.
 *
 * They do not select a physical implementation.
 * ============================================================================
 */

hdlStateMachineHeaderClause
    : hdlStateMachineRequirement
    | hdlStateMachineConstraint
    | hdlStateMachinePreference
    | hdlStateMachineHint
    | hdlStateMachineProperty
    | hdlStateMachineAttribute
    ;

hdlStateMachineRequirement
    : REQUIRES
      expression
      SEMICOLON?
    ;

hdlStateMachineConstraint
    : CONSTRAINT
      expression
      SEMICOLON?
    ;

hdlStateMachinePreference
    : PREFER
      expression
      SEMICOLON?
    ;

hdlStateMachineHint
    : HINT
      expression
      SEMICOLON?
    ;

hdlStateMachineProperty
    : identifier
      COLON
      expression
      SEMICOLON
    ;

hdlStateMachineAttribute
    : attribute
    ;


/*
 * ============================================================================
 * 5. STATE-MACHINE BODY
 * ============================================================================
 */

hdlStateMachineBody
    : LBRACE
      hdlStateMachineItem*
      RBRACE
    ;

hdlStateMachineItem
    : hdlStateMachineItemAttribute*
      (
          hdlStateDeclaration
        | hdlTransitionDeclaration
        | hdlDefaultTransitionDeclaration
        | hdlStateMachineProperty
        | hdlStateMachineControlBlock
    )
    ;

hdlStateMachineItemAttribute
    : attribute
    ;


/*
 * ============================================================================
 * 6. STATE DECLARATIONS
 * ============================================================================
 *
 * Canonical:
 *
 *     state idle;
 *
 * Optional value/metadata:
 *
 *     state idle = value;
 *
 * Optional behavior:
 *
 *     state idle {
 *         entry { ... }
 *         exit  { ... }
 *     }
 *
 * The state value is semantic information.
 *
 * It MUST NOT be interpreted by this grammar as a physical state encoding.
 * ============================================================================
 */

hdlStateDeclaration
    : hdlStateKeyword
      identifier
      hdlStateAttribute*
      hdlStateInitializer?
      hdlStateBody?
      SEMICOLON?
    ;

hdlStateKeyword
    : identifier
    ;

hdlStateAttribute
    : attribute
    ;

hdlStateInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 7. STATE BODY
 * ============================================================================
 *
 * State bodies use the canonical block grammar.
 *
 * The state-machine grammar does not create another statement language.
 * ============================================================================
 */

hdlStateBody
    : LBRACE
      hdlStateBodyItem*
      RBRACE
    ;

hdlStateBodyItem
    : hdlStateBehavior
    | attribute
    ;

hdlStateBehavior
    : identifier
      (
          COLON
          expression
          SEMICOLON
        | LPAREN
          argumentList?
          RPAREN
          block
        | block
      )
    ;


/*
 * ============================================================================
 * 8. TRANSITIONS
 * ============================================================================
 *
 * Canonical:
 *
 *     transition idle -> running;
 *
 * Guard:
 *
 *     transition idle -> running : ready;
 *
 * Action block:
 *
 *     transition idle -> running : ready {
 *         start();
 *     }
 *
 * The source and target names are symbolic.
 *
 * ============================================================================
 */

hdlTransitionDeclaration
    : hdlTransitionKeyword
      hdlStateReference
      THIN_ARROW
      hdlStateReference
      hdlTransitionGuard?
      hdlTransitionAction?
      SEMICOLON?
    ;

hdlTransitionKeyword
    : identifier
    ;

hdlTransitionGuard
    : COLON
      expression
    ;

hdlTransitionAction
    : block
    ;


/*
 * ============================================================================
 * 9. DEFAULT TRANSITIONS
 * ============================================================================
 *
 * Canonical:
 *
 *     default -> recovery;
 *
 * Guarded:
 *
 *     default -> recovery : fault;
 *
 * Action:
 *
 *     default -> recovery {
 *         recover();
 *     }
 *
 * The semantic layer determines whether a default transition is legal and
 * whether it is complete/deterministic.
 * ============================================================================
 */

hdlDefaultTransitionDeclaration
    : hdlDefaultTransitionKeyword
      THIN_ARROW
      hdlStateReference
      hdlTransitionGuard?
      hdlTransitionAction?
      SEMICOLON?
    ;

hdlDefaultTransitionKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 10. STATE REFERENCES
 * ============================================================================
 *
 * State references are symbolic names.
 *
 * They are deliberately not numeric hardware encodings.
 * ============================================================================
 */

hdlStateReference
    : identifier
    ;


/*
 * ============================================================================
 * 11. EXTENSIBLE CONTROL BLOCKS
 * ============================================================================
 *
 * Named blocks permit future state-machine contracts without requiring this
 * grammar to enumerate every possible downstream analysis.
 *
 * Examples:
 *
 *     invariants { ... }
 *     properties { ... }
 *     verification { ... }
 *     timing { ... }
 *
 * Semantic ownership remains with the relevant subsystem.
 * ============================================================================
 */

hdlStateMachineControlBlock
    : identifier
      hdlStateMachineControlBlockArguments?
      block
    ;

hdlStateMachineControlBlockArguments
    : LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 12. REUSABLE COLLECTION RULES
 * ============================================================================
 *
 * These rules are intentionally unbounded.
 * ============================================================================
 */

hdlStateList
    : hdlStateDeclaration*
    ;

hdlTransitionList
    : hdlTransitionDeclaration*
    ;

hdlStateMachineItemList
    : hdlStateMachineItem*
    ;


/*
 * ============================================================================
 * 13. SEMANTICALLY DISTINCT REFERENCE GROUPS
 * ============================================================================
 *
 * These names give the semantic/AST layer stable integration points without
 * creating another IR.
 * ============================================================================
 */

hdlInitialStateReference
    : hdlStateReference
    ;

hdlResetStateReference
    : hdlStateReference
    ;

hdlTerminalStateReference
    : hdlStateReference
    ;


/*
 * ============================================================================
 * 14. COMPLETION CONTRACT
 * ============================================================================
 *
 * This grammar is complete when:
 *
 * [x] It retains the existing state_machines.g4 filename.
 *
 * [x] It has one canonical parser grammar name.
 *
 * [x] It consumes ZamaniLexer.
 *
 * [x] It imports canonical names/types/expressions/attributes/blocks.
 *
 * [x] It defines only state-machine syntax.
 *
 * [x] It does not define a second lexer.
 *
 * [x] It does not define a second expression grammar.
 *
 * [x] It does not define a second type grammar.
 *
 * [x] It does not define a second block grammar.
 *
 * [x] It has no machine-size limits.
 *
 * [x] It has no fixed state count.
 *
 * [x] It has no fixed transition count.
 *
 * [x] It has no fixed state width.
 *
 * [x] It has no fixed clock frequency.
 *
 * [x] It has no physical encoding semantics.
 *
 * [x] It supports parameterization.
 *
 * [x] It supports guards.
 *
 * [x] It supports actions.
 *
 * [x] It supports state behavior.
 *
 * [x] It supports default transitions.
 *
 * [x] It supports semantic requirements.
 *
 * [x] It supports constraints.
 *
 * [x] It supports preferences.
 *
 * [x] It supports hints.
 *
 * [x] It supports attributes.
 *
 * [x] It supports extensible semantic blocks.
 *
 * [x] It remains domain-neutral at the AST boundary.
 *
 * [x] It does not introduce a state-machine-specific IR.
 *
 * [x] It preserves the canonical quantum::ir boundary.
 *
 * [x] It contains no Rust actions.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It is compatible with the Rust 1.97 / 1.97.1 frontend baseline.
 *
 * [x] It defines integration responsibilities before downstream implementation.
 *
 * ============================================================================
 */