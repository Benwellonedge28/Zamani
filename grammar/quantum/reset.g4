/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/quantum/reset.g4
 *
 * GRAMMAR
 * -------
 * QuantumReset
 *
 * STATUS
 * ------
 * NORMATIVE QUANTUM RESET SYNTAX COMPONENT
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE SYNTAX OWNER for source-level quantum reset
 * operations.
 *
 * It defines the structural syntax required to express:
 *
 *     reset q;
 *     reset q0, q1;
 *     reset register;
 *     reset register[i];
 *     reset register[start:end];
 *     reset dynamically_selected_qubit;
 *     reset logical_resource;
 *
 * Target expressions remain open-ended because the language must support
 * scalar qubits, registers, indexed resources, slices, dynamically selected
 * resources, logical resources, and future quantum resource abstractions.
 *
 * This grammar deliberately does NOT decide whether a target is actually
 * quantum. That is a semantic/type/resource-analysis responsibility.
 *
 * ============================================================================
 * ARCHITECTURAL OWNER
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     quantumResetOperation
 *     quantumResetTargetList
 *     quantumResetTarget
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     - lexer rules;
 *     - keyword definitions;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - expression precedence;
 *     - types;
 *     - qubit declarations;
 *     - register declarations;
 *     - logical-qubit declarations;
 *     - physical-qubit declarations;
 *     - operation invocation;
 *     - measurement;
 *     - barriers;
 *     - control flow;
 *     - QEC;
 *     - ZQN;
 *     - noise models;
 *     - resilience;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - resource allocation;
 *     - capability discovery;
 *     - hardware discovery;
 *     - calibration;
 *     - target selection;
 *     - HAL;
 *     - runtime execution;
 *     - canonical quantum::ir.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * The detailed reset syntax is owned here.
 *
 * The statement-layer adapter:
 *
 *     grammar/statements/quantum.g4
 *
 * owns the statement-family name:
 *
 *     quantumResetStatement
 *
 * and delegates to:
 *
 *     quantumResetOperation
 *
 * No other grammar file should redefine quantumResetOperation.
 *
 * Likewise, this file must not redefine quantumResetStatement.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     canonical ZamaniParser
 *          |
 *          v
 *     quantumResetOperation
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type validation
 *          +--> quantum-resource validation
 *          +--> ownership/alias analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> effect analysis
 *          |
 *          v
 *     canonical quantum semantic representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
 *          +--> decomposition
 *          +--> routing
 *          +--> scheduling
 *          +--> QEC
 *          +--> ZQN
 *          +--> resilience
 *          +--> HAL
 *          |
 *          v
 *     target lowering
 *          |
 *          v
 *     runtime
 *
 * This grammar has no direct dependency on any downstream implementation.
 *
 * ============================================================================
 * RESET SEMANTICS
 * ============================================================================
 *
 * Source-level reset expresses the semantic intent:
 *
 *     restore each addressed quantum resource to the language-defined
 *     reset state.
 *
 * The grammar does NOT specify how that intent is realized.
 *
 * A backend may implement reset using, for example:
 *
 *     - native hardware reset;
 *     - active reset;
 *     - measurement followed by conditional correction;
 *     - dissipative reset;
 *     - pulse-level reset;
 *     - simulator state replacement;
 *     - logical/fault-tolerant reset;
 *     - another target-supported implementation.
 *
 * Such choices belong downstream.
 *
 * ============================================================================
 * IMPORTANT SEMANTIC DISTINCTION
 * ============================================================================
 *
 * Syntactic validity:
 *
 *     reset q;
 *
 * means the source has valid reset syntax.
 *
 * It does NOT prove:
 *
 *     q exists;
 *     q is a qubit;
 *     q is writable;
 *     q is currently allocated;
 *     q is in the correct scope;
 *     q is not aliased illegally;
 *     q is supported by the selected target;
 *     the target has a native reset operation;
 *     a reset implementation is available.
 *
 * Those are semantic/resource/capability/backend questions.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Reset syntax is target-independent.
 *
 * This grammar contains NO universal limits for:
 *
 *     qubits
 *     registers
 *     reset targets
 *     devices
 *     nodes
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     memory
 *     threads
 *     controls
 *     circuit depth
 *     operation count
 *
 * It contains NO:
 *
 *     MAX_QUBITS
 *     MAX_RESET_TARGETS
 *     MAX_REGISTERS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * or equivalent language-level hardware ceilings.
 *
 * The repetition:
 *
 *     (COMMA quantumResetTarget)*
 *
 * allows arbitrary source-level target cardinality subject only to the
 * practical resources required to parse and compile the source.
 *
 * Physical execution limits remain outside the grammar.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * This grammar does NOT contain:
 *
 *     requires qubits >= ...
 *     requires capability(...)
 *     target(...)
 *     physical_qubit(...)
 *     device(...)
 *
 * if such constructs are part of Zamani, they belong to the resource,
 * hardware, capability, or target-intent grammar.
 *
 * A reset operation says WHAT should happen.
 *
 * Resource/capability analysis determines WHETHER and WHERE it can happen.
 *
 * ============================================================================
 * TARGET ABSTRACTION
 * ============================================================================
 *
 * The target is represented by:
 *
 *     expression
 *
 * rather than a second quantum-specific target-reference language.
 *
 * This intentionally permits future semantic resource models without changing
 * reset syntax.
 *
 * Examples:
 *
 *     q
 *     q[i]
 *     q[start:end]
 *     register
 *     logical_qubit
 *     selected[index]
 *
 * Whether any particular expression is a legal reset operand is determined
 * downstream.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical keyword is:
 *
 *     RESET
 *
 * with source spelling:
 *
 *     reset
 *
 * It is owned by:
 *
 *     grammar/lexer/keywords.g4
 *
 * This file MUST consume RESET through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It MUST NOT introduce:
 *
 *     K_RESET
 *
 * or any second reset token.
 *
 * Likewise, punctuation is consumed from the canonical Zamani lexer.
 *
 * ============================================================================
 * NO LEXER DUPLICATION
 * ============================================================================
 *
 * Do NOT add lexer rules here such as:
 *
 *     RESET : 'reset' ;
 *
 * Do NOT add:
 *
 *     ',' ;
 *     ';' ;
 *
 * as lexer rules here.
 *
 * All lexical ownership remains in grammar/lexer/.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Reset targets reuse the repository's canonical:
 *
 *     expression
 *
 * production.
 *
 * This prevents reset.g4 from creating a parallel quantum expression
 * language.
 *
 * The imported expression grammar remains responsible for:
 *
 *     identifiers;
 *     indexing;
 *     ranges;
 *     member access;
 *     calls;
 *     literals;
 *     expressions;
 *     generic constructs;
 *     other language-wide expression forms.
 *
 * ============================================================================
 * TYPE / SEMANTIC INTEGRATION
 * ============================================================================
 *
 * This grammar does not define a special:
 *
 *     ResetTargetType
 *
 * or:
 *
 *     QuantumResetType
 *
 * because that would duplicate the language type system.
 *
 * Semantic analysis should establish that a reset target denotes a resettable
 * quantum resource.
 *
 * A representative semantic requirement is:
 *
 *     target : QuantumResettableResource
 *
 * but the exact Rust semantic type belongs to the repository's semantic/AST
 * implementation.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve:
 *
 *     - reset operation source span;
 *     - target-list source span;
 *     - target ordering;
 *     - each target expression;
 *     - source positions;
 *     - delimiters where required for diagnostics/formatting.
 *
 * A representative domain-neutral AST shape is:
 *
 *     ResetOperation {
 *         targets: Vec<Expression>,
 *         source_span: Span,
 *     }
 *
 * This is a CONTRACT SHAPE, not a new Rust type defined by this grammar.
 *
 * The authoritative AST remains:
 *
 *     src/frontend/ast/
 *
 * or the repository's established frontend AST boundary.
 *
 * This grammar must not introduce a competing quantum AST.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis of a reset operation should validate, as applicable:
 *
 *     1. Every target expression resolves successfully.
 *
 *     2. Every target denotes a quantum resource or quantum resource view.
 *
 *     3. Every target is resettable.
 *
 *     4. Target ownership/borrowing/aliasing rules are respected.
 *
 *     5. Target lifetimes are valid.
 *
 *     6. Target dimensions/ranges are valid.
 *
 *     7. Duplicate or overlapping targets are handled according to the
 *        language's quantum aliasing/operand rules.
 *
 *     8. Dynamic target expressions satisfy the applicable runtime rules.
 *
 *     9. Required capabilities can be satisfied.
 *
 *    10. Required resources can be satisfied.
 *
 *    11. The operation can be represented by the canonical quantum IR.
 *
 * A semantic failure MUST NOT be reported as a syntax failure.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * A valid reset operation lowers through the existing canonical quantum
 * semantic boundary.
 *
 * Conceptually:
 *
 *     ResetOperation
 *          |
 *          v
 *     semantic quantum reset
 *          |
 *          v
 *     quantum::ir
 *
 * The grammar does NOT define:
 *
 *     ResetIR
 *     QuantumResetIR
 *     PhysicalResetIR
 *     VendorResetIR
 *
 * as a second intermediate representation.
 *
 * Existing quantum IR ownership remains unchanged.
 *
 * ============================================================================
 * IMPLEMENTATION CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no Rust actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no target selection;
 *     - no randomness;
 *     - no runtime execution.
 *
 * Therefore the grammar itself requires no unsafe Rust.
 *
 * Rust 1.97 / Rust 1.97.1 compatibility is established by the generated
 * parser/frontend implementation and the repository's Rust toolchain.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Structurally invalid examples should be rejected by parsing:
 *
 *     reset;
 *     reset ;
 *     reset (, q);
 *     reset q,;
 *     reset q,,r;
 *     reset (;
 *     reset q);
 *
 * Semantic analysis, rather than this grammar, should reject examples such as:
 *
 *     reset classical_integer;
 *
 * when the target does not denote a resettable quantum resource.
 *
 * Likewise:
 *
 *     reset unavailable_q;
 *
 * must not become a syntax error merely because the target resource is
 * unavailable.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source text;
 *     - canonical ZamaniLexer tokenization;
 *     - imported grammar definitions.
 *
 * It does not depend on:
 *
 *     - machine type;
 *     - CPU count;
 *     - GPU availability;
 *     - QPU availability;
 *     - network state;
 *     - calibration;
 *     - runtime state;
 *     - deployment topology;
 *     - random numbers;
 *     - wall-clock time.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The canonical source form remains:
 *
 *     reset <target-list> ;
 *
 * Existing source:
 *
 *     reset q;
 *
 * remains valid.
 *
 * This file does not introduce a new spelling or replacement keyword.
 *
 * The existing canonical RESET token is preserved.
 *
 * Any future change to reset syntax must follow:
 *
 *     specification
 *         ->
 *     AST contract
 *         ->
 *     grammar
 *         ->
 *     semantic contract
 *         ->
 *     IR contract
 *         ->
 *     conformance tests
 *         ->
 *     compatibility policy
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive cases:
 *
 *     reset q;
 *     reset q0, q1;
 *     reset register;
 *     reset register[i];
 *     reset register[start:end];
 *     reset selected[index];
 *     reset logical_qubit;
 *
 * Required structural negative cases:
 *
 *     reset;
 *     reset ;
 *     reset , q;
 *     reset q,;
 *     reset q,,r;
 *     reset (;
 *     reset );
 *
 * Required semantic-negative cases:
 *
 *     reset classical_value;
 *     reset immutable_non_quantum_resource;
 *     reset unknown_name;
 *     reset invalid_index;
 *
 * Required scalability cases:
 *
 *     reset q;
 *
 *     reset q0, q1, q2, ...;
 *
 * where the test generator determines cardinality from available test
 * resources rather than a grammar-level maximum.
 *
 * No test may establish an artificial language maximum such as:
 *
 *     "reset supports at most N targets."
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Reset may participate in:
 *
 *     classical/quantum hybrid programs;
 *     dynamic circuits;
 *     error-correction workflows;
 *     resource-aware compilation;
 *     resilience;
 *     simulation;
 *     hardware/software co-design.
 *
 * This file remains responsible only for the reset syntax.
 *
 * Dynamic conditions belong to the dynamic-circuit/control grammar.
 *
 * Error-correction semantics belong to QEC.
 *
 * Noise/fault semantics belong to ZQN.
 *
 * Recovery policies belong to resilience.
 *
 * Hardware realization belongs to HAL/backend infrastructure.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file intentionally contains no:
 *
 *     MAX_QUBITS
 *     MAX_RESET_TARGETS
 *     MAX_REGISTERS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     QUBIT_0
 *     QUBIT_1
 *     physical_qubit(0)
 *     physical_qubit(1)
 *
 * Numbers appearing in a reset target expression remain program data and are
 * not language-level hardware limits.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing reset syntax must not:
 *
 *     - execute reset;
 *     - access hardware;
 *     - access QPU state;
 *     - access files;
 *     - access network resources;
 *     - access credentials;
 *     - bypass capability checks.
 *
 * Reset authority is established downstream by semantic/resource/security
 * infrastructure.
 *
 * ============================================================================
 * GENERATED-ARTIFACT CONTRACT
 * ============================================================================
 *
 * This file is authoritative grammar source.
 *
 * Generated ANTLR artifacts are derived outputs and must not become a second
 * source of truth.
 *
 * Regeneration must be deterministic using the repository's declared ANTLR
 * toolchain.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE as an independent grammar component when:
 *
 * [x] It is a valid ANTLR parser grammar.
 *
 * [x] Its grammar name is QuantumReset.
 *
 * [x] It consumes the canonical ZamaniLexer vocabulary.
 *
 * [x] It uses the canonical RESET token.
 *
 * [x] It does not define lexer tokens.
 *
 * [x] It owns quantumResetOperation.
 *
 * [x] It owns quantumResetTargetList.
 *
 * [x] It owns quantumResetTarget.
 *
 * [x] It does not redefine quantumResetStatement.
 *
 * [x] It reuses the canonical expression grammar.
 *
 * [x] It supports one or arbitrarily many reset targets.
 *
 * [x] It does not encode hardware capacities.
 *
 * [x] It does not encode physical qubit identifiers.
 *
 * [x] It does not enumerate hardware reset mechanisms.
 *
 * [x] It does not create a second quantum IR.
 *
 * [x] It defines AST integration in advance.
 *
 * [x] It defines semantic integration in advance.
 *
 * [x] It defines quantum::ir integration in advance.
 *
 * [x] It defines compiler/runtime boundaries in advance.
 *
 * [x] It defines positive and negative test requirements.
 *
 * [x] It defines scalability requirements.
 *
 * [x] It defines compatibility requirements.
 *
 * [x] It contains no Rust actions.
 *
 * [x] It requires no unsafe Rust.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This file answers one question:
 *
 *     "How is a portable quantum reset operation represented syntactically?"
 *
 * It does NOT answer:
 *
 *     "Which physical qubit performs the reset?"
 *
 *     "Which QPU performs the reset?"
 *
 *     "How is reset implemented?"
 *
 *     "How many qubits can the machine reset?"
 *
 *     "Which pulse sequence is used?"
 *
 *     "Which QEC code is used?"
 *
 *     "Which scheduler is used?"
 *
 *     "Which hardware topology is required?"
 *
 * Those questions belong downstream.
 *
 * The governing architecture remains:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * subject to the program's actual semantics and the resources/capabilities
 * available to the selected execution environment.
 *
 * ============================================================================
 */

parser grammar QuantumReset;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions,
    Types
;


/*
 * ============================================================================
 * CANONICAL RESET OPERATION
 * ============================================================================
 *
 * Canonical source form:
 *
 *     reset <quantum-target-list>;
 *
 * Examples:
 *
 *     reset q;
 *     reset q0, q1;
 *     reset register;
 *     reset register[i];
 *     reset register[start:end];
 *
 * The semicolon is owned here because this rule represents the complete
 * reset statement-level operation consumed by:
 *
 *     grammar/statements/quantum.g4
 *
 * through its:
 *
 *     quantumResetStatement
 *
 * adapter.
 *
 * ============================================================================
 */

quantumResetOperation
    : RESET quantumResetTargetList SEMICOLON
    ;


/*
 * ============================================================================
 * RESET TARGET LIST
 * ============================================================================
 *
 * The list has no grammar-level cardinality limit.
 *
 * Cardinality is therefore determined by the source program and practical
 * parser/compiler resources, not by a machine-specific constant.
 *
 * Examples:
 *
 *     reset q0;
 *
 *     reset q0, q1;
 *
 *     reset q0, q1, q2, q3;
 *
 *     reset register[0], register[1], register[2];
 *
 * ============================================================================
 */

quantumResetTargetList
    : quantumResetTarget
      (
          COMMA
          quantumResetTarget
      )*
    ;


/*
 * ============================================================================
 * RESET TARGET
 * ============================================================================
 *
 * A reset target is represented syntactically as a general Zamani expression.
 *
 * This deliberately avoids introducing a second quantum-target grammar.
 *
 * Semantic analysis determines whether the expression denotes:
 *
 *     - a qubit;
 *     - a qubit register;
 *     - a register element;
 *     - a quantum slice/view;
 *     - a logical quantum resource;
 *     - a dynamically selected quantum resource;
 *     - another resettable quantum resource.
 *
 * Examples:
 *
 *     q
 *     q[i]
 *     register
 *     register[i]
 *     register[start:end]
 *     selected[index]
 *     logical_qubit
 *
 * Examples that may parse but must be rejected semantically when their type
 * is not resettable:
 *
 *     classical_value
 *     integer_value
 *     string_value
 *
 * ============================================================================
 */

quantumResetTarget
    : expression
    ;