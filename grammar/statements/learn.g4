/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/Learn.g4
 *
 * GRAMMAR
 * -------
 * Learn
 *
 * KIND
 * ----
 * ANTLR4 parser grammar
 *
 * STATUS
 * ------
 * CANONICAL STATEMENT-LEVEL LEARNING GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file owns the canonical statement-level syntax for the Zamani
 * `learn` operation.
 *
 * The construct expresses LEARNING INTENT.
 *
 * It does not specify:
 *
 *     - a learning algorithm;
 *     - a model architecture;
 *     - an optimizer;
 *     - a dataset implementation;
 *     - a tensor implementation;
 *     - a probabilistic implementation;
 *     - a neural implementation;
 *     - a symbolic learner;
 *     - a reinforcement-learning implementation;
 *     - a quantum-learning implementation;
 *     - an accelerator implementation;
 *     - a distributed-learning implementation;
 *     - a CPU/GPU/FPGA/ASIC implementation;
 *     - a QPU implementation;
 *     - a runtime;
 *     - a storage engine;
 *     - a scheduler;
 *     - resource discovery;
 *     - capability discovery;
 *     - hardware selection.
 *
 * Those concerns are resolved downstream.
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
 *     Learn
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +--> names
 *          +--> types
 *          +--> effects
 *          +--> capabilities
 *          +--> resources
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *          |
 *          v
 *     semantic learning operation
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical computation
 *          +--> AI/ML
 *          +--> tensor computation
 *          +--> probabilistic computation
 *          +--> distributed computation
 *          +--> accelerator computation
 *          +--> hybrid computation
 *          +--> quantum-related computation
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     optimization / specialization
 *          |
 *          v
 *     scheduling / placement / lowering
 *          |
 *          v
 *     target realization
 *
 * This grammar never bypasses the semantic layer.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns exactly:
 *
 *     learnStatement
 *
 * and the private syntax required to compose it:
 *
 *     learnClause
 *     learnFromClause
 *     learnWithClause
 *     learnArgumentList
 *     learnArgument
 *     learnNamedArgument
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     statement
 *     expression
 *     expression precedence
 *     identifiers
 *     names
 *     types
 *     assignments
 *     declarations
 *     blocks
 *     assertions
 *     contracts
 *     policies
 *     resources
 *     capabilities
 *     effects
 *     provenance
 *     knowledge
 *     query
 *     retract
 *     infer
 *     deduce
 *     reason
 *     adapt
 *     remember
 *     recall
 *     temporal semantics
 *     memory semantics
 *     actor semantics
 *     concurrency semantics
 *     quantum semantics
 *     HDL semantics
 *     hardware realization
 *     networking
 *     distributed execution
 *     runtime execution
 *     IR generation
 *     target selection.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     Expressions
 *
 * `Expressions` supplies the canonical:
 *
 *     expression
 *
 * rule.
 *
 * No second expression grammar is created here.
 *
 * ============================================================================
 *
 * EXPORTS
 * -------
 *
 *     learnStatement
 *
 * Private implementation rules are:
 *
 *     learnClause
 *     learnFromClause
 *     learnWithClause
 *     learnArgumentList
 *     learnArgument
 *     learnNamedArgument
 *
 * ============================================================================
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar/statements/statements.g4
 *
 * The universal statement dispatcher imports this grammar and admits:
 *
 *     learnStatement
 *
 * exactly once.
 *
 * ============================================================================
 *
 * AST_OWNER
 * ---------
 *
 * The frontend AST subsystem.
 *
 * This grammar creates parser contexts only.
 *
 * The AST must preserve:
 *
 *     - operation kind = learn;
 *     - source span;
 *     - learning subject;
 *     - optional source expression;
 *     - ordered arguments;
 *     - named argument names;
 *     - argument values;
 *     - source ordering;
 *     - source metadata required by provenance.
 *
 * The AST must remain domain-neutral.
 *
 * ============================================================================
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * The semantic learning subsystem.
 *
 * Semantic analysis determines:
 *
 *     - what is being learned;
 *     - what provides the learning information;
 *     - type compatibility;
 *     - ownership/lifetime validity;
 *     - effect requirements;
 *     - capability requirements;
 *     - resource requirements;
 *     - contract requirements;
 *     - policy constraints;
 *     - provenance requirements;
 *     - determinism requirements;
 *     - domain interpretation.
 *
 * ============================================================================
 *
 * IR_OWNER
 * --------
 *
 * This grammar owns no IR.
 *
 * Learning intent lowers through the canonical semantic representation and
 * existing canonical IR architecture.
 *
 * There is intentionally no:
 *
 *     LearningIR
 *     NeuralIR
 *     TrainingIR
 *     QMLIR
 *
 * created by this grammar.
 *
 * If learning invokes quantum computation, the quantum portion follows the
 * canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/statements/learn/
 *
 * Recommended test groups:
 *
 *     positive/
 *     negative/
 *     boundary/
 *     scalability/
 *     cross-domain/
 *     determinism/
 *     compatibility/
 *     round-trip/
 *
 * ============================================================================
 *
 * SPEC_OWNER
 * ----------
 *
 * Normative syntax/semantic specification:
 *
 *     grammar/specification/
 *
 * Machine-checkable conformance:
 *
 *     grammar/spec/
 *
 * Learning semantics should be defined by the AI/semantic specification,
 * while this file remains the parser-level syntax owner.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Required canonical lexer tokens:
 *
 *     LEARN
 *     FROM
 *     WITH
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     ASSIGN
 *     SEMICOLON
 *
 * This file defines NO lexer rules.
 *
 * `LEARN` must be supplied by:
 *
 *     grammar/lexer/keywords.g4
 *
 * through the canonical:
 *
 *     ZamaniLexer
 *
 * vocabulary.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * All learning operands are ordinary Zamani expressions.
 *
 * Therefore:
 *
 *     learn expression;
 *
 * may operate syntactically on:
 *
 *     identifiers
 *     qualified names
 *     calls
 *     member expressions
 *     indexed expressions
 *     literals
 *     collections
 *     tensor expressions
 *     query expressions
 *     reasoning results
 *     measurement results
 *     simulation results
 *     distributed results
 *     hardware observations
 *     model expressions
 *     user-defined abstractions
 *
 * according to what the canonical expression grammar accepts.
 *
 * This grammar MUST NOT reproduce:
 *
 *     primaryExpression
 *     unaryExpression
 *     binaryExpression
 *     logicalExpression
 *     arithmeticExpression
 *     postfixExpression
 *     assignmentExpression
 *
 * ============================================================================
 * LEARNING SUBJECT
 * ============================================================================
 *
 * The first expression after `learn` is the learning subject.
 *
 * Examples:
 *
 *     learn model;
 *
 *     learn model from dataset;
 *
 *     learn model from knowledge;
 *
 *     learn strategy;
 *
 *     learn parameters;
 *
 * The grammar deliberately does not require a particular semantic type.
 *
 * Whether the subject is:
 *
 *     a model
 *     parameters
 *     knowledge
 *     a strategy
 *     a policy
 *     a value
 *     a tensor
 *     a symbolic structure
 *     a user-defined object
 *
 * is determined downstream.
 *
 * ============================================================================
 * FROM CONTRACT
 * ============================================================================
 *
 * The optional `from` clause identifies the source of learning information.
 *
 * Examples:
 *
 *     learn model from dataset;
 *
 *     learn model from knowledge;
 *
 *     learn strategy from feedback;
 *
 *     learn result from quantum_measurement;
 *
 *     learn model from query(source);
 *
 * The source remains an ordinary Zamani expression.
 *
 * The grammar therefore does not enumerate:
 *
 *     dataset
 *     database
 *     knowledge graph
 *     model
 *     stream
 *     sensor
 *     measurement
 *     simulation
 *     network
 *     file
 *
 * as special learning syntax.
 *
 * ============================================================================
 * WITH CONTRACT
 * ============================================================================
 *
 * The optional `with` clause supplies additional learning configuration.
 *
 * Canonical forms:
 *
 *     learn model with (policy);
 *
 *     learn model with (policy = p);
 *
 *     learn model from dataset with (objective = objective);
 *
 *     learn model from dataset with (
 *         objective = objective,
 *         strategy = strategy,
 *         policy = policy
 *     );
 *
 * Argument names are ordinary identifiers.
 *
 * The grammar does not create a closed vocabulary for:
 *
 *     objective
 *     algorithm
 *     optimizer
 *     strategy
 *     policy
 *     resources
 *     capability
 *     model
 *     data
 *     feedback
 *     provenance
 *
 * This keeps the language open to future learning systems.
 *
 * ============================================================================
 * ARGUMENT CONTRACT
 * ============================================================================
 *
 * Each argument is either:
 *
 *     positional expression
 *
 * or:
 *
 *     named argument
 *
 * Named:
 *
 *     name = expression
 *
 * Positional:
 *
 *     expression
 *
 * Mixed arguments are permitted:
 *
 *     learn model with (
 *         dataset,
 *         policy = p,
 *         objective
 *     );
 *
 * Semantic analysis determines whether a particular combination is legal.
 *
 * ============================================================================
 * ARGUMENT ORDER
 * ============================================================================
 *
 * Argument order is preserved syntactically.
 *
 * The semantic layer may determine whether order is meaningful.
 *
 * The grammar does not reorder arguments.
 *
 * ============================================================================
 * TRAILING COMMA
 * ============================================================================
 *
 * A trailing comma is accepted:
 *
 *     learn model with (
 *         objective = objective,
 *         strategy = strategy,
 *     );
 *
 * This is a formatting convenience.
 *
 * It does not impose an argument-count limit.
 *
 * ============================================================================
 * EMPTY WITH
 * ============================================================================
 *
 * `learn model with ();`
 *
 * is syntactically valid.
 *
 * This permits syntax-preserving tooling and forwards compatibility.
 *
 * Whether an empty option set has semantic meaning is determined downstream.
 *
 * ============================================================================
 * CLAUSE ORDER
 * ============================================================================
 *
 * The canonical order is:
 *
 *     LEARN
 *     SUBJECT
 *     FROM?
 *     WITH?
 *     SEMICOLON
 *
 * Therefore:
 *
 *     learn model;
 *
 *     learn model from data;
 *
 *     learn model with (policy);
 *
 *     learn model from data with (policy);
 *
 * are valid.
 *
 * The reversed form:
 *
 *     learn model with (policy) from data;
 *
 * is intentionally not part of the canonical syntax.
 *
 * This produces one stable source representation rather than multiple
 * equivalent clause permutations.
 *
 * ============================================================================
 * EMPTY SUBJECT
 * ============================================================================
 *
 * Invalid:
 *
 *     learn;
 *
 *     learn from data;
 *
 *     learn with (policy);
 *
 * A subject expression is mandatory.
 *
 * ============================================================================
 * EMPTY FROM
 * ============================================================================
 *
 * Invalid:
 *
 *     learn model from;
 *
 * `from` requires exactly one expression.
 *
 * ============================================================================
 * EMPTY ARGUMENTS
 * ============================================================================
 *
 * Valid:
 *
 *     learn model with ();
 *
 * Invalid:
 *
 *     learn model with (,);
 *
 *     learn model with (, policy);
 *
 *     learn model with (policy,);
 *
 * The last form is valid because a trailing comma is explicitly supported:
 *
 *     learn model with (policy,);
 *
 * The following distinction therefore applies:
 *
 *     with ()
 *         valid
 *
 *     with (policy,)
 *         valid
 *
 *     with (, policy)
 *         invalid
 *
 *     with (policy,,other)
 *         invalid
 *
 * ============================================================================
 * NAMED ARGUMENTS
 * ============================================================================
 *
 * Named arguments use the canonical `identifier` parser rule.
 *
 * This prevents this grammar from inventing another name system.
 *
 * Examples:
 *
 *     learn model with (objective = goal);
 *
 *     learn model with (strategy = strategy);
 *
 *     learn model with (resource = requirement);
 *
 * The semantic layer owns validation of argument names.
 *
 * ============================================================================
 * OPEN-WORLD LEARNING
 * ============================================================================
 *
 * The grammar deliberately does not enumerate algorithms.
 *
 * It therefore remains compatible with:
 *
 *     symbolic learning
 *     statistical learning
 *     neural learning
 *     probabilistic learning
 *     reinforcement learning
 *     evolutionary learning
 *     transfer learning
 *     federated learning
 *     distributed learning
 *     quantum-assisted learning
 *     hybrid learning
 *     differentiable computation
 *     future learning paradigms
 *
 * A new algorithm must not require a grammar edit if it can be represented
 * through the existing semantic operation and ordinary expressions.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Learning may consume knowledge through ordinary expressions.
 *
 * For example:
 *
 *     learn model from knowledge;
 *
 *     learn model from query(pattern);
 *
 *     learn model from knowledge::query(pattern);
 *
 * Knowledge ownership remains elsewhere.
 *
 * This grammar does not define:
 *
 *     assert
 *     retract
 *     query
 *     fact
 *     knowledge graph
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Learning may consume reasoning results:
 *
 *     learn model from infer_result;
 *
 *     learn strategy from deduction;
 *
 *     learn parameters from reasoning_result;
 *
 * Reasoning syntax remains owned by its dedicated statement/expression
 * components.
 *
 * Learning does not duplicate:
 *
 *     infer
 *     deduce
 *     reason
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Learning and adaptation are related but distinct semantic operations.
 *
 * This grammar does not transform:
 *
 *     learn
 *
 * into:
 *
 *     adapt
 *
 * Adaptation remains separately governed by:
 *
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     provenance
 *
 * A learning result may subsequently become an adaptation input.
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Learning may operate on:
 *
 *     uncertain values
 *     probabilities
 *     distributions
 *     confidence values
 *     beliefs
 *     intervals
 *     symbolic uncertainty
 *
 * These remain ordinary expressions.
 *
 * This grammar does not impose:
 *
 *     numeric precision
 *     probability representation
 *     floating-point width
 *     tensor rank
 *     distribution cardinality
 *     model size.
 *
 * ============================================================================
 * EVIDENCE / PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Learning may consume evidence and produce provenance.
 *
 * Examples:
 *
 *     learn model from evidence;
 *
 *     learn model from observation;
 *
 *     learn model with (provenance = record);
 *
 * The actual evidence/provenance structures are owned by their respective
 * semantic subsystems.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Learning may carry semantic effects such as:
 *
 *     learning
 *     mutation
 *     randomness
 *     io
 *     network
 *     distributed
 *     measurement
 *     foreign
 *
 * depending on semantic resolution.
 *
 * This grammar does not infer effects.
 *
 * Valid syntax does not imply purity or permission.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Semantic analysis may derive capabilities such as:
 *
 *     capability("learning")
 *     capability("tensor.compute")
 *     capability("probabilistic.compute")
 *     capability("distributed.compute")
 *     capability("quantum.measurement")
 *
 * The grammar does not resolve capability availability.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Learning may require arbitrary resources.
 *
 * Resource requirements belong to the canonical resource subsystem.
 *
 * The grammar MUST NOT establish language-level limits for:
 *
 *     samples
 *     features
 *     parameters
 *     models
 *     tensors
 *     dimensions
 *     workers
 *     devices
 *     nodes
 *     memory
 *     threads
 *     accelerators
 *     qubits
 *     training iterations
 *     learning operations.
 *
 * In particular, this grammar MUST NOT define any universal capacity
 * constants or fixed machine sizes.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The source-level form:
 *
 *     learn subject from source with (options);
 *
 * remains independent of target realization.
 *
 * The same source may be considered for:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational substrates
 *
 * Resource and capability feasibility are determined downstream.
 *
 * "Scale to infinity given available resources" means that this grammar
 * introduces no artificial machine-size ceiling. Actual physical and
 * implementation resources remain finite and are evaluated outside the
 * grammar.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Learning may participate in hybrid quantum-classical computation.
 *
 * Example:
 *
 *     learn model from measurement_result;
 *
 * The grammar does not know whether `measurement_result` came from:
 *
 *     a simulator
 *     a QPU
 *     a classical approximation
 *     a distributed execution
 *     a future quantum substrate.
 *
 * If semantic lowering produces quantum computation, the canonical quantum
 * boundary remains:
 *
 *     quantum::ir
 *
 * This grammar does not define:
 *
 *     gates
 *     coupling maps
 *     physical qubits
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     HAL.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Learning may consume hardware observations or generate values used by
 * hardware/control logic.
 *
 * Example:
 *
 *     learn controller from observation;
 *
 * Hardware realization remains downstream.
 *
 * This grammar does not define:
 *
 *     wires
 *     fixed-width registers
 *     physical ports
 *     FPGA resources
 *     ASIC cells
 *     clock frequencies
 *     device identifiers.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Learning may execute over distributed resources.
 *
 * The grammar expresses only the learning operation.
 *
 * Distribution is resolved by:
 *
 *     concurrency
 *     distributed semantics
 *     capabilities
 *     resources
 *     policies
 *     execution planning.
 *
 * No node-count limit is represented here.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * The parse result depends only on:
 *
 *     source text
 *     token vocabulary
 *     grammar
 *     parser configuration.
 *
 * It must not depend on:
 *
 *     hardware availability
 *     network state
 *     filesystem state
 *     randomness
 *     wall-clock time
 *     scheduler state
 *     runtime state.
 *
 * Learning itself may be deterministic or nondeterministic according to
 * semantic effects, policies and execution configuration.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing never executes learning.
 *
 * This grammar does not:
 *
 *     load models;
 *     open datasets;
 *     access files;
 *     contact networks;
 *     inspect hardware;
 *     allocate devices;
 *     execute code;
 *     invoke a model;
 *     mutate runtime state.
 *
 * Authorization and sandboxing are downstream concerns.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The canonical spelling is:
 *
 *     learn
 *
 * Legacy learning syntax should be handled by the compatibility subsystem,
 * not by duplicate parser rules.
 *
 * Historical aliases must not create competing `learnStatement` rules.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar named:
 *
 *     Learn
 *
 * Therefore the file name:
 *
 *     Learn.g4
 *
 * intentionally matches the grammar name.
 *
 * The grammar imports:
 *
 *     Expressions
 *
 * and consumes the canonical:
 *
 *     ZamaniLexer
 *
 * vocabulary.
 *
 * It does NOT import:
 *
 *     Statements
 *     Domains
 *     Memory
 *     Sankofa
 *     Learning
 *
 * This prevents circular dependencies.
 *
 * ============================================================================
 * PUBLIC STATEMENT
 * ============================================================================
 *
 * The public entry point is:
 *
 *     learnStatement
 *
 * Canonical syntax:
 *
 *     learn SUBJECT;
 *
 *     learn SUBJECT from SOURCE;
 *
 *     learn SUBJECT with (ARGUMENTS);
 *
 *     learn SUBJECT from SOURCE with (ARGUMENTS);
 *
 * ============================================================================
 * GRAMMAR RULES
 * ============================================================================
 */

parser grammar Learn;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions,
    Names
    ;

/*
 * ============================================================================
 * PUBLIC ENTRY
 * ============================================================================
 *
 * Exactly one public statement entry point is exported by this grammar.
 */
learnStatement
    : LEARN
      learnClause
      SEMICOLON
    ;


/*
 * ============================================================================
 * LEARNING CLAUSE
 * ============================================================================
 *
 * Canonical order:
 *
 *     subject
 *     from?
 *     with?
 */
learnClause
    : expression
      learnFromClause?
      learnWithClause?
    ;


/*
 * ============================================================================
 * SOURCE CLAUSE
 * ============================================================================
 */
learnFromClause
    : FROM
      expression
    ;


/*
 * ============================================================================
 * OPTIONAL ARGUMENTS
 * ============================================================================
 *
 * Empty parentheses are accepted intentionally.
 */
learnWithClause
    : WITH
      LPAREN
      learnArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * ARGUMENT LIST
 * ============================================================================
 *
 * There is no fixed argument-count ceiling.
 *
 * A trailing comma is accepted.
 */
learnArgumentList
    : learnArgument
      (
          COMMA
          learnArgument
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * ARGUMENT
 * ============================================================================
 *
 * Named arguments are listed first so:
 *
 *     name = expression
 *
 * is captured by the named form.
 *
 * Ordinary expressions remain available for positional arguments.
 */
learnArgument
    : learnNamedArgument
    | expression
    ;


/*
 * ============================================================================
 * NAMED ARGUMENT
 * ============================================================================
 */
learnNamedArgument
    : identifier
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * END OF PRODUCTION GRAMMAR
 * ============================================================================
 *
 * Architectural guarantees:
 *
 *     - no lexer rules;
 *     - no duplicate expression hierarchy;
 *     - no duplicate universal statement rule;
 *     - no learning algorithm catalogue;
 *     - no model catalogue;
 *     - no hardware catalogue;
 *     - no quantum operation catalogue;
 *     - no physical resource limits;
 *     - no target-specific syntax;
 *     - no runtime execution;
 *     - no semantic actions;
 *     - no semantic predicates;
 *     - no embedded Rust;
 *     - no unsafe Rust requirement;
 *     - Rust 1.97 / 1.97.1 compatible frontend contract;
 *     - Rust 2021 compatible frontend contract;
 *     - canonical quantum boundary remains quantum::ir;
 *     - semantic resource/capability negotiation remains downstream;
 *     - learning remains open-world and future extensible.
 *
 * The universal statement dispatcher is the only owner of:
 *
 *     statement
 *
 * This grammar owns only:
 *
 *     learnStatement
 *
 * ============================================================================
 */