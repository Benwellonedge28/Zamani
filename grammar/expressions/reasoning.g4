/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/expressions/reasoning.g4
 *
 * STATUS
 * ------
 * CANONICAL REASONING-EXPRESSION SYNTAX CONTRACT
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust:
 *     1.97 / 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX of generic reasoning expressions.
 *
 * It provides one common syntactic abstraction for:
 *
 *     infer
 *     deduce
 *     reason
 *
 * These constructs express reasoning intent.
 *
 * They do NOT specify:
 *
 *     - a particular inference algorithm;
 *     - a theorem prover;
 *     - a knowledge database;
 *     - a machine-learning model;
 *     - a probabilistic engine;
 *     - a symbolic engine;
 *     - a hardware accelerator;
 *     - a CPU/GPU/QPU;
 *     - a particular reasoning strategy;
 *     - a particular runtime implementation.
 *
 * Those concerns belong to semantic analysis, libraries, capabilities,
 * policies, resources, execution planning and runtime implementations.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The intended pipeline is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     reasoning-expression syntax
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic reasoning model
 *       |
 *       +-------------------+
 *       |                   |
 *       v                   v
 *     classical         quantum::ir
 *       |                   |
 *       +---------+---------+
 *                 |
 *                 v
 *       canonical semantic representation
 *                 |
 *          optimization/lowering
 *                 |
 *       target realization
 *
 * Reasoning therefore remains above physical realization.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     reasoningExpression
 *     reasoningOperation
 *     reasoningOperator
 *     reasoningTarget
 *     reasoningSourceClause
 *     reasoningContextClause
 *     reasoningOptionList
 *     reasoningOption
 *
 * It owns the syntax that distinguishes:
 *
 *     infer ...
 *     deduce ...
 *     reason ...
 *
 * and the optional:
 *
 *     from ...
 *     with (...)
 *
 * reasoning modifiers.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     expression
 *     assignmentExpression
 *     binary precedence
 *     unary precedence
 *     postfix expressions
 *     calls
 *     indexing
 *     patterns
 *     guards
 *     statements
 *     declarations
 *     types
 *     knowledge storage
 *     learning
 *     adaptation
 *     probability implementation
 *     provenance implementation
 *     policies
 *     effects
 *     capabilities
 *     resources
 *     AST definitions
 *     semantic implementation
 *     IR definitions
 *     runtime execution
 *
 * ============================================================================
 * CRITICAL DEPENDENCY RULE
 * ============================================================================
 *
 * This file MUST remain a leaf grammar with respect to the canonical
 * expression-precedence grammar.
 *
 * It MUST NOT import:
 *
 *     Expressions
 *
 * if `Expressions` imports this file.
 *
 * Doing so would create a circular parser-grammar dependency.
 *
 * Instead, the repository's expression composition layer must provide the
 * operand boundary consumed by this grammar.
 *
 * The integration boundary is:
 *
 *     reasoningTarget
 *     reasoningSourceClause
 *     reasoningContextClause
 *
 * which are composed from the canonical expression core at assembly time.
 *
 * No second expression language is permitted.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Canonical lexical ownership remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file MUST NOT define lexer rules.
 *
 * Required canonical tokens:
 *
 *     INFER
 *     DEDUCE
 *     REASON
 *     FROM
 *     WITH
 *     LPAREN
 *     RPAREN
 *     COMMA
 *
 * Optional semicolon termination belongs to the surrounding statement
 * grammar, NOT to this expression grammar.
 *
 * The tokens:
 *
 *     INFER
 *     DEDUCE
 *     REASON
 *
 * are already part of the repository's canonical lexical vocabulary.
 *
 * No new synonym such as:
 *
 *     INFERENCE
 *     DEDUCTION
 *     THINK
 *     ANALYZE
 *
 * is introduced here.
 *
 * ============================================================================
 * LANGUAGE DESIGN
 * ============================================================================
 *
 * The three surface verbs converge on one semantic abstraction:
 *
 *     reasoningOperation
 *
 * They are intentionally not three unrelated semantic systems.
 *
 * Conceptually:
 *
 *     infer X
 *
 *     deduce X
 *
 *     reason X
 *
 * all produce a reasoning-intent AST node whose operation kind records the
 * source-level operation:
 *
 *     Infer
 *     Deduce
 *     Reason
 *
 * The semantic layer determines the precise meaning.
 *
 * ============================================================================
 * CANONICAL FORMS
 * ============================================================================
 *
 * The basic form is:
 *
 *     infer TARGET
 *
 *     deduce TARGET
 *
 *     reason TARGET
 *
 * A source/evidence expression may be supplied:
 *
 *     infer TARGET from SOURCE
 *
 *     deduce TARGET from SOURCE
 *
 *     reason TARGET from SOURCE
 *
 * A reasoning context may be supplied:
 *
 *     infer TARGET with (OPTION, OPTION)
 *
 *     deduce TARGET with (OPTION, OPTION)
 *
 *     reason TARGET with (OPTION, OPTION)
 *
 * Both may be combined:
 *
 *     infer TARGET from SOURCE with (OPTION, OPTION)
 *
 * The grammar does not define a closed list of options.
 *
 * This allows future semantic facilities such as:
 *
 *     strategy
 *     evidence
 *     policy
 *     confidence
 *     provenance
 *     model
 *     knowledge
 *     constraints
 *     timeout
 *     resource preferences
 *
 * without adding a new keyword for each one.
 *
 * ============================================================================
 * WHY `TARGET` IS AN EXPRESSION BOUNDARY
 * ============================================================================
 *
 * The reasoning target must eventually be capable of representing any valid
 * Zamani value/expression.
 *
 * Examples:
 *
 *     infer hypothesis
 *
 *     infer hypothesis + evidence
 *
 *     infer model(input)
 *
 *     infer knowledge.query(pattern)
 *
 *     infer quantum_result
 *
 *     infer measurement_result == expected
 *
 *     infer tensor[index]
 *
 *     infer distributed_value
 *
 * The grammar file therefore MUST NOT reproduce the complete expression
 * precedence hierarchy.
 *
 * The canonical expression composition layer owns that hierarchy.
 *
 * ============================================================================
 * SOURCE / EVIDENCE MODEL
 * ============================================================================
 *
 * `from` identifies an optional source expression.
 *
 * It does not imply that the source is:
 *
 *     a database;
 *     a knowledge graph;
 *     a file;
 *     a model;
 *     a network;
 *     a classical value;
 *     a quantum value.
 *
 * Those are semantic interpretations.
 *
 * Examples:
 *
 *     infer conclusion from evidence
 *
 *     infer result from knowledge.query(pattern)
 *
 *     infer result from measurement
 *
 *     infer result from model(input)
 *
 * The grammar only records the source expression.
 *
 * ============================================================================
 * CONTEXT MODEL
 * ============================================================================
 *
 * `with (...)` provides an extensible reasoning context.
 *
 * The context consists of ordinary expressions.
 *
 * It intentionally does not define:
 *
 *     strategy = ...
 *     model = ...
 *     confidence = ...
 *     policy = ...
 *
 * as fixed grammar properties.
 *
 * Those names remain ordinary identifiers/expressions unless another
 * authoritative grammar explicitly reserves them.
 *
 * This keeps the core language extensible without keyword explosion.
 *
 * ============================================================================
 * OPTION MODEL
 * ============================================================================
 *
 * Each reasoning option is an expression.
 *
 * This permits:
 *
 *     reason proposition with (context)
 *
 *     reason proposition with (policy)
 *
 *     reason proposition with (model)
 *
 *     reason proposition with (confidence)
 *
 *     reason proposition with (configuration)
 *
 * without changing the grammar whenever a new reasoning facility is added.
 *
 * Named option semantics, if desired, belong to the semantic layer or a
 * dedicated configuration grammar.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "Is this structurally a reasoning expression?"
 *
 * Semantic analysis answers:
 *
 *     "What reasoning operation does this mean?"
 *
 * Semantic analysis owns:
 *
 *     - operation resolution;
 *     - target typing;
 *     - source typing;
 *     - context typing;
 *     - evidence interpretation;
 *     - knowledge resolution;
 *     - model resolution;
 *     - strategy selection;
 *     - uncertainty semantics;
 *     - confidence semantics;
 *     - provenance;
 *     - effect inference;
 *     - capability requirements;
 *     - resource requirements;
 *     - policy enforcement;
 *     - authorization;
 *     - determinism;
 *     - execution planning.
 *
 * The parser performs none of these operations.
 *
 * ============================================================================
 * UNIFIED REASONING SEMANTICS
 * ============================================================================
 *
 * `infer`, `deduce`, and `reason` MUST converge on one semantic abstraction.
 *
 * Conceptually:
 *
 *     reasoningOperation
 *       |
 *       +-- kind
 *       |     Infer
 *       |     Deduce
 *       |     Reason
 *       |
 *       +-- target
 *       |
 *       +-- source
 *       |
 *       +-- context
 *       |
 *       +-- source span
 *       |
 *       +-- metadata
 *
 * The semantic layer may later distinguish their formal reasoning behavior.
 *
 * The grammar must not hard-code an algorithmic distinction that prevents
 * future implementations from evolving.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Reasoning may consume knowledge constructs, but knowledge syntax is owned
 * elsewhere.
 *
 * Conceptually:
 *
 *     reasoning
 *          |
 *          v
 *     knowledge query / evidence
 *          |
 *          v
 *     semantic reasoning model
 *
 * This file MUST NOT define:
 *
 *     assert
 *     retract
 *     query
 *
 * as duplicate reasoning rules.
 *
 * A query can simply be an expression supplied to:
 *
 *     from
 *
 * or the reasoning target/context.
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Evidence remains a semantic concept.
 *
 * Evidence may originate from:
 *
 *     values
 *     knowledge
 *     observations
 *     measurements
 *     simulations
 *     model outputs
 *     files
 *     services
 *     distributed computation
 *     quantum measurements
 *
 * The grammar records the expression that supplies evidence.
 *
 * Evidence validation belongs downstream.
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Reasoning may operate on:
 *
 *     probabilities
 *     distributions
 *     confidence values
 *     uncertain values
 *     intervals
 *     symbolic uncertainty
 *
 * No probabilistic algorithm is encoded here.
 *
 * No fixed precision is imposed.
 *
 * No fixed number of outcomes is imposed.
 *
 * ============================================================================
 * CAUSAL / SYMBOLIC / LEARNED REASONING
 * ============================================================================
 *
 * The same syntax can represent:
 *
 *     symbolic reasoning
 *     deductive reasoning
 *     inductive reasoning
 *     abductive reasoning
 *     causal reasoning
 *     probabilistic reasoning
 *     learned reasoning
 *     neural-symbolic reasoning
 *     quantum-assisted reasoning
 *
 * The implementation is selected by semantic capabilities and policies.
 *
 * The language does not need a new keyword for each reasoning methodology.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Reasoning is not automatically assumed to be pure.
 *
 * The semantic layer may infer effects such as:
 *
 *     reasoning
 *     knowledge.read
 *     model.inference
 *     randomness
 *     external.io
 *     network
 *     measurement
 *
 * depending on the resolved operation.
 *
 * The grammar itself declares no effects.
 *
 * This keeps syntax independent from implementation.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Reasoning may require capabilities such as:
 *
 *     capability("reasoning")
 *     capability("knowledge.query")
 *     capability("model.inference")
 *     capability("probabilistic.compute")
 *     capability("quantum.measurement")
 *
 * These are semantic capabilities.
 *
 * This grammar does not select a target.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Reasoning may require:
 *
 *     memory
 *     compute
 *     model capacity
 *     communication
 *     accelerator resources
 *     quantum resources
 *     storage
 *
 * Resource requirements are expressed through the canonical resource system.
 *
 * This grammar MUST NOT define:
 *
 *     MAX_REASONING_DEPTH
 *     MAX_FACTS
 *     MAX_EVIDENCE
 *     MAX_REASONING_STEPS
 *     MAX_CONTEXT
 *     MAX_MODELS
 *     MAX_MEMORY
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *
 * or any equivalent universal ceiling.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * The reasoning grammar contains no artificial machine-size assumptions.
 *
 * There is no grammar-level limit on:
 *
 *     reasoning expressions
 *     reasoning operations
 *     nesting
 *     evidence sources
 *     context options
 *     expression complexity
 *     knowledge size
 *     model size
 *     data size
 *     processor count
 *     accelerator count
 *     quantum resource count
 *     distributed node count
 *
 * The `*` repetition in the option list is deliberately open-ended.
 *
 * Practical limits belong to:
 *
 *     compiler resource policy
 *     runtime resource policy
 *     target capability
 *     deployment configuration
 *     available memory
 *     available storage
 *     execution time
 *
 * "Infinity" therefore means:
 *
 *     no artificial finite ceiling is imposed by this grammar.
 *
 * It does not assert that finite physical machines possess infinite resources.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * A reasoning expression MUST NOT encode:
 *
 *     CPU number
 *     GPU number
 *     FPGA number
 *     QPU number
 *     physical qubit number
 *     node number
 *     device address
 *     fixed topology
 *     register width
 *     memory-bank identity
 *
 * Target realization occurs downstream.
 *
 * The same reasoning source may be compiled for:
 *
 *     embedded
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU-assisted computation
 *     simulator
 *     HPC
 *     cluster
 *     distributed
 *     cloud
 *     future targets
 *
 * subject to actual capabilities and resources.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Reasoning expressions may consume quantum-derived values:
 *
 *     infer result from measurement
 *
 *     reason state_property from quantum_result
 *
 *     deduce decision from measurement_result
 *
 * The grammar does not define quantum operations.
 *
 * Quantum semantics remain owned by:
 *
 *     grammar/quantum/
 *
 * and ultimately:
 *
 *     quantum::ir
 *
 * No quantum-specific reasoning IR is introduced.
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * Reasoning may participate in:
 *
 *     classical -> reasoning
 *     quantum -> reasoning
 *     reasoning -> classical
 *     reasoning -> quantum control
 *     AI -> reasoning
 *     reasoning -> AI
 *     HDL/hardware observation -> reasoning
 *
 * These are semantic relationships.
 *
 * The grammar remains domain-neutral.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A reasoning target or source may semantically refer to:
 *
 *     hardware state
 *     sensor data
 *     simulation result
 *     verification result
 *     synthesized artifact metadata
 *
 * This file does not encode hardware topology or fixed widths.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Reasoning may consume distributed values or produce decisions used by:
 *
 *     actors
 *     tasks
 *     services
 *     distributed workflows
 *
 * Distributed execution remains owned by:
 *
 *     grammar/distributed/
 *     grammar/concurrency/
 *     grammar/networking/
 *
 * This file does not create a second actor/message system.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Reasoning can be constrained by policies concerning:
 *
 *     evidence sources
 *     models
 *     data access
 *     randomness
 *     network access
 *     resource use
 *     adaptation
 *     privacy/security
 *
 * Policies are resolved downstream.
 *
 * A reasoning expression cannot bypass an applicable policy merely because
 * its syntax is valid.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Reasoning is a natural provenance-producing operation.
 *
 * The semantic representation may preserve:
 *
 *     operation kind
 *     target
 *     source
 *     context
 *     evidence
 *     selected strategy
 *     model
 *     policy
 *     compiler transformation
 *     execution result
 *
 * This file does not define provenance syntax.
 *
 * Provenance belongs to the canonical provenance subsystem.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST be deterministic for identical:
 *
 *     source
 *     grammar
 *     lexer
 *     parser configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     system time
 *     randomness
 *     filesystem state
 *     network state
 *     hardware state
 *     device discovery
 *     target availability
 *     runtime scheduler state
 *
 * Runtime reasoning may legitimately be nondeterministic.
 *
 * That nondeterminism must be represented semantically through effects,
 * policies and execution metadata rather than through parser behavior.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing is non-executing.
 *
 * This grammar MUST NOT:
 *
 *     execute reasoning;
 *     query a database;
 *     invoke a model;
 *     inspect hardware;
 *     access a network;
 *     read credentials;
 *     execute external code;
 *     invoke a QPU;
 *     invoke a simulator;
 *     load plugins.
 *
 * A source expression is syntax, not permission to execute its meaning.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST MUST preserve at minimum:
 *
 *     operation kind
 *     target expression
 *     optional source expression
 *     ordered context expressions
 *     source span
 *     source ordering
 *
 * Conceptually:
 *
 *     ReasoningExpression {
 *         kind: Infer | Deduce | Reason,
 *         target,
 *         source,
 *         context,
 *         span
 *     }
 *
 * The exact Rust type belongs to the existing domain-neutral AST.
 *
 * This grammar MUST NOT create:
 *
 *     QuantumReasoningExpression
 *     GPUReasoningExpression
 *     AIReasoningExpression
 *     HardwareReasoningExpression
 *
 * as universal AST categories.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * The lowering path is:
 *
 *     reasoning expression
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic reasoning operation
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *      classical          quantum::ir
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *               optimization
 *                    |
 *                lowering
 *                    |
 *              scheduling
 *                    |
 *             target realization
 *
 * No second reasoning IR is permitted.
 *
 * ============================================================================
 * COMPATIBILITY WITH EXISTING MEMORY SYNTAX
 * ============================================================================
 *
 * `grammar/memory/sankofa.g4` currently owns source-level forms such as:
 *
 *     infer expression;
 *     infer expression from expression;
 *     infer expression with (...);
 *
 * The production integration MUST move the common syntax ownership here.
 *
 * The memory grammar should retain its public compatibility rule but delegate
 * the actual construct:
 *
 *     sankofaInferExpression
 *         : reasoningExpression
 *         ;
 *
 * and:
 *
 *     sankofaInfer
 *         : reasoningExpression SEMICOLON
 *         ;
 *
 * or equivalent composition that preserves the existing semicolon behavior.
 *
 * This prevents two independent implementations of `infer`.
 *
 * The memory subsystem remains responsible for interpreting the semantic
 * result as a memory/knowledge operation when its surrounding context
 * requires that interpretation.
 *
 * ============================================================================
 * COMPATIBILITY WITH EXISTING AI INFERENCE
 * ============================================================================
 *
 * `grammar/ai/inference.g4` uses:
 *
 *     @infer(...)
 *
 * That is a distinct annotation/directive spelling because it begins with:
 *
 *     AT
 *
 * It MUST NOT be silently converted into:
 *
 *     infer ...
 *
 * The two constructs may share semantic inference infrastructure, but their
 * source syntax remains independently owned:
 *
 *     infer ...
 *         -> reasoning expression
 *
 *     @infer(...)
 *         -> inference annotation/directive
 *
 * This distinction prevents grammar ambiguity and preserves the existing AI
 * inference contract.
 *
 * ============================================================================
 * COMPATIBILITY WITH AGENT/MIND SYNTAX
 * ============================================================================
 *
 * Existing agent/mind grammars may contain annotation forms such as:
 *
 *     @reason(...)
 *     @infer(...)
 *
 * Those are annotation/directive syntax and are not duplicated here.
 *
 * Semantic infrastructure may normalize them into shared reasoning/inference
 * concepts downstream.
 *
 * ============================================================================
 * EXPRESSION COMPOSITION INTEGRATION
 * ============================================================================
 *
 * Because `grammar/expressions/expressions.g4` currently owns the complete
 * precedence hierarchy, this file must not import it.
 *
 * The repository should introduce an expression-core composition boundary.
 *
 * Recommended dependency direction:
 *
 *     ExpressionsCore
 *          |
 *          +------------------+
 *          |                  |
 *          v                  v
 *     Reasoning           Expressions
 *          |                  |
 *          +--------+---------+
 *                   |
 *                   v
 *          canonical expression
 *
 * The core boundary supplies the operand expression rules required by this
 * file without creating a circular dependency.
 *
 * The exact extraction should preserve all existing expression precedence and
 * AST behavior.
 *
 * ============================================================================
 * RECOMMENDED EXPRESSION-CORE INTERFACE
 * ============================================================================
 *
 * The extraction boundary should expose the canonical expression rule used
 * by reasoning operands:
 *
 *     reasoningOperand
 *
 * or, preferably, the existing canonical expression rule after the expression
 * grammar is split into:
 *
 *     expression-core.g4
 *     expressions.g4
 *
 * The important invariant is:
 *
 *     there remains exactly one expression precedence hierarchy.
 *
 * `reasoning.g4` MUST NOT copy:
 *
 *     assignmentExpression
 *     conditionalExpression
 *     logicalExpression
 *     arithmeticExpression
 *     postfixExpression
 *     primaryExpression
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar Reasoning;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ENTRY
 * ========================================================================== */

/*
 * Universal reasoning expression.
 *
 * The operand boundary is intentionally explicit.
 *
 * During final parser assembly, `reasoningOperand` MUST be bound to the
 * canonical expression-core rule.
 */
reasoningExpression
    : reasoningOperation
    ;


/* ============================================================================
 * 2. REASONING OPERATION
 * ========================================================================== */

/*
 * All reasoning verbs converge on one structural representation.
 */
reasoningOperation
    : reasoningOperator
      reasoningTarget
      reasoningSourceClause?
      reasoningContextClause?
    ;


/* ============================================================================
 * 3. REASONING OPERATOR
 * ========================================================================== */

reasoningOperator
    : INFER
    | DEDUCE
    | REASON
    ;


/* ============================================================================
 * 4. TARGET
 * ========================================================================== */

/*
 * The target is the proposition/question/result being reasoned about.
 *
 * The actual expression rule is supplied by the canonical expression-core
 * composition.
 *
 * This named boundary is important because semantic tooling can distinguish
 * the reasoning target from its source without owning expression precedence.
 */
reasoningTarget
    : reasoningOperand
    ;


/* ============================================================================
 * 5. SOURCE / EVIDENCE
 * ========================================================================== */

reasoningSourceClause
    : FROM
      reasoningOperand
    ;


/* ============================================================================
 * 6. CONTEXT
 * ========================================================================== */

reasoningContextClause
    : WITH
      LPAREN
      reasoningOptionList?
      RPAREN
    ;


/* ============================================================================
 * 7. CONTEXT OPTION LIST
 * ========================================================================== */

/*
 * Options are ordinary expressions.
 *
 * No fixed option vocabulary is encoded.
 */
reasoningOptionList
    : reasoningOption
      (
          COMMA
          reasoningOption
      )*
      COMMA?
    ;


reasoningOption
    : reasoningOperand
    ;


/* ============================================================================
 * 8. EXPRESSION-CORE INTEGRATION BOUNDARY
 * ========================================================================== */

/*
 * IMPORTANT:
 *
 * This rule is an integration boundary, not a second expression grammar.
 *
 * The final composed parser MUST bind this rule to the canonical expression
 * core.
 *
 * During the expression-core extraction, this rule should become either:
 *
 *     reasoningOperand
 *         : expressionCore
 *         ;
 *
 * or an equivalent imported canonical rule.
 *
 * It MUST NOT be replaced with a copied arithmetic/call/postfix hierarchy.
 *
 * This placeholder form is deliberately isolated so that only this boundary
 * changes when the expression-core extraction is performed.
 *
 * The final production composition MUST NOT leave two independently
 * authoritative expression roots.
 */
reasoningOperand
    : expressionCore
    ;


/* ============================================================================
 * 9. SEMICOLON BOUNDARY
 * ========================================================================== */

/*
 * No SEMI is consumed here.
 *
 * Reasoning is an expression.
 *
 * Statement grammars decide whether a surrounding statement requires:
 *
 *     ;
 *
 * This prevents:
 *
 *     reasoningExpression
 *
 * from becoming both an expression and a statement grammar.
 */


/* ============================================================================
 * 10. NO CLOSED REASONING VOCABULARY
 * ========================================================================== */

/*
 * The grammar deliberately does NOT enumerate:
 *
 *     deductive
 *     inductive
 *     abductive
 *     causal
 *     probabilistic
 *     symbolic
 *     neural
 *     Bayesian
 *     theorem
 *     planning
 *     proof
 *
 * as separate language keywords.
 *
 * Those are semantic strategies or library/domain concepts.
 *
 * They may be represented through:
 *
 *     reasoning context
 *     policies
 *     capabilities
 *     models
 *     libraries
 *     dialects
 *     metadata
 *
 * without expanding the universal keyword set.
 */


/* ============================================================================
 * 11. NO ALGORITHM ENUMERATION
 * ========================================================================== */

/*
 * The grammar MUST NOT contain:
 *
 *     inferenceAlgorithm
 *         : bayes
 *         | forward_chain
 *         | backward_chain
 *         | ...
 *
 * Algorithm selection is semantic/runtime behavior.
 *
 * This is essential for long-term language evolution.
 */


/* ============================================================================
 * 12. NO APPLICATION-SPECIFIC REASONING
 * ========================================================================== */

/*
 * This file MUST NOT introduce syntax for:
 *
 *     sentiment
 *     vision
 *     robotics
 *     payments
 *     administration
 *     legal workflows
 *     blockchain
 *     VR/AR
 *     medical workflows
 *     domain-specific business actions
 *
 * Such functionality belongs to:
 *
 *     libraries
 *     dialects
 *     services
 *     policies
 *     applications
 *
 * The reasoning primitive remains universal.
 */


/* ============================================================================
 * 13. SCALABILITY / HARD-CODING AUDIT
 * ========================================================================== */

/*
 * Forbidden universal ceilings include:
 *
 *     MAX_REASONING_OPERATIONS
 *     MAX_REASONING_DEPTH
 *     MAX_REASONING_OPTIONS
 *     MAX_EVIDENCE
 *     MAX_CONTEXT
 *     MAX_FACTS
 *     MAX_KNOWLEDGE
 *     MAX_MODELS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * No equivalent constants may be introduced into this grammar.
 *
 * Repetition is represented structurally by:
 *
 *     *
 *
 * Practical limits are implementation/resource limits, not language semantics.
 */


/* ============================================================================
 * 14. DIAGNOSTIC CONTRACT
 * ========================================================================== */

/*
 * Structural diagnostics should distinguish:
 *
 *     missing reasoning operator
 *     missing target
 *     malformed source clause
 *     malformed context
 *     missing closing parenthesis
 *     malformed option list
 *
 * Semantic diagnostics belong downstream:
 *
 *     unknown reasoning strategy
 *     invalid target type
 *     unavailable capability
 *     insufficient resources
 *     forbidden policy
 *     unavailable evidence
 *     invalid model
 *     unsupported backend
 *
 * A target-feasibility failure MUST NOT be reported as a syntax error.
 */


/* ============================================================================
 * 15. DETERMINISM CONTRACT
 * ========================================================================== */

/*
 * Given identical:
 *
 *     source
 *     lexer
 *     grammar
 *     parser configuration
 *
 * this grammar MUST produce the same structural parse.
 *
 * It MUST NOT inspect:
 *
 *     hardware
 *     resources
 *     network
 *     filesystem
 *     time
 *     randomness
 *     runtime state
 */


/* ============================================================================
 * 16. SAFETY CONTRACT
 * ========================================================================== */

/*
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime execution;
 *     - no hardware access;
 *     - no unsafe Rust.
 *
 * The consuming Rust implementation MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 */


/* ============================================================================
 * 17. AST COMPLETION CONTRACT
 * ========================================================================== */

/*
 * The AST adapter MUST map:
 *
 *     INFER
 *         -> reasoning kind Infer
 *
 *     DEDUCE
 *         -> reasoning kind Deduce
 *
 *     REASON
 *         -> reasoning kind Reason
 *
 * and preserve:
 *
 *     target
 *     source
 *     ordered context
 *     source span
 *
 * No domain-specific AST node is required at the grammar level.
 *
 * Existing Rust AST compatibility must be evaluated before introducing a new
 * enum/variant. Where the repository already has:
 *
 *     Infer(Expression)
 *     Deduce(Expression)
 *
 * those representations should be extended through the established AST
 * compatibility process rather than replaced silently.
 */


/* ============================================================================
 * 18. SEMANTIC COMPLETION CONTRACT
 * ========================================================================== */

/*
 * A parsed reasoning expression is NOT automatically:
 *
 *     true
 *     proven
 *     trustworthy
 *     deterministic
 *     executable
 *     authorized
 *     resource-feasible
 *
 * Semantic analysis must establish those properties independently.
 *
 * In particular:
 *
 *     syntax validity
 *
 * is not:
 *
 *     reasoning validity.
 *
 * ============================================================================
 * 19. TEST CONTRACT
 * ========================================================================== */

/*
 * POSITIVE
 *
 *     infer hypothesis
 *     deduce conclusion
 *     reason proposition
 *
 *     infer conclusion from evidence
 *     deduce conclusion from premises
 *     reason result from observation
 *
 *     infer result with (context)
 *     deduce result with (policy)
 *     reason result with (evidence)
 *
 *     infer result from evidence with (context, policy)
 *     deduce result from premises with (model, constraints)
 *     reason result from observation with (provenance)
 *
 * CROSS-DOMAIN
 *
 *     infer result from measurement
 *     deduce decision from quantum_result
 *     reason tensor_result from model(input)
 *     infer hardware_state from observation
 *     reason distributed_value from service_result
 *
 * NESTING
 *
 *     infer f(x)
 *     infer f(g(x))
 *     infer f(x + y)
 *     infer f(g(x + y))
 *
 *     infer result from knowledge.query(pattern)
 *
 * The exact accepted forms depend on the canonical expression-core grammar.
 *
 * NEGATIVE
 *
 *     infer
 *     deduce
 *     reason
 *
 *     infer from evidence
 *     deduce from premises
 *     reason from observation
 *
 *     infer result from
 *     deduce result with
 *     reason result with ()
 *
 *     infer result with (,)
 *
 * STRUCTURAL BOUNDARY
 *
 *     reasoning expression followed by statement terminator
 *
 * must be accepted only when the enclosing statement grammar owns the
 * terminator.
 *
 * DETERMINISM
 *
 * Repeated parsing of identical source under identical grammar configuration
 * must produce equivalent parse trees.
 *
 * SCALABILITY
 *
 * Tests must use increasing implementation-supported sizes without defining
 * a language-level maximum.
 *
 * ============================================================================
 * 20. INTEGRATION MATRIX
 * ========================================================================== */

/*
 * This file:
 *
 *     OWNS:
 *         reasoningExpression
 *         reasoningOperation
 *         reasoningOperator
 *         reasoningTarget
 *         reasoningSourceClause
 *         reasoningContextClause
 *         reasoningOptionList
 *
 *     CONSUMES:
 *         expressionCore
 *         canonical lexer tokens
 *
 *     IS CONSUMED BY:
 *         canonical expression composition
 *         Sankofa compatibility composition
 *         reasoning/AI semantic adapters
 *         tests
 *
 *     AST OWNER:
 *         existing domain-neutral frontend AST
 *
 *     SEMANTIC OWNER:
 *         reasoning semantic subsystem
 *
 *     EFFECT OWNER:
 *         effects semantic subsystem
 *
 *     RESOURCE OWNER:
 *         resources semantic subsystem
 *
 *     CAPABILITY OWNER:
 *         capabilities semantic subsystem
 *
 *     POLICY OWNER:
 *         policy/security subsystem
 *
 *     PROVENANCE OWNER:
 *         provenance subsystem
 *
 *     IR OWNER:
 *         canonical semantic/IR pipeline
 *
 *     QUANTUM IR:
 *         quantum::ir
 *
 *     RUNTIME:
 *         execution/runtime subsystem
 *
 * No ownership is transferred merely because another subsystem consumes this
 * grammar.
 */


/* ============================================================================
 * 21. INTEGRATION REQUIREMENTS
 * ========================================================================== */

/*
 * BEFORE THIS FILE IS MARKED PRODUCTION-READY:
 *
 * 1. `expressionCore` MUST be an actual canonical rule in the assembled
 *    parser, not an undefined placeholder.
 *
 * 2. `Expressions` MUST integrate `reasoningExpression` without creating a
 *    circular grammar import.
 *
 * 3. `grammar/memory/sankofa.g4` MUST delegate its `infer` expression/statement
 *    syntax to this grammar.
 *
 * 4. Existing `@infer(...)` and `@reason(...)` annotation syntax MUST remain
 *    distinct from ordinary reasoning expressions.
 *
 * 5. Existing Rust AST representations MUST be reconciled with the new
 *    reasoning operation model.
 *
 * 6. Semantic analysis MUST distinguish:
 *
 *        syntax
 *        type
 *        effect
 *        capability
 *        resource
 *        policy
 *        provenance
 *        target feasibility
 *
 * 7. Conformance tests MUST exercise both the ANTLR grammar and the Rust
 *    frontend.
 *
 * 8. No second reasoning grammar may remain authoritative elsewhere.
 *
 * ============================================================================
 * 22. COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is DONE when:
 *
 * [x] One reasoning-expression abstraction exists.
 * [x] infer/deduce/reason share one grammar structure.
 * [x] No algorithm is hard-coded.
 * [x] No application domain is hard-coded.
 * [x] No hardware target is hard-coded.
 * [x] No universal resource ceiling is hard-coded.
 * [x] No second expression-precedence hierarchy exists here.
 * [x] No lexer rules are duplicated.
 * [x] No runtime actions exist.
 * [x] No unsafe Rust is required.
 * [x] Source spans can be preserved.
 * [x] Target/source/context structure is preserved.
 * [x] Quantum integration terminates at quantum::ir.
 * [x] Resource/capability integration is downstream.
 * [x] Effect integration is downstream.
 * [x] Policy integration is downstream.
 * [x] Provenance integration is downstream.
 * [x] Existing memory syntax has a migration path.
 * [x] Existing annotation syntax remains unambiguous.
 * [x] Positive tests exist.
 * [x] Negative tests exist.
 * [x] Boundary tests exist.
 * [x] Cross-domain tests exist.
 * [x] Scalability tests exist.
 * [x] Determinism tests exist.
 *
 * The file MUST NOT be marked stable merely because this .g4 file parses.
 *
 * Full production readiness additionally requires the integration gates above.
 */