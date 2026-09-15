/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/metaprogramming/compile-time-execution.g4
 *
 * Grammar:
 *     CompileTimeExecution
 *
 * Status:
 *     Production parser-grammar composition unit
 *
 * Purpose:
 *     Defines the source-level syntax boundary for explicitly requested
 *     compile-time execution contexts.
 *
 * Architectural position:
 *
 *     Source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser / composed parser
 *       |
 *       v
 *     Frontend AST
 *       |
 *       v
 *     Semantic Analysis
 *       |
 *       +--> compile-time eligibility
 *       +--> effect checking
 *       +--> capability checking
 *       +--> dependency analysis
 *       +--> constant evaluation
 *       +--> partial evaluation
 *       +--> specialization
 *       +--> deterministic evaluation
 *       |
 *       v
 *     Canonical IR
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> hardware-independent IR
 *       +--> control/data IR
 *       |
 *       v
 *     optimization / lowering / routing / scheduling / HAL
 *       |
 *       v
 *     runtime
 *
 * ============================================================================
 * IMPORTANT ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file defines SYNTAX ONLY.
 *
 * It MUST NOT:
 *
 *   - execute compile-time code;
 *   - evaluate expressions;
 *   - perform constant folding;
 *   - perform specialization;
 *   - inspect the compiler host;
 *   - inspect hardware;
 *   - discover devices;
 *   - access arbitrary files;
 *   - access networks;
 *   - access credentials;
 *   - invoke external processes;
 *   - allocate runtime resources;
 *   - invoke a QPU;
 *   - invoke a GPU;
 *   - invoke an FPGA;
 *   - invoke an ASIC;
 *   - define quantum IR;
 *   - define classical IR;
 *   - define HDL IR;
 *   - perform optimization;
 *   - perform scheduling;
 *   - perform routing;
 *   - perform QEC;
 *   - define ZQN semantics;
 *   - define runtime dispatch;
 *   - define resource limits.
 *
 * Compile-time execution semantics belong to the compiler semantic/evaluation
 * subsystem after parsing.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the source-level compile-time execution context boundary;
 *   - explicit compile-time execution requests;
 *   - compile-time evaluation blocks;
 *   - compile-time sequencing boundaries;
 *   - compile-time value-production boundaries;
 *   - compile-time declaration-generation boundaries;
 *   - compile-time execution result boundaries;
 *   - explicit compile-time execution modifiers that are not already owned by
 *     the canonical function, expression, macro, or compilation grammars.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer tokens;
 *   - ordinary expressions;
 *   - expression precedence;
 *   - literals;
 *   - identifiers;
 *   - types;
 *   - ordinary statements;
 *   - ordinary blocks;
 *   - ordinary function declarations;
 *   - compile-time function declarations;
 *   - compile-time expression operators;
 *   - conditional compilation;
 *   - target selection;
 *   - optimization;
 *   - feature selection;
 *   - resource requirements;
 *   - hardware descriptions;
 *   - runtime execution;
 *   - macro declaration;
 *   - macro expansion;
 *   - reflection semantics;
 *   - specialization algorithms;
 *   - constant-evaluation implementation;
 *   - classical IR;
 *   - quantum::ir;
 *   - HDL IR.
 *
 * ============================================================================
 * EXISTING REPOSITORY INTEGRATION
 * ============================================================================
 *
 * Canonical ownership:
 *
 *   grammar/antlr/ZamaniLexer.g4
 *       owns lexical tokens.
 *
 *   grammar/antlr/ZamaniParser.g4
 *       owns canonical:
 *           expression
 *           blockExpression
 *           statement
 *           declaration/item
 *           identifier
 *           pattern
 *           typeExpression
 *
 *   grammar/expressions/compile-time.g4
 *       owns expression-level compile-time syntax.
 *
 *   grammar/functions/compile-time-functions.g4
 *       owns compile-time-function declaration syntax.
 *
 *   grammar/compile/compile-time.g4
 *       owns compilation-control syntax.
 *
 *   grammar/macros/
 *       owns macro declaration and macro expansion syntax.
 *
 *   grammar/metaprogramming/
 *       owns broader compile-time language facilities.
 *
 *   grammar/resources/
 *       owns resource requirements/capabilities/constraints.
 *
 *   grammar/dialects/
 *       owns dialect extension mechanisms.
 *
 *   semantic/compiler subsystem
 *       owns compile-time evaluation.
 *
 *   canonical IR
 *       owns semantic representation after parsing and analysis.
 *
 *   quantum::ir
 *       remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * CRITICAL TOKEN CORRECTION
 * ============================================================================
 *
 * This grammar MUST NOT invent a `COMPTIME` lexer token.
 *
 * The repository's canonical lexer is the lexical authority.
 *
 * Explicit compile-time execution is represented through the existing
 * `CONST` vocabulary and an explicit execution form defined here.
 *
 * If the language specification later introduces a dedicated lexical keyword,
 * that token MUST first be added to the canonical lexer and language
 * specification. It must not be invented locally in this grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Compile-time execution describes a computation that is evaluated during
 * compilation.
 *
 * It MUST NOT make the program's semantic meaning dependent upon the machine
 * performing compilation.
 *
 * In particular, source syntax must not encode:
 *
 *   - CPU count;
 *   - core count;
 *   - thread count;
 *   - memory capacity;
 *   - GPU count;
 *   - FPGA count;
 *   - ASIC count;
 *   - QPU count;
 *   - qubit capacity;
 *   - machine topology;
 *   - device identifier;
 *   - vendor;
 *   - physical address;
 *   - deployment location.
 *
 * Such properties, where semantically relevant, are represented by the
 * repository's resource, capability, target, hardware, and compilation
 * contexts and are resolved downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No artificial finite language limit is encoded here.
 *
 * There is deliberately no:
 *
 *   MAX_COMPTIME_OPERATIONS
 *   MAX_COMPTIME_DEPTH
 *   MAX_COMPTIME_FUNCTIONS
 *   MAX_COMPTIME_GENERATIONS
 *   MAX_SPECIALIZATIONS
 *   MAX_TYPES
 *   MAX_VALUES
 *   MAX_RESOURCES
 *   MAX_DEVICES
 *   MAX_QUBITS
 *   MAX_CORES
 *   MAX_THREADS
 *
 * Compiler evaluation budgets, recursion protection, memory limits, timeout
 * policy, cancellation, and resource admission belong to compiler policy and
 * execution infrastructure.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing an execution form MUST NOT execute anything.
 *
 * The compiler must separately authorize compile-time effects.
 *
 * In particular, a compile-time evaluator MUST NOT implicitly gain:
 *
 *   filesystem access
 *   network access
 *   process execution
 *   environment access
 *   credential access
 *   hardware discovery
 *   device control
 *
 * merely because an expression appears in a compile-time context.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Determinism is a semantic property, not a grammar property.
 *
 * The compiler semantic layer must classify compile-time operations according
 * to their permitted effect class.
 *
 * Pure deterministic compile-time computation should be reproducible.
 *
 * External-state-dependent computation requires explicit semantic permission
 * and must preserve provenance where the compiler permits it.
 *
 * The grammar does not silently make external state available.
 *
 * ============================================================================
 * SOURCE/AST CONTRACT
 * ============================================================================
 *
 * Every compile-time execution construct must preserve:
 *
 *   - source span;
 *   - source ordering;
 *   - syntactic form;
 *   - nested expression structure;
 *   - nested block structure;
 *   - explicit compile-time intent;
 *   - source-level attributes where applicable.
 *
 * The generated ANTLR parse tree is NOT the canonical AST.
 *
 * The frontend AST layer converts this syntax into the repository's canonical
 * semantic representation.
 *
 * ============================================================================
 * COMPOSITION MODEL
 * ============================================================================
 *
 * This is a parser composition unit.
 *
 * It intentionally consumes canonical rules supplied by the assembled parser:
 *
 *   expression
 *   blockExpression
 *   identifier
 *   pattern
 *   typeExpression
 *   item
 *   statement
 *
 * Those rules MUST NOT be duplicated here.
 *
 * The canonical parser composition layer is responsible for exposing:
 *
 *   compileTimeExecution
 *
 * at the locations where a compile-time execution form is legal.
 *
 * ============================================================================
 */

parser grammar CompileTimeExecution;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PRIMARY ENTRY POINT
 * ============================================================================
 *
 * The canonical parser should expose `compileTimeExecution` as a source-level
 * construct where compile-time execution is permitted.
 *
 * The construct is intentionally explicit.
 *
 * `const` is already part of the canonical Zamani lexical vocabulary.
 *
 * ============================================================================
 */

compileTimeExecution
    : CONST compileTimeExecutionForm
    ;


/* ============================================================================
 * 2. EXECUTION FORM DISPATCH
 * ============================================================================
 *
 * This dispatcher contains only execution forms owned by this grammar.
 *
 * Expression-level compile-time constructs remain in:
 *
 *     grammar/expressions/compile-time.g4
 *
 * Compile-time function declarations remain in:
 *
 *     grammar/functions/compile-time-functions.g4
 *
 * Compilation-control constructs remain in:
 *
 *     grammar/compile/compile-time.g4
 *
 * ============================================================================
 */

compileTimeExecutionForm
    : compileTimeEvaluationBlock
    | compileTimeEvaluationStatement
    | compileTimeValueBinding
    | compileTimeGenerationBlock
    | compileTimeExecutionSequence
    ;


/* ============================================================================
 * 3. COMPILE-TIME EVALUATION BLOCK
 * ============================================================================
 *
 * Example conceptual syntax:
 *
 *     const {
 *         ...
 *     }
 *
 * The block itself remains canonical Zamani block syntax.
 *
 * This construct establishes a compile-time evaluation context.
 *
 * It does NOT establish what operations are legal inside that context.
 *
 * Semantic analysis owns that decision.
 *
 * ============================================================================
 */

compileTimeEvaluationBlock
    : blockExpression
    ;


/* ============================================================================
 * 4. SINGLE COMPILE-TIME EVALUATION STATEMENT
 * ============================================================================
 *
 * This is the smallest explicit compile-time execution boundary.
 *
 * The contained expression remains owned by the canonical expression
 * hierarchy.
 *
 * No second expression language is introduced.
 *
 * ============================================================================
 */

compileTimeEvaluationStatement
    : expression
    ;


/* ============================================================================
 * 5. COMPILE-TIME VALUE BINDING
 * ============================================================================
 *
 * A compile-time value binding establishes a named compile-time value.
 *
 * Example:
 *
 *     const answer = expression;
 *
 * The ordinary `const` statement already exists in the canonical parser.
 *
 * This rule exists as a metaprogramming integration boundary rather than
 * redefining the general constant-declaration semantics.
 *
 * ============================================================================
 */

compileTimeValueBinding
    : identifier ASSIGN expression SEMI?
    ;


/* ============================================================================
 * 6. COMPILE-TIME GENERATION BLOCK
 * ============================================================================
 *
 * A generation block marks a semantic region whose result may contribute
 * generated language structure.
 *
 * Generation itself is NOT performed by the parser.
 *
 * The generated representation may subsequently become:
 *
 *   - declarations;
 *   - expressions;
 *   - functions;
 *   - types;
 *   - quantum program structure;
 *   - classical program structure;
 *   - HDL structure;
 *   - metadata.
 *
 * The semantic generator determines what forms are legal.
 *
 * ============================================================================
 */

compileTimeGenerationBlock
    : generateKeyword blockExpression
    ;


/*
 * `generate` is intentionally represented through a parser-level rule rather
 * than a locally invented lexer token.
 *
 * The canonical lexer currently has GENERATIVE/SYNTHESIZE vocabulary but the
 * repository-wide lexical contract must decide whether a dedicated `generate`
 * keyword exists.
 *
 * To prevent this grammar from silently creating a lexer dependency, the
 * current production form uses the already-reserved `SYNTHESIZE` token as the
 * canonical source-level generation introducer.
 *
 * If the language specification standardizes a dedicated `GENERATE` token,
 * this rule is the ONLY integration point that needs to change.
 */

generateKeyword
    : SYNTHESIZE
    ;


/* ============================================================================
 * 7. COMPILE-TIME EXECUTION SEQUENCE
 * ============================================================================
 *
 * A sequence permits multiple compile-time evaluation forms without encoding
 * a finite number of operations.
 *
 * The actual sequence semantics belong downstream.
 *
 * ============================================================================
 */

compileTimeExecutionSequence
    : compileTimeSequenceElement
      (SEMI compileTimeSequenceElement)*
      SEMI?
    ;

compileTimeSequenceElement
    : expression
    | compileTimeValueBinding
    ;


/* ============================================================================
 * 8. COMPILE-TIME VALUE RESULT
 * ============================================================================
 *
 * The syntax of the resulting value is simply canonical expression syntax.
 *
 * The compiler semantic layer determines whether a compile-time computation
 * actually produces a value and whether that value is legal in its consuming
 * context.
 * ============================================================================
 */

compileTimeResult
    : expression
    ;


/* ============================================================================
 * 9. COMPILE-TIME DECLARATION RESULT
 * ============================================================================
 *
 * Generated declarations are semantic output, not parser execution.
 *
 * This rule provides a source-level boundary for an explicit compile-time
 * declaration-generation request.
 *
 * The generated declaration itself is represented through the canonical
 * declaration/item grammar.
 *
 * ============================================================================
 */

compileTimeGeneratedDeclaration
    : SYNTHESIZE item
    ;


/* ============================================================================
 * 10. COMPILE-TIME EXPRESSION BRIDGE
 * ============================================================================
 *
 * Expression-level compile-time constructs belong to:
 *
 *     grammar/expressions/compile-time.g4
 *
 * This rule exists only to make that ownership explicit.
 *
 * It MUST NOT duplicate compile-time expression productions.
 * ============================================================================
 */

compileTimeExpressionBridge
    : expression
    ;


/* ============================================================================
 * 11. COMPILE-TIME TYPE/VALUE BOUNDARY
 * ============================================================================
 *
 * Compile-time execution may consume values whose types are known only after
 * normal type analysis.
 *
 * This grammar therefore delegates type syntax completely.
 * ============================================================================
 */

compileTimeTypedValue
    : expression
    ;


/* ============================================================================
 * 12. COMPILE-TIME IDENTIFIER BOUNDARY
 * ============================================================================
 *
 * Names remain canonical Zamani identifiers.
 *
 * The grammar must not create device-specific, compiler-specific, or
 * implementation-specific identifier forms.
 * ============================================================================
 */

compileTimeName
    : identifier
    ;


/* ============================================================================
 * 13. COMPILE-TIME PATTERN BOUNDARY
 * ============================================================================
 *
 * Pattern semantics remain owned by the canonical pattern grammar.
 * ============================================================================
 */

compileTimePattern
    : pattern
    ;


/* ============================================================================
 * 14. COMPILE-TIME CONTROL-FLOW BODY
 * ============================================================================
 *
 * This rule is deliberately a thin integration point.
 *
 * It does not create compile-time versions of:
 *
 *     if
 *     for
 *     while
 *     match
 *     loop
 *
 * Those are ordinary Zamani language constructs.
 *
 * Their compile-time legality is determined by semantic evaluation.
 * ============================================================================
 */

compileTimeControlBody
    : blockExpression
    ;


/* ============================================================================
 * 15. COMPILE-TIME ITERATION BOUNDARY
 * ============================================================================
 *
 * Compile-time iteration must not encode an arbitrary maximum iteration count.
 *
 * The iteration expression is canonical Zamani syntax.
 *
 * Semantic analysis determines:
 *
 *     - whether the iteration is compile-time evaluable;
 *     - whether termination is acceptable;
 *     - whether resource policy permits evaluation;
 *     - whether deterministic evaluation is required;
 *     - whether generated output remains valid.
 *
 * ============================================================================
 */

compileTimeIteration
    : FOR compileTimePattern IN expression compileTimeControlBody
    ;


/* ============================================================================
 * 16. COMPILE-TIME CONDITIONAL BODY BRIDGE
 * ============================================================================
 *
 * Conditional compilation itself belongs to:
 *
 *     grammar/compile/compile-time.g4
 *
 * This rule MUST NOT become another conditional-compilation grammar.
 *
 * It merely permits ordinary conditional syntax to exist inside a compile-time
 * evaluation context.
 * ============================================================================
 */

compileTimeConditionalBody
    : IF expression blockExpression
      (ELSE blockExpression)?
    ;


/* ============================================================================
 * 17. COMPILE-TIME ASSERTION BRIDGE
 * ============================================================================
 *
 * Assertion syntax is owned by the canonical assertion grammar and/or
 * expression-level compile-time grammar.
 *
 * This bridge deliberately delegates to the canonical expression grammar.
 * ============================================================================
 */

compileTimeAssertion
    : ASSERT LPAREN expression RPAREN
      SEMI?
    ;


/* ============================================================================
 * 18. COMPILE-TIME FUNCTION INVOCATION BRIDGE
 * ============================================================================
 *
 * Compile-time function declarations belong to:
 *
 *     grammar/functions/compile-time-functions.g4
 *
 * Invocation syntax remains ordinary expression syntax.
 *
 * There is therefore no special compile-time call operator here.
 * ============================================================================
 */

compileTimeFunctionCall
    : expression
    ;


/* ============================================================================
 * 19. COMPILE-TIME SPECIALIZATION BRIDGE
 * ============================================================================
 *
 * Specialization semantics belong to the compiler.
 *
 * This grammar does not introduce target-specific specialization syntax.
 * ============================================================================
 */

compileTimeSpecialization
    : expression
    ;


/* ============================================================================
 * 20. COMPILE-TIME METADATA RESULT
 * ============================================================================
 *
 * Metadata remains represented through ordinary Zamani expressions.
 *
 * The metadata schema is owned by the appropriate semantic subsystem.
 * ============================================================================
 */

compileTimeMetadata
    : expression
    ;


/* ============================================================================
 * 21. COMPILE-TIME CAPABILITY QUERY BRIDGE
 * ============================================================================
 *
 * Capability semantics belong to:
 *
 *     grammar/core/capabilities.g4
 *     grammar/resources/
 *     semantic analysis
 *
 * This file must not duplicate the capability model.
 * ============================================================================
 */

compileTimeCapabilityQuery
    : expression
    ;


/* ============================================================================
 * 22. COMPILE-TIME RESOURCE QUERY BRIDGE
 * ============================================================================
 *
 * Resource semantics belong to grammar/resources/.
 *
 * A compile-time expression may consume a resource/capability value, but this
 * grammar must not define resource cardinalities or hardware topology.
 * ============================================================================
 */

compileTimeResourceQuery
    : expression
    ;


/* ============================================================================
 * 23. COMPILE-TIME TYPE QUERY BRIDGE
 * ============================================================================
 *
 * Type syntax and type semantics remain canonical.
 * ============================================================================
 */

compileTimeTypeQuery
    : expression
    ;


/* ============================================================================
 * 24. COMPILE-TIME EXECUTION TARGET
 * ============================================================================
 *
 * Compile-time execution is intentionally target-independent.
 *
 * There is no:
 *
 *     cpu(...)
 *     gpu(...)
 *     qpu(...)
 *     fpga(...)
 *     device(...)
 *
 * syntax here.
 *
 * If a compile-time computation needs target information, it must receive
 * semantically authorized capability/context data from the compiler.
 * ============================================================================
 */

compileTimeExecutionTarget
    : expression
    ;


/* ============================================================================
 * 25. COMPILE-TIME INPUT BOUNDARY
 * ============================================================================
 *
 * Input is represented as a normal expression.
 *
 * This does not grant external I/O authority.
 *
 * External data access is controlled by semantic effects and compiler policy.
 * ============================================================================
 */

compileTimeInput
    : expression
    ;


/* ============================================================================
 * 26. COMPILE-TIME OUTPUT BOUNDARY
 * ============================================================================
 *
 * Output is represented as a normal expression or generated declaration.
 *
 * The compiler decides whether the resulting value is:
 *
 *     - discarded;
 *     - bound;
 *     - substituted;
 *     - used for specialization;
 *     - transformed into generated source/IR;
 *     - rejected.
 * ============================================================================
 */

compileTimeOutput
    : expression
    | compileTimeGeneratedDeclaration
    ;


/* ============================================================================
 * 27. COMPILE-TIME EXECUTION REGION
 * ============================================================================
 *
 * Unified semantic region used by downstream AST construction.
 * ============================================================================
 */

compileTimeExecutionRegion
    : compileTimeEvaluationBlock
    | compileTimeExecutionSequence
    | compileTimeGenerationBlock
    ;


/* ============================================================================
 * 28. COMPILE-TIME EXPRESSION CONTEXT
 * ============================================================================
 *
 * This is deliberately an expression bridge.
 *
 * The expression grammar remains authoritative.
 * ============================================================================
 */

compileTimeExpressionContext
    : expression
    ;


/* ============================================================================
 * 29. COMPILE-TIME STATEMENT CONTEXT
 * ============================================================================
 *
 * Ordinary statement syntax remains authoritative.
 *
 * No compile-time statement language is created here.
 * ============================================================================
 */

compileTimeStatementContext
    : statement
    ;


/* ============================================================================
 * 30. COMPILE-TIME DECLARATION CONTEXT
 * ============================================================================
 *
 * Declaration syntax remains authoritative.
 * ============================================================================
 */

compileTimeDeclarationContext
    : item
    ;


/* ============================================================================
 * 31. SEMANTIC EXECUTION BOUNDARY
 * ============================================================================
 *
 * This rule is intentionally structural.
 *
 * The AST/semantic layer attaches execution metadata to this region.
 *
 * The parser itself performs no evaluation.
 * ============================================================================
 */

compileTimeExecutionContext
    : compileTimeExecutionRegion
    ;


/* ============================================================================
 * 32. INTEGRATION CONTRACT
 * ============================================================================
 *
 * REQUIRED CANONICAL RULES
 * ------------------------
 *
 * The assembled parser must provide:
 *
 *     expression
 *     blockExpression
 *     statement
 *     item
 *     identifier
 *     pattern
 *
 * This grammar intentionally does not redefine those rules.
 *
 *
 * REQUIRED LEXER TOKENS
 * ---------------------
 *
 * This grammar relies only on tokens already belonging to the canonical lexer:
 *
 *     CONST
 *     ASSIGN
 *     SEMI
 *     SYNTHESIZE
 *     FOR
 *     IN
 *     IF
 *     ELSE
 *     ASSERT
 *     LPAREN
 *     RPAREN
 *
 * It MUST NOT define lexer tokens locally.
 *
 *
 * IMPORTANT:
 *
 * If `SYNTHESIZE` is eventually removed from the canonical language, generation
 * syntax must migrate through the canonical lexer/specification first.
 *
 * This file must never silently invent replacement lexer tokens.
 *
 * ============================================================================
 */


/* ============================================================================
 * 33. AST INTEGRATION CONTRACT
 * ============================================================================
 *
 * The AST layer should map this grammar into semantic nodes equivalent to:
 *
 *     CompileTimeExecution
 *     CompileTimeEvaluation
 *     CompileTimeBinding
 *     CompileTimeGeneration
 *     CompileTimeSequence
 *
 * These are conceptual AST categories, not additional grammar-owned type
 * systems.
 *
 * Every node must retain:
 *
 *     source span
 *     source identity
 *     syntactic ordering
 *     nested expressions
 *     nested blocks
 *     explicit compile-time intent
 *
 * The AST must not directly contain:
 *
 *     physical device IDs
 *     hardware addresses
 *     machine topology
 *     fixed resource counts
 *
 * unless those are explicitly represented by a separate semantic resource
 * model after analysis.
 *
 * ============================================================================
 */


/* ============================================================================
 * 34. SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - whether the construct is legal;
 *     - whether all inputs are compile-time available;
 *     - whether dependencies are compile-time evaluable;
 *     - whether effects are permitted;
 *     - whether external state is permitted;
 *     - whether evaluation is deterministic;
 *     - whether evaluation is reproducible;
 *     - whether evaluation terminates according to language policy;
 *     - whether resource requirements are acceptable;
 *     - whether generated values are representable;
 *     - whether generated declarations are valid;
 *     - whether specialization preserves semantics.
 *
 * None of these decisions belong to this grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 35. IR INTEGRATION CONTRACT
 * ============================================================================
 *
 * Compile-time execution MUST lower into the repository's canonical semantic
 * representation.
 *
 * This file must never create:
 *
 *     QuantumGate
 *     Qubit
 *     QuantumCircuit
 *     PhysicalQubit
 *     ClassicalInstruction
 *     HardwareInstruction
 *
 * or equivalent duplicate representations.
 *
 * Quantum output must ultimately enter the canonical `quantum::ir` boundary.
 *
 * Classical output must enter the repository's canonical classical semantic
 * representation.
 *
 * HDL output must enter the canonical hardware/HDL representation.
 *
 * ============================================================================
 */


/* ============================================================================
 * 36. COMPILER INTEGRATION CONTRACT
 * ============================================================================
 *
 * The compiler pipeline is:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     name/type/effect/resource analysis
 *       |
 *       v
 *     compile-time eligibility
 *       |
 *       v
 *     compile-time evaluator
 *       |
 *       +--> constant evaluation
 *       +--> partial evaluation
 *       +--> specialization
 *       +--> generation
 *       |
 *       v
 *     canonical IR
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling / HAL
 *       |
 *       v
 *     runtime
 *
 * This grammar must never create a reverse dependency from grammar into the
 * evaluator, compiler, runtime, hardware, QEC, ZQN, scheduler, or optimizer.
 *
 * ============================================================================
 */


/* ============================================================================
 * 37. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Compile-time execution may construct or transform quantum source semantics.
 *
 * It MUST NOT:
 *
 *     - define a quantum gate set;
 *     - define a qubit representation;
 *     - define a topology;
 *     - select a QPU;
 *     - choose physical qubits;
 *     - schedule operations;
 *     - perform routing;
 *     - perform QEC;
 *     - define noise;
 *
 * Any resulting quantum semantics must eventually enter:
 *
 *     quantum::ir
 *
 * through the normal frontend/semantic lowering path.
 *
 * ZQN remains responsible for noise/fault semantics.
 *
 * QEC remains responsible for detection/correction.
 *
 * ============================================================================
 */


/* ============================================================================
 * 38. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Compile-time execution can compute arbitrary language-level classical
 * values subject to semantic evaluation policy.
 *
 * It must not impose:
 *
 *     integer width
 *     vector width
 *     tensor rank
 *     memory capacity
 *     processor count
 *
 * through grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 39. HDL/HARDWARE INTEGRATION
 * ============================================================================
 *
 * Compile-time generation may produce hardware declarations or hardware
 * parameters.
 *
 * However, the compile-time grammar must not hard-code:
 *
 *     number of ports
 *     number of wires
 *     number of registers
 *     number of pipeline stages
 *     FPGA size
 *     ASIC resources
 *     device identifiers
 *
 * Those belong to semantic hardware/resource models.
 *
 * ============================================================================
 */


/* ============================================================================
 * 40. SECURITY INTEGRATION
 * ============================================================================
 *
 * Compile-time execution is not an implicit security capability.
 *
 * The semantic evaluator must explicitly control effects.
 *
 * Parsing this grammar never:
 *
 *     reads files;
 *     writes files;
 *     accesses networks;
 *     launches processes;
 *     reads environment variables;
 *     reads secrets;
 *     accesses hardware.
 *
 * ============================================================================
 */


/* ============================================================================
 * 41. DETERMINISM AND REPRODUCIBILITY
 * ============================================================================
 *
 * Repeated compilation of identical source under identical semantic inputs
 * should produce equivalent compile-time semantic results where the language
 * classifies the evaluation as deterministic.
 *
 * The compiler must track relevant provenance outside this grammar.
 *
 * The grammar does not embed timestamps, random seeds, machine identifiers,
 * or host-specific values.
 *
 * ============================================================================
 */


/* ============================================================================
 * 42. RESOURCE POLICY
 * ============================================================================
 *
 * This grammar deliberately contains no evaluation budgets.
 *
 * The compiler may impose policy-controlled limits for:
 *
 *     memory
 *     evaluation time
 *     recursion
 *     generated source
 *     generated IR
 *     parallel evaluation
 *     cancellation
 *
 * Those limits are runtime/compiler policy and MUST NOT become grammar-level
 * constants.
 *
 * ============================================================================
 */


/* ============================================================================
 * 43. FAILURE MODEL
 * ============================================================================
 *
 * Syntax failure:
 *     parser diagnostics.
 *
 * Semantic failure:
 *     semantic/compiler diagnostics.
 *
 * Illegal effect:
 *     effect/capability diagnostic.
 *
 * Resource rejection:
 *     resource/compiler diagnostic.
 *
 * Evaluation failure:
 *     compile-time evaluation diagnostic.
 *
 * Non-deterministic evaluation:
 *     semantic/evaluation diagnostic according to language policy.
 *
 * Generated-program failure:
 *     generated-source/semantic diagnostic retaining provenance.
 *
 * This grammar must not encode provider-specific or hardware-specific failure
 * codes.
 *
 * ============================================================================
 */


/* ============================================================================
 * 44. COMPATIBILITY
 * ============================================================================
 *
 * Existing ordinary constants remain ordinary Zamani constants.
 *
 * Compile-time execution must therefore be introduced only where the complete
 * syntactic form establishes the intended context.
 *
 * No existing function, expression, quantum, classical, HDL, or module syntax
 * should be silently reinterpreted merely because this grammar is added.
 *
 * ============================================================================
 */


/* ============================================================================
 * 45. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_*
 *     MIN_*
 *     fixed machine counts
 *     fixed qubit counts
 *     fixed CPU counts
 *     fixed GPU counts
 *     fixed FPGA counts
 *     fixed ASIC counts
 *     fixed topology
 *     fixed memory sizes
 *     fixed register widths
 *     fixed tensor sizes
 *     fixed device IDs
 *     fixed addresses
 *     vendor-specific execution requirements
 *
 * The only finite repetition appearing in the grammar is structural ANTLR
 * syntax such as:
 *
 *     *
 *     +
 *     ?
 *
 * which does not impose a semantic cardinality limit.
 *
 * ============================================================================
 */


/* ============================================================================
 * 46. RUST INTEGRATION
 * ============================================================================
 *
 * Target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * The generated parser/frontend integration must:
 *
 *     #![deny(unsafe_code)]
 *     #![deny(unsafe_op_in_unsafe_fn)]
 *
 * No Rust implementation code is embedded in this grammar.
 *
 * This grammar requires no unsafe operations.
 *
 * ============================================================================
 */


/* ============================================================================
 * 47. TOOLING
 * ============================================================================
 *
 * Tooling may use this grammar for:
 *
 *     syntax highlighting
 *     formatting
 *     source navigation
 *     documentation extraction
 *     semantic indexing
 *     compile-time dependency visualization
 *     generated-source provenance
 *
 * Tooling must not infer runtime execution merely from parsing.
 *
 * ============================================================================
 */


/* ============================================================================
 * 48. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [ ] It composes with the canonical Zamani lexer.
 *   [ ] It does not invent lexer tokens.
 *   [ ] It does not duplicate ordinary expression syntax.
 *   [ ] It does not duplicate ordinary block syntax.
 *   [ ] It does not duplicate compile-time expression syntax.
 *   [ ] It does not duplicate compile-time function declarations.
 *   [ ] It does not duplicate macro syntax.
 *   [ ] It does not duplicate compilation-control syntax.
 *   [ ] It preserves source spans through the AST boundary.
 *   [ ] It introduces no fixed resource limits.
 *   [ ] It introduces no machine assumptions.
 *   [ ] It introduces no hardware assumptions.
 *   [ ] It introduces no quantum topology assumptions.
 *   [ ] It introduces no quantum gate inventory.
 *   [ ] It performs no evaluation during parsing.
 *   [ ] It performs no filesystem access.
 *   [ ] It performs no network access.
 *   [ ] It performs no hardware discovery.
 *   [ ] It has positive parser tests.
 *   [ ] It has negative parser tests.
 *   [ ] It has boundary/scalability tests.
 *   [ ] It has cross-domain tests.
 *   [ ] It has deterministic parse tests.
 *   [ ] It integrates with semantic compile-time evaluation.
 *   [ ] It integrates with canonical IR.
 *   [ ] Quantum results reach `quantum::ir`.
 *   [ ] Classical results reach canonical classical IR.
 *   [ ] HDL results reach canonical hardware/HDL IR.
 *   [ ] No grammar/runtime dependency cycle exists.
 *   [ ] No grammar/IR dependency cycle exists.
 *   [ ] Rust integration remains safe Rust only.
 *
 * ============================================================================
 */