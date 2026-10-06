/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/quantum-inference.g4
 *
 * Grammar:
 *     QuantumInference
 *
 * Status:
 *     CANONICAL QUANTUM-INFERENCE INTEGRATION GRAMMAR
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97+
 *     Rust 2021 edition
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX BOUNDARY for inference that
 * participates explicitly in quantum computation.
 *
 * It does NOT create a second inference language.
 *
 * It does NOT replace:
 *
 *     grammar/ai/inference.g4
 *
 * It does NOT replace:
 *
 *     grammar/expressions/reasoning.g4
 *
 * It does NOT replace:
 *
 *     grammar/expressions/expressions.g4
 *
 * It does NOT replace:
 *
 *     grammar/quantum/operations.g4
 *
 * It does NOT create a quantum-specific IR.
 *
 * The purpose of this grammar is to make quantum inference a first-class
 * compositional boundary while preserving the repository's single-authority
 * architecture.
 *
 * The intended conceptual pipeline is:
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
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +-------------------------------+
 *          |               |               |
 *          v               v               v
 *        types          effects        provenance
 *          |               |               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *                 semantic inference model
 *                          |
 *              +-----------+-----------+
 *              |                       |
 *              v                       v
 *      classical semantics      quantum semantics
 *                                      |
 *                                      v
 *                                  quantum::ir
 *                                      |
 *                  +-------------------+-------------------+
 *                  |                   |                   |
 *                  v                   v                   v
 *             optimization         routing            scheduling
 *                                                          |
 *                                                          v
 *                                               resilience / QEC
 *                                                          |
 *                                                          v
 *                                                         ZQN
 *                                                          |
 *                                                          v
 *                                                         HAL
 *                                                          |
 *                                                          v
 *                                                   target realization
 *
 * ============================================================================
 * FILE CONTRACT
 * ============================================================================
 *
 * OWNS
 * ----
 *
 * This file owns:
 *
 *     quantumInferenceConstruct
 *     quantumInferenceStatement
 *     quantumInferenceInvocation
 *     quantumInferenceTarget
 *     quantumInferenceSource
 *     quantumInferenceContext
 *     quantumInferenceOptionList
 *     quantumInferenceOption
 *     quantumInferenceBinding
 *     quantumInferenceResultBinding
 *     quantumInferenceInputBinding
 *     quantumInferenceQuantumContext
 *     quantumInferenceClassicalContext
 *     quantumInferenceExtension
 *
 * DOES NOT OWN
 * -------------
 *
 * This file does not own:
 *
 *     lexer rules
 *     keywords
 *     identifiers
 *     qualified names
 *     general expressions
 *     expression precedence
 *     general reasoning
 *     general AI inference
 *     models
 *     datasets
 *     tensors
 *     learning
 *     training
 *     knowledge
 *     probability
 *     uncertainty
 *     evidence
 *     provenance
 *     policies
 *     contracts
 *     effects
 *     capabilities
 *     resource requirements
 *     quantum operations
 *     quantum measurement
 *     quantum registers
 *     quantum states
 *     quantum types
 *     dynamic circuits
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * Repository architecture:
 *
 *     grammar/DESIGN.md
 *
 * Quantum specification:
 *
 *     grammar/spec/quantum.md
 *
 * AI specification:
 *
 *     grammar/spec/ai.md
 *
 * Syntax specification:
 *
 *     grammar/spec/syntax.md
 *
 * Quantum subsystem composition:
 *
 *     grammar/quantum/quantum.g4
 *
 * General expression composition:
 *
 *     grammar/expressions/expressions.g4
 *
 * General reasoning:
 *
 *     grammar/expressions/reasoning.g4
 *
 * General inference:
 *
 *     grammar/ai/inference.g4
 *
 * Quantum operations:
 *
 *     grammar/quantum/operations.g4
 *
 * Canonical frontend AST:
 *
 *     src/frontend/ast/
 *
 * Canonical quantum semantic/IR boundary:
 *
 *     quantum::ir
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     ZamaniLexer
 *     Expressions
 *
 * The dependency on Expressions is intentional.
 *
 * This grammar consumes the canonical:
 *
 *     expression
 *
 * rule rather than defining another expression hierarchy.
 *
 * It therefore inherits the repository's single expression precedence model.
 *
 *
 * EXPORTS
 * -------
 *
 *     quantumInferenceConstruct
 *     quantumInferenceStatement
 *     quantumInferenceInvocation
 *
 * Secondary rules are local implementation details unless explicitly listed
 * above.
 *
 *
 * CONSUMED_BY
 * ----------
 *
 *     grammar/quantum/quantum.g4
 *
 * The quantum composition root should dispatch quantum inference through:
 *
 *     quantumInferenceConstruct
 *
 *
 * AST_OWNER
 * ---------
 *
 *     src/frontend/ast/
 *
 * The AST adapter must preserve the structural information required by
 * semantic analysis without creating a quantum-specific inference IR.
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Quantum inference semantic analysis, composed with:
 *
 *     type analysis
 *     reasoning analysis
 *     model analysis
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     contract analysis
 *     policy analysis
 *     provenance
 *     portability analysis
 *
 *
 * IR_OWNER
 * --------
 *
 * No IR is owned by this grammar.
 *
 * Quantum execution semantics ultimately cross:
 *
 *     quantum::ir
 *
 * Classical inference semantics remain in the canonical classical semantic
 * pipeline.
 *
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/quantum/inference/
 *
 * Recommended test groups:
 *
 *     invocation/
 *     target/
 *     source/
 *     context/
 *     bindings/
 *     quantum-classical/
 *     expressions/
 *     resources/
 *     capabilities/
 *     policies/
 *     provenance/
 *     scalability/
 *     negative/
 *     compatibility/
 *     determinism/
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file defines NO lexer rules.
 *
 * It consumes the canonical Zamani lexer vocabulary:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The following lexical concepts are therefore supplied by the repository:
 *
 *     INFER
 *     FROM
 *     WITH
 *     AS
 *     AT
 *     IDENTIFIER
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     COLON
 *     ASSIGN
 *     SEMICOLON
 *
 * The exact token names are determined by the current canonical lexer.
 *
 * This grammar MUST NOT introduce:
 *
 *     quantum-specific lexer rules
 *     quantum operation keyword catalogs
 *     model-name keyword catalogs
 *     hardware keyword catalogs
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Quantum inference is intentionally open-ended.
 *
 * The grammar does NOT enumerate:
 *
 *     neural architectures
 *     inference algorithms
 *     quantum machine-learning algorithms
 *     model vendors
 *     model formats
 *     quantum providers
 *     accelerator types
 *     QPU names
 *     simulator names
 *     observables
 *     gate sets
 *     decomposition strategies
 *     routing algorithms
 *     scheduling algorithms
 *
 * Such concepts are semantic data, library definitions, dialect extensions,
 * capability declarations, policies, or runtime providers.
 *
 * ============================================================================
 * QUANTUM INFERENCE MODEL
 * ============================================================================
 *
 * A quantum inference invocation conceptually contains:
 *
 *     inference identity
 *     target/result
 *     source/input
 *     optional context
 *     optional quantum context
 *     optional classical context
 *     optional named bindings
 *     optional extensible options
 *
 * Conceptually:
 *
 *     QuantumInference {
 *         target
 *         source?
 *         context*
 *         bindings*
 *         extensions*
 *     }
 *
 * The exact AST and semantic representation is owned downstream.
 *
 * ============================================================================
 * WHY THIS IS DISTINCT FROM GENERAL INFERENCE
 * ============================================================================
 *
 * The repository already contains:
 *
 *     grammar/ai/inference.g4
 *
 * That grammar owns general inference declarations and invocations.
 *
 * This file must therefore not duplicate those constructs.
 *
 * The distinction is:
 *
 *     general inference
 *         -> grammar/ai/inference.g4
 *
 *     general reasoning expression
 *         -> grammar/expressions/reasoning.g4
 *
 *     quantum-specific inference boundary
 *         -> this file
 *
 * A semantic analyzer may normalize all three into a common inference
 * semantic representation.
 *
 * The parser architecture must nevertheless preserve their source ownership.
 *
 * ============================================================================
 * QUANTUM / CLASSICAL BOUNDARY
 * ============================================================================
 *
 * Quantum inference may consume values originating from:
 *
 *     quantum operations
 *     measurements
 *     quantum state expressions
 *     observables
 *     classical expressions
 *     tensors
 *     models
 *     datasets
 *     distributed values
 *     hardware-independent data
 *
 * These are represented using the canonical expression system.
 *
 * This file therefore does NOT define:
 *
 *     quantumExpression
 *     classicalExpression
 *     tensorExpression
 *     modelExpression
 *
 * as competing expression systems.
 *
 * ============================================================================
 * QUANTUM OPERATION BOUNDARY
 * ============================================================================
 *
 * Quantum operation syntax remains owned by:
 *
 *     grammar/quantum/operations.g4
 *
 * Therefore this grammar does not enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     RX
 *     RY
 *     RZ
 *     SWAP
 *
 * or any future operation.
 *
 * A quantum operation may appear as part of a canonical expression or as
 * surrounding quantum computation, depending on the established quantum
 * parser composition.
 *
 * Its semantic representation ultimately crosses:
 *
 *     quantum::ir
 *
 * ============================================================================
 * MEASUREMENT BOUNDARY
 * ============================================================================
 *
 * Measurement syntax remains owned by:
 *
 *     grammar/quantum/measurement.g4
 *
 * Measurement results may be supplied to inference through ordinary
 * expressions.
 *
 * For example, the semantic intent may correspond to:
 *
 *     infer classifier(measurement);
 *
 * or:
 *
 *     infer result from measurement;
 *
 * without this file defining a second measurement grammar.
 *
 * Whether a target supports mid-circuit measurement, streaming measurement,
 * repeated sampling, or another execution model is a semantic capability
 * question.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY BOUNDARY
 * ============================================================================
 *
 * Resource requirements and capabilities are NOT redefined here.
 *
 * They remain owned by:
 *
 *     grammar/resources/
 *     grammar/quantum/quantum-resources.g4
 *     grammar/quantum/quantum-capabilities.g4
 *
 * This grammar merely preserves the source structure necessary for semantic
 * analysis.
 *
 * Therefore an implementation may associate quantum inference with semantic
 * requirements such as:
 *
 *     capability("quantum.measurement")
 *     capability("quantum.dynamic_control")
 *     capability("quantum.inference")
 *     capability("tensor.compute")
 *
 * without requiring those capability names to become grammar productions.
 *
 * Likewise:
 *
 *     requires qubits >= required_qubits;
 *
 *     requires memory >= required_memory;
 *
 *     requires topology(required_topology);
 *
 * remain owned by the common resource/requirement subsystem.
 *
 * ============================================================================
 * EFFECT BOUNDARY
 * ============================================================================
 *
 * Quantum inference can semantically involve effects such as:
 *
 *     measurement
 *     randomness
 *     learning
 *     external_model_access
 *     network
 *     distributed
 *     foreign
 *     adaptation
 *
 * This grammar does not define those effects.
 *
 * Effect inference and validation remain downstream.
 *
 * ============================================================================
 * POLICY BOUNDARY
 * ============================================================================
 *
 * Inference may be affected by:
 *
 *     execution policies
 *     model policies
 *     resource policies
 *     security policies
 *     adaptation policies
 *     reproducibility policies
 *
 * This grammar does not define policy semantics.
 *
 * Policy expressions or statements supplied by surrounding Zamani syntax
 * remain responsible for those declarations.
 *
 * ============================================================================
 * PROVENANCE BOUNDARY
 * ============================================================================
 *
 * Quantum inference may need provenance for:
 *
 *     source data
 *     measurement data
 *     model selection
 *     transformation
 *     inference result
 *     compilation decisions
 *     target realization
 *
 * This grammar preserves the source structure but does not implement
 * provenance.
 *
 * Provenance is attached downstream by the semantic/compilation pipeline.
 *
 * ============================================================================
 * CONTRACT BOUNDARY
 * ============================================================================
 *
 * Contracts such as:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * remain owned by the validation/contract subsystem.
 *
 * This grammar must not duplicate their syntax.
 *
 * A quantum inference construct can therefore participate in contracts
 * through the surrounding canonical statement/declaration architecture.
 *
 * ============================================================================
 * PORTABILITY
 * ============================================================================
 *
 * The source representation is target-independent.
 *
 * The same semantic quantum inference may eventually be realized through:
 *
 *     classical simulation
 *     tensor simulation
 *     accelerator execution
 *     hybrid execution
 *     QPU execution
 *     distributed execution
 *     future execution substrates
 *
 * without changing the source construct.
 *
 * The compiler may specialize the implementation according to:
 *
 *     target capabilities
 *     resource availability
 *     execution policies
 *     performance constraints
 *     reliability requirements
 *     numerical requirements
 *     quantum capabilities
 *
 * The grammar itself does not choose the realization.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar introduces NO universal finite limits.
 *
 * It does not limit:
 *
 *     inference operations
 *     input count
 *     source expression size
 *     target expression size
 *     context count
 *     parameter count
 *     model count
 *     qubit count
 *     logical qubit count
 *     physical qubit count
 *     circuit width
 *     circuit depth
 *     number of measurements
 *     number of samples
 *     tensor rank
 *     tensor dimensions
 *     number of devices
 *     number of nodes
 *     number of workers
 *     memory
 *     storage
 *
 * Repetition is represented structurally through ANTLR repetition operators
 * or through source-level expressions.
 *
 * "Infinity" therefore means that the grammar introduces no artificial
 * language-level finite ceiling. Physical execution remains bounded only by
 * actual implementation and available resources.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source text
 *     canonical lexer
 *     grammar version
 *     explicitly selected parser configuration
 *     explicitly selected dialect configuration
 *
 * Parsing must not inspect:
 *
 *     hardware
 *     QPU availability
 *     memory availability
 *     filesystem state
 *     network state
 *     runtime state
 *     wall-clock time
 *     random state
 *     calibration state
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust
 *     no target-language actions
 *     no semantic predicates
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no runtime execution
 *     no generated-code execution
 *
 * The Rust implementation consuming this grammar remains based on:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar QuantumInference;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/*
 * ============================================================================
 * 1. PUBLIC CONSTRUCT
 * ============================================================================
 *
 * This is the single public quantum-inference entry point.
 *
 * The surrounding quantum composition grammar decides whether this construct
 * is legal in a declaration, statement, block, kernel, circuit, or other
 * quantum context.
 *
 * This grammar does not consume the surrounding semicolon unless the public
 * quantum statement rule explicitly requires it.
 */

quantumInferenceConstruct
    : quantumInferenceStatement
    | quantumInferenceInvocation
    ;


/*
 * ============================================================================
 * 2. STATEMENT FORM
 * ============================================================================
 *
 * The statement form provides a convenient source-level quantum inference
 * operation.
 *
 * The terminator belongs here because this rule is explicitly a statement
 * boundary.
 */

quantumInferenceStatement
    : quantumInferenceInvocation SEMICOLON
    ;


/*
 * ============================================================================
 * 3. INVOCATION
 * ============================================================================
 *
 * Canonical structural forms include:
 *
 *     infer target;
 *
 *     infer target from source;
 *
 *     infer target with (context);
 *
 *     infer target from source with (context);
 *
 *     infer target as binding from source;
 *
 *     infer target from source with (quantum_context, classical_context);
 *
 * The actual meaning of the target/source/context is determined semantically.
 */

quantumInferenceInvocation
    : INFER
      quantumInferenceTarget
      quantumInferenceResultBinding?
      quantumInferenceSourceClause?
      quantumInferenceContextClause?
      quantumInferenceExtension*
    ;


/*
 * ============================================================================
 * 4. TARGET
 * ============================================================================
 *
 * The target is an ordinary Zamani expression.
 *
 * This permits:
 *
 *     model(input)
 *     classifier
 *     result
 *     namespace::model
 *     expression
 *
 * and future source-level value forms without extending this grammar.
 */

quantumInferenceTarget
    : expression
    ;


/*
 * ============================================================================
 * 5. RESULT BINDING
 * ============================================================================
 *
 * A result may optionally be bound to a source name.
 *
 * Example:
 *
 *     infer result as prediction from sample;
 *
 * The binding name remains an ordinary identifier.
 */

quantumInferenceResultBinding
    : AS identifier
    ;


/*
 * ============================================================================
 * 6. SOURCE CLAUSE
 * ============================================================================
 *
 * The source/evidence/input expression remains a canonical Zamani expression.
 *
 * This permits quantum and classical values to participate uniformly.
 */

quantumInferenceSourceClause
    : FROM
      quantumInferenceSource
    ;


/*
 * ============================================================================
 * 7. SOURCE
 * ============================================================================
 */

quantumInferenceSource
    : expression
    ;


/*
 * ============================================================================
 * 8. CONTEXT CLAUSE
 * ============================================================================
 *
 * Context is explicitly delimited so that additional inference metadata can
 * be supplied without introducing a second option language.
 */

quantumInferenceContextClause
    : WITH
      LPAREN
      quantumInferenceOptionList?
      RPAREN
    ;


/*
 * ============================================================================
 * 9. OPTION LIST
 * ============================================================================
 *
 * The list is open-ended.
 *
 * No finite list of inference strategies, models, algorithms, providers,
 * hardware targets, or quantum techniques is encoded.
 */

quantumInferenceOptionList
    : quantumInferenceOption
      (
          COMMA
          quantumInferenceOption
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 10. OPTION
 * ============================================================================
 *
 * An option is an ordinary expression.
 *
 * This intentionally avoids creating:
 *
 *     QuantumInferenceOptionType
 *     QuantumInferenceModelType
 *     QuantumInferenceStrategyType
 *
 * or another competing type hierarchy.
 *
 * Examples of semantic options may include:
 *
 *     model
 *     evidence
 *     confidence
 *     policy
 *     reproducibility
 *     precision
 *     execution
 *     sampling
 *     mitigation
 *     provenance
 *
 * but none of those names is reserved here.
 */

quantumInferenceOption
    : expression
    ;


/*
 * ============================================================================
 * 11. QUANTUM-SPECIFIC CONTEXT
 * ============================================================================
 *
 * This rule is intentionally generic.
 *
 * It provides a named AST/semantic boundary without encoding hardware
 * semantics in the grammar.
 *
 * A context can be an expression representing:
 *
 *     quantum state
 *     measurement result
 *     observable
 *     circuit result
 *     quantum register
 *     quantum-derived tensor
 *     symbolic quantum value
 *
 * Semantic analysis determines which interpretations are valid.
 */

quantumInferenceQuantumContext
    : expression
    ;


/*
 * ============================================================================
 * 12. CLASSICAL CONTEXT
 * ============================================================================
 *
 * Classical context is also represented by ordinary expressions.
 *
 * This permits hybrid inference without creating a second classical grammar.
 */

quantumInferenceClassicalContext
    : expression
    ;


/*
 * ============================================================================
 * 13. EXPLICIT HYBRID CONTEXT
 * ============================================================================
 *
 * The explicit context form allows semantic tooling to distinguish a quantum
 * context from a classical context without forcing a special value grammar.
 *
 * Example structural intent:
 *
 *     infer result with (
 *         quantum_context,
 *         classical_context
 *     );
 *
 * The actual interpretation of each expression is determined downstream.
 *
 * This rule deliberately uses a generic expression list.
 */

quantumInferenceHybridContext
    : quantumInferenceQuantumContext
      COMMA
      quantumInferenceClassicalContext
    ;


/*
 * ============================================================================
 * 14. NAMED CONTEXT ENTRY
 * ============================================================================
 *
 * Named context entries permit future extensibility while keeping names open.
 *
 * Example:
 *
 *     with (model = classifier, policy = deterministic)
 *
 * The semantic layer determines whether a particular name is recognized.
 *
 * No closed option vocabulary is created.
 */

quantumInferenceNamedContextEntry
    : identifier
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 15. EXTENSIBLE CONTEXT ENTRY
 * ============================================================================
 *
 * A context entry can be either:
 *
 *     ordinary expression
 *
 * or:
 *
 *     named expression
 *
 * This keeps the syntax future-proof without making the grammar responsible
 * for semantic registries.
 */

quantumInferenceContextEntry
    : quantumInferenceNamedContextEntry
    | expression
    ;


/*
 * ============================================================================
 * 16. EXTENDED CONTEXT LIST
 * ============================================================================
 *
 * This named boundary is useful to semantic tooling and AST adapters.
 *
 * The public context clause above remains deliberately simple.
 */

quantumInferenceExtendedContext
    : WITH
      LPAREN
      quantumInferenceContextEntry
      (
          COMMA
          quantumInferenceContextEntry
      )*
      COMMA?
      RPAREN
    ;


/*
 * ============================================================================
 * 17. EXTENSION
 * ============================================================================
 *
 * Extensions use the common annotation mechanism:
 *
 *     @name(...)
 *
 * This grammar does not define an application-specific annotation catalogue.
 *
 * Extension validation belongs to dialect/semantic infrastructure.
 */

quantumInferenceExtension
    : AT
      identifier
      quantumInferenceExtensionArguments?
    ;


/*
 * ============================================================================
 * 18. EXTENSION ARGUMENTS
 * ============================================================================
 */

quantumInferenceExtensionArguments
    : LPAREN
      quantumInferenceOptionList?
      RPAREN
    ;


/*
 * ============================================================================
 * 19. EXPRESSION INTEGRATION
 * ============================================================================
 *
 * All inference operands are canonical Zamani expressions.
 *
 * This rule exists as a named semantic boundary only.
 *
 * It MUST NOT evolve into another expression precedence hierarchy.
 */

quantumInferenceExpression
    : expression
    ;


/*
 * ============================================================================
 * 20. QUANTUM RESULT BINDING
 * ============================================================================
 *
 * A named result remains an ordinary source identifier.
 *
 * Its semantic type may be:
 *
 *     classical
 *     probabilistic
 *     tensor
 *     quantum-derived
 *     distributed
 *     user-defined
 *
 * depending on semantic inference.
 *
 * The grammar does not prescribe a physical representation.
 */

quantumInferenceBinding
    : identifier
    ;


/*
 * ============================================================================
 * 21. BINDING LIST
 * ============================================================================
 *
 * Open-ended repetition is used instead of a fixed binding count.
 */

quantumInferenceBindingList
    : quantumInferenceBinding
      (
          COMMA
          quantumInferenceBinding
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 22. EXTENSION DECLARATION BOUNDARY
 * ============================================================================
 *
 * A semantic extension may attach additional inference metadata without
 * requiring the core grammar to know every future extension.
 */

quantumInferenceExtensionDeclaration
    : AT
      identifier
      (
          LPAREN
          quantumInferenceOptionList?
          RPAREN
      )?
    ;


/*
 * ============================================================================
 * 23. SEMANTIC EXTENSION PAYLOAD
 * ============================================================================
 *
 * This rule intentionally remains expression-based.
 *
 * A future extension can therefore carry:
 *
 *     scalar values
 *     collections
 *     model references
 *     quantum values
 *     measurement results
 *     policies
 *     capabilities
 *     resource expressions
 *     user-defined values
 *
 * without modifying this grammar for each new domain.
 */

quantumInferenceExtensionPayload
    : quantumInferenceOptionList
    ;


/*
 * ============================================================================
 * 24. DECLARATIVE INFERENCE REGION
 * ============================================================================
 *
 * Some quantum programs may need a region in which inference participates in
 * ordinary quantum/classical control flow.
 *
 * The body remains owned by the canonical quantum composition.
 *
 * This rule is therefore deliberately limited to inference structure.
 */

quantumInferenceRegion
    : INFER
      quantumInferenceTarget
      quantumInferenceResultBinding?
      quantumInferenceSourceClause?
      quantumInferenceContextClause?
      LBRACE
      quantumInferenceExtensionDeclaration*
      RBRACE
    ;


/*
 * ============================================================================
 * 25. MODEL REFERENCE BOUNDARY
 * ============================================================================
 *
 * Models remain ordinary expressions.
 *
 * This named rule exists for AST/semantic tooling only.
 *
 * It does not create a model type.
 */

quantumInferenceModel
    : expression
    ;


/*
 * ============================================================================
 * 26. INPUT BOUNDARY
 * ============================================================================
 *
 * Inputs may be:
 *
 *     measurement results
 *     quantum-derived values
 *     classical values
 *     tensors
 *     datasets
 *     streams
 *     distributed values
 *     user-defined values
 *
 * The canonical expression system represents all of them.
 */

quantumInferenceInput
    : expression
    ;


/*
 * ============================================================================
 * 27. OUTPUT BOUNDARY
 * ============================================================================
 *
 * Output syntax is represented by an ordinary expression/binding boundary.
 *
 * The semantic layer determines its type and execution representation.
 */

quantumInferenceOutput
    : expression
    | identifier
    ;


/*
 * ============================================================================
 * 28. CONTEXT LIST
 * ============================================================================
 *
 * Open-ended context composition.
 */

quantumInferenceContextList
    : quantumInferenceContextEntry
      (
          COMMA
          quantumInferenceContextEntry
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 29. OPTIONAL CONTEXT
 * ============================================================================
 */

quantumInferenceOptionalContext
    : WITH
      LPAREN
      quantumInferenceContextList?
      RPAREN
    ;


/*
 * ============================================================================
 * 30. SEMANTIC BOUNDARY FOR FUTURE EXTENSIONS
 * ============================================================================
 *
 * This rule intentionally accepts only canonical source expressions.
 *
 * It does not embed target/backend syntax.
 */

quantumInferenceExtensionValue
    : expression
    ;


/*
 * ============================================================================
 * 31. INTEGRATION INVARIANT
 * ============================================================================
 *
 * The following ownership remains authoritative:
 *
 *     general inference
 *         -> grammar/ai/inference.g4
 *
 *     reasoning
 *         -> grammar/expressions/reasoning.g4
 *
 *     expression
 *         -> grammar/expressions/expressions.g4
 *
 *     quantum operation
 *         -> grammar/quantum/operations.g4
 *
 *     measurement
 *         -> grammar/quantum/measurement.g4
 *
 *     resource
 *         -> grammar/resources/
 *            grammar/quantum/quantum-resources.g4
 *
 *     capability
 *         -> grammar/resources/
 *            grammar/quantum/quantum-capabilities.g4
 *
 *     policy
 *         -> grammar/policies/
 *
 *     provenance
 *         -> provenance semantic subsystem
 *
 *     quantum semantic representation
 *         -> quantum::ir
 *
 * This file only supplies the quantum-inference composition boundary.
 *
 * ============================================================================
 * 32. NO SECOND REASONING LANGUAGE
 * ============================================================================
 *
 * This grammar does not define:
 *
 *     inferOperator
 *     reasoningOperator
 *     deductionOperator
 *     inductionOperator
 *     causalOperator
 *
 * The generic reasoning grammar remains authoritative for:
 *
 *     infer
 *     deduce
 *     reason
 *
 * This file consumes the canonical INFER token only to establish the
 * quantum-specific integration boundary.
 *
 * ============================================================================
 * 33. NO ALGORITHM CATALOGUE
 * ============================================================================
 *
 * The grammar does not enumerate:
 *
 *     Bayesian inference
 *     variational inference
 *     amplitude estimation
 *     phase estimation
 *     quantum kernel methods
 *     quantum neural networks
 *     variational quantum algorithms
 *     Monte Carlo
 *     sampling algorithms
 *     theorem proving
 *     symbolic inference
 *     probabilistic inference
 *
 * Such concepts are represented semantically through:
 *
 *     model
 *     operation
 *     context
 *     capability
 *     policy
 *     library
 *     dialect
 *     extension
 *
 * ============================================================================
 * 34. NO HARDWARE CATALOGUE
 * ============================================================================
 *
 * This grammar does not enumerate:
 *
 *     QPU vendors
 *     QPU models
 *     CPU models
 *     GPU models
 *     FPGA models
 *     accelerator models
 *     simulator engines
 *     physical topologies
 *     coupling maps
 *     calibration sets
 *
 * Those are target/runtime concerns.
 *
 * ============================================================================
 * 35. NO PHYSICAL RESOURCE LIMITS
 * ============================================================================
 *
 * This grammar contains no universal ceilings for:
 *
 *     qubits
 *     logical qubits
 *     physical qubits
 *     measurements
 *     samples
 *     parameters
 *     inputs
 *     outputs
 *     inference contexts
 *     operations
 *     circuit depth
 *     circuit width
 *     devices
 *     workers
 *     nodes
 *     memory
 *     storage
 *     tensor dimensions
 *     tensor rank
 *
 * No equivalent hidden limit may be introduced through parser structure.
 *
 * ============================================================================
 * 36. NO TARGET REALIZATION
 * ============================================================================
 *
 * This grammar does not decide:
 *
 *     physical qubit placement
 *     routing
 *     scheduling
 *     gate decomposition
 *     pulse generation
 *     calibration
 *     QEC strategy
 *     noise implementation
 *     accelerator selection
 *     simulator selection
 *     provider selection
 *
 * Those belong after semantic analysis and the canonical quantum IR boundary.
 *
 * ============================================================================
 * 37. DOMAIN-NEUTRAL VALUE FLOW
 * ============================================================================
 *
 * Quantum inference is intentionally allowed to consume expressions rather
 * than domain-specific syntactic values.
 *
 * This permits future composition such as:
 *
 *     quantum result
 *         ->
 *     classical transformation
 *         ->
 *     tensor/model input
 *         ->
 *     inference
 *         ->
 *     classical decision
 *         ->
 *     quantum control
 *
 * without creating separate programming languages for each transition.
 *
 * ============================================================================
 * 38. AST CONTRACT
 * ============================================================================
 *
 * The AST adapter must preserve at least:
 *
 *     construct kind
 *     source span
 *     target expression
 *     optional result binding
 *     optional source expression
 *     optional context expressions
 *     named context entries
 *     extension names
 *     extension arguments
 *     syntactic ordering
 *
 * The AST must not store:
 *
 *     selected QPU
 *     physical qubit mapping
 *     routing result
 *     scheduler result
 *     calibration
 *     backend-specific gate set
 *
 * unless such information was explicitly represented as source-level data.
 *
 * ============================================================================
 * 39. TYPE CONTRACT
 * ============================================================================
 *
 * Types are determined by the canonical type system.
 *
 * Quantum inference may involve:
 *
 *     quantum types
 *     classical types
 *     tensor types
 *     probabilistic types
 *     user-defined types
 *     distributed types
 *     future domain types
 *
 * This grammar does not introduce a competing inference type system.
 *
 * ============================================================================
 * 40. EFFECT CONTRACT
 * ============================================================================
 *
 * Effects are determined downstream.
 *
 * Possible semantic effects include:
 *
 *     measurement
 *     randomness
 *     network
 *     distributed
 *     foreign
 *     learning
 *     adaptation
 *     model_access
 *
 * The presence of an inference construct does not automatically imply every
 * possible effect.
 *
 * Semantic analysis determines the actual effect set.
 *
 * ============================================================================
 * 41. CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability requirements are semantic.
 *
 * Examples include:
 *
 *     quantum.measurement
 *     quantum.dynamic_control
 *     quantum.inference
 *     tensor.compute
 *
 * Capability names remain open-world.
 *
 * Capability availability is determined by the compilation/execution
 * environment.
 *
 * ============================================================================
 * 42. RESOURCE CONTRACT
 * ============================================================================
 *
 * Quantum inference may require resources such as:
 *
 *     quantum resources
 *     classical compute
 *     memory
 *     tensor compute
 *     communication
 *     accelerator resources
 *
 * Resource requirements are not physical allocations.
 *
 * They are resolved downstream.
 *
 * ============================================================================
 * 43. POLICY CONTRACT
 * ============================================================================
 *
 * Policies may constrain:
 *
 *     allowed models
 *     allowed operations
 *     reproducibility
 *     data movement
 *     network use
 *     adaptation
 *     execution targets
 *     resource selection
 *
 * Policy semantics are not implemented by this grammar.
 *
 * ============================================================================
 * 44. PROVENANCE CONTRACT
 * ============================================================================
 *
 * Semantic tooling must be able to associate provenance with:
 *
 *     inference source
 *     inference model
 *     quantum inputs
 *     measurement inputs
 *     transformations
 *     result
 *     compilation decisions
 *
 * The parser preserves source structure required for this purpose.
 *
 * ============================================================================
 * 45. QUANTUM IR CONTRACT
 * ============================================================================
 *
 * If quantum inference causes quantum computation, its semantic quantum
 * operations must eventually lower through:
 *
 *     quantum::ir
 *
 * No:
 *
 *     QuantumInferenceIR
 *     QuantumMLIR
 *     QuantumInferenceCircuitIR
 *
 * or equivalent competing representation is introduced here.
 *
 * The canonical path remains:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic inference model
 *       |
 *       +--------------------+
 *       |                    |
 *       v                    v
 *   classical            quantum semantics
 *                            |
 *                            v
 *                       quantum::ir
 *
 * ============================================================================
 * 46. CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * This construct must remain composable with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     AI/model
 *     tensor
 *     distributed
 *     networking
 *     hardware-independent
 *     HDL-derived
 *     simulation
 *     future domains
 *
 * Domain composition occurs through common expressions, types, effects,
 * capabilities, resources, policies and semantic analysis.
 *
 * ============================================================================
 * 47. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing:
 *
 *     infer
 *
 * syntax remains owned by the existing reasoning/inference architecture.
 *
 * This grammar must not silently change the meaning of:
 *
 *     infer expression
 *
 * merely because the expression appears inside a quantum scope.
 *
 * The semantic analyzer determines whether a construct is:
 *
 *     generic inference
 *     reasoning
 *     quantum inference
 *     hybrid inference
 *
 * according to the enclosing semantic context and resolved types/capabilities.
 *
 * If the repository's final parser makes ordinary `infer` globally available
 * through `Reasoning`, the quantum composition should expose this grammar
 * through an explicit quantum boundary rather than adding a second global
 * `infer` alternative.
 *
 * ============================================================================
 * 48. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify structural errors such as:
 *
 *     missing inference target
 *     malformed source clause
 *     malformed context
 *     malformed option list
 *     malformed extension
 *     missing closing delimiter
 *     malformed result binding
 *
 * Semantic diagnostics belong downstream:
 *
 *     invalid quantum input type
 *     invalid model type
 *     unavailable capability
 *     insufficient resources
 *     unsupported execution mode
 *     invalid measurement dependency
 *     invalid policy
 *     invalid provenance requirement
 *     unavailable target
 *
 * A target feasibility failure must not be reported as a syntax error.
 *
 * ============================================================================
 * 49. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * Minimal:
 *
 *     infer classifier;
 *
 * Source:
 *
 *     infer classifier from sample;
 *
 * Result binding:
 *
 *     infer classifier as prediction from sample;
 *
 * Context:
 *
 *     infer classifier with (context);
 *
 * Source and context:
 *
 *     infer classifier from sample with (context);
 *
 * Quantum-derived source:
 *
 *     infer classifier from measurement_result;
 *
 * Quantum-derived expression:
 *
 *     infer classifier(quantum_value) from sample;
 *
 * Hybrid:
 *
 *     infer model(classical_value, quantum_value)
 *         from dataset
 *         with (execution_context);
 *
 * Qualified model:
 *
 *     infer library::model(input) from source;
 *
 * Extension:
 *
 *     infer model(input) @extension;
 *
 * Extension with arguments:
 *
 *     infer model(input) @extension(option);
 *
 * Named context:
 *
 *     infer model(input)
 *         with (model = selected_model, policy = execution_policy);
 *
 * The exact acceptance of annotation placement is governed by the canonical
 * expression/statement composition after this grammar is integrated.
 *
 * ============================================================================
 * 50. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must fail structurally:
 *
 *     infer;
 *
 *     infer from source;
 *
 *     infer target from;
 *
 *     infer target with ();
 *
 *     infer target with (,);
 *
 *     infer target from source with (context,);
 *
 * where a trailing comma is not accepted by the selected list production.
 *
 * An implementation should additionally test malformed delimiters and invalid
 * token placement.
 *
 * ============================================================================
 * 51. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Conformance tests must verify that grammar structure does not impose a
 * finite machine-size ceiling.
 *
 * Tests should exercise implementation-supported growth in:
 *
 *     expression depth
 *     context entries
 *     nested inference expressions
 *     extension arguments
 *     quantum-derived inputs
 *     classical inputs
 *     model composition
 *
 * Tests must not define a language maximum such as:
 *
 *     MAX_CONTEXT
 *     MAX_PARAMETERS
 *     MAX_QUBITS
 *     MAX_MODELS
 *
 * A test limit is an implementation/test-run budget, not a language rule.
 *
 * ============================================================================
 * 52. DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * For identical:
 *
 *     source
 *     lexer configuration
 *     grammar version
 *     parser configuration
 *
 * the parser must produce structurally equivalent parse trees.
 *
 * The grammar must not inspect:
 *
 *     hardware
 *     network
 *     filesystem
 *     time
 *     randomness
 *     runtime state
 *
 * ============================================================================
 * 53. INTEGRATION MATRIX
 * ============================================================================
 *
 * THIS FILE
 *     Owns:
 *         quantum inference source boundary.
 *
 * CONSUMES:
 *     ZamaniLexer
 *     Expressions
 *
 * IS CONSUMED BY:
 *     Quantum
 *
 * RELATED OWNERS:
 *     grammar/ai/inference.g4
 *         general inference
 *
 *     grammar/expressions/reasoning.g4
 *         general reasoning
 *
 *     grammar/expressions/expressions.g4
 *         canonical expression hierarchy
 *
 *     grammar/quantum/operations.g4
 *         quantum operation syntax
 *
 *     grammar/quantum/measurement.g4
 *         measurement syntax
 *
 *     grammar/quantum/quantum-resources.g4
 *         quantum resource intent
 *
 *     grammar/quantum/quantum-capabilities.g4
 *         quantum capability intent
 *
 *     grammar/quantum/quantum-classical.g4
 *         quantum/classical boundary
 *
 * AST:
 *     src/frontend/ast/
 *
 * SEMANTICS:
 *     quantum + inference semantic analysis
 *
 * IR:
 *     quantum::ir
 *
 * BACKEND:
 *     optimization
 *     decomposition
 *     routing
 *     scheduling
 *     QEC
 *     resilience
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * 54. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It has exactly one public quantum-inference entry point.
 *     [x] It uses the canonical Zamani lexer.
 *     [x] It reuses the canonical expression grammar.
 *     [x] It introduces no second expression hierarchy.
 *     [x] It does not duplicate general AI inference ownership.
 *     [x] It does not duplicate generic reasoning ownership.
 *     [x] It does not enumerate quantum operations.
 *     [x] It does not enumerate inference algorithms.
 *     [x] It does not enumerate hardware targets.
 *     [x] It does not encode physical qubit limits.
 *     [x] It does not encode memory/device/node limits.
 *     [x] It does not create another quantum IR.
 *     [x] It preserves quantum/classical value composition.
 *     [x] It supports open-ended semantic extensions.
 *     [x] It remains target-independent.
 *     [x] It remains deterministic.
 *     [x] It contains no executable actions.
 *     [x] It requires only safe Rust downstream.
 *
 * Integration gates still required:
 *
 *     [ ] Quantum composition imports this grammar exactly once.
 *     [ ] Canonical parser exposes quantumInferenceConstruct at the intended
 *         quantum statement/declaration boundary.
 *     [ ] AST adapter has a domain-neutral representation.
 *     [ ] Semantic analysis distinguishes generic inference from quantum
 *         inference without changing source meaning.
 *     [ ] Quantum semantic lowering crosses quantum::ir.
 *     [ ] Resource/capability/effect/policy/provenance analysis is connected.
 *     [ ] Positive, negative, boundary and scalability tests pass.
 *
 * ============================================================================
 */