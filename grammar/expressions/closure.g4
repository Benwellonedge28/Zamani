/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/expressions/closure.g4
* 
* Grammar:
* ExpressionClosure
* 
* Status:
* Production-ready expression-layer closure integration grammar.
* 
* Implementation baseline:
* Rust 1.97 / Rust 1.97.1
* Edition 2021
* Safe Rust only.
* No unsafe Rust is required or permitted.
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file is the EXPRESSION-LAYER CLOSURE INTEGRATION BOUNDARY.
* 
* It does not define closure-capture syntax.
* 
* The canonical owner of closure-capture syntax is:
* 
* grammar/functions/closures.g4
* 
* That file owns:
* 
* closureCaptureClause
* closureCaptureList
* closureCapture
* closureCaptureMode
* closureCaptureTarget
* closureCaptureAlias
* 
* This file owns only the expression-layer composition of those rules.
* 
* The purpose of this boundary is to allow closure capture to participate in
* expression construction without creating another closure syntax authority.
* 
* ============================================================================
* ARCHITECTURAL RULE
* ============================================================================
* 
* There must be exactly one source-syntax authority for closure captures.
* 
* Therefore:
* 
* grammar/functions/closures.g4
* 
* is authoritative.
* 
* This file MUST NOT redefine any of its rules.
* 
* In particular, this file must never contain another implementation of:
* 
* closureCaptureClause
* closureCaptureList
* closureCapture
* closureCaptureMode
* closureCaptureTarget
* closureCaptureAlias
* 
* ============================================================================
* DOMAIN-NEUTRALITY
* ============================================================================
* 
* A closure is a language-level computation.
* 
* Its captured values may belong to any supported Zamani domain, including:
* 
* classical computation
* numerical computation
* tensors
* data
* AI/model values
* knowledge values
* probabilistic values
* quantum values
* hybrid values
* HDL elaboration values
* hardware-intent values
* distributed values
* networking values
* security values
* resource descriptions
* capability descriptions
* future domain values
* 
* This grammar must not introduce domain-specific closure syntax.
* 
* There must therefore be no:
* 
* quantumClosure
* classicalClosure
* aiClosure
* tensorClosure
* hdlClosure
* hardwareClosure
* distributedClosure
* gpuClosure
* qpuClosure
* 
* A closure remains a normal language-level expression.
* 
* ============================================================================
* POCO-REAF CONTRACT
* ============================================================================
* 
* Closure syntax is independent of physical realization.
* 
* The grammar introduces no limits on:
* 
* closure count
* capture count
* parameter count
* nesting depth
* environment size
* source size
* expression count
* task count
* thread count
* processor count
* accelerator count
* device count
* node count
* memory capacity
* quantum capacity
* 
* The grammar MUST NOT introduce artificial capacity constants or equivalent
* finite enumerations.
* 
* Prohibited examples include:
* 
* MAX_CLOSURES
* MAX_CAPTURES
* MAX_PARAMETERS
* MAX_ENVIRONMENT_SIZE
* MAX_NESTING
* MAX_THREADS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_QUBITS
* MAX_NODES
* MAX_MEMORY
* MAX_DEVICE_COUNT
* 
* Structural repetition remains structural and is bounded only by actual
* parser/compiler/runtime resources.
* 
* ============================================================================
* TARGET INDEPENDENCE
* ============================================================================
* 
* This grammar does not select:
* 
* CPU
* GPU
* FPGA
* ASIC
* accelerator
* QPU
* simulator
* cluster
* cloud
* embedded target
* distributed target
* 
* Target realization is downstream.
* 
* The pipeline is:
* 
* source
*   |
*   v
* lexer
*   |
*   v
* parser
*   |
*   v
* domain-neutral AST
*   |
*   v
* name resolution
*   |
*   v
* type analysis
*   |
*   v
* ownership / borrowing / lifetime analysis
*   |
*   v
* effect analysis
*   |
*   v
* capability analysis
*   |
*   v
* resource analysis
*   |
*   v
* contract / policy analysis
*   |
*   v
* semantic model
*   |
*   +-------------------------+
*   |                         |
*   v                         v
* classical IR             quantum::ir
*   |                         |
*   +------------+------------+
*                |
*                v
*         optimization
*                |
*                v
*           lowering
*                |
*          +-----+-----+
*          |           |
*          v           v
*       routing    scheduling
*          |           |
*          +-----+-----+
*                |
*                v
*            resilience
*                |
*                v
*               ZQN
*                |
*                v
*               HAL
*                |
*                v
*         target realization
* 
* This file participates only in the parser/expression stage.
* 
* ============================================================================
* ANTLR GRAMMAR KIND
* ============================================================================
* 
* This is a parser grammar.
* 
* It consumes the canonical Zamani lexer vocabulary:
* 
* ZamaniLexer
* 
* It imports the canonical closure grammar:
* 
* Closures
* 
* declared by:
* 
* grammar/functions/closures.g4
* 
* The import is therefore a grammar-level dependency, not a filesystem path.
* 
* ============================================================================
  */

parser grammar ExpressionClosure;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* CANONICAL CLOSURE SYNTAX IMPORT
* ============================================================================
* 
* "Closures" is the grammar name declared by:
* 
* grammar/functions/closures.g4
* 
* It is the sole source-syntax authority for closure capture clauses.
* 
* This import intentionally exposes the canonical closure rules to the
* expression layer without copying them.
* ============================================================================
  */

import Closures;

/*

* ============================================================================
* PUBLIC EXPRESSION-LAYER ENTRY POINT
* ============================================================================
* 
* This is the only closure rule that expression composition should consume
* from this file.
* 
* It is deliberately an adapter.
* 
* It does not create a new closure syntax.
* ============================================================================
  */

expressionClosureCapture
: closureCaptureClause
;

/*

* ============================================================================
* OPTIONAL CLOSURE PREFIX
* ============================================================================
* 
* This rule represents the optional closure-capture portion that can precede
* a lambda expression.
* 
* Conceptual structure:
* 
* expressionClosureCapturePrefix?
* lambda modifiers
* lambda parameters
* lambda return type
* lambda body
* 
* The lambda grammar remains the owner of the complete lambda construct.
* ============================================================================
  */

expressionClosureCapturePrefix
: expressionClosureCapture
;

/*

* ============================================================================
* CLOSURE EXPRESSION BOUNDARY
* ============================================================================
* 
* A closure expression is not redefined here because the existing frontend
* architecture already represents closure/lambda expressions through the
* established expression AST.
* 
* This grammar therefore provides the capture boundary rather than creating
* another expression production that competes with lambdaExpression.
* 
* The canonical composition remains owned by:
* 
* grammar/expressions/lambdas.g4
* 
* ============================================================================
  */

/*

* ============================================================================
* LAMBDA INTEGRATION
* ============================================================================
* 
* The lambda grammar owns:
* 
* lambdaExpression
* lambdaParameterClause
* lambdaReturnTypeClause
* lambdaBody
* 
* This file owns:
* 
* expressionClosureCapture
* expressionClosureCapturePrefix
* 
* Therefore the conceptual composition is:
* 
* expressionClosureCapturePrefix?
* lambdaModifier*
* lambdaParameterClause
* lambdaReturnTypeClause?
* lambdaBody
* 
* The exact lambda syntax remains controlled by:
* 
* grammar/expressions/lambdas.g4
* 
* This file must not copy those rules.
* 
* ============================================================================
  */

/*

* ============================================================================
* FUNCTION INTEGRATION
* ============================================================================
* 
* Function declarations remain owned by:
* 
* grammar/functions/functions.g4
* 
* Function return clauses remain owned by:
* 
* grammar/functions/returns.g4
* 
* Function-type syntax remains owned by:
* 
* grammar/types/function.g4
* 
* Closure capture does not modify function declaration or function-type
* syntax.
* 
* A closure may eventually have a callable type, but that type is determined
* by semantic/type analysis rather than this grammar.
* 
* ============================================================================
  */

/*

* ============================================================================
* EXPRESSION INTEGRATION
* ============================================================================
* 
* The general expression grammar should consume this file's public boundary:
* 
* expressionClosureCapture
* 
* or:
* 
* expressionClosureCapturePrefix
* 
* as appropriate for the expression composition architecture.
* 
* The complete expression grammar MUST NOT be imported here.
* 
* This prevents cycles such as:
* 
* ExpressionClosure
*     -> Expressions
*     -> ExpressionClosure
* 
* The dependency direction must remain one-way:
* 
* expression composition
*      |
*      +--> closure integration
*      |
*      +--> lambda integration
* 
* ============================================================================
  */

/*

* ============================================================================
* NO DUPLICATE CAPTURE RULES
* ============================================================================
* 
* The following rules belong exclusively to grammar/functions/closures.g4:
* 
* closureCaptureClause
* closureCaptureList
* closureCapture
* closureCaptureMode
* closureCaptureTarget
* closureCaptureAlias
* 
* They must never be recreated here.
* 
* This includes apparently harmless aliases such as:
* 
* closureCaptureListExpression
* expressionCaptureList
* lambdaCaptureList
* closureCaptureMode2
* 
* unless a future specification explicitly establishes a distinct semantic
* construct.
* 
* ============================================================================
  */

/*

* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* This file creates no new AST hierarchy.
* 
* The existing frontend AST is authoritative.
* 
* The repository already has closure expression support under:
* 
* src/frontend/ast/node/expressions/closure.rs
* 
* and expression module integration under:
* 
* src/frontend/ast/node/expressions/mod.rs
* 
* Therefore this grammar must lower into the existing representation rather
* than introduce:
* 
* ExpressionClosure
* ClosureExpression2
* LambdaClosureExpression
* QuantumClosureExpression
* DomainClosureExpression
* 
* The parser must preserve:
* 
* source span
* capture-entry order
* capture modes
* capture targets
* wildcard targets
* aliases
* 
* without performing semantic interpretation.
* 
* ============================================================================
  */

/*

* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Parsing establishes only structural validity.
* 
* Semantic analysis is responsible for determining:
* 
* whether a captured name exists;
* whether it is in scope;
* whether the capture is permitted;
* whether a capture mode is valid;
* whether capture modes conflict;
* whether duplicate captures are legal;
* whether mutable access is legal;
* whether a move is legal;
* whether a reference remains valid;
* whether an alias conflicts;
* whether the resulting closure is callable;
* whether captured values satisfy type constraints;
* whether captured values satisfy effect constraints;
* whether captured values satisfy capability requirements;
* whether captured values satisfy resource requirements;
* whether the closure is valid for concurrent execution;
* whether the closure is valid for distributed execution.
* 
* None of these decisions belong in this grammar.
* 
* ============================================================================
  */

/*

* ============================================================================
* OWNERSHIP / BORROWING CONTRACT
* ============================================================================
* 
* The closure grammar expresses capture intent only.
* 
* It does not implement:
* 
* ownership
* borrowing
* lifetimes
* move checking
* aliasing analysis
* mutation analysis
* 
* Those belong to the semantic/type system.
* 
* This separation is necessary so the same source construct remains usable
* across different target realizations without changing its syntax.
* 
* ============================================================================
  */

/*

* ============================================================================
* EFFECT CONTRACT
* ============================================================================
* 
* Closure capture itself is not an effect declaration.
* 
* Effects are owned by:
* 
* grammar/effects/
* 
* and the corresponding semantic subsystem.
* 
* A captured value may carry effects or capabilities, but this file does not
* resolve them.
* 
* For example, a closure may eventually contain computation involving:
* 
* IO
* network
* mutation
* randomness
* native calls
* foreign calls
* measurement
* learning
* adaptation
* reflection
* distributed execution
* 
* Those meanings are resolved after parsing.
* 
* ============================================================================
  */

/*

* ============================================================================
* CAPABILITY CONTRACT
* ============================================================================
* 
* Closure syntax does not select physical capabilities.
* 
* A closure may semantically depend on capabilities such as:
* 
* tensor.compute
* quantum.measurement
* accelerator.compute
* network.transport
* native.execute
* 
* Capability requirements belong to the semantic/resource architecture.
* 
* This grammar must not inspect target hardware.
* 
* ============================================================================
  */

/*

* ============================================================================
* RESOURCE CONTRACT
* ============================================================================
* 
* A captured value is not a resource declaration.
* 
* This file must not encode:
* 
* memory capacity
* CPU count
* GPU count
* FPGA count
* accelerator count
* QPU count
* qubit count
* node count
* device count
* topology size
* bandwidth
* storage capacity
* 
* Resource requirements belong to:
* 
* grammar/resources/
* 
* and downstream semantic/resource analysis.
* 
* ============================================================================
  */

/*

* ============================================================================
* CONTRACT INTEGRATION
* ============================================================================
* 
* Closure expressions may appear inside constructs carrying:
* 
* requires
* ensures
* invariant
* assume
* guarantee
* property
* 
* Closure syntax itself does not own those contract forms.
* 
* Contracts remain owned by:
* 
* grammar/validation/
* 
* This prevents closure syntax from becoming coupled to one particular
* contract system.
* 
* ============================================================================
  */

/*

* ============================================================================
* POLICY INTEGRATION
* ============================================================================
* 
* A closure may execute under policies controlling:
* 
* effects
* capabilities
* resources
* security
* adaptation
* execution
* sandboxing
* deployment
* 
* Policy syntax remains owned by:
* 
* grammar/policies/
* 
* or the repository's established policy owner.
* 
* This file does not duplicate policy syntax.
* 
* ============================================================================
  */

/*

* ============================================================================
* PROVENANCE CONTRACT
* ============================================================================
* 
* Closure expressions must remain traceable through the normal source-span
* and semantic provenance pipeline.
* 
* Provenance may later record:
* 
* source location
* captured binding
* transformation
* closure conversion
* optimization
* lowering
* generated representation
* 
* This grammar does not implement provenance.
* 
* ============================================================================
  */

/*

* ============================================================================
* CONCURRENCY INTEGRATION
* ============================================================================
* 
* A closure may eventually be used as:
* 
* synchronous computation
* asynchronous computation
* task body
* actor behavior
* parallel operation
* distributed computation
* accelerator computation
* 
* This file does not select any of those execution models.
* 
* Existing concurrency grammar and semantic infrastructure remain authoritative.
* 
* Capture legality for concurrent execution is a semantic question involving:
* 
* ownership
* borrowing
* Send-like capabilities
* synchronization
* effects
* resource requirements
* policy
* 
* ============================================================================
  */

/*

* ============================================================================
* DISTRIBUTED INTEGRATION
* ============================================================================
* 
* Capturing a value does not automatically mean that the value is:
* 
* serializable
* transferable
* replicated
* remotely addressable
* migratable
* 
* Distributed semantics determine those properties.
* 
* The closure grammar remains independent of:
* 
* node topology
* process placement
* network location
* serialization format
* transport protocol
* 
* ============================================================================
  */

/*

* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* Quantum values remain ordinary language-level values at this syntactic
* boundary.
* 
* Conceptually:
* 
* capture { q }
* 
* may capture a quantum-semantic binding if the type and semantic systems
* permit it.
* 
* This file MUST NOT create special syntax such as:
* 
* quantumCapture
* qubitCapture
* physicalQubitCapture
* QPUCapture
* 
* Nor may it encode physical qubit identifiers or hardware topology.
* 
* If the resulting computation has quantum semantics, the canonical boundary
* remains:
* 
* quantum::ir
* 
* No additional quantum closure IR is permitted.
* 
* ============================================================================
  */

/*

* ============================================================================
* HYBRID COMPUTATION
* ============================================================================
* 
* A closure can participate in hybrid computation.
* 
* For example, a closure may capture a classical parameter used by quantum
* computation, or capture a result produced by measurement.
* 
* The syntax remains unchanged.
* 
* The semantic pipeline determines whether the closure participates in:
* 
* classical computation
* quantum computation
* hybrid computation
* 
* ============================================================================
  */

/*

* ============================================================================
* AI / REASONING / LEARNING INTEGRATION
* ============================================================================
* 
* Closures may be used by:
* 
* inference
* reasoning
* knowledge processing
* learning
* adaptation
* model evaluation
* planning
* agent execution
* neural-symbolic computation
* 
* No AI-specific closure syntax is required.
* 
* The same closure construct is reused across all domains.
* 
* ============================================================================
  */

/*

* ============================================================================
* HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* A closure may participate in compile-time or semantic hardware elaboration
* when permitted by the language's metaprogramming and HDL systems.
* 
* This grammar must not encode:
* 
* physical wires
* register widths
* FPGA resources
* ASIC cells
* timing values
* device-specific placement
* 
* Those belong to HDL/hardware semantics and downstream realization.
* 
* ============================================================================
  */

/*

* ============================================================================
* METAPROGRAMMING INTEGRATION
* ============================================================================
* 
* A closure may be used in compile-time computation if the metaprogramming
* subsystem permits it.
* 
* This file does not decide whether a closure executes:
* 
* at compile time
* at runtime
* during elaboration
* during simulation
* 
* Execution phase is a semantic/compiler concern.
* 
* ============================================================================
  */

/*

* ============================================================================
* DETERMINISM
* ============================================================================
* 
* Parsing through this grammar must be deterministic.
* 
* The parser must not depend on:
* 
* wall-clock time
* randomness
* hardware discovery
* filesystem state
* network state
* scheduler state
* resource availability
* target selection
* runtime state
* 
* For identical:
* 
* source
* lexer configuration
* grammar version
* 
* the parser must produce equivalent parse structure.
* 
* ============================================================================
  */

/*

* ============================================================================
* SOURCE-SPAN CONTRACT
* ============================================================================
* 
* Source spans must survive the expression integration boundary.
* 
* Diagnostics and AST construction must be able to identify:
* 
* capture keyword
* opening delimiter
* individual capture entry
* capture mode
* target
* alias
* separators
* closing delimiter
* 
* This is necessary for:
* 
* compiler diagnostics
* IDE/LSP
* formatting
* refactoring
* source maps
* provenance
* reproducible compilation
* incremental compilation
* 
* ============================================================================
  */

/*

* ============================================================================
* ERROR-SEPARATION CONTRACT
* ============================================================================
* 
* Parser errors include:
* 
* malformed capture clause
* missing opening delimiter
* missing closing delimiter
* malformed capture entry
* malformed target
* malformed alias
* invalid separator placement
* 
* Semantic errors include:
* 
* unknown binding
* out-of-scope binding
* illegal capture mode
* conflicting capture modes
* ownership violation
* borrow violation
* lifetime violation
* alias conflict
* type incompatibility
* 
* Resource/capability errors include:
* 
* unavailable capability
* unsatisfied resource requirement
* deployment restriction
* 
* These error classes must remain distinct.
* 
* ============================================================================
  */

/*

* ============================================================================
* COMPATIBILITY
* ============================================================================
* 
* Existing canonical closure syntax remains owned by:
* 
* grammar/functions/closures.g4
* 
* Existing expression-level closure integration historically exists under:
* 
* grammar/expressions/closures.g4
* 
* The repository must not maintain two independent implementations of the
* same expression integration boundary.
* 
* When this singular file is adopted as the canonical expression integration
* filename, the existing plural expression adapter should become either:
* 
* a compatibility wrapper
* 
* or be removed only through an explicit repository migration.
* 
* No semantic behavior should be duplicated.
* 
* The canonical capture grammar remains unchanged by this migration.
* 
* ============================================================================
  */

/*

* ============================================================================
* DEPENDENCY CONTRACT
* ============================================================================
* 
* DEPENDS_ON:
* 
* grammar/functions/closures.g4
* grammar/antlr/ZamaniLexer.g4
* 
* CONSUMED_BY:
* 
* grammar/expressions/lambdas.g4
* expression composition grammar
* 
* AST_OWNER:
* 
* src/frontend/ast/node/expressions/closure.rs
* 
* and the existing expression AST module
* 
* SEMANTIC_OWNER:
* 
* frontend semantic analysis
* 
* TYPE_OWNER:
* 
* existing type/ownership/borrow analysis
* 
* EFFECT_OWNER:
* 
* grammar/effects/ and semantic effect analysis
* 
* CAPABILITY_OWNER:
* 
* grammar/resources/ and semantic capability analysis
* 
* RESOURCE_OWNER:
* 
* grammar/resources/ and semantic resource analysis
* 
* CONTRACT_OWNER:
* 
* grammar/validation/
* 
* POLICY_OWNER:
* 
* grammar/policies/ or established policy owner
* 
* PROVENANCE_OWNER:
* 
* established provenance subsystem
* 
* IR_OWNER:
* 
* canonical semantic model
* 
* classical IR where applicable
* 
* quantum::ir where applicable
* 
* TEST_OWNER:
* 
* grammar/tests/
* 
* expression/closure conformance tests
* 
* SPEC_OWNER:
* 
* grammar/specification/
* 
* relevant expression/function specification
* 
* ============================================================================
  */

/*

* ============================================================================
* INTEGRATION WITH THE RUST FRONTEND
* ============================================================================
* 
* This grammar contains no Rust implementation code.
* 
* The Rust frontend already contains closure expression support under:
* 
* src/frontend/ast/node/expressions/closure.rs
* 
* The grammar must map to that established AST contract.
* 
* The hand-written parser under:
* 
* src/parser.rs
* 
* also contains closure parsing.
* 
* The repository therefore has two parser concerns:
* 
* ANTLR grammar
* hand-written Rust parser
* 
* They MUST NOT silently define incompatible closure languages.
* 
* Conformance testing must compare their accepted/rejected source forms and
* semantic structure.
* 
* The hand-written parser is an implementation/conformance concern and must
* not cause this grammar to acquire Rust-specific syntax.
* 
* ============================================================================
  */

/*

* ============================================================================
* SAFE-RUST CONTRACT
* ============================================================================
* 
* This grammar contains no embedded Rust actions.
* 
* Consequently:
* 
* no unsafe Rust
* no unsafe parser actions
* no raw pointers
* no FFI implementation
* no target-specific code
* 
* are required by this grammar.
* 
* The generated parser/lexer integration must remain compatible with:
* 
* Rust 1.97
* Rust 1.97.1
* Edition 2021
* 
* Repository Rust code must remain safe Rust.
* 
* ============================================================================
  */

/*

* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* The grammar must represent arbitrary-length capture sequences structurally
* rather than enumerating capture positions.
* 
* No grammar-level construct may encode:
* 
* capture0
* capture1
* capture2
* ...
* 
* or any equivalent fixed-capacity mechanism.
* 
* Scaling is therefore:
* 
* source representation
*     ->
* parser representation
*     ->
* semantic representation
* 
* subject only to implementation resources.
* 
* A target may reject a program because it lacks sufficient resources or
* capabilities, but the grammar must not reject it merely because it exceeds
* an arbitrary universal machine-size constant.
* 
* ============================================================================
  */

/*

* ============================================================================
* NO HARDWARE KNOWLEDGE
* ============================================================================
* 
* This grammar must not inspect or depend on:
* 
* processor count
* memory size
* GPU availability
* FPGA availability
* QPU availability
* accelerator availability
* network topology
* storage capacity
* deployment state
* runtime state
* 
* Those belong to resource negotiation and target realization.
* 
* ============================================================================
  */

/*

* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* POSITIVE TESTS
* ---
* 
* The expression integration must be tested with canonical capture forms
* accepted by grammar/functions/closures.g4, including:
* 
* capture { x }
* capture { x, y }
* capture { x, y, z }
* capture { x, y, }
* capture { move x }
* capture { ref x }
* capture { mut x }
* capture { move x as y }
* capture { ref x as y }
* capture { mut x as y }
* capture { * }
* capture { move * }
* 
* These tests verify integration rather than reimplementing capture syntax.
* 
* 
* LAMBDA BOUNDARY TESTS
* ---
* 
* Verify canonical closure capture can participate in lambda composition.
* 
* Test:
* 
* capture { x } |value| value
* 
* and equivalent forms defined by the canonical lambda specification.
* 
* 
* MULTIPLE CAPTURE TESTS
* ---
* 
* Verify:
* 
* capture { x, y, z } |value| value
* 
* preserves capture ordering.
* 
* 
* NESTING TESTS
* ---
* 
* Verify closure/lambda nesting does not create recursive grammar cycles.
* 
* Examples include:
* 
* capture { x } |a| {
*     capture { a } |b| b
* }
* 
* where supported by the canonical expression grammar.
* 
* 
* CROSS-DOMAIN TESTS
* ---
* 
* Verify that captured values can be syntactically ordinary bindings
* regardless of their eventual semantic domain:
* 
* classical value
* tensor value
* data value
* AI/model value
* probabilistic value
* quantum value
* hybrid value
* HDL value
* resource value
* capability value
* 
* No domain-specific closure syntax should be required.
* 
* 
* NEGATIVE TESTS
* ---
* 
* Verify malformed syntax is rejected by the canonical closure grammar,
* including malformed delimiters and malformed capture entries.
* 
* Semantic failures must be tested separately from parser failures.
* 
* 
* DETERMINISM TESTS
* ---
* 
* The same source under the same grammar/lexer configuration must produce
* equivalent parser structure and source spans.
* 
* 
* SCALABILITY TESTS
* ---
* 
* Test capture lists of increasing size without introducing language-level
* capacity constants.
* 
* Test nested closure expressions within available compiler resources.
* 
* The tests must distinguish:
* 
* language restriction
* 
* from:
* 
* implementation resource exhaustion.
* 
* 
* COMPATIBILITY TESTS
* ---
* 
* Verify that:
* 
* grammar/functions/closures.g4
* 
* remains the sole capture syntax authority.
* 
* Verify that the expression adapter introduces no new capture spelling.
* 
* ============================================================================
  */

/*

* ============================================================================
* FILE COMPLETION CRITERIA
* ============================================================================
* 
* This file is DONE when:
* 
* [x] It is a parser grammar.
* 
* [x] It uses tokenVocab=ZamaniLexer.
* 
* [x] It imports the canonical Closures grammar.
* 
* [x] It defines no lexer rules.
* 
* [x] It defines no duplicate closure-capture rules.
* 
* [x] It exposes a narrow expression-layer integration boundary.
* 
* [x] It does not import the complete expression grammar.
* 
* [x] It cannot create an expression/closure circular dependency by itself.
* 
* [x] It introduces no domain-specific closure syntax.
* 
* [x] It introduces no quantum-specific closure syntax.
* 
* [x] It introduces no hardware-specific closure syntax.
* 
* [x] It introduces no physical resource assumptions.
* 
* [x] It introduces no universal capacity constants.
* 
* [x] It introduces no semantic actions.
* 
* [x] It introduces no Rust code.
* 
* [x] It requires no unsafe Rust.
* 
* [x] It preserves source-level capture information.
* 
* [x] It maps to the existing closure AST architecture.
* 
* [x] It preserves the canonical quantum::ir boundary.
* 
* [x] It remains compatible with POCO-REAF.
* 
* [ ] ANTLR generation succeeds with the repository's actual build configuration.
* 
* [ ] Expression composition consumes this boundary exactly once.
* 
* [ ] Lambda composition consumes the canonical capture rule without copying it.
* 
* [ ] Hand-written Rust parser conformance tests agree with the grammar.
* 
* [ ] AST conformance tests pass.
* 
* [ ] Positive tests pass.
* 
* [ ] Negative tests pass.
* 
* [ ] Boundary tests pass.
* 
* [ ] Scalability tests pass within available implementation resources.
* 
* [ ] Determinism tests pass.
* 
* [ ] Cross-domain tests pass.
* 
* ============================================================================
* FINAL OWNERSHIP SUMMARY
* ============================================================================
* 
* THIS FILE OWNS:
* 
* expression-level closure integration
* expression-facing closure boundary
* 
* THIS FILE DOES NOT OWN:
* 
* capture syntax
* lexer vocabulary
* identifiers
* lambda syntax
* function syntax
* function types
* ownership semantics
* borrowing
* lifetimes
* effects
* capabilities
* resources
* contracts
* policies
* provenance
* concurrency semantics
* distributed semantics
* quantum semantics
* HDL semantics
* AI semantics
* classical IR
* quantum::ir
* lowering
* routing
* scheduling
* resilience
* ZQN
* HAL
* target realization
* 
* The canonical architecture is therefore:
* 
* functions/closures.g4
*         |
*         v
* expressions/closure.g4
*         |
*         +----------------+
*         |                |
*         v                v
*      lambda          expression
*         |                |
*         +-------+--------+
*                 |
*                 v
*            domain-neutral
*                 AST
*                 |
*                 v
*             semantics
*                 |
*          +------+------+
*          |             |
*          v             v
*     classical IR   quantum::ir
*          |             |
*          +------+------+
*                 |
*                 v
*           target-neutral
*             optimization
*                 |
*                 v
*             realization
* 
* ============================================================================
  */