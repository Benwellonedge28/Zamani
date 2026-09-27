/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/data/mining.g4
 *
 * Grammar:
 *     ZamaniDataMining
 *
 * Status:
 *     CANONICAL DATA-MINING PARSER GRAMMAR
 *
 * Language baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the SOURCE-LEVEL SYNTAX for portable data-mining intent.
 *
 * Data mining is treated as a semantic computation over logical data.
 *
 * This grammar therefore describes:
 *
 *     - mining declarations;
 *     - mining expressions;
 *     - mining sources;
 *     - feature/attribute selection;
 *     - target/label selection;
 *     - filtering;
 *     - sampling;
 *     - preprocessing;
 *     - transformation composition;
 *     - pattern discovery;
 *     - classification intent;
 *     - regression intent;
 *     - clustering intent;
 *     - anomaly/outlier detection intent;
 *     - association/pattern mining intent;
 *     - sequence mining intent;
 *     - graph mining intent;
 *     - temporal mining intent;
 *     - stream mining intent;
 *     - statistical analysis intent;
 *     - dimensionality-reduction intent;
 *     - feature construction;
 *     - feature selection;
 *     - model/evaluation intent;
 *     - validation intent;
 *     - cross-validation intent;
 *     - experiment intent;
 *     - metric intent;
 *     - mining constraints;
 *     - resource requirements;
 *     - capability requirements;
 *     - preferences;
 *     - hints;
 *     - provenance;
 *     - lineage;
 *     - reproducibility metadata;
 *     - outputs and materialization intent.
 *
 * This grammar DOES NOT define the implementation of any mining algorithm.
 *
 * It deliberately does NOT hard-code a closed algorithm vocabulary.
 *
 * Therefore future algorithms can be represented by semantic operation names
 * without changing the language grammar.
 *
 * Examples:
 *
 *     mine::cluster(data, ...)
 *     mine::classify(data, ...)
 *     analytics::discover_patterns(data, ...)
 *     future::mining::new_algorithm(data, ...)
 *
 * are structurally valid where the semantic registry recognizes them.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The grammar is designed for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A mining program describes WHAT computation is required.
 *
 * It does not prescribe:
 *
 *     - CPU;
 *     - GPU;
 *     - FPGA;
 *     - ASIC;
 *     - QPU;
 *     - accelerator;
 *     - number of cores;
 *     - number of workers;
 *     - number of nodes;
 *     - memory capacity;
 *     - storage capacity;
 *     - network topology;
 *     - device IDs;
 *     - provider IDs;
 *     - physical addresses;
 *     - physical partitions;
 *     - physical replicas.
 *
 * Those are resolved after parsing through semantic analysis, resource
 * discovery, capability negotiation, optimization, scheduling, routing,
 * deployment and runtime systems.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO universal maximum on:
 *
 *     datasets
 *     records
 *     features
 *     labels
 *     targets
 *     dimensions
 *     classes
 *     clusters
 *     patterns
 *     rules
 *     windows
 *     samples
 *     batches
 *     transformations
 *     mining stages
 *     metrics
 *     experiments
 *     candidates
 *     models
 *     partitions
 *     replicas
 *     nodes
 *     devices
 *     resources
 *
 * Repetition is represented by normal ANTLR repetition operators:
 *
 *     *
 *     +
 *
 * rather than fixed cardinalities.
 *
 * A source-level numeric value remains program semantics.
 *
 * Example:
 *
 *     top_k = 1000
 *
 * is valid program intent.
 *
 * It MUST NOT be interpreted as:
 *
 *     MAX_TOP_K = 1000
 *
 * for the language implementation.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_DATASETS
 *     MAX_RECORDS
 *     MAX_FEATURES
 *     MAX_LABELS
 *     MAX_CLASSES
 *     MAX_CLUSTERS
 *     MAX_PATTERNS
 *     MAX_RULES
 *     MAX_SAMPLES
 *     MAX_BATCH_SIZE
 *     MAX_WINDOW_SIZE
 *     MAX_DIMENSIONS
 *     MAX_MODELS
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_NETWORK_SIZE
 *
 * No equivalent hidden restriction may be encoded by enumerating a finite
 * number of alternatives.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     canonical Zamani parser
 *          |
 *          v
 *     data mining grammar
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> type checking
 *          +--> effect checking
 *          +--> capability checking
 *          +--> resource checking
 *          +--> provenance checking
 *          +--> determinism analysis
 *          +--> ownership/lifetime analysis
 *          |
 *          v
 *     canonical data semantic model
 *          |
 *          v
 *     canonical IR
 *          |
 *          +--> classical lowering
 *          +--> tensor/AI lowering
 *          +--> distributed lowering
 *          +--> accelerator lowering
 *          +--> streaming lowering
 *          +--> quantum/classical boundary
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     scheduling / routing / placement
 *          |
 *          v
 *     execution
 *
 * This grammar never constructs IR.
 *
 * This grammar never executes mining.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - mining declarations;
 *     - mining expressions;
 *     - mining sources;
 *     - mining targets;
 *     - feature declarations;
 *     - label declarations;
 *     - mining objectives;
 *     - mining methods as open semantic references;
 *     - mining parameters;
 *     - mining constraints;
 *     - mining evaluation intent;
 *     - mining validation intent;
 *     - mining experiment structure;
 *     - mining pipeline composition;
 *     - mining output declarations;
 *     - mining provenance declarations;
 *     - mining reproducibility metadata;
 *     - mining resource/capability intent;
 *     - mining execution preferences and hints.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical definitions;
 *     - identifiers;
 *     - universal expressions;
 *     - universal types;
 *     - datasets;
 *     - tables;
 *     - schemas;
 *     - records;
 *     - general transformations;
 *     - tensor semantics;
 *     - AI model semantics;
 *     - statistical algorithm implementations;
 *     - numerical algorithms;
 *     - machine learning frameworks;
 *     - database engines;
 *     - query planners;
 *     - storage engines;
 *     - schedulers;
 *     - hardware;
 *     - quantum hardware;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime resource discovery.
 *
 * ============================================================================
 * INTEGRATION WITH EXISTING DATA GRAMMAR
 * ============================================================================
 *
 * Existing authoritative data components include:
 *
 *     grammar/data/data.g4
 *     grammar/data/queries.g4
 *     grammar/data/datasets.g4
 *     grammar/data/transformations.g4
 *     grammar/data/tables.g4
 *     grammar/data/tensors.g4
 *     grammar/data/collections.g4
 *     grammar/data/streams.g4
 *
 * This file MUST NOT duplicate their responsibilities.
 *
 * Integration boundary:
 *
 *     data.g4
 *         |
 *         +--> dataMiningConstruct
 *                    |
 *                    +--> miningDeclaration
 *                    +--> miningStatement
 *                    +--> miningExpression
 *
 * Existing dataset/query/transformation constructs remain authoritative for
 * their own domains.
 *
 * Mining grammar references them semantically through ordinary expressions
 * and source references rather than recreating their declarations.
 *
 * ============================================================================
 * LEXER INTEGRATION
 * ============================================================================
 *
 * Canonical lexical authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammar:
 *
 *     parser grammar ZamaniDataMining;
 *
 * Token vocabulary:
 *
 *     ZamaniLexer
 *
 * This file defines NO lexer rules.
 *
 * ============================================================================
 * SHARED GRAMMAR CONTRACT
 * ============================================================================
 *
 * The following rules are expected to be supplied by the canonical parser
 * composition layer:
 *
 *     expression
 *     typeExpr
 *     typeExpression
 *     identifier
 *     qualifiedName
 *     parameterList
 *     genericParameters
 *     lambdaExpression
 *     argumentList
 *     block
 *     annotation
 *     visibilityModifier
 *
 * The mining grammar provides prefixed local entry rules around those shared
 * contracts where necessary.
 *
 * No second expression or type system is created here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Recommended conceptual lowering:
 *
 *     miningDeclaration
 *         -> generic domain declaration
 *
 *     miningExpression
 *         -> generic operation/expression
 *
 *     miningSource
 *         -> data source reference
 *
 *     miningFeatureSelection
 *         -> semantic selection metadata
 *
 *     miningTarget
 *         -> semantic target metadata
 *
 *     miningObjective
 *         -> semantic objective
 *
 *     miningMethod
 *         -> semantic operation reference
 *
 *     miningParameter
 *         -> semantic parameter binding
 *
 *     miningEvaluation
 *         -> semantic evaluation specification
 *
 *     miningConstraint
 *         -> semantic constraint
 *
 *     miningRequirement
 *         -> resource/capability requirement
 *
 *     miningOutput
 *         -> logical output specification
 *
 * The exact Rust AST types remain owned by the existing domain-neutral AST.
 *
 * No MiningAst, MiningIR, or MiningRuntimeIR should be introduced merely
 * because this grammar exists.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Mining syntax lowers through the canonical data/semantic pipeline.
 *
 * Conceptually:
 *
 *     mining source
 *          |
 *          v
 *     mining semantic model
 *          |
 *          v
 *     canonical data representation
 *          |
 *          v
 *     canonical IR
 *          |
 *          +--> classical execution
 *          +--> tensor execution
 *          +--> AI/ML execution
 *          +--> distributed execution
 *          +--> accelerator execution
 *          +--> streaming execution
 *
 * No mining-specific universal IR is defined here.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source text;
 *     - canonical lexical vocabulary;
 *     - grammar;
 *     - language version/dialect.
 *
 * Parsing MUST NOT inspect:
 *
 *     - hardware;
 *     - memory;
 *     - storage;
 *     - network;
 *     - database state;
 *     - runtime state;
 *     - randomness;
 *     - wall-clock time;
 *     - environment variables.
 *
 * ============================================================================
 */

parser grammar ZamaniDataMining;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC INTEGRATION ENTRY POINT
 * ============================================================================
 *
 * This is the ONLY public mining entry point that data.g4 should consume.
 *
 * data.g4 should contain one integration alternative:
 *
 *     | dataMiningConstruct
 *
 * It must not duplicate the rules below.
 *
 * ============================================================================
 */

dataMiningConstruct
    : miningDeclaration
    | miningStatement
    | miningExpression
    ;


/*
 * ============================================================================
 * 2. MINING DECLARATION
 * ============================================================================
 *
 * A mining declaration creates a named logical mining computation.
 *
 * It does not create a physical model, database job, GPU job, or cluster job.
 *
 * Example:
 *
 *     mine customer_patterns {
 *         source customers;
 *         ...
 *     }
 *
 * ============================================================================
 */

miningDeclaration
    : visibilityModifier?
      'mine'
      IDENTIFIER
      genericParameters?
      miningDeclarationSignature?
      miningDeclarationAttributes?
      miningBody
    ;


/*
 * ============================================================================
 * 3. MINING DECLARATION SIGNATURE
 * ============================================================================
 */

miningDeclarationSignature
    : LPAREN
      parameterList?
      RPAREN
      miningReturnType?
    ;

miningReturnType
    : THIN_ARROW
      typeExpr
    ;


/*
 * ============================================================================
 * 4. MINING ATTRIBUTES
 * ============================================================================
 */

miningDeclarationAttributes
    : LBRACKET
      miningAttribute*
      RBRACKET
    ;

miningAttribute
    : annotation
    | miningProperty
    ;

miningProperty
    : IDENTIFIER
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 5. MINING BODY
 * ============================================================================
 */

miningBody
    : LBRACE
      miningMember*
      RBRACE
    ;

miningMember
    : miningSourceDeclaration
    | miningInputDeclaration
    | miningFeatureDeclaration
    | miningLabelDeclaration
    | miningTargetDeclaration
    | miningObjectiveDeclaration
    | miningMethodDeclaration
    | miningParameterDeclaration
    | miningPreprocessDeclaration
    | miningTransformDeclaration
    | miningFilterDeclaration
    | miningSampleDeclaration
    | miningSplitDeclaration
    | miningWindowDeclaration
    | miningValidationDeclaration
    | miningEvaluationDeclaration
    | miningMetricDeclaration
    | miningConstraintDeclaration
    | miningRequirementDeclaration
    | miningCapabilityDeclaration
    | miningPreferenceDeclaration
    | miningHintDeclaration
    | miningExperimentDeclaration
    | miningOutputDeclaration
    | miningProvenanceDeclaration
    | miningLineageDeclaration
    | miningReproducibilityDeclaration
    | miningPropertyDeclaration
    | miningExpressionStatement
    ;


/*
 * ============================================================================
 * 6. SOURCE
 * ============================================================================
 *
 * A source is an existing logical data object.
 *
 * It may ultimately be:
 *
 *     dataset
 *     collection
 *     table
 *     stream
 *     query
 *     tensor
 *     generated data
 *     remote data
 *     sensor data
 *     simulation data
 *     future data representation
 *
 * The source's implementation is resolved downstream.
 *
 * ============================================================================
 */

miningSourceDeclaration
    : 'source'
      IDENTIFIER
      miningSourceType?
      miningSourceInitializer?
      SEMICOLON
    ;

miningSourceType
    : COLON
      typeExpr
    ;

miningSourceInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 7. INPUT
 * ============================================================================
 */

miningInputDeclaration
    : 'input'
      IDENTIFIER
      miningInputType?
      miningInputInitializer?
      SEMICOLON
    ;

miningInputType
    : COLON
      typeExpr
    ;

miningInputInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 8. FEATURES
 * ============================================================================
 *
 * Feature selection is structural intent.
 *
 * It does not prescribe storage layout or model implementation.
 *
 * Examples:
 *
 *     features age, income, location;
 *
 *     features customer.profile;
 *
 *     features select(data, predicate);
 *
 * ============================================================================
 */

miningFeatureDeclaration
    : 'features'
      miningFeatureSelector
      SEMICOLON
    ;

miningFeatureSelector
    : miningFeatureList
    | miningFeatureExpression
    ;

miningFeatureList
    : miningFeatureReference
      (
          COMMA
          miningFeatureReference
      )*
      COMMA?
    ;

miningFeatureReference
    : qualifiedName
    ;

miningFeatureExpression
    : expression
    ;


/*
 * ============================================================================
 * 9. LABELS
 * ============================================================================
 */

miningLabelDeclaration
    : 'labels'
      miningValueSelector
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. TARGETS
 * ============================================================================
 */

miningTargetDeclaration
    : 'target'
      miningValueSelector
      SEMICOLON
    ;

miningValueSelector
    : miningReferenceList
    | expression
    ;

miningReferenceList
    : qualifiedName
      (
          COMMA
          qualifiedName
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 11. OBJECTIVES
 * ============================================================================
 *
 * Objectives are semantic expressions.
 *
 * The grammar deliberately does not enumerate a closed set of objectives.
 *
 * Examples:
 *
 *     objective accuracy;
 *     objective minimize(error);
 *     objective maximize(score);
 *     objective discover(patterns);
 *     objective custom::objective(...);
 *
 * ============================================================================
 */

miningObjectiveDeclaration
    : 'objective'
      miningObjectiveExpression
      SEMICOLON
    ;

miningObjectiveExpression
    : expression
    ;


/*
 * ============================================================================
 * 12. METHOD
 * ============================================================================
 *
 * The method name is intentionally open-ended.
 *
 * This permits future mining algorithms without grammar changes.
 *
 * Examples:
 *
 *     method cluster;
 *     method classification;
 *     method association_rules;
 *     method custom::algorithm;
 *     method future::mining::method;
 *
 * Semantic analysis determines whether the referenced method exists and
 * whether its input/output contracts are satisfied.
 *
 * ============================================================================
 */

miningMethodDeclaration
    : 'method'
      miningMethodReference
      miningMethodArguments?
      miningMethodOptions?
      SEMICOLON
    ;

miningMethodReference
    : qualifiedName
    ;

miningMethodArguments
    : LPAREN
      argumentList?
      RPAREN
    ;

miningMethodOptions
    : LBRACKET
      miningOption*
      RBRACKET
    ;


/*
 * ============================================================================
 * 13. PARAMETERS
 * ============================================================================
 */

miningParameterDeclaration
    : 'parameter'
      IDENTIFIER
      miningParameterType?
      miningParameterInitializer?
      SEMICOLON
    ;

miningParameterType
    : COLON
      typeExpr
    ;

miningParameterInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 14. PREPROCESSING
 * ============================================================================
 *
 * Preprocessing is represented as an open transformation reference.
 *
 * This avoids making the grammar a fixed catalog of preprocessing algorithms.
 *
 * ============================================================================
 */

miningPreprocessDeclaration
    : 'preprocess'
      miningOperationInvocation
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. TRANSFORMATION
 * ============================================================================
 *
 * Existing grammar/data/transformations.g4 remains authoritative for general
 * transformation syntax.
 *
 * This rule only creates the mining-level invocation boundary.
 * ============================================================================
 */

miningTransformDeclaration
    : 'transform'
      miningOperationInvocation
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. FILTER
 * ============================================================================
 */

miningFilterDeclaration
    : 'filter'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. SAMPLING
 * ============================================================================
 *
 * Sample size, fraction, strategy and seed are expressions.
 *
 * No fixed sample size is encoded.
 * ============================================================================
 */

miningSampleDeclaration
    : 'sample'
      miningSampleSpecification
      SEMICOLON
    ;

miningSampleSpecification
    : miningOperationInvocation
    | expression
    ;


/*
 * ============================================================================
 * 18. DATA SPLITTING
 * ============================================================================
 *
 * The number and meaning of splits are semantic.
 *
 * ============================================================================
 */

miningSplitDeclaration
    : 'split'
      miningSplitSpecification
      SEMICOLON
    ;

miningSplitSpecification
    : expression
    ;


/*
 * ============================================================================
 * 19. WINDOWING
 * ============================================================================
 *
 * Supports temporal, streaming and bounded/unbounded mining.
 *
 * Window size, step, duration and retention are expressions.
 * ============================================================================
 */

miningWindowDeclaration
    : 'window'
      miningWindowSpecification
      SEMICOLON
    ;

miningWindowSpecification
    : expression
    ;


/*
 * ============================================================================
 * 20. VALIDATION
 * ============================================================================
 */

miningValidationDeclaration
    : 'validate'
      miningValidationSpecification
      SEMICOLON
    ;

miningValidationSpecification
    : miningOperationInvocation
    | expression
    ;


/*
 * ============================================================================
 * 21. EVALUATION
 * ============================================================================
 */

miningEvaluationDeclaration
    : 'evaluate'
      miningEvaluationSpecification
      SEMICOLON
    ;

miningEvaluationSpecification
    : miningEvaluationTarget?
      miningEvaluationMetricList?
      miningEvaluationOptions?
    ;

miningEvaluationTarget
    : 'on'
      expression
    ;

miningEvaluationMetricList
    : 'using'
      miningMetricReference
      (
          COMMA
          miningMetricReference
      )*
      COMMA?
    ;

miningMetricReference
    : qualifiedName
      miningMethodArguments?
    ;

miningEvaluationOptions
    : LBRACKET
      miningOption*
      RBRACKET
    ;


/*
 * ============================================================================
 * 22. METRICS
 * ============================================================================
 *
 * Metrics remain open-ended semantic references.
 *
 * ============================================================================
 */

miningMetricDeclaration
    : 'metric'
      IDENTIFIER
      miningMetricDefinition?
      SEMICOLON
    ;

miningMetricDefinition
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 23. CONSTRAINT
 * ============================================================================
 */

miningConstraintDeclaration
    : 'constraint'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 24. RESOURCE REQUIREMENTS
 * ============================================================================
 *
 * Requirements are logical.
 *
 * Example:
 *
 *     requires capability("tensor.compute");
 *
 *     requires memory >= required_memory;
 *
 * No machine capacity is hard-coded.
 * ============================================================================
 */

miningRequirementDeclaration
    : 'requires'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 25. CAPABILITY REQUIREMENTS
 * ============================================================================
 */

miningCapabilityDeclaration
    : 'capability'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 26. PREFERENCES
 * ============================================================================
 */

miningPreferenceDeclaration
    : 'prefer'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 27. HINTS
 * ============================================================================
 */

miningHintDeclaration
    : 'hint'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 28. EXPERIMENTS
 * ============================================================================
 *
 * An experiment groups mining alternatives or evaluation configurations.
 *
 * It does not imply a particular runtime experiment engine.
 *
 * ============================================================================
 */

miningExperimentDeclaration
    : 'experiment'
      IDENTIFIER
      miningExperimentAttributes?
      LBRACE
      miningExperimentMember*
      RBRACE
    ;

miningExperimentAttributes
    : LBRACKET
      miningAttribute*
      RBRACKET
    ;

miningExperimentMember
    : miningMethodDeclaration
    | miningParameterDeclaration
    | miningObjectiveDeclaration
    | miningValidationDeclaration
    | miningEvaluationDeclaration
    | miningMetricDeclaration
    | miningConstraintDeclaration
    | miningRequirementDeclaration
    | miningCapabilityDeclaration
    | miningPreferenceDeclaration
    | miningHintDeclaration
    | miningOutputDeclaration
    | miningPropertyDeclaration
    | miningExpressionStatement
    ;


/*
 * ============================================================================
 * 29. OUTPUT
 * ============================================================================
 */

miningOutputDeclaration
    : 'output'
      IDENTIFIER?
      miningOutputType?
      miningOutputInitializer?
      miningOutputOptions?
      SEMICOLON
    ;

miningOutputType
    : COLON
      typeExpr
    ;

miningOutputInitializer
    : ASSIGN
      expression
    ;

miningOutputOptions
    : LBRACKET
      miningOption*
      RBRACKET
    ;


/*
 * ============================================================================
 * 30. PROVENANCE
 * ============================================================================
 *
 * Provenance is declarative metadata.
 *
 * It does not implement a ledger or storage mechanism.
 * ============================================================================
 */

miningProvenanceDeclaration
    : 'provenance'
      miningMetadataBody
    ;


/*
 * ============================================================================
 * 31. LINEAGE
 * ============================================================================
 */

miningLineageDeclaration
    : 'lineage'
      miningMetadataBody
    ;


/*
 * ============================================================================
 * 32. REPRODUCIBILITY
 * ============================================================================
 *
 * Reproducibility requirements may contain expressions such as:
 *
 *     reproducible(seed)
 *     reproducible(inputs)
 *     reproducible(environment)
 *
 * The runtime/compiler determine how such requirements are realized.
 * ============================================================================
 */

miningReproducibilityDeclaration
    : 'reproducible'
      miningReproducibilitySpecification?
      SEMICOLON
    ;

miningReproducibilitySpecification
    : LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 33. GENERIC PROPERTY
 * ============================================================================
 *
 * This is the primary forward-compatibility mechanism.
 *
 * New mining concepts that do not require parser-level semantics can be
 * represented as ordinary properties.
 * ============================================================================
 */

miningPropertyDeclaration
    : IDENTIFIER
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 34. MINING STATEMENTS
 * ============================================================================
 */

miningStatement
    : miningRunStatement
    | miningTrainStatement
    | miningDiscoverStatement
    | miningAnalyzeStatement
    | miningPredictStatement
    | miningInferStatement
    | miningEvaluateStatement
    | miningExpressionStatement
    ;


/*
 * ============================================================================
 * 35. RUN
 * ============================================================================
 *
 * `run` is intentionally generic.
 *
 * The operation being run is represented by a semantic expression.
 * ============================================================================
 */

miningRunStatement
    : 'run'
      miningOperationInvocation
      SEMICOLON
    ;


/*
 * ============================================================================
 * 36. TRAIN
 * ============================================================================
 *
 * Training syntax is intentionally generic so that mining remains independent
 * of any specific machine-learning framework.
 * ============================================================================
 */

miningTrainStatement
    : 'train'
      miningOperationInvocation
      SEMICOLON
    ;


/*
 * ============================================================================
 * 37. DISCOVER
 * ============================================================================
 */

miningDiscoverStatement
    : 'discover'
      miningOperationInvocation
      SEMICOLON
    ;


/*
 * ============================================================================
 * 38. ANALYZE
 * ============================================================================
 */

miningAnalyzeStatement
    : 'analyze'
      miningOperationInvocation
      SEMICOLON
    ;


/*
 * ============================================================================
 * 39. PREDICT
 * ============================================================================
 */

miningPredictStatement
    : 'predict'
      miningOperationInvocation
      SEMICOLON
    ;


/*
 * ============================================================================
 * 40. INFER
 * ============================================================================
 */

miningInferStatement
    : 'infer'
      miningOperationInvocation
      SEMICOLON
    ;


/*
 * ============================================================================
 * 41. EVALUATE STATEMENT
 * ============================================================================
 */

miningEvaluateStatement
    : 'evaluate'
      miningOperationInvocation
      SEMICOLON
    ;


/*
 * ============================================================================
 * 42. MINING EXPRESSION STATEMENT
 * ============================================================================
 */

miningExpressionStatement
    : miningExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 43. MINING EXPRESSION
 * ============================================================================
 *
 * Mining expressions are deliberately compositional.
 *
 * They can participate in the universal Zamani expression system through
 * normal semantic lowering.
 * ============================================================================
 */

miningExpression
    : miningOperationInvocation
    | miningPipelineExpression
    | miningQueryExpression
    | miningReferenceExpression
    | miningExperimentExpression
    | miningResultExpression
    ;


/*
 * ============================================================================
 * 44. OPEN MINING OPERATION
 * ============================================================================
 *
 * This is the central extensibility mechanism.
 *
 * The grammar does NOT enumerate:
 *
 *     kmeans
 *     dbscan
 *     svm
 *     random_forest
 *     apriori
 *     pca
 *     isolation_forest
 *     etc.
 *
 * Those are semantic operations.
 *
 * A new algorithm therefore does not require a parser release.
 * ============================================================================
 */

miningOperationInvocation
    : miningOperationReference
      miningGenericArguments?
      LPAREN
      miningArgumentList?
      RPAREN
      miningOperationOptions?
    ;

miningOperationReference
    : qualifiedName
    ;

miningGenericArguments
    : DOUBLE_COLON
      LESS
      miningTypeArgumentList
      GREATER
    ;

miningTypeArgumentList
    : typeExpr
      (
          COMMA
          typeExpr
      )*
      COMMA?
    ;

miningArgumentList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 45. OPERATION OPTIONS
 * ============================================================================
 */

miningOperationOptions
    : LBRACKET
      miningOption*
      RBRACKET
    ;

miningOption
    : miningRequirementOption
    | miningPreferenceOption
    | miningHintOption
    | miningPropertyOption
    | miningMetadataOption
    ;

miningRequirementOption
    : 'requires'
      LPAREN
      expression
      RPAREN
    ;

miningPreferenceOption
    : 'prefer'
      LPAREN
      expression
      RPAREN
    ;

miningHintOption
    : 'hint'
      LPAREN
      expression
      RPAREN
    ;

miningPropertyOption
    : IDENTIFIER
      ASSIGN
      expression
    ;

miningMetadataOption
    : IDENTIFIER
      miningMetadataBody
    ;


/*
 * ============================================================================
 * 46. PIPELINE EXPRESSION
 * ============================================================================
 *
 * Mining operations compose over logical data.
 *
 * Example:
 *
 *     source
 *       |> preprocess(...)
 *       |> select(...)
 *       |> discover(...)
 *       |> evaluate(...)
 *
 * The pipe operator is consumed only if it belongs to the canonical lexer.
 * ============================================================================
 */

miningPipelineExpression
    : miningPipelineStage
      (
          PIPE
          miningPipelineStage
      )+
    ;

miningPipelineStage
    : miningOperationInvocation
    | miningReferenceExpression
    | expression
    ;


/*
 * ============================================================================
 * 47. QUERY INTEGRATION
 * ============================================================================
 *
 * Queries remain owned by grammar/data/queries.g4.
 *
 * This rule intentionally provides a semantic bridge rather than redefining
 * SELECT/FROM/JOIN/etc.
 * ============================================================================
 */

miningQueryExpression
    : 'query'
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 48. REFERENCES
 * ============================================================================
 */

miningReferenceExpression
    : qualifiedName
    ;


/*
 * ============================================================================
 * 49. EXPERIMENT EXPRESSION
 * ============================================================================
 */

miningExperimentExpression
    : 'experiment'
      LPAREN
      miningArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 50. RESULT EXPRESSION
 * ============================================================================
 */

miningResultExpression
    : 'result'
      LPAREN
      miningArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 51. METADATA
 * ============================================================================
 *
 * Metadata is intentionally open-ended.
 *
 * This prevents future provenance, lineage, reproducibility and governance
 * systems from requiring parser changes merely to add metadata fields.
 * ============================================================================
 */

miningMetadataBody
    : LBRACE
      miningMetadataEntry*
      RBRACE
    ;

miningMetadataEntry
    : IDENTIFIER
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 52. COMMON MINING SEMANTIC SHAPES
 * ============================================================================
 *
 * These rules document stable structural categories without creating a closed
 * algorithm registry.
 * ============================================================================
 */


/*
 * Classification intent.
 *
 * Example:
 *
 *     classification(data, features, target, ...)
 *
 * Semantic analysis determines whether the referenced operation implements
 * classification and whether the target types are valid.
 */
miningClassificationExpression
    : miningOperationInvocation
    ;


/*
 * Regression intent.
 */
miningRegressionExpression
    : miningOperationInvocation
    ;


/*
 * Clustering intent.
 */
miningClusteringExpression
    : miningOperationInvocation
    ;


/*
 * Anomaly detection intent.
 */
miningAnomalyDetectionExpression
    : miningOperationInvocation
    ;


/*
 * Association/pattern discovery intent.
 */
miningAssociationExpression
    : miningOperationInvocation
    ;


/*
 * Sequence mining intent.
 */
miningSequenceExpression
    : miningOperationInvocation
    ;


/*
 * Graph mining intent.
 */
miningGraphExpression
    : miningOperationInvocation
    ;


/*
 * Temporal mining intent.
 */
miningTemporalExpression
    : miningOperationInvocation
    ;


/*
 * Stream mining intent.
 */
miningStreamExpression
    : miningOperationInvocation
    ;


/*
 * Dimensionality-reduction intent.
 */
miningDimensionalityReductionExpression
    : miningOperationInvocation
    ;


/*
 * Feature-selection intent.
 */
miningFeatureSelectionExpression
    : miningOperationInvocation
    ;


/*
 * Feature-construction intent.
 */
miningFeatureConstructionExpression
    : miningOperationInvocation
    ;


/*
 * Statistical-analysis intent.
 */
miningStatisticalExpression
    : miningOperationInvocation
    ;


/*
 * ============================================================================
 * 53. OPTIONAL STRUCTURED MINING BLOCK
 * ============================================================================
 *
 * This provides a compact form for declaring mining intent without forcing
 * every program to use a large declaration body.
 *
 * Example:
 *
 *     mining {
 *         source data;
 *         features x, y;
 *         target label;
 *         method cluster(...);
 *         objective ...;
 *     }
 *
 * ============================================================================
 */

miningBlockExpression
    : 'mining'
      LBRACE
      miningMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 54. SOURCE-TO-OUTPUT PIPELINE
 * ============================================================================
 */

miningPipelineDeclaration
    : 'pipeline'
      IDENTIFIER?
      LBRACE
      miningPipelineMember*
      RBRACE
    ;

miningPipelineMember
    : miningSourceDeclaration
    | miningPreprocessDeclaration
    | miningTransformDeclaration
    | miningFilterDeclaration
    | miningSampleDeclaration
    | miningSplitDeclaration
    | miningWindowDeclaration
    | miningMethodDeclaration
    | miningObjectiveDeclaration
    | miningValidationDeclaration
    | miningEvaluationDeclaration
    | miningOutputDeclaration
    | miningRequirementDeclaration
    | miningCapabilityDeclaration
    | miningPreferenceDeclaration
    | miningHintDeclaration
    | miningProvenanceDeclaration
    | miningLineageDeclaration
    | miningReproducibilityDeclaration
    | miningPropertyDeclaration
    ;


/*
 * ============================================================================
 * 55. MODEL/RESULT BINDINGS
 * ============================================================================
 *
 * These are logical bindings, not physical model implementations.
 * ============================================================================
 */

miningBinding
    : IDENTIFIER
      ASSIGN
      miningExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 56. GENERAL DATA-MINING COMPOSITION
 * ============================================================================
 */

miningComposition
    : miningOperationInvocation
      (
          PIPE
          miningOperationInvocation
      )+
    ;


/*
 * ============================================================================
 * 57. RESOURCE/CAPABILITY BLOCK
 * ============================================================================
 *
 * Example:
 *
 *     resources {
 *         requires capability("tensor.compute");
 *         requires memory >= required_memory;
 *         prefer capability("gpu.compute");
 *     }
 *
 * The grammar does not evaluate any of these expressions.
 * ============================================================================
 */

miningResourceBlock
    : 'resources'
      LBRACE
      miningResourceMember*
      RBRACE
    ;

miningResourceMember
    : miningRequirementDeclaration
    | miningCapabilityDeclaration
    | miningPreferenceDeclaration
    | miningHintDeclaration
    | miningConstraintDeclaration
    | miningPropertyDeclaration
    ;


/*
 * ============================================================================
 * 58. EXECUTION-INTENT BLOCK
 * ============================================================================
 *
 * Execution intent is semantic.
 *
 * Examples:
 *
 *     execution {
 *         deterministic(...);
 *         streaming(...);
 *         parallel(...);
 *         latency(...);
 *         energy(...);
 *     }
 *
 * No hardware is selected here.
 * ============================================================================
 */

miningExecutionBlock
    : 'execution'
      LBRACE
      miningExecutionMember*
      RBRACE
    ;

miningExecutionMember
    : miningExecutionOperation
    | miningRequirementDeclaration
    | miningCapabilityDeclaration
    | miningPreferenceDeclaration
    | miningHintDeclaration
    | miningConstraintDeclaration
    | miningPropertyDeclaration
    ;

miningExecutionOperation
    : IDENTIFIER
      LPAREN
      miningArgumentList?
      RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 59. ERROR/FAILURE INTENT
 * ============================================================================
 *
 * Error policy is declarative.
 *
 * Runtime behavior belongs downstream.
 * ============================================================================
 */

miningErrorPolicy
    : 'on_error'
      miningErrorPolicyBody
    ;

miningErrorPolicyBody
    : LBRACE
      miningErrorPolicyMember*
      RBRACE
    ;

miningErrorPolicyMember
    : IDENTIFIER
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 60. DETERMINISM CONTRACT
 * ============================================================================
 */

miningDeterminismDeclaration
    : 'determinism'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 61. ORDERING CONTRACT
 * ============================================================================
 */

miningOrderingDeclaration
    : 'ordering'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 62. MISSING-VALUE POLICY
 * ============================================================================
 */

miningMissingValuePolicy
    : 'missing'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 63. DATA QUALITY CONTRACT
 * ============================================================================
 */

miningQualityDeclaration
    : 'quality'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 64. SECURITY / PRIVACY INTENT
 * ============================================================================
 *
 * Security semantics remain owned by grammar/security/.
 *
 * This grammar only provides a mining-level expression boundary.
 * ============================================================================
 */

miningSecurityDeclaration
    : 'security'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 65. PRIVACY INTENT
 * ============================================================================
 */

miningPrivacyDeclaration
    : 'privacy'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 66. FEDERATED/DISTRIBUTED MINING INTENT
 * ============================================================================
 *
 * This is intentionally semantic and does not define a fixed topology.
 * ============================================================================
 */

miningDistributedDeclaration
    : 'distributed'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 67. STREAMING MINING INTENT
 * ============================================================================
 */

miningStreamingDeclaration
    : 'streaming'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 68. QUANTUM/CLASSICAL DATA BOUNDARY
 * ============================================================================
 *
 * Mining may consume classical results produced by quantum programs.
 *
 * This grammar does not define quantum operations.
 *
 * The quantum grammar remains authoritative for quantum syntax and the
 * canonical quantum::ir remains the quantum semantic boundary.
 * ============================================================================
 */

miningQuantumDataDeclaration
    : 'quantum_data'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 69. TENSOR DATA BOUNDARY
 * ============================================================================
 *
 * Tensor syntax remains owned by grammar/data/tensors.g4 and the canonical
 * type system.
 * ============================================================================
 */

miningTensorDataDeclaration
    : 'tensor_data'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 70. FINAL INTEGRATION CONTRACT
 * ============================================================================
 *
 * grammar/data/data.g4
 *
 * MUST expose exactly one mining integration alternative:
 *
 *     | dataMiningConstruct
 *
 * It must not duplicate:
 *
 *     miningDeclaration
 *     miningStatement
 *     miningExpression
 *
 * Other data grammars remain authoritative for their own domains:
 *
 *     queries.g4
 *     datasets.g4
 *     transformations.g4
 *     tables.g4
 *     tensors.g4
 *     collections.g4
 *     streams.g4
 *
 * ============================================================================
 * 71. SEMANTIC ALGORITHM REGISTRY
 * ============================================================================
 *
 * The grammar intentionally contains NO closed algorithm enumeration.
 *
 * Therefore these are examples of semantic names, not grammar keywords:
 *
 *     kmeans
 *     dbscan
 *     hierarchical_cluster
 *     svm
 *     random_forest
 *     decision_tree
 *     neural_network
 *     apriori
 *     fp_growth
 *     pca
 *     ica
 *     isolation_forest
 *     local_outlier_factor
 *     association_rules
 *     sequence_patterns
 *     graph_patterns
 *     spectral_analysis
 *
 * New algorithms are introduced through semantic/library registries and
 * capability contracts rather than modifying this parser grammar.
 *
 * ============================================================================
 * 72. TARGET INDEPENDENCE
 * ============================================================================
 *
 * A mining operation may eventually be lowered to:
 *
 *     CPU
 *     multicore CPU
 *     SIMD
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     distributed cluster
 *     HPC
 *     cloud
 *     edge
 *     embedded
 *     streaming engine
 *     tensor engine
 *     quantum/classical hybrid environment
 *     future computational substrate
 *
 * The source syntax remains unchanged.
 *
 * ============================================================================
 * 73. RESOURCE AVAILABILITY
 * ============================================================================
 *
 * "Unbounded" means:
 *
 *     no artificial language-level machine ceiling.
 *
 * It does NOT mean physical execution has infinite resources.
 *
 * Actual feasibility is determined by:
 *
 *     - available memory;
 *     - available storage;
 *     - compute capability;
 *     - accelerator capability;
 *     - network capability;
 *     - compiler resources;
 *     - runtime resources;
 *     - scheduler capacity;
 *     - target constraints;
 *     - explicit program requirements.
 *
 * ============================================================================
 * 74. SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no Rust actions;
 *     - no semantic predicates;
 *     - no unsafe code;
 *     - no filesystem operations;
 *     - no network operations;
 *     - no runtime execution;
 *     - no hardware discovery.
 *
 * Rust integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and must use safe Rust only.
 *
 * ============================================================================
 * 75. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is considered complete when:
 *
 *     [x] one public mining entry point exists;
 *     [x] lexical ownership remains centralized;
 *     [x] universal expression syntax is reused;
 *     [x] universal type syntax is reused;
 *     [x] dataset syntax remains owned by datasets.g4;
 *     [x] query syntax remains owned by queries.g4;
 *     [x] transformation syntax remains owned by transformations.g4;
 *     [x] tensor syntax remains owned by tensors.g4;
 *     [x] table syntax remains owned by tables.g4;
 *     [x] no mining-specific lexer exists;
 *     [x] no mining-specific AST is required;
 *     [x] no mining-specific universal IR is created;
 *     [x] algorithms are not hard-coded;
 *     [x] hardware is not hard-coded;
 *     [x] resource capacities are not hard-coded;
 *     [x] distributed topology is not hard-coded;
 *     [x] tensor rank is not hard-coded;
 *     [x] dataset size is not hard-coded;
 *     [x] worker count is not hard-coded;
 *     [x] machine width is not hard-coded;
 *     [x] physical placement is not encoded;
 *     [x] target selection is downstream;
 *     [x] resource requirements are distinct from preferences;
 *     [x] capabilities are distinct from implementation decisions;
 *     [x] provenance is declarative;
 *     [x] reproducibility is declarative;
 *     [x] deterministic parsing is preserved;
 *     [x] forward extension is possible through open operation references.
 *
 * Remaining repository-level completion requires:
 *
 *     - data.g4 integration;
 *     - root Zamani.g4 composition;
 *     - semantic AST mapping;
 *     - semantic validation;
 *     - conformance tests;
 *     - negative tests;
 *     - scalability tests;
 *     - compatibility tests.
 *
 * Those are integration tasks and must not be hidden inside this file.
 *
 * ============================================================================
 */