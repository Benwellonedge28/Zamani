Zamani AI and Intelligent Computation Specification

Path: "grammar/spec/ai.md"
Language: Zamani
Specification status: Normative
Specification layer: AI, machine learning, reasoning, knowledge, uncertainty, intelligent computation, model computation, and AI-domain integration semantics
Rust baseline: Rust 1.97 or later
Rust edition: Rust 2021
Rust safety requirement: Safe Rust only; "unsafe" is prohibited
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

0. Document Contract

0.1 Purpose

This document defines the normative semantic contract for AI and intelligent computation in Zamani.

It specifies how AI-domain computation participates in the existing universal Zamani architecture without creating a separate programming language, type system, effect system, resource system, capability system, policy system, AST, or canonical IR.

This specification covers:

- model computation;
- tensor computation;
- datasets;
- training;
- inference;
- evaluation;
- automatic differentiation;
- optimization intent;
- reasoning;
- inference and deduction;
- knowledge;
- assertions and retractions;
- queries;
- uncertainty;
- probability;
- confidence;
- evidence;
- provenance;
- explanations;
- decisions;
- learning;
- controlled adaptation;
- agents;
- multi-agent computation;
- neural-symbolic composition;
- planning;
- feedback;
- AI pipelines;
- model composition;
- AI/classical interoperability;
- AI/quantum interoperability;
- AI/HDL interoperability;
- AI/hardware interoperability;
- AI/distributed interoperability;
- AI/networking interoperability;
- AI/data interoperability;
- AI/security boundaries;
- AI resource requirements;
- AI capability requirements;
- AI effects;
- AI contracts;
- AI policies;
- simulation;
- deterministic and reproducible execution;
- portability;
- target-independent specialization;
- extensibility;
- dialect integration.

This specification does not define a second language inside Zamani.

AI is a semantic domain of Zamani.

---

0.2 Normative terminology

The words:

- MUST
- MUST NOT
- REQUIRED
- SHALL
- SHALL NOT
- SHOULD
- SHOULD NOT
- MAY

are normative.

A feature described as experimental, proposed, planned, deprecated, or historical elsewhere in the repository is not automatically stable merely because this specification describes its semantic model.

---

0.3 Stable-feature rule

An AI feature becomes stable only when the complete implementation path exists:

specification
    ↓
lexical contract, if required
    ↓
grammar owner
    ↓
domain-neutral AST
    ↓
semantic validation
    ↓
type/effect/capability/resource analysis
    ↓
contracts/policy/provenance
    ↓
canonical semantic representation
    ↓
canonical IR
    ↓
lowering
    ↓
optimization
    ↓
routing/scheduling where applicable
    ↓
runtime/target realization
    ↓
positive tests
    ↓
negative tests
    ↓
boundary tests
    ↓
scalability tests
    ↓
compatibility tests
    ↓
determinism/reproducibility tests where applicable

No AI feature is considered production-ready merely because its parser rule exists.

---

1. Architectural Authority

The architecture is governed first by:

grammar/DESIGN.md

This specification is subordinate to the repository-wide architectural authority and is normative for the AI domain only.

The principal integration chain is:

Zamani source
      │
      ▼
canonical lexer
      │
      ▼
grammar/Zamani.g4
      │
      ▼
domain-neutral frontend AST
      │
      ▼
structural validation
      │
      ├── type analysis
      ├── effect analysis
      ├── capability analysis
      ├── resource analysis
      ├── ownership/linearity analysis
      ├── contract analysis
      ├── policy analysis
      └── provenance analysis
      │
      ▼
canonical semantic model
      │
      ├── classical semantics
      ├── AI semantics
      ├── quantum semantics
      ├── HDL semantics
      ├── distributed semantics
      ├── networking semantics
      └── other domains
      │
      ▼
canonical IR
      │
      ├── Classical IR
      ├── quantum::ir
      ├── HDL/hardware IR
      └── other canonical domain IR where formally established
      │
      ▼
optimization / specialization / lowering
      │
      ▼
routing / placement / scheduling
      │
      ▼
resilience / recovery / QEC where applicable
      │
      ▼
ZQN
      │
      ▼
HAL
      │
      ▼
target realization

AI participates in this pipeline.

AI MUST NOT bypass it.

---

2. Ownership Contract

2.1 This specification owns

This file owns the normative semantic meaning of:

- AI model computation;
- tensor/model relationships;
- learning intent;
- training intent;
- inference intent;
- evaluation intent;
- differentiation intent;
- reasoning;
- deduction;
- inference;
- knowledge;
- evidence;
- uncertainty;
- probability;
- confidence;
- explainability;
- decision semantics;
- controlled adaptation;
- AI agents;
- AI pipeline semantics;
- neural-symbolic composition;
- AI resource intent;
- AI capability intent;
- AI-specific semantic constraints;
- AI-specific portability rules;
- AI-specific provenance requirements;
- AI-specific security boundaries;
- AI interoperability semantics;
- AI scalability requirements.

---

2.2 This specification does not own

This file does NOT own:

- token spelling;
- lexer implementation;
- parser implementation;
- root grammar composition;
- general expression syntax;
- general statement syntax;
- general type syntax;
- general effect syntax;
- general resource syntax;
- general capability syntax;
- general contract syntax;
- general policy syntax;
- general provenance syntax;
- actor syntax;
- channel syntax;
- generic quantum syntax;
- HDL syntax;
- hardware placement;
- scheduler implementation;
- router implementation;
- QEC implementation;
- ZQN implementation;
- HAL implementation;
- runtime implementation;
- numerical kernels;
- tensor storage;
- accelerator APIs;
- vendor APIs;
- model-serving infrastructure;
- database implementation;
- network implementation;
- operating-system services.

Those remain owned by their existing repository contracts.

---

3. Repository Integration Ownership

The following ownership is normative.

Concern| Owner
Architecture| "grammar/DESIGN.md"
Grammar navigation| "grammar/README.md"
Root grammar composition| "grammar/Zamani.g4"
Lexical authority| "grammar/lexer/", "grammar/antlr/ZamaniLexer.g4"
AI semantic contract| "grammar/spec/ai.md"
General syntax| "grammar/spec/syntax.md"
General semantics| "grammar/spec/semantics.md"
Types| "grammar/spec/type-system.md", "grammar/types/"
Effects| "grammar/spec/effects.md", "grammar/effects/"
Resources| "grammar/spec/resources.md", "grammar/resources/"
Portability| "grammar/spec/portability.md"
Contracts| "grammar/validation/", corresponding specification
Policies| "grammar/policies/", corresponding specification
Provenance| repository-wide provenance contract
AI syntax| "grammar/ai/"
Classical syntax/semantics| "grammar/classical/"
Quantum syntax/semantics| "grammar/quantum/", "quantum::ir"
Hybrid semantics| "grammar/hybrid/"
Data| "grammar/data/"
Distributed computation| "grammar/distributed/"
Concurrency/actors| "grammar/concurrency/"
Networking| "grammar/networking/"
HDL| "grammar/hdl/"
Hardware| "grammar/hardware/"
Interoperability| "grammar/interoperability/"
Dialects| "grammar/dialects/"
Metaprogramming| "grammar/metaprogramming/"
Compilation intent| "grammar/compile/"
Execution| "grammar/execution/"
Security| "grammar/security/"
Compatibility| "grammar/compatibility/"
Tests| "grammar/tests/" and repository test infrastructure
Source AST| canonical "src/ast/" / migration-designated AST owner
Semantic implementation| canonical semantic-analysis implementation
IR| canonical IR implementation

No AI grammar file may redefine these ownership boundaries.

---

4. AI Is Not a Separate Language

Zamani AI is ordinary Zamani computation with additional semantic capabilities.

AI computation MUST be composable with:

- functions;
- modules;
- types;
- generics;
- records;
- collections;
- pattern matching;
- contracts;
- effects;
- resources;
- capabilities;
- policies;
- concurrency;
- distributed execution;
- classical computation;
- quantum computation;
- HDL;
- hardware intent;
- networking;
- data processing;
- metaprogramming;
- interoperability.

An AI computation MAY occur:

inside a function
inside a module
inside a task
inside an actor
inside a distributed computation
inside a classical computation
inside a quantum-classical hybrid computation
inside a data pipeline
inside a simulation
inside an HDL/hardware co-design
inside a compile-time computation where explicitly permitted

No separate AI execution language is introduced.

---

5. Core Principle

The fundamental rule is:

«AI syntax expresses computational meaning and portable intent; compilation and execution determine how that meaning is realized.»

Source code MAY specify:

- what computation is required;
- what mathematical relationships exist;
- what data is consumed;
- what outputs are produced;
- what model is used;
- what training objective exists;
- what evidence supports a result;
- what confidence is associated with a result;
- what capabilities are required;
- what resources are required;
- what policies constrain execution;
- what contracts must hold;
- what provenance must be retained;
- what reproducibility properties are required.

Source code MUST NOT require a particular realization merely because that realization exists on the development machine.

Portable AI code MUST NOT intrinsically require:

- a particular GPU;
- a particular CPU;
- a particular accelerator;
- a particular QPU;
- a particular FPGA;
- a particular device number;
- a particular memory bank;
- a particular cluster node;
- a particular interconnect;
- a particular physical qubit;
- a particular cache;
- a particular register width.

Target-specific realization belongs downstream or in an explicitly target-specific dialect.

---

6. POCO-REAF AI Contract

POCO-REAF applies to AI exactly as it applies to every other Zamani domain.

The semantic goal is:

Program Once
     ↓
Compile Once
     ↓
Run Everywhere
     ↓
Run Anywhere
     ↓
Run Forever

subject to:

- semantic validity;
- target capability;
- available resources;
- declared constraints;
- security policy;
- compatibility;
- numerical semantics;
- execution feasibility;
- physical feasibility.

"Forever" does not mean that hardware never fails.

It means that the source-level semantics do not expire merely because:

- a processor generation changes;
- an accelerator changes;
- a memory architecture changes;
- a network topology changes;
- a model implementation changes;
- a compiler optimization changes;
- a target becomes unavailable.

A valid implementation MAY report that a target cannot satisfy the program.

It MUST NOT silently change program meaning merely to make the program execute.

---

7. Scalability Contract

AI semantics MUST scale from the smallest meaningful computation to arbitrarily large computations subject to available resources and explicit implementation policies.

The language MUST NOT impose universal artificial limits on:

models
model parameters
layers
components
tensor rank
tensor dimensions
features
inputs
outputs
datasets
records
training steps
epochs
batches
agents
pipeline stages
knowledge assertions
relations
evidence items
provenance records
reasoning premises
reasoning conclusions
query terms
workers
devices
nodes
accelerators
memory
storage
network size

The following classes of constants MUST NOT appear as language-semantic ceilings:

MAX_MODELS
MAX_TENSORS
MAX_LAYERS
MAX_PARAMETERS
MAX_FEATURES
MAX_INPUTS
MAX_OUTPUTS
MAX_BATCH_SIZE
MAX_SEQUENCE_LENGTH
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_AGENTS
MAX_DATASET_SIZE
MAX_WORKERS
MAX_DEVICES
MAX_ACCELERATORS
MAX_NODES
MAX_TRAINING_STEPS
MAX_EPOCHS
MAX_MODEL_DEPTH
MAX_MODEL_WIDTH
MAX_KNOWLEDGE
MAX_FACTS
MAX_RELATIONS

An implementation MAY have resource limits.

Such limits are not language semantics.

For example:

RESOURCE_EXHAUSTED
COMPILATION_RESOURCE_EXHAUSTED
EXECUTION_RESOURCE_EXHAUSTED
CAPABILITY_UNAVAILABLE
POLICY_DENIED

are valid implementation/execution outcomes.

They MUST NOT be confused with syntax or type errors.

---

8. Semantic Infinity

Zamani uses "infinity" in the POCO-REAF sense of an open-ended semantic domain.

It means:

«The language does not establish an artificial finite machine-scale ceiling.»

It does NOT mean:

- infinite physical memory;
- infinite computation;
- infinite storage;
- infinite bandwidth;
- infinite energy;
- infinite execution time;
- infinite physical qubits.

Every concrete execution remains subject to:

available resources
capabilities
policies
implementation limits
physical feasibility
mathematical validity

The source language remains open-ended even when a particular target is not.

---

9. Semantic Separation of Concerns

AI constructs MUST distinguish:

9.1 Meaning

What computation is being expressed.

9.2 Requirement

What must be available.

Example:

requires capability("tensor.compute");

9.3 Resource

How much resource is required.

Example:

requires memory >= required_memory;

9.4 Constraint

What must remain true.

9.5 Preference

What realization is preferred.

9.6 Policy

What realization is permitted.

9.7 Implementation

How the computation is actually realized.

The first six are portable semantic information.

Implementation decisions belong downstream.

---

10. AI Capability Model

AI MAY require capabilities such as:

tensor.compute
tensor.autodiff
tensor.dynamic_shape
model.training
model.inference
model.evaluation
distributed.training
distributed.inference
streaming.data
accelerator.compute
secure.compute
quantum.hybrid
quantum.measurement
symbolic.reasoning
probabilistic.compute

These are examples of semantic capability identifiers.

They are NOT an exhaustive enumeration.

New capability identifiers MUST be extensible without modifying the universal AI grammar merely because a new implementation is introduced.

A capability MAY be provided by:

- software;
- CPU;
- GPU;
- FPGA;
- ASIC;
- accelerator;
- QPU;
- simulator;
- cluster;
- distributed system;
- future computational substrate.

The compiler/runtime determines satisfaction.

---

11. AI Resource Model

AI resource requirements MAY describe:

- computation;
- memory;
- storage;
- communication;
- bandwidth;
- latency;
- throughput;
- energy;
- reliability;
- precision;
- parallelism;
- accelerator availability;
- distributed execution;
- quantum resources;
- classical resources.

AI MUST reuse the universal resource model.

AI MUST NOT introduce a second resource language.

The path is:

AI source intent
      ↓
resource requirement
      ↓
resource analysis
      ↓
capability negotiation
      ↓
target realization

---

12. Model Semantics

A model is a semantic computational object.

A model MAY contain:

- inputs;
- outputs;
- parameters;
- state;
- components;
- transformations;
- submodels;
- dependencies;
- mathematical operations;
- objectives;
- contracts;
- constraints;
- capabilities;
- resource requirements;
- metadata;
- provenance.

A model MUST NOT intrinsically identify a physical execution target.

Model semantics MUST remain independent of:

device identifier
CPU identifier
GPU identifier
accelerator identifier
memory address
physical node
physical topology
physical qubit

---

13. Model Interface

A model interface consists of typed semantic inputs and outputs.

The types MUST come from the universal Zamani type system.

AI-specific type notation, if introduced, MUST map into the universal type system rather than creating a parallel type universe.

A model MAY have any number of inputs and outputs subject to semantic validity and available implementation resources.

No grammar-level maximum exists.

---

14. Model Identity

Model semantic identity MUST NOT depend on:

- source formatting;
- memory address;
- compiler process ID;
- thread ID;
- device ID;
- physical location;
- compilation order;
- allocation address.

Canonical model identity MUST be deterministic for equivalent semantic models where the selected identity scheme requires determinism.

Implementation-local identifiers MAY exist but MUST NOT become portable source semantics.

---

15. Model Parameters

Parameters MAY be:

- trainable;
- non-trainable;
- compile-time;
- runtime;
- symbolic;
- generic;
- shape-dependent;
- resource-dependent where explicitly modeled.

Parameter count is not a language-level limit.

Parameter representation belongs downstream.

---

16. Model State

Model state MAY represent:

- learned parameters;
- optimizer state;
- recurrent state;
- persistent state;
- streaming state;
- execution state;
- application state.

State ownership and lifetime are governed by the universal:

- ownership;
- memory;
- type;
- execution;
- persistence;
- effect

contracts.

The AI grammar does not define storage implementation.

---

17. Model Composition

Models MAY compose into:

- sequential computation;
- branching computation;
- graph computation;
- nested models;
- ensembles;
- mixtures;
- reusable components;
- dynamically selected components where permitted.

Composition depth is not language-limited.

Composition MUST remain representable in the canonical semantic model.

---

18. Open-World Model Components

The language MUST NOT require a keyword for every future model architecture.

The semantic model MUST support extensible operations/components.

Examples include:

- dense computation;
- convolution;
- recurrence;
- attention;
- graph computation;
- probabilistic computation;
- symbolic computation;
- differentiable computation;
- quantum-enhanced computation;
- hardware-accelerated computation;
- user-defined components;
- future components.

Framework-specific operations belong in libraries or dialects unless there is a compelling language-wide semantic reason to promote them.

---

19. Tensor Semantics

A tensor is a typed mathematical/data object.

Conceptually:

Tensor<Shape, ElementType>

where "Shape" is semantic shape information.

Tensor semantics MAY include:

- element type;
- shape;
- rank;
- symbolic dimensions;
- dynamic dimensions;
- layout metadata;
- indexing;
- slicing;
- broadcasting;
- reduction;
- transformation;
- contraction;
- permutation;
- reshape.

The type system remains authoritative for type and shape semantics.

---

20. Tensor Rank

There is no universal maximum tensor rank.

The grammar MUST NOT enumerate:

Tensor1
Tensor2
Tensor3
...
TensorN

Tensor rank MUST be represented using recursive, parametric, symbolic, or otherwise extensible semantic structures.

A concrete compiler MAY have resource limitations.

Those limitations MUST NOT redefine tensor semantics.

---

21. Tensor Dimensions

Tensor dimensions MAY be:

- literal;
- constant;
- generic;
- symbolic;
- dependent;
- runtime-derived.

Example:

Tensor<Float, [batch, sequence, features]>

does not require the source program to know physical memory capacity.

The compiler MAY specialize symbolic dimensions after target/resource discovery.

---

22. Tensor Storage

AI syntax MUST NOT prescribe:

- row-major layout;
- column-major layout;
- tiled layout;
- blocked layout;
- accelerator layout;
- cache placement;
- memory-bank placement;
- VRAM placement.

Storage layout belongs to:

type semantics where logically observable
        +
lowering
        +
optimization
        +
target realization

If layout is semantically observable, it MUST be explicitly represented as semantic information rather than inferred from a particular target.

---

23. Tensor Operations

Tensor operations MUST be represented through open-ended semantic operations.

They MAY include:

- arithmetic;
- matrix operations;
- contraction;
- broadcasting;
- reduction;
- indexing;
- slicing;
- reshape;
- permutation;
- concatenation;
- convolution;
- attention;
- normalization;
- user-defined operations.

The universal grammar MUST NOT become a catalogue of every numerical or AI framework operation.

---

24. Dataset Semantics

A dataset is a semantic data source.

It MAY contain:

- records;
- features;
- labels;
- schemas;
- metadata;
- partitions;
- transformations;
- provenance;
- streaming information;
- persistence references;
- external references.

Dataset size is not a grammar-level limit.

Dataset storage is owned by the data/storage/interoperability/runtime systems.

---

25. Dataset Security

Declaring or using a dataset does not grant access.

For example:

requires capability("data.read");

expresses a requirement.

It does not grant permission.

Authorization remains owned by security/policy/capability systems.

---

26. Training Semantics

Training expresses optimization intent.

A training operation MAY identify:

- model;
- input data;
- target data;
- objective;
- loss;
- parameters;
- evaluation;
- validation;
- stopping condition;
- checkpoint intent;
- resource requirements;
- capabilities;
- policies;
- constraints;
- provenance requirements.

The grammar MUST NOT encode a fixed list of optimization algorithms.

Algorithms belong to semantic libraries, dialects, or implementation strategies.

---

27. Training Iterations

Training steps, epochs, batches, and iterations are semantic quantities.

They MAY be:

- literal;
- symbolic;
- runtime-derived;
- policy-controlled;
- resource-controlled;
- conditionally terminated.

No language-level maximum exists.

---

28. Training Reproducibility

Training MAY be nondeterministic.

Where reproducibility is required, the program/execution configuration MUST be capable of declaring the necessary reproducibility properties.

Potential provenance includes:

- model definition;
- dataset identity;
- dataset version;
- parameter initialization;
- random source;
- random seed where meaningful;
- algorithm identity;
- compiler identity;
- semantic version;
- execution profile;
- capability realization;
- numerical profile.

A random seed alone MUST NOT be considered sufficient if the selected execution model has other nondeterministic sources.

---

29. Inference

Inference represents model evaluation against input data.

Inference MAY specify:

- model;
- input;
- output;
- execution mode;
- constraints;
- capabilities;
- resources;
- confidence requirements;
- provenance requirements;
- policy.

Inference MUST remain independent of physical deployment.

---

30. Evaluation

Evaluation represents semantic assessment of a model or computation.

It MAY involve:

- metrics;
- reference values;
- expected properties;
- validation data;
- test data;
- contracts;
- evidence;
- confidence;
- provenance.

Evaluation results MUST be represented using normal Zamani values and semantic structures.

---

31. Automatic Differentiation

Automatic differentiation is a semantic transformation.

It MUST NOT be treated as a particular runtime implementation.

The language MAY express differentiation intent.

The compiler MAY realize it using:

- forward mode;
- reverse mode;
- mixed mode;
- symbolic transformation;
- operator transformation;
- generated derivatives;
- another semantically valid mechanism.

The selected implementation MUST preserve the specified mathematical semantics.

---

32. Gradient Semantics

Gradients MUST retain their semantic relationship to the differentiated computation.

A backend MUST NOT silently substitute a mathematically different derivative merely because the target lacks a direct implementation.

Numerical approximation MAY be used only when permitted by the declared semantic/numerical contract.

---

33. Reasoning

Reasoning is a semantic operation over information, evidence, assumptions, rules, or other semantic inputs.

Reasoning MAY include:

- deduction;
- induction;
- abduction;
- inference;
- causal reasoning;
- constraint reasoning;
- symbolic reasoning;
- probabilistic reasoning;
- hybrid neural-symbolic reasoning.

The grammar MUST NOT create independent expression hierarchies for every reasoning form.

Reasoning syntax belongs to the appropriate grammar owner under "grammar/ai/" and/or universal expression/statement owners.

---

34. Inference and Deduction

Inference and deduction are semantic relations.

An inference MAY contain:

premises
evidence
rules
assumptions
conclusion
confidence
provenance

Deduction MAY express a conclusion derived from explicitly provided premises and rules.

The exact syntax is owned by the corresponding grammar files.

This document defines the meaning, not the parser rule.

---

35. Knowledge

Knowledge is structured semantic information.

A knowledge item MAY contain:

subject
relation
object
context
source
evidence
confidence
validity
provenance
version

Knowledge MUST remain usable outside AI.

It MAY support:

- scientific computation;
- compiler reasoning;
- hardware capabilities;
- configuration;
- security facts;
- data systems;
- distributed systems;
- AI systems;
- quantum experiment metadata.

---

36. Assertions

An assertion adds or establishes a semantic knowledge claim.

An assertion MUST NOT automatically grant authority.

It MAY require:

- capability;
- policy;
- authorization;
- provenance;
- validation.

---

37. Retraction

Retraction removes or invalidates a previously established knowledge claim according to the applicable knowledge model.

Retraction MUST preserve appropriate provenance when auditability is required.

A retraction MUST NOT silently rewrite historical provenance.

Where the underlying knowledge system is immutable/event-sourced, retraction MAY be represented as a new semantic event rather than physical deletion.

---

38. Queries

Knowledge queries retrieve information according to their semantic query model.

A query MUST NOT automatically imply unrestricted access.

Queries may require:

capability
policy
authorization
effect
resource

General data-query syntax remains owned by "grammar/data/" and interoperability dialects.

AI knowledge queries MUST NOT create a competing SQL grammar.

---

39. Uncertainty

AI computations MAY represent uncertainty explicitly.

Semantic concepts MAY include:

- probability;
- confidence;
- belief;
- likelihood;
- interval;
- distribution;
- uncertainty;
- evidence strength.

The AI specification does not mandate one numerical representation.

---

40. Probability

Probability semantics MUST be mathematically defined by the selected semantic type/model.

The language MUST NOT assume:

- a fixed floating-point width;
- a fixed probability precision;
- a fixed distribution count;
- a particular sampling algorithm.

A probability implementation MAY use exact, symbolic, rational, floating-point, interval, arbitrary-precision, or other semantically valid representations.

---

41. Confidence

Confidence represents an epistemic or statistical property of a result.

Confidence MUST NOT automatically be interpreted as truth.

A confidence value MAY require metadata identifying:

- interpretation;
- source;
- method;
- calibration;
- evidence;
- model;
- data;
- provenance.

---

42. Evidence

Evidence is information supporting or challenging a claim, decision, inference, or transformation.

An evidence record MAY contain:

claim
source
observation
method
derivation
confidence
verification
provenance

Evidence is not restricted to AI.

Compiler transformations, quantum results, hardware verification, and security decisions MAY also use the universal evidence model.

---

43. Explainability

An explanation describes why a result, decision, transformation, or allocation occurred.

An explanation MAY apply to:

- AI inference;
- model decisions;
- compiler transformations;
- resource allocation;
- quantum routing;
- hardware mapping;
- optimization;
- security decisions;
- scheduling;
- adaptive execution.

The explanation mechanism MUST NOT imply that every implementation can expose every internal detail.

The declared explanation contract determines what must be available.

---

44. Decision Records

A decision record MAY include:

decision
inputs
evidence
rules
policy
constraints
alternatives
selected outcome
confidence
provenance

Decision records SHOULD be immutable or append-only where auditability is required.

The language MUST NOT assume a particular storage system.

---

45. Provenance

AI computation MUST integrate with the repository-wide provenance model.

Provenance MAY capture:

source
derived_from
generated_by
transformed_by
verified_by
reason
evidence
decision
version
execution_context

AI MUST NOT create an incompatible provenance model.

The universal provenance model remains authoritative.

---

46. Learning

Learning is a semantic state transformation or model-parameter transformation.

Learning MUST declare or inherit appropriate:

- inputs;
- outputs;
- model;
- data;
- objective;
- effects;
- capabilities;
- resources;
- policy;
- provenance.

Learning MUST NOT imply unrestricted program modification.

---

47. Controlled Adaptation

Adaptation is permitted only through explicit semantic control.

Adaptation MUST be distinguishable from unrestricted self-modifying code.

A valid adaptation flow is:

observation
    ↓
evaluation
    ↓
candidate adaptation
    ↓
policy validation
    ↓
capability validation
    ↓
effect validation
    ↓
resource validation
    ↓
contract validation
    ↓
authorization
    ↓
provenance record
    ↓
authorized state/model/strategy change

Adaptation MUST NOT silently modify:

- language semantics;
- compiler trust boundaries;
- security policy;
- authorization;
- immutable source history;
- executable code outside declared capabilities.

Code generation or reflection requires the applicable metaprogramming capabilities and effects.

---

48. Adaptation Scope

Adaptation MAY operate on:

- model parameters;
- model state;
- strategy selection;
- execution plans;
- resource selection;
- scheduling choices;
- learned policies;
- permitted configuration.

Whether a specific object is adaptable is determined by its type, ownership, policy, effects, and capabilities.

---

49. Agents

An AI agent is a semantic composition of:

state
observation
decision
action
policy
knowledge
memory
communication

An agent does not create a second concurrency system.

---

50. Multi-Agent Computation

AI agents MUST integrate with the existing concurrency and actor architecture.

The preferred semantic relationship is:

AI agent
    ↓
actor/task
    ↓
message/channel
    ↓
scheduler/runtime

"grammar/concurrency/" remains the owner of general actor lifecycle and concurrency semantics.

AI defines the semantic meaning of an AI agent, not a competing actor model.

---

51. Agent Communication

Agent communication MAY use:

- messages;
- channels;
- streams;
- distributed services;
- network endpoints.

Communication MUST carry the applicable:

- type;
- effect;
- capability;
- resource;
- policy;
- security;
- provenance

information.

---

52. Agent Scaling

The language MUST NOT define a maximum number of agents.

An implementation MAY schedule one or many agents depending on resources.

The same source-level agent model MUST remain valid when realized as:

one local task
many local tasks
multicore execution
GPU/accelerator execution
cluster execution
distributed execution
future execution substrate

provided the semantics remain valid.

---

53. Neural-Symbolic Computation

Neural-symbolic computation combines:

learned computation
        +
symbolic computation
        +
knowledge
        +
reasoning

The composition MUST use existing:

- model semantics;
- tensor semantics;
- type semantics;
- knowledge semantics;
- reasoning semantics;
- effect semantics;
- capability semantics;
- resource semantics;
- provenance semantics.

No second semantic universe is introduced.

---

54. Planning

Planning is a semantic computation over:

state
goals
constraints
actions
knowledge
policies
resources

Planning MAY be:

- deterministic;
- probabilistic;
- symbolic;
- learned;
- hybrid.

The grammar MUST NOT enumerate every planning algorithm.

---

55. Feedback

Feedback MAY connect:

execution
    ↓
observation
    ↓
evaluation
    ↓
learning/adaptation

Feedback MUST obey:

- effects;
- policies;
- resource constraints;
- provenance requirements;
- authorization.

Feedback does not automatically imply adaptation.

---

56. AI Pipelines

AI pipelines are semantic compositions of operations.

A pipeline MAY contain:

- data acquisition;
- preprocessing;
- transformation;
- inference;
- training;
- evaluation;
- reasoning;
- decision;
- postprocessing;
- persistence;
- communication.

Pipeline stages MUST use the universal type and effect system.

---

57. Pipeline Parallelism

Pipeline parallelism is an execution strategy.

The source semantics describe dependency and ordering.

The compiler/runtime MAY realize the pipeline through:

- sequential execution;
- tasks;
- actors;
- threads;
- SIMD;
- vectorization;
- GPU execution;
- accelerator execution;
- distributed execution.

The source program MUST NOT require one realization unless explicitly constrained.

---

58. Classical Integration

AI computation MAY invoke ordinary classical operations.

The classical subsystem remains authoritative for:

- arithmetic;
- control flow;
- functions;
- memory;
- collections;
- ordinary data;
- numerical primitives.

AI MUST NOT duplicate these semantics.

---

59. Quantum Integration

AI MAY interact with quantum computation through hybrid semantics.

Examples include:

classical control → quantum operation
quantum measurement → classical value
quantum state → model input
model output → quantum parameter
quantum result → learning objective

Quantum syntax remains owned by "grammar/quantum/".

The canonical quantum semantic boundary remains:

quantum::ir

AI MUST NOT define:

- physical qubit identifiers;
- coupling maps;
- gate catalogues;
- routing;
- QEC;
- calibration;
- QPU placement.

---

60. Quantum Machine Learning

Quantum-enhanced learning is represented as a composition of:

AI learning semantics
        +
hybrid semantics
        +
quantum semantics

The implementation path MAY be:

AI source
    ↓
domain-neutral AST
    ↓
semantic model
    ↓
classical semantics
    +
quantum semantics
    ↓
Classical IR + quantum::ir
    ↓
optimization
    ↓
routing/scheduling

No separate QML IR is required merely because the source combines AI and quantum operations.

---

61. HDL and Hardware Integration

AI MAY describe hardware-accelerated computation.

The AI layer may express:

- computational intent;
- accelerator capability;
- throughput requirements;
- latency requirements;
- precision requirements;
- resource preferences.

HDL and hardware subsystems own:

- signals;
- hardware structure;
- timing;
- synthesis;
- physical realization;
- hardware topology.

AI MUST NOT define hardware widths or device counts as universal constants.

---

62. Distributed Integration

AI MAY execute in distributed environments.

Distributed semantics remain owned by:

grammar/distributed/
grammar/concurrency/
grammar/networking/

AI may express intent such as:

distributed training
distributed inference
distributed evaluation
distributed data processing

but must not define:

MAX_NODES
MAX_WORKERS
MAX_DEVICES

or equivalent language-level ceilings.

---

63. Networking Integration

AI network operations MUST participate in the universal effect/capability model.

For example, network-backed inference may require:

effect(network)
capability(network.service)

and appropriate policy.

AI syntax MUST NOT silently grant network access.

---

64. Data Integration

AI consumes and produces normal Zamani data.

Data may originate from:

- memory;
- files;
- databases;
- streams;
- network services;
- sensors;
- quantum measurements;
- hardware;
- external systems.

The data subsystem remains authoritative for generic data semantics.

---

65. SQL, JSON, XML, and Other Formats

AI MUST NOT embed independent SQL, JSON, XML, or other format grammars into the AI grammar.

Instead:

format/dialect
      ↓
interoperability semantic model
      ↓
Zamani data model
      ↓
AI computation

This prevents application and interchange formats from becoming permanent core-language syntax.

---

66. FFI and ABI

AI MAY call foreign implementations when permitted by the universal interoperability system.

Foreign calls MUST participate in:

- type checking;
- effects;
- capabilities;
- security policy;
- resource analysis;
- provenance where required.

An AI declaration of a foreign operation does not grant execution authority.

---

67. Reflection

AI MAY use controlled reflection when explicitly permitted.

Reflection MUST be constrained by:

- capability;
- effect;
- policy;
- type rules;
- security;
- provenance.

Reflection MUST NOT silently become unrestricted arbitrary code execution.

---

68. Metaprogramming

AI-generated or AI-assisted code generation belongs to the existing metaprogramming system.

The semantic distinction is:

model inference
    ≠
code generation

If AI causes code generation, the resulting operation MUST cross the normal metaprogramming and code-generation boundaries.

---

69. Contracts

AI computations MAY participate in:

requires
ensures
invariant
assume
guarantee
property
assert

The contract system remains universal.

Examples of AI properties include:

- tensor shape compatibility;
- output type;
- model invariants;
- probability bounds;
- confidence constraints;
- resource requirements;
- safety properties;
- reproducibility requirements.

AI MUST NOT invent a separate contract language.

---

70. Effects

AI operations MUST use the universal effect model.

Potential effects include:

io
network
mutation
randomness
native
foreign
distributed
measurement
quantum
learning
adaptation
reflection
code_generation
simulation

An operation's effects MUST be explicit or inferable according to the universal effect rules.

The mere fact that an operation is "AI" does not exempt it from effect analysis.

---

71. Capability Security

AI capabilities are requirements, not permissions.

The semantic distinction is:

requires capability(...)

versus:

authorization granted

Capability resolution and authorization occur downstream.

An AI model cannot bypass security by declaring a capability.

---

72. Policies

AI computation MUST integrate with the universal policy system.

Policies MAY constrain:

- model selection;
- model adaptation;
- dataset access;
- network access;
- resource usage;
- deployment;
- inference;
- learning;
- agent actions;
- code generation;
- reflection;
- foreign calls;
- quantum execution;
- distributed execution.

Policies are not AI-specific merely because AI consumes them.

---

73. Sandboxing

AI execution MAY occur in a sandbox.

A sandbox MAY restrict:

- effects;
- capabilities;
- resources;
- filesystem;
- network;
- native calls;
- foreign calls;
- reflection;
- adaptation;
- code generation.

The security subsystem owns sandbox semantics.

AI consumes sandbox policies.

---

74. Simulation

AI computation MAY be executed in simulation.

Simulation may represent:

- classical computation;
- tensor execution;
- model execution;
- distributed execution;
- quantum execution;
- hardware behavior;
- faults;
- performance;
- resource behavior.

Simulation is an execution strategy, not a separate source language.

---

75. Adaptive Execution

AI MAY participate in adaptive execution.

A generic adaptive execution model is:

observe
    ↓
evaluate
    ↓
select
    ↓
execute
    ↓
observe
    ↓
retry/recover/fallback/adapt

The execution subsystem remains authoritative for retry/recovery semantics.

AI may supply decision logic, but cannot silently override system-level policy.

---

76. Resilience

AI execution MUST integrate with the existing resilience model.

Where applicable, execution state MAY include:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

Outcomes MAY include:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

AI MUST NOT redefine these globally.

---

77. Determinism

AI computation MAY be:

- deterministic;
- nondeterministic;
- probabilistic;
- hardware-dependent within declared semantics;
- intentionally stochastic.

The semantic model MUST distinguish these cases.

A deterministic declaration MUST constrain all relevant nondeterministic sources sufficiently for the selected reproducibility profile.

---

78. Reproducibility

Reproducibility MAY include:

- source identity;
- semantic version;
- model identity;
- data identity;
- data version;
- random-state information;
- numerical profile;
- execution policy;
- capability realization;
- compiler/IR version;
- relevant external dependencies.

Reproducibility requirements MUST remain compatible with POCO-REAF.

A reproducible source program MUST NOT depend on undocumented hardware behavior.

---

79. Numerical Portability

Numerical semantics MUST be explicit where differences could affect observable behavior.

A compiler MUST NOT silently replace:

exact

with:

approximate

or:

strict precision

with:

implementation-selected precision

when the distinction is semantically observable.

Optimization MAY transform numerical computation only within the declared numerical contract.

---

80. Model Serialization

Model serialization is an interoperability concern.

A serialized model MUST preserve all semantic information required by its declared contract.

Serialization format MUST NOT redefine the source-level model semantics.

Model formats MAY be implemented as dialects/adapters.

---

81. Model Versioning

Model versions MAY be represented through the universal compatibility/provenance mechanisms.

Model version identity MUST NOT be confused with:

- language version;
- grammar version;
- AST version;
- IR version;
- compiler version.

Where a model depends on a particular semantic contract, that dependency MUST be represented explicitly.

---

82. Model Evolution

Model evolution MAY change:

- parameters;
- architecture;
- metadata;
- learned state;
- interfaces.

Evolution MUST respect type, compatibility, policy, and provenance requirements.

A model cannot silently evolve into an incompatible semantic type.

---

83. Knowledge and Learning Integration

Knowledge MAY provide:

facts
observations
labels
rules
evidence
features
metadata
prior information

Learning MAY produce:

model state
learned parameters
predictions
rules
knowledge
evidence
provenance

The relationship is:

knowledge
    ↓
learning
    ↓
model
    ↓
inference
    ↓
result
    ↓
knowledge/evidence/decision

This is a semantic relationship, not a requirement that every implementation use a particular storage architecture.

---

84. Reasoning and Learning Integration

Learning and reasoning MAY compose.

Examples:

learn → infer
learn → reason
reason → learn
knowledge → learn
learn → knowledge
model → reason
reason → model selection

Each boundary MUST preserve the appropriate:

- types;
- effects;
- capabilities;
- resources;
- policies;
- provenance.

---

85. Adaptation and Learning Integration

Learning MAY provide candidates for adaptation.

Adaptation MUST still pass through:

policy
capability
effect
resource
contract
authorization
provenance

Learning MUST NOT automatically authorize executable self-modification.

---

86. Agents and Knowledge Integration

Agents MAY use knowledge.

Knowledge MAY be:

- local;
- shared;
- replicated;
- remote;
- persistent;
- streaming;
- versioned.

Access remains subject to security and policy.

---

87. Agents and Learning Integration

Agents MAY learn.

Learning MAY update agent state/model parameters subject to:

- ownership;
- effects;
- adaptation policy;
- resource requirements;
- provenance;
- security.

---

88. Agents and Adaptation Integration

An agent MAY adapt its permitted behavior.

It MUST NOT:

- escape its security policy;
- obtain undeclared capabilities;
- modify immutable policy;
- silently change language semantics;
- bypass authorization.

---

89. AI and Compiler Reasoning

AI computation may assist compiler reasoning, but AI must not silently become part of compiler correctness.

Compiler transformations remain valid only when independently justified by the compiler's semantic rules.

An AI-generated optimization candidate is not automatically a valid optimization.

---

90. AI-Assisted Compilation

AI MAY assist:

- optimization search;
- scheduling search;
- lowering selection;
- resource planning;
- model specialization;
- code generation;
- diagnostics.

Any generated result MUST pass the same semantic validation required of a non-AI-generated result.

AI assistance does not create a privileged compiler trust path.

---

91. AI and Hardware Discovery

AI source MAY declare hardware capabilities required for execution.

Hardware discovery remains downstream.

The source program MUST NOT require knowledge of:

physical device IDs
physical core IDs
physical memory addresses
physical accelerator numbers
physical node numbers

unless the program intentionally enters a target-specific realization layer.

---

92. AI and Resource Negotiation

AI resource negotiation follows:

semantic requirement
       ↓
candidate capabilities
       ↓
resource availability
       ↓
policy filtering
       ↓
constraint checking
       ↓
preference ranking
       ↓
realization

The compiler/runtime may select among multiple valid realizations.

The source meaning remains unchanged.

---

93. Preferences

AI MAY express preferences such as:

prefer capability("accelerated.tensor.compute");

A preference is not a requirement.

If a preference cannot be satisfied, execution MAY continue with another valid realization unless a separate constraint forbids that realization.

---

94. Constraints

AI constraints MAY express:

- latency;
- throughput;
- precision;
- energy;
- reliability;
- locality;
- determinism;
- reproducibility;
- cost;
- availability.

Constraints are semantic conditions.

They are not device identifiers.

---

95. Fallbacks

AI execution MAY have fallback strategies.

Examples:

accelerated implementation
        ↓
software implementation

or:

quantum realization
        ↓
approved simulation

A fallback MUST preserve the semantic contract.

If it cannot, the compiler/runtime MUST report incompatibility rather than silently changing meaning.

---

96. Target-Specific Extensions

AI implementations MAY provide target-specific dialects.

A dialect MAY expose:

- vendor operations;
- accelerator intrinsics;
- special memory layouts;
- target-specific optimization controls;
- hardware-specific capabilities.

A dialect MUST NOT redefine portable Zamani AI semantics.

Portable source remains independent of the dialect unless it explicitly imports or requests it.

---

97. Open-World Extension Rule

The AI semantic universe is open-ended.

Adding a new:

- model architecture;
- learning algorithm;
- tensor operation;
- accelerator;
- inference backend;
- knowledge provider;
- reasoning algorithm;
- probability representation;
- agent implementation;
- distributed strategy;
- hardware target

MUST NOT require modification of this specification unless the addition changes the language-wide semantic contract.

This is essential for long-term POCO-REAF compatibility.

---

98. Application-Specific Features

Application concepts such as:

- computer vision;
- robotics;
- language processing;
- sentiment analysis;
- payment systems;
- administration;
- legal workflows;
- virtual/augmented reality;
- blockchain;
- domain-specific business logic;
- specialized scientific workflows

MUST NOT become universal AI keywords merely because they are useful applications.

They belong in:

libraries
dialects
capability namespaces
policies
frameworks
applications

The universal language remains domain-neutral.

---

99. No Keyword Explosion

AI MUST NOT create a keyword for every:

- model;
- algorithm;
- framework;
- architecture;
- application;
- vendor;
- device;
- dataset format;
- domain.

The preferred representation is:

generic semantic construct
        +
identifier
        +
metadata
        +
capability
        +
library/dialect registration

This keeps the language extensible.

---

100. Lexer Integration

This specification does not own lexical tokens.

Any AI-related keyword MUST be introduced only through the canonical lexical authority.

If an identifier is sufficient, it SHOULD remain an identifier.

The addition of a new AI library/model/algorithm MUST NOT require a new lexer token.

---

101. Grammar Integration

AI grammar belongs under:

grammar/ai/

AI grammar files MUST:

- declare ownership;
- declare dependencies;
- import universal grammar where applicable;
- avoid duplicate expression hierarchies;
- avoid duplicate type systems;
- avoid duplicate resource systems;
- avoid duplicate effect systems;
- avoid duplicate policy systems;
- avoid duplicate provenance systems;
- avoid target-specific assumptions.

The root grammar:

grammar/Zamani.g4

remains the composition authority.

---

102. AST Integration

AI syntax MUST lower into the canonical domain-neutral frontend AST.

AI grammar MUST NOT create a second independent AST.

The AST MUST preserve enough information for:

- type analysis;
- effect analysis;
- capability analysis;
- resource analysis;
- contract analysis;
- policy analysis;
- provenance;
- semantic lowering.

Source spans MUST be preserved.

---

103. Semantic Model Integration

AI semantic analysis MUST produce semantic objects that can participate in the common semantic model.

AI semantic nodes MAY represent:

Model
Tensor
Dataset
Training
Inference
Reasoning
Knowledge
Evidence
Decision
Adaptation
Agent
Pipeline

These are semantic concepts.

They MUST NOT become a second compiler architecture.

---

104. Type Integration

AI types MUST use the universal type system.

Examples include conceptual forms such as:

Tensor<S, T>
Model<I, O>
Distribution<T>
Probability<T>
Confidence<T>
Agent<State, Action>

where appropriate.

The exact type constructors are governed by "grammar/spec/type-system.md".

AI MUST NOT redefine:

- generic semantics;
- ownership;
- linearity;
- affine behavior;
- dependent constraints;
- type equality.

---

105. Effect Integration

AI semantic operations MUST participate in the universal effect system.

Examples:

learn
adapt
infer
query
foreign model call
network inference
random sampling
simulation
quantum measurement

may carry different effects.

An implementation MUST analyze the effects according to the universal effect specification.

---

106. Resource Integration

AI resource requirements MUST use the universal resource model.

No AI-specific resource ceiling may be introduced.

Examples of portable intent include:

requires memory >= required_memory;
requires capability("tensor.compute");
requires capability("distributed.training");
requires topology(required_topology);

The actual realization is downstream.

---

107. Capability Integration

AI capability identifiers SHOULD use hierarchical namespaces where appropriate.

Examples:

ai.inference
ai.training
ai.reasoning
ai.learning
tensor.compute
tensor.autodiff
model.inference
model.training
distributed.training
quantum.hybrid

Capability naming is open-ended.

Capability names MUST NOT encode a specific hardware instance.

---

108. Contract Integration

AI constructs MAY carry:

requires
ensures
invariant
assume
guarantee
property

AI-specific contracts MUST remain ordinary Zamani contracts.

The contract system is not duplicated.

---

109. Policy Integration

AI constructs MAY consume policy information.

Policy MAY determine whether:

- learning is permitted;
- adaptation is permitted;
- a dataset may be accessed;
- a model may be deployed;
- an agent may act;
- code may be generated;
- reflection may occur;
- foreign calls may occur;
- network access is permitted.

Policy enforcement is downstream from parsing.

---

110. Provenance Integration

Every AI transformation that requires auditability MUST preserve appropriate provenance.

At minimum, provenance MAY identify:

source construct
semantic transformation
input identity
output identity
reason
evidence
policy
execution context
version

The exact storage mechanism is outside this specification.

---

111. Canonical IR Contract

AI MUST NOT automatically create a new universal AI IR.

The preferred lowering is:

AI source
    ↓
domain-neutral AST
    ↓
semantic AI model
    ↓
canonical IR

AI computation MAY lower into:

Classical IR
quantum::ir
HDL/hardware IR
distributed representations
data representations

as required by the actual semantics.

---

112. Classical IR Boundary

Classical AI computation SHOULD lower to the canonical classical representation where appropriate.

Examples:

- arithmetic;
- control;
- memory;
- ordinary tensor operations;
- data transformation;
- classical inference.

---

113. Quantum IR Boundary

Quantum computation MUST cross the canonical:

quantum::ir

boundary.

AI MUST NOT create:

AIQuantumIR
QMLIR
AIQIR

as competing universal representations unless the repository explicitly establishes such a representation as part of canonical IR architecture.

---

114. HDL/HW IR Boundary

Hardware-oriented AI computation MAY lower into the established HDL/hardware representation.

The AI specification does not own:

- synthesis;
- placement;
- routing;
- timing closure;
- physical realization.

---

115. Optimization

AI optimization MAY include:

- constant propagation;
- graph simplification;
- common-subexpression elimination;
- fusion;
- tiling;
- vectorization;
- parallelization;
- specialization;
- differentiation transformation;
- model specialization.

Optimization MUST preserve semantic contracts.

---

116. Lowering

Lowering MAY specialize an AI program based on:

- target capabilities;
- available resources;
- declared constraints;
- policies;
- numerical profile;
- execution environment.

Lowering MUST NOT alter source semantics merely because a target is smaller or different.

---

117. Routing and Scheduling

Routing and scheduling remain downstream.

For distributed AI:

AI semantic dependency
      ↓
distributed representation
      ↓
routing
      ↓
scheduling

For quantum-enhanced AI:

quantum::ir
      ↓
decomposition
      ↓
routing
      ↓
scheduling

AI does not own these mechanisms.

---

118. Runtime Contract

Runtime MAY provide:

- model execution;
- data loading;
- memory management;
- scheduling;
- distributed execution;
- accelerator execution;
- checkpointing;
- monitoring;
- recovery.

Runtime behavior MUST remain consistent with the semantic model.

Runtime MAY reject execution when required capabilities/resources are unavailable.

---

119. Error Model

AI compilation/execution failures MUST be classified accurately.

Possible categories include:

SYNTAX_ERROR
TYPE_ERROR
SHAPE_ERROR
EFFECT_ERROR
CAPABILITY_ERROR
RESOURCE_ERROR
CONTRACT_ERROR
POLICY_ERROR
PROVENANCE_ERROR
COMPATIBILITY_ERROR
NUMERICAL_ERROR
EXECUTION_ERROR
RESOURCE_EXHAUSTED
UNAVAILABLE

A target resource shortage MUST NOT be reported as a type error merely because execution cannot proceed.

---

120. Resource Exhaustion

Resource exhaustion is not semantic invalidity.

For example, a program may be semantically valid but fail because:

memory unavailable
compute unavailable
network unavailable
accelerator unavailable
QPU unavailable
storage unavailable
execution budget exhausted

Such outcomes MUST remain distinguishable from malformed programs.

---

121. Security

AI MUST NOT bypass security because it is an intelligent computation.

The following remain enforced:

capability checks
authorization
sandboxing
policy
effects
resource restrictions
provenance
audit

AI-generated actions are subject to exactly the same security boundary as ordinary actions.

---

122. Trust Boundaries

The following are trust boundaries:

source → compiler
compiler → generated artifact
AI model → external data
AI agent → external action
AI → foreign function
AI → network
AI → hardware
AI → code generation
AI → reflection

Crossing a trust boundary requires the applicable contract.

---

123. Generated Code

AI-generated code is not automatically trusted.

Generated code MUST pass through the normal:

parse
validate
type-check
effect-check
capability-check
resource-check
policy-check
provenance
IR

pipeline.

---

124. Generated Models

A generated model MUST have semantic identity and provenance where required.

A generated model MUST NOT silently inherit privileges unavailable to its generator.

---

125. Data Provenance

AI results that depend on external data SHOULD preserve:

data identity
data version
source
transformation
timestamp where relevant
selection/filtering
model identity

The universal provenance system owns storage and representation.

---

126. Privacy and Sensitive Data

AI computation MUST respect security and policy controls for sensitive data.

The AI specification does not grant access merely because a model can consume a value.

Access is governed by:

type
capability
effect
policy
authorization

---

127. Deterministic Compilation

AI grammar and semantics MUST be deterministic with respect to equivalent source and compilation inputs.

Parser behavior MUST NOT depend on:

- current hardware;
- available GPU;
- available QPU;
- runtime memory;
- network availability.

Target discovery occurs downstream.

---

128. Parser Safety

AI grammar files MUST contain:

- no embedded Rust;
- no "unsafe";
- no filesystem operations;
- no network operations;
- no hardware access;
- no runtime execution;
- no parser-time model execution;
- no resource discovery;
- no target discovery.

The implementation baseline is:

Rust >= 1.97
Rust 2021
safe Rust only

---

129. Grammar Scalability

AI grammar MUST use:

- recursive structures;
- repetition;
- identifiers;
- generic constructs;
- symbolic expressions;
- open-ended operation identifiers;
- metadata;
- dialect extension points.

It MUST NOT use finite enumeration to represent concepts whose universe is intentionally open-ended.

---

130. AI Grammar Anti-Patterns

The following are prohibited:

HARDWARE_GPU_0
HARDWARE_GPU_1
MAX_MODELS
MAX_LAYERS
MAX_TENSOR_RANK
MAX_AGENTS
MAX_DATASET_SIZE

and equivalent source-level constructs.

Also prohibited:

one grammar rule for every vendor
one keyword for every model architecture
one keyword for every application
one IR for every framework
one actor model for AI
one resource model for AI
one effect model for AI

---

131. Feature Independence Contract

Every AI grammar file SHOULD be independently completable.

Each file MUST document:

Purpose
Owns
Does Not Own
Dependencies
Lexer Dependencies
Imported Grammar
Public Rules
AST Contract
Semantic Contract
Type Contract
Effect Contract
Capability Contract
Resource Contract
Contract Integration
Policy Integration
Provenance Integration
IR Contract
Backend Boundary
Diagnostics
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility
Completion Criteria

This prevents downstream files from unexpectedly requiring semantic redefinition.

---

132. AI Grammar File Integration Matrix

The expected AI grammar decomposition is:

grammar/ai/
├── ai.g4
├── models.g4
├── tensors.g4
├── datasets.g4
├── training.g4
├── inference.g4
├── evaluation.g4
├── differentiation.g4
├── reasoning.g4
├── deduction.g4
├── induction.g4
├── abduction.g4
├── knowledge.g4
├── facts.g4
├── assertions.g4
├── retraction.g4
├── queries.g4
├── learning.g4
├── adaptation.g4
├── feedback.g4
├── uncertainty.g4
├── probability.g4
├── distributions.g4
├── confidence.g4
├── causality.g4
├── evidence.g4
├── explanations.g4
├── decisions.g4
├── provenance.g4
├── agents.g4
├── multi-agent.g4
├── planning.g4
├── neural-symbolic.g4
├── pipelines.g4
├── policies.g4
├── capabilities.g4
├── resources.g4
└── README.md

This list is a recommended ownership decomposition.

It is not a requirement that every concept have a separate parser grammar if the existing repository architecture already gives that concept a better owner.

---

133. Universal-Construct Reuse Rule

If a construct already exists universally, AI MUST consume it rather than duplicate it.

Examples:

pattern matching → grammar/expressions/
guards → grammar/expressions/
types → grammar/types/
effects → grammar/effects/
resources → grammar/resources/
contracts → grammar/validation/
policies → grammar/policies/
provenance → universal provenance owner
actors → grammar/concurrency/
queries → grammar/data/ where appropriate
sandbox → grammar/security/
simulation → grammar/execution/
reflection → grammar/metaprogramming/
FFI → grammar/interoperability/

---

134. Knowledge Ownership Rule

If knowledge syntax is already represented by a universal grammar, "grammar/ai/knowledge.g4" MUST act as an AI-domain adapter rather than duplicating the universal knowledge grammar.

The semantic distinction is:

universal knowledge syntax
        ↓
AI knowledge composition

not:

universal knowledge syntax
        +
separate AI knowledge syntax

---

135. Query Ownership Rule

AI query constructs MUST reuse the appropriate universal query/data grammar.

The AI layer may define what a query means in an AI context.

It must not create a second SQL-like query language.

---

136. Agent Ownership Rule

AI agent semantics belong under AI.

Actor lifecycle belongs under concurrency.

Therefore:

AI agent semantics
        ↓
existing actor/concurrency model

No competing actor runtime may be created solely for AI.

---

137. Policy Ownership Rule

AI policy consumption belongs under AI.

Policy definition and enforcement belong to the universal policy/security architecture.

AI MUST NOT define a private authorization system.

---

138. Provenance Ownership Rule

AI provenance consumption belongs here.

The universal provenance representation remains the repository-wide authority.

AI MUST NOT create incompatible provenance identifiers.

---

139. Compatibility

AI semantic evolution MUST preserve compatibility where possible.

Changes MUST distinguish:

additive feature
clarification
behavioral correction
breaking semantic change
deprecated feature
removed feature
experimental feature

A new AI library or model architecture MUST NOT require a language-version change merely because the library was added.

---

140. Version Independence

The following version identities are distinct:

language version
grammar version
AST version
semantic model version
IR version
AI semantic version
model version
dataset version
dialect version
compiler version
runtime version
target capability version

They MUST NOT be conflated.

---

141. Migration

If AI semantics change incompatibly, migration information MUST identify:

old construct
new construct
semantic difference
automatic migration possibility
manual migration requirement
compatibility mode
deprecation timeline

Migration MUST preserve meaning where possible.

---

142. Testing Contract

Every stable AI construct MUST have:

Positive tests

Valid syntax and semantics.

Negative tests

Invalid syntax/semantics.

Boundary tests

Interactions with:

- classical;
- quantum;
- HDL;
- distributed;
- networking;
- data;
- concurrency;
- security;
- metaprogramming.

Scalability tests

Tests demonstrating absence of artificial semantic ceilings.

Compatibility tests

Tests across supported language/grammar versions.

Determinism tests

Where deterministic behavior is required.

Reproducibility tests

Where reproducibility is part of the declared contract.

Resource tests

Valid resource negotiation and resource exhaustion behavior.

Capability tests

Capability present/absent behavior.

Policy tests

Allowed/denied behavior.

Provenance tests

Required provenance preservation.

---

143. Required AI Conformance Examples

The repository SHOULD maintain at least:

minimal-ai.zm
model.zm
tensor.zm
symbolic-shape.zm
training.zm
inference.zm
reasoning.zm
knowledge.zm
learning.zm
adaptation.zm
uncertainty.zm
evidence.zm
provenance.zm
decision.zm
agent.zm
multi-agent.zm
neural-symbolic.zm
classical-ai.zm
quantum-ai.zm
hybrid-ai.zm
hdl-ai.zm
distributed-ai.zm
sandbox-ai.zm
simulation-ai.zm
ffi-ai.zm
poco-reaf-ai.zm

The names are test artifacts, not language keywords.

---

144. Mandatory Cross-Domain Test

At least one conformance program MUST combine:

classical computation
+
tensor computation
+
model computation
+
reasoning
+
knowledge
+
learning
+
controlled adaptation
+
uncertainty
+
evidence
+
provenance
+
contracts
+
policies
+
resource requirements
+
capability requirements
+
parallel execution
+
agent computation
+
quantum computation
+
measurement
+
hybrid control
+
simulation
+
hardware intent

The expected pipeline is:

source
 ↓
lexer
 ↓
parser
 ↓
domain-neutral AST
 ↓
structural validation
 ↓
type analysis
 ↓
effect analysis
 ↓
capability analysis
 ↓
resource analysis
 ↓
contract analysis
 ↓
policy analysis
 ↓
provenance
 ↓
canonical semantic model
 ↓
Classical IR / quantum::ir / applicable domain IR
 ↓
optimization
 ↓
lowering
 ↓
routing
 ↓
scheduling
 ↓
resilience
 ↓
ZQN
 ↓
HAL
 ↓
target

---

145. POCO-REAF Portability Test

The same source semantics SHOULD be validated against multiple realization classes:

tiny/embedded
CPU
multicore
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC
cluster
distributed system
cloud
future target

The test MUST verify that target changes affect realization rather than source meaning.

---

146. Resource-Scaling Test

A test suite MUST demonstrate that a program with symbolic requirements can be realized differently according to resources.

For example:

requires capability("tensor.compute");
requires memory >= required_memory;

The source must not contain:

24GB
8 threads
1 GPU
32-bit register
64GB RAM

as universal assumptions.

Those values may be valid explicit application constraints, but they MUST NOT be hidden language-level limits.

---

147. Symbolic-Scale Test

AI semantics SHOULD support symbolic quantities such as:

batch
sequence
features
hidden
layers
workers
replicas
nodes

without requiring physical machine quantities to be known at source-writing time.

---

148. Compile-Once Semantics

The phrase "Compile Once" MUST be interpreted as semantic compilation independence, not a requirement that one binary magically execute on physically incompatible targets.

A compiler artifact MAY require:

- compatible target capabilities;
- compatible runtime;
- compatible ABI;
- compatible execution substrate.

If multiple targets require different machine code, the toolchain MAY use:

- multi-target artifacts;
- portable IR;
- runtime specialization;
- cached lowering;
- target-specific code generation;
- fat artifacts.

The source semantic program remains one program.

---

149. Anywhere Semantics

"Anywhere" means that the program's semantic intent is not tied to a particular geographic, vendor, or hardware deployment.

Execution may occur on:

local machine
embedded system
server
accelerator
cluster
cloud
distributed system
simulator
quantum platform
future target

subject to capability and policy.

---

150. Forever Semantics

"Forever" means semantic longevity.

A program SHOULD remain understandable and migratable across:

- compiler generations;
- hardware generations;
- runtime generations;
- target architectures;
- implementation strategies.

This requires preservation of:

language version
semantic version
AST/IR compatibility metadata
provenance
dialect identity
resource/capability requirements

---

151. No Silent Semantic Downgrade

A compiler MUST NOT silently convert:

exact → approximate
deterministic → nondeterministic
authorized → unauthorized
required capability → absent capability
required precision → lower precision
required resource → insufficient resource
quantum computation → classical approximation

unless the source explicitly permits the transformation through:

- fallback;
- approximation;
- policy;
- preference;
- constraint;
- simulation;
- alternate execution mode.

---

152. Simulation Versus Real Execution

Simulation MUST remain semantically distinguishable from physical execution where the distinction is observable.

A simulation MAY be a valid realization of an abstract computation.

It MUST NOT claim physical properties that it cannot provide.

---

153. AI and Quantum Measurement

Quantum measurement results MAY become:

AI inputs
training data
evidence
knowledge
decision inputs

The measurement remains a quantum operation.

The AI layer consumes the resulting semantic value.

No AI grammar rule may redefine measurement semantics.

---

154. AI and Hardware Telemetry

Hardware telemetry MAY become:

observation
evidence
resource information
adaptation input

Hardware telemetry does not become a universal AI primitive merely because AI can consume it.

---

155. AI and Resource Adaptation

An AI system MAY select among permitted resource realizations.

For example:

CPU
GPU
accelerator
distributed

may be candidates.

The selection MUST remain constrained by:

capabilities
resources
policies
contracts
security
effects

---

156. AI and Future Domains

Future computational domains MUST be able to consume AI semantic values without modifying this specification merely because the domain exists.

The extension mechanism is:

generic type
generic operation
capability
effect
resource
policy
metadata
dialect
semantic adapter

not universal keyword growth.

---

157. Rust Implementation Contract

All implementations supporting this specification MUST support:

Rust 1.97 or later
Rust 2021
safe Rust

"unsafe" Rust MUST NOT be required by the AI grammar or semantic implementation.

The specification does not require a particular Rust crate architecture.

---

158. Integer and Cardinality Safety

Semantic quantities MUST NOT silently overflow because an implementation uses:

usize
u64
u32

internally.

If an internal representation cannot represent a semantic quantity, the implementation MUST fail explicitly rather than silently changing its meaning.

---

159. Memory Safety

The implementation MUST preserve Rust's safety guarantees.

No AI feature may justify:

unsafe

as a semantic requirement.

Performance optimization MUST use safe alternatives unless the project explicitly changes its global safety policy in a higher-level architectural specification.

---

160. Parser Resource Safety

Parser resource exhaustion MAY occur.

However:

parser resource exhaustion

MUST be distinguishable from:

invalid AI program

Implementations MAY provide configurable operational limits.

Such limits are not part of the source-language semantics.

---

161. Compiler Resource Safety

The compiler MAY reject compilation because its operational budget is exhausted.

The diagnostic SHOULD identify the implementation/resource condition rather than falsely declaring the source semantically invalid.

---

162. Runtime Resource Safety

Runtime MAY report:

RESOURCE_EXHAUSTED
CAPABILITY_UNAVAILABLE
POLICY_DENIED
EXECUTION_UNAVAILABLE

without invalidating the source program.

---

163. Diagnostics

AI diagnostics SHOULD identify:

- source span;
- construct;
- semantic problem;
- expected condition;
- actual condition;
- affected type/effect/capability/resource;
- provenance where useful;
- suggested correction where possible.

Diagnostics MUST NOT expose secrets or unauthorized data.

---

164. Source Spans

AI constructs MUST preserve source spans into the canonical AST.

Where transformations create derived semantic objects, provenance SHOULD preserve the relationship to the original source.

---

165. Canonical Identity

Semantic identity MUST NOT depend on:

memory address
pointer address
process ID
thread ID
device ID
source formatting
hash-map iteration order

Where hashes are used internally, canonical serialization/order MUST be deterministic when semantic identity requires it.

---

166. Map and Collection Determinism

AI semantics MUST NOT accidentally depend on unspecified iteration order.

If order is semantically meaningful, the appropriate ordered data type MUST be used.

If order is not meaningful, semantic equality MUST not depend on incidental iteration order.

---

167. Randomness

Randomness is an explicit semantic effect where applicable.

Learning, sampling, probabilistic reasoning, and stochastic optimization MAY require:

effect(randomness)

The random source MUST be controlled according to the declared reproducibility contract.

---

168. External Services

AI MAY use external services through networking/interoperability.

Such calls MUST participate in:

effects
capabilities
policies
resources
provenance
security

The language MUST NOT assume that an external service is always available.

---

169. Model Serving

Model serving is an execution/runtime concern.

The AI specification defines model semantics but does not mandate:

- HTTP;
- RPC;
- a particular server;
- a particular serialization format;
- a particular deployment platform.

---

170. Deployment

Deployment MAY specialize AI programs based on:

- capabilities;
- resources;
- policy;
- topology;
- reliability;
- latency;
- security.

Deployment semantics remain separate from source-level AI meaning.

---

171. Observability

AI execution MAY expose:

- metrics;
- traces;
- logs;
- decisions;
- explanations;
- provenance;
- resource information.

Observability MUST obey policy and privacy constraints.

---

172. Auditability

Where an AI computation is declared auditable, the implementation MUST preserve sufficient provenance to reconstruct the required semantic chain.

Auditability does not require storing every internal implementation detail unless the contract requires it.

---

173. Explainability Limits

The language MUST NOT promise explanations that an implementation cannot semantically justify.

An explanation MAY be:

- symbolic;
- rule-based;
- provenance-based;
- model-derived;
- approximation-based.

Its declared interpretation must be clear.

---

174. Evidence Quality

Evidence SHOULD carry enough metadata to distinguish:

observed
derived
predicted
assumed
simulated
verified
unverified

Evidence classification is semantic metadata and may be extended without changing the core grammar.

---

175. Causal Semantics

Causal computation MAY represent:

cause
effect
observation
intervention
counterfactual
dependency

Causal semantics MUST NOT be inferred merely from correlation unless the selected semantic model explicitly defines that relationship.

---

176. Probabilistic Semantics

Probabilistic computation MUST distinguish:

probability
uncertainty
confidence
belief
observation

These concepts MUST NOT be treated as interchangeable merely because they are all represented numerically.

---

177. Knowledge Consistency

Where the selected knowledge model requires consistency, contradictory assertions MUST be handled according to the declared logic.

The language MUST NOT silently choose one contradictory fact merely because it was encountered later.

Possible models MAY include:

- classical consistency;
- paraconsistent reasoning;
- versioned knowledge;
- temporal knowledge;
- probabilistic knowledge.

The chosen model must be explicit.

---

178. Temporal Knowledge

Knowledge MAY be time-dependent.

Temporal semantics MAY represent:

valid_from
valid_until
observed_at
created_at
derived_at

Time semantics remain compatible with the universal temporal/type/execution systems.

---

179. Knowledge Provenance

Every externally derived knowledge item SHOULD be capable of retaining provenance where required.

Example conceptual chain:

source observation
      ↓
data transformation
      ↓
inference
      ↓
knowledge assertion
      ↓
decision

The chain MUST remain representable without relying on physical memory addresses.

---

180. Model Provenance

Model results SHOULD be traceable to:

model identity
model version
input identity
input version
execution profile
relevant parameters
relevant policy

when auditability is required.

---

181. Learning Provenance

Training SHOULD be traceable to:

model
dataset
dataset version
objective
algorithm
configuration
randomness profile
compiler/semantic version
execution profile

as required by the declared provenance policy.

---

182. Adaptation Provenance

Every controlled adaptation SHOULD identify:

previous state
observation
reason
candidate change
policy
authorization
result
new state

where the adaptation is auditable.

---

183. Agent Provenance

Agent actions MAY record:

observation
knowledge
policy
decision
action
result

according to the applicable audit policy.

---

184. No Application-Specific Core Semantics

AI MUST remain computationally universal.

The core language SHOULD NOT grow specialized semantic primitives merely for one industry.

Industry functionality belongs in:

libraries
dialects
services
capability providers
policies
applications

This preserves the universal architecture.

---

185. Dialect Contract

An AI dialect MAY extend:

- syntax;
- operations;
- metadata;
- capabilities;
- resource descriptions.

A dialect MUST declare:

name
version
dependencies
capabilities
effects
types
operations
IR/lowering contract
compatibility
security implications

A dialect MUST NOT silently alter core AI semantics.

---

186. Library Contract

Libraries MAY define:

- models;
- algorithms;
- tensor operations;
- datasets;
- reasoning rules;
- agents;
- domain-specific abstractions.

Libraries MUST NOT require new core keywords for ordinary extension.

---

187. Vendor Independence

AI semantics MUST NOT depend on a particular vendor.

Vendor support belongs in:

dialects
backends
HAL
capability providers
runtime adapters

A vendor-specific feature MUST have a semantic capability boundary.

---

188. Accelerator Independence

Accelerator choice belongs downstream.

The source MAY say:

requires capability("accelerator.compute");

but MUST NOT require:

accelerator 3

unless the program intentionally uses a target-specific dialect.

---

189. Distributed Independence

Distributed AI source MUST describe semantic communication/dependency rather than physical node identity.

The implementation determines:

node placement
replication
partitioning
routing
scheduling

---

190. Quantum Resource Independence

AI/quantum programs MAY express quantum resource requirements.

They MUST NOT encode a universal maximum qubit count.

The compiler/runtime determines whether a target satisfies the requirement.

---

191. Hardware Resource Independence

AI/hardware programs MAY express:

requires memory >= required_memory
requires capability("tensor.compute")
requires topology(required_topology)

without fixing the physical size of the machine.

---

192. Error Recovery

AI execution MAY participate in:

retry
fallback
recover
degrade
escalate
reject

Recovery behavior is controlled by execution and policy semantics.

AI MUST NOT silently retry operations whose effects are not safely repeatable.

---

193. Idempotency

Where AI operations have external effects, retry semantics MUST account for idempotency.

A retry of:

pure inference

may have different semantics from:

external action

The effect system and policy determine the distinction.

---

194. Pure AI Computation

An AI operation MAY be pure if:

- it has no external effects;
- it does not depend on mutable external state;
- its randomness is absent or semantically controlled;
- its result depends only on declared inputs.

Pure operations are easier to optimize and reproduce.

---

195. Stateful AI Computation

Stateful AI operations MUST identify relevant state semantics.

State MUST obey:

- ownership;
- concurrency;
- persistence;
- effect;
- policy;
- provenance

requirements.

---

196. Concurrent AI Computation

Concurrent AI operations MUST use the universal concurrency model.

AI MUST NOT create a private scheduling model.

Race safety and synchronization remain governed by the universal concurrency/type/memory contracts.

---

197. Distributed AI Consistency

Distributed learning or knowledge MAY use different consistency models.

The selected model MUST be semantically explicit when observable.

The AI specification does not impose one global consistency model.

---

198. Streaming AI

Streaming data MAY feed AI computation.

Streaming semantics MUST integrate with:

- data;
- concurrency;
- networking;
- effects;
- resources.

The language MUST NOT assume that a dataset is always finite and resident in memory.

---

199. Online Learning

Online learning MAY update a model while data is streaming.

Such updates are adaptation/learning effects and MUST follow their respective policies.

---

200. Batch Learning

Batch learning is a specialization of the general learning model.

The language MUST NOT make batch execution the only model.

---

201. Incremental Learning

Incremental learning MAY update a model from subsets or streams of data.

The semantic model MUST preserve the declared update semantics.

---

202. Federated or Distributed Learning

Distributed learning MAY use:

- actors;
- tasks;
- messages;
- channels;
- network services;
- distributed resources.

The AI layer expresses learning semantics.

The distributed layer handles placement and communication.

---

203. Security of Learning

Learning MAY expose sensitive information.

Policies MUST be able to restrict:

- dataset access;
- model access;
- output access;
- parameter export;
- external communication;
- adaptation.

---

204. Security of Inference

Inference access MAY require capability and authorization.

Inference output MAY itself be security-sensitive.

The AI layer does not grant unrestricted model access.

---

205. Security of Agents

Agent actions are effects.

An agent MUST NOT obtain an action capability merely because a model predicted that action.

The action path is:

prediction
    ↓
decision
    ↓
policy
    ↓
capability
    ↓
authorization
    ↓
effect
    ↓
action

---

206. Security of Adaptation

Adaptation is privileged.

It SHOULD require explicit:

adaptation capability
policy
authorization
provenance

and applicable resource/effect checks.

---

207. Security of Code Generation

Generated executable code is privileged.

Code generation MUST NOT automatically imply:

native.execute

or equivalent authority.

---

208. Resource Fairness

AI resource scheduling MAY consider:

- priorities;
- quotas;
- fairness;
- deadlines;
- energy;
- reliability;
- policy.

These are execution/resource concerns rather than parser semantics.

---

209. Cost Models

A compiler/runtime MAY estimate AI execution cost.

Cost estimation MUST NOT change semantic meaning unless explicitly included as a constraint.

---

210. Performance Portability

AI source SHOULD express semantic requirements rather than implementation details.

Performance-sensitive source MAY express portable constraints/preferences.

Examples:

requires capability("tensor.compute");
prefer capability("accelerated.tensor.compute");
constrain latency < budget;

This permits specialization without source rewriting.

---

211. Memory Portability

AI programs MUST distinguish:

logical data size
physical memory capacity

A tensor's semantic shape does not guarantee that a particular machine can materialize it.

Failure to materialize is a resource/execution condition.

---

212. Lazy and Materialized Values

AI implementations MAY use:

- eager values;
- lazy values;
- views;
- streams;
- distributed values;
- symbolic values.

The semantic result must remain equivalent where the selected execution model requires equivalence.

---

213. Symbolic Computation

AI computations MAY remain symbolic until later compilation.

Symbolic expressions MUST be preserved through semantic analysis when required.

They MUST NOT be prematurely evaluated merely because the compiler happens to know one target's current resources.

---

214. Dynamic Shapes

Dynamic shapes MAY be used where supported by the type system.

The runtime MUST validate runtime shape requirements.

A dynamic shape MUST NOT imply an artificial maximum.

---

215. Dependent AI Types

Where the type system supports dependent/parametric information, AI MAY express relationships such as:

input shape
→ output shape
→ model constraint

Dependent semantics remain owned by the universal type system.

AI does not create a separate dependent type language.

---

216. Linear and Affine AI Resources

Where tensors, models, handles, or execution resources require linear/affine ownership, the universal type system governs the semantics.

AI MAY consume those types.

AI MUST NOT redefine ownership rules.

---

217. Model Handles

A runtime model handle is a value with implementation-specific storage.

It MUST NOT be confused with model semantic identity.

Model handles may be:

- local;
- remote;
- distributed;
- accelerator-backed.

The source semantics remain abstract.

---

218. Remote Models

A remote model MAY be represented as an external computation.

Its invocation carries the applicable:

network effect
capability
policy
resource
provenance

---

219. Model Availability

Model availability is an execution condition.

A semantically valid model invocation MAY fail because the required model provider is unavailable.

This is not automatically a source type error.

---

220. Model Fallback

A program MAY declare an alternate model or realization.

Fallback selection MUST obey the declared contract.

A fallback with different semantic behavior MUST NOT be substituted silently.

---

221. AI and Simulation-Based Testing

AI semantics SHOULD be testable using simulation.

Simulation MUST be able to test:

- model behavior;
- tensor behavior;
- quantum interaction;
- distributed behavior;
- fault behavior;
- resource behavior.

Simulation results MUST remain distinguishable from physical execution where required.

---

222. Formal Verification

AI properties MAY participate in formal verification.

Examples:

shape invariants
range constraints
probability constraints
model interface contracts
safety properties
resource contracts

Formal verification belongs to the validation/proof architecture.

---

223. Property-Based Testing

AI constructs SHOULD support property-based testing through the universal test/property system.

The AI grammar does not need a separate property language.

---

224. Contract-Driven AI

AI programs SHOULD be capable of declaring:

requires
ensures
invariant
assume
guarantee
property

for:

- inputs;
- outputs;
- shapes;
- probabilities;
- confidence;
- model state;
- resource requirements.

---

225. AI Decision Correctness

A decision contract MAY distinguish:

mathematical correctness
model correctness
data correctness
policy correctness
action authorization

A prediction being numerically valid does not automatically authorize an external action.

---

226. AI Explainability Correctness

An explanation SHOULD identify what class of explanation it provides.

For example:

logical
causal
feature-attribution
provenance
rule-based
approximate

An approximate explanation MUST NOT be represented as an exact proof.

---

227. Evidence and Proof

Evidence and proof are distinct.

Evidence supports a claim.

A proof establishes a claim under a formal system.

AI MUST NOT silently promote statistical evidence to formal proof.

---

228. Model Confidence and Contract Semantics

A confidence threshold MAY be a contract or policy.

For example, conceptually:

requires confidence >= threshold;

The threshold's interpretation must be defined by the selected confidence model.

---

229. Probability Bounds

Probability values MUST respect the semantics of the selected probability type.

Invalid probability values MUST be rejected or represented as explicit errors according to the type's contract.

---

230. Uncertainty Propagation

Where mathematically defined, transformations SHOULD preserve or propagate uncertainty.

The exact propagation method is implementation/model-specific unless specified by the source contract.

---

231. AI and Optimization

AI computation may be an optimization objective.

Optimization itself remains a semantic operation.

The compiler MAY choose a different algorithm if the semantic contract permits it.

---

232. AI and Scheduling

AI computation MAY express scheduling preferences.

Scheduling remains downstream.

No AI grammar rule may assume a fixed number of workers.

---

233. AI and Placement

AI computation MAY express placement preferences.

Physical placement is downstream.

Source code MUST NOT require physical addresses.

---

234. AI and Power

AI programs MAY declare energy constraints or preferences where the universal resource system supports them.

Energy semantics MUST remain portable.

---

235. AI and Reliability

AI execution MAY require reliability properties.

Reliability is a resource/policy/execution property.

AI does not define physical fault models independently.

---

236. AI and Resilience

AI MAY participate in:

retry
checkpoint
restore
fallback
degrade
recover

using the existing execution/resilience infrastructure.

---

237. Checkpoint Semantics

A checkpoint MAY contain:

- model state;
- optimizer state;
- execution state;
- provenance;
- data references;
- semantic version.

Checkpoint format is an interoperability/runtime concern.

---

238. State Recovery

Recovery MUST preserve the declared semantic state.

Restoring an incompatible model state MUST be rejected or explicitly migrated.

---

239. Compatibility with Older Targets

A program MAY be valid for a target that lacks a preferred AI capability if an explicitly permitted fallback exists.

Without an allowed fallback, the compiler/runtime MUST report capability incompatibility.

---

240. Future-Proofing

Future AI functionality SHOULD be introduced through:

generic constructs
metadata
capabilities
effects
resources
dialects
libraries
semantic adapters

rather than expanding the core keyword set.

---

241. Repository Completion Contract

"grammar/spec/ai.md" is complete when:

- it defines AI semantics;
- it does not duplicate parser ownership;
- it does not duplicate type semantics;
- it does not duplicate effect semantics;
- it does not duplicate resource semantics;
- it does not duplicate policy semantics;
- it does not duplicate provenance semantics;
- it does not duplicate concurrency semantics;
- it does not duplicate quantum semantics;
- it does not duplicate HDL semantics;
- it does not create a second AI IR;
- it defines AI integration boundaries;
- it defines AI portability;
- it defines AI scalability;
- it defines AI security;
- it defines AI provenance;
- it defines AI reproducibility;
- it defines AI adaptation controls;
- it defines AI agent integration;
- it defines neural-symbolic composition;
- it defines quantum/classical integration;
- it defines distributed integration;
- it defines hardware integration;
- it defines dialect extension;
- it defines testing obligations;
- it defines safe-Rust requirements.

---

242. Integration Checklist for "grammar/ai/*.g4"

Each AI grammar file is complete only when it has:

[ ] Purpose
[ ] Owns
[ ] Does Not Own
[ ] Dependencies
[ ] Lexer Dependencies
[ ] Imported Grammar
[ ] Public Rules
[ ] AST Contract
[ ] Semantic Contract
[ ] Type Contract
[ ] Effect Contract
[ ] Capability Contract
[ ] Resource Contract
[ ] Contract Integration
[ ] Policy Integration
[ ] Provenance Integration
[ ] IR Contract
[ ] Backend Boundary
[ ] Diagnostics
[ ] Positive Tests
[ ] Negative Tests
[ ] Boundary Tests
[ ] Scalability Tests
[ ] Compatibility Tests
[ ] Hard-Coding Audit
[ ] Completion Criteria

This checklist is an integration contract, not parser syntax.

---

243. AI File Dependency Principle

AI grammar files MUST depend only on stable upstream contracts.

The intended dependency direction is:

lexer
  ↓
core
  ↓
types / expressions / statements
  ↓
resources / effects / validation / policies / provenance
  ↓
AI syntax
  ↓
AI semantic model
  ↓
canonical IR

A lower-level universal subsystem MUST NOT depend on an AI grammar merely to implement a universal feature.

---

244. No Circular Semantic Ownership

The following pattern is prohibited:

AI depends on resources
resources depend on AI

Instead:

universal resources
        ↓
AI consumes resources

Likewise:

universal effects
        ↓
AI consumes effects

and:

universal policies
        ↓
AI consumes policies

---

245. Integration with "grammar/ai/ai.g4"

"ai.g4" is the AI composition point.

It MUST compose the AI-domain grammar modules without redefining their internal semantics.

It SHOULD integrate concepts such as:

model
dataset
tensor
training
inference
evaluation
differentiation
reasoning
knowledge
learning
adaptation
uncertainty
evidence
agents
pipelines

through their dedicated grammar owners.

---

246. Integration with "grammar/ai/models.g4"

"models.g4" owns model source syntax.

It MUST defer:

type meaning → type system
resource meaning → resources
effect meaning → effects
capability meaning → capabilities
contracts → validation
policies → policies
provenance → provenance
IR → canonical IR

---

247. Integration with "grammar/ai/tensors.g4"

"tensors.g4" owns tensor source syntax.

It MUST defer:

element typing → type system
shape semantics → type system
storage → runtime/lowering
layout optimization → optimizer
device placement → backend
memory capacity → resources

---

248. Integration with "grammar/ai/reasoning.g4"

"reasoning.g4" owns reasoning source composition.

It MUST integrate with:

knowledge
evidence
uncertainty
contracts
provenance
policies

without redefining them.

---

249. Integration with "grammar/ai/learning.g4"

"learning.g4" owns learning syntax.

It MUST integrate with:

models
datasets
tensors
effects
resources
capabilities
policies
provenance
adaptation

---

250. Integration with "grammar/ai/adaptation.g4"

"adaptation.g4" owns adaptation syntax.

It MUST NOT authorize adaptation.

Authorization remains downstream.

---

251. Integration with "grammar/ai/agents.g4"

"agents.g4" owns AI-agent semantics.

Actor lifecycle remains owned by "grammar/concurrency/".

---

252. Integration with "grammar/quantum/"

AI/quantum integration MUST be through semantic composition.

No quantum operation enumeration belongs in the AI specification.

Quantum operations remain data-driven and extensible.

---

253. Integration with "grammar/hybrid/"

Hybrid AI computation SHOULD use:

classical
quantum
AI
hardware

as semantic domains that compose rather than separate languages.

---

254. Integration with "grammar/data/"

Datasets, data transformations, queries, schemas, and streams MUST reuse the data subsystem.

AI MUST NOT create a second data model.

---

255. Integration with "grammar/distributed/"

Distributed AI uses the universal distributed model.

AI does not define physical node topology.

---

256. Integration with "grammar/networking/"

Network-backed AI uses the universal network/effect/capability model.

---

257. Integration with "grammar/security/"

AI operations requiring privileged behavior MUST cross the security boundary.

---

258. Integration with "grammar/metaprogramming/"

AI-generated source/code MUST use the existing metaprogramming and code-generation contracts.

---

259. Integration with "grammar/interoperability/"

External model/data formats MUST use interoperability adapters.

---

260. Integration with "grammar/dialects/"

Framework/vendor/domain-specific AI functionality belongs in dialects when it cannot be expressed as ordinary library functionality.

---

261. Integration with "grammar/compile/"

AI compilation intent MAY specify:

specialization
optimization
target capability
resource preference
fallback
reproducibility

Compilation realization remains owned by the compilation system.

---

262. Integration with "grammar/execution/"

AI execution MAY use:

simulation
adaptive execution
recovery
retry
fallback
checkpointing

Execution semantics remain universal.

---

263. Integration with "grammar/validation/"

AI contracts, invariants, evidence, and properties MUST use the universal validation system.

---

264. Integration with "grammar/resources/"

AI requirements MUST be represented using the universal:

requirement
capability
constraint
budget
preference
hint
negotiation

model.

---

265. Integration with "grammar/effects/"

AI operations MUST declare or inherit applicable effects.

---

266. Integration with "grammar/policies/"

AI policy constraints MUST be represented using the universal policy model.

---

267. Integration with Provenance

AI provenance MUST remain compatible with repository-wide provenance.

A new AI-specific provenance namespace MUST NOT fragment the universal provenance graph.

---

268. Integration with Compatibility

AI version changes MUST participate in the universal compatibility model.

---

269. Integration with "grammar/grammar.md"

"grammar/grammar.md" MUST record implementation status independently of this semantic specification.

This file does not claim that every described feature is already implemented.

The conformance states remain:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

with implementation dimensions where supported:

AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL

---

270. Integration with Historical Documentation

Historical or aspirational AI concepts may remain in:

grammar/Zamani-Grammar.md

but they become normative only after promotion through the repository authority model.

---

271. Hard-Coding Audit

This specification MUST contain no universal machine-capacity assumptions.

It MUST NOT define:

MAX_MODELS
MAX_TENSORS
MAX_LAYERS
MAX_PARAMETERS
MAX_FEATURES
MAX_INPUTS
MAX_OUTPUTS
MAX_BATCH_SIZE
MAX_SEQUENCE_LENGTH
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_AGENTS
MAX_DATASET_SIZE
MAX_WORKERS
MAX_DEVICES
MAX_ACCELERATORS
MAX_NODES
MAX_EPOCHS
MAX_TRAINING_STEPS
MAX_MEMORY
MAX_NETWORK_SIZE

or equivalent semantic limits.

Any future proposal introducing such a limit MUST be rejected unless it is proven to be a genuine semantic property rather than a machine-capacity assumption.

---

272. Target Independence Audit

This specification MUST NOT require:

CPU ID
GPU ID
FPGA ID
ASIC ID
QPU ID
device index
node index
memory address
physical qubit
physical interconnect

for portable AI semantics.

---

273. Vendor Independence Audit

This specification MUST NOT require:

vendor-specific instruction
vendor-specific device
vendor-specific runtime
vendor-specific API
vendor-specific memory layout

for core AI semantics.

Vendor integration belongs downstream.

---

274. Framework Independence Audit

Core AI semantics MUST NOT depend on one framework.

The language MUST remain capable of representing future frameworks without changing the core semantic model.

---

275. Application Independence Audit

Core AI semantics MUST NOT require application-specific keywords.

Application-specific functionality belongs outside the universal grammar.

---

276. Safety Audit

The implementation requirements are:

Rust >= 1.97
Rust 2021
safe Rust only
no unsafe

No AI feature may require unsafe memory access as part of its language semantics.

---

277. Semantic Completeness Audit

For every AI feature, the implementation team MUST be able to answer:

What does it mean?
What type does it produce?
What types does it consume?
What effects does it have?
What capabilities does it require?
What resources does it require?
What contracts constrain it?
What policies constrain it?
What provenance must it preserve?
Where does it enter the semantic model?
Where does it lower?
Which IR represents it?
How is it optimized?
How is it realized?
How is failure represented?
How is it tested?
How does it scale?
How does it remain portable?

If any answer is missing, the feature is incomplete.

---

278. Production-Readiness Gate

AI is production-ready only when every stable AI construct has a complete trace:

SPEC
 ↓
LEXICAL OWNER, if needed
 ↓
GRAMMAR OWNER
 ↓
AST
 ↓
SEMANTICS
 ↓
TYPE
 ↓
EFFECT
 ↓
CAPABILITY
 ↓
RESOURCE
 ↓
CONTRACT
 ↓
POLICY
 ↓
PROVENANCE
 ↓
CANONICAL IR
 ↓
OPTIMIZATION
 ↓
LOWERING
 ↓
ROUTING/SCHEDULING
 ↓
EXECUTION
 ↓
TESTS

No feature may be declared production-ready merely because parsing succeeds.

---

279. Definition of Done

"grammar/spec/ai.md" is considered complete when the repository can use it as the stable normative AI semantic contract without requiring this file to be modified when:

- a new model architecture is added;
- a new tensor operation is added;
- a new ML algorithm is added;
- a new accelerator is added;
- a new GPU is added;
- a new CPU is added;
- a new FPGA is added;
- a new ASIC is added;
- a new QPU is added;
- a new simulator is added;
- a new dataset provider is added;
- a new knowledge backend is added;
- a new reasoning algorithm is added;
- a new agent implementation is added;
- a new distributed topology is added;
- a new vendor dialect is added;
- a new library is added;
- a new application domain is added.

This file SHOULD change only when the semantic contract of AI itself changes.

---

280. Final Architectural Principle

The production architecture is:

                 ZAMANI
                    │
        ┌───────────┴───────────┐
        │                       │
   UNIVERSAL CORE          DOMAIN SYSTEMS
        │                       │
   ┌────┼────┬────┐       ┌────┼────┬────┐
   │    │    │    │       │    │    │    │
 Types Effects Resources Contracts Classical Quantum HDL
   │    │    │    │       │    │    │
   └────┴────┴────┴───────┴────┴────┘
                    │
          ┌─────────┴─────────┐
          │                   │
        AI DOMAIN        OTHER DOMAINS
          │                   │
   ┌──────┼─────────────┐     │
   │      │      │      │     │
 Model Knowledge Learning Agents
   │      │      │      │
 Reasoning Evidence Uncertainty
   │      │      │      │
 Adaptation Decisions Provenance
   └──────┴──────┴──────┘
                    │
             SEMANTIC MODEL
                    │
        ┌───────────┴───────────┐
        │                       │
   Classical IR             quantum::ir
        │                       │
        └───────────┬───────────┘
                    │
          optimization/lowering
                    │
          routing/scheduling
                    │
             resilience
                    │
                   ZQN
                    │
                   HAL
                    │
       ┌────────────┼─────────────┐
       │            │             │
      CPU          GPU          FPGA
       │            │             │
      ASIC      accelerator      QPU
       │            │             │
       └────────────┼─────────────┘
                    │
          simulator / HPC /
       cluster / distributed /
            future targets

The decisive rule is:

«AI expands Zamani's computational semantics; it does not create a second Zamani.»

The AI layer therefore provides a universal semantic vocabulary for models, tensors, learning, reasoning, knowledge, uncertainty, evidence, decisions, agents, adaptation, and intelligent computation, while continuing to use the same types, effects, capabilities, resources, contracts, policies, provenance, concurrency, data, quantum, HDL, distributed, interoperability, compilation, execution, IR, and HAL architecture used by the rest of the language.

That is what makes the AI subsystem compatible with POCO-REAF rather than merely portable across today's machines.