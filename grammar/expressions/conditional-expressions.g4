/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/conditional-expressions.g4
 *
 * Status:
 *     Canonical modular conditional-expression grammar component.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar component.
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only.
 *     No unsafe Rust is required or permitted by the compiler implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the source-level syntax for VALUE-PRODUCING conditional
 * expressions.
 *
 * It covers:
 *
 *     1. `if` conditional expressions;
 *     2. `else if` conditional branches;
 *     3. optional final `else` branches;
 *     4. ternary `condition ? then : else` expressions;
 *     5. arbitrary nesting;
 *     6. arbitrary branch-chain length;
 *     7. arbitrary expression complexity within conditions and branches.
 *
 * This file defines syntax only.
 *
 * It does NOT define:
 *
 *     - type checking;
 *     - boolean coercion;
 *     - branch reachability;
 *     - constant evaluation;
 *     - effect checking;
 *     - capability checking;
 *     - resource allocation;
 *     - scheduling;
 *     - routing;
 *     - quantum execution;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - physical hardware;
 *     - runtime execution;
 *     - backend selection;
 *     - optimization;
 *     - branch prediction;
 *     - CFG construction;
 *     - SSA construction.
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - conditional-expression syntax;
 *     - `if` expression syntax;
 *     - `else if` expression syntax;
 *     - optional `else` expression syntax;
 *     - ternary conditional syntax;
 *     - composition of conditional expressions with the canonical expression
 *       hierarchy.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - statement-level `if`;
 *     - blocks;
 *     - expressions generally;
 *     - assignment;
 *     - arithmetic;
 *     - comparison;
 *     - logical operators;
 *     - pattern matching;
 *     - loops;
 *     - functions;
 *     - calls;
 *     - indexing;
 *     - types;
 *     - semantic analysis;
 *     - IR.
 *
 * Existing ownership:
 *
 *     grammar/expressions/expressions.g4
 *         canonical expression composition.
 *
 *     grammar/core/blocks.g4
 *         canonical blockExpression syntax.
 *
 *     grammar/statements/conditionals.g4
 *         statement-level conditional control flow.
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         canonical lexical/token authority.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * The intended authority chain is:
 *
 *     language specification
 *             |
 *             v
 *     modular grammar components
 *             |
 *             v
 *     grammar/expressions/expressions.g4
 *             |
 *             v
 *     grammar/Zamani.g4
 *             |
 *             v
 *     canonical parser
 *             |
 *             v
 *     domain-neutral frontend AST
 *             |
 *             v
 *     semantic analysis
 *             |
 *             v
 *     canonical semantic model / ZUIR
 *             |
 *             +-------------------+--------------------+
 *             |                   |                    |
 *             v                   v                    v
 *        classical           quantum::ir          HDL/hardware
 *             |                   |                    |
 *             +-------------------+--------------------+
 *                         |
 *                         v
 *                optimization/lowering
 *                         |
 *                    routing/scheduling
 *                         |
 *                    resilience/QEC/ZQN
 *                         |
 *                         HAL
 *                         |
 *                    target realization
 *
 * `grammar/Zamani-Grammar.md` is not an independent syntax authority.
 *
 * `grammar/grammar.md` describes implementation conformance and must not
 * silently define syntax that is absent from the authoritative grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Conditional expressions describe PROGRAM MEANING.
 *
 * They do not describe the physical machine that realizes that meaning.
 *
 * Consequently this grammar imposes no universal maximum on:
 *
 *     - branch count;
 *     - conditional nesting depth;
 *     - expression size;
 *     - program size;
 *     - machine count;
 *     - CPU count;
 *     - GPU count;
 *     - FPGA count;
 *     - QPU count;
 *     - qubit count;
 *     - memory capacity;
 *     - tensor dimensions;
 *     - distributed participants;
 *     - execution targets.
 *
 * "Infinity" means that this grammar introduces no artificial finite
 * language-level resource limit.
 *
 * Actual parser/compiler/runtime resource limits are implementation policies
 * and must never change the language's semantic meaning.
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * No lexer rules are defined here.
 *
 * All tokens come from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Relevant canonical tokens include:
 *
 *     IF
 *     ELSE
 *
 *     QUESTION_MARK
 *     COLON
 *
 *     LBRACE
 *     RBRACE
 *
 * The lexical spelling of these tokens is exclusively owned by the lexer.
 *
 * This file MUST NOT introduce:
 *
 *     IF : 'if' ;
 *     ELSE : 'else' ;
 *
 * or equivalent duplicate lexer rules.
 *
 * ============================================================================
 * IMPORTANT TOKEN CORRECTION
 * ============================================================================
 *
 * Earlier modular expression material incorrectly used generic names such as:
 *
 *     QUESTION
 *
 * where the repository's canonical lexer vocabulary uses:
 *
 *     QUESTION_MARK
 *
 * The canonical lexer vocabulary must be used here.
 *
 * Likewise, this file does not invent:
 *
 *     LESS
 *     GREATER
 *     AND_AND
 *     OR_OR
 *
 * or any other operator token.
 *
 * Those operators belong to their respective expression layers.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * The canonical expression aggregate is:
 *
 *     grammar/expressions/expressions.g4
 *
 * That file currently contains the expression hierarchy and must integrate
 * this component.
 *
 * This file MUST NOT redefine the public `expression` rule.
 *
 * The intended composition is:
 *
 *     expression
 *         |
 *         v
 *     assignmentExpression
 *         |
 *         v
 *     conditionalExpression
 *         |
 *         +-----------------------+
 *         |                       |
 *         v                       v
 *     ifExpression        ternaryExpression
 *         |                       |
 *         v                       v
 *     range/lower expressions / canonical expression
 *
 * The exact lower-precedence hierarchy remains owned by
 * `grammar/expressions/expressions.g4`.
 *
 * ============================================================================
 * CONDITIONAL EXPRESSION MODEL
 * ============================================================================
 *
 * Zamani has TWO source-level conditional expression forms:
 *
 *     A. structured `if` expression
 *
 *         if condition {
 *             then_value
 *         } else {
 *             else_value
 *         }
 *
 *     B. ternary expression
 *
 *         condition ? then_value : else_value
 *
 * They are semantically different syntactic forms but both produce an
 * expression value.
 *
 * The AST may normalize both forms into the appropriate source-level
 * conditional representation during frontend lowering.
 *
 * ============================================================================
 * OPTIONAL ELSE
 * ============================================================================
 *
 * The existing native AST explicitly permits:
 *
 *     else_branch: Option<NodeId>
 *
 * Therefore the grammar MUST preserve the possibility of an omitted `else`.
 *
 * This is deliberately a syntax/AST decision.
 *
 * Semantic analysis decides whether:
 *
 *     if condition {
 *         value
 *     }
 *
 * is legal in a particular value-producing context.
 *
 * For example, semantic analysis may determine that an omitted else is valid
 * only when the expression's result is not required, or may assign an
 * appropriate unit/optional/partial-value semantic according to the language
 * specification.
 *
 * The grammar must not invent that semantic rule.
 *
 * This corrects the previous grammar contract that required a final `else`.
 *
 * ============================================================================
 * STRUCTURED IF EXPRESSION
 * ============================================================================
 *
 * A structured conditional expression has the form:
 *
 *     if <condition> <block>
 *
 * with an optional alternative:
 *
 *     if <condition> <block>
 *     else <block>
 *
 * or a chained alternative:
 *
 *     if <condition_a> <block_a>
 *     else if <condition_b> <block_b>
 *     else <block_c>
 *
 * No fixed number of `else if` branches is encoded.
 *
 * ============================================================================
 * IF EXPRESSION
 * ============================================================================
 */

ifExpression
    : IF expression blockExpression
      (
          ELSE
          (
              ifExpression
            | blockExpression
          )
      )?
    ;

/*
 * ============================================================================
 * STRUCTURED CONDITIONAL BRANCH
 * ============================================================================
 *
 * The branch is deliberately a `blockExpression`.
 *
 * Block ownership remains:
 *
 *     grammar/core/blocks.g4
 *
 * This file MUST NOT redefine:
 *
 *     blockExpression
 *
 * This preserves one block syntax for:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     distributed
 *     AI
 *     accelerator
 *     embedded
 *     future
 *
 * computational domains.
 *
 * ============================================================================
 * TERNARY CONDITIONAL
 * ============================================================================
 *
 * Ternary conditionals are right-associative.
 *
 * Examples:
 *
 *     condition ? a : b
 *
 *     a ? b : c ? d : e
 *
 * The latter is interpreted structurally as:
 *
 *     a ? b : (c ? d : e)
 *
 * This is essential for deterministic parsing and conventional conditional
 * association.
 *
 * The condition is consumed from the expression layer immediately below
 * ternary conditional syntax.
 *
 * The true and false branches consume the canonical `expression` rule so that
 * nested conditionals are naturally representable.
 *
 * The exact lower-precedence rule name is deliberately kept at the integration
 * boundary rather than duplicated here.
 *
 * ============================================================================
 */

ternaryConditionalExpression
    : conditionalOperand
      (
          QUESTION_MARK
          expression
          COLON
          expression
      )?
    ;

/*
 * ============================================================================
 * CONDITIONAL OPERAND
 * ============================================================================
 *
 * `conditionalOperand` is an integration rule.
 *
 * It MUST be bound by the canonical expression hierarchy to the expression
 * layer immediately below ternary conditional syntax.
 *
 * In the current expression architecture this is the range/lower expression
 * layer.
 *
 * Do not duplicate:
 *
 *     rangeExpression
 *     logicalOrExpression
 *     logicalAndExpression
 *     equalityExpression
 *     arithmeticExpression
 *     prefixExpression
 *     postfixExpression
 *     primaryExpression
 *
 * here.
 *
 * The canonical composition file decides the exact rule wiring.
 *
 * ============================================================================
 * CANONICAL COMPOSITION CONTRACT
 * ============================================================================
 *
 * `grammar/expressions/expressions.g4` should expose the following conceptual
 * composition:
 *
 *     conditionalExpression
 *         : ifExpression
 *         | ternaryConditionalExpression
 *         ;
 *
 * followed by the appropriate higher/lower precedence wiring.
 *
 * IMPORTANT:
 *
 * This file intentionally does NOT define `expression`.
 *
 * It therefore cannot accidentally become a second expression hierarchy.
 *
 * ============================================================================
 */

conditionalExpression
    : ifExpression
    | ternaryConditionalExpression
    ;

/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis must establish:
 *
 *     - condition type;
 *     - truth-value interpretation;
 *     - branch result types;
 *     - type unification/coercion;
 *     - branch reachability;
 *     - effects;
 *     - capabilities;
 *     - ownership;
 *     - borrowing;
 *     - resource requirements;
 *     - domain legality;
 *     - control-flow semantics.
 *
 * The grammar MUST NOT require a condition to be a particular Rust type,
 * compiler type, hardware predicate, or backend representation.
 *
 * ============================================================================
 * CONDITION SEMANTICS
 * ============================================================================
 *
 * Examples:
 *
 *     if ready {
 *         value
 *     }
 *
 *     if measurement_result {
 *         quantum_path()
 *     } else {
 *         classical_path()
 *     }
 *
 *     if accelerator_available {
 *         accelerated()
 *     } else {
 *         portable()
 *     }
 *
 * Whether any of these conditions are semantically valid is determined after
 * parsing.
 *
 * In particular, the grammar does not decide whether a quantum measurement
 * result may control a subsequent operation.
 *
 * That belongs to semantic analysis and, eventually, `quantum::ir`.
 *
 * ============================================================================
 * BRANCH TYPE SEMANTICS
 * ============================================================================
 *
 * Branches may have different syntax.
 *
 * Examples:
 *
 *     if condition {
 *         1
 *     } else {
 *         2
 *     }
 *
 *     if condition {
 *         compute()
 *     } else {
 *         fallback()
 *     }
 *
 *     if condition {
 *         quantum_value()
 *     } else {
 *         classical_value()
 *     }
 *
 * The grammar does not impose type equality.
 *
 * Semantic analysis decides whether the branch result types are compatible.
 *
 * ============================================================================
 * BLOCK SEMANTICS
 * ============================================================================
 *
 * A block may contain the language constructs permitted by the canonical block
 * grammar.
 *
 * The grammar does not determine:
 *
 *     - which statements may execute;
 *     - whether a final expression supplies a value;
 *     - whether an early return is legal;
 *     - whether a branch diverges;
 *     - whether an effect is permitted.
 *
 * Those are semantic responsibilities.
 *
 * ============================================================================
 * NESTING
 * ============================================================================
 *
 * Structured conditionals can nest:
 *
 *     if a {
 *         if b {
 *             x
 *         } else {
 *             y
 *         }
 *     } else {
 *         z
 *     }
 *
 * Ternary conditionals can nest:
 *
 *     a ? (b ? c : d) : e
 *
 * Mixed forms can nest:
 *
 *     if a {
 *         b ? c : d
 *     } else {
 *         if e {
 *             f
 *         } else {
 *             g
 *         }
 *     }
 *
 * No nesting depth is encoded.
 *
 * ============================================================================
 * ELSE-IF ASSOCIATION
 * ============================================================================
 *
 * The structured rule:
 *
 *     ELSE (ifExpression | blockExpression)
 *
 * makes:
 *
 *     else if condition { ... } else { ... }
 *
 * structurally equivalent to:
 *
 *     else (
 *         if condition { ... } else { ... }
 *     )
 *
 * This avoids a finite enumeration of:
 *
 *     elseIf1
 *     elseIf2
 *     elseIf3
 *     ...
 *
 * and therefore introduces no artificial branch-count limit.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     - source;
 *     - token sequence;
 *     - grammar version;
 *     - parser configuration;
 *
 * the parse structure must be deterministic.
 *
 * Parsing MUST NOT depend on:
 *
 *     - CPU count;
 *     - GPU count;
 *     - QPU availability;
 *     - physical topology;
 *     - memory capacity;
 *     - network state;
 *     - calibration state;
 *     - scheduler state;
 *     - runtime state;
 *     - randomness;
 *     - current time.
 *
 * ============================================================================
 * POCO-REAF / TARGET INDEPENDENCE
 * ============================================================================
 *
 * The following are valid semantic patterns:
 *
 *     if capability("tensor.compute") {
 *         compute()
 *     } else {
 *         fallback()
 *     }
 *
 *     if requires("quantum.measurement") {
 *         quantum_path()
 *     } else {
 *         classical_path()
 *     }
 *
 * The grammar does not encode how those capabilities are satisfied.
 *
 * A conditional may eventually lower to:
 *
 *     classical branch
 *     quantum-classical feedback
 *     accelerator control
 *     distributed control
 *     HDL control
 *     runtime dispatch
 *     compile-time specialization
 *     another future realization
 *
 * without changing the source grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A conditional expression may occur around quantum-related source syntax.
 *
 * Example:
 *
 *     if condition {
 *         apply H to q
 *     } else {
 *         apply X to q
 *     }
 *
 * The grammar does NOT determine:
 *
 *     - whether `condition` is classical;
 *     - whether it derives from measurement;
 *     - whether dynamic control is supported;
 *     - how many qubits are involved;
 *     - which physical qubits are used;
 *     - how operations are routed;
 *     - when operations execute.
 *
 * Downstream pipeline:
 *
 *     source
 *       ↓
 *     frontend AST
 *       ↓
 *     semantic analysis
 *       ↓
 *     canonical semantic model / ZUIR
 *       ↓
 *     quantum::ir
 *       ↓
 *     optimization
 *       ↓
 *     routing
 *       ↓
 *     scheduling
 *       ↓
 *     QEC / resilience / ZQN
 *       ↓
 *     HAL
 *       ↓
 *     physical target
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar introduces no quantum IR.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * Conditional expressions may appear in hardware/software co-design contexts.
 *
 * Examples:
 *
 *     if enable {
 *         generate()
 *     } else {
 *         idle()
 *     }
 *
 *     width > threshold ? fast_path : wide_path
 *
 * This grammar does not encode:
 *
 *     - FPGA resources;
 *     - ASIC cell counts;
 *     - register widths;
 *     - clock frequency;
 *     - physical pins;
 *     - timing closure;
 *     - placement;
 *     - routing.
 *
 * Those are downstream semantic/resource/HDL responsibilities.
 *
 * ============================================================================
 * DISTRIBUTED / PARALLEL INTEGRATION
 * ============================================================================
 *
 * A conditional may control a computation whose semantic realization later
 * becomes distributed or parallel.
 *
 * The grammar does not impose:
 *
 *     - node count;
 *     - process count;
 *     - worker count;
 *     - task count;
 *     - communication topology.
 *
 * ============================================================================
 * AI / DATA / TENSOR INTEGRATION
 * ============================================================================
 *
 * Conditions can be used around:
 *
 *     model inference
 *     tensor operations
 *     data processing
 *     training
 *     symbolic computation
 *     accelerator selection
 *
 * No tensor rank, shape, batch size, accelerator count, or model size is
 * encoded here.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Conditional syntax must remain separate from resource realization.
 *
 * These concepts are different:
 *
 *     requirement
 *     capability
 *     preference
 *     hint
 *     implementation decision
 *
 * The grammar only represents the conditional structure.
 *
 * Semantic/resource analysis decides whether an expression's condition
 * references one of those concepts and how it should be interpreted.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The repository already provides:
 *
 *     src/frontend/ast/node/expressions/conditional.rs
 *
 * Its structural representation is:
 *
 *     ConditionalExpression
 *         condition: NodeId
 *         then_branch: NodeId
 *         else_branch: Option<NodeId>
 *
 * The grammar must therefore preserve:
 *
 *     condition
 *     then branch
 *     optional else branch
 *
 * and their source ordering.
 *
 * The parser/frontend builder is responsible for constructing the existing
 * AST node.
 *
 * This grammar MUST NOT introduce:
 *
 *     QuantumConditional
 *     ClassicalConditional
 *     HardwareConditional
 *     PhysicalQubitConditional
 *     GPUConditional
 *
 * or other domain-specific AST variants.
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * The frontend AST must preserve the complete source span of the conditional.
 *
 * The parser should also preserve child spans through their child nodes.
 *
 * The grammar itself does not construct spans.
 *
 * Required source regions include:
 *
 *     condition
 *     then branch
 *     else-if condition
 *     else branch
 *     complete conditional expression
 *
 * ============================================================================
 * STRUCTURAL VALIDATION
 * ============================================================================
 *
 * Structural validation belongs to:
 *
 *     src/frontend/ast/node/validation/
 *
 * It may verify:
 *
 *     - required condition child exists;
 *     - required then child exists;
 *     - optional else child is structurally valid;
 *     - child identities are valid;
 *     - source span invariants hold;
 *     - node kind is correct.
 *
 * It must not perform:
 *
 *     - type checking;
 *     - capability resolution;
 *     - hardware discovery;
 *     - quantum routing;
 *     - scheduling.
 *
 * ============================================================================
 * SEMANTIC / IR INTEGRATION
 * ============================================================================
 *
 * This grammar does not directly depend on:
 *
 *     ZUIR
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime
 *
 * The dependency direction is:
 *
 *     grammar
 *       ↓
 *     AST
 *       ↓
 *     semantic analysis
 *       ↓
 *     canonical semantic model / IR
 *       ↓
 *     domain lowering
 *
 * There must be no reverse dependency from this grammar into those systems.
 *
 * ============================================================================
 * STATEMENT INTEGRATION
 * ============================================================================
 *
 * `grammar/statements/conditionals.g4` owns statement-level conditional
 * constructs.
 *
 * It must NOT redefine this file's:
 *
 *     conditionalExpression
 *     ifExpression
 *     ternaryConditionalExpression
 *
 * A statement-level conditional and a value-producing conditional are distinct
 * grammatical concepts even though both use the lexical token `IF`.
 *
 * ============================================================================
 * BLOCK INTEGRATION
 * ============================================================================
 *
 * `grammar/core/blocks.g4` owns:
 *
 *     blockExpression
 *
 * This file consumes it.
 *
 * No duplicate:
 *
 *     blockExpression
 *     block
 *     blockBody
 *
 * rules should be introduced here.
 *
 * ============================================================================
 * PATTERN MATCHING
 * ============================================================================
 *
 * Pattern matching belongs to the match-expression grammar.
 *
 * Do not add:
 *
 *     if let
 *     if pattern
 *
 * here unless the language specification explicitly defines such syntax and
 * assigns ownership to this file.
 *
 * Pattern semantics remain owned by the pattern/match subsystem.
 *
 * ============================================================================
 * EFFECTS
 * ============================================================================
 *
 * A conditional may contain effectful expressions or effectful blocks.
 *
 * This grammar does not prohibit or approve them.
 *
 * Effect legality belongs to semantic/effect analysis.
 *
 * ============================================================================
 * ASYNC / CONCURRENCY
 * ============================================================================
 *
 * A conditional may contain asynchronous expressions where the canonical
 * expression grammar permits them.
 *
 * This grammar does not define:
 *
 *     await
 *     spawn
 *     parallel
 *     actor
 *     channel
 *     scheduler
 *
 * Those remain owned by their respective grammar domains.
 *
 * ============================================================================
 * MACROS / METAPROGRAMMING
 * ============================================================================
 *
 * Conditional expressions may occur in macro/metaprogramming syntax when
 * those subsystems explicitly consume `expression`.
 *
 * This file does not execute macros or compile-time code.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing this grammar performs no:
 *
 *     - filesystem I/O;
 *     - network I/O;
 *     - process execution;
 *     - device probing;
 *     - hardware discovery;
 *     - runtime evaluation.
 *
 * Conditional syntax therefore remains deterministic and side-effect free at
 * the grammar layer.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Parser errors include malformed syntax such as:
 *
 *     if
 *     if condition
 *     if condition {
 *     if condition { value } else
 *     else { value }
 *     condition ? value
 *     condition ? : value
 *
 * Semantic errors do NOT belong here.
 *
 * Examples:
 *
 *     condition has non-Boolean type
 *     branch types cannot be unified
 *     condition references an unavailable capability
 *     quantum feedback is unsupported by a selected target
 *     branch requires unavailable resources
 *
 * Those are downstream diagnostics.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing structured conditional syntax:
 *
 *     if condition { ... } else { ... }
 *
 * remains valid.
 *
 * Existing chained syntax:
 *
 *     if a { ... } else if b { ... } else { ... }
 *
 * remains valid.
 *
 * Existing ternary syntax:
 *
 *     condition ? a : b
 *
 * remains valid.
 *
 * The addition of an optional final `else` for structured conditional
 * expressions is compatible with the existing AST model and must be reflected
 * consistently in the canonical expression grammar and semantic specification.
 *
 * ============================================================================
 * NO HARD-CODING
 * ============================================================================
 *
 * Forbidden language-level constants include concepts such as:
 *
 *     MAX_BRANCHES
 *     MAX_CONDITIONAL_DEPTH
 *     MAX_CONDITIONAL_EXPRESSIONS
 *     MAX_AST_NODES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * None belong in this grammar.
 *
 * Practical parser-resource limits, if needed for denial-of-service protection
 * or compiler resource management, belong outside the language semantics.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar uses:
 *
 *     repetition;
 *     recursion;
 *     structural composition;
 *
 * rather than finite enumeration.
 *
 * Therefore it supports arbitrarily long source-level conditional chains
 * subject only to external implementation/resource constraints.
 *
 * Examples:
 *
 *     if c1 { v1 }
 *     else if c2 { v2 }
 *     else if c3 { v3 }
 *     ...
 *     else { vn }
 *
 * are represented structurally rather than by a fixed number of grammar
 * alternatives.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests:
 *
 *     1. `if condition { value }`
 *     2. `if condition { value } else { value }`
 *     3. `if a { x } else if b { y }`
 *     4. `if a { x } else if b { y } else { z }`
 *     5. nested if expression;
 *     6. nested ternary expression;
 *     7. mixed if/ternary expression;
 *     8. conditional as function argument;
 *     9. conditional as initializer;
 *    10. conditional as return expression;
 *    11. conditional as indexing expression;
 *    12. conditional around classical computation;
 *    13. conditional around quantum syntax;
 *    14. conditional around hybrid computation;
 *    15. conditional in HDL-compatible expression context;
 *    16. long else-if chain;
 *    17. deeply nested conditional expression;
 *    18. large branch expressions.
 *
 * Required negative tests:
 *
 *     1. missing condition;
 *     2. missing block;
 *     3. malformed `else`;
 *     4. malformed `else if`;
 *     5. missing ternary true expression;
 *     6. missing ternary colon;
 *     7. missing ternary false expression;
 *     8. unmatched braces;
 *     9. invalid token between condition and branch;
 *    10. invalid token between `else` and branch.
 *
 * Semantic tests, downstream rather than grammar tests:
 *
 *     - invalid condition type;
 *     - incompatible branch types;
 *     - invalid effects;
 *     - invalid capability use;
 *     - invalid resource requirement;
 *     - invalid quantum feedback;
 *     - invalid hardware realization.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Parse the same source repeatedly and verify identical:
 *
 *     token stream;
 *     parse tree;
 *     AST structure;
 *     child ordering;
 *     source spans.
 *
 * Parsing must not depend on:
 *
 *     hardware;
 *     environment;
 *     current time;
 *     randomness;
 *     scheduler state.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Include generated fixtures for:
 *
 *     - long conditional chains;
 *     - deeply nested conditionals;
 *     - large branch expressions;
 *     - large program-level conditional density.
 *
 * Tests must verify that no semantic language limit has accidentally been
 * introduced.
 *
 * External resource limits must be tested separately from language semantics.
 *
 * ============================================================================
 * ROUND-TRIP TESTS
 * ============================================================================
 *
 * Where the repository formatter/printer supports conditional expressions:
 *
 *     source
 *       ↓
 *     lexer
 *       ↓
 *     parser
 *       ↓
 *     AST
 *       ↓
 *     formatter
 *       ↓
 *     parser
 *
 * must preserve conditional meaning.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] Ownership is explicitly defined.
 *     [x] Lexer authority is not duplicated.
 *     [x] Block authority is not duplicated.
 *     [x] Statement-level conditionals are separated.
 *     [x] `if` expressions are represented.
 *     [x] `else if` chains are represented without finite limits.
 *     [x] Optional `else` is represented consistently with the existing AST.
 *     [x] Ternary conditional syntax is represented.
 *     [x] Ternary nesting is right-associative by structure.
 *     [x] No machine/resource limits are encoded.
 *     [x] No quantum-specific grammar is introduced.
 *     [x] No IR is introduced.
 *     [x] No runtime behavior is introduced.
 *     [x] POCO-REAF is preserved.
 *     [x] AST integration is predetermined.
 *     [x] semantic integration is predetermined.
 *     [x] IR integration is predetermined.
 *     [x] quantum::ir remains the canonical quantum boundary.
 *     [x] Rust 1.97/1.97.1 compatibility is documented.
 *     [x] safe-Rust/no-unsafe requirement is preserved downstream.
 *     [x] scalability and boundary requirements are documented.
 *     [x] deterministic parsing requirements are documented.
 *
 * ============================================================================
 * REQUIRED COMPOSITION CHANGE
 * ============================================================================
 *
 * This file deliberately does NOT redefine `expression`.
 *
 * The one canonical integration point that must be made in:
 *
 *     grammar/expressions/expressions.g4
 *
 * is to make its existing conditional layer delegate to this contract rather
 * than maintaining an independent conditional implementation.
 *
 * Conceptually:
 *
 *     conditionalExpression
 *         : ifExpression
 *         | ternaryConditionalExpression
 *         ;
 *
 * The existing conditional implementation in `expressions.g4` must not remain
 * as a competing definition.
 *
 * The root:
 *
 *     grammar/Zamani.g4
 *
 * continues to consume the canonical expression aggregate.
 *
 * No rename of existing files is required.
 *
 * ============================================================================
 */