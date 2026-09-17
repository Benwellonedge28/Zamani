/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/expressions/conditionals.g4
* 
* Status:
* Canonical modular grammar component for value-producing conditional
* expressions.
* 
* Grammar technology:
* ANTLR4 parser grammar.
* 
* Rust implementation baseline:
* Rust 1.97 / Rust 1.97.1
* Edition 2021.
* 
* Safety:
* Grammar contains no embedded Rust actions.
* Grammar contains no semantic predicates.
* Grammar contains no unsafe Rust.
* Compiler/frontend implementation must use safe Rust only.
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file is the SINGLE MODULAR SYNTAX OWNER for VALUE-PRODUCING
* conditional expressions.
* 
* It defines:
* 
* - conditionalExpression
* - structured if expressions
* - else-if chains
* - optional else branches
* - ternary conditional expressions
* - nesting of conditional expressions
* 
* It does NOT define:
* 
* - statement-level if control flow
* - general expression syntax
* - expression precedence outside this construct
* - blocks
* - statements
* - types
* - semantic analysis
* - AST structures
* - IR
* - quantum IR
* - QEC
* - ZQN
* - routing
* - scheduling
* - HAL
* - hardware selection
* - runtime execution
* - optimization
* 
* ============================================================================
* SINGLE-AUTHORITY RULE
* ============================================================================
* 
* There MUST be exactly one authoritative modular definition of:
* 
* conditionalExpression
* 
* in grammar/expressions/.
* 
* This file is that authority.
* 
* The existing:
* 
* grammar/expressions/conditional-expressions.g4
* 
* MUST NOT independently define conditionalExpression, ifExpression,
* ternaryConditionalExpression, or another competing conditional-expression
* hierarchy once this file is authoritative.
* 
* It may be retained temporarily as:
* 
* - migration documentation;
* - compatibility documentation;
* - a non-authoritative wrapper;
* - a reference to this file.
* 
* It must not create a second parser definition.
* 
* ============================================================================
* REPOSITORY ARCHITECTURE
* ============================================================================
* 
* Canonical language pipeline:
* 
* source
*   |
*   v
* grammar/antlr/ZamaniLexer.g4
*   |
*   v
* canonical parser composition
*   |
*   v
* domain-neutral frontend AST
*   |
*   v
* structural validation
*   |
*   v
* semantic analysis
*   |
*   v
* canonical semantic model / ZUIR
*   |
*   +-----------------------+----------------------+
*   |                       |                      |
*   v                       v                      v
* classical              quantum::ir           HDL/hardware
*   |                       |                      |
*   +-----------------------+----------------------+
*                           |
*                           v
*                optimization / lowering
*                           |
*                routing / scheduling
*                           |
*                resilience / QEC / ZQN
*                           |
*                          HAL
*                           |
*                   target realization
* 
* "quantum::ir" remains the canonical quantum semantic boundary.
* 
* This grammar does not create or define any IR.
* 
* ============================================================================
* FILE OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* conditionalExpression
* ifExpression
* elseIfExpressionBranch
* elseExpressionBranch
* ternaryConditionalExpression
* 
* THIS FILE DOES NOT OWN:
* 
* expression
* assignmentExpression
* rangeExpression
* logical expressions
* arithmetic expressions
* comparison expressions
* blocks
* statements
* types
* declarations
* lexer tokens
* AST types
* semantic rules
* IR
* 
* ============================================================================
* UPSTREAM CONTRACTS
* ============================================================================
* 
* Lexer:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* supplies:
* 
* IF
* ELSE
* QUESTION
* COLON
* LBRACE
* RBRACE
* 
* This file MUST NOT define lexer rules.
* 
* The repository's canonical lexer currently uses:
* 
* QUESTION
* 
* for "?".
* 
* Therefore this grammar intentionally uses QUESTION rather than introducing
* QUESTION_MARK or QUESTION as a second lexical vocabulary.
* 
* ============================================================================
* BLOCK CONTRACT
* ============================================================================
* 
* Block syntax is owned by:
* 
* grammar/core/blocks.g4
* 
* That grammar owns:
* 
* block
* blockExpression
* blockElement
* 
* This file consumes "blockExpression".
* 
* This file MUST NOT redefine:
* 
* block
* blockExpression
* blockElement
* 
* ============================================================================
* EXPRESSION CONTRACT
* ============================================================================
* 
* General expression syntax is owned by the canonical expression composition.
* 
* This file consumes:
* 
* expression
* 
* but does not redefine it.
* 
* The canonical expression composition is responsible for wiring this
* conditional-expression component into the complete expression hierarchy.
* 
* Conceptually:
* 
* expression
*     |
*     v
* assignmentExpression
*     |
*     v
* conditionalExpression
*     |
*     +------------------------+
*     |                        |
*     v                        v
* ifExpression       ternaryConditionalExpression
* 
* Lower-precedence expression layers remain owned by the canonical expression
* composition.
* 
* ============================================================================
* IMPORTANT COMPOSITION REQUIREMENT
* ============================================================================
* 
* ANTLR parser grammars cannot resolve arbitrary references merely because a
* similarly named .g4 file exists in another directory.
* 
* Therefore this component must be incorporated into the repository's actual
* parser composition mechanism.
* 
* The composition layer must provide:
* 
* expression
* blockExpression
* 
* and must import/include this component exactly once.
* 
* No implementation should attempt to compile this file as an unrelated,
* complete parser while simultaneously compiling another parser containing
* another conditionalExpression definition.
* 
* ============================================================================
* CONDITIONAL EXPRESSION SEMANTICS
* ============================================================================
* 
* A conditional expression is a source construct that may produce a value.
* 
* Structured form:
* 
* if condition {
*     then_value
* } else {
*     else_value
* }
* 
* Optional else:
* 
* if condition {
*     then_value
* }
* 
* Chained form:
* 
* if condition_a {
*     value_a
* } else if condition_b {
*     value_b
* } else {
*     value_c
* }
* 
* Ternary form:
* 
* condition ? then_value : else_value
* 
* The grammar establishes structure only.
* 
* Semantic analysis determines whether a particular conditional expression is
* legal in its context.
* 
* ============================================================================
* CONDITIONAL EXPRESSION
* ============================================================================
* 
* This is the ONLY public conditional-expression rule owned by this file.
* 
* It deliberately separates the two source forms:
* 
* structured if
* ternary
* 
* Both are expressions.
* 
* Neither is a statement-level conditional.
* 
* ============================================================================
  */

parser grammar ConditionalsParser;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* 1. CANONICAL CONDITIONAL EXPRESSION
* ============================================================================
* 
* A conditional expression is either:
* 
* - a structured if expression; or
* - a ternary conditional expression.
* 
* There is no third competing conditional hierarchy.
  */
  conditionalExpression
  : ifExpression
  | ternaryConditionalExpression
  ;

/*

* ============================================================================
* 2. STRUCTURED IF EXPRESSION
* ============================================================================
* 
* Canonical forms:
* 
* if condition {
*     then_value
* }
* 
* if condition {
*     then_value
* } else {
*     else_value
* }
* 
* if condition_a {
*     value_a
* } else if condition_b {
*     value_b
* } else {
*     value_c
* }
* 
* The final else is optional at the grammar level.
* 
* Whether an omitted else is semantically valid is determined by semantic
* analysis and the Zamani type/control-flow specification.
* 
* This preserves compatibility with the frontend AST contract where an
* else branch may be optional.
  /
  ifExpression
  : IF expression blockExpression elseIfExpressionBranch elseExpressionBranch?
  ;

/*

* ============================================================================
* 3. ELSE-IF EXPRESSION BRANCH
* ============================================================================
* 
* Each branch is:
* 
* else if condition block
* 
* Repetition allows an arbitrary number of branches without enumerating:
* 
* elseIf1
* elseIf2
* elseIf3
* ...
* 
* No grammar-level branch-count limit exists.
  */
  elseIfExpressionBranch
  : ELSE IF expression blockExpression
  ;

/*

* ============================================================================
* 4. ELSE EXPRESSION BRANCH
* ============================================================================
* 
* The final alternative branch is:
* 
* else block
* 
* The branch is optional in "ifExpression".
  */
  elseExpressionBranch
  : ELSE blockExpression
  ;

/*

* ============================================================================
* 5. TERNARY CONDITIONAL EXPRESSION
* ============================================================================
* 
* Canonical form:
* 
* condition ? then_value : else_value
* 
* Both branches consume the canonical "expression" rule.
* 
* This permits nested expressions while keeping the lexical tokens owned by
* the canonical lexer.
* 
* Examples:
* 
* ready ? value_a : value_b
* 
* a ? b : c
* 
* a ? b : c ? d : e
* 
* function(
*     condition ? value_a : value_b
* )
* 
* The semantic layer determines type compatibility and legality.
  */
  ternaryConditionalExpression
  : expression QUESTION expression COLON expression
  ;

/*

* ============================================================================
* 6. IMPORTANT NOTE ABOUT TERNARY ASSOCIATIVITY
* ============================================================================
* 
* The repository's canonical expression hierarchy must determine the final
* precedence/associativity of ternary conditional syntax.
* 
* This component intentionally does not create a second expression hierarchy.
* 
* The canonical expression composition MUST ensure that:
* 
* a ? b : c ? d : e
* 
* receives the language-specified association.
* 
* If Zamani specifies right associativity, the semantic/AST structure must be:
* 
* a ? b : (c ? d : e)
* 
* rather than:
* 
* (a ? b : c) ? d : e
* 
* This must be established in the canonical expression composition rather than
* by maintaining two competing ternary implementations.
* 
* ============================================================================
  */

/*

* ============================================================================
* 7. NESTING
* ============================================================================
* 
* Structured conditional expressions may contain conditional expressions in:
* 
* - conditions;
* - branch blocks;
* - nested expressions;
* - function arguments;
* - indices;
* - member expressions;
* - assignments;
* - returns;
* - collection elements;
* - other domain expressions.
* 
* Example:
* 
* if outer_condition {
*     if inner_condition {
*         value_a
*     } else {
*         value_b
*     }
* } else {
*     value_c
* }
* 
* Ternary nesting:
* 
* a ? (b ? c : d) : e
* 
* Mixed nesting:
* 
* if a {
*     b ? c : d
* } else {
*     if e {
*         f
*     } else {
*         g
*     }
* }
* 
* No finite language-level nesting limit is encoded.
  */

/*

* ============================================================================
* 8. BLOCK OWNERSHIP
* ============================================================================
* 
* Every structured conditional branch uses:
* 
* blockExpression
* 
* from:
* 
* grammar/core/blocks.g4
* 
* This guarantees that conditional branches use the same source-level block
* representation as the rest of Zamani.
* 
* Blocks may therefore contain constructs from:
* 
* classical
* quantum
* hybrid
* HDL
* hardware
* distributed
* AI
* data
* networking
* security
* concurrency
* effects
* future domains
* 
* without requiring domain-specific conditional grammar.
  */

/*

* ============================================================================
* 9. STATEMENT/EXPRESSION SEPARATION
* ============================================================================
* 
* Statement-level conditionals are owned by:
* 
* grammar/statements/conditionals.g4
* 
* That file owns:
* 
* ifStatement
* elseIfClause
* elseClause
* 
* It MUST NOT define:
* 
* conditionalExpression
* ifExpression
* ternaryConditionalExpression
* 
* This file owns only value-producing conditional syntax.
* 
* Therefore:
* 
* if condition {
*     work();
* }
* 
* in statement position is handled by the statement grammar.
* 
* Whereas:
* 
* let result =
*     if condition {
*         value_a
*     } else {
*         value_b
*     };
* 
* in expression position is handled through this file.
* 
* The shared IF/ELSE lexical tokens do not make the two constructs the same
* semantic entity.
  */

/*

* ============================================================================
* 10. NO CONDITION-SPECIFIC BOOLEAN GRAMMAR
* ============================================================================
* 
* This grammar deliberately uses:
* 
* expression
* 
* as the condition.
* 
* It does not introduce:
* 
* booleanExpression
* conditionExpression
* quantumCondition
* classicalCondition
* hardwareCondition
* resourceCondition
* 
* Semantic analysis determines whether the resulting expression is a valid
* condition.
* 
* This allows conditional syntax to remain domain-neutral.
  */

/*

* ============================================================================
* 11. AST CONTRACT
* ============================================================================
* 
* Every conditional expression must lower into the existing domain-neutral
* frontend AST.
* 
* The frontend AST already identifies:
* 
* ConditionalExpression
* 
* as a canonical expression node kind.
* 
* The AST must preserve:
* 
* - complete source span;
* - condition;
* - then branch;
* - ordered else-if branches;
* - optional else branch;
* - ternary-vs-structured source form where required;
* - child ordering;
* - source locations;
* - syntactic nesting.
* 
* Conceptual structured representation:
* 
* ConditionalExpression {
*     condition: NodeId,
*     then_branch: NodeId,
*     else_if_branches: [
*         {
*             condition: NodeId,
*             branch: NodeId
*         },
*         ...
*     ],
*     else_branch: Option<NodeId>,
*     source_span: Span
* }
* 
* The exact Rust structure remains owned by:
* 
* src/frontend/ast/
* 
* This grammar must not define or duplicate that Rust type.
  */

/*

* ============================================================================
* 12. TERNARY AST CONTRACT
* ============================================================================
* 
* The frontend must preserve sufficient source information to distinguish:
* 
* if condition {
*     a
* } else {
*     b
* }
* 
* from:
* 
* condition ? a : b
* 
* unless the AST specification explicitly defines normalization between the
* forms.
* 
* If normalization occurs, it must preserve source spans and diagnostics.
* 
* Grammar changes must not force a new domain-specific AST hierarchy.
  */

/*

* ============================================================================
* 13. SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis owns:
* 
* - name resolution;
* - condition validity;
* - condition type checking;
* - truth-value interpretation;
* - branch result typing;
* - type unification;
* - contextual typing;
* - coercions/conversions;
* - reachability;
* - definite assignment;
* - ownership;
* - borrowing;
* - lifetime rules;
* - effect checking;
* - capability checking;
* - resource requirements;
* - domain legality;
* - control-flow semantics;
* - divergence;
* - constant evaluation.
* 
* None of those rules belong in this grammar.
  */

/*

* ============================================================================
* 14. OPTIONAL ELSE SEMANTICS
* ============================================================================
* 
* The grammar permits:
* 
* if condition {
*     value
* }
* 
* because the frontend AST contract permits an optional else branch.
* 
* Semantic analysis must determine whether the expression is valid in context.
* 
* Possible language semantics may include:
* 
* - unit-valued conditional;
* - optional result;
* - partial result;
* - context where the value is discarded;
* - never-returning branch;
* - another explicitly specified semantic model.
* 
* This grammar does not select among those possibilities.
* 
* The language specification must define the semantic rule.
  */

/*

* ============================================================================
* 15. BRANCH TYPE COMPATIBILITY
* ============================================================================
* 
* The grammar intentionally permits:
* 
* if condition {
*     integer_expression
* } else {
*     compatible_expression
* }
* 
* and:
* 
* if condition {
*     quantum_expression
* } else {
*     another_expression
* }
* 
* It does not require syntactic identity between branches.
* 
* Semantic analysis determines whether the branch results are compatible.
* 
* The grammar must never encode:
* 
* branch type == branch type
* 
* as a parser restriction.
  */

/*

* ============================================================================
* 16. EFFECTS AND CAPABILITIES
* ============================================================================
* 
* A condition or branch may participate in effectful computation.
* 
* Examples include:
* 
* if ready {
*     perform_effect()
* } else {
*     fallback()
* }
* 
* if measurement_result {
*     quantum_path()
* } else {
*     classical_path()
* }
* 
* if capability_available {
*     accelerated_path()
* } else {
*     portable_path()
* }
* 
* Whether these are legal is determined downstream by:
* 
* type/effect/capability/resource analysis.
* 
* This grammar remains unaware of those mechanisms.
  */

/*

* ============================================================================
* 17. QUANTUM INTEGRATION
* ============================================================================
* 
* Conditional expressions may participate in hybrid quantum/classical
* computation.
* 
* Example:
* 
* if measurement_result {
*     quantum_operation()
* } else {
*     classical_operation()
* }
* 
* The grammar does NOT decide:
* 
* - number of qubits;
* - logical qubits;
* - physical qubits;
* - QPU count;
* - gate set;
* - topology;
* - physical mapping;
* - calibration;
* - noise model;
* - QEC;
* - ZQN;
* - routing;
* - scheduling;
* - backend.
* 
* Downstream architecture remains:
* 
* source
*   -> frontend AST
*   -> semantic analysis
*   -> canonical semantic model
*   -> quantum::ir
*   -> optimization
*   -> routing/scheduling
*   -> QEC/ZQN/resilience
*   -> HAL
*   -> target realization
* 
* "quantum::ir" remains the canonical quantum semantic boundary.
* 
* This grammar must never introduce another quantum IR.
  */

/*

* ============================================================================
* 18. CLASSICAL INTEGRATION
* ============================================================================
* 
* Conditional expressions may control:
* 
* scalar computation
* vector computation
* matrix computation
* tensor computation
* symbolic computation
* numerical computation
* scientific computation
* accelerator computation
* 
* No classical target is selected by the grammar.
  */

/*

* ============================================================================
* 19. HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* Conditional expressions may occur wherever the canonical expression grammar
* permits expressions in HDL or hardware/software co-design contexts.
* 
* This grammar does not define:
* 
* cpuConditional
* gpuConditional
* fpgaConditional
* asicConditional
* qpuConditional
* 
* Hardware realization remains downstream.
  */

/*

* ============================================================================
* 20. DISTRIBUTED / PARALLEL INTEGRATION
* ============================================================================
* 
* Conditional expressions may participate in distributed or parallel programs.
* 
* The grammar imposes no limits on:
* 
* - participants;
* - processes;
* - workers;
* - tasks;
* - branches;
* - nesting;
* - execution regions.
* 
* Placement and scheduling remain downstream semantic/compiler/runtime
* responsibilities.
  */

/*

* ============================================================================
* 21. RESOURCE / CAPABILITY INDEPENDENCE
* ============================================================================
* 
* This grammar contains no:
* 
* MAX_BRANCHES
* MAX_NESTING
* MAX_EXPRESSIONS
* MAX_THREADS
* MAX_CORES
* MAX_GPUS
* MAX_FPGAS
* MAX_QPUS
* MAX_QUBITS
* MAX_NODES
* MAX_MEMORY
* MAX_ACCELERATORS
* 
* It contains no physical device identifiers.
* 
* It contains no topology constants.
* 
* It contains no fixed hardware dimensions.
* 
* Resource requirements and capabilities are semantic constructs elsewhere in
* the language.
  */

/*

* ============================================================================
* 22. POCO-REAF
* ============================================================================
* 
* Conditional expressions describe program semantics, not machine topology.
* 
* The same source construct can therefore be lowered to:
* 
* tiny embedded systems
* CPUs
* multicore systems
* GPUs
* FPGAs
* ASIC-oriented targets
* quantum systems
* hybrid systems
* distributed systems
* HPC systems
* heterogeneous systems
* future computational architectures
* 
* The grammar imposes no universal machine-size restriction.
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever remains a
* downstream compiler/runtime responsibility built on target-independent
* language semantics.
  */

/*

* ============================================================================
* 23. SCALABILITY
* ============================================================================
* 
* Branch chains use repetition:
* 
* elseIfExpressionBranch*
* 
* rather than a finite enumeration.
* 
* Nested expressions are represented through the canonical expression
* composition.
* 
* Branch body size is governed by the general block grammar.
* 
* No language-level finite maximum is introduced for:
* 
* - branch count;
* - nesting depth;
* - expression size;
* - program size.
* 
* "Infinity" means absence of an artificial grammar-level finite limit.
* 
* Actual parser/compiler/runtime limits may exist because physical resources
* are finite. Such limits are implementation/resource policies and MUST NOT
* alter the language's semantic definition.
  */

/*

* ============================================================================
* 24. DETERMINISM
* ============================================================================
* 
* Given identical:
* 
* - source;
* - token sequence;
* - grammar version;
* - parser configuration;
* 
* parsing must produce the same structural interpretation.
* 
* Parsing MUST NOT depend on:
* 
* - CPU availability;
* - GPU availability;
* - FPGA availability;
* - QPU availability;
* - machine topology;
* - memory capacity;
* - calibration state;
* - scheduler state;
* - network state;
* - runtime state;
* - current time;
* - randomness.

*/

/*

* ============================================================================
* 25. SECURITY
* ============================================================================
* 
* This grammar performs no:
* 
* - filesystem access;
* - network access;
* - process execution;
* - device discovery;
* - hardware probing;
* - runtime dispatch.
* 
* There are no embedded actions.
* 
* There is no embedded unsafe Rust.
* 
* Any macro/metaprogramming execution policy belongs to its own semantic and
* capability system.
  */

/*

* ============================================================================
* 26. ERROR AND DIAGNOSTIC CONTRACT
* ============================================================================
* 
* The parser/frontend diagnostic layer should be able to report:
* 
* - missing condition;
* - malformed condition;
* - missing branch block;
* - malformed else-if;
* - malformed else;
* - missing ternary then expression;
* - missing ternary colon;
* - missing ternary else expression;
* - incomplete conditional at EOF;
* - unexpected tokens between conditional clauses.
* 
* Diagnostics must preserve source spans.
* 
* Diagnostic wording and structured Rust error types belong to the frontend
* diagnostic subsystem.
* 
* No Rust actions are embedded in this grammar.
  */

/*

* ============================================================================
* 27. COMPATIBILITY CONTRACT
* ============================================================================
* 
* The historical monolithic Zamani grammar has conditional syntax.
* 
* The modular grammar must preserve its intended source forms without keeping
* duplicate definitions.
* 
* Migration:
* 
* historical Zamani.g4 conditional syntax
*            |
*            v
* grammar/expressions/conditionals.g4
*            |
*            v
* canonical expression composition
*            |
*            v
* frontend AST
* 
* "grammar/Zamani-Grammar.md" may document additional proposed syntax but does
* not independently authorize syntax.
* 
* "grammar/grammar.md" records implementation conformance and must not silently
* introduce syntax absent from the canonical grammar.
  */

/*

* ============================================================================
* 28. RELATIONSHIP TO conditional-expressions.g4
* ============================================================================
* 
* IMPORTANT MIGRATION RULE:
* 
* grammar/expressions/conditionals.g4
* 
* is the canonical owner.
* 
* grammar/expressions/conditional-expressions.g4
* 
* must NOT define a competing:
* 
* conditionalExpression
* 
* rule.
* 
* The existing repository contains overlapping conditional-expression
* definitions. Those must be consolidated.
* 
* The existing conditional-expressions.g4 may be retained without renaming as
* a compatibility/reference file, but its grammar authority must be removed.
* 
* Preferred eventual relationship:
* 
* conditionals.g4
*     |
*     +--> canonical conditional-expression rules
* 
* conditional-expressions.g4
*     |
*     +--> compatibility/reference documentation
* 
* There must be only one effective parser rule implementation.
  */

/*

* ============================================================================
* 29. RELATIONSHIP TO expressions.g4 / expression.g4
* ============================================================================
* 
* The repository currently contains expression-composition material whose
* content identifies "expression" as the public expression entry point and
* "conditionalExpression" as part of the expression hierarchy.
* 
* The final composition must not create:
* 
* one conditionalExpression in expressions.g4
* another conditionalExpression in conditionals.g4
* another conditionalExpression in conditional-expressions.g4
* 
* Instead:
* 
* canonical expression composition
*          |
*          +--> conditionalExpression
*                     |
*                     +--> this file
* 
* The exact generated ANTLR composition mechanism must be chosen once and used
* consistently throughout grammar/.
  */

/*

* ============================================================================
* 30. NO DUPLICATE TOKEN AUTHORITY
* ============================================================================
* 
* This file intentionally uses only parser token references.
* 
* It must not define:
* 
* IF
* ELSE
* QUESTION
* COLON
* LBRACE
* RBRACE
* 
* The canonical lexer remains the sole lexical authority.
  */

/*

* ============================================================================
* 31. NO BACKEND COUPLING
* ============================================================================
* 
* This grammar must remain independent of:
* 
* LLVM
* MLIR
* QIR
* OpenQASM implementation details
* CUDA
* ROCm
* vendor FPGA languages
* vendor QPU APIs
* physical device identifiers
* backend instruction sets.
* 
* Interoperability and lowering occur downstream.
  */

/*

* ============================================================================
* 32. NO RESOURCE POLICY IN GRAMMAR
* ============================================================================
* 
* The grammar must not decide whether:
* 
* a branch fits memory;
* a computation fits a device;
* a quantum operation fits a QPU;
* a distributed computation fits available nodes;
* an accelerator is available;
* a schedule is feasible.
* 
* These are semantic/compiler/runtime resource questions.
  */

/*

* ============================================================================
* 33. TEST CONTRACT
* ============================================================================
* 
* The repository must contain tests covering this grammar through the actual
* canonical parser composition.
* 
* POSITIVE SYNTAX TESTS:
* 
* 1. Basic if expression.
* 
*    if condition {
*        value
*    }
* 
* 2. If/else expression.
* 
*    if condition {
*        value_a
*    } else {
*        value_b
*    }
* 
* 3. One else-if.
* 
*    if a {
*        x
*    } else if b {
*        y
*    }
* 
* 4. Multiple else-if branches.
* 
*    if a {
*        x
*    } else if b {
*        y
*    } else if c {
*        z
*    } else {
*        fallback
*    }
* 
* 5. Nested structured conditionals.
* 
* 6. Ternary conditional.
* 
*    condition ? a : b
* 
* 7. Nested ternary conditional.
* 
* 8. Structured conditional containing ternary expression.
* 
* 9. Ternary containing structured conditional where expression grammar
*    permits it.
* 
* 10. Conditional as function argument.
* 
* 11. Conditional as index expression.
* 
* 12. Conditional as initializer.
* 
* 13. Conditional as return expression.
* 
* 14. Conditional inside collection expressions.
* 
* 15. Conditional involving quantum/classical expressions.
* 
* 16. Conditional involving hardware/resource capability expressions where
*    those expressions are valid.
* 
* NEGATIVE SYNTAX TESTS:
* 
* 1. Missing condition.
* 2. Missing branch.
* 3. Missing closing brace.
* 4. Malformed else-if.
* 5. Malformed else branch.
* 6. Missing ternary condition.
* 7. Missing ternary true expression.
* 8. Missing ternary colon.
* 9. Missing ternary false expression.
* 10. Invalid token between condition and branch.
* 
* SEMANTIC TESTS:
* 
* 1. Invalid condition type.
* 2. Incompatible branch types.
* 3. Invalid branch effect.
* 4. Invalid capability requirement.
* 5. Invalid ownership/borrow behavior.
* 6. Unreachable branch.
* 7. Valid optional-else context.
* 
* BOUNDARY TESTS:
* 
* 1. Empty branch.
* 2. Large condition expression.
* 3. Large branch body.
* 4. Long else-if chain.
* 5. Deep nesting.
* 6. Large ternary chain.
* 
* SCALABILITY TESTS:
* 
* Verify absence of language-level finite limits on:
* 
*     branch count
*     nesting
*     expression size
*     program size
* 
* Practical resource limits must be tested separately as implementation
* policies rather than language semantics.
* 
* DETERMINISM TESTS:
* 
* Parse identical source repeatedly and verify identical structural output.
* 
* CROSS-DOMAIN TESTS:
* 
* Classical:
*     if classical_condition { ... } else { ... }
* 
* Quantum/hybrid:
*     if measurement_result { ... } else { ... }
* 
* HDL/hardware:
*     conditional expressions in legal HDL/hardware expression positions.
* 
* Distributed:
*     conditional expressions inside legal distributed computation.
* 
* AI/data:
*     conditional expressions inside legal model/data expressions.
* 
* ============================================================================
  */

/*

* ============================================================================
* 34. HARD-CODING AUDIT
* ============================================================================
* 
* This file passes the following intended audit:
* 
* [x] No fixed qubit count.
* [x] No fixed CPU count.
* [x] No fixed core count.
* [x] No fixed thread count.
* [x] No fixed GPU count.
* [x] No fixed FPGA count.
* [x] No fixed QPU count.
* [x] No fixed node count.
* [x] No fixed memory capacity.
* [x] No fixed tensor dimension.
* [x] No fixed vector width.
* [x] No physical device IDs.
* [x] No topology constants.
* [x] No timing constants.
* [x] No calibration constants.
* [x] No backend-specific instructions.
* [x] No finite branch enumeration.
* [x] No finite nesting enumeration.
* [x] No embedded Rust.
* [x] No unsafe Rust.

*/

/*

* ============================================================================
* 35. COMPLETION CRITERIA
* ============================================================================
* 
* This file is COMPLETE only when all of the following are true:
* 
* [ ] It is the sole modular owner of conditionalExpression.
* 
* [ ] conditional-expressions.g4 no longer defines a competing
*     conditionalExpression hierarchy.
* 
* [ ] statements/conditionals.g4 owns only statement-level conditionals.
* 
* [ ] expressions/ canonical composition wires this rule exactly once.
* 
* [ ] `expression` resolves through the canonical expression hierarchy.
* 
* [ ] `blockExpression` resolves through grammar/core/blocks.g4.
* 
* [ ] IF/ELSE/QUESTION/COLON are supplied by the canonical lexer.
* 
* [ ] No lexer rules exist in this file.
* 
* [ ] No duplicate block grammar exists in this file.
* 
* [ ] AST mapping is documented and implemented downstream.
* 
* [ ] Semantic typing is implemented downstream.
* 
* [ ] Effects/capabilities are implemented downstream.
* 
* [ ] Classical lowering is implemented downstream.
* 
* [ ] Quantum lowering reaches quantum::ir rather than a second quantum IR.
* 
* [ ] HDL/hardware lowering is implemented downstream.
* 
* [ ] Resource/capability analysis is downstream.
* 
* [ ] No hardware limit is encoded.
* 
* [ ] Positive tests pass.
* 
* [ ] Negative tests pass.
* 
* [ ] Boundary tests pass.
* 
* [ ] Scalability tests pass.
* 
* [ ] Determinism tests pass.
* 
* [ ] Cross-domain tests pass.
* 
* [ ] Compatibility tests pass.
* 
* [ ] Rust 1.97/1.97.1 integration remains safe Rust only.
* 
* [ ] No generated parser contains duplicate conditional rule authority.
* 
* ============================================================================
  */