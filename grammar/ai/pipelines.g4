/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/pipelines.g4
 *
 * Grammar:
 *     AIPipelines
 *
 * Status:
 *     CANONICAL AI / DATAFLOW / COMPUTATION-PIPELINE LEAF GRAMMAR
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the source-level syntax for portable computational pipelines.
 *
 * A pipeline is a target-independent composition of:
 *
 *     computation
 *     data flow
 *     dependencies
 *     control/dataflow regions
 *     semantic requirements
 *     capabilities
 *     constraints
 *     preferences
 *     metadata
 *     provenance
 *     reproducibility intent
 *     checkpoint intent
 *     extensible operations
 *
 * A pipeline may contain computation that is eventually realized as:
 *
 *     classical computation
 *     AI/ML computation
 *     tensor computation
 *     quantum computation
 *     hybrid computation
 *     HDL/hardware computation
 *     accelerator computation
 *     distributed computation
 *     networking computation
 *     security computation
 *     future computational domains
 *
 * This grammar deliberately does NOT decide the physical realization.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Pipeline source describes WHAT the computation means.
 *
 * Later compiler/runtime layers determine:
 *
 *     WHERE
 *     WHEN
 *     HOW
 *     ON WHICH TARGET
 *     USING WHICH RESOURCE
 *     USING WHICH IMPLEMENTATION
 *
 * This grammar therefore MUST NOT encode:
 *
 *     MAX_STAGES
 *     MAX_WORKERS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_ACCELERATORS
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_PIPELINE_DEPTH
 *     MAX_STREAMS
 *     MAX_BATCH_SIZE
 *     MAX_INPUTS
 *     MAX_OUTPUTS
 *     MAX_EDGES
 *
 * There is intentionally no grammar-level machine-size ceiling.
 *
 * Practical limits belong to:
 *
 *     parser implementation
 *     compiler resources
 *     runtime resources
 *     target capabilities
 *     deployment policy
 *     resource availability
 *
 * Such limits MUST NOT become Zamani language semantics.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     pipelineConstruct                 <-- THIS FILE
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *     +----+----------------------+----------------------+
 *     |                           |                      |
 *     v                           v                      v
 * resource/capability        type/effect            dependency
 * analysis                   analysis               analysis
 *     |                           |                      |
 *     +---------------------------+----------------------+
 *                                 |
 *                                 v
 *                       canonical semantic model
 *                                 |
 *                    +------------+-------------+
 *                    |            |             |
 *                    v            v             v
 *                classical    quantum::ir   HDL/hardware
 *                    |            |             |
 *                    +------------+-------------+
 *                                 |
 *                           optimization
 *                                 |
 *                       routing / scheduling
 *                                 |
 *                         resilience / QEC
 *                                 |
 *                                ZQN
 *                                 |
 *                                HAL
 *                                 |
 *                         target realization
 *
 * This file creates no IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     pipelineConstruct
 *     pipelineDeclaration
 *     pipelineInvocation
 *     pipelineReference
 *     pipelineRegion
 *     pipelineBody
 *     pipelineMember
 *     pipelineAnnotatedMember
 *     pipeline annotation payload structure
 *     pipeline dependency-edge structure
 *     pipeline argument structure
 *     pipeline nested regions
 *     pipeline extensibility boundaries
 *     pipeline compatibility helper rules
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexer tokens
 *     identifier syntax
 *     general expression syntax
 *     general type syntax
 *     ordinary statement syntax
 *     model semantics
 *     dataset semantics
 *     tensor semantics
 *     training algorithms
 *     inference algorithms
 *     differentiation algorithms
 *     quantum operations
 *     quantum IR
 *     QEC
 *     ZQN
 *     HDL implementation
 *     hardware discovery
 *     resource allocation
 *     scheduling
 *     routing
 *     optimization
 *     runtime execution
 *     physical deployment
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer provides:
 *
 *     AT
 *     IDENTIFIER
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     COLON
 *     SEMICOLON
 *     ASSIGN
 *     DOT
 *     DOUBLE_COLON
 *     THIN_ARROW
 *
 * Annotation names are NOT lexer keywords.
 *
 * For example:
 *
 *     @pipeline
 *     @stage
 *     @model
 *     @dataset
 *     @quantum
 *     @requires
 *     @capability
 *     @constraint
 *     @preference
 *
 * are lexically represented as:
 *
 *     AT IDENTIFIER
 *
 * Their semantic meaning is determined downstream.
 *
 * This avoids a closed AI vocabulary and preserves future extensibility.
 *
 * ============================================================================
 * IMPORTANT LEXICAL CORRECTION
 * ============================================================================
 *
 * The previous version used:
 *
 *     NANO_ANNOTATION
 *
 * That is NOT used here.
 *
 * The canonical annotation boundary is:
 *
 *     AT
 *
 * followed by an ordinary:
 *
 *     IDENTIFIER
 *
 * This keeps pipelines consistent with the canonical annotation architecture.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Type syntax belongs to:
 *
 *     Types
 *
 * This grammar consumes:
 *
 *     typeExpression
 *
 * It does not create:
 *
 *     PipelineType
 *     StageType
 *     ModelType
 *     DeviceType
 *     TensorType
 *
 * as competing type systems.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Expression syntax belongs to:
 *
 *     Expressions
 *
 * This grammar consumes:
 *
 *     expression
 *
 * Pipeline expressions may therefore represent:
 *
 *     values
 *     symbolic values
 *     tensor values
 *     model values
 *     dataset values
 *     quantum-derived values
 *     hardware-backed values
 *     resource quantities
 *     capability expressions
 *     constraints
 *     predicates
 *     schedules
 *     batch sizes
 *     window sizes
 *     stream parameters
 *     configuration values
 *
 * No pipeline-specific expression language is created.
 *
 * ============================================================================
 * STATEMENT CONTRACT
 * ============================================================================
 *
 * Ordinary Zamani statements belong to:
 *
 *     Statements
 *
 * Pipeline bodies may contain canonical statements.
 *
 * This grammar imports Statements rather than inventing:
 *
 *     pipelineIf
 *     pipelineLoop
 *     pipelineReturn
 *     pipelineLet
 *     pipelineMatch
 *
 * or another second statement language.
 *
 * ============================================================================
 * DETERMINISTIC STRUCTURAL MODEL
 * ============================================================================
 *
 * Pipeline annotations are intentionally parsed structurally.
 *
 * After:
 *
 *     AT IDENTIFIER
 *
 * the next token determines the structural form:
 *
 *     LPAREN       -> argument/directive form
 *     IDENTIFIER  -> named form
 *     ASSIGN      -> expression-valued form
 *     LBRACE      -> region form
 *     SEMICOLON   -> empty directive
 *
 * A dependency edge has an additional explicit:
 *
 *     THIN_ARROW
 *
 * marker.
 *
 * This means the grammar does not need semantic predicates or a list of
 * annotation keywords to decide which rule applies.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / CONSTRAINT / PREFERENCE
 * ============================================================================
 *
 * These concepts are deliberately NOT separate parser keywords.
 *
 * The parser accepts generic annotation structures such as:
 *
 *     @requires(capability("tensor.compute"));
 *     @requires(qubits >= n);
 *     @capability("quantum.measurement");
 *     @constraint(latency <= budget);
 *     @preference(accelerator("quantum"));
 *
 * Semantic analysis determines whether an annotation is:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     hint
 *     budget
 *     policy
 *     metadata
 *     another registered semantic category
 *
 * This prevents the grammar from becoming a fixed dictionary of AI concepts.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * This file MUST NOT encode:
 *
 *     GPU 0
 *     CPU 0
 *     QPU 0
 *     FPGA 0
 *     node 0
 *     device 0
 *     fixed VRAM
 *     fixed RAM
 *     fixed worker count
 *     fixed cluster size
 *     fixed topology
 *     fixed register width
 *     fixed accelerator count
 *
 * A pipeline may state:
 *
 *     @requires(capability("gpu.compute"));
 *
 * but not impose a universal hardware identity.
 *
 * Physical realization is downstream.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * A pipeline may reference quantum computation through:
 *
 *     expressions
 *     types
 *     operation names
 *     capabilities
 *     requirements
 *     nested computation regions
 *
 * This file MUST NOT define:
 *
 *     quantum gates
 *     qubit IDs
 *     physical qubits
 *     coupling maps
 *     quantum topology
 *     calibration
 *     QEC
 *     ZQN
 *     quantum scheduling
 *
 * Quantum semantics ultimately cross:
 *
 *     quantum::ir
 *
 * as the canonical quantum semantic boundary.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware-oriented pipeline stages may be represented through generic
 * annotations, expressions, types and capability requirements.
 *
 * This grammar does not define:
 *
 *     register widths
 *     bus widths
 *     clock frequencies
 *     FPGA resources
 *     ASIC resources
 *     physical wiring
 *     device inventories
 *
 * Those belong to:
 *
 *     hardware/
 *     hdl/
 *     resources/
 *     compile/
 *     execution/
 *
 * ============================================================================
 * AI CONTRACT
 * ============================================================================
 *
 * AI-specific concepts remain extensible.
 *
 * A pipeline may contain:
 *
 *     model execution
 *     dataset transformation
 *     training
 *     inference
 *     tensor computation
 *     differentiation
 *     evaluation
 *     serving
 *     orchestration
 *     accelerator computation
 *
 * The grammar does not enumerate framework-specific names.
 *
 * For example, the grammar does not reserve:
 *
 *     PyTorch
 *     TensorFlow
 *     JAX
 *     CUDA
 *     ROCm
 *     ONNX
 *     XGBoost
 *
 * Such names remain ordinary identifiers or semantic library/dialect entities.
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Pipeline dependency syntax represents logical computation/data dependency.
 *
 * It is NOT:
 *
 *     physical network topology
 *     physical node topology
 *     placement
 *     routing
 *     worker allocation
 *
 * For example:
 *
 *     @edge preprocess -> inference;
 *
 * means semantic dependency/dataflow.
 *
 * It does not mean:
 *
 *     connect machine A to machine B.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve:
 *
 *     annotation name
 *     source spelling
 *     declaration/reference name
 *     argument expressions
 *     types
 *     initializers
 *     nested members
 *     source span
 *     source provenance
 *     member order
 *
 * This grammar does not invent provenance identifiers.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar maps into the existing domain-neutral frontend AST.
 *
 * The preferred representation is:
 *
 *     generic declaration / annotation / region / expression / statement
 *
 * rather than a pipeline-specific parallel AST hierarchy.
 *
 * A semantic pipeline model may be constructed after parsing.
 *
 * No pipeline-specific IR is introduced here.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     resolve pipeline names;
 *     resolve stage names;
 *     resolve references;
 *     validate input/output relationships;
 *     validate dependency edges;
 *     detect invalid dependency cycles;
 *     distinguish legal feedback from illegal cycles;
 *     validate types;
 *     validate requirements;
 *     validate capabilities;
 *     validate constraints;
 *     validate preferences;
 *     validate provenance;
 *     validate reproducibility;
 *     validate checkpoint semantics;
 *     validate resource feasibility;
 *     validate portability;
 *     validate cross-domain references;
 *     validate security constraints;
 *     preserve deterministic semantic ordering.
 *
 * None of these decisions are made by this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * Semantic lowering may eventually produce:
 *
 *     classical representation
 *     AI semantic representation
 *     dataflow representation
 *     quantum::ir
 *     HDL/hardware representation
 *     distributed representation
 *
 * according to the semantics of the contained computation.
 *
 * There is no pipeline-specific competing IR defined here.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime responsibilities include:
 *
 *     resource acquisition
 *     target discovery
 *     placement
 *     scheduling
 *     execution
 *     checkpoint realization
 *     recovery
 *     streaming transport
 *     distributed communication
 *     device management
 *
 * This grammar performs none of these.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The stable public rule is:
 *
 *     pipelineConstruct
 *
 * Existing consumers should integrate through that rule.
 *
 * Existing helper rule names are retained below where practical as compatibility
 * surfaces, but the canonical pipeline member implementation is the generic
 * structural model.
 *
 * ============================================================================
 */

parser grammar AIPipelines;

options {
    tokenVocab = ZamaniLexer;
}

import Types,
       Expressions,
       Statements;


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * Stable public boundary.
 *
 * A pipeline construct may be:
 *
 *     declaration
 *     invocation
 *     reference
 *     anonymous/nested region
 */
pipelineConstruct
    : pipelineDeclaration
    | pipelineInvocation
    | pipelineReference
    | pipelineRegion
    ;


/* ============================================================================
 * PIPELINE DECLARATION
 * ========================================================================== */

/*
 * Canonical declaration form:
 *
 *     @pipeline name {
 *         ...
 *     }
 *
 * Also valid structurally:
 *
 *     @pipeline name;
 *     @pipeline name: PipelineType;
 *     @pipeline name = expression;
 *
 * The annotation name is intentionally not hard-coded.
 *
 * Semantic analysis determines whether the annotation denotes a pipeline
 * declaration.
 */
pipelineDeclaration
    : pipelineAnnotation
      IDENTIFIER
      pipelineDeclarationTail
    ;


pipelineDeclarationTail
    : pipelineGenericCall?
      pipelineTypeClause?
      pipelineInitializer?
      pipelineBody?
      SEMICOLON?
    ;


/*
 * Optional declaration parameterization.
 *
 * This is expressed as ordinary expressions rather than a second generic
 * parameter grammar. Generic type parameterization remains owned by the
 * canonical type/function systems.
 */
pipelineGenericCall
    : LPAREN
      pipelineArgumentList?
      RPAREN
    ;


/*
 * Canonical type clause.
 */
pipelineTypeClause
    : COLON
      typeExpression
    ;


/*
 * Canonical initializer.
 */
pipelineInitializer
    : ASSIGN
      expression
    ;


/*
 * Canonical annotation boundary.
 *
 * This deliberately uses:
 *
 *     AT IDENTIFIER
 *
 * and never:
 *
 *     NANO_ANNOTATION
 */
pipelineAnnotation
    : AT
      IDENTIFIER
    ;


/* ============================================================================
 * PIPELINE BODY
 * ========================================================================== */

/*
 * A pipeline body is an ordered semantic region.
 *
 * No maximum member count or nesting depth is encoded.
 */
pipelineBody
    : LBRACE
      pipelineMember*
      RBRACE
    ;


/*
 * Anonymous pipeline region.
 */
pipelineRegion
    : LBRACE
      pipelineMember*
      RBRACE
    ;


/* ============================================================================
 * PIPELINE MEMBERS
 * ========================================================================== */

/*
 * The generic annotation form is the primary pipeline extension mechanism.
 *
 * Ordinary statements remain available.
 *
 * An annotated statement should be represented by a pipeline directive/region
 * when it is intended to be interpreted as pipeline metadata or orchestration.
 *
 * This avoids trying to classify arbitrary annotation names in the parser.
 */
pipelineMember
    : pipelineAnnotatedMember
    | pipelineStatement
    ;


/*
 * Generic pipeline annotation construct.
 *
 * Examples:
 *
 *     @stage preprocess { ... }
 *     @input data: Data;
 *     @output result: Tensor;
 *     @model encoder;
 *     @model encoder(...);
 *     @requires(qubits >= n);
 *     @capability(capability("tensor.compute"));
 *     @checkpoint state;
 *     @stream(source);
 *     @batch(size);
 *     @window(duration);
 *     @provenance(metadata);
 *     @custom_operation(value);
 *
 * No annotation vocabulary is hard-coded here.
 */
pipelineAnnotatedMember
    : pipelineAnnotation
      pipelineAnnotationPayload
    ;


/*
 * Structural dispatch after:
 *
 *     AT IDENTIFIER
 *
 * The first token of the payload determines the shape.
 */
pipelineAnnotationPayload
    : LPAREN
      pipelineArgumentList?
      RPAREN
      pipelinePostCallTail?
    | IDENTIFIER
      pipelineNamedPayloadTail
    | IDENTIFIER
      THIN_ARROW
      pipelineEndpoint
      pipelineEdgeTail?
    | ASSIGN
      expression
      SEMICOLON
    | pipelineBody
    | SEMICOLON
    ;


/* ============================================================================
 * NAMED ANNOTATION PAYLOAD
 * ========================================================================== */

/*
 * Examples:
 *
 *     @stage preprocess;
 *     @stage preprocess { ... }
 *     @input data: Tensor;
 *     @output result = value;
 *     @model encoder;
 *     @requires capability(...);
 *
 * A named payload may itself be called.
 */
pipelineNamedPayloadTail
    : LPAREN
      pipelineArgumentList?
      RPAREN
      pipelinePostCallTail?
    | pipelineNamedDeclarationTail
    ;


pipelineNamedDeclarationTail
    : pipelineTypeClause?
      pipelineInitializer?
      pipelineBody?
      SEMICOLON?
    ;


/*
 * The post-call tail permits:
 *
 *     @stage(foo) { ... }
 *     @requires(qubits >= n);
 *     @operation(value);
 */
pipelinePostCallTail
    : pipelineBody
    | SEMICOLON
    ;


/* ============================================================================
 * DEPENDENCY EDGES
 * ========================================================================== */

/*
 * Dependency edges are structurally distinguished by:
 *
 *     THIN_ARROW
 *
 * Example:
 *
 *     @edge preprocess -> inference;
 *
 * or:
 *
 *     @edge preprocess -> inference {
 *         ...
 *     }
 *
 * This is logical dependency/data flow.
 *
 * It is NOT physical topology.
 */
pipelineEdgeTail
    : pipelineEdgePayload?
      pipelineEdgeTerminator
    ;


pipelineEdgePayload
    : ASSIGN
      expression
    | pipelineBody
    ;


pipelineEdgeTerminator
    : SEMICOLON
    | /* empty when a body terminates the construct */
    ;


pipelineEndpoint
    : pipelineQualifiedReference
    ;


pipelineQualifiedReference
    : IDENTIFIER
      (
          DOT IDENTIFIER
        | DOUBLE_COLON IDENTIFIER
      )*
    ;


/* ============================================================================
 * REFERENCES
 * ========================================================================== */

/*
 * Bare pipeline references.
 *
 * Examples:
 *
 *     inference
 *     pipelines::inference
 *     module.pipeline
 */
pipelineReference
    : pipelineQualifiedReference
    ;


/* ============================================================================
 * INVOCATIONS
 * ========================================================================== */

/*
 * Pipeline invocation:
 *
 *     inference();
 *     inference(data);
 *     module::inference(data, configuration);
 *
 * The invocation uses canonical expressions.
 */
pipelineInvocation
    : pipelineQualifiedReference
      LPAREN
      pipelineArgumentList?
      RPAREN
      SEMICOLON?
    ;


pipelineArgumentList
    : expression
      (
          COMMA
          expression
      )*
    ;


/* ============================================================================
 * NESTED REGIONS
 * ========================================================================== */

/*
 * Nested pipeline regions have no grammar-level depth limit.
 */
pipelineNestedRegion
    : pipelineAnnotation
      pipelineBody
    ;


/* ============================================================================
 * CANONICAL STATEMENTS
 * ========================================================================== */

/*
 * Ordinary Zamani statements remain owned by Statements.
 *
 * No pipeline-specific statement language is created.
 */
pipelineStatement
    : statement
    ;


/* ============================================================================
 * COMPATIBILITY RULES
 * ============================================================================
 *
 * These rules preserve useful public names from the previous pipeline grammar
 * without making those names separate grammar authorities.
 *
 * They all delegate to the canonical structural representation.
 *
 * Semantic analysis determines the actual annotation role.
 * ========================================================================== */

pipelineInput
    : pipelineAnnotatedMember
    ;


pipelineOutput
    : pipelineAnnotatedMember
    ;


pipelineParameter
    : pipelineAnnotatedMember
    ;


pipelineLocal
    : pipelineAnnotatedMember
    ;


pipelineStage
    : pipelineAnnotatedMember
    ;


pipelineEdge
    : pipelineAnnotatedMember
    ;


pipelineBranch
    : pipelineAnnotatedMember
    ;


pipelineJoin
    : pipelineAnnotatedMember
    ;


pipelineStream
    : pipelineAnnotatedMember
    ;


pipelineBatch
    : pipelineAnnotatedMember
    ;


pipelineWindow
    : pipelineAnnotatedMember
    ;


pipelineCheckpoint
    : pipelineAnnotatedMember
    ;


pipelineRequirement
    : pipelineAnnotatedMember
    ;


pipelineCapability
    : pipelineAnnotatedMember
    ;


pipelineConstraint
    : pipelineAnnotatedMember
    ;


pipelinePreference
    : pipelineAnnotatedMember
    ;


pipelineProvenance
    : pipelineAnnotatedMember
    ;


pipelineReproducibility
    : pipelineAnnotatedMember
    ;


pipelineMetadata
    : pipelineAnnotatedMember
    ;


pipelineOperation
    : pipelineAnnotatedMember
    ;


pipelineNestedPipeline
    : pipelineAnnotatedMember
    ;


/* ============================================================================
 * COMPATIBILITY TYPE / VALUE HELPERS
 * ========================================================================== */

/*
 * These helper rules intentionally delegate to canonical shared syntax.
 */
pipelineType
    : typeExpression
    ;


pipelineValueType
    : COLON
      typeExpression
    ;


pipelineValueInitializer
    : ASSIGN
      expression
    ;


pipelineOperationPayload
    : ASSIGN
      expression
    | LPAREN
      pipelineArgumentList?
      RPAREN
    ;


/*
 * Existing downstream consumers may use this name.
 *
 * It is intentionally equivalent to the canonical pipeline body.
 */
pipelineContents
    : pipelineBody
    ;


pipelineMembers
    : pipelineMember*
    ;


pipelineArguments
    : pipelineArgumentList
    ;


/* ============================================================================
 * SEMANTIC ROLE EXAMPLES
 * ============================================================================
 *
 * The following are examples of valid structural forms.
 *
 * They are NOT parser-level reserved keywords.
 *
 * --------------------------------------------------------------------------
 *
 * Pipeline declaration:
 *
 *     @pipeline inference {
 *         ...
 *     }
 *
 * --------------------------------------------------------------------------
 *
 * Input:
 *
 *     @input data: Dataset;
 *
 * --------------------------------------------------------------------------
 *
 * Output:
 *
 *     @output result: Tensor;
 *
 * --------------------------------------------------------------------------
 *
 * Stage:
 *
 *     @stage preprocess {
 *         ...
 *     }
 *
 * --------------------------------------------------------------------------
 *
 * Model:
 *
 *     @model encoder;
 *
 * --------------------------------------------------------------------------
 *
 * Model invocation:
 *
 *     @model encoder(input);
 *
 * --------------------------------------------------------------------------
 *
 * Dependency:
 *
 *     @edge preprocess -> inference;
 *
 * --------------------------------------------------------------------------
 *
 * Requirement:
 *
 *     @requires(qubits >= n);
 *
 * --------------------------------------------------------------------------
 *
 * Capability:
 *
 *     @capability(capability("tensor.compute"));
 *
 * --------------------------------------------------------------------------
 *
 * Constraint:
 *
 *     @constraint(latency <= budget);
 *
 * --------------------------------------------------------------------------
 *
 * Preference:
 *
 *     @preference(accelerator("quantum"));
 *
 * --------------------------------------------------------------------------
 *
 * Streaming:
 *
 *     @stream(source);
 *
 * --------------------------------------------------------------------------
 *
 * Batching:
 *
 *     @batch(batch_size);
 *
 * --------------------------------------------------------------------------
 *
 * Window:
 *
 *     @window(window_spec);
 *
 * --------------------------------------------------------------------------
 *
 * Checkpoint:
 *
 *     @checkpoint(state);
 *
 * --------------------------------------------------------------------------
 *
 * Provenance:
 *
 *     @provenance(metadata);
 *
 * --------------------------------------------------------------------------
 *
 * Reproducibility:
 *
 *     @reproducible(configuration);
 *
 * --------------------------------------------------------------------------
 *
 * Nested pipeline:
 *
 *     @pipeline nested {
 *         ...
 *     }
 *
 * --------------------------------------------------------------------------
 *
 * Arbitrary future operation:
 *
 *     @future_operation(value);
 *
 * The grammar accepts the structure without knowing what the operation means.
 *
 * Semantic registration determines whether it is valid.
 *
 * ============================================================================
 * CROSS-DOMAIN EXAMPLES
 * ============================================================================
 *
 * Classical:
 *
 *     @stage classical {
 *         let result = input + offset;
 *     }
 *
 * AI:
 *
 *     @stage inference {
 *         @model model(input);
 *     }
 *
 * Quantum:
 *
 *     @stage quantum {
 *         @requires(qubits >= n);
 *         @requires(capability("quantum.measurement"));
 *         @operation(custom_quantum_operation);
 *     }
 *
 * Hardware:
 *
 *     @stage accelerator {
 *         @requires(capability("accelerator.compute"));
 *     }
 *
 * HDL:
 *
 *     @stage hardware {
 *         @requires(capability("hardware.synthesis"));
 *     }
 *
 * Distributed:
 *
 *     @stage distributed {
 *         @requires(capability("distributed.compute"));
 *     }
 *
 * Hybrid:
 *
 *     @stage hybrid {
 *         @requires(capability("quantum.measurement"));
 *         @model classical_model;
 *     }
 *
 * The grammar does not need to import each of those domains.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * These constructs are deliberately unbounded by grammar-level constants:
 *
 *     pipelineMember*
 *     pipelineArgumentList
 *     pipelineBody
 *     pipelineQualifiedReference
 *     nested pipeline regions
 *     dependency edges
 *
 * Therefore the language does not impose a fixed:
 *
 *     stage count
 *     edge count
 *     input count
 *     output count
 *     worker count
 *     node count
 *     device count
 *     model count
 *     tensor count
 *     stream count
 *     batch count
 *     nesting count
 *
 * A finite implementation may still have parser/runtime resource limits.
 *
 * Those limits are implementation policies and MUST NOT be encoded as source
 * language semantics.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * For identical source and language version:
 *
 *     identical source
 *          ->
 *     identical token sequence
 *          ->
 *     equivalent parse tree
 *          ->
 *     equivalent AST structure
 *
 * This grammar contains:
 *
 *     no semantic predicates
 *     no embedded actions
 *     no random behavior
 *     no target discovery
 *     no hardware discovery
 *     no runtime state
 *     no resource allocation
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics belong to the parser.
 *
 * Semantic diagnostics belong to semantic analysis.
 *
 * The grammar MUST NOT silently invent:
 *
 *     stages
 *     dependencies
 *     resources
 *     devices
 *     workers
 *     values
 *     types
 *     capabilities
 *
 * Missing or malformed source must remain diagnosable.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *     performs no I/O;
 *     performs no network operations;
 *     performs no process execution;
 *     performs no filesystem operations;
 *     performs no hardware access;
 *     performs no dynamic loading;
 *     performs no runtime execution.
 *
 * Generated compiler/runtime code must remain safe Rust.
 *
 * ============================================================================
 * VERSIONING CONTRACT
 * ============================================================================
 *
 * New pipeline semantic roles SHOULD preferably be introduced through:
 *
 *     registered annotations
 *     dialects
 *     semantic registries
 *     versioned semantic contracts
 *
 * rather than adding a parser alternative for every new AI framework or
 * hardware vendor.
 *
 * Existing syntax must be retained or migrated according to the repository's
 * compatibility policy.
 *
 * ============================================================================
 * NO-CIRCULARITY CONTRACT
 * ============================================================================
 *
 * This grammar imports ONLY:
 *
 *     Types
 *     Expressions
 *     Statements
 *
 * It MUST NOT import:
 *
 *     AI
 *     Models
 *     Datasets
 *     Training
 *     Inference
 *     Tensors
 *     Agents
 *     Quantum
 *     Hardware
 *     Distributed
 *     Networking
 *
 * merely to obtain generic syntax.
 *
 * The dependency direction remains:
 *
 *     canonical shared grammar
 *             |
 *             v
 *       AIPipelines
 *             |
 *             v
 *      semantic analysis
 *
 * This prevents circular grammar dependencies.
 *
 * ============================================================================
 * INTEGRATION WITH AI.G4
 * ============================================================================
 *
 * `ai.g4` is the AI composition boundary.
 *
 * It should expose pipeline syntax through:
 *
 *     pipelineConstruct
 *
 * at the canonical parser composition layer.
 *
 * It should NOT duplicate pipeline productions.
 *
 * If the root parser directly imports AIPipelines, that root composition remains
 * authoritative and AI.g4 remains a semantic/domain composition boundary.
 *
 * The important invariant is:
 *
 *     exactly one implementation of pipeline syntax.
 *
 * ============================================================================
 * INTEGRATION WITH MODELS / DATASETS / TRAINING / INFERENCE
 * ============================================================================
 *
 * This grammar references those domains only through:
 *
 *     identifiers
 *     expressions
 *     types
 *     capabilities
 *     requirements
 *     semantic annotations
 *
 * It does not import their grammars.
 *
 * Therefore:
 *
 *     pipeline -> model
 *     pipeline -> dataset
 *     pipeline -> training
 *     pipeline -> inference
 *
 * does not create grammar cycles.
 *
 * ============================================================================
 * INTEGRATION WITH QUANTUM
 * ============================================================================
 *
 * Quantum stages remain generic pipeline computation.
 *
 * The lowering chain is:
 *
 *     pipeline AST
 *          |
 *          v
 *     semantic quantum operation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target
 *
 * No second quantum IR is created by this grammar.
 *
 * ============================================================================
 * INTEGRATION WITH HARDWARE / HDL
 * ============================================================================
 *
 * Hardware intent remains semantic.
 *
 * Pipeline syntax can require:
 *
 *     capability("gpu.compute")
 *     capability("fpga.synthesis")
 *     capability("hardware.synthesis")
 *
 * without selecting a physical target.
 *
 * Hardware/HDL grammars remain the owners of their domain-specific source
 * constructs.
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCES
 * ============================================================================
 *
 * Pipeline annotations can carry resource expressions:
 *
 *     @requires(memory >= required_memory);
 *     @requires(qubits >= n);
 *     @requires(capability("tensor.compute"));
 *
 * The resource subsystem determines feasibility.
 *
 * The pipeline grammar never converts those expressions into hard-coded
 * compiler limits.
 *
 * ============================================================================
 * INTEGRATION WITH EXECUTION
 * ============================================================================
 *
 * Pipeline dependency and region semantics may later inform:
 *
 *     scheduling
 *     placement
 *     concurrency
 *     checkpointing
 *     recovery
 *     streaming
 *     distributed execution
 *
 * This grammar does not perform those operations.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests:
 *
 *     minimal pipeline
 *     typed pipeline
 *     initialized pipeline
 *     empty pipeline
 *     nested pipeline
 *     pipeline invocation
 *     pipeline reference
 *     named stage
 *     typed input
 *     typed output
 *     model reference
 *     model invocation
 *     dependency edge
 *     edge with payload
 *     branch directive
 *     join directive
 *     stream directive
 *     batch directive
 *     window directive
 *     checkpoint directive
 *     provenance directive
 *     reproducibility directive
 *     requirement directive
 *     capability directive
 *     constraint directive
 *     preference directive
 *     arbitrary future annotation
 *     ordinary statement
 *     classical stage
 *     quantum stage
 *     HDL/hardware stage
 *     distributed stage
 *     hybrid stage
 *
 * Required negative syntax tests:
 *
 *     missing annotation name
 *     missing declaration name
 *     malformed argument list
 *     malformed dependency arrow
 *     missing dependency target
 *     malformed type clause
 *     malformed initializer
 *     missing closing body
 *     missing closing argument list
 *     malformed qualified reference
 *
 * Required semantic negative tests:
 *
 *     duplicate pipeline name
 *     duplicate stage name
 *     unresolved reference
 *     invalid edge endpoint
 *     illegal dependency cycle
 *     incompatible input/output types
 *     contradictory requirements
 *     impossible constraints
 *     unsupported capability
 *     invalid checkpoint
 *     invalid cross-domain value
 *
 * Required scalability tests:
 *
 *     one stage
 *     many stages
 *     one edge
 *     many edges
 *     many inputs
 *     many outputs
 *     many arguments
 *     deeply nested regions
 *     large expressions
 *     symbolic resource quantities
 *
 * The tests must not define those cases as language maximums.
 *
 * Required determinism tests:
 *
 *     same source -> same token sequence
 *     same source -> equivalent parse tree
 *     repeated parse -> equivalent AST
 *
 * Required compatibility tests:
 *
 *     existing pipeline declarations
 *     existing pipeline invocations
 *     existing annotation spellings
 *     migration from legacy annotation representation
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no universal constants for:
 *
 *     qubits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     QPUs
 *     devices
 *     nodes
 *     workers
 *     memory
 *     tensor rank
 *     tensor dimensions
 *     pipeline stages
 *     pipeline edges
 *     pipeline depth
 *
 * It contains no physical device IDs.
 *
 * It contains no vendor-specific target requirements.
 *
 * It contains no scheduling implementation.
 *
 * It contains no placement implementation.
 *
 * It contains no runtime implementation.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     1. AIPipelines compiles against the canonical ZamaniLexer.
 *
 *     2. Types resolves to the canonical type grammar.
 *
 *     3. Expressions resolves to the canonical expression grammar.
 *
 *     4. Statements resolves to the canonical statement grammar.
 *
 *     5. pipelineConstruct is the sole public pipeline entry point.
 *
 *     6. No second pipeline grammar exists elsewhere.
 *
 *     7. No NANO_ANNOTATION dependency remains.
 *
 *     8. No local identifier token/rule is redefined.
 *
 *     9. No pipeline-specific expression grammar exists.
 *
 *    10. No pipeline-specific type system exists.
 *
 *    11. No universal hardware limits are encoded.
 *
 *    12. Requirements/capabilities/constraints/preferences remain semantically
 *        distinct even though their lexical structure is generic.
 *
 *    13. Dependency edges remain logical rather than physical topology.
 *
 *    14. Quantum semantics ultimately use quantum::ir.
 *
 *    15. No QEC implementation exists here.
 *
 *    16. No ZQN implementation exists here.
 *
 *    17. No scheduler implementation exists here.
 *
 *    18. No hardware discovery exists here.
 *
 *    19. No runtime execution exists here.
 *
 *    20. Positive tests exist.
 *
 *    21. Negative tests exist.
 *
 *    22. Boundary tests exist.
 *
 *    23. Scalability tests exist.
 *
 *    24. Determinism tests exist.
 *
 *    25. Compatibility tests exist.
 *
 *    26. AST mapping is documented.
 *
 *    27. Semantic mapping is documented.
 *
 *    28. IR mapping is documented.
 *
 *    29. Downstream consumers are documented.
 *
 *    30. Hard-coding audit passes.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */