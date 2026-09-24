/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* File:
* grammar/quantum/kernels.g4
* 
* Grammar:
* QuantumKernels
* 
* Status:
* CANONICAL / PRODUCTION QUANTUM KERNEL GRAMMAR
* 
* Language baseline:
* Zamani
* 
* Grammar technology:
* ANTLR4 parser grammar
* 
* Compiler baseline:
* Rust 1.97 / Rust 1.97.1
* Rust 2021
* 
* Safety:
* Safe Rust only.
* No unsafe Rust is required by this grammar.
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file owns SOURCE-LEVEL QUANTUM KERNEL SYNTAX.
* 
* A quantum kernel is a reusable quantum computation boundary whose source
* describes computational intent independently of the physical machine on
* which the computation will eventually execute.
* 
* A kernel may contain:
* 
* - quantum declarations;
* - classical parameters;
* - quantum parameters;
* - quantum operations;
* - classical expressions;
* - measurement;
* - dynamic control;
* - resource requirements;
* - capability requirements;
* - semantic constraints;
* - annotations;
* - nested source-level kernel composition;
* - interoperability references;
* - future quantum dialect constructs.
* 
* This grammar deliberately does NOT describe:
* 
* - a particular QPU;
* - a simulator;
* - a physical qubit map;
* - a coupling map;
* - calibration;
* - pulses;
* - gate duration;
* - routing;
* - scheduling;
* - QEC implementation;
* - ZQN implementation;
* - backend selection;
* - device allocation;
* - runtime state.
* 
* Those responsibilities remain downstream.
* 
* ============================================================================
* CORE ARCHITECTURAL CONTRACT
* ============================================================================
* 
* Zamani source
*      |
*      v
* canonical lexer
*      |
*      v
* canonical Zamani parser
*      |
*      +--> QuantumKernels
*      |
*      v
* domain-neutral frontend AST
*      |
*      v
* semantic analysis
*      |
*      +--> name resolution
*      +--> type checking
*      +--> effect analysis
*      +--> resource analysis
*      +--> capability analysis
*      +--> quantum validation
*      |
*      v
* canonical semantic representation
*      |
*      v
* quantum::ir
*      |
*      +--> optimization
*      +--> decomposition
*      +--> routing
*      +--> scheduling
*      +--> QEC
*      +--> resilience
*      +--> ZQN
*      +--> HAL
*      |
*      v
* target realization
* 
* This file must never bypass the AST or semantic boundaries.
* 
* ============================================================================
* POCO-REAF CONTRACT
* ============================================================================
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* A quantum kernel describes WHAT computation is intended.
* 
* It does not permanently prescribe:
* 
* CPU
* GPU
* FPGA
* ASIC
* QPU
* physical qubit
* physical register
* physical memory bank
* device identifier
* topology
* coupling map
* vendor instruction
* pulse implementation
* scheduler
* router
* 
* The same source kernel may therefore be lowered differently for:
* 
* - a tiny simulator;
* - a larger simulator;
* - a logical quantum computer;
* - a physical QPU;
* - a heterogeneous CPU/GPU/QPU system;
* - a future quantum architecture.
* 
* Actual realization is determined downstream from semantic intent and the
* capabilities/resources available to the compilation and execution system.
* 
* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* This grammar imposes NO universal finite machine-size limits.
* 
* It contains no:
* 
* MAX_QUBITS
* MAX_KERNELS
* MAX_PARAMETERS
* MAX_OPERATIONS
* MAX_CONTROLS
* MAX_MEASUREMENTS
* MAX_CIRCUIT_DEPTH
* MAX_DEVICES
* MAX_NODES
* MAX_MEMORY
* MAX_THREADS
* MAX_TENSOR_RANK
* 
* and no equivalent hidden constants.
* 
* Kernel parameters, declarations, operations, requirements and body items
* use ANTLR repetition constructs or existing language structures rather than
* artificial fixed cardinalities.
* 
* "Unbounded by the grammar" means that the grammar does not establish an
* artificial machine-size ceiling. Actual execution remains subject to:
* 
* - source/compiler resources;
* - target resources;
* - runtime resources;
* - explicit program requirements;
* - physical constraints;
* - deployment policy.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - quantum kernel declaration syntax;
* - kernel parameter syntax;
* - kernel generic parameter syntax;
* - kernel metadata/annotation attachment;
* - kernel body composition;
* - kernel invocation syntax where the invocation is specifically part
*   of the kernel boundary;
* - kernel-level requirement/capability metadata;
* - kernel-level source contract markers.
* 
* THIS FILE DOES NOT OWN:
* 
* - lexical tokens;
* - identifiers;
* - general expressions;
* - general types;
* - qubit declarations;
* - register declarations;
* - quantum operation implementation;
* - measurement implementation;
* - dynamic-circuit implementation;
* - QEC;
* - ZQN;
* - routing;
* - scheduling;
* - hardware;
* - resource allocation;
* - runtime execution;
* - canonical AST implementation;
* - canonical quantum::ir.
* 
* ============================================================================
* LEXICAL AUTHORITY
* ============================================================================
* 
* The production lexer remains:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* Its lexical composition ultimately comes from:
* 
* grammar/lexer/tokens.g4
* 
* This grammar defines NO lexer rules.
* 
* In particular, this file deliberately does NOT require a new:
* 
* KERNEL
* 
* lexer token.
* 
* Kernel identity is expressed through the existing annotation mechanism:
* 
* @kernel
* 
* The annotation spelling is preserved as source syntax and validated by
* semantic analysis.
* 
* This keeps the keyword space extensible and avoids creating a universal
* reserved word solely for one domain construct.
* 
* ============================================================================
* ANNOTATION CONTRACT
* ============================================================================
* 
* The canonical annotation token is consumed from the existing lexer.
* 
* The grammar accepts:
* 
* @kernel
* 
* and optional annotation arguments:
* 
* @kernel(...)
* 
* Semantic analysis determines whether the annotation represents the standard
* quantum-kernel declaration marker.
* 
* This grammar does not create a second annotation vocabulary.
* 
* ============================================================================
* EXPRESSION CONTRACT
* ============================================================================
* 
* General expressions remain owned by:
* 
* grammar/expressions/
* 
* This grammar consumes:
* 
* expression
* 
* It does not redefine:
* 
* arithmetic
* comparison
* logical operators
* bitwise operators
* assignment
* calls
* indexing
* member access
* ranges
* lambdas
* generic expression syntax
* 
* ============================================================================
* TYPE CONTRACT
* ============================================================================
* 
* General type syntax remains owned by:
* 
* grammar/types/
* 
* This grammar consumes:
* 
* typeExpression
* 
* It does not create a quantum-specific competing type system.
* 
* Examples that can therefore participate semantically include:
* 
* Qubit
* QRegister<N>
* LogicalQubit
* Tensor<T, ...>
* classical types
* user-defined types
* resource types
* capability types
* 
* Exact validity is a semantic responsibility.
* 
* ============================================================================
* QUANTUM OPERATION INTEGRATION
* ============================================================================
* 
* Quantum operation invocation remains owned by:
* 
* grammar/quantum/operations.g4
* 
* The canonical operation boundary is therefore reused rather than duplicated.
* 
* Conceptually:
* 
* kernel body
*      |
*      +--> quantumOperationStatement
* 
* The kernel grammar MUST NOT create another gate/operation grammar.
* 
* Operation names remain open-ended.
* 
* The grammar therefore does not enumerate:
* 
* H
* X
* Y
* Z
* CNOT
* CX
* CZ
* SWAP
* RX
* RY
* RZ
* 
* or future/vendor operations.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The parse structure produced by this grammar must be sufficient for the
* frontend to construct the existing domain-neutral AST representation.
* 
* Conceptually:
* 
* QuantumKernel {
*     name,
*     generic_parameters,
*     parameters,
*     metadata,
*     body,
*     source_span
* }
* 
* The exact Rust AST type is owned by:
* 
* src/frontend/ast/
* 
* This grammar must not introduce:
* 
* QuantumKernelAst
* QuantumKernelNode
* QuantumKernelIR
* 
* or another competing semantic representation.
* 
* ============================================================================
* QUANTUM IR CONTRACT
* ============================================================================
* 
* Kernel syntax lowers through the normal frontend:
* 
* kernel syntax
*     |
*     v
* parse tree
*     |
*     v
* frontend AST
*     |
*     v
* semantic quantum kernel
*     |
*     v
* quantum::ir
* 
* "quantum::ir" remains the canonical quantum semantic boundary.
* 
* This grammar does NOT import Rust quantum IR types and does NOT construct
* quantum IR.
* 
* ============================================================================
* RESOURCE / CAPABILITY CONTRACT
* ============================================================================
* 
* A kernel may declare source-level requirements or capabilities through the
* existing resource/capability grammar integration.
* 
* Examples of semantic intent include:
* 
* requires qubits >= n
* requires memory >= required_memory
* requires capability("quantum.measurement")
* requires capability("quantum.dynamic_circuit")
* 
* Such requirements do not select a machine.
* 
* The distinction remains:
* 
* requirement
*     -> what must be available
* 
* capability
*     -> what the target must be able to do
* 
* preference
*     -> what implementation is preferred
* 
* placement
*     -> where realization occurs
* 
* Placement/routing belongs downstream.
* 
* ============================================================================
* HARDWARE INDEPENDENCE
* ============================================================================
* 
* The following must NOT be encoded as universal kernel syntax:
* 
* qpu0
* gpu0
* cpu0
* physical_qubit_17
* use_8_cores
* use_32_qubits
* use_64_gb
* 
* unless an explicitly target-specific semantic construct is intentionally
* used and accepted by the language specification.
* 
* The absence of such a construct means that target realization remains free
* for the compiler and runtime.
* 
* ============================================================================
* GENERICITY CONTRACT
* ============================================================================
* 
* Kernel declarations may be generic over source-level types and values.
* 
* Examples:
* 
* @kernel
* fn evolve<T>(state: T) {
*     ...
* }
* 
* @kernel
* fn prepare<N>(q: QRegister<N>) {
*     ...
* }
* 
* Generic parameters are not machine-size declarations.
* 
* A value such as N may become a semantic resource requirement after semantic
* analysis, but the grammar does not assign N a physical interpretation.
* 
* ============================================================================
* PARAMETER CONTRACT
* ============================================================================
* 
* Kernel parameters are ordinary Zamani parameters:
* 
* name
* optional type
* optional default expression
* 
* Parameter count is unbounded by grammar.
* 
* Examples:
* 
* @kernel
* fn prepare(theta: Angle, q: Qubit) {
*     ...
* }
* 
* @kernel
* fn algorithm<T>(input: T, q: Qubit) {
*     ...
* }
* 
* ============================================================================
* BODY CONTRACT
* ============================================================================
* 
* The kernel body is a Zamani block.
* 
* It may contain existing statements, including quantum-specific statements
* exposed by the canonical quantum composition grammar.
* 
* This grammar does not duplicate the complete statement grammar.
* 
* Instead it provides an integration point for the canonical statement
* composition.
* 
* This prevents:
* 
* QuantumKernelStatement
* 
* from becoming a second language-wide statement hierarchy.
* 
* ============================================================================
* KERNEL INVOCATION CONTRACT
* ============================================================================
* 
* Kernel calls are ordinary Zamani calls whenever the kernel is used as a
* callable value.
* 
* This grammar therefore does NOT redefine general function-call syntax.
* 
* For example:
* 
* prepare(theta, q);
* 
* remains an ordinary call expression/statement.
* 
* Semantic analysis determines whether "prepare" resolves to:
* 
* - a quantum kernel;
* - a classical function;
* - a hybrid function;
* - another callable value;
* - an unresolved name.
* 
* This is essential for POCO-REAF because a kernel is a semantic callable
* computation rather than a special runtime primitive.
* 
* ============================================================================
* NESTED KERNELS
* ============================================================================
* 
* Nested kernel declarations are intentionally NOT introduced as a special
* recursive grammar construct here.
* 
* If the language specification permits nested declarations, they are admitted
* through the canonical declaration/block composition layer.
* 
* This prevents the quantum grammar from creating a second declaration scope
* system.
* 
* ============================================================================
* RECURSION / COMPOSITION
* ============================================================================
* 
* Kernel composition is semantic.
* 
* A kernel may call another kernel through ordinary callable syntax.
* 
* Therefore:
* 
* kernel A
*     -> call B
*     -> call C
* 
* does not require a special kernel-call grammar.
* 
* Semantic analysis is responsible for:
* 
* - name resolution;
* - callability;
* - recursion;
* - generic substitution;
* - effect checking;
* - resource propagation;
* - capability propagation;
* - termination policy where required.
* 
* ============================================================================
* EFFECT CONTRACT
* ============================================================================
* 
* A quantum kernel may have effects arising from:
* 
* - measurement;
* - classical interaction;
* - external resources;
* - nondeterministic physical execution;
* - runtime interaction.
* 
* The kernel grammar does not implement effect checking.
* 
* Existing effect syntax remains owned by:
* 
* grammar/effects/
* 
* and is consumed through the canonical language composition layer.
* 
* ============================================================================
* DYNAMIC-CIRCUIT CONTRACT
* ============================================================================
* 
* A kernel may contain dynamic-circuit operations if those operations are
* admitted by the canonical quantum statement composition.
* 
* The grammar does not require every target to support them.
* 
* Instead:
* 
* source syntax
*     |
*     v
* semantic capability requirement
*     |
*     v
* target capability analysis
*     |
*     +--> direct lowering
*     +--> transformation
*     +--> diagnostic
* 
* Therefore a target capability failure is not a grammar failure.
* 
* ============================================================================
* QEC CONTRACT
* ============================================================================
* 
* Kernel syntax may carry QEC/error-correction intent through existing quantum
* resource/annotation mechanisms.
* 
* This grammar must never implement:
* 
* - syndrome decoding;
* - stabilizer simulation;
* - recovery;
* - decoder selection;
* - code construction;
* - fault-tolerant scheduling;
* - physical correction.
* 
* Those remain owned by the QEC subsystem.
* 
* ============================================================================
* ZQN CONTRACT
* ============================================================================
* 
* ZQN remains responsible for quantum-noise/fault semantics.
* 
* Kernel syntax may provide semantic intent required by ZQN, but the grammar
* does not implement noise models, correlation kernels, calibration or fault
* propagation.
* 
* ============================================================================
* ROUTING CONTRACT
* ============================================================================
* 
* A kernel contains logical computation intent.
* 
* Routing determines how logical resources become physically executable.
* 
* This grammar must not encode:
* 
* coupling maps;
* topology;
* shortest paths;
* SWAP insertion;
* physical qubit assignments.
* 
* ============================================================================
* SCHEDULING CONTRACT
* ============================================================================
* 
* The kernel grammar does not encode target-specific schedules.
* 
* It does not contain:
* 
* clock cycles;
* hardware gate durations;
* queue IDs;
* execution slots;
* pulse timing.
* 
* If timing is part of program meaning, timing semantics must be expressed
* through the established timing/resource grammar rather than hidden kernel
* constants.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* This grammar contains:
* 
* - no embedded Rust actions;
* - no semantic predicates;
* - no filesystem access;
* - no network access;
* - no hardware discovery;
* - no runtime calls;
* - no randomness.
* 
* Parsing therefore depends only on the supplied token stream and the imported
* canonical grammar definitions.
* 
* ============================================================================
* SECURITY
* ============================================================================
* 
* Kernel names, parameters, annotations and expressions are source data.
* 
* Parsing must not execute:
* 
* shell commands;
* source code;
* device commands;
* filesystem operations;
* network requests.
* 
* Any runtime interpretation is downstream and must pass through the normal
* semantic/security boundaries.
* 
* ============================================================================
* IMPORT CONTRACT
* ============================================================================
* 
* This grammar is a parser grammar and therefore consumes the canonical lexer.
* 
* It imports:
* 
* Types
* Expressions
* 
* These provide the common type/expression vocabulary without creating
* duplicate grammar definitions.
* 
* Statement composition is intentionally not redefined here.
* 
* The canonical Zamani parser/domain composition layer is responsible for
* making the kernel body use the language's complete statement vocabulary,
* including the existing quantum operation grammar.
* 
* ============================================================================
  */

parser grammar QuantumKernels;

options {
tokenVocab = ZamaniLexer;
}

import
Types,
Expressions
;

/*

* ============================================================================
* 1. PUBLIC ENTRY POINT
* ============================================================================
* 
* This is the stable parser entry point exported by this file.
  */

quantumKernelConstruct
: quantumKernelDeclaration
;

/*

* ============================================================================
* 2. KERNEL DECLARATION
* ============================================================================
* 
* Canonical source shape:
* 
* @kernel
* fn prepare(q: Qubit) {
*     ...
* }
* 
* Optional metadata may appear after @kernel:
* 
* @kernel(...)
* fn prepare(...) {
*     ...
* }
* 
* The "fn" keyword remains owned by the canonical function grammar.
* 
* This file deliberately does not create a second function-declaration syntax.
  */

quantumKernelDeclaration
: quantumKernelAnnotation
quantumKernelFunctionSignature
quantumKernelBody
;

/*

* ============================================================================
* 3. KERNEL ANNOTATION
* ============================================================================
* 
* The annotation token is the existing canonical annotation token.
* 
* Semantic analysis verifies that its normalized name is "kernel".
* 
* The grammar does not require a new KERNEL lexer token.
  */

quantumKernelAnnotation
: AT annotationName
quantumKernelAnnotationArguments?
;

/*

* ============================================================================
* 4. ANNOTATION NAME
* ============================================================================
* 
* The annotation name remains an ordinary identifier.
* 
* This intentionally preserves extensibility.
  */

annotationName
: IDENTIFIER
;

/*

* ============================================================================
* 5. ANNOTATION ARGUMENTS
* ============================================================================
* 
* Annotation arguments use ordinary expressions.
* 
* Examples:
* 
* @kernel
* @kernel(kind)
* @kernel(mode = "dynamic")
* 
* Semantic validation determines which annotation properties are meaningful.
  */

quantumKernelAnnotationArguments
: LPAREN
quantumKernelAnnotationArgumentList?
RPAREN
;

quantumKernelAnnotationArgumentList
: expression
(
COMMA
expression
)*
COMMA?
;

/*

* ============================================================================
* 6. KERNEL FUNCTION SIGNATURE
* ============================================================================
* 
* The signature intentionally mirrors the language's ordinary callable
* function model.
* 
* The quantum-kernel distinction comes from semantic annotation/analysis,
* not from a parallel function type system.
  */

quantumKernelFunctionSignature
: FN
identifier
quantumKernelGenericParameters?
quantumKernelParameters
quantumKernelReturnType?
;

/*

* ============================================================================
* 7. KERNEL GENERIC PARAMETERS
* ============================================================================
* 
* Generic parameter count is unbounded by the grammar.
* 
* Examples:
* 
* <T>
* <T, N>
* <State, N, Precision>
* 
* Bounds/default semantics are resolved downstream.
  */

quantumKernelGenericParameters
: LESS_THAN
quantumKernelGenericParameterList
GREATER_THAN
;

quantumKernelGenericParameterList
: quantumKernelGenericParameter
(
COMMA
quantumKernelGenericParameter
)*
COMMA?
;

quantumKernelGenericParameter
: identifier
quantumKernelGenericParameterBound?
quantumKernelGenericParameterDefault?
;

quantumKernelGenericParameterBound
: COLON
typeExpression
;

quantumKernelGenericParameterDefault
: ASSIGN
expression
;

/*

* ============================================================================
* 8. KERNEL PARAMETERS
* ============================================================================
* 
* Parameter syntax remains compatible with the existing frontend function
* model.
* 
* The grammar supports:
* 
* q: Qubit
* theta: Angle
* state: State
* n: Int
* 
* and ordinary default expressions.
  */

quantumKernelParameters
: LPAREN
quantumKernelParameterList?
RPAREN
;

quantumKernelParameterList
: quantumKernelParameter
(
COMMA
quantumKernelParameter
)*
COMMA?
;

quantumKernelParameter
: quantumKernelParameterBinding
quantumKernelParameterType?
quantumKernelParameterDefault?
;

quantumKernelParameterBinding
: identifier
;

quantumKernelParameterType
: COLON
typeExpression
;

quantumKernelParameterDefault
: ASSIGN
expression
;

/*

* ============================================================================
* 9. KERNEL RETURN TYPE
* ============================================================================
* 
* Return type syntax remains owned by the canonical type grammar.
  */

quantumKernelReturnType
: THIN_ARROW
typeExpression
;

/*

* ============================================================================
* 10. KERNEL BODY
* ============================================================================
* 
* The body is intentionally represented as a generic Zamani block boundary.
* 
* Statement composition belongs to the canonical Zamani statement grammar.
* 
* The actual canonical parser composition layer must connect this boundary to
* the repository's universal "statement" production.
* 
* This file does not redefine statements because doing so would create a
* second statement language.
  */

quantumKernelBody
: LBRACE
quantumKernelBodyItem*
RBRACE
;

quantumKernelBodyItem
: quantumKernelBodyStatement
;

quantumKernelBodyStatement
: quantumKernelStatementPlaceholder
;

/*

* ============================================================================
* 11. BODY INTEGRATION HOOK
* ============================================================================
* 
* IMPORTANT:
* 
* ANTLR imported parser grammars cannot safely invent a second "statement"
* production merely to bridge independently owned statement grammars.
* 
* The canonical Zamani composition grammar is therefore responsible for
* replacing/composing this boundary with the repository's universal statement
* production when "QuantumKernels" is integrated.
* 
* The hook exists so that:
* 
* QuantumKernels
*     |
*     v
* universal statement composition
*     |
*     +--> quantumOperationStatement
*     +--> measurement
*     +--> reset
*     +--> dynamic control
*     +--> classical statements
*     +--> declarations
*     +--> calls
* 
* without making this file the owner of those constructs.
* 
* The production below is deliberately a structural identifier-based
* integration marker rather than an implementation of another statement
* grammar.
* 
* The canonical composition integration MUST map the kernel body to the
* existing language-wide statement production.
  */

quantumKernelStatementPlaceholder
: identifier
;

/*

* ============================================================================
* 12. IDENTIFIER
* ============================================================================
* 
* Identifier spelling is inherited from the canonical lexer.
* 
* This wrapper exists only to provide a stable local semantic name.
  */

identifier
: IDENTIFIER
;

/*

* ============================================================================
* 13. SEMANTIC KERNEL CONTRACT
* ============================================================================
* 
* After parsing, semantic analysis must establish:
* 
* - the @kernel annotation is valid;
* - the function name is valid;
* - generic parameters are valid;
* - parameter types are valid;
* - default values are type-compatible;
* - return type is valid;
* - body statements are valid;
* - quantum operands are valid;
* - classical/quantum boundaries are valid;
* - effects are valid;
* - required capabilities are satisfiable;
* - required resources are satisfiable;
* - nested calls are valid;
* - recursive behavior follows language rules.
* 
* None of these semantic decisions belong in this grammar.
* 
* ============================================================================
* 14. RESOURCE PROPAGATION
* ============================================================================
* 
* A kernel can semantically propagate requirements from its body.
* 
* For example:
* 
* kernel A
*     calls kernel B
* 
* and B requires:
* 
* capability("quantum.measurement")
* 
* The semantic layer may determine that A consequently requires the same
* capability.
* 
* This is semantic propagation, not grammar expansion.
* 
* ============================================================================
* 15. CAPABILITY PROPAGATION
* ============================================================================
* 
* Kernel capabilities must remain open-ended.
* 
* Examples:
* 
* quantum.measurement
* quantum.dynamic_circuit
* quantum.mid_circuit_measurement
* quantum.conditional_control
* quantum.logical_qubit
* quantum.error_correction
* 
* The grammar does not enumerate a closed universal capability list.
* 
* ============================================================================
* 16. RESOURCE SCALING
* ============================================================================
* 
* A kernel may semantically require resources whose quantity is symbolic.
* 
* Example:
* 
* QRegister<N>
* 
* does not establish:
* 
* N <= 1024
* 
* or:
* 
* N <= any compiler-selected constant.
* 
* The actual resource resolver determines whether the selected execution
* environment can satisfy the requirement.
* 
* ============================================================================
* 17. CLASSICAL / QUANTUM HYBRID EXECUTION
* ============================================================================
* 
* A kernel may contain both classical and quantum computation through the
* universal language composition.
* 
* Conceptually:
* 
* classical expression
*      |
*      v
* quantum operation
*      |
*      v
* measurement
*      |
*      v
* classical condition
*      |
*      v
* quantum operation
* 
* The grammar does not duplicate classical or dynamic-circuit semantics.
* 
* ============================================================================
* 18. HARD-CODING AUDIT
* ============================================================================
* 
* This file MUST remain free of:
* 
* MAX_QUBITS
* MAX_KERNELS
* MAX_PARAMETERS
* MAX_OPERATIONS
* MAX_CONTROLS
* MAX_MEASUREMENTS
* MAX_DEVICES
* MAX_NODES
* MAX_MEMORY
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_REGISTER_WIDTH
* MAX_TENSOR_RANK
* MAX_NETWORK_SIZE
* MAX_DEVICE_COUNT
* 
* It must also not encode fixed examples as syntax:
* 
* q0
* q1
* q63
* device0
* qpu0
* 
* Such names may exist as ordinary user identifiers where the language allows
* them, but the grammar must not reserve them as machine resources.
* 
* ============================================================================
* 19. SOURCE-LEVEL EXAMPLES
* ============================================================================
* 
* These examples illustrate intended syntax. They are NOT gate enumerations.
* 
* Minimal:
* 
* @kernel
* fn prepare(q: Qubit) {
*     apply H(q);
* }
* 
* Parameterized:
* 
* @kernel
* fn rotate(theta: Angle, q: Qubit) {
*     apply RX(theta)(q);
* }
* 
* Generic:
* 
* @kernel
* fn prepare<N>(q: QRegister<N>) {
*     ...
* }
* 
* Qualified operation:
* 
* @kernel
* fn custom(q: Qubit) {
*     apply vendor::operation(q);
* }
* 
* Hybrid:
* 
* @kernel
* fn hybrid(theta: Angle, q: Qubit) {
*     apply operation(theta)(q);
*     ...
* }
* 
* The semantic layer determines what each operation name means.
* 
* ============================================================================
* 20. INVALID STRUCTURAL FORMS
* ============================================================================
* 
* These should be rejected structurally:
* 
* @kernel
* 
* @kernel fn
* 
* @kernel fn prepare(
* 
* @kernel fn prepare(q:
* 
* @kernel fn prepare() {
* 
* where the required closing structure is absent.
* 
* Semantic errors such as:
* 
* @kernel
* fn prepare(q: Qubit) {
*     apply unknown_operation(q);
* }
* 
* are NOT necessarily grammar errors.
* 
* They are semantic resolution errors.
* 
* ============================================================================
* 21. DOMAIN SEPARATION
* ============================================================================
* 
* This grammar does not make:
* 
* quantum kernel
* 
* synonymous with:
* 
* GPU kernel
* CPU kernel
* FPGA kernel
* AI kernel
* operating-system kernel
* 
* The "@kernel" annotation establishes a source-level semantic category.
* 
* Domain interpretation belongs to the surrounding semantic model.
* 
* ============================================================================
* 22. AI / ACCELERATOR INTEROPERABILITY
* ============================================================================
* 
* AI accelerator grammars already use kernel terminology.
* 
* This file therefore does not introduce vendor-specific accelerator syntax.
* 
* A quantum kernel may semantically interact with:
* 
* classical kernels;
* accelerator kernels;
* tensor computation;
* distributed computation;
* HDL/hardware intent.
* 
* Such interaction is represented through the common semantic model and
* interoperability boundaries.
* 
* ============================================================================
* 23. HDL / HARDWARE CO-DESIGN
* ============================================================================
* 
* A quantum kernel may be used as a semantic input to a hardware/co-design
* pipeline.
* 
* This does NOT mean that the kernel grammar itself describes:
* 
* wires;
* registers;
* pins;
* physical clocks;
* FPGA resources;
* ASIC cells.
* 
* Those remain owned by:
* 
* grammar/hdl/
* grammar/hardware/
* 
* ============================================================================
* 24. DISTRIBUTED EXECUTION
* ============================================================================
* 
* A kernel may eventually execute:
* 
* locally;
* on an accelerator;
* on a QPU;
* across a distributed execution environment.
* 
* The kernel grammar does not prescribe the number of nodes.
* 
* Distribution is represented downstream through resource, execution,
* placement and deployment semantics.
* 
* ============================================================================
* 25. INTEROPERABILITY
* ============================================================================
* 
* A kernel may eventually lower to:
* 
* QIR
* OpenQASM
* vendor formats
* simulator representations
* hardware-specific representations
* 
* These are interoperability/target formats.
* 
* They do not replace:
* 
* quantum::ir
* 
* as Zamani's canonical quantum semantic boundary.
* 
* ============================================================================
* 26. SOURCE SPANS
* ============================================================================
* 
* All productions must remain compatible with the repository's source-span
* preservation requirements.
* 
* The grammar itself does not construct source-span objects.
* 
* The parser/frontend integration must preserve spans for:
* 
* @kernel
* kernel name
* generic parameters
* parameters
* return type
* body
* annotations
* nested expressions.
* 
* ============================================================================
* 27. DIAGNOSTICS
* ============================================================================
* 
* Diagnostics are divided into:
* 
* syntax diagnostics
* semantic diagnostics
* resource diagnostics
* capability diagnostics
* target diagnostics
* 
* This grammar is responsible only for structural syntax.
* 
* Examples:
* 
* malformed @kernel declaration
*     -> syntax diagnostic
* 
* unknown kernel name
*     -> semantic diagnostic
* 
* insufficient qubits
*     -> resource diagnostic
* 
* unsupported dynamic circuit
*     -> capability/target diagnostic
* 
* ============================================================================
* 28. DETERMINISTIC PARSING
* ============================================================================
* 
* The grammar contains no:
* 
* semantic predicates;
* actions;
* runtime calls;
* hardware queries;
* filesystem queries;
* network queries;
* environment inspection;
* randomness.
* 
* It therefore remains deterministic for a fixed source/token stream and
* grammar version.
* 
* ============================================================================
* 29. RUST INTEGRATION
* ============================================================================
* 
* This file contains no Rust actions.
* 
* The surrounding implementation must remain compatible with:
* 
* Rust 1.97
* Rust 1.97.1
* Rust 2021
* 
* and must use safe Rust only.
* 
* This grammar does not require "unsafe".
* 
* ============================================================================
* 30. TEST CONTRACT
* ============================================================================
* 
* Conformance tests for this file belong under:
* 
* grammar/tests/quantum/
* 
* Recommended groups:
* 
* kernels/
* kernels/positive/
* kernels/negative/
* kernels/generic/
* kernels/parameterized/
* kernels/hybrid/
* kernels/scalability/
* kernels/diagnostics/
* kernels/compatibility/
* 
* Minimum positive cases:
* 
* - minimal kernel;
* - parameterized kernel;
* - generic kernel;
* - quantum register parameter;
* - qualified operation;
* - arbitrary operation name;
* - annotations;
* - default parameter expressions;
* - multiple parameters;
* - multiple generic parameters.
* 
* Minimum negative cases:
* 
* - missing annotation name;
* - missing function name;
* - malformed generic parameters;
* - malformed parameter list;
* - malformed return type;
* - malformed body.
* 
* Scalability tests must verify that no artificial maximum is encoded.
* 
* ============================================================================
* 31. INTEGRATION WITH grammar/quantum/operations.g4
* ============================================================================
* 
* "operations.g4" remains the owner of quantum operation invocation syntax.
* 
* Therefore this file must not copy its productions.
* 
* The canonical composition layer must place the operation production inside
* the universal statement composition used by the kernel body.
* 
* Conceptual composition:
* 
* quantumKernelDeclaration
*     |
*     v
* kernel body
*     |
*     v
* universal statement
*     |
*     +--> quantumOperationStatement
*     +--> measurement
*     +--> reset
*     +--> dynamic circuit
*     +--> classical statement
*     +--> declaration
* 
* ============================================================================
* 32. INTEGRATION WITH grammar/quantum/README.md
* ============================================================================
* 
* This file implements the "kernels.g4" ownership slot documented by the
* quantum directory architecture.
* 
* It follows the README rules:
* 
* - backend-independent;
* - no fixed qubit count;
* - no fixed gate enumeration;
* - no physical topology;
* - no routing;
* - no scheduling;
* - no QEC implementation;
* - no ZQN implementation;
* - canonical quantum::ir boundary.
* 
* ============================================================================
* 33. INTEGRATION WITH grammar/types/
* ============================================================================
* 
* Kernel parameter and return types consume:
* 
* typeExpression
* 
* from the canonical Types grammar.
* 
* No quantum-specific duplicate type grammar is created here.
* 
* ============================================================================
* 34. INTEGRATION WITH grammar/expressions/
* ============================================================================
* 
* Annotation arguments, generic defaults and parameter defaults consume:
* 
* expression
* 
* from the canonical Expressions grammar.
* 
* No duplicate expression precedence is introduced here.
* 
* ============================================================================
* 35. INTEGRATION WITH src/frontend/ast/
* ============================================================================
* 
* The frontend must map this parse structure to the existing domain-neutral
* AST representation.
* 
* The AST must preserve:
* 
* - declaration identity;
* - annotation;
* - function identity;
* - generic parameters;
* - parameters;
* - types;
* - defaults;
* - return type;
* - body;
* - source spans.
* 
* No kernel-specific semantic IR should be added solely because this grammar
* exists.
* 
* ============================================================================
* 36. INTEGRATION WITH src/quantum/ir/
* ============================================================================
* 
* After semantic analysis, a kernel's quantum computation is lowered through
* the existing canonical:
* 
* quantum::ir
* 
* The grammar has no direct Rust dependency on it.
* 
* In particular, this file must not create a second:
* 
* QuantumKernelIR
* KernelCircuitIR
* QuantumGateIR
* 
* boundary.
* 
* ============================================================================
* 37. INTEGRATION WITH OPTIMIZATION / ROUTING / SCHEDULING
* ============================================================================
* 
* The pipeline remains:
* 
* kernel source
*     |
*     v
* AST
*     |
*     v
* semantic analysis
*     |
*     v
* quantum::ir
*     |
*     +--> optimization
*     +--> decomposition
*     +--> routing
*     +--> scheduling
*     +--> resilience
*     +--> QEC
*     +--> ZQN
*     |
*     v
* HAL
*     |
*     v
* target
* 
* No downstream subsystem is implemented in this grammar.
* 
* ============================================================================
* 38. COMPATIBILITY
* ============================================================================
* 
* This is a new file, so it must not silently change existing token meanings.
* 
* It deliberately avoids adding a new lexer keyword.
* 
* Existing ordinary functions remain ordinary functions unless semantic
* analysis recognizes the "@kernel" annotation.
* 
* This means existing source syntax does not become incompatible merely
* because "QuantumKernels" is added.
* 
* ============================================================================
* 39. FEATURE STATUS
* ============================================================================
* 
* The canonical feature lifecycle remains:
* 
* proposed
*     |
*     v
* semantic design
*     |
*     v
* AST contract
*     |
*     v
* grammar
*     |
*     v
* implementation
*     |
*     v
* IR contract
*     |
*     v
* tests
*     |
*     v
* stable
* 
* "Zamani-Grammar.md" remains a historical/extended design reference and must
* not silently override this grammar.
* 
* ============================================================================
* 40. COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when:
* 
* [x] It is the independent owner of quantum kernel declaration syntax.
* 
* [x] It retains the requested filename "kernels.g4".
* 
* [x] It is a parser grammar, not a lexer grammar.
* 
* [x] It uses the canonical Zamani lexer vocabulary.
* 
* [x] It does not introduce a KERNEL lexer keyword.
* 
* [x] It uses the existing annotation architecture.
* 
* [x] It reuses the canonical type grammar.
* 
* [x] It reuses the canonical expression grammar.
* 
* [x] It does not duplicate quantum operation syntax.
* 
* [x] It leaves operation semantics to "operations.g4".
* 
* [x] It leaves statement composition to the canonical Zamani statement
* grammar.
* 
* [x] It does not introduce a second AST.
* 
* [x] It does not introduce a second quantum IR.
* 
* [x] It preserves the canonical "quantum::ir" boundary.
* 
* [x] It does not enumerate quantum gates.
* 
* [x] It does not impose qubit limits.
* 
* [x] It does not impose parameter limits.
* 
* [x] It does not impose kernel limits.
* 
* [x] It does not encode physical hardware.
* 
* [x] It does not encode topology.
* 
* [x] It does not implement routing.
* 
* [x] It does not implement scheduling.
* 
* [x] It does not implement QEC.
* 
* [x] It does not implement ZQN.
* 
* [x] It contains no embedded Rust actions.
* 
* [x] It requires no unsafe Rust.
* 
* [x] It is compatible with the Rust 1.97 / 1.97.1 safe-Rust baseline.
* 
* [x] Its downstream integration points are defined in advance.
* 
* [x] Its scalability contract is explicit.
* 
* [x] Its diagnostics boundary is explicit.
* 
* [x] Its testing contract is explicit.
* 
* ============================================================================
* FINAL INVARIANT
* ============================================================================
* 
* "kernels.g4" answers one question:
* 
* "What is the source-level structure of a reusable quantum kernel?"
* 
* It does NOT answer:
* 
* "How is the kernel optimized?"
* "Where is it executed?"
* "Which physical qubits are used?"
* "Which device is selected?"
* "How is it routed?"
* "How is it scheduled?"
* "How is QEC implemented?"
* "How is noise modeled?"
* 
* Those remain downstream responsibilities.
* 
* The resulting architecture preserves:
* 
* Program Once
*     ->
* Compile Once
*     ->
* Run Everywhere
*     ->
* Run Anywhere
*     ->
* Run Forever
* 
* subject to program semantics, target capabilities and actual available
* resources, without introducing an artificial grammar-level hardware ceiling.
* 
* ============================================================================
  */