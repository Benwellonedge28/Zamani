/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/contracts.g4
 *
 * Grammar:
 *     FunctionContracts
 *
 * Role:
 *     Canonical parser delegate for source-level FUNCTION CONTRACTS.
 *
 * Status:
 *     Production parser contract
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This file contains ANTLR4 parser grammar only.
 *
 *     It contains:
 *       - no embedded Rust;
 *       - no semantic predicates;
 *       - no unsafe code;
 *       - no filesystem access;
 *       - no network access;
 *       - no runtime execution;
 *       - no hardware discovery;
 *       - no resource allocation;
 *       - no target selection.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE OWNER of source-level contracts attached to
 * callable/function declarations.
 *
 * A contract expresses a logical condition associated with a function.
 *
 * Examples:
 *
 *     fn sqrt(x: Float) -> Float
 *     contract {
 *         requires(x >= 0);
 *         ensures(result >= 0);
 *     }
 *     {
 *         ...
 *     }
 *
 *     fn measure(q: Qubit) -> Measurement
 *     contract {
 *         requires(capability("quantum.measurement"));
 *         ensures(result.is_valid());
 *     }
 *     {
 *         ...
 *     }
 *
 *     fn update(state: State)
 *     contract {
 *         invariant(state.is_valid());
 *     }
 *     {
 *         ...
 *     }
 *
 * The expressions inside contracts are ordinary Zamani expressions.
 *
 * Contract interpretation is performed downstream.
 *
 * ============================================================================
 * CORE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - functionContractClause;
 *     - functionContractItem;
 *     - functionRequiresContract;
 *     - functionEnsuresContract;
 *     - functionInvariantContract;
 *     - the syntactic association of a contract predicate with its expression;
 *     - ordering of contract predicates inside a contract block;
 *     - optional source-level contract terminators where permitted.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - keywords;
 *     - identifiers;
 *     - expressions;
 *     - operators;
 *     - types;
 *     - function declarations;
 *     - function names;
 *     - parameters;
 *     - generic parameters;
 *     - return types;
 *     - effects;
 *     - where/generic constraints;
 *     - function bodies;
 *     - statements;
 *     - modules;
 *     - capabilities;
 *     - resources;
 *     - hardware;
 *     - quantum semantics;
 *     - HDL semantics;
 *     - AI semantics;
 *     - distributed semantics;
 *     - proof execution;
 *     - theorem proving;
 *     - runtime verification;
 *     - resource allocation;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL IR.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The following parser rules MUST have exactly one authoritative owner:
 *
 *     functionContractClause
 *     functionContractItem
 *     functionRequiresContract
 *     functionEnsuresContract
 *     functionInvariantContract
 *
 * Their authoritative owner is this file.
 *
 * `grammar/functions/functions.g4` MUST consume these rules and MUST NOT
 * redefine them.
 *
 * `grammar/core/constraints.g4` MUST NOT redefine these function-contract
 * rules.
 *
 * `grammar/functions/constraints.g4` MUST NOT redefine these function-contract
 * rules.
 *
 * Generic constraints and contracts are distinct concepts:
 *
 *     generic constraint
 *         -> constrains types/generic parameters
 *
 *     function contract
 *         -> states logical conditions over a function invocation,
 *            its inputs, outputs, state, or permitted semantic context.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     parser
 *       |
 *       +-------------------------------+
 *       |                               |
 *       v                               v
 * function declaration             expression grammar
 *       |                               |
 *       +---------------+---------------+
 *                       |
 *                       v
 *                function contracts
 *                       |
 *                       v
 *                 frontend AST
 *                       |
 *                       v
 *              structural validation
 *                       |
 *                       v
 *                semantic analysis
 *                       |
 *             +---------+---------+
 *             |         |         |
 *             v         v         v
 *          types     effects   capabilities
 *             |         |         |
 *             +---------+---------+
 *                       |
 *                       v
 *                canonical semantics
 *                       |
 *          +------------+-------------+
 *          |            |             |
 *          v            v             v
 *    classical IR   quantum::ir   HDL/hardware IR
 *
 * This grammar never lowers directly to a backend.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This parser grammar consumes the canonical lexer vocabulary.
 *
 *     tokenVocab = ZamaniLexer
 *
 * The lexer owns the actual spelling of:
 *
 *     CONTRACT
 *     REQUIRES
 *     ENSURES
 *     INVARIANT
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     SEMICOLON
 *
 * This file MUST NOT define lexer rules.
 *
 * It MUST NOT introduce alternative spellings for these keywords.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Contract conditions are ordinary Zamani expressions.
 *
 * This means the contract grammar deliberately does not define a second
 * expression language.
 *
 * Examples of syntactically valid contract expressions include:
 *
 *     x >= 0
 *     result != null
 *     result.is_valid()
 *     input.shape == expected_shape
 *     capability("quantum.measurement")
 *     resources.available("memory")
 *     state.is_consistent()
 *
 * Whether any of those expressions are semantically legal in a particular
 * contract position is determined by semantic analysis.
 *
 * This grammar therefore delegates:
 *
 *     expression
 *
 * to the canonical expression grammar.
 *
 * ============================================================================
 * CONTRACT BLOCK
 * ============================================================================
 *
 * A function contract is a block containing zero or more contract items.
 *
 * The block syntax is:
 *
 *     contract {
 *         requires(...);
 *         ensures(...);
 *         invariant(...);
 *     }
 *
 * Empty contract blocks remain syntactically representable:
 *
 *     contract {}
 *
 * Whether an empty contract is meaningful is a semantic/documentation
 * decision, not a parsing decision.
 *
 * ============================================================================
 */

parser grammar FunctionContracts;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/* ============================================================================
 * 1. FUNCTION CONTRACT CLAUSE
 * ============================================================================
 *
 * Canonical source-level contract attachment.
 *
 * A function declaration may consume:
 *
 *     functionContractClause*
 *
 * The surrounding function grammar owns placement and cardinality.
 *
 * This rule owns only the contract construct itself.
 */
functionContractClause
    : CONTRACT
      LBRACE
      functionContractItem*
      RBRACE
    ;


/* ============================================================================
 * 2. CONTRACT ITEM
 * ============================================================================
 *
 * A contract block consists of zero or more ordered predicates.
 *
 * Source order is preserved by the parser/AST layer.
 *
 * The semantic layer may subsequently normalize or classify these predicates,
 * but grammar must preserve their original ordering and source locations.
 */
functionContractItem
    : functionRequiresContract
    | functionEnsuresContract
    | functionInvariantContract
    ;


/* ============================================================================
 * 3. PRECONDITION / REQUIRES
 * ============================================================================
 *
 * `requires` expresses a condition that must hold at the contract boundary
 * defined by the semantic contract model.
 *
 * Grammar does not decide whether this means:
 *
 *     - caller precondition;
 *     - capability requirement;
 *     - type-state requirement;
 *     - resource requirement;
 *     - logical predicate;
 *     - verification obligation.
 *
 * Those meanings belong downstream.
 */
functionRequiresContract
    : REQUIRES
      LPAREN
      expression
      RPAREN
      functionContractTerminator
    ;


/* ============================================================================
 * 4. POSTCONDITION / ENSURES
 * ============================================================================
 *
 * `ensures` expresses a condition associated with successful completion of the
 * callable.
 *
 * References such as `result` are ordinary language expressions if the
 * language/type/semantic system defines them.
 *
 * This grammar MUST NOT introduce a special result expression grammar.
 */
functionEnsuresContract
    : ENSURES
      LPAREN
      expression
      RPAREN
      functionContractTerminator
    ;


/* ============================================================================
 * 5. INVARIANT
 * ============================================================================
 *
 * `invariant` expresses a condition that must remain valid according to the
 * semantic contract model.
 *
 * The grammar does not decide:
 *
 *     - when an invariant is checked;
 *     - how often it is checked;
 *     - whether checking is compile-time or runtime;
 *     - whether checking is exhaustive;
 *     - whether formal verification is available.
 */
functionInvariantContract
    : INVARIANT
      LPAREN
      expression
      RPAREN
      functionContractTerminator
    ;


/* ============================================================================
 * 6. CONTRACT TERMINATOR
 * ============================================================================
 *
 * Existing Zamani function grammar permits contract predicates to terminate
 * with an optional semicolon.
 *
 * Preserve that source compatibility here.
 *
 * The contract block itself is terminated by RBRACE.
 *
 * This rule MUST NOT consume statement terminators belonging to the function
 * body.
 */
functionContractTerminator
    : SEMICOLON?
    ;


/* ============================================================================
 * 7. REUSABLE CONTRACT PREDICATE
 * ============================================================================
 *
 * This rule provides a stable semantic/parser integration point for tooling
 * that needs to identify a contract assertion without duplicating the three
 * concrete predicate rules.
 *
 * It is intentionally an alias over canonical contract items.
 */
functionContractPredicate
    : functionRequiresContract
    | functionEnsuresContract
    | functionInvariantContract
    ;


/* ============================================================================
 * 8. CONTRACT EXPRESSION BOUNDARY
 * ============================================================================
 *
 * This named boundary makes the AST/semantic integration explicit without
 * introducing a second expression grammar.
 *
 * It exists as a parser-level contract boundary only.
 */
functionContractExpression
    : expression
    ;


/* ============================================================================
 * 9. CONTRACT LIST
 * ============================================================================
 *
 * A reusable contract-item sequence.
 *
 * This is intentionally unbounded at the grammar level.
 *
 * There is no:
 *
 *     MAX_CONTRACTS
 *     MAX_PRECONDITIONS
 *     MAX_POSTCONDITIONS
 *     MAX_INVARIANTS
 *
 * Semantic/compiler resource limits, if any, are implementation concerns and
 * MUST NOT become language-level grammar limits.
 */
functionContractList
    : functionContractItem*
    ;


/* ============================================================================
 * 10. CONTRACT REQUIREMENT CATEGORY
 * ============================================================================
 *
 * This grammar does not introduce specialized syntax for:
 *
 *     resource requirements
 *     capability requirements
 *     security requirements
 *     quantum requirements
 *     hardware requirements
 *     distributed requirements
 *     timing requirements
 *     reliability requirements
 *
 * Those are represented by ordinary expressions and/or the repository's
 * resource/capability/effect systems.
 *
 * Example:
 *
 *     requires(capability("quantum.measurement"));
 *
 *     requires(resource.available("memory"));
 *
 *     requires(topology.supports("required_connectivity"));
 *
 * The expression grammar remains the sole expression authority.
 */


/* ============================================================================
 * 11. DOMAIN-NEUTRAL CONTRACT MODEL
 * ============================================================================
 *
 * A contract can be attached to a function operating in any supported domain.
 *
 * Examples include:
 *
 *     classical:
 *         requires(x >= 0);
 *
 *     quantum:
 *         requires(capability("quantum.measurement"));
 *
 *     HDL:
 *         requires(signal.is_defined(clk));
 *
 *     hybrid:
 *         requires(classical_condition);
 *
 *     distributed:
 *         requires(capability("distributed.communication"));
 *
 *     AI/data:
 *         requires(input.schema == schema);
 *
 * The function-contract grammar does not create domain-specific contract
 * syntaxes.
 *
 * This is required for extensibility.
 */


/* ============================================================================
 * 12. QUANTUM BOUNDARY
 * ============================================================================
 *
 * Contracts may constrain quantum computations through ordinary expressions.
 *
 * Example:
 *
 *     fn execute(q: Qubit) -> Measurement
 *     contract {
 *         requires(capability("quantum.measurement"));
 *         ensures(result.is_valid());
 *     }
 *     {
 *         ...
 *     }
 *
 * This grammar does NOT define:
 *
 *     - QEC;
 *     - noise models;
 *     - fault models;
 *     - physical qubit IDs;
 *     - routing;
 *     - pulse schedules;
 *     - calibration;
 *     - device topology;
 *     - quantum::ir.
 *
 * The downstream architecture remains:
 *
 *     source
 *       ->
 *     AST
 *       ->
 *     semantic analysis
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 * Contracts can contribute semantic obligations to this pipeline but never
 * replace those subsystems.
 */


/* ============================================================================
 * 13. CLASSICAL / HDL / HYBRID BOUNDARY
 * ============================================================================
 *
 * Function contracts must remain independent of the computational domain.
 *
 * A future domain must therefore NOT require a new contract grammar such as:
 *
 *     quantumContract
 *     gpuContract
 *     fpgaContract
 *     distributedContract
 *     aiContract
 *
 * unless that domain genuinely introduces syntax that cannot be represented
 * through the canonical expression/semantic contract model.
 *
 * Ordinary domain capabilities and requirements should be represented by:
 *
 *     types
 *     expressions
 *     effects
 *     capabilities
 *     resources
 *     attributes
 *
 * owned by their respective grammar modules.
 */


/* ============================================================================
 * 14. RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Contracts may express semantic requirements.
 *
 * They do NOT allocate resources.
 *
 * For example:
 *
 *     requires(qubits >= n);
 *
 * can describe a requirement if the surrounding semantic language defines
 * `qubits` and its meaning.
 *
 * It does NOT mean:
 *
 *     allocate n physical qubits now
 *
 * and it does not identify:
 *
 *     physical qubit 0
 *     physical qubit 1
 *     QPU 0
 *     GPU 0
 *     CPU 0
 *
 * Resource realization remains downstream.
 *
 * This distinction is essential to POCO-REAF.
 */


/* ============================================================================
 * 15. NO HARD-CODED RESOURCE LIMITS
 * ============================================================================
 *
 * This grammar imposes no fixed maximum on:
 *
 *     - contract blocks;
 *     - contract predicates;
 *     - expression size;
 *     - functions;
 *     - parameters;
 *     - generic parameters;
 *     - quantum values;
 *     - classical values;
 *     - HDL objects;
 *     - distributed resources;
 *     - hardware capabilities;
 *     - devices;
 *     - nodes;
 *     - threads;
 *     - cores;
 *     - GPUs;
 *     - FPGAs;
 *     - QPUs;
 *     - memory;
 *     - storage;
 *     - tensor dimensions.
 *
 * Repetition is represented by ANTLR repetition operators.
 *
 * Actual parser/compiler/runtime limits, where unavoidable, are implementation
 * resource policies and MUST NOT be encoded as language semantics.
 */


/* ============================================================================
 * 16. NO TARGET-SPECIFIC CONTRACT GRAMMAR
 * ============================================================================
 *
 * Do NOT add constructs such as:
 *
 *     requires_8_cores
 *     requires_32_qubits
 *     requires_gpu_0
 *     requires_qpu_1
 *     requires_fpga_2
 *     requires_64gb
 *
 * as core grammar constructs.
 *
 * A portable contract expresses semantic requirements.
 *
 * Target realization is determined later by:
 *
 *     resource analysis
 *     capability discovery
 *     compilation
 *     routing
 *     scheduling
 *     deployment
 *     runtime.
 */


/* ============================================================================
 * 17. CONTRACT SEMANTICS
 * ============================================================================
 *
 * Parsing establishes only:
 *
 *     contract predicate -> expression
 *
 * Semantic analysis determines:
 *
 *     - scope;
 *     - name resolution;
 *     - type correctness;
 *     - purity requirements;
 *     - effect requirements;
 *     - capability requirements;
 *     - resource requirements;
 *     - temporal meaning;
 *     - state meaning;
 *     - invocation boundary;
 *     - exceptional behavior;
 *     - verification mode;
 *     - runtime-checkability;
 *     - compile-time-checkability;
 *     - satisfiability;
 *     - consistency;
 *     - redundancy;
 *     - implication relationships.
 *
 * None of these decisions belong in this grammar.
 */


/* ============================================================================
 * 18. PRECONDITION SEMANTICS
 * ============================================================================
 *
 * A `requires` expression is syntactically just an expression.
 *
 * Semantic analysis determines whether it can refer to:
 *
 *     - parameters;
 *     - generic parameters;
 *     - receiver/self;
 *     - capabilities;
 *     - resources;
 *     - module state;
 *     - compile-time information;
 *     - permitted environment state.
 *
 * The grammar deliberately does not hard-code a list of permitted names.
 */


/* ============================================================================
 * 19. POSTCONDITION SEMANTICS
 * ============================================================================
 *
 * An `ensures` expression is syntactically just an expression.
 *
 * The semantic layer determines whether special semantic bindings exist for:
 *
 *     result
 *     old(...)
 *     previous state
 *     receiver/self
 *     modified resources
 *
 * This grammar MUST NOT introduce special lexer tokens for those concepts
 * unless the language specification establishes them independently.
 *
 * If `result` is a normal identifier in the canonical language, it remains
 * governed by the ordinary identifier/expression grammar.
 */


/* ============================================================================
 * 20. INVARIANT SEMANTICS
 * ============================================================================
 *
 * An invariant is a logical condition.
 *
 * Its verification model is outside grammar.
 *
 * Possible downstream interpretations include:
 *
 *     compile-time proof obligation
 *     static verification
 *     runtime assertion
 *     symbolic verification
 *     model checking
 *     domain-specific verification
 *
 * The grammar does not choose among them.
 */


/* ============================================================================
 * 21. CONTRACT COMPOSITION
 * ============================================================================
 *
 * Multiple contract blocks are syntactically supported by the surrounding
 * function grammar.
 *
 * For example:
 *
 *     fn f(x: Int)
 *     contract {
 *         requires(x >= 0);
 *     }
 *     contract {
 *         ensures(result >= x);
 *     }
 *     {
 *         ...
 *     }
 *
 * Whether multiple blocks are semantically merged, preserved separately,
 * rejected, or normalized is a semantic/specification decision.
 *
 * Source order must be preserved in the AST.
 */


/* ============================================================================
 * 22. CONTRACT ORDER
 * ============================================================================
 *
 * The grammar permits any ordering of:
 *
 *     requires
 *     ensures
 *     invariant
 *
 * For example:
 *
 *     contract {
 *         invariant(...);
 *         requires(...);
 *         ensures(...);
 *     }
 *
 * If the language specification later requires a canonical semantic ordering,
 * that normalization belongs in semantic analysis rather than the parser.
 *
 * This avoids making source ordering an accidental semantic restriction.
 */


/* ============================================================================
 * 23. DUPLICATE CONTRACTS
 * ============================================================================
 *
 * The grammar intentionally permits repeated predicates:
 *
 *     contract {
 *         requires(a);
 *         requires(b);
 *         requires(c);
 *     }
 *
 * and:
 *
 *     contract {
 *         ensures(a);
 *         ensures(a);
 *     }
 *
 * Duplicate detection, simplification, implication, contradiction detection,
 * and normalization are semantic concerns.
 *
 * This avoids arbitrary grammar-level cardinality limits.
 */


/* ============================================================================
 * 24. EXPRESSION SIDE EFFECTS
 * ============================================================================
 *
 * The grammar does not determine whether a contract expression is pure.
 *
 * The semantic/effect system determines whether a contract expression may:
 *
 *     - mutate state;
 *     - perform I/O;
 *     - access hardware;
 *     - access secrets;
 *     - communicate over a network;
 *     - invoke quantum operations;
 *     - allocate resources;
 *     - access distributed state.
 *
 * If contract expressions must be pure, that is a semantic rule enforced after
 * parsing.
 */


/* ============================================================================
 * 25. SECURITY BOUNDARY
 * ============================================================================
 *
 * Contracts do not implicitly grant privileged access.
 *
 * Writing:
 *
 *     requires(secret_available);
 *
 * does not grant access to the secret.
 *
 * Writing:
 *
 *     ensures(device_state.valid());
 *
 * does not grant access to a device.
 *
 * Capability and security analysis remains responsible for authorization.
 *
 * The grammar must never turn contract syntax into an implicit privilege
 * escalation mechanism.
 */


/* ============================================================================
 * 26. COMPILE-TIME CONTRACTS
 * ============================================================================
 *
 * A contract may participate in compile-time verification where supported.
 *
 * This grammar does not distinguish:
 *
 *     compile-time requires
 *     runtime requires
 *     proof-time requires
 *
 * through additional syntax.
 *
 * Such execution/verification modes belong to semantic attributes,
 * verification configuration, effects, or dedicated verification grammar
 * where appropriate.
 *
 * This prevents contract syntax from being coupled to one compiler strategy.
 */


/* ============================================================================
 * 27. RUNTIME CONTRACTS
 * ============================================================================
 *
 * Runtime checking is a semantic/runtime concern.
 *
 * The grammar does not generate runtime assertions.
 *
 * It merely preserves the contract in the source representation so downstream
 * tooling can decide whether and how it is verified.
 */


/* ============================================================================
 * 28. FORMAL VERIFICATION BOUNDARY
 * ============================================================================
 *
 * This file is compatible with future formal verification systems.
 *
 * A contract may become:
 *
 *     proof obligation
 *     theorem hypothesis
 *     model-checking property
 *     symbolic constraint
 *     runtime assertion
 *     static analysis condition
 *
 * without changing the contract grammar.
 *
 * The proof system must consume the semantic representation rather than parse
 * this file independently.
 */


/* ============================================================================
 * 29. SANKOFA / TEMPORAL / HISTORY INTEGRATION
 * ============================================================================
 *
 * Historical or temporal properties must not be added here as a second
 * contract language.
 *
 * If the language supports expressions such as:
 *
 *     history(...)
 *     previous(...)
 *     before(...)
 *     after(...)
 *     temporal(...)
 *
 * they remain ordinary expressions provided by the appropriate grammar and
 * semantic subsystems.
 *
 * A future temporal contract can therefore be represented as:
 *
 *     invariant(history_is_consistent(...));
 *
 * without requiring this file to know anything about Sankofa runtime state.
 */


/* ============================================================================
 * 30. DISTRIBUTED COMPUTING INTEGRATION
 * ============================================================================
 *
 * Contracts may express distributed properties through ordinary expressions
 * and capabilities.
 *
 * Examples:
 *
 *     requires(capability("distributed.communication"));
 *
 *     requires(consistency.is_satisfied());
 *
 *     ensures(result.is_replicated());
 *
 * This grammar does not encode:
 *
 *     node counts;
 *     cluster sizes;
 *     network topology;
 *     replica counts;
 *     placement;
 *     scheduling;
 *     transport protocol.
 *
 * Those belong to distributed/resource/networking subsystems.
 */


/* ============================================================================
 * 31. HDL INTEGRATION
 * ============================================================================
 *
 * HDL-related functions may carry contracts.
 *
 * Example:
 *
 *     contract {
 *         requires(signal.width_valid());
 *         ensures(protocol.is_satisfied());
 *     }
 *
 * Contract expressions remain domain-neutral.
 *
 * Timing, clocking, synthesis, placement, routing, and physical implementation
 * remain HDL/hardware/compiler responsibilities.
 */


/* ============================================================================
 * 32. AI / DATA INTEGRATION
 * ============================================================================
 *
 * AI/data functions may carry contracts such as:
 *
 *     requires(input.schema_matches(schema));
 *     requires(model.is_compatible(input));
 *     ensures(output.shape_valid());
 *
 * The grammar does not enumerate:
 *
 *     TensorFlow
 *     PyTorch
 *     JAX
 *     CUDA
 *     ROCm
 *     vendor-specific frameworks
 *
 * Framework integration belongs to interoperability and semantic lowering.
 */


/* ============================================================================
 * 33. RESOURCE CONTRACT INTEGRATION
 * ============================================================================
 *
 * Resource requirements may be expressed through canonical resource/capability
 * expressions where the language specification permits them.
 *
 * The distinction is:
 *
 *     contract requirement
 *         !=
 *     resource allocation
 *
 * For example:
 *
 *     requires(capability("tensor.compute"));
 *
 * describes an obligation/requirement.
 *
 * It does not select:
 *
 *     GPU 0
 *     GPU 1
 *     accelerator 7
 *
 * nor does it reserve a physical resource.
 */


/* ============================================================================
 * 34. FUNCTION GRAMMAR INTEGRATION
 * ============================================================================
 *
 * `grammar/functions/functions.g4` MUST integrate this grammar.
 *
 * Its imports must include:
 *
 *     FunctionContracts
 *
 * and its canonical function declaration should continue to use:
 *
 *     functionContractClause*
 *
 * after this file becomes the owner of that rule.
 *
 * The following rules MUST be removed from functions.g4:
 *
 *     functionContractClause
 *     functionContractItem
 *     functionRequiresContract
 *     functionEnsuresContract
 *     functionInvariantContract
 *
 * They MUST NOT be duplicated.
 */


/* ============================================================================
 * 35. RETURNS INTEGRATION
 * ============================================================================
 *
 * This file does not own return syntax.
 *
 * Return syntax remains:
 *
 *     grammar/functions/returns.g4
 *     or the repository's selected canonical return-types.g4
 *
 * depending on the repository authority decision.
 *
 * Contracts may refer semantically to a result value, but this grammar does
 * not define result/return syntax.
 */


/* ============================================================================
 * 36. GENERICS INTEGRATION
 * ============================================================================
 *
 * Generic declarations and generic constraints remain owned elsewhere.
 *
 * A contract expression may refer to generic parameters when the semantic
 * system permits it.
 *
 * This grammar does not define:
 *
 *     genericParameter
 *     genericBound
 *     whereClause
 *     traitBound
 *
 * and must not duplicate those rules.
 */


/* ============================================================================
 * 37. EFFECT INTEGRATION
 * ============================================================================
 *
 * Contract expressions participate in the function's effect model according
 * to semantic rules.
 *
 * This grammar does not define:
 *
 *     effect
 *     effectType
 *     handler
 *     capability
 *
 * Those remain owned by the effects/capability grammar.
 */


/* ============================================================================
 * 38. AST CONTRACT
 * ============================================================================
 *
 * The parser must expose enough structure for the frontend AST to preserve:
 *
 *     - contract keyword span;
 *     - contract block span;
 *     - predicate kind;
 *     - expression span;
 *     - predicate terminator span where retained;
 *     - ordering;
 *     - enclosing function association.
 *
 * Conceptually:
 *
 *     FunctionContract {
 *         span,
 *         items: Vec<FunctionContractItem>
 *     }
 *
 *     FunctionContractItem {
 *         kind,
 *         expression,
 *         span
 *     }
 *
 * The exact Rust AST representation belongs to:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT define the Rust structures.
 */


/* ============================================================================
 * 39. SEMANTIC CONTRACT
 * ============================================================================
 *
 * The semantic layer must determine:
 *
 *     - expression validity;
 *     - identifier resolution;
 *     - type correctness;
 *     - legal bindings;
 *     - effect legality;
 *     - capability legality;
 *     - resource semantics;
 *     - contract consistency;
 *     - implication;
 *     - contradiction;
 *     - satisfiability;
 *     - verification strategy;
 *     - runtime-checkability;
 *     - compile-time-checkability.
 *
 * Grammar success MUST NOT be treated as semantic contract validity.
 */


/* ============================================================================
 * 40. IR INTEGRATION
 * ============================================================================
 *
 * Contracts may be represented in semantic metadata attached to functions.
 *
 * They MUST NOT require a new universal IR.
 *
 * For classical functions:
 *
 *     function contract
 *         ->
 *     semantic contract metadata
 *         ->
 *     classical compiler/verification consumers
 *
 * For quantum functions:
 *
 *     function contract
 *         ->
 *     semantic contract metadata
 *         ->
 *     quantum semantic analysis
 *         ->
 *     quantum::ir
 *
 * The function-contract grammar MUST NOT create:
 *
 *     ContractIR
 *     QuantumContractIR
 *     HardwareContractIR
 *
 * merely to represent source contracts.
 */


/* ============================================================================
 * 41. COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler stages may consume contract metadata for:
 *
 *     - static verification;
 *     - optimization assumptions where proven safe;
 *     - diagnostics;
 *     - specialization;
 *     - test generation;
 *     - formal verification;
 *     - runtime instrumentation;
 *     - provenance.
 *
 * A compiler MUST NOT silently assume that an arbitrary contract is true merely
 * because it parsed successfully.
 *
 * Any optimization based on a contract requires semantic validation according
 * to the language's contract model.
 */


/* ============================================================================
 * 42. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime systems may consume contracts only when the semantic/compiler model
 * explicitly lowers them to runtime-checkable obligations.
 *
 * This grammar itself performs no runtime work.
 *
 * Runtime contract checking must remain:
 *
 *     explicit;
 *     deterministic where required;
 *     capability-aware;
 *     resource-aware;
 *     diagnosable.
 */


/* ============================================================================
 * 43. TOOLING INTEGRATION
 * ============================================================================
 *
 * Tooling should be able to identify:
 *
 *     contract blocks;
 *     requires clauses;
 *     ensures clauses;
 *     invariant clauses;
 *     expression source spans.
 *
 * This enables:
 *
 *     IDE diagnostics;
 *     documentation generation;
 *     contract visualization;
 *     static analysis;
 *     verification tooling;
 *     test generation;
 *     refactoring.
 *
 * Tooling must consume parser/AST structures rather than independently
 * reimplementing contract parsing.
 */


/* ============================================================================
 * 44. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Diagnostics should distinguish at least:
 *
 *     malformed contract block;
 *     malformed requires expression;
 *     malformed ensures expression;
 *     malformed invariant expression;
 *     unexpected token;
 *     missing delimiter;
 *     malformed expression.
 *
 * Semantic diagnostics are separate:
 *
 *     unknown identifier;
 *     invalid type;
 *     invalid effect;
 *     unsatisfied capability;
 *     unsatisfied resource requirement;
 *     contradictory contract;
 *     unprovable contract;
 *     illegal contract side effect.
 *
 * The parser grammar must not encode those semantic diagnostics as parser
 * alternatives.
 */


/* ============================================================================
 * 45. ERROR RECOVERY
 * ============================================================================
 *
 * The grammar should remain structurally simple so ANTLR's standard recovery
 * mechanisms can identify the beginning/end of malformed contract items.
 *
 * Keyword-led contract items:
 *
 *     REQUIRES
 *     ENSURES
 *     INVARIANT
 *
 * provide natural synchronization points.
 *
 * No embedded parser actions or target-specific recovery code is permitted.
 */


/* ============================================================================
 * 46. DETERMINISM
 * ============================================================================
 *
 * Given the same source text, lexer vocabulary, grammar version, and parser
 * configuration, this grammar must produce deterministic parse structure.
 *
 * No semantic lookup or runtime state may influence parsing.
 *
 * In particular, parsing must not depend on:
 *
 *     hardware;
 *     device availability;
 *     number of CPUs;
 *     number of GPUs;
 *     number of QPUs;
 *     runtime memory;
 *     network state;
 *     calibration;
 *     scheduler state.
 */


/* ============================================================================
 * 47. SCALABILITY
 * ============================================================================
 *
 * The grammar uses repetition rather than fixed enumerations.
 *
 * There is no grammar-level finite maximum for:
 *
 *     contract blocks;
 *     contract items;
 *     contract expressions;
 *     nested expression structures;
 *     functions;
 *     parameters;
 *     generic parameters;
 *     resources;
 *     capabilities;
 *     quantum objects;
 *     distributed objects.
 *
 * Actual implementation resource limits remain external.
 *
 * This is the required interpretation of:
 *
 *     tiny -> everywhere
 *
 * and:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 */


/* ============================================================================
 * 48. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar intentionally contains no:
 *
 *     MAX_
 *     MIN_
 *     QUBIT_COUNT
 *     CPU_COUNT
 *     GPU_COUNT
 *     FPGA_COUNT
 *     QPU_COUNT
 *     THREAD_COUNT
 *     NODE_COUNT
 *     MEMORY_SIZE
 *     DEVICE_COUNT
 *
 * Any future change introducing such a construct must be reviewed as a
 * potential portability violation.
 */


/* ============================================================================
 * 49. COMPATIBILITY
 * ============================================================================
 *
 * Existing source syntax:
 *
 *     contract {
 *         requires(...);
 *         ensures(...);
 *         invariant(...);
 *     }
 *
 * remains supported.
 *
 * Existing optional predicate terminators remain supported:
 *
 *     requires(...);
 *
 *     requires(...)
 *
 * This preserves the existing grammar's accepted forms while centralizing
 * their ownership.
 *
 * No existing valid contract syntax should be removed merely because ownership
 * moves from functions.g4 to this file.
 */


/* ============================================================================
 * 50. VERSIONING
 * ============================================================================
 *
 * This file does not define an independent language version.
 *
 * Function-contract syntax follows the repository's canonical Zamani language
 * version and compatibility policy.
 *
 * Any breaking change requires:
 *
 *     - language-version classification;
 *     - migration documentation;
 *     - compatibility tests;
 *     - parser tests;
 *     - AST compatibility review;
 *     - semantic compatibility review.
 */


/* ============================================================================
 * 51. DEPRECATION
 * ============================================================================
 *
 * Deprecated contract syntax must not be silently removed.
 *
 * The compatibility subsystem owns:
 *
 *     - deprecation version;
 *     - replacement syntax;
 *     - diagnostic policy;
 *     - removal version;
 *     - migration guidance.
 *
 * This grammar only represents syntax that is currently accepted.
 */


/* ============================================================================
 * 52. TEST CONTRACT
 * ============================================================================
 *
 * The corresponding tests belong under:
 *
 *     grammar/tests/functions/
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *     grammar/tests/determinism/
 *     grammar/tests/compatibility/
 *
 * At minimum, test:
 *
 * POSITIVE
 * --------
 *
 *     contract {}
 *
 *     contract {
 *         requires(x >= 0);
 *     }
 *
 *     contract {
 *         ensures(result >= 0);
 *     }
 *
 *     contract {
 *         invariant(state.is_valid());
 *     }
 *
 *     contract {
 *         requires(a);
 *         requires(b);
 *         ensures(c);
 *         invariant(d);
 *     }
 *
 *     multiple contract blocks where supported by functions.g4.
 *
 * NEGATIVE
 * --------
 *
 *     contract {
 *         requires();
 *     }
 *
 *     contract {
 *         ensures();
 *     }
 *
 *     contract {
 *         invariant();
 *     }
 *
 *     contract {
 *         requires(x >= 0;
 *     }
 *
 *     contract {
 *         unknown_keyword(x);
 *     }
 *
 * BOUNDARY
 * --------
 *
 *     empty contract;
 *     many contract items;
 *     deeply nested expressions;
 *     long qualified names;
 *     large generic expressions;
 *     multiple contracts.
 *
 * SCALABILITY
 * ----------
 *
 *     large contract blocks;
 *     many independent predicates;
 *     large expressions;
 *     large source files;
 *     cross-domain contract expressions.
 *
 * DETERMINISM
 * -----------
 *
 *     parse identical source repeatedly and verify equivalent parser/AST
 *     structure.
 *
 * CROSS-DOMAIN
 * -----------
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     distributed;
 *     AI/data;
 *     networking;
 *     security.
 */


/* ============================================================================
 * 53. NO DOMAIN-SPECIFIC DUPLICATION
 * ============================================================================
 *
 * Do NOT create:
 *
 *     quantumRequiresContract
 *     quantumEnsuresContract
 *     hdlRequiresContract
 *     gpuRequiresContract
 *     distributedRequiresContract
 *     aiRequiresContract
 *
 * unless a future specification introduces genuinely distinct syntax.
 *
 * Domain semantics belong downstream.
 */


/* ============================================================================
 * 54. NO PROOF LANGUAGE DUPLICATION
 * ============================================================================
 *
 * Formal proof syntax belongs to the verification/proof subsystem if Zamani
 * exposes such syntax.
 *
 * This file does not become a theorem prover merely because contracts can
 * become proof obligations.
 *
 * The separation is:
 *
 *     contract syntax
 *         ->
 *     semantic contract
 *         ->
 *     verification/proof subsystem
 *
 */


/* ============================================================================
 * 55. NO RUNTIME LANGUAGE DUPLICATION
 * ============================================================================
 *
 * Runtime assertions, monitors, tracing, instrumentation, and recovery
 * policies belong to execution/runtime subsystems.
 *
 * Contract syntax only states the source-level contract.
 */


/* ============================================================================
 * 56. NO RESOURCE ALLOCATION
 * ============================================================================
 *
 * A contract cannot itself allocate:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     memory
 *     storage
 *     network bandwidth
 *     physical qubits
 *     devices
 *
 * Resource systems consume semantic requirements later.
 */


/* ============================================================================
 * 57. NO HARDWARE COUPLING
 * ============================================================================
 *
 * The contract grammar remains valid for:
 *
 *     embedded systems;
 *     desktops;
 *     servers;
 *     clusters;
 *     cloud systems;
 *     edge systems;
 *     accelerators;
 *     quantum systems;
 *     FPGA/ASIC systems;
 *     future computational substrates.
 *
 * No hardware-specific grammar branch is required.
 */


/* ============================================================================
 * 58. POCO-REAF CONTRACT
 * ============================================================================
 *
 * A function contract is portable when its meaning is independent of the
 * particular target on which the function is eventually realized.
 *
 * Therefore:
 *
 *     source contract
 *          |
 *          v
 *     semantic contract
 *          |
 *          +------------------+
 *          |                  |
 *          v                  v
 *     static verification   runtime verification
 *          |
 *          v
 *     compiler lowering
 *          |
 *          +---------+--------+--------+
 *          |         |        |        |
 *         CPU       GPU      QPU      FPGA
 *          |         |        |        |
 *          +---------+--------+--------+
 *                    |
 *                    v
 *                future targets
 *
 * The contract remains a property of the computation, not of a particular
 * machine.
 */


/* ============================================================================
 * 59. FINAL AUTHORITY CONTRACT
 * ============================================================================
 *
 * This file is authoritative for FUNCTION CONTRACT SYNTAX only.
 *
 * Authority chain:
 *
 *     specification
 *          |
 *          v
 *     lexer
 *          |
 *          v
 *     FunctionContracts
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic contract model
 *          |
 *          v
 *     verification/compiler/runtime consumers
 *
 * There must be no parallel implementation of these parser rules in:
 *
 *     functions.g4
 *     core/constraints.g4
 *     functions/constraints.g4
 *     Zamani.g4
 *     legacy ANTLR grammars.
 *
 */


/* ============================================================================
 * 60. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when all of the following are true:
 *
 * [ ] FunctionContracts compiles under ANTLR4.
 *
 * [ ] tokenVocab is the canonical ZamaniLexer.
 *
 * [ ] No lexer rules are duplicated.
 *
 * [ ] expression is consumed from the canonical expression grammar.
 *
 * [ ] functionContractClause has exactly one authoritative owner.
 *
 * [ ] functionContractItem has exactly one authoritative owner.
 *
 * [ ] requires syntax is covered.
 *
 * [ ] ensures syntax is covered.
 *
 * [ ] invariant syntax is covered.
 *
 * [ ] Existing valid contract syntax remains accepted.
 *
 * [ ] Function grammar imports this grammar.
 *
 * [ ] functions.g4 no longer duplicates these rules.
 *
 * [ ] Return syntax remains owned by the return grammar.
 *
 * [ ] Generic constraints remain owned by constraints.g4.
 *
 * [ ] No type grammar is duplicated.
 *
 * [ ] No expression grammar is duplicated.
 *
 * [ ] No resource limit is encoded.
 *
 * [ ] No hardware-specific syntax is encoded.
 *
 * [ ] No quantum-specific contract grammar is duplicated.
 *
 * [ ] AST mapping is documented.
 *
 * [ ] Semantic contract mapping is documented.
 *
 * [ ] IR integration is documented.
 *
 * [ ] Compiler integration is documented.
 *
 * [ ] Runtime integration is documented.
 *
 * [ ] Tooling integration is documented.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Hard-coding audit passes.
 *
 * [ ] No unsafe Rust is required anywhere in the implementation.
 *
 * ============================================================================
 * END OF FUNCTION CONTRACTS GRAMMAR
 * ============================================================================
 */