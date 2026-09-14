/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hybrid/accelerator-interoperability.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Toolchain/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL ACCELERATOR INTEROPERABILITY.
 *
 * It describes how a Zamani computation may:
 *
 *     - declare an accelerator-facing semantic interface;
 *     - declare a symbolic accelerator binding;
 *     - invoke an accelerator operation;
 *     - exchange logical values across computation domains;
 *     - express accelerator execution intent;
 *     - express requirements, constraints, preferences, capabilities and hints.
 *
 * The accelerator may ultimately be realized by:
 *
 *     - CPU
 *     - multicore CPU
 *     - SIMD/vector unit
 *     - GPU
 *     - FPGA
 *     - ASIC
 *     - DSP
 *     - tensor accelerator
 *     - quantum processor
 *     - quantum simulator
 *     - heterogeneous accelerator
 *     - distributed accelerator
 *     - future accelerator classes
 *
 * This grammar does NOT determine which physical implementation is selected.
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
 *     parser composition
 *          |
 *          +--> Expressions
 *          +--> Types
 *          +--> Statements
 *          +--> Hybrid
 *          +--> Accelerator interoperability <--- THIS FILE
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> capability validation
 *          +--> resource validation
 *          +--> requirement validation
 *          +--> target-independent lowering
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> hardware representation
 *          |
 *          v
 *     optimization
 *          |
 *     routing
 *          |
 *     scheduling
 *          |
 *     resilience / QEC / ZQN where applicable
 *          |
 *     target lowering
 *          |
 *     runtime
 *
 * This grammar NEVER constructs IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - accelerator interoperability declarations;
 *     - accelerator semantic interfaces;
 *     - accelerator interface members;
 *     - symbolic accelerator bindings;
 *     - accelerator invocation syntax;
 *     - accelerator data-transfer intent;
 *     - accelerator execution intent;
 *     - accelerator interoperability clauses;
 *     - accelerator-side semantic contracts.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - expressions;
 *     - expression precedence;
 *     - types;
 *     - statements;
 *     - blocks;
 *     - modules;
 *     - functions;
 *     - quantum gates;
 *     - quantum registers;
 *     - quantum IR;
 *     - classical IR;
 *     - hardware discovery;
 *     - device selection;
 *     - device IDs;
 *     - physical addresses;
 *     - physical topology;
 *     - resource allocation;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - calibration;
 *     - runtime dispatch;
 *     - QEC algorithms;
 *     - ZQN fault/noise semantics;
 *     - resilience decisions.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Source syntax expresses:
 *
 *     WHAT the computation requires
 *     WHAT operation is requested
 *     WHAT values cross a domain boundary
 *     WHAT capabilities are required
 *     WHAT constraints apply
 *     WHAT implementation preferences exist
 *     WHAT optional hints may guide realization
 *
 * Source syntax MUST NOT permanently encode:
 *
 *     a particular device;
 *     a physical device ID;
 *     a fixed accelerator count;
 *     a fixed number of processing elements;
 *     a fixed memory capacity;
 *     a fixed topology;
 *     a fixed bus;
 *     a fixed PCIe/NVLink/etc. path;
 *     a fixed CPU/GPU/QPU;
 *     a fixed qubit count;
 *     a fixed register count;
 *     a fixed deployment location.
 *
 * Therefore:
 *
 *     requires accelerator
 *
 * is semantically different from:
 *
 *     use device X
 *
 * and this grammar does not introduce a `use device X` construct.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No grammar rule contains:
 *
 *     MAX_ACCELERATORS
 *     MAX_DEVICES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_NODES
 *     MAX_OPERATIONS
 *     MAX_TRANSFER_SIZE
 *
 * Repetition is represented structurally with ANTLR repetition operators.
 *
 * Practical parser/resource limits belong to compiler/tooling policy and MUST
 * NOT become Zamani language semantics.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Canonical parser delegates:
 *
 *     Expressions
 *     Types
 *     Statements
 *
 * The eventual parser-composition layer is responsible for making the grammar
 * reachable from the appropriate Zamani source constructs.
 *
 * This file must never import the root parser.
 *
 * The dependency direction is:
 *
 *     lexer
 *       |
 *       +--> Expressions
 *       +--> Types
 *       +--> Statements
 *       |
 *       +--> AcceleratorInteroperability
 *                  |
 *                  v
 *             parser composition
 *
 * There must be no reverse dependency:
 *
 *     AcceleratorInteroperability -> root parser
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes syntax only.
 *
 * Semantic analysis must subsequently determine:
 *
 *     - whether a named accelerator interface exists;
 *     - whether an operation exists;
 *     - whether argument types are compatible;
 *     - whether result types are compatible;
 *     - whether a capability requirement is satisfiable;
 *     - whether a resource requirement is satisfiable;
 *     - whether a constraint is valid;
 *     - whether a preference is merely advisory;
 *     - whether a hint is permitted;
 *     - whether a transfer is semantically legal;
 *     - whether an execution target is callable;
 *     - whether the requested operation is compatible with the selected
 *       target;
 *     - whether lowering preserves source semantics.
 *
 * This file must never perform those decisions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Accelerator interoperability syntax lowers into semantic frontend/IR
 * structures owned elsewhere.
 *
 * For quantum computation:
 *
 *     quantum::ir
 *
 * remains the canonical quantum semantic boundary.
 *
 * This grammar must never define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     QuantumGate
 *     QuantumInstruction
 *     QuantumCircuit
 *     QuantumRegister
 *
 * For classical computation:
 *
 * the canonical classical representation remains authoritative.
 *
 * For hardware:
 *
 * hardware capability/resource/target representations remain authoritative.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no process execution;
 *     - no mutable global state;
 *     - no unsafe Rust;
 *     - no target-specific implementation code.
 *
 * Rust 1.97 / 1.97.1 is a repository implementation/toolchain constraint,
 * not a source-language hardware constraint.
 *
 * ============================================================================
 */

parser grammar AcceleratorInteroperability;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * DELEGATE GRAMMARS
 * ============================================================================
 *
 * These grammars own the language foundations consumed here.
 *
 * Expressions:
 *     canonical expression syntax and precedence.
 *
 * Types:
 *     canonical type syntax.
 *
 * Statements:
 *     canonical statement/block syntax.
 *
 * No expression/type/statement production is duplicated below.
 */
import Expressions, Types, Statements;


/*
 * ============================================================================
 * 1. ROOT
 * ============================================================================
 *
 * This is the accelerator-interoperability composition entry point.
 *
 * The root parser/composition layer decides where this rule is reachable.
 */
acceleratorInteroperability
    : acceleratorInteroperabilityItem*
    ;


/*
 * ============================================================================
 * 2. TOP-LEVEL ITEMS
 * ============================================================================
 */

acceleratorInteroperabilityItem
    : acceleratorInterfaceDeclaration
    | acceleratorBindingDeclaration
    | acceleratorOperationDeclaration
    | acceleratorTransferDeclaration
    | acceleratorExecutionDeclaration
    ;


/*
 * ============================================================================
 * 3. ACCELERATOR INTERFACE
 * ============================================================================
 *
 * An accelerator interface describes a PROGRAM-LEVEL semantic contract.
 *
 * It is not a physical device.
 *
 * Example:
 *
 *     accelerator interface LinearAlgebra {
 *         input  values: Tensor;
 *         output result: Tensor;
 *
 *         operation matmul(
 *             left: Tensor,
 *             right: Tensor
 *         ) -> Tensor;
 *
 *         requires capability("matrix_multiply");
 *     }
 *
 * The actual capability representation is resolved semantically.
 */

acceleratorInterfaceDeclaration
    : 'accelerator'
      'interface'
      qualifiedName
      acceleratorInterfaceBody
    ;

acceleratorInterfaceBody
    : '{'
      acceleratorInterfaceMember*
      '}'
    ;

acceleratorInterfaceMember
    : acceleratorInterfaceInput
    | acceleratorInterfaceOutput
    | acceleratorInterfaceOperation
    | acceleratorInterfaceRequirement
    | acceleratorInterfaceAttribute
    ;


/*
 * ============================================================================
 * 4. INTERFACE INPUTS / OUTPUTS
 * ============================================================================
 */

acceleratorInterfaceInput
    : 'input'
      acceleratorParameterDeclaration
      ';'
    ;

acceleratorInterfaceOutput
    : 'output'
      acceleratorParameterDeclaration
      ';'
    ;

acceleratorParameterDeclaration
    : IDENTIFIER
      (
          ':'
          typeExpr
      )?
    ;


/*
 * ============================================================================
 * 5. INTERFACE OPERATIONS
 * ============================================================================
 *
 * Operations describe semantic callable capabilities.
 *
 * They do not prescribe:
 *
 *     CPU instructions
 *     GPU kernels
 *     FPGA implementation
 *     ASIC implementation
 *     quantum gates
 *     pulse sequences
 *     physical placement
 */

acceleratorInterfaceOperation
    : 'operation'
      IDENTIFIER
      '('
      acceleratorParameterList?
      ')'
      acceleratorReturnType?
      acceleratorOperationClause*
      ';'
    ;

acceleratorParameterList
    : acceleratorParameterDeclaration
      (
          ','
          acceleratorParameterDeclaration
      )*
    ;

acceleratorReturnType
    : '->'
      typeExpr
    ;


/*
 * ============================================================================
 * 6. INTERFACE REQUIREMENTS
 * ============================================================================
 */

acceleratorInterfaceRequirement
    : 'requires'
      expression
      ';'
    ;


/*
 * ============================================================================
 * 7. INTERFACE ATTRIBUTES
 * ============================================================================
 *
 * Attributes are syntactic metadata only.
 *
 * Their interpretation belongs to semantic analysis/tooling.
 */

acceleratorInterfaceAttribute
    : '@'
      IDENTIFIER
      (
          '('
          expressionList?
          ')'
      )?
    ;


/*
 * ============================================================================
 * 8. SYMBOLIC ACCELERATOR BINDING
 * ============================================================================
 *
 * A binding associates a PROGRAM-LEVEL accelerator role/interface with a
 * symbolic implementation target.
 *
 * The target is deliberately a qualified name, not a physical device ID.
 *
 * Example:
 *
 *     bind LinearAlgebra to runtime.linalg
 *         requires capability("matrix_multiply")
 *         prefer capability("tensor")
 *         hint expression;
 *
 * A compiler/runtime may resolve the symbolic target against:
 *
 *     local implementation
 *     remote service
 *     CPU implementation
 *     GPU implementation
 *     FPGA implementation
 *     ASIC implementation
 *     quantum implementation
 *     simulator
 *     future accelerator
 *
 * without changing the program's semantic identity.
 */

acceleratorBindingDeclaration
    : 'bind'
      qualifiedName
      'to'
      acceleratorBindingTarget
      acceleratorBindingClause*
      ';'
    ;

acceleratorBindingTarget
    : qualifiedName
    | expression
    ;

acceleratorBindingClause
    : acceleratorCapabilityClause
    | acceleratorRequirementClause
    | acceleratorConstraintClause
    | acceleratorPreferenceClause
    | acceleratorHintClause
    ;


/*
 * ============================================================================
 * 9. ACCELERATOR OPERATION INVOCATION
 * ============================================================================
 *
 * Invocation is semantic.
 *
 * It does not specify how the operation is physically realized.
 *
 * Example:
 *
 *     accelerate matmul(a, b)
 *         inputs(a, b)
 *         outputs(c);
 *
 * The operation may eventually become:
 *
 *     CPU code
 *     GPU kernel
 *     FPGA pipeline
 *     ASIC operation
 *     quantum operation
 *     distributed execution
 *     simulator execution
 *     future accelerator implementation
 */

acceleratorOperationDeclaration
    : 'accelerate'
      qualifiedName
      '('
      expressionList?
      ')'
      acceleratorOperationClause*
      ';'
    ;

acceleratorOperationClause
    : acceleratorUsingClause
    | acceleratorInputsClause
    | acceleratorOutputsClause
    | acceleratorCapabilityClause
    | acceleratorRequirementClause
    | acceleratorConstraintClause
    | acceleratorPreferenceClause
    | acceleratorHintClause
    ;


/*
 * ============================================================================
 * 10. EXPLICIT IMPLEMENTATION/ROLE REFERENCE
 * ============================================================================
 *
 * `using` remains symbolic.
 *
 * It does not mean:
 *
 *     use physical device N
 *
 * It means:
 *
 *     use this named semantic implementation/role.
 *
 * Resolution is downstream.
 */

acceleratorUsingClause
    : 'using'
      qualifiedName
    ;


/*
 * ============================================================================
 * 11. INPUT / OUTPUT ASSOCIATION
 * ============================================================================
 *
 * These clauses express logical data flow.
 *
 * They do not encode:
 *
 *     DMA
 *     PCIe
 *     NVLink
 *     network topology
 *     memory addresses
 *     physical buffers
 *     physical channels
 */

acceleratorInputsClause
    : 'inputs'
      '('
      expressionList?
      ')'
    ;

acceleratorOutputsClause
    : 'outputs'
      '('
      expressionList?
      ')'
    ;


/*
 * ============================================================================
 * 12. DATA TRANSFER
 * ============================================================================
 *
 * A transfer expresses a semantic domain-boundary value movement.
 *
 * Example:
 *
 *     transfer quantum_result to classical_result
 *         as ClassicalValue;
 *
 * This grammar does not define the physical mechanism.
 */

acceleratorTransferDeclaration
    : 'transfer'
      expression
      'to'
      expression
      acceleratorTransferClause*
      ';'
    ;

acceleratorTransferClause
    : acceleratorAsTypeClause
    | acceleratorUsingClause
    | acceleratorCapabilityClause
    | acceleratorRequirementClause
    | acceleratorConstraintClause
    | acceleratorPreferenceClause
    | acceleratorHintClause
    ;

acceleratorAsTypeClause
    : 'as'
      typeExpr
    ;


/*
 * ============================================================================
 * 13. EXECUTION INTENT
 * ============================================================================
 *
 * Execution remains semantically abstract.
 *
 * The grammar does not prescribe:
 *
 *     synchronous execution
 *     asynchronous execution
 *     local execution
 *     remote execution
 *     queued execution
 *     distributed execution
 *
 * Those are execution/runtime concerns.
 */

acceleratorExecutionDeclaration
    : 'execute'
      acceleratorExecutionTarget
      acceleratorExecutionClause*
      ';'
    ;

acceleratorExecutionTarget
    : qualifiedName
    | expression
    ;

acceleratorExecutionClause
    : acceleratorWithClause
    | acceleratorOnCompletionClause
    | acceleratorCapabilityClause
    | acceleratorRequirementClause
    | acceleratorConstraintClause
    | acceleratorPreferenceClause
    | acceleratorHintClause
    ;


/*
 * ============================================================================
 * 14. EXECUTION ARGUMENTS
 * ============================================================================
 */

acceleratorWithClause
    : 'with'
      '('
      expressionList?
      ')'
    ;


/*
 * ============================================================================
 * 15. COMPLETION HANDLER
 * ============================================================================
 *
 * The handler is ordinary Zamani statement syntax.
 *
 * This prevents accelerator interoperability from creating a second statement
 * language.
 */

acceleratorOnCompletionClause
    : 'on'
      'completion'
      block
    ;


/*
 * ============================================================================
 * 16. CAPABILITY
 * ============================================================================
 *
 * A capability describes what an implementation can do.
 *
 * It is NOT a resource quantity.
 *
 * It is NOT a device ID.
 *
 * It is NOT a placement.
 *
 * It is NOT a scheduling decision.
 */

acceleratorCapabilityClause
    : 'requires'
      'capability'
      '('
      expression
      ')'
    ;


/*
 * ============================================================================
 * 17. REQUIREMENT
 * ============================================================================
 *
 * Requirements describe semantic conditions that must be satisfied.
 *
 * Requirements must remain distinct from preferences and hints.
 */

acceleratorRequirementClause
    : 'requires'
      expression
    ;


/*
 * ============================================================================
 * 18. CONSTRAINT
 * ============================================================================
 *
 * Constraints restrict valid realizations.
 *
 * They do not select the realization.
 */

acceleratorConstraintClause
    : 'constrained'
      'by'
      expression
    ;


/*
 * ============================================================================
 * 19. PREFERENCE
 * ============================================================================
 *
 * Preferences are advisory unless semantic analysis explicitly defines a
 * stronger contract for a particular language construct.
 *
 * A preference MUST NOT silently become a semantic requirement.
 */

acceleratorPreferenceClause
    : 'prefer'
      expression
    ;


/*
 * ============================================================================
 * 20. HINT
 * ============================================================================
 *
 * Hints are non-authoritative implementation guidance.
 *
 * Hints must never be used to smuggle a target-specific semantic requirement
 * into portable source code.
 */

acceleratorHintClause
    : 'hint'
      expression
    ;


/*
 * ============================================================================
 * 21. SHARED CLAUSE TERMINATION
 * ============================================================================
 *
 * Every clause above deliberately excludes the semicolon.
 *
 * This permits:
 *
 *     clause clause clause ;
 *
 * while retaining one unambiguous declaration terminator.
 *
 * No optional semicolon is accepted here because production syntax should have
 * one deterministic declaration boundary.
 */


/*
 * ============================================================================
 * 22. CANONICAL NAME USE
 * ============================================================================
 *
 * Qualified names are consumed through the canonical expression grammar where
 * possible.
 *
 * This local rule is restricted to symbolic accelerator references and does
 * not redefine the general expression/name model.
 */

qualifiedName
    : IDENTIFIER
      (
          '::'
          IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * 23. EXPRESSION LIST ADAPTER
 * ============================================================================
 *
 * This is a composition adapter only.
 *
 * The individual expression remains owned by Expressions.
 *
 * No expression grammar is duplicated here.
 */

expressionList
    : expression
      (
          ','
          expression
      )*
    ;


/*
 * ============================================================================
 * 24. COMPLETION CONTRACT
 * ============================================================================
 *
 * This grammar is complete when:
 *
 *     1. ZamaniLexer contains the accelerator-interoperability keywords used
 *        above, or the canonical lexer represents them through an equivalent
 *        validated token strategy.
 *
 *     2. Expressions exports:
 *
 *            expression
 *
 *        and the parser composition layer exposes it to this delegate.
 *
 *     3. Types exports:
 *
 *            typeExpr
 *
 *     4. Statements exports:
 *
 *            block
 *
 *     5. `qualifiedName` is reconciled with the repository's canonical
 *        name-resolution grammar.
 *
 *     6. This grammar is imported exactly once by the hybrid/parser
 *        composition layer.
 *
 *     7. No root parser is imported by this grammar.
 *
 *     8. Semantic analysis converts these nodes into the repository's
 *        canonical semantic representation.
 *
 *     9. Quantum operations eventually lower through `quantum::ir` rather
 *        than through an accelerator-specific quantum representation.
 *
 *    10. Hardware selection, routing, scheduling, resource allocation and
 *        runtime dispatch remain downstream.
 *
 * ============================================================================
 */