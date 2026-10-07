/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/compile/specialization.g4
 *
 * GRAMMAR
 * -------
 * CompileSpecialization
 *
 * STATUS
 * ------
 * PRODUCTION-READY COMPILATION SPECIALIZATION GRAMMAR
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical source-level grammar for COMPILATION
 * SPECIALIZATION INTENT.
 *
 * Specialization intent describes how an already-valid Zamani semantic
 * program may be specialized by the compiler while preserving the program's
 * declared semantic contract.
 *
 * This file describes SOURCE SYNTAX ONLY.
 *
 * It does not perform specialization.
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
 *     domain-neutral AST
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     program semantics          compilation intent
 *                                        |
 *                                        v
 *                              specialization policy
 *                                        |
 *                                        v
 *                               semantic analysis
 *                                        |
 *              +-----------------+-------+-----------------+
 *              |                 |                         |
 *              v                 v                         v
 *       type/generic         resource/capability      portability/
 *       validation           analysis                 policy analysis
 *              |                 |                         |
 *              +-----------------+-------------------------+
 *                                        |
 *                                        v
 *                              specialization planning
 *                                        |
 *                                        v
 *                              canonical semantic model
 *                                        |
 *                              +---------+---------+
 *                              |                   |
 *                              v                   v
 *                         classical            quantum::ir
 *                              |                   |
 *                              +---------+---------+
 *                                        |
 *                                        v
 *                                optimization
 *                                        |
 *                                   lowering
 *                                        |
 *                              routing/scheduling
 *                                        |
 *                              resilience / QEC
 *                                        |
 *                                       ZQN
 *                                        |
 *                                       HAL
 *                                        |
 *                                 target realization
 *
 * Specialization is therefore a compilation-policy boundary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * -------------
 *
 *     compileSpecializationDeclaration
 *     compileSpecializationSpecification
 *     compileSpecializationClause
 *     compileSpecializationTarget
 *     compileSpecializationArguments
 *     compileSpecializationCondition
 *     compileSpecializationRequirement
 *     compileSpecializationConstraint
 *     compileSpecializationPreference
 *     compileSpecializationHint
 *     compileSpecializationPolicy
 *     compileSpecializationFallback
 *     compileSpecializationScope
 *     compileSpecializationMetadata
 *
 * It owns the SOURCE-LEVEL COMPOSITION of these specialization policies.
 *
 * THIS FILE DOES NOT OWN
 * ---------------------
 *
 *     generic declarations
 *     generic parameter declarations
 *     generic type applications
 *     ordinary expressions
 *     ordinary types
 *     resource declarations
 *     capability declarations
 *     target declarations
 *     target selection
 *     optimization algorithms
 *     specialization algorithms
 *     monomorphization
 *     constant evaluation
 *     partial evaluation
 *     code generation
 *     lowering implementation
 *     routing
 *     scheduling
 *     quantum physical mapping
 *     QEC
 *     ZQN
 *     HAL
 *     runtime dispatch
 *     hardware discovery
 *     resource allocation
 *
 * ============================================================================
 * AUTHORITY SEPARATION
 * ============================================================================
 *
 * Explicit source specialization requests are owned by:
 *
 *     grammar/metaprogramming/specialization.g4
 *
 * Compilation-time control is owned by:
 *
 *     grammar/compile/compile-time.g4
 *
 * Target intent is owned by:
 *
 *     grammar/compile/target.g4
 *
 * Target-selection intent is owned by:
 *
 *     grammar/compile/target-selection.g4
 *
 * Optimization intent is owned by:
 *
 *     grammar/compile/optimization.g4
 *
 * Resource intent is owned by:
 *
 *     grammar/resources/
 *
 * Generic/type syntax is owned by:
 *
 *     grammar/types/
 *     grammar/functions/
 *
 * This grammar MUST NOT redefine those authorities.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION
 * ============================================================================
 *
 * There are three related but distinct concepts:
 *
 *     1. Generic specialization request
 *        --------------------------------
 *        Explicit source-level specialization/application.
 *
 *        Owner:
 *            grammar/metaprogramming/specialization.g4
 *
 *     2. Compilation specialization policy
 *        ----------------------------------
 *        Compilation intent describing when/under what constraints a
 *        specialization may or should occur.
 *
 *        Owner:
 *            THIS FILE
 *
 *     3. Specialization implementation
 *        -----------------------------
 *        The compiler algorithm that performs specialization.
 *
 *        Owner:
 *            compiler semantic/specialization implementation
 *
 * These must not become one grammar authority.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Specialization may change implementation representation, but MUST preserve
 * the source program's declared semantic meaning.
 *
 * This grammar therefore does not encode:
 *
 *     a particular CPU
 *     a particular CPU core
 *     a particular GPU
 *     a particular FPGA
 *     a particular ASIC
 *     a particular QPU
 *     a physical qubit
 *     a physical memory bank
 *     a physical device identifier
 *     a provider
 *     a machine topology
 *     a deployment location
 *     a fixed node count
 *     a fixed processor count
 *
 * Target/resource information is consumed semantically through the existing
 * resource, capability, target, policy, and execution systems.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains NO language-level capacity ceilings.
 *
 * In particular it contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_SPECIALIZATIONS
 *     MAX_ARGUMENTS
 *     MAX_CLAUSES
 *
 * Repetition is represented by grammar repetition operators.
 *
 * Actual implementation limits belong to compiler/resource policy and
 * execution infrastructure, not the language definition.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * which delegates to:
 *
 *     grammar/lexer/
 *
 * This grammar defines NO lexer rules.
 *
 * Existing canonical tokens consumed here include:
 *
 *     COMPILE
 *     WITH
 *     IF
 *     REQUIRES
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     FALLBACK
 *     POLICY
 *     PORTABILITY
 *     DETERMINISTIC
 *     REPRODUCIBLE
 *     SCOPE
 *     CAPABILITY
 *
 * IMPORTANT:
 *
 * The current canonical lexer does not contain a dedicated `SPECIALIZE`
 * token.
 *
 * Therefore this grammar intentionally does NOT reference:
 *
 *     K_SPECIALIZE
 *     SPECIALIZE
 *
 * Instead, the source spelling `specialize` is represented as an ordinary
 * identifier and validated contextually by the semantic/compiler layer.
 *
 * This preserves the current lexer authority and avoids a phantom token.
 *
 * If a future language revision reserves `specialize` lexically, that is a
 * versioned lexer change and does not alter the semantic ownership of this
 * file.
 *
 * ============================================================================
 * PARSER DEPENDENCIES
 * ============================================================================
 *
 * Direct parser dependencies:
 *
 *     Core
 *     Types
 *     Expressions
 *
 * Core supplies:
 *
 *     identifier
 *     qualifiedName
 *     attribute
 *
 * Types supplies:
 *
 *     typeExpression
 *
 * Expressions supplies:
 *
 *     expression
 *
 * Resource-specific expressions are intentionally NOT duplicated here.
 *
 * Resource requirements and capability requirements are expressed through
 * canonical expressions and interpreted by semantic resource analysis.
 *
 * ============================================================================
 */

parser grammar CompileSpecialization;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Types,
    Expressions;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Canonical complete source form:
 *
 *     compile specialize target ... ;
 *
 * `specialize` remains an IDENTIFIER under the current lexer.
 *
 * The semantic layer MUST validate that the contextual identifier is the
 * specialization introducer.
 *
 * A compile declaration may contain other compilation clauses, but this file
 * owns only the specialization clause.
 *
 * ============================================================================
 */

compileSpecializationDeclaration
    : COMPILE
      compileSpecializationKeyword
      compileSpecializationTarget
      compileSpecializationSpecification?
      SEMI?
    ;


/*
 * ============================================================================
 * SPECIALIZATION KEYWORD
 * ============================================================================
 *
 * The current lexer deliberately does not reserve `specialize`.
 *
 * This rule therefore consumes IDENTIFIER and semantic analysis validates its
 * canonical spelling/context.
 *
 * This avoids:
 *
 *     K_SPECIALIZE
 *     SPECIALIZE
 *
 * phantom-token references.
 *
 * It also avoids forcing every future contextual specialization vocabulary
 * item into the global lexer.
 */

compileSpecializationKeyword
    : identifier
    ;


/*
 * ============================================================================
 * SPECIALIZATION TARGET
 * ============================================================================
 *
 * The target is a symbolic semantic entity.
 *
 * Examples:
 *
 *     compute
 *     math::compute
 *     quantum::algorithm
 *     model::inference
 *     hardware::kernel
 *
 * The target is NOT a physical device.
 *
 * Physical target selection remains owned by:
 *
 *     grammar/compile/target.g4
 *     grammar/compile/target-selection.g4
 */

compileSpecializationTarget
    : qualifiedName
    ;


/*
 * ============================================================================
 * SPECIALIZATION SPECIFICATION
 * ============================================================================
 *
 * A specialization specification is an ordered, open-ended sequence of
 * specialization policies.
 *
 * The source order is significant for:
 *
 *     diagnostics
 *     tooling
 *     provenance
 *     deterministic formatting
 *
 * Semantic precedence is NOT determined by parser alternative ordering.
 *
 * The semantic model determines the precedence between:
 *
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     policies
 *     fallback behavior
 *
 * ============================================================================
 */

compileSpecializationSpecification
    : compileSpecializationClause+
    ;


/*
 * ============================================================================
 * SPECIALIZATION CLAUSE
 * ============================================================================
 *
 * This is the sole clause dispatcher for this grammar.
 *
 * It intentionally delegates to narrowly owned rules.
 *
 * No generic `identifier expression` fallback is provided.
 *
 * This is important because a generic fallback would make malformed
 * specialization syntax indistinguishable from arbitrary identifiers.
 */

compileSpecializationClause
    : compileSpecializationArguments
    | compileSpecializationCondition
    | compileSpecializationRequirement
    | compileSpecializationConstraint
    | compileSpecializationPreference
    | compileSpecializationHint
    | compileSpecializationPolicy
    | compileSpecializationFallback
    | compileSpecializationScope
    | compileSpecializationMetadata
    ;


/*
 * ============================================================================
 * EXPLICIT SPECIALIZATION ARGUMENTS
 * ============================================================================
 *
 * Canonical form:
 *
 *     with (argument, argument, ...)
 *
 * Arguments are ordinary expressions.
 *
 * Named arguments use:
 *
 *     name = expression
 *
 * Type-specific meaning is determined downstream.
 *
 * This grammar does not define generic declarations or generic constraints.
 */

compileSpecializationArguments
    : WITH
      LPAREN
      compileSpecializationArgumentList?
      RPAREN
    ;


compileSpecializationArgumentList
    : compileSpecializationArgument
      (
          COMMA
          compileSpecializationArgument
      )*
      COMMA?
    ;


compileSpecializationArgument
    : identifier
      ASSIGN
      expression
    | expression
    ;


/*
 * ============================================================================
 * CONDITIONAL SPECIALIZATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     if expression
 *
 * The expression is not evaluated by the parser.
 *
 * The semantic/compiler layer determines whether the condition is valid in
 * the compilation context.
 */

compileSpecializationCondition
    : IF
      expression
    ;


/*
 * ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * Canonical form:
 *
 *     requires expression
 *
 * Examples:
 *
 *     requires qubits >= required_qubits
 *
 *     requires memory >= required_memory
 *
 *     requires capability("quantum.measurement")
 *
 *     requires capability("tensor.compute")
 *
 * This grammar deliberately does not define resource or capability syntax.
 *
 * Those concepts remain semantically owned by the existing resource and
 * capability systems.
 */

compileSpecializationRequirement
    : REQUIRES
      expression
    ;


/*
 * ============================================================================
 * CONSTRAINT
 * ============================================================================
 *
 * Canonical form:
 *
 *     constraint expression
 *
 * A constraint restricts the legal specialization space.
 *
 * It is distinct from a requirement:
 *
 *     requirement = mandatory semantic condition
 *
 *     constraint = restriction on the legal solution space
 *
 * Semantic enforcement is downstream.
 */

compileSpecializationConstraint
    : CONSTRAINT
      expression
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 *
 * Canonical form:
 *
 *     prefer expression
 *
 * A preference is non-binding.
 *
 * It MUST NOT silently become a requirement.
 */

compileSpecializationPreference
    : PREFER
      expression
    ;


/*
 * ============================================================================
 * HINT
 * ============================================================================
 *
 * Canonical form:
 *
 *     hint expression
 *
 * A hint is non-binding information for downstream compilation planning.
 *
 * It MUST NOT be treated as a correctness condition unless another
 * semantic construct explicitly establishes that meaning.
 */

compileSpecializationHint
    : HINT
      expression
    ;


/*
 * ============================================================================
 * POLICY
 * ============================================================================
 *
 * Policy provides a structured specialization-policy boundary.
 *
 * Canonical forms:
 *
 *     policy expression
 *
 *     policy {
 *         ...
 *     }
 *
 * This grammar intentionally keeps policy internals generic.
 *
 * The universal policy subsystem remains the semantic authority.
 */

compileSpecializationPolicy
    : POLICY
      compileSpecializationPolicyBody
    ;


compileSpecializationPolicyBody
    : expression
    | compileSpecializationPolicyBlock
    ;


compileSpecializationPolicyBlock
    : LBRACE
      compileSpecializationPolicyEntry*
      RBRACE
    ;


compileSpecializationPolicyEntry
    : compileSpecializationPolicyAssignment
    | compileSpecializationRequirement
    | compileSpecializationConstraint
    | compileSpecializationPreference
    | compileSpecializationHint
    | compileSpecializationMetadata
    ;


compileSpecializationPolicyAssignment
    : identifier
      ASSIGN
      expression
      SEMI?
    ;


/*
 * ============================================================================
 * FALLBACK
 * ============================================================================
 *
 * Fallback is compilation policy.
 *
 * It is NOT runtime exception handling.
 *
 * Canonical forms:
 *
 *     fallback target
 *
 *     fallback expression
 *
 * The semantic layer determines whether the fallback is:
 *
 *     valid
 *     equivalent
 *     portable
 *     permitted
 *     applicable
 *
 * No physical target is selected by this grammar.
 */

compileSpecializationFallback
    : FALLBACK
      compileSpecializationFallbackTarget
    ;


compileSpecializationFallbackTarget
    : qualifiedName
    | expression
    ;


/*
 * ============================================================================
 * SCOPE
 * ============================================================================
 *
 * Scope identifies the semantic region to which the specialization policy
 * applies.
 *
 * It does NOT create a lexical scope.
 *
 * Canonical form:
 *
 *     scope qualifiedName
 *
 * Scope resolution is semantic.
 */

compileSpecializationScope
    : SCOPE
      qualifiedName
    ;


/*
 * ============================================================================
 * METADATA
 * ============================================================================
 *
 * Attributes remain owned by Core.
 *
 * This rule merely provides the specialization-policy composition boundary.
 */

compileSpecializationMetadata
    : attribute
    ;


/*
 * ============================================================================
 * REUSABLE SPECIALIZATION PAYLOAD
 * ============================================================================
 *
 * Parent compilation grammars should consume:
 *
 *     compileSpecializationDeclaration
 *
 * for a complete source declaration.
 *
 * They may consume:
 *
 *     compileSpecializationSpecification
 *
 * when specialization policy is embedded in another already-owned construct.
 *
 * This provides one reusable payload boundary without introducing another
 * compilation root.
 */

compileSpecializationPolicySpecification
    : compileSpecializationSpecification
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The frontend AST must preserve, at minimum:
 *
 *     specialization declaration
 *     contextual specialization introducer
 *     target
 *     ordered arguments
 *     named argument names
 *     argument expressions
 *     ordered clauses
 *     clause kind
 *     clause expression
 *     policy structure
 *     fallback target/expression
 *     scope
 *     metadata
 *     source spans
 *     source ordering
 *     source provenance
 *
 * The AST MUST remain domain-neutral.
 *
 * It MUST NOT contain:
 *
 *     physical CPU IDs
 *     physical GPU IDs
 *     physical FPGA coordinates
 *     physical qubit IDs
 *     device handles
 *     backend handles
 *     schedules
 *     routes
 *     calibration records
 *     allocation records
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     1. Validate the contextual `specialize` introducer.
 *
 *     2. Resolve the specialization target.
 *
 *     3. Determine whether the target is specializable.
 *
 *     4. Resolve named and positional arguments.
 *
 *     5. Validate generic/type/value compatibility.
 *
 *     6. Resolve generic constraints.
 *
 *     7. Validate compile-time evaluability where required.
 *
 *     8. Resolve requirements.
 *
 *     9. Resolve constraints.
 *
 *    10. Rank preferences.
 *
 *    11. Record hints without treating them as requirements.
 *
 *    12. Resolve policy.
 *
 *    13. Validate fallback semantics.
 *
 *    14. Validate scope.
 *
 *    15. Validate portability.
 *
 *    16. Validate capability/resource feasibility.
 *
 *    17. Determine whether specialization preserves semantics.
 *
 *    18. Determine whether specialization is:
 *
 *            unnecessary
 *            deferred
 *            partial
 *            complete
 *            target-aware
 *            rejected
 *
 *    19. Preserve provenance.
 *
 *    20. Produce deterministic diagnostics.
 *
 * None of these operations occur in this grammar.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * This grammar intentionally does not own resource or capability syntax.
 *
 * The following remain valid semantic concepts:
 *
 *     requires qubits >= required_qubits
 *
 *     requires memory >= required_memory
 *
 *     requires capability("quantum.measurement")
 *
 *     requires capability("tensor.compute")
 *
 * The values remain symbolic/source-level information.
 *
 * The grammar never converts them into:
 *
 *     physical allocation
 *     machine inventory
 *     device selection
 *     fixed hardware capacity
 *
 * ============================================================================
 * PORTABILITY CONTRACT
 * ============================================================================
 *
 * A specialization must preserve the declared portable semantics unless the
 * source explicitly requests a non-portable semantic property through an
 * authoritative target/policy mechanism.
 *
 * A compiler MUST NOT silently turn specialization into:
 *
 *     vendor lock-in
 *     physical-device dependence
 *     physical-qubit dependence
 *     fixed topology dependence
 *     unavailable capability dependence
 *
 * If specialization cannot satisfy the semantic contract, the compiler must:
 *
 *     reject it,
 *
 * or:
 *
 *     select another legal specialization/fallback according to the declared
 *     policy.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum specialization remains target-neutral.
 *
 * Examples include specialization according to:
 *
 *     logical resource requirements
 *     semantic quantum capabilities
 *     algorithm parameters
 *     measurement requirements
 *     resilience requirements
 *     symbolic resource constraints
 *
 * This grammar MUST NOT enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     CZ
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *
 * or any other finite operation catalogue.
 *
 * Quantum semantic operations ultimately cross:
 *
 *     quantum::ir
 *
 * before optimization, decomposition, routing, scheduling, resilience/QEC,
 * ZQN and HAL.
 *
 * No specialization-specific quantum IR is permitted.
 *
 * ============================================================================
 * CLASSICAL CONTRACT
 * ============================================================================
 *
 * Specialization may apply to:
 *
 *     scalar computation
 *     vector computation
 *     tensor computation
 *     numerical algorithms
 *     symbolic algorithms
 *     parallel computation
 *     data transformations
 *     accelerator-independent computation
 *
 * Instruction sets and processor models are not grammar concepts.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Specialization may apply to semantic hardware/HDL intent.
 *
 * It MUST NOT define:
 *
 *     wire widths
 *     register widths
 *     fixed memory sizes
 *     fixed pipeline depths
 *     fixed device counts
 *     fixed clock frequencies
 *
 * unless such values are ordinary program semantics rather than universal
 * implementation ceilings.
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * AI / DATA CONTRACT
 * ============================================================================
 *
 * The same specialization policy may apply to:
 *
 *     models
 *     tensors
 *     datasets
 *     inference
 *     training
 *     symbolic computation
 *     data pipelines
 *     distributed computation
 *
 * Framework-specific behavior remains outside this grammar.
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Specialization may adapt semantic computation to distributed capabilities.
 *
 * This grammar does not enumerate:
 *
 *     node identifiers
 *     fixed cluster sizes
 *     process counts
 *     network sizes
 *     topology shapes
 *
 * Distributed realization remains a resource/target/execution concern.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Specialization itself does not grant effects.
 *
 * If specialization causes or requests compile-time evaluation that has
 * effects, those effects must be represented and checked by the existing
 * effect and security systems.
 *
 * In particular this grammar does not grant implicit access to:
 *
 *     filesystem
 *     network
 *     subprocesses
 *     environment variables
 *     devices
 *     secrets
 *     mutable compiler-global state
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing specialization syntax grants no authority.
 *
 * Sandbox, capability, trust, authorization, provenance, and policy
 * enforcement remain downstream responsibilities.
 *
 * Compile-time evaluation must obey the repository's safe execution model.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     token sequence
 *     grammar version
 *
 * It MUST NOT depend on:
 *
 *     wall-clock time
 *     random state
 *     hardware availability
 *     filesystem state
 *     network state
 *     runtime state
 *
 * Source ordering must be preserved.
 *
 * If deterministic specialization is requested semantically, the compiler
 * must make specialization decisions deterministic under the applicable
 * compilation contract.
 *
 * ============================================================================
 * REPRODUCIBILITY CONTRACT
 * ============================================================================
 *
 * Reproducibility is distinct from deterministic parsing.
 *
 * Reproducible specialization may depend on:
 *
 *     language version
 *     compiler version
 *     specialization policy
 *     canonical semantic inputs
 *     explicit configuration
 *     stable capability/resource descriptions
 *     explicit random seeds where applicable
 *
 * These are compiler/provenance concerns.
 *
 * This grammar preserves the source-level intent only.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The semantic representation must preserve enough information to trace:
 *
 *     source
 *       |
 *       v
 *     specialization request
 *       |
 *       v
 *     specialization decision
 *       |
 *       v
 *     derived semantic representation
 *       |
 *       v
 *     canonical IR
 *
 * Provenance must not be replaced by implementation-specific specialization
 * keys.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * It must not introduce:
 *
 *     SpecializationIR
 *     CompileSpecializationIR
 *     QuantumSpecializationIR
 *     HardwareSpecializationIR
 *
 * Specialization results are represented through the repository's canonical
 * semantic representation.
 *
 * Quantum constructs continue through:
 *
 *     quantum::ir
 *
 * There is exactly one canonical quantum IR boundary.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Required semantic pipeline:
 *
 *     compileSpecializationDeclaration
 *                |
 *                v
 *          frontend AST
 *                |
 *                v
 *          name resolution
 *                |
 *                v
 *       type/generic validation
 *                |
 *                v
 *      requirement/constraint
 *          analysis
 *                |
 *                v
 *     capability/resource analysis
 *                |
 *                v
 *       portability/policy
 *          validation
 *                |
 *                v
 *     specialization planning
 *                |
 *        +-------+-------+
 *        |       |       |
 *        v       v       v
 *      defer   partial  complete
 *        |       |       |
 *        +-------+-------+
 *                |
 *                v
 *      canonical semantic model
 *                |
 *        +-------+-------+
 *        |       |       |
 *        v       v       v
 *    classical quantum::ir HDL/hardware
 *                |
 *                v
 *       optimization/lowering
 *                |
 *          routing/scheduling
 *                |
 *        resilience/QEC
 *                |
 *               ZQN
 *                |
 *               HAL
 *                |
 *         target realization
 *
 * ============================================================================
 * COMPOSITION INTEGRATION
 * ============================================================================
 *
 * Canonical compilation composition root:
 *
 *     grammar/compile/compile.g4
 *
 * It imports:
 *
 *     CompileSpecialization
 *
 * and exposes:
 *
 *     compileSpecializationDeclaration
 *
 * through its existing:
 *
 *     compileSpecializationReference
 *
 * rule.
 *
 * NO second composition root is required.
 *
 * `grammar/compile/compilation.g4`, if retained for compatibility, must not
 * become another authoritative compilation root or duplicate this rule.
 *
 * ============================================================================
 * METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * `grammar/metaprogramming/specialization.g4` owns explicit specialization
 * requests.
 *
 * It must remain independent from this grammar.
 *
 * The semantic model may combine:
 *
 *     explicit specialization request
 *     +
 *     compilation specialization policy
 *
 * without making either grammar import the other.
 *
 * This avoids circular grammar dependencies.
 *
 * ============================================================================
 * COMPILE-TIME INTEGRATION
 * ============================================================================
 *
 * `grammar/compile/compile-time.g4` owns compile-time control.
 *
 * It may cause specialization through compile-time semantics, but it must not
 * redefine the specialization-policy grammar.
 *
 * Any common specialization semantic model belongs downstream in semantic
 * analysis, not in either parser grammar.
 *
 * ============================================================================
 * TARGET INTEGRATION
 * ============================================================================
 *
 * `grammar/compile/target.g4` owns target intent.
 *
 * `grammar/compile/target-selection.g4` owns target-selection policy.
 *
 * This grammar may be consumed alongside those grammars semantically.
 *
 * It must not:
 *
 *     select a physical target
 *     allocate hardware
 *     enumerate devices
 *     bind physical qubits
 *     select CPU cores
 *     select GPU indices
 *
 * ============================================================================
 * OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * `grammar/compile/optimization.g4` owns optimization intent.
 *
 * Optimization may consume specialization results.
 *
 * This grammar must not define:
 *
 *     optimization passes
 *     cost models
 *     optimizer algorithms
 *     machine instruction selection
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource and capability information remains owned by:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *
 * The specialization grammar consumes their semantic results indirectly
 * through expressions and semantic analysis.
 *
 * It does not import the entire Resources composition grammar merely to
 * repeat resource syntax.
 *
 * ============================================================================
 * FRONTEND / AST INTEGRATION
 * ============================================================================
 *
 * The canonical frontend must map parser contexts into the repository's
 * domain-neutral AST.
 *
 * The AST must preserve:
 *
 *     target
 *     arguments
 *     clauses
 *     source spans
 *     source ordering
 *     provenance
 *
 * It must not introduce backend-specific specialization structures.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime does not parse this grammar.
 *
 * Runtime consumes the semantic/IR result after specialization planning and
 * any permitted specialization have completed.
 *
 * Runtime hardware discovery is never a parser dependency.
 *
 * ============================================================================
 * TOOLING INTEGRATION
 * ============================================================================
 *
 * Tooling must be able to:
 *
 *     locate specialization declarations
 *     locate specialization targets
 *     locate specialization arguments
 *     locate named arguments
 *     locate requirements
 *     locate constraints
 *     locate preferences
 *     locate hints
 *     locate policies
 *     locate fallbacks
 *     locate scope
 *     preserve source spans
 *     format source deterministically
 *     expose specialization provenance
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics include:
 *
 *     missing specialization introducer
 *     missing target
 *     malformed argument list
 *     malformed argument
 *     malformed condition
 *     malformed requirement
 *     malformed constraint
 *     malformed preference
 *     malformed hint
 *     malformed policy
 *     malformed fallback
 *     malformed scope
 *     malformed metadata
 *
 * Semantic diagnostics include:
 *
 *     invalid specialization introducer
 *     unknown specialization target
 *     target is not specializable
 *     wrong specialization argument count
 *     duplicate named specialization argument
 *     unknown specialization argument
 *     invalid type argument
 *     invalid value argument
 *     unsatisfied generic constraint
 *     unsatisfied resource requirement
 *     unavailable capability
 *     incompatible policy
 *     invalid fallback
 *     non-portable specialization
 *     semantics-changing specialization
 *     impossible specialization
 *
 * Parser grammar cardinality MUST NOT be used to represent semantic errors.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing explicit specialization syntax remains owned by:
 *
 *     grammar/metaprogramming/specialization.g4
 *
 * This file does not replace that grammar.
 *
 * The contextual source spelling:
 *
 *     compile specialize ...
 *
 * is compatible with the current lexer because `specialize` remains an
 * identifier.
 *
 * If `specialize` becomes a reserved keyword in a future language version:
 *
 *     lexer vocabulary
 *     compatibility rules
 *     conformance tests
 *
 * must be updated as one versioned change.
 *
 * No phantom `K_SPECIALIZE` token may be introduced into this file.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms should be accepted structurally:
 *
 *     compile specialize compute;
 *
 *     compile specialize compute with (width = width);
 *
 *     compile specialize ns::compute
 *         with (precision = precision);
 *
 *     compile specialize quantum_algorithm
 *         requires qubits >= required_qubits;
 *
 *     compile specialize tensor_kernel
 *         requires capability("tensor.compute");
 *
 *     compile specialize algorithm
 *         requires memory >= required_memory;
 *
 *     compile specialize algorithm
 *         prefer capability("gpu.compute");
 *
 *     compile specialize algorithm {
 *         requires capability("tensor.compute");
 *         prefer vectorization;
 *         hint cache_locality;
 *     };
 *
 *     compile specialize algorithm
 *         policy {
 *             deterministic = true;
 *             reproducible = true;
 *         };
 *
 *     compile specialize algorithm
 *         fallback simulation;
 *
 *     compile specialize algorithm
 *         scope module::algorithm;
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The parser must reject malformed forms such as:
 *
 *     compile specialize;
 *
 *     compile specialize with (...);
 *
 *     compile specialize target with ();
 *
 *     compile specialize target with (a =);
 *
 *     compile specialize target requires;
 *
 *     compile specialize target constraint;
 *
 *     compile specialize target prefer;
 *
 *     compile specialize target hint;
 *
 *     compile specialize target fallback;
 *
 *     compile specialize target scope;
 *
 *     compile specialize target {
 *
 *     compile specialize target with (,);
 *
 *     compile specialize target with (a =, b);
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Tests must cover:
 *
 *     empty policy block
 *     single clause
 *     many clauses
 *     repeated clause categories
 *     nested expressions
 *     deeply qualified names
 *     named arguments
 *     positional arguments
 *     mixed named/positional arguments
 *     nested policy blocks
 *     unknown capability names
 *     symbolic resources
 *     quantum requirements
 *     classical requirements
 *     HDL requirements
 *     distributed requirements
 *     accelerator requirements
 *     AI/data specialization
 *     fallback expressions
 *     metadata
 *
 * Duplicate semantic properties are not parser errors merely because they
 * repeat. The semantic layer decides whether repetition is legal.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must progressively exercise:
 *
 *     larger argument lists
 *     larger clause sequences
 *     larger policy blocks
 *     deeper qualified names
 *     deeper expression trees
 *     many specialization declarations
 *
 * No grammar-level maximum is defined.
 *
 * Tests must therefore be parameterized rather than written around an
 * artificial universal maximum.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Identical source and identical grammar version must produce structurally
 * equivalent parse trees.
 *
 * Source ordering must remain stable.
 *
 * Tooling formatting must not reorder semantic clauses unless the language
 * specification explicitly defines such canonicalization.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Specialization must be tested with:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware intent
 *     AI/model computation
 *     data computation
 *     accelerator computation
 *     distributed computation
 *     networking-related semantic requirements
 *
 * The grammar remains domain-neutral in all cases.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS:
 *
 *     [x] No universal hardware capacity constants.
 *     [x] No fixed resource cardinalities.
 *     [x] No physical device enumeration.
 *     [x] No physical qubit enumeration.
 *     [x] No vendor enumeration.
 *     [x] No fixed topology.
 *     [x] No fixed memory size.
 *     [x] No fixed register width.
 *     [x] No fixed tensor rank.
 *     [x] No fixed node count.
 *     [x] No fixed processor count.
 *     [x] No fixed specialization count.
 *     [x] No fixed argument count.
 *     [x] No embedded Rust.
 *     [x] No unsafe requirement.
 *     [x] No filesystem access.
 *     [x] No network access.
 *     [x] No hardware probing.
 *     [x] No runtime execution.
 *     [x] No second IR.
 *     [x] No second quantum IR.
 *     [x] No second compilation root.
 *
 * ============================================================================
 * SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust
 *     no actions
 *     no semantic predicates
 *     no filesystem access
 *     no network access
 *     no process execution
 *     no hardware access
 *     no runtime execution
 *
 * Generated parser/compiler integration must use:
 *
 *     Rust 2021
 *     Rust 1.97 or later
 *     safe Rust only
 *
 * `unsafe` is not required by this grammar or its intended compiler
 * integration.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * THIS FILE IS COMPLETE WHEN:
 *
 *     [ ] canonical lexer vocabulary is consumed;
 *     [ ] no phantom specialization token is referenced;
 *     [ ] `specialize` is validated contextually under the current lexer;
 *     [ ] Compile is the sole compilation composition root;
 *     [ ] no import cycle exists;
 *     [ ] target remains symbolic;
 *     [ ] resources remain symbolic;
 *     [ ] capabilities remain open-world;
 *     [ ] requirements remain distinct from constraints;
 *     [ ] constraints remain distinct from preferences;
 *     [ ] preferences remain distinct from hints;
 *     [ ] policy remains semantically authoritative;
 *     [ ] fallback remains compilation policy rather than runtime exception
 *         handling;
 *     [ ] generic specialization remains owned by metaprogramming;
 *     [ ] no duplicate type/expression/name grammar exists;
 *     [ ] no AST is defined here;
 *     [ ] no IR is defined here;
 *     [ ] quantum specialization reaches `quantum::ir`;
 *     [ ] no physical hardware selection exists here;
 *     [ ] no artificial resource ceiling exists;
 *     [ ] source order is preserved;
 *     [ ] source spans are preserved by the AST;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] determinism tests exist;
 *     [ ] cross-domain tests exist;
 *     [ ] compatibility tests exist;
 *     [ ] ANTLR generation succeeds;
 *     [ ] generated Rust is compatible with Rust 1.97+;
 *     [ ] repository safe-Rust requirements remain satisfied.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 *     specialization syntax
 *          !=
 *     specialization implementation
 *
 *     specialization syntax
 *          !=
 *     generic resolution
 *
 *     specialization syntax
 *          !=
 *     target selection
 *
 *     specialization syntax
 *          !=
 *     resource allocation
 *
 *     specialization syntax
 *          !=
 *     hardware realization
 *
 *     specialization syntax
 *          !=
 *     quantum physical mapping
 *
 * The stable boundary is:
 *
 *     SOURCE SPECIALIZATION INTENT
 *                  |
 *                  v
 *          DOMAIN-NEUTRAL AST
 *                  |
 *                  v
 *          SEMANTIC SPECIALIZATION
 *                  |
 *                  v
 *        CANONICAL SEMANTIC MODEL
 *                  |
 *          +-------+-------+
 *          |               |
 *          v               v
 *      classical       quantum::ir
 *          |               |
 *          +-------+-------+
 *                  |
 *                  v
 *          target-independent
 *          optimization/lowering
 *                  |
 *                  v
 *          target realization
 *
 * This is the required production boundary for POCO-REAF.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */