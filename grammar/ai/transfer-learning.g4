/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* FILE
* ---
* grammar/ai/transfer-learning.g4
* 
* GRAMMAR
* ---
* AITransferLearning
* 
* STATUS
* ---
* CANONICAL AI TRANSFER-LEARNING SEMANTIC COMPOSITION BOUNDARY
* 
* LANGUAGE BASELINE
* ---
* Rust 1.97 or later
* Rust 2021
* Safe Rust only
* 
* GRAMMAR TECHNOLOGY
* ---
* ANTLR4 parser grammar
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file defines the parser-level composition boundary for transfer
* learning without creating a second learning language.
* 
* Transfer learning is a semantic specialization of the universal learning
* operation:
* 
* learn
* 
* The canonical source syntax remains owned by:
* 
* grammar/statements/learn.g4
* 
* whose grammar identity is:
* 
* Learn
* 
* and whose public source-level rule is:
* 
* learnStatement
* 
* This file therefore MUST NOT reproduce:
* 
* learnStatement
* learnClause
* learnFromClause
* learnWithClause
* learnArgumentList
* learnArgument
* 
* or any other learning syntax.
* 
* ============================================================================
* IMPORTANT ARCHITECTURAL DECISION
* ============================================================================
* 
* Transfer learning is NOT a separate core-language keyword family.
* 
* The language must remain open-ended.
* 
* The grammar must not introduce:
* 
* TRANSFER_LEARNING
* FINE_TUNE
* FREEZE_LAYER
* SOURCE_MODEL
* TARGET_MODEL
* PRETRAINED_MODEL
* 
* as permanent lexical tokens merely to describe one learning technique.
* 
* Likewise, this file must not enumerate:
* 
* neural-network architectures;
* layer families;
* optimizer families;
* fine-tuning algorithms;
* parameter-efficient adaptation methods;
* model vendors;
* frameworks;
* model formats;
* accelerator types;
* hardware configurations.
* 
* Those are semantic/library/dialect capabilities.
* 
* ============================================================================
* SEMANTIC DEFINITION
* ============================================================================
* 
* Transfer learning means that a learning operation may reuse knowledge,
* representation, parameters, state, features, structure, or another
* semantically transferable artifact obtained from a source model/task/domain
* when producing or adapting a target model/task/domain.
* 
* The exact realization is deliberately open.
* 
* Possible semantic realizations include, without being limited to:
* 
* - parameter reuse;
* - representation reuse;
* - feature reuse;
* - model initialization;
* - partial adaptation;
* - full adaptation;
* - parameter-efficient adaptation;
* - domain adaptation;
* - task adaptation;
* - cross-domain learning;
* - continual reuse;
* - knowledge transfer;
* - learned representation transfer;
* - classical transfer;
* - tensor transfer;
* - distributed transfer;
* - quantum-assisted transfer;
* - hybrid transfer;
* - future transfer mechanisms.
* 
* The grammar deliberately does not choose among these.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - the transfer-learning semantic composition boundary;
* - the canonical AI-domain identity of transfer-learning intent;
* - the relationship between transfer-learning semantics and the universal
*   learning operation;
* - the integration contract between transfer-learning semantics and the
*   domain-neutral frontend/semantic model.
* 
* THIS FILE DOES NOT OWN:
* 
* - lexer tokens;
* - keyword spelling;
* - identifiers;
* - qualified names;
* - expressions;
* - types;
* - learning statement syntax;
* - training declaration syntax;
* - model declaration syntax;
* - dataset syntax;
* - inference syntax;
* - adaptation syntax;
* - reasoning syntax;
* - knowledge syntax;
* - uncertainty syntax;
* - policy syntax;
* - contract syntax;
* - effect syntax;
* - resource syntax;
* - capability syntax;
* - provenance syntax;
* - quantum syntax;
* - HDL syntax;
* - hardware syntax;
* - distributed syntax;
* - runtime execution;
* - model loading;
* - parameter freezing;
* - optimizer implementation;
* - gradient computation;
* - accelerator selection;
* - target selection;
* - physical placement;
* - scheduling;
* - routing;
* - QEC;
* - ZQN;
* - HAL.
* 
* ============================================================================
* SINGLE-AUTHORITY RULE
* ============================================================================
* 
* There is exactly one canonical source-level learning statement:
* 
* Learn.learnStatement
* 
* Transfer learning MUST use that canonical syntax.
* 
* There must not be a second grammar such as:
* 
* transferLearningStatement
* 
* that independently parses:
* 
* transfer_learning ...
* 
* or:
* 
* fine_tune ...
* 
* This prevents the following invalid architecture:
* 
* generic learning grammar
*         +
* transfer-learning grammar
*         +
* training grammar
* 
* all independently evolving the same source concepts.
* 
* ============================================================================
* WHY THIS FILE EXISTS
* ============================================================================
* 
* A semantic feature may require an explicit architectural home even when it
* does not require new parser syntax.
* 
* This file provides that home for transfer-learning semantics.
* 
* The intended semantic distinction is:
* 
* generic learning
*     |
*     +--> ordinary learning
*     |
*     +--> transfer learning
*     |
*     +--> continual learning
*     |
*     +--> distributed learning
*     |
*     +--> reinforcement learning
*     |
*     +--> future learning strategy
* 
* New learning strategies must therefore normally be represented as semantic
* strategy metadata/capabilities rather than permanent language keywords.
* 
* ============================================================================
* DEPENDENCY CONTRACT
* ============================================================================
* 
* DEPENDS_ON
* ---
* 
* grammar/statements/learn.g4
* 
* Grammar:
* 
* Learn
* 
* Public rule:
* 
* learnStatement
* 
* Transitive universal dependencies:
* 
* ZamaniLexer
* Expressions
* Names
* 
* This file does not duplicate those dependencies.
* 
* ============================================================================
* IMPORT CONTRACT
* ============================================================================
* 
* ANTLR imports grammar identities, not filesystem paths.
* 
* Therefore the canonical dependency is:
* 
* import Learn;
* 
* The ANTLR build must expose:
* 
* grammar/statements/
* 
* through its grammar-library/import path.
* 
* ============================================================================
* EXPORT CONTRACT
* ============================================================================
* 
* This grammar intentionally exports one explicit semantic-composition rule:
* 
* aiTransferLearningConstruct
* 
* The rule delegates to:
* 
* learnStatement
* 
* without reproducing learning syntax.
* 
* IMPORTANT:
* 
* This rule is a semantic-domain boundary and MUST NOT be added as a second
* alternative beside "aiLearningConstruct" for the same "learnStatement".
* 
* The AI composition root must continue to route canonical "learn" syntax
* through:
* 
* AILearning.aiLearningConstruct
* 
* Transfer-learning classification occurs downstream from the canonical
* learning parse tree.
* 
* This prevents ANTLR ambiguity caused by two parser alternatives matching
* the same token sequence.
* 
* ============================================================================
* CONSUMER CONTRACT
* ============================================================================
* 
* Primary semantic consumer:
* 
* AI semantic analysis
* 
* Parser-level composition:
* 
* grammar/ai/ai.g4
*     |
*     v
* AILearning
*     |
*     v
* Learn.learnStatement
* 
* Transfer-learning semantics are associated with the resulting learning
* construct during semantic classification.
* 
* This file MUST NOT require "ai.g4" to introduce a competing:
* 
* aiTransferLearningConstruct
* 
* alternative for the same "learnStatement".
* 
* If a future parser architecture requires explicit source-level strategy
* dispatch, that architecture must establish an unambiguous strategy owner
* before this file is changed.
* 
* ============================================================================
* SOURCE-LEVEL REPRESENTATION
* ============================================================================
* 
* The canonical learning grammar currently accepts forms such as:
* 
* learn model;
* 
* learn model from dataset;
* 
* learn model with (objective = objective);
* 
* learn model from dataset with (
*     strategy = strategy,
*     policy = policy
* );
* 
* Transfer-learning intent is represented semantically through the ordinary
* learning construct.
* 
* A transfer-learning semantic instance should identify, as applicable:
* 
* source artifact;
* target artifact;
* source task/domain;
* target task/domain;
* transferable knowledge;
* transfer strategy;
* adaptation objective;
* data;
* constraints;
* requirements;
* capabilities;
* policies;
* contracts;
* provenance.
* 
* These are semantic properties, not additional parser keywords.
* 
* ============================================================================
* SEMANTIC CLASSIFICATION CONTRACT
* ============================================================================
* 
* Semantic analysis may classify a canonical learning operation as transfer
* learning when its resolved arguments and metadata establish a source/target
* reuse relationship.
* 
* Conceptually:
* 
* learn TARGET from SOURCE with (
*     strategy = TRANSFER_STRATEGY,
*     ...
* );
* 
* may produce:
* 
* LearningIntent {
*     kind: Transfer,
*     target: TARGET,
*     source: SOURCE,
*     ...
* }
* 
* The exact Rust semantic type is owned by the frontend semantic model.
* 
* This grammar MUST NOT require that exact type name.
* 
* ============================================================================
* OPEN-WORLD STRATEGY CONTRACT
* ============================================================================
* 
* The grammar must not contain a finite catalogue such as:
* 
* fine_tune
* feature_extract
* freeze_layers
* parameter_efficient
* adapter
* prompt_tuning
* distillation
* domain_adaptation
* 
* as parser alternatives.
* 
* Strategy identity remains semantic data.
* 
* This permits future transfer mechanisms to be introduced without changing
* the universal parser architecture.
* 
* ============================================================================
* MODEL INTEGRATION
* ============================================================================
* 
* Model declarations remain owned by:
* 
* grammar/ai/models.g4
* 
* This file must not redefine:
* 
* modelDeclaration
* modelReference
* modelMember
* modelInput
* modelOutput
* 
* A source or target may resolve semantically to:
* 
* model;
* model interface;
* model state;
* learned representation;
* parameter set;
* feature representation;
* knowledge artifact;
* strategy;
* user-defined transferable artifact.
* 
* Model identity remains open-world.
* 
* ============================================================================
* LEARNING INTEGRATION
* ============================================================================
* 
* Canonical learning syntax is owned by:
* 
* grammar/statements/learn.g4
* 
* Existing:
* 
* grammar/ai/learning.g4
* 
* is the AI composition adapter for that universal operation.
* 
* Transfer learning is therefore a specialization of:
* 
* AILearning
* 
* rather than a replacement for it.
* 
* The dependency direction is:
* 
* Learn
*   |
*   v
* AILearning
*   |
*   v
* AI semantic learning model
*   |
*   +--> transfer strategy
*   |
*   +--> ordinary strategy
*   |
*   +--> future strategy
* 
* There must be no dependency:
* 
* Learn -> AITransferLearning
* 
* because the universal statement grammar must remain independent of the AI
* semantic domain.
* 
* ============================================================================
* TRAINING INTEGRATION
* ============================================================================
* 
* Training declarations remain owned by:
* 
* grammar/ai/training.g4
* 
* Transfer learning may be used by a training declaration, but this file must
* not redefine:
* 
* trainingConstruct
* training declaration members
* training phases
* training steps
* optimizer syntax
* metric syntax
* loss syntax.
* 
* The semantic relationship may be:
* 
* training
*    |
*    v
* learning intent
*    |
*    v
* transfer strategy
* 
* or:
* 
* transfer-learning intent
*    |
*    v
* training realization
* 
* without creating a second grammar authority.
* 
* ============================================================================
* ADAPTATION INTEGRATION
* ============================================================================
* 
* Adaptation is a related but distinct universal operation.
* 
* Its canonical source grammar is:
* 
* grammar/statements/adapt.g4
* 
* with:
* 
* Adapt.adaptStatement
* 
* Transfer learning may semantically result in model adaptation.
* 
* However:
* 
* transfer learning != unrestricted adaptation
* 
* and:
* 
* learn != arbitrary self-modifying code.
* 
* Any resulting adaptation must pass through the existing:
* 
* effects
* capabilities
* resources
* contracts
* policies
* authorization
* provenance
* 
* systems.
* 
* ============================================================================
* INFERENCE INTEGRATION
* ============================================================================
* 
* Inference remains owned by:
* 
* grammar/ai/inference.g4
* 
* Transfer learning may produce a model that is subsequently used for:
* 
* inference
* 
* but transfer-learning grammar must not redefine inference syntax.
* 
* Semantic flow may be:
* 
* source model
*      |
*      v
* transfer learning
*      |
*      v
* target model
*      |
*      v
* inference
* 
* ============================================================================
* DATA INTEGRATION
* ============================================================================
* 
* Dataset syntax remains owned by:
* 
* grammar/ai/datasets.g4
* 
* Data values remain ordinary semantic values.
* 
* Transfer learning may consume:
* 
* datasets
* streams
* query results
* knowledge
* observations
* measurements
* simulation results
* distributed results
* hardware observations
* future data domains.
* 
* This file does not define data formats.
* 
* ============================================================================
* KNOWLEDGE INTEGRATION
* ============================================================================
* 
* Transfer learning may consume or produce knowledge.
* 
* Knowledge syntax remains owned by the existing knowledge subsystem.
* 
* No:
* 
* assert
* retract
* query
* lookup
* 
* syntax is introduced here.
* 
* ============================================================================
* REASONING INTEGRATION
* ============================================================================
* 
* Transfer learning may use:
* 
* inference;
* reasoning;
* evidence;
* uncertainty;
* causal information;
* 
* but these remain independently owned grammar/semantic components.
* 
* A semantic transfer operation may therefore use reasoning to determine:
* 
* what knowledge is transferable;
* whether source and target are compatible;
* which evidence supports the transfer;
* whether transfer constraints are satisfied.
* 
* None of those decisions occur during parsing.
* 
* ============================================================================
* UNCERTAINTY INTEGRATION
* ============================================================================
* 
* Transferability may itself be uncertain.
* 
* Semantic analysis may therefore associate:
* 
* probability;
* confidence;
* uncertainty;
* evidence;
* distribution;
* 
* with a transfer decision.
* 
* This file does not create an uncertainty type system.
* 
* The universal type/AI uncertainty systems remain authoritative.
* 
* ============================================================================
* EFFECT CONTRACT
* ============================================================================
* 
* Parsing has no effects.
* 
* Semantic transfer-learning operations may carry effects including, depending
* on actual program meaning:
* 
* learning
* mutation
* randomness
* io
* network
* distributed
* foreign
* native
* measurement
* simulation
* reflection
* 
* This grammar does not declare a competing effect taxonomy.
* 
* Effect ownership remains under:
* 
* grammar/effects/
* 
* and the corresponding semantic effect model.
* 
* ============================================================================
* CAPABILITY CONTRACT
* ============================================================================
* 
* Transfer learning may require capabilities such as:
* 
* learning
* model.read
* model.write
* model.execute
* data.read
* tensor.compute
* distributed.compute
* distributed.training
* probabilistic.compute
* quantum.measurement
* simulation.execute
* 
* These names are semantic capability identifiers.
* 
* The grammar does not enumerate hardware devices.
* 
* In particular, it does not select:
* 
* CPU
* GPU
* FPGA
* ASIC
* QPU
* accelerator
* node
* device.
* 
* Capability resolution belongs downstream.
* 
* ============================================================================
* RESOURCE CONTRACT
* ============================================================================
* 
* Transfer learning may require arbitrary computational resources.
* 
* Semantic requirements may include:
* 
* memory >= required_memory
* storage >= required_storage
* capability("tensor.compute")
* capability("distributed.training")
* topology(required_topology)
* 
* The exact resource-expression syntax remains owned by:
* 
* grammar/resources/
* 
* This grammar does not duplicate resource grammar.
* 
* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* Transfer learning must not impose language-level limits on:
* 
* source models;
* target models;
* model members;
* transferable artifacts;
* parameters;
* tensors;
* tensor rank;
* tensor dimensions;
* datasets;
* samples;
* observations;
* learning operations;
* training steps;
* adaptation steps;
* workers;
* threads;
* processes;
* nodes;
* accelerators;
* devices;
* quantum resources;
* memory;
* storage;
* network size.
* 
* There must be no:
* 
* MAX_TRANSFER_MODELS
* MAX_TRANSFER_PARAMETERS
* MAX_TRANSFER_LAYERS
* MAX_TRANSFER_DATA
* MAX_TRANSFER_WORKERS
* MAX_TRANSFER_NODES
* MAX_TRANSFER_DEVICES
* 
* or equivalent language-capacity constants.
* 
* "Infinity" means that this grammar introduces no artificial finite capacity
* ceiling. Actual compilation and execution remain subject to:
* 
* program semantics;
* representation;
* compiler resources;
* runtime resources;
* target capabilities;
* physical resources;
* policy.
* 
* ============================================================================
* POCO-REAF CONTRACT
* ============================================================================
* 
* Transfer-learning source intent must remain target independent.
* 
* The same source semantics must be representable on:
* 
* tiny systems;
* embedded systems;
* CPUs;
* multicore systems;
* GPUs;
* FPGAs;
* ASICs;
* accelerators;
* QPUs;
* simulators;
* HPC systems;
* clusters;
* distributed systems;
* cloud systems;
* heterogeneous systems;
* future computational systems.
* 
* Source syntax must not change merely because the target has a different:
* 
* memory capacity;
* processor count;
* accelerator count;
* topology;
* quantum capacity;
* storage capacity;
* network capacity.
* 
* A target that cannot satisfy a semantic transfer requirement must report
* an explicit capability/resource/semantic diagnostic.
* 
* It must not silently change the meaning of the source program.
* 
* ============================================================================
* TYPE CONTRACT
* ============================================================================
* 
* This grammar introduces no transfer-specific type hierarchy.
* 
* It must not create:
* 
* TransferLearningType
* TransferModelType
* SourceModelType
* TargetModelType
* FineTuneType
* 
* as universal parser types.
* 
* Source and target values are ordinary Zamani expressions.
* 
* Semantic analysis determines whether they are compatible for transfer.
* 
* This allows transfer to work with future domain values without changing the
* parser grammar.
* 
* ============================================================================
* CONTRACT INTEGRATION
* ============================================================================
* 
* Transfer learning may participate in:
* 
* requires
* ensures
* invariant
* assume
* guarantee
* property
* 
* contracts.
* 
* Contract syntax is not owned here.
* 
* Contract validation may establish properties such as:
* 
* source artifact is available;
* target accepts the transferred representation;
* required capabilities exist;
* transfer policy is satisfied;
* provenance is complete;
* adaptation remains within declared constraints.
* 
* ============================================================================
* POLICY CONTRACT
* ============================================================================
* 
* Transfer learning may be constrained by policies governing:
* 
* source-data access;
* model access;
* model modification;
* data provenance;
* privacy;
* security;
* licensing metadata;
* network access;
* external model loading;
* randomness;
* reproducibility;
* resource usage;
* deployment;
* adaptation.
* 
* Policies do not belong to this grammar.
* 
* Policy interpretation remains downstream.
* 
* ============================================================================
* PROVENANCE CONTRACT
* ============================================================================
* 
* Transfer learning is especially dependent on provenance.
* 
* The semantic model should preserve, where applicable:
* 
* source artifact;
* source identity;
* source origin;
* source version;
* source task/domain;
* target identity;
* target task/domain;
* transferred artifact;
* data origin;
* transformation;
* strategy;
* evidence;
* decision;
* policy;
* verification;
* language version;
* toolchain version;
* reproducibility information.
* 
* This file does not define a second provenance grammar.
* 
* The universal provenance subsystem remains authoritative.
* 
* ============================================================================
* DETERMINISM CONTRACT
* ============================================================================
* 
* Parser behavior must be deterministic.
* 
* Parsing must depend only on:
* 
* source;
* lexer configuration;
* parser grammar;
* explicit language/dialect configuration.
* 
* Parsing must not depend on:
* 
* hardware availability;
* resource availability;
* network state;
* filesystem state;
* runtime model state;
* randomness;
* wall-clock time;
* scheduler state.
* 
* ============================================================================
* QUANTUM / HYBRID CONTRACT
* ============================================================================
* 
* Transfer learning may consume quantum-derived values or may participate in
* hybrid quantum-classical computation.
* 
* This grammar does not define quantum syntax.
* 
* Quantum computation remains owned by:
* 
* grammar/quantum/
* 
* The canonical quantum semantic boundary remains:
* 
* quantum::ir
* 
* The path is:
* 
* transfer-learning intent
*      |
*      v
* domain-neutral AST
*      |
*      v
* semantic learning model
*      |
*      +--------------------+
*      |                    |
*      v                    v
* classical           quantum semantics
*                           |
*                           v
*                      quantum::ir
* 
* This file must not create:
* 
* TransferQuantumIR
* QMLTransferIR
* AIQuantumTransferIR
* 
* or any other competing quantum representation.
* 
* ============================================================================
* HDL / HARDWARE CONTRACT
* ============================================================================
* 
* Transfer-learning computation may eventually be lowered to:
* 
* CPU;
* GPU;
* FPGA;
* ASIC;
* accelerator;
* embedded hardware;
* heterogeneous hardware.
* 
* This grammar does not describe physical realization.
* 
* It must not encode:
* 
* register widths;
* fixed memory banks;
* physical addresses;
* device identifiers;
* fixed accelerator counts;
* fixed FPGA resources;
* clock frequencies;
* physical topology.
* 
* Hardware intent remains downstream.
* 
* ============================================================================
* DISTRIBUTED CONTRACT
* ============================================================================
* 
* Transfer learning may be realized through:
* 
* sequential;
* parallel;
* distributed;
* federated;
* heterogeneous;
* accelerator-backed;
* future execution models.
* 
* The grammar does not encode:
* 
* worker count;
* node count;
* cluster size;
* network topology;
* physical placement.
* 
* Those are resource/capability/execution concerns.
* 
* ============================================================================
* INTEROPERABILITY CONTRACT
* ============================================================================
* 
* Transfer-learning source artifacts may originate from:
* 
* native Zamani models;
* foreign model formats;
* FFI providers;
* data systems;
* external services;
* dialects;
* libraries.
* 
* Interoperability remains owned by:
* 
* grammar/interoperability/
* 
* A foreign source artifact must carry the appropriate semantic effects,
* capabilities, policies, and provenance.
* 
* This grammar must not hard-code:
* 
* ONNX;
* TensorFlow;
* PyTorch;
* JAX;
* vendor model formats;
* 
* as universal language syntax.
* 
* ============================================================================
* METAPROGRAMMING CONTRACT
* ============================================================================
* 
* Transfer-learning declarations may be generated or specialized by
* metaprogramming.
* 
* Such generation remains subject to the existing:
* 
* reflection;
* compile-time;
* code-generation;
* policy;
* effect;
* capability;
* provenance
* 
* systems.
* 
* This grammar performs no code generation.
* 
* ============================================================================
* SECURITY CONTRACT
* ============================================================================
* 
* Parsing transfer-learning source must never:
* 
* - load model files;
* - access credentials;
* - access networks;
* - access datasets;
* - access hardware;
* - invoke external frameworks;
* - allocate devices;
* - execute model code.
* 
* Such operations belong to explicitly authorized downstream components.
* 
* A transfer-learning source construct does not itself grant access to a
* source model or dataset.
* 
* ============================================================================
* COMPILER CONTRACT
* ============================================================================
* 
* The compiler may:
* 
* specialize transfer learning;
* optimize it;
* fuse operations;
* select capabilities;
* negotiate resources;
* distribute computation;
* lower tensor operations;
* lower classical computation;
* lower quantum computation;
* select accelerators;
* schedule execution;
* preserve provenance;
* enforce contracts and policies.
* 
* None of these decisions belong to this grammar.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This grammar creates NO IR.
* 
* Transfer learning must lower through the canonical semantic architecture:
* 
* source
*   |
*   v
* parser
*   |
*   v
* domain-neutral AST
*   |
*   v
* semantic transfer-learning model
*   |
*   +--> types
*   +--> effects
*   +--> capabilities
*   +--> resources
*   +--> contracts
*   +--> policies
*   +--> provenance
*   |
*   v
* canonical semantic representation
*   |
*   +--> classical representation
*   +--> tensor/data representation
*   +--> distributed representation
*   +--> hardware representation
*   +--> quantum::ir where genuinely quantum
*   |
*   v
* target-independent optimization
*   |
*   v
* lowering
*   |
*   v
* routing / scheduling where required
*   |
*   v
* resilience / recovery where required
*   |
*   v
* ZQN / HAL where applicable
*   |
*   v
* target realization
* 
* There must not be a grammar-created:
* 
* TransferLearningIR
* TransferLearningQuantumIR
* TransferLearningDeviceIR
* 
* merely because this file exists.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* This grammar must not require a transfer-learning-specific AST hierarchy
* that duplicates universal learning nodes.
* 
* The parser context must preserve enough structure for the frontend to map:
* 
* learnStatement
* 
* into the existing domain-neutral AST.
* 
* Semantic classification may subsequently represent:
* 
* generic learning
* 
* or:
* 
* transfer learning
* 
* using the repository's semantic model.
* 
* The semantic representation should be able to preserve, where applicable:
* 
* learning kind;
* source;
* target;
* arguments;
* strategy;
* source spans;
* attributes;
* effects;
* capabilities;
* requirements;
* constraints;
* contracts;
* policies;
* provenance.
* 
* The exact Rust type names are owned by:
* 
* src/frontend/ast/
* semantic model
* 
* and must not be invented by this grammar.
* 
* ============================================================================
* RUST CONTRACT
* ============================================================================
* 
* This grammar contains:
* 
* no Rust actions;
* no semantic predicates;
* no filesystem access;
* no network access;
* no hardware access;
* no runtime execution;
* no embedded code.
* 
* The generated parser is intended for the repository's:
* 
* Rust 1.97 or later
* Rust 2021
* safe-Rust-only
* 
* frontend.
* 
* This grammar requires no "unsafe".
* 
* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* Because transfer learning uses the canonical learning syntax, the primary
* parser tests belong to:
* 
* grammar/tests/statements/learn/
* 
* Transfer-learning semantic tests should additionally exist under:
* 
* grammar/tests/ai/transfer-learning/
* 
* ---
* POSITIVE STRUCTURAL CASES
* ---
* 
* The following must remain valid through the canonical Learn grammar:
* 
* learn target_model;
* 
* learn target_model from source_model;
* 
* learn target_model from source_model with (
*     strategy = transfer_strategy
* );
* 
* learn target_model from source_model with (
*     data = target_data,
*     strategy = transfer_strategy
* );
* 
* learn target_model from source_model with (
*     objective = objective,
*     policy = policy,
*     provenance = provenance
* );
* 
* learn target_model from knowledge_source;
* 
* learn target_model from query_result;
* 
* learn target_model from measurement_result;
* 
* learn target_model from simulation_result;
* 
* The parser only establishes learning structure.
* 
* Semantic analysis determines whether each construct actually represents a
* valid transfer-learning operation.
* 
* ---
* POSITIVE SEMANTIC CASES
* ---
* 
* Semantic tests should cover:
* 
* source model -> target model;
* source representation -> target model;
* source knowledge -> target model;
* source task -> target task;
* source domain -> target domain;
* source data -> target task;
* transfer with explicit strategy;
* transfer with policy;
* transfer with provenance;
* transfer with contracts;
* transfer with uncertainty;
* transfer with resource requirements;
* transfer with capability requirements;
* transfer with distributed execution;
* transfer with accelerator realization;
* transfer with hybrid computation;
* transfer with quantum-derived information.
* 
* ---
* NEGATIVE SEMANTIC CASES
* ---
* 
* The semantic layer must reject or diagnose, as appropriate:
* 
* unknown source;
* unknown target;
* incompatible source/target types;
* unavailable source artifact;
* unauthorized source access;
* invalid transfer strategy;
* incompatible target interface;
* unsatisfied capability;
* unsatisfied resource requirement;
* policy violation;
* provenance violation;
* contract violation;
* invalid effect usage;
* invalid quantum boundary;
* unsupported foreign artifact.
* 
* These are semantic diagnostics, not parser diagnostics.
* 
* ---
* SCALABILITY TESTS
* ---
* 
* Tests must vary without changing this grammar:
* 
* number of source artifacts;
* number of target artifacts;
* number of learning operations;
* argument count;
* model complexity;
* tensor rank;
* tensor dimensions;
* dataset size;
* distributed participants;
* accelerator count;
* quantum resources;
* resource quantities.
* 
* No test value becomes a language capacity constant.
* 
* ---
* DETERMINISM TESTS
* ---
* 
* Identical source/token streams must produce equivalent parse structures.
* 
* Parser behavior must not vary with:
* 
* target hardware;
* available GPUs;
* available QPUs;
* current memory;
* current network;
* current filesystem;
* runtime model state;
* random seeds.
* 
* ---
* CROSS-DOMAIN TESTS
* ---
* 
* Transfer learning must be tested with:
* 
* classical computation;
* tensor/data computation;
* probabilistic computation;
* knowledge/reasoning;
* quantum-derived data;
* hybrid computation;
* HDL/hardware intent;
* accelerator intent;
* distributed computation;
* networking;
* simulation;
* security;
* FFI/interoperability.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This file MUST NOT contain universal capacity constants.
* 
* In particular, it contains no:
* 
* MAX_TRANSFER_MODELS
* MAX_TRANSFER_PARAMETERS
* MAX_TRANSFER_LAYERS
* MAX_TRANSFER_DATA
* MAX_TRANSFER_WORKERS
* MAX_TRANSFER_THREADS
* MAX_TRANSFER_NODES
* MAX_TRANSFER_DEVICES
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
* It also contains no:
* 
* fixed layer count;
* fixed tensor rank;
* fixed model size;
* fixed dataset size;
* fixed worker count;
* fixed device count;
* fixed accelerator count;
* fixed quantum-resource count.
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* This file introduces no new lexical spelling and therefore does not create
* a language-version compatibility problem by itself.
* 
* Existing canonical "learn" source syntax remains governed by:
* 
* grammar/statements/learn.g4
* 
* Historical aliases must be handled by:
* 
* grammar/compatibility/
* 
* rather than by adding duplicate transfer-learning parser rules.
* 
* ============================================================================
* INTEGRATION CONTRACT
* ============================================================================
* 
* 1. LEXER
* ---
* 
* No lexer modification is required for this file.
* 
* In particular, do not add:
* 
* TRANSFER_LEARNING
* FINE_TUNE
* FREEZE_LAYER
* 
* merely for this feature.
* 
* Transfer-learning strategy names remain identifiers or semantic names.
* 
* 2. UNIVERSAL LEARNING
* ---
* 
* The canonical source syntax is owned by:
* 
* grammar/statements/learn.g4
* 
* This file imports:
* 
* Learn
* 
* and consumes:
* 
* learnStatement
* 
* 3. AI LEARNING
* ---
* 
* grammar/ai/learning.g4
* 
* remains the canonical AI composition adapter for "learn".
* 
* It must not be replaced by this file.
* 
* The semantic transfer-learning classifier should operate after canonical
* learning parsing.
* 
* 4. AI COMPOSITION ROOT
* ---
* 
* grammar/ai/ai.g4
* 
* must continue to expose:
* 
* aiLearningConstruct
* 
* for canonical "learn" syntax.
* 
* It must NOT add both:
* 
* aiLearningConstruct
* aiTransferLearningConstruct
* 
* when both match the same "learnStatement".
* 
* Such an arrangement would create duplicate parser paths.
* 
* 5. MODELS
* ---
* 
* Model declarations/references remain owned by:
* 
* grammar/ai/models.g4
* 
* 6. TRAINING
* ---
* 
* Training syntax remains owned by:
* 
* grammar/ai/training.g4
* 
* 7. ADAPTATION
* ---
* 
* Adaptation syntax remains owned by:
* 
* grammar/statements/adapt.g4
* 
* Transfer-learning semantics may lower to adaptation semantics only after
* semantic validation.
* 
* 8. EXPRESSIONS
* ---
* 
* All learning operands remain ordinary:
* 
* expression
* 
* values under:
* 
* grammar/expressions/
* 
* No expression hierarchy is duplicated.
* 
* 9. TYPES
* ---
* 
* Types remain owned by:
* 
* grammar/types/
* 
* Transfer learning does not introduce a second type system.
* 
* 10. EFFECTS
* ---
* 
* Effects remain owned by:
* 
* grammar/effects/
* 
* Learning, mutation, network, distributed, measurement, foreign and other
* effects are determined semantically.
* 
* 11. RESOURCES
* ---
* 
* Resource requirements remain owned by:
* 
* grammar/resources/
* 
* No physical target is selected by this file.
* 
* 12. CAPABILITIES
* ---
* 
* Capability requirements remain owned by the canonical capability subsystem.
* 
* 13. CONTRACTS
* ---
* 
* Contract syntax remains owned by the universal contract subsystem.
* 
* 14. POLICIES
* ---
* 
* Policy syntax and enforcement remain owned by the policy/security systems.
* 
* 15. PROVENANCE
* ---
* 
* Provenance remains repository-wide and must not be duplicated for AI.
* 
* 16. CLASSICAL
* ---
* 
* Classical realization remains downstream.
* 
* 17. QUANTUM
* ---
* 
* Quantum realization remains downstream through:
* 
* quantum::ir
* 
* 18. HDL / HARDWARE
* ---
* 
* Hardware realization remains downstream.
* 
* 19. DISTRIBUTED
* ---
* 
* Distributed realization remains downstream.
* 
* 20. INTEROPERABILITY
* ---
* 
* Foreign model/data artifacts remain under:
* 
* grammar/interoperability/
* 
* and the semantic FFI/ABI system.
* 
* 21. RUNTIME
* ---
* 
* Runtime model loading, data access, resource discovery, scheduling and
* execution remain outside this grammar.
* 
* ============================================================================
* DEPENDENCY GRAPH
* ============================================================================
* 
* Source:
* 
* learn ...
* 
*   |
*   v
* 
* ZamaniLexer
*   |
*   v
* 
* Learn.learnStatement
*   |
*   v
* 
* AILearning.aiLearningConstruct
*   |
*   v
* 
* Domain-neutral AST
*   |
*   v
* 
* Semantic learning model
*   |
*   +--> transfer-learning classification
*   |
*   +--> types
*   +--> effects
*   +--> capabilities
*   +--> resources
*   +--> contracts
*   +--> policies
*   +--> provenance
*   |
*   v
* 
* Canonical semantic representation
*   |
*   +--> classical
*   +--> tensor/data
*   +--> distributed
*   +--> hardware
*   +--> quantum::ir
*   |
*   v
* 
* Target-independent optimization
*   |
*   v
* 
* Lowering
*   |
*   v
* 
* Routing / scheduling
*   |
*   v
* 
* Resilience / recovery
*   |
*   v
* 
* ZQN / HAL where applicable
*   |
*   v
* 
* Target realization
* 
* ============================================================================
* FILE-LOCAL COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when:
* 
* [x] It is an ANTLR4 parser grammar.
* 
* [x] Grammar identity is AITransferLearning.
* 
* [x] Canonical ZamaniLexer vocabulary is used.
* 
* [x] Canonical Learn grammar is imported.
* 
* [x] No learning syntax is duplicated.
* 
* [x] No transfer-learning keyword is introduced.
* 
* [x] No framework-specific syntax is introduced.
* 
* [x] No model-family catalogue is introduced.
* 
* [x] No optimizer catalogue is introduced.
* 
* [x] No hardware target is encoded.
* 
* [x] No physical resource capacity is encoded.
* 
* [x] No AI-specific type system is introduced.
* 
* [x] No AI-specific IR is introduced.
* 
* [x] No second quantum IR is introduced.
* 
* [x] quantum::ir remains the canonical quantum boundary.
* 
* [x] No runtime execution is performed.
* 
* [x] No Rust actions are embedded.
* 
* [x] No semantic predicates are embedded.
* 
* [x] No unsafe Rust is required.
* 
* [x] Rust 1.97+ compatibility is preserved.
* 
* [x] POCO-REAF source portability is preserved.
* 
* [x] Resource/capability decisions remain downstream.
* 
* [x] Policy decisions remain downstream.
* 
* [x] Provenance remains downstream.
* 
* [x] Transfer strategy remains open-world.
* 
* [x] Canonical "learnStatement" remains the single source-level learning
* authority.
* 
* [ ] Semantic AST classification is implemented downstream.
* 
* [ ] Semantic transfer-learning representation is implemented downstream.
* 
* [ ] Transfer-learning conformance tests pass.
* 
* [ ] Cross-domain tests pass.
* 
* [ ] Scalability tests pass.
* 
* [ ] Determinism tests pass.
* 
* [ ] Compatibility tests pass.
* 
* ============================================================================
* FINAL ARCHITECTURAL INVARIANT
* ============================================================================
* 
* This file answers:
* 
* "Where does transfer-learning intent belong in the Zamani AI grammar
*  architecture?"
* 
* It does NOT answer:
* 
* "How is transfer learning implemented?"
* 
* "Which model is loaded?"
* 
* "Which parameters are frozen?"
* 
* "Which optimizer is used?"
* 
* "Which accelerator executes it?"
* 
* "Which device is selected?"
* 
* "How many workers are used?"
* 
* "How is quantum routing performed?"
* 
* "How is scheduling performed?"
* 
* "How is resilience implemented?"
* 
* Those responsibilities belong downstream.
* 
* The resulting architecture is:
* 
* one learning syntax
*      +
* open-world transfer semantics
*      +
* universal types/effects/capabilities/resources
*      +
* contracts/policies/provenance
*      +
* canonical semantic representation
*      +
* quantum::ir where required
*      +
* target-independent compilation
* 
* This preserves:
* 
* Program_Once
* Compile_Once
* Run_Everywhere
* Run_Anywhere
* Forever
* 
* subject to program semantics, declared requirements, capabilities, policies,
* implementation resources and target feasibility.
* 
* ============================================================================
* PRODUCTION GRAMMAR
* ============================================================================
  */

parser grammar AITransferLearning;

options {
tokenVocab = ZamaniLexer;
}

import
Learn
;

/*

* ============================================================================
* PUBLIC COMPOSITION BOUNDARY
* ============================================================================
* 
* This rule provides a stable parser-context identity for tooling and semantic
* analysis.
* 
* It deliberately delegates to the canonical universal learning statement.
* 
* IMPORTANT:
* 
* This rule is NOT a second alternative in AI.aiConstruct for the same source
* sequence. The canonical AI learning path remains:
* 
* AILearning.aiLearningConstruct
* 
* Semantic transfer classification occurs downstream.
* ============================================================================
  */

aiTransferLearningConstruct
: learnStatement
;
*/

 