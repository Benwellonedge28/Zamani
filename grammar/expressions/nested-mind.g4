/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/nested-mind.g4
 *
 * Grammar:
 *     NestedMindExpressions
 *
 * Status:
 *     CANONICAL / PRODUCTION EXPRESSION LEAF
 *
 * Language:
 *     Zamani
 *
 * ANTLR:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the syntax boundary for recursively composable
 * computational-mind expressions.
 *
 * A nested mind is NOT a second programming language.
 *
 * It is a source-level expression container for composing semantic operations
 * such as reasoning, knowledge access, learning, adaptation, explanation,
 * uncertainty handling, decision making, simulation, hybrid computation,
 * quantum-assisted computation, and future semantic capabilities.
 *
 * The actual meaning of an operation is resolved by semantic analysis.
 *
 * This grammar therefore remains open-world.
 *
 * New semantic operations do not require this grammar to be modified.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       v
 *     expressions.g4
 *       |
 *       +--------------------------+
 *       |                          |
 *       v                          v
 * ordinary expressions      nestedMindExpression
 *                                  |
 *                                  v
 *                         domain-neutral AST
 *                                  |
 *                                  v
 *                         structural validation
 *                                  |
 *                                  v
 *                         semantic analysis
 *                                  |
 *              +-------------------+-------------------+
 *              |                   |                   |
 *              v                   v                   v
 *          reasoning          knowledge           learning
 *              |                   |                   |
 *              +-------------------+-------------------+
 *                                  |
 *                                  v
 *                    capabilities / resources / effects
 *                                  |
 *                                  v
 *                         contracts / policies
 *                                  |
 *                                  v
 *                             provenance
 *                                  |
 *                                  v
 *                    canonical semantic representation
 *                                  |
 *              +-------------------+-------------------+
 *              |                   |                   |
 *              v                   v                   v
 *          classical          quantum::ir          HDL/hardware
 *              |                   |                   |
 *              +-------------------+-------------------+
 *                                  |
 *                                  v
 *                         optimization/lowering
 *                                  |
 *                          routing/scheduling
 *                                  |
 *                         resilience / QEC
 *                                  |
 *                                 ZQN
 *                                  |
 *                                 HAL
 *                                  |
 *                           target realization
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * OWNS
 * ----
 *
 *     nestedMindExpression
 *     nestedMindBody
 *     nestedMindClause
 *     nestedMindOperation
 *     nestedMindOperationName
 *     nestedMindArguments
 *     nestedMindArgumentList
 *     nestedMindArgument
 *     nestedMindGuard
 *
 * DOES NOT OWN
 * -------------
 *
 *     expression
 *     assignmentExpression
 *     conditionalExpression
 *     logical precedence
 *     arithmetic precedence
 *     unary precedence
 *     postfix precedence
 *     ordinary calls
 *     ordinary indexing
 *     ordinary member access
 *     ordinary assignment
 *     types
 *     patterns
 *     reasoning semantics
 *     knowledge semantics
 *     learning semantics
 *     adaptation semantics
 *     uncertainty semantics
 *     explanation semantics
 *     provenance semantics
 *     policy semantics
 *     effect semantics
 *     capability semantics
 *     resource semantics
 *     actor semantics
 *     quantum semantics
 *     HDL semantics
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     target selection
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/core/qualified-names.g4
 *     canonical expression-core integration boundary
 *
 * IMPORTANT:
 *
 * This grammar MUST NOT import:
 *
 *     grammar/expressions/expressions.g4
 *
 * when expressions.g4 imports this grammar.
 *
 * Doing so creates a parser grammar dependency cycle.
 *
 * EXPORTS
 * -------
 *
 *     nestedMindExpression
 *
 * CONSUMED_BY
 * ----------
 *
 *     grammar/expressions/expressions.g4
 *     canonical parser composition
 *     semantic expression validation
 *
 * AST_OWNER
 * ---------
 *
 *     Existing domain-neutral Zamani frontend AST.
 *
 * This grammar creates parser contexts only.
 *
 * SEMANTIC_OWNER
 * --------------
 *
 *     Generic semantic operation resolution.
 *
 * The resolved operation is then handled by its owning subsystem:
 *
 *     reasoning.*       -> reasoning semantic model
 *     knowledge.*       -> knowledge semantic model
 *     learning.*        -> learning semantic model
 *     adaptation.*      -> adaptation semantic model
 *     policy.*          -> policy semantic model
 *     provenance.*      -> provenance semantic model
 *     quantum.*         -> quantum semantic model
 *     hybrid.*          -> hybrid semantic model
 *     execution.*       -> execution semantic model
 *
 * IR_OWNER
 * --------
 *
 *     Canonical semantic IR.
 *
 * This file creates NO nested-mind-specific IR.
 *
 * Quantum operations ultimately cross:
 *
 *     quantum::ir
 *
 * Classical operations ultimately cross the canonical classical/domain IR.
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/expressions/nested-mind/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/ai.md
 *     grammar/spec/expression-model.md
 *     grammar/spec/poco-reaf.md
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This file contains NO lexer rules.
 *
 * All lexical tokens come from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through the repository's canonical lexer hierarchy.
 *
 * The word:
 *
 *     mind
 *
 * MUST NOT be implemented as a private lexer token in this file.
 *
 * A canonical reserved token may be introduced later by the central lexer
 * architecture, but this grammar must remain independent of that decision.
 *
 * ============================================================================
 * NAME CONTRACT
 * ============================================================================
 *
 * Names are consumed through:
 *
 *     grammar/core/names.g4
 *
 * and:
 *
 *     grammar/core/qualified-names.g4
 *
 * The grammar does not recreate identifier syntax.
 *
 * Cognitive operations therefore remain open-world:
 *
 *     reason
 *     infer
 *     deduce
 *     knowledge::query
 *     model::learn
 *     strategy::adapt
 *     decision::explain
 *     quantum::measure
 *     hybrid::execute
 *
 * are names, not a closed keyword catalogue.
 *
 * ============================================================================
 * OPERATION MODEL
 * ============================================================================
 *
 * The semantic model is:
 *
 *     operation
 *         =
 *     qualified name
 *         +
 *     optional arguments
 *         +
 *     optional guard
 *
 * Examples:
 *
 *     reason(problem)
 *     infer(evidence)
 *     knowledge::query(pattern)
 *     model::learn(model, data)
 *     strategy::adapt(strategy, feedback)
 *     decision::explain(result)
 *     quantum::measure(register)
 *
 * The grammar does not determine which operation exists.
 *
 * Unknown operations are semantic-resolution diagnostics, not parser errors.
 *
 * ============================================================================
 * NESTING MODEL
 * ============================================================================
 *
 * A nested mind may contain operations whose arguments contain another nested
 * mind expression.
 *
 * Conceptually:
 *
 *     mind {
 *         reason(
 *             mind {
 *                 infer(problem)
 *             }
 *         )
 *     }
 *
 * This recursion is intentional.
 *
 * There is no grammar-level maximum for:
 *
 *     nesting depth
 *     clause count
 *     operation count
 *     argument count
 *     qualified-name depth
 *
 * Practical implementation limits are resource limits, not language limits.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar introduces no universal finite capacity.
 *
 * It MUST NOT contain:
 *
 *     MAX_MIND_DEPTH
 *     MAX_COGNITIVE_DEPTH
 *     MAX_CLAUSES
 *     MAX_ARGUMENTS
 *     MAX_OPERATIONS
 *     MAX_CONTEXTS
 *     MAX_REASONING_STEPS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * or equivalent universal ceilings.
 *
 * Repetition is represented structurally through ANTLR repetition operators.
 *
 * Actual limits belong to:
 *
 *     parser resources
 *     compiler resources
 *     memory availability
 *     execution policy
 *     target capability
 *     deployment constraints
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     cluster
 *     cloud
 *     network node
 *
 * A mind operation can ultimately be lowered to any supported computational
 * domain according to semantic capabilities, resource requirements,
 * constraints, policies, and compilation context.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * This grammar does not define quantum operations.
 *
 * It permits a semantic operation name such as:
 *
 *     quantum::measure
 *     quantum::prepare
 *     quantum::execute
 *
 * to be represented as a source-level operation reference.
 *
 * Semantic analysis decides whether the operation is quantum.
 *
 * If so:
 *
 *     semantic operation
 *         ->
 *     quantum semantic model
 *         ->
 *     quantum::ir
 *
 * The grammar does not know physical qubits, topology, routing, calibration,
 * decomposition, scheduling, or QEC.
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * This grammar does not define HDL syntax.
 *
 * An operation may refer semantically to HDL/hardware computation, but
 * realization belongs to the HDL/hardware semantic and lowering layers.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * This grammar does not assign effects.
 *
 * Semantic resolution may determine effects such as:
 *
 *     reasoning
 *     knowledge.read
 *     knowledge.write
 *     learning
 *     adaptation
 *     randomness
 *     measurement
 *     network
 *     foreign
 *     native
 *     reflection
 *     simulation
 *     distributed
 *
 * The same syntactic operation shape can therefore be implemented differently
 * without changing the grammar.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capabilities are resolved semantically.
 *
 * Examples include:
 *
 *     reasoning
 *     knowledge.query
 *     model.inference
 *     tensor.compute
 *     quantum.measurement
 *     hardware.accelerator
 *
 * The grammar does not enumerate capabilities.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements belong to the repository's resource system.
 *
 * A nested mind expression may semantically require:
 *
 *     compute
 *     memory
 *     storage
 *     communication
 *     accelerator resources
 *     quantum resources
 *     model resources
 *
 * No resource capacity is encoded here.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Contracts are not reimplemented here.
 *
 * A nested mind operation may participate in the repository's existing:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * model through the surrounding expression/validation architecture.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies remain owned by:
 *
 *     grammar/expressions/policy.g4
 *     grammar/policies/
 *     grammar/security/
 *     grammar/execution/
 *
 * A nested mind expression does not grant permissions merely because an
 * operation has a particular name.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parser preserves source structure and source spans.
 *
 * Semantic analysis may associate operations with:
 *
 *     evidence
 *     provenance
 *     decisions
 *     transformations
 *     model lineage
 *     execution lineage
 *
 * Provenance storage and recording remain outside this grammar.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source tokens
 *     active grammar
 *     parser configuration
 *
 * Parsing must not inspect:
 *
 *     hardware
 *     filesystem state
 *     network state
 *     runtime state
 *     target state
 *     QPU state
 *     GPU state
 *     environment variables
 *     wall-clock time
 *     randomness
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no embedded Rust actions.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust.
 *
 * No unsafe Rust is required.
 *
 * ============================================================================
 */

parser grammar NestedMindExpressions;

options {
    tokenVocab = ZamaniLexer;
}

import QualifiedNames;


/*
 * ============================================================================
 * PUBLIC ENTRY
 * ============================================================================
 *
 * `nestedMindExpression` is the only public rule exported by this grammar.
 *
 * The enclosing expression grammar determines where it can occur.
 *
 * ============================================================================
 */

nestedMindExpression
    : nestedMindMarker
      nestedMindBody
    ;


/*
 * ============================================================================
 * MIND MARKER
 * ============================================================================
 *
 * IMPORTANT:
 *
 * The current repository does not yet expose a canonical MIND lexer token.
 *
 * Therefore this grammar does not invent one.
 *
 * However, accepting every IDENTIFIER here would make:
 *
 *     name { ... }
 *
 * structurally indistinguishable from a future ordinary construct.
 *
 * The marker is consequently isolated behind this rule so that the semantic
 * and lexical architecture has exactly one promotion point.
 *
 * The current source spelling is:
 *
 *     mind
 *
 * represented lexically as IDENTIFIER.
 *
 * Semantic validation MUST verify that the identifier spelling is exactly the
 * canonical language marker.
 *
 * A future canonical reserved token can replace this rule centrally without
 * changing the semantic model.
 *
 * ============================================================================
 */

nestedMindMarker
    : identifier
    ;


/*
 * ============================================================================
 * BODY
 * ============================================================================
 *
 * Zero or more clauses are syntactically permitted.
 *
 * Whether an empty body is meaningful is a semantic/contextual question.
 *
 * ============================================================================
 */

nestedMindBody
    : LEFT_BRACE
      nestedMindClause*
      RIGHT_BRACE
    ;


/*
 * ============================================================================
 * CLAUSE
 * ============================================================================
 *
 * A clause contains:
 *
 *     operation
 *     optional guard
 *     optional expression terminator
 *
 * Result binding is deliberately NOT implemented here.
 *
 * Assignment already belongs to:
 *
 *     grammar/expressions/expressions.g4
 *
 * and:
 *
 *     grammar/expressions/assignment.g4
 *
 * Keeping assignment outside this grammar prevents a second assignment model.
 *
 * ============================================================================
 */

nestedMindClause
    : nestedMindOperation
      nestedMindGuard?
      nestedMindTerminator?
    ;


/*
 * ============================================================================
 * OPERATION
 * ============================================================================
 *
 * The operation vocabulary is intentionally open.
 *
 * ============================================================================
 */

nestedMindOperation
    : nestedMindOperationName
      nestedMindArguments?
    ;


/*
 * ============================================================================
 * OPERATION NAME
 * ============================================================================
 *
 * Reuses the repository's canonical qualified-name model.
 *
 * ============================================================================
 */

nestedMindOperationName
    : qualifiedName
    ;


/*
 * ============================================================================
 * ARGUMENTS
 * ============================================================================
 *
 * Argument cardinality is open-ended.
 *
 * ============================================================================
 */

nestedMindArguments
    : LEFT_PAREN
      nestedMindArgumentList?
      RIGHT_PAREN
    ;


/*
 * ============================================================================
 * ARGUMENT LIST
 * ============================================================================
 */

nestedMindArgumentList
    : nestedMindArgument
      (
          COMMA
          nestedMindArgument
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * ARGUMENT
 * ============================================================================
 *
 * This is the ONLY place where the shared expression operand boundary is
 * consumed.
 *
 * `expressionCore` MUST be supplied by the canonical expression-composition
 * layer described below.
 *
 * This grammar deliberately does not define it.
 *
 * ============================================================================
 */

nestedMindArgument
    : expressionCore
    ;


/*
 * ============================================================================
 * GUARD
 * ============================================================================
 *
 * A guard is a semantic predicate attached to the operation.
 *
 * Example:
 *
 *     infer(problem) if confidence > threshold;
 *
 * The guard uses the shared expression operand boundary.
 *
 * ============================================================================
 */

nestedMindGuard
    : IF
      expressionCore
    ;


/*
 * ============================================================================
 * TERMINATOR
 * ============================================================================
 *
 * Statement/expression termination remains owned by the canonical parser
 * composition.
 *
 * This leaf accepts the canonical semicolon token when the surrounding
 * composition permits an explicit terminator.
 * ============================================================================
 */

nestedMindTerminator
    : SEMICOLON
    ;


/*
 * ============================================================================
 * EXPRESSION-CORE INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar intentionally does NOT define `expressionCore`.
 *
 * A production integration requires one shared, acyclic parser boundary:
 *
 *     canonical expression implementation
 *                 |
 *                 v
 *          expressionCore
 *                 |
 *        +--------+---------+
 *        |                  |
 *        v                  v
 *   nested-mind       other expression leaves
 *
 * The dependency graph MUST NOT become:
 *
 *     expressions.g4
 *         ->
 *     nested-mind.g4
 *         ->
 *     expressions.g4
 *
 * because that creates a grammar cycle.
 *
 * Therefore `expressionCore` must be introduced as an independent lower-level
 * expression composition boundary before this grammar is integrated into
 * `expressions.g4`.
 *
 * The canonical ownership contract is:
 *
 *     expression
 *         -> full precedence expression
 *
 *     expressionCore
 *         -> reusable expression operand boundary
 *
 *     nestedMindExpression
 *         -> leaf expression construct
 *
 * The exact implementation of `expressionCore` belongs to the shared
 * expression architecture, not to this file.
 *
 * ============================================================================
 * SEMANTIC RESOLUTION
 * ============================================================================
 *
 * The following are examples of semantic operation names:
 *
 *     infer
 *     deduce
 *     reason
 *     knowledge::assert
 *     knowledge::retract
 *     knowledge::query
 *     learn
 *     adapt
 *     explain
 *     decide
 *     observe
 *     verify
 *     quantum::measure
 *     hybrid::execute
 *
 * None are grammar alternatives.
 *
 * This prevents the grammar from becoming a closed catalogue of current
 * algorithms.
 *
 * ============================================================================
 * ERROR OWNERSHIP
 * ============================================================================
 *
 * Parser diagnostics:
 *
 *     missing marker
 *     missing body
 *     missing closing brace
 *     malformed operation name
 *     malformed argument list
 *     malformed guard
 *     malformed terminator
 *
 * Semantic diagnostics:
 *
 *     unknown operation
 *     unavailable operation
 *     invalid argument type
 *     unavailable capability
 *     unavailable resource
 *     forbidden effect
 *     violated policy
 *     violated contract
 *     invalid provenance
 *     unsupported target realization
 *
 * A hardware/resource failure MUST NOT be converted into a parser error.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms must be structurally representable once the shared
 * expression-core boundary is integrated:
 *
 *     mind {}
 *
 *     mind {
 *         reason(problem);
 *     }
 *
 *     mind {
 *         infer(evidence);
 *         deduce(premises);
 *         explain(decision);
 *     }
 *
 *     mind {
 *         knowledge::query(pattern);
 *         model::learn(model, data);
 *         strategy::adapt(strategy, feedback);
 *     }
 *
 *     mind {
 *         reason(
 *             mind {
 *                 infer(problem);
 *             }
 *         );
 *     }
 *
 *     mind {
 *         quantum::measure(register)
 *             if ready;
 *     }
 *
 *     mind {
 *         hybrid::execute(
 *             classical_value,
 *             quantum_value
 *         );
 *     }
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * These must produce parser diagnostics:
 *
 *     mind
 *
 *     mind {
 *
 *     mind }
 *
 *     mind {
 *         ;
 *     }
 *
 *     mind {
 *         reason(
 *     }
 *
 *     mind {
 *         reason(, value);
 *     }
 *
 *     mind {
 *         reason(value,,other);
 *     }
 *
 *     mind {
 *         reason(value) if;
 *     }
 *
 * Semantic failures must be tested separately:
 *
 *     mind {
 *         operation_that_does_not_exist(value);
 *     }
 *
 * That is a semantic name-resolution error, not a grammar error.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Required cross-domain tests include:
 *
 *     reasoning + knowledge
 *     reasoning + uncertainty
 *     reasoning + provenance
 *     learning + policy
 *     adaptation + resource requirements
 *     AI + concurrency
 *     AI + classical computation
 *     AI + quantum computation
 *     AI + hybrid computation
 *     AI + HDL/hardware references
 *     AI + distributed computation
 *     AI + interoperability
 *     AI + metaprogramming
 *     simulation + adaptive execution
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must demonstrate that the same grammar handles:
 *
 *     tiny mind expressions
 *     large operation sequences
 *     deeply nested mind expressions
 *     large argument structures
 *     long qualified names
 *     nested ordinary expressions
 *     mixed-domain operations
 *
 * Tests MUST NOT introduce language-level limits.
 *
 * Generated stress sizes are test parameters, not grammar constants.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     no machine-size constants
 *     no quantum-size constants
 *     no AI-model-size constants
 *     no operation catalogue
 *     no vendor catalogue
 *     no hardware catalogue
 *     no resource ceiling
 *     no target ceiling
 *     no algorithm ceiling
 *
 * The only closed syntactic concepts are structural punctuation and the
 * generic `mind` container.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Adding a new semantic cognitive operation MUST NOT require changing this
 * grammar.
 *
 * Adding:
 *
 *     infer
 *     deduce
 *     reason
 *     learn
 *     adapt
 *     query
 *     explain
 *     decide
 *     future.operation
 *
 * is therefore a semantic-registry operation, not a grammar change.
 *
 * A change to the spelling/reservation of `mind` is a lexical compatibility
 * change and must be handled centrally by the lexer/compatibility subsystem.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * REQUIRED INTEGRATION 1:
 *
 *     grammar/expressions/expressions.g4
 *
 * must import this grammar exactly once and expose:
 *
 *     nestedMindExpression
 *
 * from the appropriate primary-expression boundary.
 *
 * REQUIRED INTEGRATION 2:
 *
 * The canonical expression architecture must provide:
 *
 *     expressionCore
 *
 * without importing this grammar back into itself.
 *
 * REQUIRED INTEGRATION 3:
 *
 *     grammar/core/names.g4
 *
 * remains the authority for:
 *
 *     identifier
 *     qualifiedName
 *
 * REQUIRED INTEGRATION 4:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * remains the sole public lexer.
 *
 * REQUIRED INTEGRATION 5:
 *
 *     grammar/ai/mind.g4
 *
 * remains the AI/mind semantic/declaration layer.
 *
 * This file must NOT duplicate its declaration/annotation model.
 *
 * REQUIRED INTEGRATION 6:
 *
 *     grammar/expressions/reasoning.g4
 *     grammar/expressions/knowledge.g4
 *     grammar/expressions/policy.g4
 *     grammar/expressions/provenance.g4
 *
 * remain owners of their respective expression semantics.
 *
 * This grammar merely provides the generic nesting/composition boundary.
 *
 * REQUIRED INTEGRATION 7:
 *
 * Semantic resolution must convert:
 *
 *     nestedMindOperation
 *
 * into the existing generic semantic operation representation.
 *
 * No NestedMindIR may be introduced.
 *
 * REQUIRED INTEGRATION 8:
 *
 * Quantum-resolved operations must ultimately lower through:
 *
 *     quantum::ir
 *
 * rather than a second quantum representation.
 *
 * ============================================================================
 * FILE COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] ANTLR accepts the grammar.
 *
 * [ ] All token names match the canonical Zamani lexer.
 *
 * [ ] QualifiedNames resolves through the repository's canonical name grammar.
 *
 * [ ] `expressionCore` is supplied by the independent shared expression
 *     boundary.
 *
 * [ ] No parser grammar cycle exists.
 *
 * [ ] expressions.g4 imports this grammar exactly once.
 *
 * [ ] nestedMindExpression occurs exactly once in the public expression
 *     composition.
 *
 * [ ] No second assignment hierarchy exists.
 *
 * [ ] No second call hierarchy exists.
 *
 * [ ] No second precedence hierarchy exists.
 *
 * [ ] No cognitive operation catalogue is embedded.
 *
 * [ ] No target-specific syntax is embedded.
 *
 * [ ] No machine-capacity limit is embedded.
 *
 * [ ] No hardware catalogue is embedded.
 *
 * [ ] No quantum gate catalogue is embedded.
 *
 * [ ] No AI algorithm catalogue is embedded.
 *
 * [ ] No semantic actions exist.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] Positive parser tests pass.
 *
 * [ ] Negative parser tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass within available implementation resources.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Semantic resolution tests pass.
 *
 * [ ] Cross-domain lowering tests pass.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * The purpose of this file is:
 *
 *     generic nested computational cognition syntax
 *
 * not:
 *
 *     a second expression language
 *
 * not:
 *
 *     an AI-only language
 *
 * not:
 *
 *     a hardware language
 *
 * not:
 *
 *     a quantum language
 *
 * not:
 *
 *     a runtime
 *
 * not:
 *
 *     an IR.
 *
 * The invariant is:
 *
 *     nested mind syntax
 *         ->
 *     domain-neutral AST
 *         ->
 *     semantic operation resolution
 *         ->
 *     effects/capabilities/resources/contracts/policies/provenance
 *         ->
 *     canonical semantic representation
 *         ->
 *     appropriate domain IR
 *         ->
 *     target realization
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */