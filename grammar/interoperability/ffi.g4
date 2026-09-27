/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/ffi.g4
 *
 * Grammar:
 *     Ffi
 *
 * Status:
 *     CANONICAL FFI BOUNDARY GRAMMAR DELEGATE
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     No unsafe Rust.
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No filesystem access.
 *     No network access.
 *     No process execution.
 *     No runtime execution.
 *     No hardware discovery.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL FFI BOUNDARY CONTRACT.
 *
 * FFI allows Zamani source code to describe an interoperability boundary
 * without making a foreign programming language, ABI, operating system,
 * processor, device, runtime, library implementation, or hardware topology
 * part of Zamani's permanent semantic core.
 *
 * The central rule is:
 *
 *     FFI syntax describes WHAT crosses a boundary and WHAT guarantees
 *     the boundary requires.
 *
 * It does not prescribe HOW a target realizes that boundary.
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
 *     authoritative parser
 *          |
 *          v
 *     FFI syntax
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *     +----+---------+---------+---------+
 *     |              |         |         |
 *     v              v         v         v
 *   types           ABI     effects   capabilities
 *     |              |         |         |
 *     +--------------+---------+---------+
 *                    |
 *                    v
 *             canonical semantic model
 *                    |
 *                    v
 *              canonical IR boundary
 *                    |
 *          +---------+----------+
 *          |         |          |
 *          v         v          v
 *      classical quantum::ir hardware/HDL
 *          |         |          |
 *          +---------+----------+
 *                    |
 *                    v
 *          optimization / lowering
 *                    |
 *                    v
 *             ABI realization
 *                    |
 *                    v
 *          linker / runtime / target
 *
 * FFI MUST NOT reverse this dependency direction.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file owns only FFI-specific boundary syntax.
 *
 * It does NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     stringLiteral
 *     integerLiteral
 *     expression
 *     argumentList
 *     parameter
 *     parameterList
 *     typeExpr
 *     attribute
 *     annotation
 *     genericParameterClause
 *     genericParameter
 *
 * Those constructs belong to their canonical grammar owners.
 *
 * ABI syntax belongs to:
 *
 *     grammar/interoperability/abi.g4
 *
 * General external declarations belong to:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * Interoperability composition belongs to the interoperability composition
 * grammar.
 *
 * Ordinary functions remain owned by grammar/functions/.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Foreign interfaces MUST remain portable.
 *
 * FFI syntax MUST NOT permanently encode:
 *
 *     CPU model
 *     GPU model
 *     FPGA model
 *     ASIC model
 *     QPU model
 *     device identifier
 *     physical address
 *     register identifier
 *     register count
 *     pointer width
 *     word width
 *     node count
 *     core count
 *     thread count
 *     qubit count
 *     memory capacity
 *     machine topology
 *     deployment location
 *
 * A portable FFI declaration describes:
 *
 *     symbolic identity
 *     callable signature
 *     type boundary
 *     conversion policy
 *     ownership policy
 *     lifetime policy
 *     nullability
 *     effects
 *     capabilities
 *     resource requirements
 *     compatibility requirements
 *     security requirements
 *
 * Concrete realization is downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar deliberately imposes no finite source-level limit on:
 *
 *     FFI declarations
 *     interfaces
 *     functions
 *     parameters
 *     arguments
 *     callbacks
 *     contracts
 *     attributes
 *     requirements
 *     effects
 *     resources
 *     adapters
 *     interfaces
 *     targets
 *     devices
 *     nodes
 *     threads
 *     cores
 *     qubits
 *     memory
 *
 * Repetition is represented by ANTLR repetition operators.
 *
 * No language-level MAX_* constants belong here.
 *
 * ============================================================================
 * SECURITY / INERTNESS
 * ============================================================================
 *
 * Parsing FFI syntax MUST NEVER:
 *
 *     load a library;
 *     open a file;
 *     access a network;
 *     resolve a foreign symbol;
 *     invoke a process;
 *     execute foreign code;
 *     inspect hardware;
 *     dereference an address;
 *     allocate native memory;
 *     create a runtime handle.
 *
 * All such behavior belongs downstream to explicitly authorized compiler,
 * linker, runtime, deployment, or security components.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * FFI can describe boundaries involving:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware
 *     accelerators
 *     AI
 *     distributed computation
 *     networking
 *     system interfaces
 *     future computational domains
 *
 * This grammar MUST NOT create separate FFI languages for those domains.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * An FFI boundary may expose a quantum or hybrid semantic type:
 *
 *     ffi fn submit(program: QuantumProgram) -> Result;
 *
 * The FFI grammar does NOT define:
 *
 *     QubitId
 *     physical qubits
 *     gates
 *     topology
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *
 * Quantum semantics remain downstream and use the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * FFI may expose abstract hardware/HDL interfaces through types, capabilities,
 * resources, and symbolic interface identities.
 *
 * It MUST NOT encode:
 *
 *     fixed pin numbers
 *     fixed register addresses
 *     fixed bus widths
 *     fixed FPGA capacities
 *     fixed ASIC capacities
 *     fixed accelerator counts
 *
 * ============================================================================
 * ABI INTEGRATION
 * ============================================================================
 *
 * This grammar may reference ABI identities symbolically.
 *
 * It does NOT implement ABI rules.
 *
 * ABI layout, alignment, calling sequence, register assignment, stack layout,
 * binary encoding, and target compatibility belong to ABI semantic analysis
 * and lowering.
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * The importing/composition grammar MUST make these canonical rules available:
 *
 *     identifier
 *     qualifiedName
 *     stringLiteral
 *     expression
 *     typeExpr
 *     argumentList
 *     parameterList
 *     parameter
 *     attribute
 *
 * The composition grammar may provide these through imported delegates.
 *
 * This file MUST NOT duplicate them.
 *
 * ============================================================================
 */

parser grammar Ffi;


/*
 * ============================================================================
 * FFI ITEM
 * ============================================================================
 *
 * This is the reusable integration entry point.
 *
 * The composition grammar decides which FFI constructs are legal in a
 * particular source position.
 *
 * It MUST NOT be used as the compilation-unit root.
 *
 * ============================================================================
 */

ffiItem
    : ffiInterfaceDeclaration
    | ffiBindingDeclaration
    | ffiCallbackDeclaration
    | ffiAdapterDeclaration
    | ffiPolicyDeclaration
    ;


/*
 * ============================================================================
 * FFI INTERFACE
 * ============================================================================
 *
 * Example:
 *
 *     ffi interface math {
 *         ...
 *     }
 *
 * The interface identifier is semantic identity.
 *
 * It is not:
 *
 *     a library filename
 *     a filesystem path
 *     a device identifier
 *     a network address
 *     an executable
 *
 * ============================================================================
 */

ffiInterfaceDeclaration
    : attribute*
      'ffi'
      'interface'
      identifier
      ffiInterfaceParameterClause?
      ffiInterfaceBody
    ;


ffiInterfaceParameterClause
    : '<'
      ffiInterfaceParameter
      (',' ffiInterfaceParameter)*
      '>'
    ;


ffiInterfaceParameter
    : identifier
      (':' qualifiedName)?
    ;


ffiInterfaceBody
    : '{'
      ffiInterfaceMember*
      '}'
    ;


ffiInterfaceMember
    : attribute* ffiBindingDeclaration
    | attribute* ffiCallbackDeclaration
    | attribute* ffiAdapterDeclaration
    | attribute* ffiPolicyDeclaration
    ;


/*
 * ============================================================================
 * FFI BINDING
 * ============================================================================
 *
 * Binds a Zamani-visible callable contract to a symbolic foreign identity.
 *
 * Example:
 *
 *     ffi fn sin(x: Real) -> Real;
 *
 *     ffi fn sin(x: Real) -> Real
 *         using foreign::math::sin;
 *
 * The declaration does not resolve or execute the target.
 *
 * ============================================================================
 */

ffiBindingDeclaration
    : attribute*
      'ffi'
      'fn'
      identifier
      ffiGenericParameterClause?
      '(' parameterList? ')'
      ffiReturnClause?
      ffiContractClause?
      ffiBindingTargetClause?
      ';'
    ;


ffiGenericParameterClause
    : '<'
      ffiGenericParameter
      (',' ffiGenericParameter)*
      '>'
    ;


ffiGenericParameter
    : identifier
      (':' qualifiedName)*
    ;


ffiReturnClause
    : '->'
      typeExpr
    ;


ffiBindingTargetClause
    : 'using'
      ffiForeignTarget
    ;


ffiForeignTarget
    : qualifiedName
    | stringLiteral
    | stringLiteral
      '::'
      qualifiedName
    ;


/*
 * ============================================================================
 * FFI CONTRACT
 * ============================================================================
 *
 * This is the principal boundary contract.
 *
 * All optional FFI semantics are grouped here so the callable declaration
 * remains deterministic and structurally clear.
 *
 * ============================================================================
 */

ffiContractClause
    : 'with'
      '{'
      ffiContractItem*
      '}'
    ;


ffiContractItem
    : ffiBoundaryClause
    | ffiEffectClause
    | ffiRequirementClause
    | ffiErrorClause
    | ffiCompatibilityClause
    | ffiSecurityClause
    | ffiResourceClause
    | ffiExecutionClause
    | ffiConcurrencyClause
    | ffiDeterminismClause
    | ffiVersionClause
    | ffiAttributeClause
    ;


/*
 * ============================================================================
 * BOUNDARY SEMANTICS
 * ============================================================================
 *
 * Boundary clauses describe semantic behavior at the FFI crossing.
 *
 * ============================================================================
 */

ffiBoundaryClause
    : ffiMarshalClause
    | ffiOwnershipClause
    | ffiBorrowClause
    | ffiLifetimeClause
    | ffiNullabilityClause
    | ffiRepresentationClause
    | ffiEncodingClause
    | ffiSizeClause
    | ffiAlignmentClause
    | ffiDirectionClause
    | ffiPinningClause
    | ffiBlockingClause
    | ffiAsyncClause
    | ffiStreamingClause
    | ffiCallbackLifetimeClause
    ;


/*
 * ============================================================================
 * MARSHALLING
 * ============================================================================
 *
 * Describes semantic conversion/marshalling intent.
 *
 * The implementation of the conversion is downstream.
 *
 * ============================================================================
 */

ffiMarshalClause
    : 'marshal'
      ffiValueSpec
    ;


ffiMarshalPairClause
    : 'marshal'
      ffiValueSpec
      'as'
      ffiValueSpec
    ;


/*
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 */

ffiOwnershipClause
    : 'ownership'
      '='
      ffiOwnershipMode
    ;


ffiOwnershipMode
    : 'borrowed'
    | 'owned'
    | 'shared'
    | 'transferred'
    | 'retained'
    | 'returned'
    | ffiSymbolicValue
    ;


ffiOwnershipTransferClause
    : 'ownership'
      'transfer'
      ffiOwnershipDirection
    ;


ffiOwnershipDirection
    : 'in'
    | 'out'
    | 'inout'
    | 'return'
    | 'borrow'
    | 'retain'
    | 'release'
    ;


/*
 * ============================================================================
 * BORROWING
 * ============================================================================
 */

ffiBorrowClause
    : 'borrow'
      ffiValueSpec
    ;


/*
 * ============================================================================
 * LIFETIME
 * ============================================================================
 */

ffiLifetimeClause
    : 'lifetime'
      ffiValueSpec
    ;


ffiCallbackLifetimeClause
    : 'callback'
      'lifetime'
      ffiValueSpec
    ;


/*
 * ============================================================================
 * NULLABILITY
 * ============================================================================
 */

ffiNullabilityClause
    : 'nullability'
      '='
      ffiNullabilityMode
    ;


ffiNullabilityMode
    : 'nullable'
    | 'nonnull'
    | 'unknown'
    | ffiSymbolicValue
    ;


/*
 * ============================================================================
 * REPRESENTATION
 * ============================================================================
 */

ffiRepresentationClause
    : 'representation'
      '='
      ffiValueSpec
    ;


/*
 * ============================================================================
 * ENCODING
 * ============================================================================
 */

ffiEncodingClause
    : 'encoding'
      '='
      ffiValueSpec
    ;


/*
 * ============================================================================
 * SIZE
 * ============================================================================
 *
 * A size expression is a semantic expression.
 *
 * It does NOT establish a universal maximum or machine representation.
 *
 * ============================================================================
 */

ffiSizeClause
    : 'size'
      '='
      expression
    ;


/*
 * ============================================================================
 * ALIGNMENT
 * ============================================================================
 *
 * Alignment is semantic interoperability metadata.
 *
 * Target-specific realization belongs to ABI lowering.
 * ============================================================================
 */

ffiAlignmentClause
    : 'alignment'
      '='
      expression
    ;


/*
 * ============================================================================
 * DATA DIRECTION
 * ============================================================================
 */

ffiDirectionClause
    : 'direction'
      '='
      ffiDirection
    ;


ffiDirection
    : 'in'
    | 'out'
    | 'inout'
    ;


/*
 * ============================================================================
 * PINNING
 * ============================================================================
 *
 * Pinning is symbolic semantic intent.
 *
 * It does not mean a concrete physical memory address.
 * ============================================================================
 */

ffiPinningClause
    : 'pinning'
      '='
      ffiValueSpec
    ;


/*
 * ============================================================================
 * BLOCKING
 * ============================================================================
 */

ffiBlockingClause
    : 'blocking'
      '='
      ffiBlockingMode
    ;


ffiBlockingMode
    : 'blocking'
    | 'nonblocking'
    | 'unknown'
    | ffiSymbolicValue
    ;


/*
 * ============================================================================
 * ASYNCHRONOUS EXECUTION
 * ============================================================================
 */

ffiAsyncClause
    : 'async'
      ffiValueSpec?
    ;


/*
 * ============================================================================
 * STREAMING
 * ============================================================================
 */

ffiStreamingClause
    : 'streaming'
      ffiValueSpec?
    ;


/*
 * ============================================================================
 * EFFECTS
 * ============================================================================
 *
 * Effect names are symbolic.
 *
 * The canonical effect system remains the authority for effect semantics.
 *
 * ============================================================================
 */

ffiEffectClause
    : 'effects'
      '{'
      ffiEffectReferenceList?
      '}'
    ;


ffiEffectReferenceList
    : qualifiedName
      (',' qualifiedName)*
    ;


/*
 * ============================================================================
 * REQUIREMENTS / CAPABILITIES
 * ============================================================================
 *
 * Requirements are semantic predicates.
 *
 * They are deliberately not target-selection statements.
 *
 * Examples:
 *
 *     requires { capability::ffi; }
 *     requires { capability::network; }
 *     requires { capability::quantum; }
 *
 * ============================================================================
 */

ffiRequirementClause
    : 'requires'
      '{'
      ffiRequirement*
      '}'
    ;


ffiRequirement
    : ffiRequirementName
      ('=' expression)?
      ';'?
    ;


ffiRequirementName
    : qualifiedName
    | ffiSymbolicValue
    ;


/*
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Requirement, preference, prohibition, and hint remain distinct concepts.
 *
 * ============================================================================
 */

ffiResourceClause
    : 'resources'
      '{'
      ffiResourceItem*
      '}'
    ;


ffiResourceItem
    : 'requires'
      '='
      expression
      ';'
    | 'prefers'
      '='
      expression
      ';'
    | 'forbids'
      '='
      expression
      ';'
    | 'hints'
      '='
      expression
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Foreign errors must have an explicit semantic boundary.
 *
 * ============================================================================
 */

ffiErrorClause
    : 'errors'
      '{'
      ffiErrorItem*
      '}'
    ;


ffiErrorItem
    : 'mode'
      '='
      ffiErrorMode
      ';'?
    | 'type'
      '='
      typeExpr
      ';'?
    | 'map'
      qualifiedName
      'to'
      typeExpr
      ';'?
    | 'exception'
      '='
      ffiExceptionPolicy
      ';'?
    | 'attribute'
      identifier
      '='
      expression
      ';'?
    ;


ffiErrorMode
    : 'result'
    | 'exception'
    | 'status'
    | 'sentinel'
    | 'panic'
    | 'abort'
    | 'custom'
    | ffiSymbolicValue
    ;


ffiExceptionPolicy
    : 'translate'
    | 'propagate'
    | 'catch'
    | 'forbid'
    | ffiSymbolicValue
    ;


/*
 * ============================================================================
 * CALLBACKS
 * ============================================================================
 *
 * Callback declarations are contracts for foreign code calling back into
 * Zamani.
 *
 * They do not define native function-pointer representation.
 *
 * ============================================================================
 */

ffiCallbackDeclaration
    : attribute*
      'ffi'
      'callback'
      identifier
      ffiGenericParameterClause?
      '(' parameterList? ')'
      ffiReturnClause?
      ffiContractClause?
      ';'
    ;


/*
 * ============================================================================
 * CALLBACK REFERENCE
 * ============================================================================
 *
 * A callback reference is a symbolic callable boundary.
 * ============================================================================
 */

ffiCallbackReference
    : 'callback'
      qualifiedName
    ;


/*
 * ============================================================================
 * CALLBACK INVOCATION
 * ============================================================================
 *
 * Invocation remains declarative at parse time.
 *
 * Runtime invocation occurs only after semantic resolution and lowering.
 * ============================================================================
 */

ffiCallbackCallExpression
    : 'ffi'
      'callback'
      qualifiedName
      '('
      argumentList?
      ')'
    ;


/*
 * ============================================================================
 * FOREIGN CALL
 * ============================================================================
 *
 * Explicit FFI invocation.
 *
 * This rule does not perform lookup or execution.
 * ============================================================================
 */

ffiCallExpression
    : 'ffi'
      'call'
      ffiForeignCallTarget
      '('
      argumentList?
      ')'
    ;


ffiForeignCallTarget
    : qualifiedName
    | stringLiteral
      '::'
      qualifiedName
    | stringLiteral
    ;


ffiCallStatement
    : ffiCallExpression
      ';'
    ;


ffiCallbackCallStatement
    : ffiCallbackCallExpression
      ';'
    ;


/*
 * ============================================================================
 * FOREIGN CALLABLE REFERENCE
 * ============================================================================
 */

ffiCallableReferenceExpression
    : 'ffi'
      'ref'
      ffiForeignTarget
    ;


/*
 * ============================================================================
 * ADAPTER
 * ============================================================================
 *
 * An adapter describes semantic adaptation between two interface contracts.
 *
 * It does not specify whether the implementation is:
 *
 *     generated code
 *     wrapper code
 *     marshaling
 *     serialization
 *     runtime dispatch
 *     compiler lowering
 *     hardware bridge
 *     quantum/classical bridge
 *
 * ============================================================================
 */

ffiAdapterDeclaration
    : attribute*
      'ffi'
      'adapter'
      identifier
      ffiAdapterTargetClause?
      ffiAdapterBody
    ;


ffiAdapterTargetClause
    : 'from'
      qualifiedName
      'to'
      qualifiedName
    ;


ffiAdapterBody
    : '{'
      ffiAdapterItem*
      '}'
    ;


ffiAdapterItem
    : 'requires'
      '='
      expression
      ';'
    | 'using'
      '='
      qualifiedName
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * LINKAGE
 * ============================================================================
 *
 * Linkage is deliberately represented as part of an FFI contract rather than
 * as an instruction to load or execute anything.
 *
 * The actual ABI/linker grammar owns detailed ABI declarations.
 *
 * ============================================================================
 */

ffiLinkClause
    : 'link'
      ffiLinkItem*
      ';'?
    ;


ffiLinkItem
    : 'name'
      '='
      stringLiteral
    | 'kind'
      '='
      ffiSymbolicValue
    | 'version'
      '='
      expression
    | 'interface'
      '='
      qualifiedName
    | 'attribute'
      identifier
      '='
      expression
    ;


/*
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 */

ffiCompatibilityClause
    : 'compatible'
      'with'
      ffiCompatibilityTarget
      ffiCompatibilityBody?
    ;


ffiCompatibilityTarget
    : qualifiedName
    | stringLiteral
    ;


ffiCompatibilityBody
    : '{'
      ffiCompatibilityItem*
      '}'
    ;


ffiCompatibilityItem
    : 'version'
      '='
      expression
      ';'
    | 'feature'
      '='
      expression
      ';'
    | 'requires'
      '='
      expression
      ';'
    | 'forbid'
      '='
      expression
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Security metadata remains symbolic and declarative.
 * ============================================================================
 */

ffiSecurityClause
    : 'security'
      '{'
      ffiSecurityItem*
      '}'
    ;


ffiSecurityItem
    : 'capability'
      '='
      qualifiedName
      ';'
    | 'trust'
      '='
      ffiSymbolicValue
      ';'
    | 'sandbox'
      '='
      ffiSymbolicValue
      ';'
    | 'isolation'
      '='
      ffiSymbolicValue
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * EXECUTION CONTRACT
 * ============================================================================
 *
 * Execution mode is symbolic.
 *
 * The grammar does not enumerate CPU/GPU/FPGA/QPU/etc. as a closed universe.
 * ============================================================================
 */

ffiExecutionClause
    : 'execution'
      '{'
      ffiExecutionItem*
      '}'
    ;


ffiExecutionItem
    : 'mode'
      '='
      ffiSymbolicValue
      ';'
    | 'placement'
      '='
      expression
      ';'
    | 'availability'
      '='
      expression
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * CONCURRENCY / REENTRANCY
 * ============================================================================
 */

ffiConcurrencyClause
    : 'concurrency'
      '{'
      ffiConcurrencyItem*
      '}'
    ;


ffiConcurrencyItem
    : 'reentrancy'
      '='
      ffiSymbolicValue
      ';'
    | 'thread_safety'
      '='
      ffiSymbolicValue
      ';'
    | 'serialization'
      '='
      ffiSymbolicValue
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * A declaration is a contract/claim for semantic validation.
 *
 * The grammar does not independently establish that a foreign implementation
 * is deterministic.
 * ============================================================================
 */

ffiDeterminismClause
    : 'determinism'
      '='
      ffiSymbolicValue
    ;


/*
 * ============================================================================
 * VERSION
 * ============================================================================
 */

ffiVersionClause
    : 'version'
      '='
      expression
    ;


/*
 * ============================================================================
 * GENERIC FFI POLICY
 * ============================================================================
 *
 * Policies are declarative project/source contracts.
 *
 * They do not execute during parsing.
 * ============================================================================
 */

ffiPolicyDeclaration
    : attribute*
      'ffi'
      'policy'
      identifier
      ffiPolicyBody
    ;


ffiPolicyBody
    : '{'
      ffiPolicyItem*
      '}'
    ;


ffiPolicyItem
    : 'requires'
      '='
      expression
      ';'
    | 'forbid'
      '='
      expression
      ';'
    | 'prefer'
      '='
      expression
      ';'
    | 'allow'
      '='
      expression
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * ATTRIBUTES
 * ============================================================================
 *
 * FFI-specific attributes remain deliberately open-ended.
 *
 * The canonical language attribute grammar remains authoritative for general
 * attributes. This local form is used only inside the explicitly-owned FFI
 * contract structures.
 * ============================================================================
 */

ffiAttributeClause
    : 'attributes'
      '{'
      ffiAttributeItem*
      '}'
    ;


ffiAttributeItem
    : identifier
      ('=' expression)?
      ';'?
    ;


/*
 * ============================================================================
 * PARAMETER / RESULT BOUNDARY METADATA
 * ============================================================================
 *
 * These are standalone reusable FFI contracts for tooling and semantic
 * normalization.
 *
 * They do not redefine the canonical parameter or result syntax.
 * ============================================================================
 */

ffiParameterBoundary
    : 'parameter'
      identifier
      ffiBoundaryClause*
      ffiAttributeClause?
    ;


ffiResultBoundary
    : 'result'
      ffiBoundaryClause*
      ffiAttributeClause?
    ;


/*
 * ============================================================================
 * BUFFER CONTRACT
 * ============================================================================
 *
 * Buffer semantics remain abstract.
 *
 * There is intentionally no fixed:
 *
 *     pointer width
 *     address
 *     buffer capacity
 *     register width
 *     memory-bank identity
 *
 * ============================================================================
 */

ffiBufferBoundary
    : 'buffer'
      identifier?
      ffiBufferBody
    ;


ffiBufferBody
    : '{'
      ffiBufferItem*
      '}'
    ;


ffiBufferItem
    : 'direction'
      '='
      ffiDirection
      ';'
    | 'ownership'
      '='
      ffiSymbolicValue
      ';'
    | 'lifetime'
      '='
      ffiValueSpec
      ';'
    | 'size'
      '='
      expression
      ';'
    | 'alignment'
      '='
      expression
      ';'
    | 'representation'
      '='
      ffiSymbolicValue
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * FUTURE / ASYNC RESULT CONTRACT
 * ============================================================================
 *
 * No particular runtime future/promise type is assumed.
 * ============================================================================
 */

ffiFutureBoundary
    : 'future'
      ffiFutureBody?
    ;


ffiFutureBody
    : '{'
      ffiFutureItem*
      '}'
    ;


ffiFutureItem
    : 'result'
      '='
      typeExpr
      ';'
    | 'cancel'
      '='
      expression
      ';'
    | 'completion'
      '='
      expression
      ';'
    | 'error'
      '='
      typeExpr
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * STREAM CONTRACT
 * ============================================================================
 *
 * The transport remains a downstream semantic/runtime decision.
 * ============================================================================
 */

ffiStreamBoundary
    : 'stream'
      ffiStreamBody?
    ;


ffiStreamBody
    : '{'
      ffiStreamItem*
      '}'
    ;


ffiStreamItem
    : 'item'
      '='
      typeExpr
      ';'
    | 'error'
      '='
      typeExpr
      ';'
    | 'close'
      '='
      expression
      ';'
    | 'backpressure'
      '='
      expression
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Provenance is declarative metadata.
 * ============================================================================
 */

ffiProvenanceClause
    : 'provenance'
      '{'
      ffiProvenanceItem*
      '}'
    ;


ffiProvenanceItem
    : 'source'
      '='
      expression
      ';'
    | 'version'
      '='
      expression
      ';'
    | 'hash'
      '='
      expression
      ';'
    | 'identity'
      '='
      expression
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * ADAPTATION CONTRACT
 * ============================================================================
 */

ffiAdaptationClause
    : 'adapt'
      '{'
      ffiAdaptationItem*
      '}'
    ;


ffiAdaptationItem
    : 'from'
      '='
      qualifiedName
      ';'
    | 'to'
      '='
      qualifiedName
      ';'
    | 'using'
      '='
      qualifiedName
      ';'
    | 'requires'
      '='
      expression
      ';'
    | 'attribute'
      identifier
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * SYMBOLIC VALUES
 * ============================================================================
 *
 * Open symbolic values are essential for POCO-REAF.
 *
 * They permit future:
 *
 *     languages
 *     ABIs
 *     calling conventions
 *     runtimes
 *     representations
 *     execution models
 *     hardware domains
 *     interoperability systems
 *
 * without requiring the grammar to enumerate every future possibility.
 *
 * ============================================================================
 */

ffiSymbolicValue
    : qualifiedName
    | stringLiteral
    | identifier
    ;


ffiValueSpec
    : ffiSymbolicValue
    | '(' expression ')'
    ;


/*
 * ============================================================================
 * INTEGRATION ALIASES
 * ============================================================================
 *
 * These aliases provide stable composition names while keeping the actual
 * syntax owned by the rules above.
 *
 * The aliases intentionally do not duplicate grammar.
 * ============================================================================
 */

ffiQualifiedCallExpression
    : ffiCallExpression
    ;


ffiQualifiedCallbackCallExpression
    : ffiCallbackCallExpression
    ;


ffiForeignDeclaration
    : ffiInterfaceDeclaration
    | ffiBindingDeclaration
    | ffiCallbackDeclaration
    | ffiAdapterDeclaration
    | ffiPolicyDeclaration
    ;


/*
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * The interoperability composition grammar should expose only these public
 * integration entry points:
 *
 *     ffiItem
 *     ffiForeignDeclaration
 *     ffiInterfaceDeclaration
 *     ffiBindingDeclaration
 *     ffiCallbackDeclaration
 *     ffiAdapterDeclaration
 *     ffiPolicyDeclaration
 *     ffiCallExpression
 *     ffiCallbackCallExpression
 *     ffiCallableReferenceExpression
 *     ffiCallStatement
 *     ffiCallbackCallStatement
 *
 * Internal helpers remain implementation details of this delegate grammar.
 *
 * The composition grammar must NOT create a second FFI grammar containing
 * overlapping definitions.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must represent FFI semantically, not as backend objects.
 *
 * Minimum semantic concepts:
 *
 *     FfiInterface
 *     FfiBinding
 *     FfiCallback
 *     FfiAdapter
 *     FfiCall
 *     FfiCallableReference
 *     FfiBoundaryContract
 *     FfiOwnership
 *     FfiLifetime
 *     FfiNullability
 *     FfiMarshalling
 *     FfiEffectRequirement
 *     FfiCapabilityRequirement
 *     FfiResourceRequirement
 *     FfiErrorBoundary
 *     FfiCompatibility
 *     FfiSecurityPolicy
 *
 * Every node must retain source spans.
 *
 * The AST MUST NOT contain:
 *
 *     native library handles
 *     function pointers
 *     device handles
 *     physical addresses
 *     linker state
 *     runtime state
 *     hardware IDs
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     name resolution
 *     foreign symbol resolution
 *     type compatibility
 *     ABI compatibility
 *     calling-convention compatibility
 *     representation compatibility
 *     ownership validation
 *     lifetime validation
 *     nullability validation
 *     conversion validation
 *     effect validation
 *     capability validation
 *     resource validation
 *     security validation
 *     error-boundary validation
 *     compatibility validation
 *     target availability
 *
 * Unknown symbolic values may remain syntactically valid and are rejected only
 * when semantic policy requires them to be known.
 *
 * ============================================================================
 * ABI INTEGRATION
 * ============================================================================
 *
 * FFI may refer to ABI contracts symbolically.
 *
 * Correct dependency:
 *
 *     FFI
 *       |
 *       v
 *      ABI
 *       |
 *       v
 *   semantic/lowering
 *
 * ABI MUST NOT depend on FFI grammar to define its own fundamental syntax.
 *
 * The ABI grammar remains the authority for:
 *
 *     ABI identity
 *     calling convention
 *     linkage contract
 *     ABI compatibility
 *     ABI features
 *
 * ============================================================================
 * FOREIGN-FUNCTION INTEGRATION
 * ============================================================================
 *
 * `foreign-functions.g4` remains the authority for the general `extern`
 * declaration family.
 *
 * This file MUST NOT silently replace that family.
 *
 * If the language accepts both:
 *
 *     extern fn ...
 *
 * and:
 *
 *     ffi fn ...
 *
 * they must have clearly defined semantic distinctions:
 *
 *     extern
 *         -> general external declaration
 *
 *     ffi
 *         -> explicit FFI boundary contract
 *
 * The semantic layer must normalize both into the same canonical
 * interoperability model where their semantics overlap.
 *
 * ============================================================================
 * FUNCTION INTEGRATION
 * ============================================================================
 *
 * FFI declarations MUST reuse canonical parameter and type syntax.
 *
 * They must NOT define:
 *
 *     a second parameter grammar
 *     a second type grammar
 *     a second generic grammar
 *
 * This file therefore uses:
 *
 *     parameterList
 *     typeExpr
 *
 * from the canonical function/type grammar.
 *
 * `ffiGenericParameterClause` is intentionally FFI-specific because the
 * repository already reserves generic-parameter ownership for the canonical
 * function/type grammar and this delegate must not collide with it.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * FFI calls consume canonical:
 *
 *     expression
 *     argumentList
 *
 * No separate FFI expression language is introduced.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * FFI may carry quantum types and quantum capability requirements.
 *
 * It MUST NOT define quantum operations.
 *
 * Example semantic flow:
 *
 *     ffi call
 *         |
 *         v
 *     semantic boundary
 *         |
 *         v
 *     quantum semantic model
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     routing / scheduling / QEC / ZQN / HAL
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-facing FFI remains symbolic.
 *
 * Hardware-specific realization belongs to:
 *
 *     hardware/
 *     hdl/
 *     resources/
 *     compile/
 *     execution/
 *
 * This file must not duplicate their target/resource models.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORK INTEGRATION
 * ============================================================================
 *
 * A foreign boundary may represent a distributed or networked interface.
 *
 * That does NOT turn an FFI declaration into a network implementation.
 *
 * Network semantics remain owned by networking/distributed domains.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Foreign code is an explicit trust boundary.
 *
 * Semantic validation should be capable of rejecting:
 *
 *     missing capability
 *     forbidden effect
 *     incompatible ownership
 *     invalid lifetime
 *     unsafe conversion
 *     incompatible ABI
 *     unsupported exception crossing
 *     unsupported execution mode
 *     prohibited resource access
 *
 * Parsing itself remains inert.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this grammar is deterministic.
 *
 * No grammar rule depends on:
 *
 *     time
 *     randomness
 *     filesystem state
 *     network state
 *     hardware discovery
 *     environment variables
 *     runtime state
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics must preserve source locations.
 *
 * Semantic diagnostics should identify:
 *
 *     FFI declaration
 *     foreign target
 *     callable
 *     parameter/result boundary
 *     incompatible contract
 *     source span
 *
 * The grammar must never hide an interoperability error merely to recover
 * parsing.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * New foreign languages, ABIs, runtimes, representations, capabilities,
 * execution models, and resource classes should normally be expressible using
 * the open symbolic forms.
 *
 * Adding a new symbolic value MUST NOT require adding a new parser alternative
 * merely because a new ecosystem appeared.
 *
 * Syntax removal or incompatible syntax changes require the normal Zamani
 * compatibility/migration process.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT introduce:
 *
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
 * It MUST NOT introduce:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     fpga0
 *     physical_qubit0
 *     device0
 *     register0
 *     memory_bank0
 *
 * as universal language constructs.
 *
 * ============================================================================
 * NO UNSAFE
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * The downstream Zamani compiler/runtime implementation remains subject to:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     unsafe forbidden
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when all of the following are true:
 *
 * [ ] No duplicate canonical identifier grammar exists here.
 * [ ] No duplicate canonical type grammar exists here.
 * [ ] No duplicate canonical expression grammar exists here.
 * [ ] No duplicate canonical parameter grammar exists here.
 * [ ] No duplicate ABI grammar exists here.
 * [ ] FFI declarations are syntactically deterministic.
 * [ ] FFI calls are syntactically deterministic.
 * [ ] Callback contracts are representable.
 * [ ] Ownership contracts are representable.
 * [ ] Lifetime contracts are representable.
 * [ ] Nullability contracts are representable.
 * [ ] Marshalling contracts are representable.
 * [ ] Error-boundary contracts are representable.
 * [ ] Effect requirements are representable.
 * [ ] Capability requirements are representable.
 * [ ] Resource requirements are representable.
 * [ ] Security requirements are representable.
 * [ ] Compatibility requirements are representable.
 * [ ] Asynchronous boundaries are representable.
 * [ ] Streaming boundaries are representable.
 * [ ] Adapters are representable.
 * [ ] Symbolic future ecosystems remain representable.
 * [ ] No hardware capacity is hard-coded.
 * [ ] No physical address is hard-coded.
 * [ ] No machine topology is hard-coded.
 * [ ] No runtime action occurs during parsing.
 * [ ] AST mapping is defined.
 * [ ] Semantic mapping is defined.
 * [ ] ABI integration is defined.
 * [ ] Canonical IR integration is defined.
 * [ ] Quantum integration preserves quantum::ir.
 * [ ] HDL/hardware integration remains downstream.
 * [ ] Positive tests exist.
 * [ ] Negative tests exist.
 * [ ] Boundary tests exist.
 * [ ] Scalability tests exist.
 * [ ] Compatibility tests exist.
 * [ ] Determinism tests exist.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */