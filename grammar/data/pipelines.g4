/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* FILE
* ---
* grammar/data/pipelines.g4
* 
* GRAMMAR
* ---
* ZamaniDataPipelinesParser
* 
* STATUS
* ---
* CANONICAL DATA-PIPELINE SYNTAX
* 
* IMPLEMENTATION BASELINE
* ---
* Rust 2021
* Rust 1.97 / Rust 1.97.1
* Safe Rust only; no unsafe Rust required by this grammar contract.
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file is the SINGLE SYNTAX OWNER for DATA PIPELINE CONSTRUCTS.
* 
* A data pipeline is a target-independent composition of:
* 
* data sources
* data sinks
* transformations
* queries
* computations
* dependencies
* contracts
* requirements
* capabilities
* constraints
* preferences
* hints
* provenance
* checkpoint intent
* materialization intent
* distribution intent
* 
* A pipeline may eventually execute as:
* 
* local computation
* multicore computation
* vectorized computation
* GPU computation
* FPGA computation
* accelerator computation
* quantum/classical hybrid computation
* distributed computation
* HPC computation
* cloud computation
* embedded computation
* edge computation
* hardware/software co-design
* future computational substrates
* 
* The grammar describes WHAT the pipeline means.
* 
* It does not prescribe:
* 
* WHERE it executes
* HOW it executes
* WHICH machine executes it
* WHICH CPU executes it
* WHICH GPU executes it
* WHICH FPGA executes it
* WHICH QPU executes it
* WHICH node executes it
* WHICH memory bank stores it
* WHICH network carries it
* WHICH database implements it
* WHICH scheduler realizes it
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* Pipeline syntax is deliberately open-ended.
* 
* A pipeline can grow with the amount of available computation and data without
* changing the source language merely because the target machine is larger.
* 
* The grammar therefore contains NO universal limits for:
* 
* pipeline stages
* inputs
* outputs
* dependencies
* transformations
* records
* rows
* columns
* elements
* partitions
* replicas
* workers
* threads
* processes
* devices
* nodes
* GPUs
* CPUs
* FPGAs
* QPUs
* accelerators
* memory
* storage
* tensor rank
* tensor dimensions
* 
* Practical limits are implementation/resource limits, never language-level
* grammar limits.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* dataPipelineConstruct
* dataPipelineDeclaration
* dataPipelineInvocation
* dataPipelineReference
* dataPipelineExpression
* dataPipelineBody
* dataPipelineMember
* dataPipelineStage
* dataPipelineInput
* dataPipelineOutput
* dataPipelineDependency
* dataPipelineContract
* dataPipelineRequirement
* dataPipelineConstraint
* dataPipelinePreference
* dataPipelineHint
* dataPipelinePolicy
* dataPipelineMetadata
* dataPipelineCheckpoint
* dataPipelineMaterialization
* dataPipelinePartitioning
* dataPipelineDistribution
* dataPipelineReplication
* dataPipelineProvenance
* dataPipelineStageInvocation
* dataPipelineComposition
* 
* THIS FILE DOES NOT OWN:
* 
* identifiers
* lexer rules
* keywords
* general expressions
* general types
* functions
* blocks
* ordinary statements
* schemas
* records
* collections
* streams
* transformations
* queries
* serialization
* hardware
* resource discovery
* scheduling
* routing
* optimization
* runtime execution
* quantum IR
* QEC
* ZQN
* HAL
* 
* Those responsibilities remain owned by their canonical grammar/domain
* components and downstream compiler/runtime layers.
* 
* ============================================================================
* SINGLE-AUTHORITY RULE
* ============================================================================
* 
* Existing grammar/data/data.g4 currently contains pipeline rules such as:
* 
* dataPipelineDecl
* dataPipelineMember
* dataPipelineInput
* dataPipelineOutput
* dataPipelineStage
* dataPipelinePolicy
* dataPipelineRequirement
* dataPipelineConstraint
* dataPipelineStmt
* dataPipelineExpression
* 
* Those rules must eventually delegate to this file instead of remaining as
* a second pipeline syntax authority.
* 
* The intended ownership is:
* 
* grammar/data/pipelines.g4
*         |
*         +--> canonical data-pipeline syntax
* 
* grammar/data/data.g4
*         |
*         +--> data-domain composition/adapter
* 
* Therefore data.g4 should ultimately expose:
* 
* dataPipelineDeclaration
* dataPipelineExpression
* 
* through delegation rather than reimplementing their internals.
* 
* Existing public rule names should be preserved through compatibility
* adapters where required by the repository migration.
* 
* ============================================================================
* DEPENDENCY DIRECTION
* ============================================================================
* 
* The dependency direction is:
* 
* ZamaniLexer
*      |
*      v
* core lexical/parser contracts
*      |
*      +--------------------+
*      |                    |
*      v                    v
* Expressions            Types
*      |                    |
*      +---------+----------+
*                |
*                v
*      Data Pipelines
*                |
*                v
*         Data Composition
*                |
*                v
*         Semantic Analysis
*                |
*                v
*       Canonical Semantic Model
*                |
*      +---------+----------+
*      |         |          |
*      v         v          v
*   Data/AI  Distributed  Quantum
*                        -> quantum::ir
*                |
*                v
*          Optimization
*                |
*                v
*         Routing/Scheduling
*                |
*                v
*         Resilience/QEC/ZQN
*                |
*                v
*               HAL
*                |
*                v
*        Target realization
* 
* This grammar MUST NOT depend on runtime or compiler implementation.
* 
* ============================================================================
* CANONICAL LEXER
* ============================================================================
* 
* Production parser grammars consume:
* 
* ZamaniLexer
* 
* through:
* 
* tokenVocab = ZamaniLexer;
* 
* This file MUST NOT define lexer rules.
* 
* It MUST NOT introduce a second lexer.
* 
* It MUST NOT define:
* 
* PIPELINE_OPEN
* PIPELINE_CLOSE
* DATA_PIPELINE
* K_PIPELINE
* 
* locally merely to make this file convenient.
* 
* If the canonical lexer does not yet expose a required symbolic token, the
* lexer contract must be updated centrally rather than creating a private
* vocabulary here.
* 
* ============================================================================
* EXPRESSION INTEGRATION
* ============================================================================
* 
* General expressions belong to:
* 
* grammar/expressions/
* 
* This grammar consumes:
* 
* expression
* lambdaExpression
* argumentList
* expressionList
* 
* where provided by the canonical expression grammar.
* 
* Pipeline syntax MUST NOT create:
* 
* pipelineExpressionLanguage
* 
* as a second general expression system.
* 
* ============================================================================
* TYPE INTEGRATION
* ============================================================================
* 
* General types belong to:
* 
* grammar/types/
* 
* This grammar consumes:
* 
* typeExpression
* 
* Pipeline syntax MUST NOT create a pipeline-specific type system.
* 
* Therefore:
* 
* pipeline input type
* pipeline output type
* stage input type
* stage output type
* 
* all use the canonical type system.
* 
* ============================================================================
* TRANSFORMATION INTEGRATION
* ============================================================================
* 
* General data transformation syntax belongs to:
* 
* grammar/data/transformations.g4
* 
* This file consumes the canonical transformation entry point.
* 
* This file does NOT enumerate:
* 
* map
* filter
* reduce
* fold
* scan
* join
* sort
* reshape
* normalize
* aggregate
* 
* as a closed keyword catalogue.
* 
* Transformation names remain extensible.
* 
* ============================================================================
* QUERY INTEGRATION
* ============================================================================
* 
* Query syntax belongs to:
* 
* grammar/data/queries.g4
* 
* Pipeline stages may consume canonical query constructs.
* 
* Query semantics are not reimplemented here.
* 
* ============================================================================
* STREAM INTEGRATION
* ============================================================================
* 
* Stream syntax belongs to:
* 
* grammar/data/streams.g4
* 
* A pipeline can consume or produce streams through ordinary expressions,
* references, types, and stage specifications.
* 
* This file does not create a second stream language.
* 
* ============================================================================
* SERIALIZATION INTEGRATION
* ============================================================================
* 
* Serialization syntax belongs to:
* 
* grammar/data/serialization.g4
* 
* Pipeline boundaries may express serialization intent through a generic stage,
* operation, or contract.
* 
* The grammar does not implement a serializer.
* 
* ============================================================================
* RESOURCE / CAPABILITY INTEGRATION
* ============================================================================
* 
* Pipeline requirements are source-level intent.
* 
* Examples:
* 
* @requires(capability("tensor.compute"));
* @requires(capability("quantum.measurement"));
* @requires(memory >= required_memory);
* @requires(distributed);
* 
* These do not select a machine.
* 
* The grammar preserves the expression.
* 
* Semantic analysis determines its meaning.
* 
* Resource analysis determines feasibility.
* 
* Compilation determines realization.
* 
* Runtime determines actual availability.
* 
* ============================================================================
* REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
* ============================================================================
* 
* These concepts are deliberately represented structurally rather than as a
* closed universal keyword inventory.
* 
* Examples:
* 
* @requires(capability("gpu.compute"));
* @constraint(latency <= budget);
* @prefer(accelerator("quantum"));
* @hint(vectorize);
* 
* The annotation name is syntactic data.
* 
* Semantic analysis determines whether the annotation is a:
* 
* requirement
* capability
* constraint
* preference
* hint
* budget
* policy
* metadata declaration
* future registered category
* 
* This preserves extensibility.
* 
* ============================================================================
* HARDWARE INDEPENDENCE
* ============================================================================
* 
* Pipeline syntax MUST NOT encode universal machine identities or capacities.
* 
* Forbidden as language-level assumptions:
* 
* CPU 0
* GPU 0
* FPGA 0
* QPU 0
* node 0
* device 0
* 32-bit register
* 64 GB memory
* 24 GB VRAM
* 16 nodes
* 8 workers
* 32 threads
* 
* A source program MAY contain a number as program data or an explicit semantic
* requirement.
* 
* Example:
* 
* requires workers >= desired_workers
* 
* is semantic intent.
* 
* It is not a grammar implementation limit.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* A pipeline may feed data into quantum computation or consume quantum-derived
* data.
* 
* Example conceptual flow:
* 
* data
*   |
*   v
* classical preparation
*   |
*   v
* quantum computation
*   |
*   v
* measurement
*   |
*   v
* data transformation
* 
* This file does NOT define quantum operations.
* 
* It does NOT define:
* 
* gates
* physical qubits
* coupling maps
* routing
* calibration
* QEC
* ZQN
* 
* Quantum semantics ultimately cross the canonical:
* 
* quantum::ir
* 
* boundary.
* 
* ============================================================================
* HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* A pipeline may describe computation eventually realized by hardware or HDL.
* 
* It must not define:
* 
* wires
* registers
* clock domains
* FPGA resources
* ASIC layout
* physical buses
* physical addresses
* 
* Those belong to:
* 
* grammar/hdl/
* grammar/hardware/
* grammar/resources/
* 
* ============================================================================
* AI INTEGRATION
* ============================================================================
* 
* AI pipelines can use this generic data-pipeline syntax.
* 
* Examples:
* 
* dataset
*   -> preprocessing
*   -> model
*   -> evaluation
* 
* stream
*   -> feature extraction
*   -> inference
*   -> aggregation
* 
* AI-specific model/training/inference syntax remains owned by:
* 
* grammar/ai/
* 
* This grammar does not enumerate AI frameworks or accelerators.
* 
* ============================================================================
* DISTRIBUTED INTEGRATION
* ============================================================================
* 
* A dependency edge represents logical data/control dependency.
* 
* Example:
* 
* prepare -> train;
* 
* does NOT mean:
* 
* machine A -> machine B
* 
* or:
* 
* node 0 -> node 1
* 
* Physical placement, routing, replication and scheduling remain downstream.
* 
* ============================================================================
* PIPELINE MODEL
* ============================================================================
* 
* A pipeline has:
* 
* declaration
* optional type/parameter information
* body
* 
* The body contains an ordered sequence of members.
* 
* Members may describe:
* 
* inputs
* outputs
* stages
* dependencies
* contracts
* requirements
* constraints
* preferences
* hints
* policies
* checkpoint intent
* materialization intent
* partitioning intent
* distribution intent
* replication intent
* provenance
* metadata
* 
* Member order is preserved in the parse tree and frontend AST.
* 
* Semantic analysis determines whether ordering has semantic significance.
* 
* ============================================================================
* ANNOTATION MODEL
* ============================================================================
* 
* Annotation syntax is intentionally generic:
* 
* @name
* @name(...)
* @name = expression
* @name { ... }
* 
* The parser does not need to know every future annotation name.
* 
* This prevents an ever-growing keyword list.
* 
* ============================================================================
* PIPELINE DECLARATION FORMS
* ============================================================================
* 
* Canonical named form:
* 
* pipeline name {
*     ...
* }
* 
* Parameterized form:
* 
* pipeline name<T> {
*     ...
* }
* 
* Typed form:
* 
* pipeline name: PipelineType {
*     ...
* }
* 
* The exact semantic interpretation of generic/type forms belongs downstream.
* 
* ============================================================================
* PIPELINE STAGES
* ============================================================================
* 
* A stage is a named logical computation.
* 
* Canonical form:
* 
* stage preprocess = transform(data);
* 
* The right-hand side is an expression and may reference:
* 
* transformations
* queries
* functions
* models
* tensors
* streams
* collections
* quantum-derived values
* hardware-backed values
* user-defined operations
* 
* The grammar does not determine the implementation.
* 
* ============================================================================
* PIPELINE DEPENDENCIES
* ============================================================================
* 
* Dependencies are explicit.
* 
* Canonical form:
* 
* prepare -> transform;
* 
* Optional dependency labels may be expressed:
* 
* prepare -> transform : data;
* 
* Dependency semantics are validated later.
* 
* In particular, the semantic layer must distinguish:
* 
* valid DAG dependencies
* legal feedback constructs
* illegal dependency cycles
* data dependencies
* control dependencies
* 
* The grammar does not perform graph analysis.
* 
* ============================================================================
* INPUTS / OUTPUTS
* ============================================================================
* 
* Pipeline inputs and outputs use canonical types.
* 
* Example:
* 
* input source: Stream<Record>;
* output result: Collection<Result>;
* 
* The grammar does not define:
* 
* maximum number of inputs
* maximum number of outputs
* fixed storage
* fixed stream size
* 
* ============================================================================
* PIPELINE EXPRESSIONS
* ============================================================================
* 
* A pipeline can be composed using a forward-pipeline operator.
* 
* Conceptually:
* 
* source |> transform |> analyze
* 
* The operator expresses composition.
* 
* It does not imply:
* 
* CPU
* GPU
* thread
* process
* node
* device
* 
* The operation at each stage remains semantically resolved.
* 
* ============================================================================
* OPEN-WORLD OPERATION MODEL
* ============================================================================
* 
* Pipeline stages must not be restricted to a finite list of operation names.
* 
* Valid examples include:
* 
* map(data, |x| f(x))
* filter(data, |x| predicate(x))
* aggregate(data, f)
* custom_transform(data)
* analytics::normalize(data)
* ai::inference(model, data)
* distributed::repartition(data, key)
* quantum::measure(data)
* 
* Whether these names are valid semantic operations is determined downstream.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* This grammar MUST lower into the existing domain-neutral frontend AST.
* 
* It must not introduce a parallel:
* 
* PipelineAst
* DataPipelineAst
* PipelineIR
* DataPipelineIR
* 
* merely because pipeline syntax is convenient to model separately.
* 
* The preferred conceptual mappings are:
* 
* pipeline declaration
*     -> canonical declaration representation
* 
* pipeline stage
*     -> declaration / binding / generic operation representation
* 
* pipeline dependency
*     -> generic structured relationship / semantic metadata
* 
* pipeline expression
*     -> generic expression composition
* 
* pipeline annotation
*     -> canonical attribute/metadata representation
* 
* Exact Rust AST structures remain owned by:
* 
* src/ast/
* 
* The parser grammar must not force an unrelated AST redesign.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis is responsible for:
* 
* name resolution
* stage resolution
* type checking
* input/output compatibility
* transformation resolution
* query resolution
* dependency validation
* cycle validation
* contract checking
* capability checking
* resource checking
* effect checking
* ownership checking
* provenance checking
* reproducibility checking
* determinism checking
* portability checking
* security checking
* distributed semantics
* quantum/classical boundary validation
* hardware intent validation
* 
* The grammar does none of these things.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This file creates NO IR.
* 
* Pipeline semantics are lowered by the compiler into the canonical semantic
* representation appropriate for the contained computation.
* 
* Possible downstream forms include:
* 
* data/computation IR
* classical representation
* distributed representation
* AI semantic representation
* HDL/hardware representation
* quantum::ir
* 
* There is deliberately no:
* 
* pipeline::ir
* 
* introduced here.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* Parsing must depend only on:
* 
* source tokens
* grammar version
* active parser composition
* explicitly configured dialect syntax
* 
* Parsing must not depend on:
* 
* machine size
* runtime state
* hardware availability
* resource discovery
* wall-clock time
* randomness
* filesystem state
* network state
* scheduler state
* 
* ============================================================================
* SECURITY
* ============================================================================
* 
* This grammar contains:
* 
* no actions
* no semantic predicates
* no embedded Rust
* no filesystem operations
* no network operations
* no process execution
* no plugin loading
* no hardware discovery
* no secret access
* 
* A syntactically valid external operation remains data until semantic/runtime
* layers explicitly authorize and execute it.
* 
* ============================================================================
* ERROR / RECOVERY CONTRACT
* ============================================================================
* 
* Exact diagnostic wording belongs to the repository's canonical diagnostics
* system.
* 
* This grammar should expose unambiguous structural errors for:
* 
* missing pipeline name
* missing body
* missing stage name
* missing stage expression
* malformed dependency
* malformed input
* malformed output
* malformed annotation
* malformed contract
* malformed expression
* malformed type
* 
* ============================================================================
* COMPATIBILITY
* ============================================================================
* 
* The repository currently has pipeline-related grammar surfaces in:
* 
* grammar/data/data.g4
* grammar/data/transformations.g4
* grammar/data/queries.g4
* grammar/ai/pipelines.g4
* grammar/concurrency/pipeline.g4
* grammar/hdl/pipelines.g4
* 
* These are different semantic domains and must not become mutually recursive
* copies of the same pipeline language.
* 
* This file owns DATA PIPELINE syntax.
* 
* AI pipeline syntax remains AI-owned.
* 
* Concurrency pipeline syntax remains concurrency-owned.
* 
* HDL pipeline syntax remains HDL-owned.
* 
* Cross-domain composition occurs through shared expressions, types,
* declarations, statements, capabilities and semantic models.
* 
* ============================================================================
* PUBLIC RULES
* ============================================================================
* 
* Stable public entry:
* 
* dataPipelineConstruct
* 
* Canonical declaration:
* 
* dataPipelineDeclaration
* 
* Canonical invocation:
* 
* dataPipelineInvocation
* 
* Canonical expression:
* 
* dataPipelineExpression
* 
* Canonical body:
* 
* dataPipelineBody
* 
* ============================================================================
  */

parser grammar ZamaniDataPipelinesParser;

options {
tokenVocab = ZamaniLexer;
}

import Expressions,
Types;

/*

* ============================================================================
* 1. PUBLIC DATA-PIPELINE ENTRY POINT
* ============================================================================
* 
* No EOF is consumed here.
* 
* The complete-program entry point remains owned by:
* 
* grammar/Zamani.g4
* 
* ============================================================================
  */

dataPipelineConstruct
: dataPipelineDeclaration
| dataPipelineInvocation
| dataPipelineReference
| dataPipelineExpression
;

/*

* ============================================================================
* 2. PIPELINE DECLARATION
* ============================================================================
* 
* Canonical:
* 
* pipeline name {
*     ...
* }
* 
* Optional annotations may precede the declaration.
* 
* The literal word "pipeline" remains parser-level syntax here only if the
* canonical lexer represents it accordingly. The preferred long-term model is
* a centralized keyword token in ZamaniLexer.
* ============================================================================
  */

dataPipelineDeclaration
: dataPipelineAnnotation*
PIPELINE
IDENTIFIER
dataPipelineTypeParameters?
dataPipelineTypeClause?
dataPipelineBody
;

/*

* ============================================================================
* 3. TYPE PARAMETERS
* ============================================================================
* 
* Generic parameterization is intentionally kept structurally small here.
* 
* Full generic/type semantics remain owned by the canonical type system.
* ============================================================================
  */

dataPipelineTypeParameters
: LESS
dataPipelineTypeParameterList
GREATER
;

dataPipelineTypeParameterList
: dataPipelineTypeParameter
(
COMMA
dataPipelineTypeParameter
)*
;

dataPipelineTypeParameter
: IDENTIFIER
| IDENTIFIER
COLON
typeExpression
;

/*

* ============================================================================
* 4. OPTIONAL PIPELINE TYPE
* ============================================================================
  */

dataPipelineTypeClause
: COLON
typeExpression
;

/*

* ============================================================================
* 5. PIPELINE BODY
* ============================================================================
  */

dataPipelineBody
: LBRACE
dataPipelineMember*
RBRACE
;

/*

* ============================================================================
* 6. PIPELINE MEMBER
* ============================================================================
* 
* Members are deliberately explicit at the structural level.
* 
* Future semantic categories can be introduced through annotations without
* requiring a parser keyword for every new feature.
* ============================================================================
  */

dataPipelineMember
: dataPipelineAnnotation*
(
dataPipelineInput
| dataPipelineOutput
| dataPipelineStage
| dataPipelineDependency
| dataPipelineContract
| dataPipelineRequirement
| dataPipelineConstraint
| dataPipelinePreference
| dataPipelineHint
| dataPipelinePolicy
| dataPipelineCheckpoint
| dataPipelineMaterialization
| dataPipelinePartitioning
| dataPipelineDistribution
| dataPipelineReplication
| dataPipelineProvenance
| dataPipelineMetadata
| dataPipelineExpressionMember
)
;

/*

* ============================================================================
* 7. INPUT
* ============================================================================
* 
* Example:
* 
* input source: Stream<Record>;
* 
* The type is canonical Zamani type syntax.
* ============================================================================
  */

dataPipelineInput
: INPUT
IDENTIFIER
COLON
typeExpression
dataPipelineInitializer?
SEMICOLON
;

/*

* ============================================================================
* 8. OUTPUT
* ============================================================================
  */

dataPipelineOutput
: OUTPUT
IDENTIFIER
COLON
typeExpression
dataPipelineInitializer?
SEMICOLON
;

/*

* ============================================================================
* 9. STAGE
* ============================================================================
* 
* Example:
* 
* stage normalize = normalize(input);
* 
* Optional stage signature:
* 
* stage normalize(input: Data) -> Result = normalize(input);
* 
* The expression remains owned by Expressions.
* ============================================================================
  */

dataPipelineStage
: STAGE
IDENTIFIER
dataPipelineStageParameters?
dataPipelineStageReturnType?
ASSIGN
expression
SEMICOLON
;

/*

* ============================================================================
* 10. STAGE PARAMETERS
* ============================================================================
* 
* Stage parameter syntax is intentionally aligned with ordinary parameter
* structure but remains structurally limited here so the file does not create
* a competing function grammar.
* 
* ============================================================================
  */

dataPipelineStageParameters
: LPAREN
dataPipelineStageParameterList?
RPAREN
;

dataPipelineStageParameterList
: dataPipelineStageParameter
(
COMMA
dataPipelineStageParameter
)*
;

dataPipelineStageParameter
: IDENTIFIER
(
COLON
typeExpression
)?
(
ASSIGN
expression
)?
;

dataPipelineStageReturnType
: THIN_ARROW
typeExpression
;

/*

* ============================================================================
* 11. DEPENDENCY
* ============================================================================
* 
* Canonical:
* 
* prepare -> transform;
* 
* Optional semantic label:
* 
* prepare -> transform : data;
* 
* This is a logical dependency.
* 
* It is NOT physical network topology or machine placement.
* ============================================================================
  */

dataPipelineDependency
: dataPipelineReference
THIN_ARROW
dataPipelineReference
dataPipelineDependencyLabel?
SEMICOLON
;

dataPipelineDependencyLabel
: COLON
dataPipelineName
;

/*

* ============================================================================
* 12. PIPELINE REFERENCE
* ============================================================================
  */

dataPipelineReference
: dataPipelineName
;

dataPipelineName
: IDENTIFIER
(
DOUBLE_COLON
IDENTIFIER
)*
;

/*

* ============================================================================
* 13. PIPELINE INVOCATION
* ============================================================================
* 
* Examples:
* 
* process();
* analytics::process(input);
* 
* Pipeline invocation does not imply a specific execution target.
* ============================================================================
  */

dataPipelineInvocation
: dataPipelineReference
LPAREN
argumentList?
RPAREN
dataPipelineInvocationOptions?
;

dataPipelineInvocationOptions
: dataPipelineAnnotation*
;

/*

* ============================================================================
* 14. PIPELINE EXPRESSION
* ============================================================================
* 
* Canonical composition:
* 
* source |> transform |> analyze
* 
* The operation at each stage is resolved semantically.
* 
* ============================================================================
  */

dataPipelineExpression
: dataPipelineExpressionBase
(
PIPE_FORWARD
dataPipelineStageExpression
)+
;

dataPipelineExpressionBase
: expression
;

dataPipelineStageExpression
: expression
;

/*

* ============================================================================
* 15. EXPRESSION MEMBER
* ============================================================================
* 
* Allows a pipeline body to contain an ordinary expression as an explicit
* pipeline-level operation.
* ============================================================================
  */

dataPipelineExpressionMember
: expression
SEMICOLON
;

/*

* ============================================================================
* 16. INITIALIZER
* ============================================================================
  */

dataPipelineInitializer
: ASSIGN
expression
;

/*

* ============================================================================
* 17. CONTRACT
* ============================================================================
* 
* A contract is structural source intent.
* 
* Semantic verification belongs downstream.
* ============================================================================
  */

dataPipelineContract
: CONTRACT
dataPipelineContractName?
LPAREN
expression
RPAREN
SEMICOLON
;

dataPipelineContractName
: IDENTIFIER
;

/*

* ============================================================================
* 18. REQUIREMENT
* ============================================================================
* 
* Examples:
* 
* requires(capability("tensor.compute"));
* requires(qubits >= n);
* 
* No hardware availability is evaluated here.
* ============================================================================
  */

dataPipelineRequirement
: REQUIRES
LPAREN
expression
RPAREN
SEMICOLON
;

/*

* ============================================================================
* 19. CONSTRAINT
* ============================================================================
  */

dataPipelineConstraint
: CONSTRAINT
LPAREN
expression
RPAREN
SEMICOLON
;

/*

* ============================================================================
* 20. PREFERENCE
* ============================================================================
  */

dataPipelinePreference
: PREFER
LPAREN
expression
RPAREN
SEMICOLON
;

/*

* ============================================================================
* 21. HINT
* ============================================================================
  */

dataPipelineHint
: HINT
LPAREN
expression
RPAREN
SEMICOLON
;

/*

* ============================================================================
* 22. POLICY
* ============================================================================
* 
* Policies remain semantic expressions.
* 
* They do not execute during parsing.
* ============================================================================
  */

dataPipelinePolicy
: POLICY
dataPipelineName?
ASSIGN
expression
SEMICOLON
;

/*

* ============================================================================
* 23. CHECKPOINT
* ============================================================================
* 
* Checkpoint intent is deliberately open-ended.
* 
* Example:
* 
* checkpoint(expression);
* 
* Actual persistence/recovery behavior belongs downstream.
* ============================================================================
  */

dataPipelineCheckpoint
: CHECKPOINT
LPAREN
expression?
RPAREN
SEMICOLON
;

/*

* ============================================================================
* 24. MATERIALIZATION
* ============================================================================
* 
* Example:
* 
* materialize(expression);
* 
* The grammar does not select RAM, disk, object storage, accelerator memory,
* distributed storage, or any other physical medium.
* ============================================================================
  */

dataPipelineMaterialization
: MATERIALIZE
LPAREN
expression
RPAREN
SEMICOLON
;

/*

* ============================================================================
* 25. PARTITIONING
* ============================================================================
* 
* Partitioning expresses logical data partitioning.
* 
* Example:
* 
* partition by key;
* 
* No partition count or machine placement is encoded.
* ============================================================================
  */

dataPipelinePartitioning
: PARTITION
BY
expression
SEMICOLON
;

/*

* ============================================================================
* 26. DISTRIBUTION
* ============================================================================
* 
* Distribution expresses semantic execution/data-distribution intent.
* 
* Example:
* 
* distribute by key;
* 
* It does not select nodes or network topology.
* ============================================================================
  */

dataPipelineDistribution
: DISTRIBUTE
dataPipelineDistributionSpecification?
SEMICOLON
;

dataPipelineDistributionSpecification
: BY
expression
| expression
;

/*

* ============================================================================
* 27. REPLICATION
* ============================================================================
* 
* A replication expression may contain a program-level value.
* 
* Example:
* 
* replicate replicas;
* 
* This does not create a universal replica limit.
* ============================================================================
  */

dataPipelineReplication
: REPLICATE
expression
SEMICOLON
;

/*

* ============================================================================
* 28. PROVENANCE
* ============================================================================
* 
* Provenance remains declarative.
* ============================================================================
  */

dataPipelineProvenance
: PROVENANCE
(
LPAREN
expression
RPAREN
| ASSIGN
expression
)
SEMICOLON
;

/*

* ============================================================================
* 29. METADATA
* ============================================================================
* 
* Generic metadata avoids creating a keyword for every future pipeline
* property.
* ============================================================================
  */

dataPipelineMetadata
: METADATA
(
LPAREN
expression
RPAREN
| ASSIGN
expression
)
SEMICOLON
;

/*

* ============================================================================
* 30. GENERIC PIPELINE ANNOTATIONS
* ============================================================================
* 
* The annotation grammar is intentionally open-world.
* 
* Examples:
* 
* @pipeline
* @stage
* @requires(capability("gpu.compute"))
* @constraint(latency <= budget)
* @preference(accelerator("quantum"))
* @future_feature(...)
* 
* Annotation names remain ordinary identifiers.
* ============================================================================
  */

dataPipelineAnnotation
: AT
IDENTIFIER
dataPipelineAnnotationPayload?
;

dataPipelineAnnotationPayload
: LPAREN
argumentList?
RPAREN
| ASSIGN
expression
| LBRACE
dataPipelineAnnotationMember*
RBRACE
;

dataPipelineAnnotationMember
: dataPipelineAnnotation
| IDENTIFIER
ASSIGN
expression
SEMICOLON
;

/*

* ============================================================================
* 31. OPTIONAL PIPELINE BLOCK COMPOSITION
* ============================================================================
* 
* This permits a pipeline expression to be embedded as a structured expression
* without introducing a second block language.
* ============================================================================
  */

dataPipelineBlockExpression
: LBRACE
dataPipelineExpressionMember*
RBRACE
;

/*

* ============================================================================
* 32. PIPELINE CALL ARGUMENTS
* ============================================================================
* 
* This compatibility rule exists so pipeline invocation continues to use the
* canonical expression argument model.
* ============================================================================
  */

dataPipelineArgumentList
: argumentList
;

/*

* ============================================================================
* 33. LEGACY COMPATIBILITY ADAPTERS
* ============================================================================
* 
* Existing data.g4 currently uses names including:
* 
* dataPipelineDecl
* dataPipelineStmt
* dataPipelineExpression
* 
* The canonical implementation should migrate those rules to the names above.
* 
* During migration, these adapters MAY be retained temporarily.
* 
* They must contain no independent syntax.
* ============================================================================
  */

dataPipelineDecl
: dataPipelineDeclaration
;

dataPipelineStmt
: dataPipelineConstruct
SEMICOLON?
;

/*

* ============================================================================
* 34. SEMANTIC EXTENSION BOUNDARY
* ============================================================================
* 
* Future data pipeline features must enter through one of these mechanisms:
* 
* canonical stage expression
* canonical pipeline member
* annotation
* canonical expression
* canonical type
* dialect extension
* 
* A future transformation MUST NOT require this file to enumerate the
* transformation name merely because it is new.
* 
* ============================================================================
  */

/*

* ============================================================================
* 35. SOURCE-SPAN CONTRACT
* ============================================================================
* 
* Every syntactic construct must remain source-locatable.
* 
* The frontend must preserve source spans for:
* 
* pipeline declaration
* pipeline name
* pipeline parameters
* inputs
* outputs
* stages
* dependencies
* annotations
* contracts
* requirements
* constraints
* preferences
* hints
* policies
* checkpoint declarations
* materialization declarations
* partitioning declarations
* distribution declarations
* replication declarations
* provenance
* expressions
* 
* This grammar does not manufacture source spans itself.
* 
* The parser/frontend infrastructure owns span construction.
* 
* ============================================================================
  */

/*

* ============================================================================
* 36. SCALABILITY CONTRACT
* ============================================================================
* 
* Grammar repetition is intentionally open:
* 
* dataPipelineMember*
* dataPipelineAnnotation*
* dependency lists
* parameter lists
* 
* There are no bounded alternatives such as:
* 
* stage1
* stage2
* stage3
* 
* and no universal constants such as:
* 
* MAX_PIPELINE_STAGES
* MAX_INPUTS
* MAX_OUTPUTS
* MAX_EDGES
* MAX_WORKERS
* MAX_NODES
* 
* Resource exhaustion is an implementation concern and must never be converted
* into a language semantic ceiling.
* 
* ============================================================================
  */

/*

* ============================================================================
* 37. DETERMINISTIC PARSING CONTRACT
* ============================================================================
* 
* No rule in this file:
* 
* probes hardware
* checks resources
* executes code
* evaluates expressions
* accesses files
* accesses networks
* reads environment state
* 
* Parsing therefore remains deterministic.
* 
* ============================================================================
  */

/*

* ============================================================================
* 38. NO UNSAFE RUST CONTRACT
* ============================================================================
* 
* This grammar contains no Rust actions.
* 
* Generated parser integration must remain compatible with:
* 
* Rust 1.97
* Rust 1.97.1
* Rust 2021
* 
* Zamani-owned Rust integration must not require:
* 
* unsafe
* 
* for this grammar.
* 
* Memory/resource exhaustion handling belongs to the implementation and test
* harness and must not be expressed as grammar-level artificial limits.
* 
* ============================================================================
  */

/*

* ============================================================================
* 39. HARD-CODING AUDIT
* ============================================================================
* 
* Forbidden universal capacities:
* 
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_NODES
* MAX_MEMORY
* MAX_THREADS
* MAX_TENSOR_RANK
* MAX_REGISTER_WIDTH
* MAX_NETWORK_SIZE
* MAX_DEVICE_COUNT
* 
* Pipeline-specific forbidden capacities:
* 
* MAX_PIPELINE_STAGES
* MAX_PIPELINE_INPUTS
* MAX_PIPELINE_OUTPUTS
* MAX_PIPELINE_EDGES
* MAX_PIPELINE_WORKERS
* MAX_PIPELINE_BATCHES
* MAX_PIPELINE_STREAMS
* 
* None are encoded by this grammar.
* 
* ============================================================================
  */

/*

* ============================================================================
* 40. TEST CONTRACT
* ============================================================================
* 
* POSITIVE:
* 
* pipeline ingest {
*     input source: Stream<Record>;
*     output result: Collection<Result>;
*     stage clean = clean(source);
*     stage analyze = analyze(clean);
*     clean -> analyze;
* }
* 
* pipeline generic<T> {
*     input source: T;
*     stage process = transform(source);
* }
* 
* pipeline distributed_data {
*     input source: Stream<Data>;
*     stage prepare = prepare(source);
*     stage compute = compute(prepare);
*     prepare -> compute;
*     @requires(capability("distributed.data"));
* }
* 
* pipeline quantum_data {
*     input source: Data;
*     stage prepare = prepare(source);
*     stage measure = quantum::measure(prepare);
*     prepare -> measure;
*     @requires(capability("quantum.measurement"));
* }
* 
* pipeline hardware_data {
*     input source: Data;
*     stage compute = accelerator::compute(source);
*     @requires(capability("tensor.compute"));
* }
* 
* source |> transform |> analyze;
* 
* analytics::pipeline(source);
* 
* pipeline p {
*     input source: Data;
*     output result: Data;
*     requires(capability("gpu.compute"));
*     constraint(latency <= budget);
*     prefer(accelerator("gpu"));
*     hint(vectorize);
* }
* 
* SCALABILITY:
* 
* generated pipelines with:
* 
*     many inputs
*     many outputs
*     many stages
*     many dependencies
*     deeply nested expressions
*     large generic parameter lists
* 
* must not encounter a grammar-defined cardinality limit.
* 
* NEGATIVE STRUCTURAL CASES:
* 
* pipeline
* 
* pipeline name
* 
* pipeline name {
* 
* pipeline name {
*     input : Data;
* }
* 
* pipeline name {
*     stage = expression;
* }
* 
* pipeline name {
*     input value:
* }
* 
* pipeline name {
*     output value:
* }
* 
* pipeline name {
*     stage process = ;
* }
* 
* pipeline name {
*     a -> ;
* }
* 
* Exact diagnostic wording belongs to the canonical diagnostics subsystem.
* 
* ============================================================================
  */

/*

* ============================================================================
* 41. CROSS-DOMAIN TEST CONTRACT
* ============================================================================
* 
* Classical:
* 
* pipeline classical {
*     input x: Tensor<Scalar>;
*     stage y = classical::compute(x);
* }
* 
* AI:
* 
* pipeline training {
*     input data: Dataset;
*     stage prepared = preprocess(data);
*     stage model = train(prepared);
*     stage result = evaluate(model, prepared);
*     prepared -> model;
*     model -> result;
* }
* 
* Quantum/classical:
* 
* pipeline hybrid {
*     input parameters: Data;
*     stage quantum_result = quantum::execute(parameters);
*     stage analysis = analyze(quantum_result);
*     quantum_result -> analysis;
* }
* 
* HDL/hardware:
* 
* pipeline accelerator {
*     input data: Tensor<Data>;
*     stage result = hardware::compute(data);
* }
* 
* Distributed:
* 
* pipeline cluster {
*     input data: Stream<Data>;
*     stage prepared = prepare(data);
*     stage result = compute(prepared);
*     prepared -> result;
*     @requires(capability("distributed.data"));
* }
* 
* These tests verify shared semantic composition, not hardware realization.
* 
* ============================================================================
  */

/*

* ============================================================================
* 42. INTEGRATION CHECKLIST
* ============================================================================
* 
* This file is complete when:
* 
* [x] It is a parser grammar.
* [x] It uses the canonical ZamaniLexer vocabulary.
* [x] It does not define lexer rules.
* [x] It does not define a second expression language.
* [x] It does not define a second type language.
* [x] It does not define a second function language.
* [x] It does not define a second statement language.
* [x] It does not define a pipeline IR.
* [x] It does not define a data-specific hardware model.
* [x] It does not enumerate transformation algorithms.
* [x] It does not enumerate AI frameworks.
* [x] It does not enumerate quantum gates.
* [x] It does not encode QEC.
* [x] It does not encode ZQN.
* [x] It does not encode HAL behavior.
* [x] It does not encode physical topology.
* [x] It does not encode machine capacities.
* [x] It supports arbitrary logical pipeline cardinality.
* [x] It supports typed inputs.
* [x] It supports typed outputs.
* [x] It supports named stages.
* [x] It supports stage parameters.
* [x] It supports stage return types.
* [x] It supports logical dependencies.
* [x] It supports contracts.
* [x] It supports requirements.
* [x] It supports constraints.
* [x] It supports preferences.
* [x] It supports hints.
* [x] It supports checkpoint intent.
* [x] It supports materialization intent.
* [x] It supports partitioning intent.
* [x] It supports distribution intent.
* [x] It supports replication intent.
* [x] It supports provenance.
* [x] It supports generic annotations.
* [x] It supports open-ended operation names through expressions/references.
* [x] It supports classical computation.
* [x] It supports AI/dataflow composition.
* [x] It supports distributed computation.
* [x] It supports quantum/classical dataflow.
* [x] It supports hardware/accelerator intent through semantic expressions.
* [x] It preserves source order.
* [x] It is compatible with source-span preservation.
* [x] It performs no semantic evaluation.
* [x] It performs no resource discovery.
* [x] It performs no target selection.
* [x] It performs no runtime execution.
* [x] It requires no unsafe Rust.
* 
* ============================================================================
* 43. REQUIRED DOWNSTREAM INTEGRATION
* ============================================================================
* 
* After this file is accepted, the following repository integration should be
* performed ONCE, using this file as the syntax authority:
* 
* 1. grammar/data/data.g4
* 
* Remove its independent pipeline implementation and delegate to:
* 
*    dataPipelineDeclaration
*    dataPipelineExpression
* 
* Compatibility aliases may remain temporarily.
* 
* 2. grammar/data/README.md
* 
* Change the pipeline ownership matrix so:
* 
*    pipelines.g4 -> canonical pipeline syntax
* 
* and:
* 
*    data.g4 -> data-domain composition
* 
* 3. grammar/data/transformations.g4
* 
* Ensure transformation composition consumes canonical expressions and does
* not recursively depend on pipelines.g4.
* 
* 4. grammar/data/queries.g4
* 
* Ensure query stages can be consumed by pipeline expressions without
* creating a query/pipeline grammar cycle.
* 
* 5. grammar/ai/pipelines.g4
* 
* Keep AI pipeline semantics AI-owned. It may consume the generic pipeline
* model through the canonical composition boundary but must not duplicate
* data-pipeline syntax.
* 
* 6. grammar/concurrency/pipeline.g4
* 
* Keep concurrency pipeline semantics concurrency-owned. Do not merge its
* scheduling/task semantics into data pipelines.
* 
* 7. grammar/hdl/pipelines.g4
* 
* Keep HDL pipeline semantics HDL-owned. Data pipelines may reference
* hardware-capability intent, but must not own hardware syntax.
* 
* 8. grammar/Zamani.g4
* 
* No direct data-pipeline implementation should be added here.
* 
* The root only composes the canonical parser.
* 
* 9. grammar/antlr/ZamaniParser.g4
* 
* The parser composition hierarchy should expose the data pipeline entry
* point exactly once.
* 
* 10. src/lexer.rs
* 
* Verify that the actual Rust lexer emits the canonical tokens required by
* this grammar, especially:
* 
*     PIPELINE
*     INPUT
*     OUTPUT
*     STAGE
*     CONTRACT
*     REQUIRES
*     CONSTRAINT
*     PREFER
*     HINT
*     POLICY
*     CHECKPOINT
*     MATERIALIZE
*     PARTITION
*     BY
*     DISTRIBUTE
*     REPLICATE
*     PROVENANCE
*     METADATA
*     PIPE_FORWARD
* 
* If the current lexer instead emits identifiers for any of these names,
* the lexer vocabulary must be normalized centrally rather than adding
* another lexer implementation to this file.
* 
* 11. src/parser.rs
* 
* Rust parser conformance must map the accepted pipeline syntax into the
* existing domain-neutral AST.
* 
* It must not create a second pipeline AST or IR merely to match this
* grammar.
* 
* 12. src/ast/
* 
* Preserve the existing domain-neutral AST architecture.
* 
* Pipeline syntax must lower through generic declarations, expressions,
* attributes/metadata and relationships as appropriate.
* 
* 13. semantic analysis
* 
* Resolve:
* 
*     pipeline names
*     stage names
*     types
*     transformations
*     queries
*     requirements
*     capabilities
*     constraints
*     preferences
*     dependencies
*     provenance
* 
* 14. canonical IR
* 
* Do not introduce pipeline::ir.
* 
* Lower pipeline computation into the canonical semantic/IR architecture.
* 
* If a pipeline contains quantum computation, the quantum path remains:
* 
*     semantic model -> quantum::ir
* 
* 15. compiler
* 
* Pipeline realization remains target-independent until lowering.
* 
* 16. runtime
* 
* Runtime performs actual:
* 
*     resource acquisition
*     placement
*     scheduling
*     execution
*     checkpointing
*     recovery
*     communication
* 
* 17. validation
* 
* Add grammar validation for:
* 
*     duplicate rules
*     ambiguous alternatives
*     undefined token references
*     undefined imported rules
*     grammar cycles
*     hard-coded capacities
*     unreachable rules
* 
* ============================================================================
* 44. FINAL ARCHITECTURAL GUARANTEE
* ============================================================================
* 
* The finished data pipeline path is:
* 
* Zamani source
*      |
*      v
* canonical lexer
*      |
*      v
* canonical parser
*      |
*      v
* dataPipelineConstruct
*      |
*      v
* domain-neutral AST
*      |
*      v
* semantic analysis
*      |
*      v
* canonical semantic model
*      |
*      +-----------------------+
*      |                       |
*      v                       v
*  data/classical          quantum::ir
*      |                       |
*      +-----------+-----------+
*                  |
*                  v
*            optimization
*                  |
*                  v
*            routing/scheduling
*                  |
*                  v
*            resilience/QEC/ZQN
*                  |
*                  v
*                 HAL
*                  |
*                  v
*           target realization
* 
* Therefore the source program describes portable computation rather than
* today's machine.
* 
* The pipeline can scale from:
* 
* one value
* one record
* one stream
* one processor
* one accelerator
* one quantum device
* one node
* 
* through larger available resource configurations without the grammar
* establishing an artificial upper bound.
* 
* ============================================================================
  */