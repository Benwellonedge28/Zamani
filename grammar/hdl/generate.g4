/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/generate.g4
 *
 * Grammar:
 *     HdlGenerate
 *
 * Status:
 *     CANONICAL / PRODUCTION HDL GENERATION GRAMMAR
 *
 * Purpose:
 *     Define target-independent, parameterized structural generation syntax
 *     for Zamani HDL.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     Rust 2021
 *
 * Safety:
 *     Action-free ANTLR parser grammar.
 *     No embedded Rust.
 *     No unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
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
 *     grammar/hdl/generate.g4
 *          |
 *          v
 *     grammar/hdl/hdl.g4
 *          |
 *          v
 *     grammar/antlr/ZamaniParser.g4
 *          |
 *          v
 *     grammar/Zamani.g4
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical hardware / HDL semantic representation
 *          |
 *          +--> elaboration
 *          +--> optimization
 *          +--> verification
 *          +--> scheduling
 *          +--> synthesis
 *          +--> placement
 *          +--> routing
 *          +--> target lowering
 *          |
 *          v
 *     target realization
 *
 * This file owns SOURCE-LEVEL GENERATION SYNTAX ONLY.
 *
 * It does NOT perform generation.
 *
 * ============================================================================
 * SINGLE AUTHORITY
 * ============================================================================
 *
 * This file is the sole owner of GENERAL HDL structural generation syntax.
 *
 * It owns:
 *
 *     hdlGenerateDeclaration
 *     hdlGenerateConstruct
 *     hdlGenerateFor
 *     hdlGenerateForInitializer
 *     hdlGenerateForCondition
 *     hdlGenerateForUpdate
 *     hdlGenerateIf
 *     hdlGenerateElse
 *     hdlGenerateBlock
 *     hdlGenerateBody
 *     hdlGenerateItem
 *
 * It does NOT own:
 *
 *     identifiers
 *     qualified names
 *     expressions
 *     types
 *     attributes
 *     assignments
 *     modules
 *     ports
 *     signals
 *     nets
 *     registers
 *     memories
 *     pipelines
 *     state machines
 *     clocks
 *     timing
 *     synthesis
 *     placement
 *     routing
 *     scheduling
 *     resource allocation
 *     target selection
 *     physical hardware
 *
 * Existing domain grammars remain owners of those constructs.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION
 * ============================================================================
 *
 * Generate syntax describes ELABORATION-TIME STRUCTURAL INTENT.
 *
 * It is different from ordinary procedural control flow.
 *
 * Ordinary:
 *
 *     if
 *     for
 *     while
 *     repeat
 *
 * describes runtime/procedural behavior where permitted by the enclosing
 * semantic context.
 *
 * Generate:
 *
 *     generate for
 *     generate if
 *     generate <named block>
 *
 * describes source-level structural expansion/elaboration.
 *
 * The grammar records the distinction.
 *
 * The semantic/elaboration layer decides whether the supplied expressions
 * are valid for structural generation.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Generate constructs are parameterized and target-independent.
 *
 * They may depend on:
 *
 *     parameters
 *     generics
 *     compile-time expressions
 *     symbolic dimensions
 *     type information
 *     capability information
 *     semantic constraints
 *
 * They MUST NOT select a physical target.
 *
 * A generate construct must not encode:
 *
 *     FPGA number
 *     ASIC number
 *     CPU number
 *     GPU number
 *     QPU number
 *     physical register
 *     physical memory block
 *     physical routing path
 *     physical placement
 *     vendor primitive
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar deliberately contains no universal finite limits.
 *
 * There is no:
 *
 *     MAX_GENERATES
 *     MAX_GENERATED_INSTANCES
 *     MAX_GENERATE_DEPTH
 *     MAX_ITERATIONS
 *     MAX_BRANCHES
 *     MAX_MODULES
 *     MAX_SIGNALS
 *     MAX_REGISTERS
 *     MAX_MEMORIES
 *     MAX_PIPELINE_STAGES
 *     MAX_WIDTH
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *
 * Repetition is represented with normal ANTLR repetition operators.
 *
 * A source expression such as:
 *
 *     N
 *
 * may represent any semantically valid quantity.
 *
 * A source expression such as:
 *
 *     1024
 *
 * is program data.
 *
 * It MUST NOT become a language-wide maximum.
 *
 * Actual elaboration limits may arise from:
 *
 *     available compiler memory;
 *     compiler execution policy;
 *     implementation safeguards;
 *     target capabilities;
 *     synthesis capacity;
 *     deployment resources.
 *
 * Those are implementation/resource concerns and are not encoded here.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE / REALIZATION
 * ============================================================================
 *
 * Generation syntax MUST preserve the distinction between:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     hint
 *     realization
 *
 * Example:
 *
 *     requires capability("parallel-structure")
 *
 * expresses source intent.
 *
 * It does not mean:
 *
 *     instantiate on FPGA X
 *
 * or:
 *
 *     allocate physical resources 0..N
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * The grammar does not enumerate:
 *
 *     vendor generators
 *     FPGA primitives
 *     ASIC cells
 *     GPU blocks
 *     QPU components
 *     accelerator families
 *
 * Future hardware families can therefore use the same generation model.
 *
 * ============================================================================
 * EXPRESSION REUSE
 * ============================================================================
 *
 * This file MUST reuse the HDL expression hierarchy supplied by the
 * composition grammar:
 *
 *     hdlExpression
 *
 * It MUST NOT define:
 *
 *     generateExpression
 *     generateArithmeticExpression
 *     generateConditionExpression
 *     generateConstantExpression
 *
 * as duplicate expression languages.
 *
 * Semantic analysis determines whether an expression is legal in an
 * elaboration-time position.
 *
 * ============================================================================
 * TYPE REUSE
 * ============================================================================
 *
 * This file reuses:
 *
 *     hdlTypeExpression
 *
 * where a generated binding explicitly declares a type.
 *
 * It does not create another hardware type system.
 *
 * ============================================================================
 * NAME REUSE
 * ============================================================================
 *
 * This file reuses:
 *
 *     identifier
 *     hdlQualifiedName
 *
 * from the existing HDL/name hierarchy.
 *
 * ============================================================================
 * ASSIGNMENT REUSE
 * ============================================================================
 *
 * Generate initializers and updates may use the existing:
 *
 *     hdlLValue
 *     hdlAssignmentOperator
 *
 * contracts.
 *
 * The terminating semicolon is deliberately NOT consumed by the assignment
 * form used inside a generate-for header.
 *
 * This avoids coupling generate syntax to the normal statement terminator.
 *
 * ============================================================================
 * STRUCTURAL BODY
 * ============================================================================
 *
 * A generate body is structurally different from an ordinary procedural
 * block.
 *
 * Therefore this file does not simply alias:
 *
 *     hdlGenerateBody : hdlBlock ;
 *
 * Instead it defines a structural generation body whose members reuse the
 * existing HDL module-member contract.
 *
 * This prevents generate constructs from accidentally acquiring an unrelated
 * procedural-only grammar contract.
 *
 * ============================================================================
 * NESTING
 * ============================================================================
 *
 * Generate constructs may be nested.
 *
 * For example:
 *
 *     generate for (...) {
 *         generate if (...) {
 *             ...
 *         }
 *     }
 *
 * The grammar does not impose a nesting depth.
 *
 * Semantic/elaboration safeguards may exist, but those safeguards MUST NOT
 * become source-language maxima.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing of generate syntax is deterministic with respect to:
 *
 *     source text
 *     grammar version
 *     lexical configuration
 *     explicitly selected dialects
 *
 * Generation itself MUST be deterministic for identical semantic input unless
 * a downstream semantic construct explicitly introduces nondeterminism.
 *
 * This grammar does not contain:
 *
 *     randomness
 *     timestamps
 *     environment inspection
 *     filesystem access
 *     network access
 *     hardware discovery
 *     runtime execution
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser produces ordinary parse-tree structure.
 *
 * The frontend AST must represent generate constructs as generic structural
 * generation semantics rather than vendor-specific generator objects.
 *
 * Recommended semantic mapping:
 *
 *     hdlGenerateFor
 *         -> structural generation / iteration node
 *
 *     hdlGenerateIf
 *         -> conditional structural generation node
 *
 *     hdlGenerateBlock
 *         -> structural generation scope/block node
 *
 *     hdlGenerateForInitializer
 *         -> generation binding/initialization
 *
 *     hdlGenerateForCondition
 *         -> generation condition
 *
 *     hdlGenerateForUpdate
 *         -> generation update expression
 *
 * The exact AST type is owned by the frontend AST contract.
 *
 * This grammar MUST NOT introduce a vendor-specific AST type.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether generation expressions are evaluable/valid;
 *     - whether a generate binding is well formed;
 *     - whether a generated name is legal;
 *     - whether generated declarations conflict;
 *     - whether generated connections are type-correct;
 *     - whether generated structures are valid;
 *     - whether generated resources are satisfiable;
 *     - whether expansion is finite where required;
 *     - whether symbolic generation can remain deferred;
 *     - whether the generated design is compatible with target capabilities.
 *
 * The parser MUST NOT attempt these checks.
 *
 * ============================================================================
 * ELABORATION CONTRACT
 * ============================================================================
 *
 * Generate syntax may describe:
 *
 *     explicitly finite generation;
 *     parameterized finite generation;
 *     symbolic generation;
 *     target-specialized generation;
 *     capability-dependent generation.
 *
 * The elaborator decides whether the semantic form can be materialized.
 *
 * The parser does not expand generated structures.
 *
 * Therefore:
 *
 *     source size != generated design size
 *
 * and no parser rule may assume they are equal.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * A generate loop such as:
 *
 *     generate for (let i = 0; i < count; i = i + 1) {
 *         ...
 *     }
 *
 * does not imply a fixed hardware resource count.
 *
 * `count` may be:
 *
 *     constant;
 *     parameterized;
 *     symbolic;
 *     capability-derived;
 *     target-specialized.
 *
 * Resource analysis occurs downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Generate constructs may generate hardware surrounding quantum computation.
 *
 * They MUST NOT:
 *
 *     enumerate physical qubits;
 *     encode a QPU topology;
 *     select a QPU;
 *     perform quantum routing;
 *     perform QEC;
 *     construct quantum::ir;
 *
 * If generated source contains quantum operations, those operations retain
 * their normal semantic path:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Generate may create:
 *
 *     arithmetic structures
 *     vector structures
 *     matrix structures
 *     data paths
 *     compute units
 *     control structures
 *
 * Their meaning remains owned by the corresponding semantic domains.
 *
 * ============================================================================
 * PIPELINE INTEGRATION
 * ============================================================================
 *
 * Pipeline-specific generation remains owned by:
 *
 *     grammar/hdl/pipelines.g4
 *
 * That grammar may consume the general generation contract where appropriate.
 *
 * Pipeline generation MUST NOT create a competing general generate grammar.
 *
 * ============================================================================
 * MODULE INTEGRATION
 * ============================================================================
 *
 * Module-level generation is structurally integrated through this grammar.
 *
 * The existing:
 *
 *     hdlModuleGenerate
 *     hdlModuleGenerateFor
 *     hdlModuleGenerateIf
 *     hdlModuleGenerateBlock
 *
 * forms in hardware-modules.g4 are legacy/duplicate ownership.
 *
 * They must not remain a second production implementation of the same general
 * generate semantics.
 *
 * Module composition should consume:
 *
 *     hdlGenerateDeclaration
 *
 * or an explicitly module-specific adapter whose only responsibility is
 * contextual placement.
 *
 * ============================================================================
 * MEMORY INTEGRATION
 * ============================================================================
 *
 * Generate constructs may create repeated logical memories.
 *
 * This grammar does not own memory declarations.
 *
 * Memory semantics remain owned by:
 *
 *     grammar/hdl/memories.g4
 *
 * No physical memory technology is implied.
 *
 * ============================================================================
 * SIGNAL / REGISTER INTEGRATION
 * ============================================================================
 *
 * Generated signals and registers continue to use:
 *
 *     grammar/hdl/signals.g4
 *     grammar/hdl/registers.g4
 *
 * No generated register width or count is hard-coded here.
 *
 * ============================================================================
 * ARRAY INTEGRATION
 * ============================================================================
 *
 * Array-valued expressions remain owned by:
 *
 *     grammar/hdl/arrays.g4
 *
 * Generate expressions consume:
 *
 *     hdlExpression
 *
 * and therefore may naturally contain array expressions where legal.
 *
 * This file does not duplicate array syntax.
 *
 * ============================================================================
 * PARAMETER INTEGRATION
 * ============================================================================
 *
 * Generate parameters are ordinary HDL expressions and parameter values.
 *
 * Parameter declarations remain owned by:
 *
 *     grammar/hdl/parameters.g4
 *
 * Generate syntax does not create a second parameter system.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware capabilities and target requirements remain downstream.
 *
 * Generate does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *
 * Hardware realization is determined after semantic analysis.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler pipeline should be:
 *
 *     parse
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     name/type/constraint analysis
 *       |
 *       v
 *     generation semantic analysis
 *       |
 *       v
 *     elaboration
 *       |
 *       v
 *     canonical hardware semantic representation / IR
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     scheduling / synthesis / routing / placement
 *       |
 *       v
 *     target lowering
 *
 * Generate elaboration MUST happen before target-specific realization.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Generate syntax is normally compile/elaboration-time structure.
 *
 * It does not itself execute at runtime.
 *
 * If generated constructs contain runtime behavior, only the generated
 * semantic constructs execute at runtime.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     embedded Rust
 *     unsafe
 *     semantic predicates
 *     filesystem operations
 *     network operations
 *     hardware operations
 *     target discovery
 *     runtime execution
 *
 * Rust 1.97 / 1.97.1 compatibility is therefore a downstream generated-parser
 * and frontend implementation requirement, not a grammar-language feature.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * The primary public integration rule is:
 *
 *     hdlGenerateDeclaration
 *
 * Supporting rules are public only because ANTLR grammar composition exposes
 * them to the generated parser.
 *
 * ============================================================================
 */

parser grammar HdlGenerate;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * GENERATION ENTRY
 * ============================================================================
 *
 * Examples:
 *
 *     generate for (let i = 0; i < count; i = i + 1) {
 *         ...
 *     }
 *
 *     generate if condition {
 *         ...
 *     }
 *
 *     generate if condition {
 *         ...
 *     } else {
 *         ...
 *     }
 *
 *     generate if condition {
 *         ...
 *     } else if other_condition {
 *         ...
 *     } else {
 *         ...
 *     }
 *
 *     generate {
 *         ...
 *     }
 *
 *     generate instances {
 *         ...
 *     }
 *
 * The grammar deliberately does not prescribe how many generated entities
 * result from any construct.
 */
hdlGenerateDeclaration
    : K_GENERATE
      hdlGenerateConstruct
    ;


/*
 * ============================================================================
 * GENERATION CONSTRUCT
 * ============================================================================
 */

hdlGenerateConstruct
    : hdlGenerateFor
    | hdlGenerateIf
    | hdlGenerateBlock
    ;


/*
 * ============================================================================
 * GENERATE-FOR
 * ============================================================================
 *
 * The three clauses are intentionally expression-driven.
 *
 * Initialization:
 *     establishes the generation binding/state.
 *
 * Condition:
 *     determines whether another generation step is semantically selected.
 *
 * Update:
 *     advances the generation binding/state.
 *
 * No iteration count is encoded in the grammar.
 */
hdlGenerateFor
    : K_FOR
      LPAREN
      hdlGenerateForInitializer
      SEMICOLON
      hdlGenerateForCondition
      SEMICOLON
      hdlGenerateForUpdate
      RPAREN
      hdlGenerateBody
    ;


/*
 * ============================================================================
 * GENERATE-FOR INITIALIZER
 * ============================================================================
 *
 * Reuses the existing HDL variable kinds and type/expression system.
 *
 * Examples:
 *
 *     let i = 0
 *     var i: uint = 0
 *     const i: uint = 0
 *     i = 0
 *
 * The initializer has no terminating semicolon because the enclosing
 * generate-for header owns that delimiter.
 */
hdlGenerateForInitializer
    : hdlVariableKind
      identifier
      (
          COLON
          hdlTypeExpression
      )?
      (
          ASSIGN
          hdlExpression
      )?
    | hdlLValue
      hdlAssignmentOperator
      hdlExpression
    | hdlExpression
    ;


/*
 * ============================================================================
 * GENERATE-FOR CONDITION
 * ============================================================================
 *
 * The condition is an ordinary HDL expression.
 *
 * Semantic analysis determines whether it is suitable for structural
 * elaboration.
 *
 * An empty condition is intentionally NOT accepted.
 *
 * This avoids silently introducing an unbounded elaboration loop.
 */
hdlGenerateForCondition
    : hdlExpression
    ;


/*
 * ============================================================================
 * GENERATE-FOR UPDATE
 * ============================================================================
 *
 * Examples:
 *
 *     i = i + 1
 *     i += 1
 *     advance(i)
 *     next_index(i)
 *
 * Whether a particular update expression is semantically suitable for
 * elaboration is decided downstream.
 */
hdlGenerateForUpdate
    : hdlLValue
      hdlAssignmentOperator
      hdlExpression
    | hdlExpression
    ;


/*
 * ============================================================================
 * GENERATE-IF
 * ============================================================================
 *
 * Supports:
 *
 *     if
 *     if / else
 *     if / else if / else
 *
 * without introducing a separate conditional-expression grammar.
 */
hdlGenerateIf
    : K_IF
      hdlExpression
      hdlGenerateBlock
      hdlGenerateElse?
    ;


hdlGenerateElse
    : K_ELSE
      (
          hdlGenerateIf
        | hdlGenerateBlock
      )
    ;


/*
 * ============================================================================
 * NAMED / UNNAMED GENERATE BLOCK
 * ============================================================================
 *
 * Both forms are legal:
 *
 *     generate {
 *         ...
 *     }
 *
 * and:
 *
 *     generate instances {
 *         ...
 *     }
 *
 * The optional identifier is a logical source-level name.
 *
 * It does not identify a physical hardware location.
 */
hdlGenerateBlock
    : identifier?
      hdlGenerateBody
    ;


/*
 * ============================================================================
 * STRUCTURAL GENERATE BODY
 * ============================================================================
 *
 * A generate body is a collection of HDL structural members.
 *
 * The body is intentionally unbounded.
 *
 * Existing HDL member syntax is reused rather than duplicated here.
 */
hdlGenerateBody
    : LBRACE
      hdlGenerateItem*
      RBRACE
    ;


/*
 * ============================================================================
 * GENERATE ITEM
 * ============================================================================
 *
 * The existing HDL module-member contract remains the source of truth for
 * concrete declarations.
 *
 * This allows generation of:
 *
 *     parameters
 *     types
 *     interfaces
 *     ports where context permits
 *     signals
 *     nets
 *     registers
 *     memories
 *     clocks
 *     resets
 *     assignments
 *     processes
 *     pipelines
 *     instances
 *     nested generate constructs
 *     assertions
 *     expression-bearing constructs
 *
 * Semantic analysis remains responsible for contextual legality.
 */
hdlGenerateItem
    : hdlAttribute*
      hdlModuleMemberCore
    ;