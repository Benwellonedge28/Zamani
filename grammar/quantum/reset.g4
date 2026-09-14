/*
 * ============================================================================
 * Zamani Quantum Reset Grammar
 * File: grammar/quantum/reset.g4
 * ============================================================================
 *
 * PURPOSE
 * -------
 * Defines the source-language syntax for quantum reset operations.
 *
 * ARCHITECTURAL OWNER
 * -------------------
 * This file owns ONLY the syntax of reset operations.
 *
 * It does NOT own:
 *   - Qubit identifiers or physical qubit identifiers
 *   - Quantum state representations
 *   - Quantum IR
 *   - Gate semantics
 *   - Measurement semantics
 *   - QEC algorithms
 *   - Noise/fault models
 *   - Hardware discovery
 *   - Hardware capabilities
 *   - Scheduling
 *   - Routing
 *   - Resource allocation
 *   - Runtime execution
 *
 * The canonical quantum semantic boundary remains quantum::ir.
 *
 *
 * DEPENDENCY MODEL
 * ----------------
 *
 *   source
 *      |
 *      v
 *   lexer
 *      |
 *      v
 *   parser
 *      |
 *      +--> quantumResetOperation
 *              |
 *              v
 *        semantic analysis
 *              |
 *              v
 *        canonical quantum::ir
 *              |
 *              +--> optimization
 *              +--> routing
 *              +--> scheduling
 *              +--> ZQN
 *              +--> hardware HAL
 *              +--> runtime
 *
 *
 * IMPORTANT SCALABILITY RULE
 * --------------------------
 * There is intentionally NO fixed number of reset targets.
 *
 * The grammar does not contain:
 *
 *   MAX_QUBITS
 *   MAX_RESET_TARGETS
 *   q[0]
 *   q[1]
 *   fixed register sizes
 *   fixed machine topology
 *   fixed device identifiers
 *
 * Target cardinality is determined by the program and validated against
 * semantic/resource/capability information outside this grammar.
 *
 *
 * IMPORTANT SEMANTIC RULE
 * -----------------------
 * A syntactically valid reset target is not automatically a semantically
 * valid quantum reset target.
 *
 * Semantic analysis must verify that every target:
 *
 *   1. resolves to a quantum resource,
 *   2. is valid in the current quantum scope,
 *   3. has compatible logical/physical semantics,
 *   4. is not illegally aliased,
 *   5. satisfies the applicable execution/context constraints,
 *   6. can be lowered into the canonical quantum IR.
 *
 *
 * RESET SEMANTICS
 * ---------------
 * The source-level reset operation expresses the semantic intent:
 *
 *     "restore the addressed quantum resource to the language-defined
 *      reset state."
 *
 * The grammar does not decide how this is implemented.
 *
 * A backend may implement reset using:
 *
 *   - native hardware reset,
 *   - measurement + conditional correction,
 *   - active reset,
 *   - dissipative reset,
 *   - pulse-level reset,
 *   - simulator state replacement,
 *   - fault-tolerant logical reset,
 *   - another backend-supported mechanism.
 *
 * Such implementation decisions belong outside this grammar.
 *
 *
 * RUST COMPATIBILITY
 * ------------------
 * This file contains no embedded Rust actions, predicates, or unsafe code.
 *
 * Rust 1.97 / 1.97.1 compatibility is therefore determined by the generated
 * parser/frontend and the repository's Rust implementation, not by this
 * grammar fragment itself.
 *
 *
 * GRAMMAR OWNERSHIP
 * -----------------
 * This file is the sole owner of the parser rule:
 *
 *     quantumResetOperation
 *
 * Do not duplicate that rule in quantum/quantum.g4 or operations.g4.
 *
 * Those files may reference this rule through the repository's grammar
 * aggregation mechanism.
 * ============================================================================
 */


/*
 * ============================================================================
 * Quantum reset operation
 * ============================================================================
 *
 * Canonical source form:
 *
 *     reset <quantum-target>;
 *
 * Multiple targets are supported without imposing a fixed cardinality:
 *
 *     reset q;
 *     reset q[i];
 *     reset q[i], r[j], ancilla;
 *     reset register;
 *     reset dynamically_selected_target;
 *
 * Whether a particular expression denotes a valid quantum target is a
 * semantic question, not a lexical one.
 *
 * The expression-based target model allows the grammar to remain compatible
 * with:
 *
 *   - scalar qubit references,
 *   - register references,
 *   - indexed references,
 *   - slices/ranges,
 *   - dynamically selected logical resources,
 *   - future quantum resource abstractions.
 *
 * No finite target count is encoded here.
 * ============================================================================
 */

quantumResetOperation
    : 'reset' quantumResetTargetList ';'
    ;


/*
 * ============================================================================
 * Reset target list
 * ============================================================================
 *
 * The list is deliberately unbounded by grammar-level constants.
 *
 * The parser accepts any number of syntactically valid target expressions.
 *
 * Resource limits, if any, are enforced later by:
 *
 *   - semantic analysis,
 *   - capability checking,
 *   - resource management,
 *   - compilation,
 *   - scheduling,
 *   - runtime/backend validation.
 *
 * This preserves POCO-REAF.
 * ============================================================================
 */

quantumResetTargetList
    : quantumResetTarget (',' quantumResetTarget)*
    ;


/*
 * ============================================================================
 * Reset target
 * ============================================================================
 *
 * This rule deliberately delegates target syntax to the canonical expression
 * system instead of defining a second quantum-target language here.
 *
 * Examples that may be accepted syntactically, depending on the canonical
 * expression grammar:
 *
 *     q
 *     q[i]
 *     q[start:end]
 *     register
 *     selected[index]
 *     logical_qubit
 *
 * Semantic analysis determines whether the resulting expression is actually
 * a valid quantum reset target.
 *
 * This prevents reset.g4 from becoming coupled to a particular representation
 * of qubits or registers.
 * ============================================================================
 */

quantumResetTarget
    : expression
    ;