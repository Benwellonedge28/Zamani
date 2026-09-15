Zamani AI / Machine Learning Specification

Path: "grammar/spec/ai.md"
Language: Zamani
Specification status: Normative
Specification layer: AI / ML domain semantics and grammar integration contract
Implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: Rust 2021
Rust safety requirement: Safe Rust only; "unsafe" is prohibited
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

0. Document Contract

0.1 Purpose

This document defines the normative language contract for artificial intelligence, machine learning, differentiable computation, model computation, tensor computation, datasets, training, inference, agents, AI pipelines, AI resource intent, AI capabilities, and AI interoperability in Zamani.

It defines:

- what AI constructs mean at the source-language level;
- which AI syntax is permitted;
- how AI syntax integrates with the universal Zamani language;
- how AI constructs interact with the common type system;
- how tensors and symbolic dimensions are represented;
- how models are declared and composed;
- how datasets are represented;
- how training and inference are expressed;
- how differentiation is expressed;
- how agents and AI pipelines are expressed;
- how AI interacts with classical, quantum, HDL, hardware, distributed, networking, data, and security domains;
- how resource requirements and capabilities are expressed without hard-coding machines;
- how AI constructs lower through the canonical semantic pipeline;
- how AI remains independent of specific frameworks, vendors, devices, accelerators, and runtimes;
- how AI scales from tiny computations to arbitrarily large computations subject only to semantic validity and available resources.

This document is normative.

An AI feature is not part of stable Zamani merely because it appears in:

- "grammar/Zamani-Grammar.md";
- an example;
- a design document;
- an experimental grammar;
- a generated reference;
- a parser prototype;
- an implementation branch.

A feature becomes stable only when it satisfies the promotion and conformance requirements defined here.

---

1. Architectural Position

The authoritative architecture is:

Zamani Source
     │
     ▼
Canonical Lexer
     │
     ▼
Canonical Parser
     │
     ▼
Domain-Neutral Frontend AST
     │
     ▼
Structural Validation
     │
     ▼
Semantic Analysis
     │
     ├── Type Analysis
     ├── Effect Analysis
     ├── Capability Analysis
     ├── Resource Analysis
     ├── Ownership Analysis
     ├── Portability Analysis
     └── AI Domain Validation
     │
     ▼
Canonical Semantic Model
     │
     ▼
Canonical IR
     │
     ├── Classical IR
     ├── quantum::ir
     ├── HDL / Hardware IR
     └── Other domain IR
     │
     ▼
Optimization / Lowering
     │
     ├── Differentiation lowering
     ├── Tensor lowering
     ├── Model specialization
     ├── Parallelization
     ├── Distribution
     └── Accelerator lowering
     │
     ▼
Routing / Scheduling / Placement
     │
     ▼
Resilience / QEC / ZQN where applicable
     │
     ▼
HAL / Target realization
     │
     ▼
Runtime

AI syntax participates in this pipeline.

AI syntax does not replace it.

The grammar is therefore a source-language boundary, not an AI runtime, compiler, optimizer, tensor engine, model server, accelerator API, or AI-specific IR.

---

2. Authority Hierarchy

The following authority hierarchy is mandatory:

grammar/DESIGN.md
        │
        ▼
grammar/spec/ai.md
        │
        ▼
grammar/spec/type-system.md
grammar/spec/semantics.md
grammar/spec/resources.md
grammar/spec/effects.md
grammar/spec/portability.md
        │
        ▼
grammar/ai/*.g4
        │
        ▼
grammar/Zamani.g4
        │
        ▼
Lexer / Parser
        │
        ▼
Frontend AST
        │
        ▼
Semantic Model
        │
        ▼
Canonical IR
        │
        ▼
Compiler / Runtime

The authority rules are:

1. "grammar/spec/ai.md" defines AI semantic intent.
2. "grammar/ai/" owns AI-domain syntax.
3. "grammar/Zamani.g4" owns root grammar composition.
4. "grammar/grammar.md" describes implementation conformance.
5. "grammar/Zamani-Grammar.md" remains design/history material unless features are promoted.
6. "src/frontend/ast/" owns the actual source AST representation.
7. Semantic analysis owns meaning validation.
8. Canonical IR owns lowered semantic representation.
9. AI does not create a second canonical language-wide IR.
10. AI does not create a competing quantum IR.
11. "quantum::ir" remains the canonical quantum semantic boundary.

No lower-level implementation may silently redefine the semantics specified here.

---

3. Ownership

3.1 This specification owns

This file owns the normative AI-domain contract for:

- AI model semantics;
- tensor semantics;
- dataset semantics;
- training intent;
- inference intent;
- evaluation intent;
- differentiation intent;
- AI pipelines;
- AI agents;
- model composition;
- AI capabilities;
- AI resource intent;
- AI constraints;
- AI preferences;
- AI portability;
- AI/classical interoperability;
- AI/quantum interoperability;
- AI/HDL interoperability;
- AI/hardware interoperability;
- AI/distributed interoperability;
- AI/networking interoperability;
- AI/security boundaries;
- AI dialect integration;
- AI feature lifecycle;
- AI conformance requirements;
- AI scalability requirements;
- AI hard-coding rules.

3.2 This specification does not own

It does not own:

- lexical tokenization;
- general identifiers;
- general expressions;
- general statements;
- general types;
- ownership implementation;
- memory allocation;
- tensor storage;
- numerical kernels;
- automatic differentiation algorithms;
- optimizer implementations;
- model execution;
- model serving infrastructure;
- GPU discovery;
- CPU discovery;
- accelerator discovery;
- hardware topology;
- physical placement;
- distributed scheduling;
- runtime scheduling;
- compiler optimization algorithms;
- vendor APIs;
- CUDA/ROCm/TPU/NPU implementation;
- quantum gates;
- quantum routing;
- QEC;
- ZQN;
- HAL;
- calibration;
- physical qubit mapping.

Those belong to their respective repository contracts.

---

4. AI Is a Domain, Not a Separate Language

Zamani AI is part of the same programming language as:

- classical computing;
- quantum computing;
- hybrid computing;
- HDL;
- hardware/software co-design;
- distributed computing;
- parallel computing;
- networking;
- security;
- data processing;
- scientific computing;
- embedded computing;
- accelerators;
- future computational domains.

AI syntax must therefore compose with ordinary Zamani constructs.

An AI computation can occur:

- inside a function;
- inside a module;
- inside a loop;
- inside a conditional;
- inside a concurrent task;
- inside a distributed computation;
- as part of a quantum-classical hybrid computation;
- as part of an HDL/hardware co-design;
- as part of a data pipeline;
- as part of an agent;
- as a library-defined operation.

AI must not require a separate source language.

---

5. Core AI Principle

The fundamental rule is:

«AI syntax describes computational meaning and portable intent. Compilation and runtime determine how that computation is realized.»

A Zamani AI program may express:

what model is computed
what data is consumed
what outputs are produced
what mathematical relationships exist
what capabilities are required
what resources are required
what constraints apply
what preferences exist
what correctness properties apply

It must not require the programmer to specify:

which physical GPU
which CPU core
which TPU
which NPU
which FPGA
which device ID
which memory bank
which VRAM address
which cluster node
which physical interconnect
which accelerator index
which hardware thread

unless the program explicitly enters a target-specific realization layer.

---

6. POCO-REAF Contract

POCO-REAF means:

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
- resource availability;
- declared constraints;
- compatibility policy;
- explicit target-specific requirements.

AI syntax must be portable across machines with different:

- CPU architectures;
- GPU architectures;
- accelerator types;
- memory capacities;
- vector widths;
- tensor units;
- device counts;
- cluster sizes;
- network topologies;
- quantum/classical resources;
- storage capacities.

The source program should describe the computation rather than today's machine.

---

7. No Artificial AI Limits

The language MUST NOT establish universal limits for:

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
MAX_PIPELINE_STAGES
MAX_DATASET_SIZE
MAX_WORKERS
MAX_DEVICES
MAX_ACCELERATORS
MAX_NODES
MAX_TRAINING_STEPS
MAX_EPOCHS
MAX_MODEL_DEPTH
MAX_MODEL_WIDTH

or equivalent constructs.

The absence of a language-level limit does not mean every implementation can execute every program.

Implementations may encounter:

- parser resource exhaustion;
- compiler resource exhaustion;
- memory exhaustion;
- target resource exhaustion;
- execution budget exhaustion;
- security policy limits;
- deployment limits;
- numerical feasibility limits.

Such failures are implementation/resource conditions.

They must not be represented as artificial language semantics.

---

8. Semantic Requirements Versus Implementation Decisions

Zamani AI must distinguish:

8.1 Semantic requirement

requires capability("tensor.autodiff")

8.2 Resource requirement

requires resource("memory", amount)

8.3 Preference

prefers capability("accelerated.tensor.compute")

8.4 Constraint

constrains latency < budget

8.5 Implementation decision

map model X to device Y

The first four are portable program intent.

The fifth is target realization and must remain outside portable AI semantics unless explicitly permitted by a target-specific dialect.

---

9. AI Resource Semantics

AI resource expressions may describe:

- compute requirements;
- memory requirements;
- storage requirements;
- communication requirements;
- bandwidth requirements;
- latency requirements;
- throughput requirements;
- energy preferences;
- reliability requirements;
- accelerator capabilities;
- parallelism requirements;
- distributed execution requirements;
- quantum capabilities;
- classical capabilities.

Resource syntax must integrate with:

grammar/resources/
grammar/hardware/
grammar/compile/
grammar/execution/

AI must not create an independent resource model.

---

10. Capability Semantics

Capabilities describe what an execution environment must be able to do.

Examples include:

tensor.compute
tensor.autodiff
tensor.dynamic_shape
model.inference
model.training
distributed.training
distributed.inference
accelerator.compute
quantum.hybrid
streaming.data
secure.compute

Capability identifiers are semantic names.

They are not device identifiers.

A capability may be provided by:

- a CPU;
- a GPU;
- an FPGA;
- an NPU;
- a QPU;
- a distributed cluster;
- a software implementation;
- a future execution platform.

The compiler and runtime determine how a capability is satisfied.

---

11. AI Model Contract

A model is a semantic computational object.

A model may contain:

- inputs;
- outputs;
- parameters;
- state;
- components;
- layers;
- submodels;
- connections;
- transformations;
- constraints;
- capabilities;
- resource requirements;
- metadata;
- contracts.

A model must not intrinsically encode a particular hardware target.

The following are semantic model properties:

input types
output types
parameter types
state
computation graph
dependencies
mathematical operations
training objective
inference behavior

The following are not intrinsic model semantics:

GPU number
CPU number
physical memory address
device index
specific accelerator
physical topology
specific memory bank

---

12. Model Identity

A model declaration has:

- a source-level name;
- optional generic parameters;
- optional semantic metadata;
- an interface;
- a body;
- optional contracts.

Model identity must be independent of:

- source formatting;
- compiler process ID;
- memory address;
- host pointer;
- compilation order;
- physical device;
- runtime process;
- hardware identifier.

Canonical semantic identity belongs to semantic analysis.

---

13. Model Interfaces

A model interface consists of typed semantic ports.

Inputs and outputs must use the canonical Zamani type system.

For example:

@model Classifier {
    @input features: Tensor<Float>;
    @output prediction: Tensor<Float>;
}

The exact syntax is owned by "grammar/ai/models.g4".

This specification owns the meaning.

A model may have an arbitrary finite number of inputs and outputs subject to implementation resources.

No grammar-level maximum exists.

---

14. Model Parameters

Model parameters may be:

- trainable;
- non-trainable;
- compile-time;
- runtime;
- symbolic;
- dependent;
- dynamically shaped where the type system permits it.

Parameter count is unbounded by language semantics.

Parameter storage is not owned by the grammar.

---

15. Model State

Model state may represent:

- learned state;
- execution state;
- persistent state;
- recurrent state;
- optimizer state;
- streaming state;
- application state.

State lifetime is governed by the memory, ownership, persistence, and execution specifications.

The AI grammar must not implement state storage.

---

16. Model Components

Components provide semantic decomposition.

A component may represent:

- a layer;
- a submodel;
- a transformation;
- an encoder;
- a decoder;
- an attention mechanism;
- a classical operation;
- a quantum operation;
- an HDL/hardware operation;
- a user-defined operation;
- a future AI construct.

The grammar must not enumerate every possible component type.

This permits extension without changing the core grammar.

---

17. Model Layers

Layer syntax must be generic enough to support:

- dense layers;
- convolution;
- recurrent computation;
- attention;
- transformer components;
- graph computation;
- probabilistic components;
- symbolic components;
- differentiable programs;
- quantum-enhanced layers;
- user-defined layers;
- future architectures.

A layer is a semantic operation/component.

A layer keyword must not become a catalogue of every AI framework.

Framework-specific layer libraries belong in libraries or dialects.

---

18. Model Composition

Models may contain or reference other models.

Composition may form:

- sequential structures;
- branching structures;
- graph structures;
- nested models;
- ensembles;
- mixtures;
- reusable components;
- dynamically selected components where semantics permit.

Composition count is unbounded.

Composition depth is not a language-level constant.

---

19. Model Graphs

Model graphs represent semantic dependencies.

They do not represent physical hardware topology.

A model edge means:

value/data/control dependency

not:

network cable
GPU interconnect
PCIe link
quantum coupling
physical memory connection

Physical mapping occurs downstream.

---

20. Tensor Contract

Tensors are typed mathematical/data objects.

A tensor may have:

- element type;
- shape;
- rank;
- symbolic dimensions;
- dynamic dimensions;
- layout metadata;
- semantic attributes;
- indexing;
- slicing;
- transformation operations.

Tensor syntax belongs to "grammar/ai/tensors.g4" while type semantics remain governed by "grammar/spec/type-system.md".

---

21. Tensor Dimensions

Tensor dimensions may be:

- literal;
- constant;
- generic;
- symbolic;
- runtime-derived;
- dependent where supported.

Example:

Tensor<T, [batch, sequence, features]>

does not require the programmer to know the machine's physical memory capacity.

The compiler may specialize the program after discovering target resources.

---

22. Tensor Rank

There is no universal maximum tensor rank in Zamani.

The grammar must use repetition or recursive structure rather than enumerating:

Tensor1
Tensor2
Tensor3
...
TensorN

The semantic model must support arbitrary finite rank subject to implementation resources.

---

23. Tensor Storage

Tensor storage is downstream from syntax.

The language does not determine:

- row-major layout;
- column-major layout;
- tiled layout;
- blocked layout;
- accelerator-specific layout;
- physical memory bank;
- cache placement;
- VRAM placement.

These may be selected during lowering and optimization.

---

24. Tensor Operations

Tensor operations should be represented through semantic operations and canonical expressions rather than requiring a keyword for every mathematical function.

The language may support:

- arithmetic;
- matrix operations;
- contraction;
- broadcasting;
- reduction;
- indexing;
- slicing;
- reshaping;
- permutation;
- concatenation;
- transformation;
- convolution;
- attention;
- user-defined tensor operations.

Library-level operations should not automatically become core grammar keywords.

---

25. Dataset Contract

Datasets represent structured or streaming data used by computation.

A dataset may have:

- schema;
- features;
- labels;
- metadata;
- partitions;
- transformations;
- provenance;
- streaming semantics;
- persistence references;
- external references.

Dataset size is not a grammar limit.

A dataset may contain:

zero or more records

subject to semantic validity and available resources.

---

26. Dataset Storage

The AI grammar does not own:

- filesystem access;
- object storage;
- databases;
- cloud storage;
- network transport;
- compression;
- serialization implementation.

These belong to data, storage, networking, interoperability, and runtime subsystems.

---

27. Dataset Security

Dataset declarations may express security/capability requirements.

For example:

requires capability("data.read")

The grammar does not grant permission.

Authorization belongs to the security and capability systems.

---

28. Training Contract

Training expresses model optimization intent.

A training construct may specify:

- model;
- training data;
- objective;
- loss;
- parameters;
- evaluation;
- stopping condition;
- checkpoint intent;
- validation;
- resource requirements;
- capabilities;
- constraints;
- preferences.

Training syntax must not implement optimization algorithms.

---

29. Training Steps and Epochs

Training steps and epochs are semantic quantities.

They may be:

- constants;
- symbolic;
- runtime-derived;
- conditionally terminated;
- resource-dependent.

The grammar must not establish a maximum number of:

- epochs;
- steps;
- iterations;
- samples;
- batches.

---

30. Batch Semantics

Batch size is a semantic/program value when explicitly specified.

The compiler may later transform:

batch

into a target-dependent execution strategy.

The source language must not require a specific batch size merely because a particular accelerator has a particular memory capacity.

---

31. Training Optimization

Optimization intent may express:

- minimize loss;
- maximize objective;
- satisfy constraints;
- converge according to a condition;
- optimize a declared metric.

The implementation of optimization belongs downstream.

Zamani does not require the grammar to encode:

- SGD;
- Adam;
- RMSProp;
- L-BFGS;
- vendor-specific optimizer APIs;

as core language constructs.

Such algorithms may exist as standard-library or dialect operations.

---

32. Inference Contract

Inference represents application of a model to input data.

It may support:

- single inference;
- batch inference;
- streaming inference;
- asynchronous inference;
- distributed inference;
- pipelined inference;
- interactive inference.

The grammar must not prescribe the serving architecture.

---

33. Inference Scalability

The language must permit:

one input
many inputs
streaming inputs
distributed inputs
dynamic request volumes

without modifying the source language.

Capacity is a deployment/runtime property.

---

34. Evaluation

AI evaluation may express:

- metrics;
- comparison;
- validation;
- test datasets;
- objectives;
- thresholds;
- statistical properties;
- correctness contracts.

Metric implementations belong to libraries/semantic operations.

The grammar should not become an exhaustive list of all possible AI metrics.

---

35. Differentiation Contract

Differentiation syntax expresses mathematical differentiation intent.

Supported semantic concepts may include:

- derivative;
- gradient;
- Jacobian;
- Hessian;
- directional derivative;
- differentiability boundary;
- gradient propagation;
- custom derivative definition.

The grammar does not select the implementation strategy.

---

36. Automatic Differentiation

The compiler may implement differentiation through:

- forward mode;
- reverse mode;
- symbolic transformation;
- mixed mode;
- numerical approximation where explicitly requested;
- another valid strategy.

The source semantics must distinguish mathematical intent from implementation strategy.

---

37. Differentiation Correctness

A differentiation operation must preserve:

- type semantics;
- effect semantics;
- ownership semantics;
- resource semantics;
- numerical semantics;
- quantum semantics where applicable.

Differentiation must not silently change the meaning of a program because it was compiled for another target.

---

38. Quantum-AI Integration

AI may interact with quantum computation.

Examples include:

- quantum-enhanced models;
- hybrid optimization;
- quantum feature maps;
- variational computations;
- quantum kernels;
- quantum sampling;
- quantum inference components.

However:

«AI grammar does not define quantum semantics.»

Quantum semantics remain owned by:

grammar/quantum/

and ultimately:

quantum::ir

The AI subsystem must never define:

- "QubitId";
- physical qubit identifiers;
- gate semantics;
- quantum topology;
- QEC;
- ZQN;
- quantum scheduling;
- physical mapping.

AI-to-quantum integration is a semantic/lowering relationship.

---

39. Quantum Differentiation

When AI differentiation crosses into quantum computation:

AI source
   ↓
domain-neutral AST
   ↓
semantic analysis
   ↓
quantum semantic representation
   ↓
quantum::ir
   ↓
quantum differentiation / optimization / lowering

AI grammar must not create a second quantum representation.

---

40. Classical-AI Integration

AI operations may use:

- scalar computation;
- integer computation;
- floating point;
- vectors;
- matrices;
- tensors;
- symbolic computation;
- numerical computation;
- control flow;
- functions;
- concurrency.

AI must reuse the universal classical language facilities.

It must not duplicate classical expression or type syntax.

---

41. HDL / Hardware Integration

AI programs may express hardware intent through:

- accelerator requirements;
- compute capabilities;
- memory requirements;
- latency constraints;
- throughput preferences;
- hardware/software co-design constructs.

AI grammar does not define:

- FPGA routing;
- ASIC placement;
- physical wires;
- timing closure;
- transistor layout;
- device-specific accelerator instructions.

Those belong to HDL/hardware/compiler subsystems.

---

42. AI Accelerator Contract

Accelerator syntax expresses capability and intent.

Valid semantic concepts include:

requires capability("accelerated.tensor.compute")
prefers capability("low_precision.compute")
requires capability("tensor.autodiff")

The language must not require:

GPU 0
GPU 1
TPU 2
NPU 7

as portable AI semantics.

---

43. Vendor Independence

The core AI grammar must remain vendor-neutral.

Vendor-specific support belongs under:

grammar/dialects/
grammar/interoperability/

or downstream compiler/backend contracts.

A vendor-specific dialect must declare:

- name;
- version;
- semantic additions;
- syntax additions;
- compatibility;
- portability implications;
- lowering contract;
- target requirements.

---

44. Distributed AI

AI must compose with the distributed subsystem.

Supported semantic patterns may include:

- data parallelism;
- model parallelism;
- pipeline parallelism;
- parameter partitioning;
- distributed inference;
- distributed training;
- replication;
- sharding;
- collective operations;
- fault tolerance.

The grammar must not require a fixed number of workers or nodes.

---

45. Distributed Resource Semantics

A program may express:

requires capability("distributed.training")

rather than:

use 128 nodes

unless the exact node count is genuinely part of program semantics.

The compiler/runtime determines an admissible realization.

---

46. AI Pipelines

An AI pipeline represents semantic stages and dependencies.

A pipeline may contain:

data ingestion
preprocessing
transformation
training
evaluation
inference
postprocessing
storage
communication
classical computation
quantum computation
hardware computation

Pipeline stage count is unbounded.

The grammar does not implement pipeline scheduling.

---

47. Pipeline Dependencies

Pipeline dependencies represent:

- data dependencies;
- control dependencies;
- resource dependencies;
- semantic ordering constraints.

They do not directly represent:

- CPU scheduling;
- GPU stream assignment;
- physical network topology;
- accelerator placement.

Those are downstream decisions.

---

48. AI Agents

An AI agent is a semantic computational entity capable of:

- observing;
- reasoning;
- maintaining state;
- selecting actions;
- invoking tools;
- executing workflows;
- interacting with environments;
- using models;
- communicating.

The grammar may describe agent intent.

It must not directly execute tools.

---

49. Agent Security

An agent declaring a capability does not grant that capability.

For example:

requires capability("network.access")

means:

«this computation requires an environment satisfying that capability.»

It does not mean:

«grant unrestricted network access.»

Authorization is owned by:

security/
effects/
capabilities/
execution/

---

50. Agent Tooling

Tools may be represented as semantic interfaces.

Tool execution belongs to runtime/interoperability systems.

The grammar must not:

- execute processes;
- open files;
- access networks;
- inspect hardware;
- invoke arbitrary system calls.

---

51. Agent State and Memory

AI agent state may use the universal memory and persistence semantics.

The AI grammar must not implement a separate memory subsystem.

Where Zamani's broader Sankofa/temporal-memory facilities are used, they remain governed by their own semantic contracts.

---

52. AI + Dataflow

AI computations may compose with dataflow constructs.

Dataflow describes semantic movement of values.

It must remain distinct from physical memory placement.

A dataflow edge is not automatically:

- a memory channel;
- a network connection;
- a DMA path;
- a PCIe connection.

---

53. AI + Networking

AI programs may use networking through the networking subsystem.

Examples:

- model serving;
- distributed inference;
- remote datasets;
- distributed training;
- agent communication.

AI grammar does not own:

- IP addresses;
- sockets;
- transport protocols;
- routing tables.

Those belong to "grammar/networking/".

---

54. AI + Security

AI computations may declare:

- confidentiality requirements;
- integrity requirements;
- access capabilities;
- provenance requirements;
- privacy requirements;
- secure-computation requirements.

Security enforcement is downstream.

AI syntax must not weaken or bypass the security model.

---

55. AI + Cryptography

AI may use cryptographic operations through:

security/
interoperability/

rather than adding every cryptographic primitive to the AI grammar.

---

56. AI + Effects

AI operations may have effects.

Examples:

- model loading;
- dataset access;
- network access;
- persistent state;
- random sampling;
- accelerator interaction;
- distributed communication.

Effects must integrate with:

grammar/effects/
grammar/spec/effects.md

AI must not create an independent effect system.

---

57. Determinism

AI programs may be deterministic or nondeterministic.

The source language must make relevant nondeterminism explicit where semantics require it.

Sources of nondeterminism may include:

- random sampling;
- parallel execution;
- distributed execution;
- asynchronous arrival;
- unspecified ordering.

A compiler must not silently change a program's declared determinism semantics merely because the target differs.

---

58. Reproducibility

AI reproducibility may involve:

- random seeds;
- numerical precision;
- deterministic execution;
- dataset version;
- model version;
- dependency versions;
- semantic configuration;
- provenance.

Reproducibility is distinct from physical device identity.

---

59. Numerical Semantics

AI numerical operations must follow the universal numeric semantics.

The compiler must not silently change:

- precision;
- rounding;
- overflow behavior;
- NaN semantics;
- infinity semantics;

unless the source program explicitly permits such transformation or a formally valid optimization preserves semantics.

---

60. Mixed Precision

Mixed precision may be expressed as semantic intent.

For example:

prefer precision(...)

or through type-level semantics.

The grammar must not assume:

- one fixed accelerator precision;
- one fixed tensor-core width;
- one fixed vector width.

---

61. Quantization

Quantization may be represented as:

- a type;
- a transformation;
- an optimization intent;
- a library operation;
- a compiler transformation.

It must not require a fixed target architecture.

---

62. Sparsity

Sparsity is a semantic property where relevant.

The implementation may choose:

- dense representation;
- sparse representation;
- compressed representation;
- accelerator-specific representation.

The source program must not need to know the physical representation unless representation is explicitly part of its semantics.

---

63. Model Serialization

Model serialization is an interoperability concern.

The AI grammar may reference serialization formats.

It must not make a particular serialization format the universal semantic representation.

Examples of external formats may be handled through interoperability contracts.

---

64. Framework Independence

The core Zamani AI grammar must not become:

PyTorch language
TensorFlow language
JAX language
ONNX language
CUDA language
ROCm language
TPU language
vendor-specific accelerator language

Those may be interoperable dialects.

The core language describes AI computation independently of frameworks.

---

65. AI Dialects

AI dialects may extend the language.

A dialect must define:

dialect name
dialect version
syntax additions
semantic additions
type additions
capabilities
resource requirements
AST mapping
semantic mapping
IR mapping
compatibility
portability
deprecation policy

A dialect must not silently change the meaning of core Zamani syntax.

---

66. AI Grammar Files

The existing AI grammar directory should retain its existing files.

The intended responsibilities are:

grammar/ai/
├── README.md
├── ai.g4
├── models.g4
├── tensors.g4
├── datasets.g4
├── training.g4
├── inference.g4
├── agents.g4
├── pipelines.g4
├── differentiation.g4
└── ai-accelerators.g4

No unnecessary renaming is required.

---

67. "grammar/ai/ai.g4"

Owns

- AI composition boundary;
- AI parser-domain entry;
- AI construct dispatch;
- AI-domain integration points.

Does not own

- tensor internals;
- model internals;
- training algorithms;
- inference implementation;
- differentiation implementation;
- accelerator selection.

Integration

"ai.g4" must compose with:

core
types
expressions
statements
declarations
functions
effects
resources
classical
quantum
hardware
distributed
data
security

without redefining their grammar.

Completion criteria

"ai.g4" is complete when:

- every stable AI construct has an explicit entry;
- ordinary expressions cannot accidentally become AI constructs;
- no AI-specific duplicate type system exists;
- no AI-specific duplicate expression system exists;
- no physical device selection exists;
- no artificial resource limits exist;
- every entry has an AST contract;
- every entry has a semantic contract;
- every stable entry has conformance tests.

---

68. "grammar/ai/models.g4"

Owns:

- model declaration syntax;
- model interfaces;
- model inputs;
- model outputs;
- model parameters;
- model state;
- components;
- layers;
- submodels;
- connections;
- model-local metadata;
- model contracts.

Uses canonical:

identifier
qualifiedName
typeExpression
expression
genericParameterList
attribute

It must not redefine these.

Integration:

models.g4
    ↓
Frontend AST
    ↓
semantic model
    ↓
canonical IR

---

69. "grammar/ai/tensors.g4"

Owns:

- tensor syntax;
- tensor declarations;
- tensor shapes;
- symbolic dimensions;
- indexing;
- slicing;
- transformations;
- tensor metadata.

Does not own:

- tensor allocation;
- physical layout;
- memory placement;
- numerical kernels.

Integration:

tensors.g4
    ↓
canonical type/expression model
    ↓
tensor semantic analysis
    ↓
canonical IR

---

70. "grammar/ai/datasets.g4"

Owns:

- dataset declarations;
- schemas;
- features;
- labels;
- splits;
- transformations;
- streams;
- dataset metadata.

Does not own:

- filesystem;
- database;
- object storage;
- network transport.

Integration is through the data, storage, networking, security, and runtime contracts.

---

71. "grammar/ai/training.g4"

Owns:

- training syntax;
- objectives;
- loss declarations;
- evaluation;
- training configuration;
- checkpoint intent;
- training constraints.

Does not own:

- optimizer algorithms;
- scheduling algorithms;
- distributed runtime;
- accelerator allocation.

---

72. "grammar/ai/inference.g4"

Owns:

- inference declarations;
- model invocation;
- input/output bindings;
- inference policies;
- serving intent;
- batching intent;
- streaming intent.

Does not own:

- model servers;
- network servers;
- runtime scheduling;
- accelerator allocation.

---

73. "grammar/ai/agents.g4"

Owns:

- agent declarations;
- goals;
- observations;
- actions;
- state;
- tool interfaces;
- workflows;
- agent relationships.

Does not own:

- authorization;
- tool execution;
- networking implementation;
- process execution;
- model implementation.

---

74. "grammar/ai/pipelines.g4"

Owns:

- pipeline declarations;
- stages;
- dependencies;
- dataflow;
- model composition;
- pipeline intent.

Does not own:

- scheduler implementation;
- graph execution;
- physical placement.

---

75. "grammar/ai/differentiation.g4"

Owns:

- differentiation syntax;
- gradient requests;
- Jacobian requests;
- Hessian requests;
- derivative boundaries;
- differentiation annotations.

Does not own:

- AD algorithms;
- symbolic differentiation implementation;
- numerical differentiation implementation.

---

76. "grammar/ai/ai-accelerators.g4"

Owns:

- accelerator capability syntax;
- accelerator requirements;
- accelerator preferences;
- accelerator constraints.

It does not own:

- device enumeration;
- device IDs;
- hardware discovery;
- vendor APIs;
- physical topology;
- accelerator scheduling.

---

77. Root Grammar Integration

"grammar/Zamani.g4" remains the canonical ANTLR composition/root grammar.

AI-specific grammar files must not become a second root grammar.

The final composition must conceptually provide:

program
  └── compilationUnitItem*
        └── declaration
              └── AI domain entry

AI constructs must therefore be reachable from the normal Zamani compilation unit without requiring a separate parser mode.

---

78. Existing AI Grammar Dependency Correction

Existing AI grammar documentation may refer to paths such as:

grammar/antlr/ZamaniLexer.g4
grammar/antlr/Types.g4
grammar/expressions/expressions.g4
grammar/statements/statements.g4

Those references are implementation details and must not establish a second authority.

The canonical dependency rule is:

«The actual current parser/lexer implementation and "grammar/Zamani.g4" determine composition; this specification determines semantics.»

If an existing fragment path differs from the final canonical modular structure, the implementation may be reorganized without changing the semantic contract in this document.

No semantic specification is to be duplicated merely to match a physical file path.

---

79. AST Contract

AI syntax must lower into the existing domain-neutral frontend AST.

AI must not require a second AST architecture.

The AST must preserve enough information to represent:

- source span;
- construct kind;
- identifier/name;
- generic parameters;
- inputs;
- outputs;
- expressions;
- types;
- attributes;
- annotations;
- resource intent;
- capability intent;
- constraints;
- preferences;
- nested constructs;
- source ordering where semantically relevant.

AI AST nodes must remain source-structural.

They must not contain:

- physical GPU IDs;
- physical CPU IDs;
- physical qubit IDs;
- scheduler decisions;
- routing decisions;
- calibration data;
- runtime handles.

---

80. Generic Operation Principle

AI operations should follow the repository-wide generic operation principle.

Do not create:

enum AiOperation {
    Dense,
    Conv,
    Transformer,
    ...
}

merely to enumerate known AI operations.

Instead, the semantic model should support generic operations with:

name
namespace
operands
parameters
results
attributes
modifiers
effects
capabilities
source

Specific operations can then be validated by semantic registries, standard libraries, or dialects.

This permits future AI operations without grammar redesign.

---

81. Semantic Contract

Semantic analysis must validate:

- model declarations;
- references;
- types;
- tensor shapes;
- dimensions;
- parameter compatibility;
- data compatibility;
- model graph validity;
- differentiation legality;
- effect legality;
- capability requirements;
- resource requirements;
- security requirements;
- portability;
- determinism;
- domain interoperability.

Grammar parsing must not perform these checks.

---

82. Canonical IR Contract

AI must lower into the repository's canonical semantic representation.

Possible lowerings include:

AI computation
    ↓
classical IR

AI tensor operation
    ↓
classical/tensor semantic IR

AI quantum operation
    ↓
quantum::ir

AI hardware intent
    ↓
hardware/target semantic IR

AI distributed computation
    ↓
distributed semantic representation

There is no requirement for a separate universal "ai::ir".

If a dedicated AI semantic representation becomes necessary, it must live outside "grammar/" and integrate with the canonical IR architecture.

---

83. Quantum IR Boundary

AI must never create:

ai::quantum_ir

or another competing quantum representation.

Quantum computation originating in AI must eventually use:

quantum::ir

as the canonical quantum semantic boundary.

---

84. Compiler Integration

The compiler owns:

- specialization;
- constant propagation;
- tensor optimization;
- differentiation lowering;
- model optimization;
- graph transformation;
- fusion;
- parallelization;
- distribution;
- accelerator lowering;
- target adaptation.

The grammar does not own compiler algorithms.

---

85. Runtime Integration

The runtime receives lowered execution representations.

The runtime may determine:

- actual resource allocation;
- device selection;
- scheduling;
- memory placement;
- execution order where permitted;
- distributed placement;
- resilience;
- recovery.

The runtime must not need to parse AI grammar as its execution representation.

---

86. Resource Negotiation

A portable AI program may declare:

requires capability(...)
requires resource(...)
prefers capability(...)
constrains ...

The compiler/runtime may then negotiate a valid realization.

Possible outcomes include:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

where those states are governed by the repository's resilience/execution contracts.

---

87. Resource Failure

If a target cannot satisfy:

requires capability("tensor.autodiff")

the failure is a capability/resource feasibility failure.

It is not a grammar error.

Likewise:

requires resource("memory", required)

must not become a language-level memory maximum.

---

88. Hardware Discovery

Hardware discovery belongs downstream.

AI source code must not need to enumerate:

all GPUs
all CPUs
all accelerators
all QPUs
all nodes

in order to remain valid.

The compiler/runtime/HAL can discover available capabilities.

---

89. Scaling From Tiny to Arbitrarily Large

The same semantic program must be representable for:

tiny model
small model
large model
distributed model
massively distributed model
future computational scale

subject to:

- available resources;
- target capabilities;
- program semantics;
- explicit constraints.

The language itself must not become less expressive because scale increases.

---

90. Compile-Once Principle

Compilation must preserve semantic portability.

A compiler may specialize an artifact for a target when specialization is explicitly part of the compilation model.

However, the portable source semantics remain independent of that specialization.

The language must distinguish:

portable semantic program

from:

target realization

---

91. Reproducible Compilation

AI compilation should preserve:

- source identity;
- semantic identity;
- dependency versions;
- model metadata;
- dataset references;
- compilation profile;
- relevant capabilities;
- resource requirements;
- deterministic settings.

Compiler-generated artifacts must not become the semantic source of truth.

---

92. Provenance

AI computations may expose provenance for:

- datasets;
- models;
- transformations;
- training runs;
- inference;
- dependencies;
- generated artifacts.

Provenance is metadata/semantic information.

It does not authorize access.

---

93. Versioning

AI syntax follows Zamani language versioning.

A feature may be:

proposed
experimental
preview
stable
deprecated
removed

A feature is stable only after:

1. specification;
2. grammar;
3. AST contract;
4. semantic contract;
5. IR contract;
6. implementation;
7. positive tests;
8. negative tests;
9. boundary tests;
10. scalability tests;
11. compatibility tests;
12. diagnostics;
13. hard-coding audit.

---

94. Compatibility

AI changes must preserve source compatibility where promised.

Breaking changes require:

- language version;
- migration documentation;
- compatibility entry;
- updated conformance tests.

"grammar/Zamani-Grammar.md" cannot independently introduce a breaking AI syntax change.

---

95. Diagnostics

AI diagnostics must identify:

- source location;
- construct;
- violated rule;
- relevant types;
- relevant capabilities;
- relevant resources;
- suggested semantic correction where appropriate.

Diagnostics must distinguish:

SYNTAX_ERROR
TYPE_ERROR
SEMANTIC_ERROR
CAPABILITY_ERROR
RESOURCE_ERROR
PORTABILITY_ERROR
SECURITY_ERROR
COMPATIBILITY_ERROR

as appropriate.

A lack of target resources must not be misreported as malformed AI syntax.

---

96. Source Spans

Every AI AST construct must retain source provenance sufficient for:

- diagnostics;
- IDE tooling;
- formatting;
- navigation;
- semantic errors;
- compiler diagnostics;
- provenance;
- testing.

Source spans must not depend on runtime object addresses.

---

97. Tooling Integration

AI grammar must support:

- syntax highlighting;
- parser diagnostics;
- formatter integration;
- language-server parsing;
- semantic navigation;
- documentation extraction;
- refactoring;
- source mapping;
- conformance tooling.

Tooling must consume the canonical AST/specification.

---

98. Safety Requirements

The Rust implementation of AI grammar and its tooling must target:

Rust 1.97 / Rust 1.97.1
Rust 2021

and must use safe Rust.

No AI grammar implementation may require:

unsafe

or equivalent unsound escape mechanisms.

The grammar files themselves must contain no embedded runtime code or unsafe actions.

ANTLR actions and semantic predicates should not be used to implement AI semantics.

---

99. Deterministic Parsing

AI grammar parsing must be deterministic to the extent permitted by the parser architecture.

The grammar must avoid:

- accidental ambiguous alternatives;
- duplicate entry points;
- parser-level AI/type ambiguity;
- parser-level AI/expression ambiguity;
- semantic decisions hidden inside grammar actions.

Semantic ambiguity must be handled by semantic analysis rather than unsafe parser tricks.

---

100. AI Grammar and Ordinary Expressions

An ordinary expression must not automatically become an AI construct.

For example:

x + y

is an ordinary expression.

It should not become an AI expression simply because "x" and "y" happen to be tensors.

Semantic type information can determine that the expression is tensor-valued.

This keeps syntax separate from domain semantics.

---

101. AI Annotations

Annotations may identify AI-domain roles such as:

@model
@input
@output
@parameter
@state
@dataset
@tensor
@training
@inference
@agent
@pipeline
@accelerator

The annotation mechanism must remain extensible.

Adding a new AI concept must not necessarily require adding a lexer keyword.

Semantic analysis determines whether an annotation is registered and valid.

---

102. Annotation Normalization

Annotation identity must be canonicalized semantically.

For example:

@model

must have one canonical semantic identity.

Unknown annotations may be:

- rejected;
- accepted as dialect extensions;
- preserved for tooling;

according to the annotation/dialect policy.

The parser must not assign implementation-specific meaning to unknown annotations.

---

103. AI Standard Library Boundary

Core language grammar must remain small.

AI libraries may provide:

- standard model components;
- mathematical operations;
- optimization algorithms;
- metrics;
- data transforms;
- activation functions;
- distributions;
- probabilistic operations;
- model architectures.

A library operation is not automatically a grammar keyword.

This is essential for long-term scalability.

---

104. Framework Interoperability

Framework-specific import/export belongs to:

grammar/interoperability/

The AI specification must support interoperable representation without making external frameworks the semantic authority.

Imported framework constructs must be mapped into Zamani semantics.

They must not force Zamani's core grammar to become framework-specific.

---

105. External Model Formats

External model formats may be supported through interoperability adapters.

The semantic flow is:

external format
      ↓
interoperability frontend
      ↓
Zamani semantic model
      ↓
canonical IR

not:

external format
      ↓
special runtime representation
      ↓
bypass Zamani semantics

---

106. Security Boundary

AI syntax must never bypass:

- ownership;
- effects;
- capabilities;
- authorization;
- resource policy;
- sandboxing;
- provenance.

A model declaration does not grant permissions.

An agent declaration does not grant tool access.

A dataset declaration does not grant data access.

An accelerator requirement does not grant device access.

---

107. Privacy

Privacy requirements may be represented semantically where supported.

Examples include:

- data classification;
- access constraints;
- provenance;
- privacy requirements;
- secure computation capabilities.

The AI grammar does not itself implement privacy enforcement.

---

108. Fault Tolerance

AI computation may interact with the repository's resilience system.

AI grammar may express requirements such as:

requires capability("fault_tolerant_execution")

but must not implement:

- retries;
- recovery;
- health-state transitions;
- checkpoint orchestration;
- QEC;
- ZQN.

Those remain downstream concerns.

---

109. AI + ZQN

If AI computation interacts with quantum or fault/noise-aware execution:

AI
 ↓
semantic analysis
 ↓
quantum::ir
 ↓
ZQN

AI grammar must not define ZQN semantics.

---

110. AI + QEC

AI programs may require error-corrected quantum computation.

The source may express a capability or resource requirement.

QEC implementation remains outside AI grammar.

---

111. AI + HAL

AI may require a capability that the HAL can satisfy.

For example:

requires capability("tensor.acceleration")

The HAL determines whether and how the target can provide it.

AI source does not directly manipulate HAL objects.

---

112. AI + Scheduling

AI pipeline and graph structure provide semantic dependencies.

Scheduling decides:

- execution order;
- parallelism;
- placement;
- resource allocation;
- synchronization.

AI grammar must never encode a universal scheduler.

---

113. AI + Optimization

AI semantics may permit optimization.

The optimizer may perform:

- graph simplification;
- operation fusion;
- constant folding;
- differentiation transformations;
- memory optimization;
- parallelization;
- distribution;
- accelerator lowering.

Optimization must preserve source semantics.

---

114. AI + Concurrency

AI may compose with:

- async;
- tasks;
- actors;
- channels;
- parallel loops;
- data parallelism;
- task parallelism;
- pipelines.

The AI grammar must not duplicate concurrency syntax.

---

115. AI + Memory

Tensor/model/data memory must use the universal memory semantics.

AI must not create a separate ownership model.

Memory placement remains an implementation concern unless explicitly part of a target-specific semantic contract.

---

116. AI + Resource Scaling

The semantic resource model must allow expressions such as:

required_memory = model_memory(...)
required_compute = computation_cost(...)
required_bandwidth = ...

without requiring a fixed host integer representation for every semantic quantity.

This follows the universal type-system rule that semantic cardinalities must not silently become host "usize" values.

---

117. Semantic Quantity Rule

AI quantities such as:

- parameter count;
- tensor dimensions;
- model size;
- dataset cardinality;
- sequence length;
- batch size;
- training steps;

must support the universal Zamani semantic quantity model.

They may be:

- literal;
- arbitrary finite;
- symbolic;
- constrained;
- dependent;
- runtime-derived where permitted.

They must not silently overflow.

---

118. AI Type Integration

AI types use the universal type system.

Examples include semantic forms such as:

Tensor<T>
Tensor<T, Shape>
Model<Input, Output>
Dataset<T>
Agent<State>

where supported.

AI must not define an incompatible second generic/type system.

---

119. Generic AI Constructs

AI constructs may be generic over:

- element types;
- shapes;
- precision;
- model configuration;
- labels;
- feature types;
- data schemas;
- capabilities where supported.

Generic parameters must remain semantic.

---

120. Dependent and Symbolic Shapes

Where the universal type system permits dependent values, AI tensors may use symbolic relationships.

Example:

Tensor<T, [batch, sequence, features]>

The compiler may later determine concrete values.

This is essential for POCO-REAF.

---

121. Dynamic Shapes

AI syntax may represent runtime-determined shapes.

The compiler must not require all dimensions to be compile-time constants unless the selected target or operation explicitly requires such specialization.

When specialization is required, it belongs to compilation/lowering.

---

122. Static Shape Checking

Where dimensions are statically known, semantic analysis should detect invalid operations.

For example:

Matrix<A, B> × Matrix<C, D>

may be rejected when the semantic constraints prove:

B != C

This is semantic validation, not grammar parsing.

---

123. Dynamic Shape Errors

When dimensions cannot be proven statically, the semantic/runtime contract may require runtime validation.

The grammar remains unchanged.

---

124. Model Graph Validation

Semantic analysis must detect:

- invalid references;
- missing inputs;
- incompatible outputs;
- type mismatches;
- invalid cycles where cycles are prohibited;
- illegal recursion where prohibited;
- unresolved model components.

The grammar only establishes structural syntax.

---

125. Pipeline Validation

Semantic analysis must validate:

- stage references;
- dependency existence;
- type compatibility;
- dataflow compatibility;
- resource compatibility;
- effect compatibility;
- capability requirements.

Scheduling remains downstream.

---

126. Agent Validation

Semantic analysis must validate:

- tool references;
- state references;
- model references;
- capability requirements;
- effect requirements;
- type compatibility.

Authorization remains outside grammar.

---

127. AI Testing Contract

The AI grammar must have tests under:

grammar/tests/

and, where appropriate, domain-specific AI fixtures under:

grammar/tests/ai/

Tests must cover:

- lexical conformance;
- syntax;
- AST mapping;
- semantic expectations;
- diagnostics;
- compatibility;
- scalability.

---

128. Positive Tests

Positive tests must include at minimum:

minimal model
model with inputs
model with outputs
model with parameters
model with state
nested model
composed model
tensor declaration
symbolic tensor
dynamic tensor
dataset
streaming dataset
training
inference
evaluation
differentiation
pipeline
agent
accelerator requirement
resource requirement
capability requirement
classical-AI composition
quantum-AI composition
distributed-AI composition
hardware-AI composition

---

129. Negative Tests

Negative tests must include:

- malformed model;
- malformed tensor;
- invalid shape;
- invalid type;
- invalid model reference;
- invalid pipeline dependency;
- invalid agent tool;
- malformed training declaration;
- invalid differentiation;
- invalid capability expression;
- invalid resource expression;
- invalid annotation;
- invalid cross-domain construct.

---

130. Boundary Tests

Boundary tests must verify:

- empty legal structures where permitted;
- one-element structures;
- deeply nested structures;
- large numbers of members;
- long symbolic expressions;
- large model graphs;
- large pipeline graphs;
- large tensor rank;
- large generic parameter sets.

The test suite must not turn a practical test size into a language maximum.

---

131. Scalability Tests

Scalability tests must verify that the grammar does not contain artificial limits.

Test generators should vary:

number of models
number of parameters
number of tensors
tensor rank
tensor dimensions
number of layers
number of pipeline stages
number of agents
number of dataset transformations
number of distributed workers

without assuming a universal upper bound.

---

132. Hard-Coding Tests

The validation system must search for prohibited universal constants and constructs such as:

MAX_TENSORS
MAX_LAYERS
MAX_PARAMETERS
MAX_GPUS
MAX_CPUS
MAX_DEVICES
MAX_NODES
MAX_BATCH_SIZE
MAX_TENSOR_RANK

and equivalent aliases.

The check must distinguish:

program-defined constant

from:

language implementation limit

A user program may legitimately contain:

const BATCH_SIZE = 1024;

when that is program semantics.

The language specification must not turn "1024" into a universal maximum.

---

133. Device Identifier Rule

Portable AI grammar must not define physical device identifiers as universal semantics.

Forbidden universal constructs include:

gpu0
gpu1
device0
accelerator0
cpu0
node0

when they mean physical target assignment.

Target-specific dialects may provide such constructs only when explicitly marked non-portable.

---

134. Portability Classification

Every AI feature should have one of:

PORTABLE
TARGET_AWARE
TARGET_SPECIFIC
NON_PORTABLE

Core AI features should be "PORTABLE".

Target-specific features must explicitly declare portability consequences.

---

135. Resource Availability

The meaning of:

«scale from tiny to infinity»

is interpreted as:

«the language imposes no artificial finite resource ceiling; actual execution is bounded only by semantic requirements, implementation limits, available resources, target capabilities, and explicit policies.»

No programming language can guarantee physically infinite execution.

The grammar must nevertheless remain semantically unbounded with respect to machine-specific cardinalities.

---

136. AI Grammar Completion Contract

A file is not complete merely because its parser rules compile.

Each AI grammar file is complete only when:

- purpose is documented;
- ownership is documented;
- non-ownership is documented;
- dependencies are documented;
- upstream contracts are documented;
- downstream consumers are documented;
- lexical ownership is documented;
- AST mapping is documented;
- semantic mapping is documented;
- IR mapping is documented;
- compiler integration is documented;
- runtime integration is documented;
- cross-domain integration is documented;
- diagnostics are documented;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- compatibility tests exist;
- hard-coding audit passes;
- source spans are preserved;
- no duplicate semantic subsystem exists.

This makes each file independently completable.

---

137. Feature Manifest Contract

Future AI features should have a machine-readable feature manifest under the repository's feature-contract system when that system is introduced.

A feature manifest should identify:

feature_id
name
status
version
specification
grammar_files
lexer_dependencies
ast_contract
semantic_contract
type_contract
effect_contract
resource_contract
capability_contract
ir_mapping
compiler_consumers
runtime_consumers
tooling_consumers
positive_tests
negative_tests
boundary_tests
scalability_tests
compatibility_tests
portability
hard_coding_policy

The manifest prevents a feature from being considered complete merely because grammar syntax exists.

---

138. AI Feature Promotion

The promotion pipeline is:

Idea
 ↓
AI specification
 ↓
type/effect/resource analysis
 ↓
AST contract
 ↓
grammar
 ↓
semantic implementation
 ↓
IR mapping
 ↓
compiler integration
 ↓
runtime integration
 ↓
tests
 ↓
compatibility
 ↓
hard-coding audit
 ↓
stable

No feature should skip this process.

---

139. "grammar/Zamani-Grammar.md" Relationship

"grammar/Zamani-Grammar.md" may contain broader AI concepts.

Those concepts are not automatically normative.

A feature becomes normative only after it is reflected in:

grammar/spec/ai.md

and its corresponding contracts and implementation.

This prevents the broad design grammar from silently becoming a second specification.

---

140. "grammar/grammar.md" Relationship

"grammar/grammar.md" should describe what the implementation currently accepts.

It must distinguish:

SPECIFIED
IMPLEMENTED
PARTIALLY_IMPLEMENTED
EXPERIMENTAL
DEPRECATED

AI syntax documented here must correspond to actual parser behavior.

---

141. "grammar/README.md" Relationship

The grammar root README should point to:

grammar/spec/ai.md

as the normative AI semantic contract.

It should not reproduce this document.

---

142. "grammar/ai/README.md" Relationship

"grammar/ai/README.md" should remain the navigation and implementation guide for the AI grammar directory.

It should defer semantic authority to:

grammar/spec/ai.md

It must not redefine the AI semantics independently.

---

143. "grammar/spec/type-system.md" Relationship

AI types must use the universal type-system contract.

In particular:

- tensor dimensions;
- model parameters;
- dataset cardinalities;
- symbolic quantities;
- generic AI types;

must obey the semantic quantity and type rules already defined there.

No AI-specific type system may override the universal type system.

---

144. "grammar/spec/resources.md" Relationship

AI resource declarations must use the universal resource model.

The resource model distinguishes:

requirement
constraint
capability
preference
hint
implementation decision

AI must not collapse these into one construct.

---

145. "grammar/spec/effects.md" Relationship

AI effects such as:

- data access;
- network access;
- model loading;
- persistence;
- randomness;
- accelerator interaction;
- distributed communication;

must use the universal effect model.

---

146. "grammar/spec/semantics.md" Relationship

AI computation is ultimately governed by the universal semantics contract.

This specification provides AI-domain specialization only where necessary.

Universal semantics remain authoritative for:

- evaluation;
- scope;
- binding;
- control flow;
- effects;
- ownership;
- determinism;
- concurrency.

---

147. "grammar/spec/portability.md" Relationship

AI portability must comply with the universal portability specification.

A feature that requires a particular vendor or device must explicitly declare that dependency.

Core AI syntax should remain portable.

---

148. "grammar/classical/" Relationship

AI may lower to classical numerical computation.

Classical arithmetic and mathematical semantics remain owned by the classical subsystem.

AI must not duplicate:

- vector semantics;
- matrix semantics;
- arithmetic;
- numerical types.

---

149. "grammar/quantum/" Relationship

Quantum semantics remain owned by the quantum subsystem.

AI may reference quantum computation.

AI must not redefine quantum semantics.

---

150. "grammar/hardware/" Relationship

Hardware requirements belong to the hardware/resource contracts.

AI may declare requirements.

Hardware realization belongs downstream.

---

151. "grammar/distributed/" Relationship

Distributed execution semantics belong to the distributed subsystem.

AI may express distributed computation intent.

The distributed subsystem determines communication and placement semantics.

---

152. "grammar/data/" Relationship

Datasets and data transformations should reuse universal data semantics wherever possible.

AI-specific dataset constructs should not duplicate the complete data language.

---

153. "grammar/networking/" Relationship

Remote model/data/service interactions use the networking subsystem.

AI grammar does not define networking protocols.

---

154. "grammar/security/" Relationship

AI security requirements use the security subsystem.

AI grammar does not grant permissions.

---

155. "grammar/interoperability/" Relationship

Framework/model interchange belongs to interoperability.

The core AI grammar remains framework-neutral.

---

156. "grammar/dialects/" Relationship

Vendor/framework-specific AI constructs must be explicit dialect extensions.

They must not silently contaminate core Zamani semantics.

---

157. Compiler Safety Contract

The Rust implementation of all AI semantic and compilation support must:

- compile with Rust 1.97/1.97.1;
- use Rust 2021;
- contain no "unsafe";
- avoid undefined behavior;
- use explicit error handling;
- preserve source spans;
- preserve semantic quantities without silent overflow;
- distinguish resource exhaustion from semantic invalidity.

---

158. No Host-Width Leakage

AI semantic quantities must not silently become:

usize
u32
u64

merely because the host implementation uses those types.

Where arbitrary or symbolic quantities are required, the implementation must use an appropriate representation.

Host indexing may use "usize" internally where safe and appropriate, but this must not change language semantics.

---

159. No Hardware Leakage

The following must never become implicit AI semantics:

host pointer width
CPU word width
GPU warp width
SIMD width
cache size
VRAM size
device count
core count
thread count
node count
network topology

---

160. No Framework Leakage

The following must never silently become core semantic assumptions:

specific tensor library
specific neural framework
specific model format
specific accelerator SDK
specific runtime
specific vendor compiler

---

161. No Parser-Time Execution

AI parsing must never:

- load models;
- open datasets;
- query devices;
- inspect GPUs;
- access networks;
- execute model code;
- execute training;
- execute inference;
- allocate accelerator memory.

Parsing produces syntax.

---

162. No Semantic Actions in Grammar

AI grammar should not use embedded Rust actions to implement semantics.

No:

@members
@parser::members
embedded execution
unsafe parser actions

for AI semantics.

Semantic analysis belongs in the Rust frontend/compiler architecture.

---

163. No Semantic Predicates for AI Policy

AI semantic rules must not be hidden in parser predicates merely to force parser acceptance/rejection.

For example:

GPU count > X
tensor size < Y

must never be parser predicates.

Such conditions belong to semantic/resource analysis.

---

164. AI Grammar Modularity

The existing AI subdirectory should remain modular.

Each file has one coherent responsibility.

Do not create:

ai_everything.g4

containing models, tensors, training, inference, agents, hardware, and runtime semantics.

---

165. Stable Integration Boundary

The stable integration boundary is:

Zamani.g4
    ↓
AI construct
    ↓
AI semantic AST

The individual AI grammar files are implementation modules behind that boundary.

Future additions should normally add a specialized grammar component rather than modifying unrelated AI components.

---

166. Independent File Completion

When completing "grammar/ai/models.g4", it should be possible to determine completion without waiting for:

tensors.g4
training.g4
inference.g4
agents.g4

to be redesigned.

It must instead consume their already-defined public contracts.

The same rule applies to every AI grammar file.

---

167. AI File Contract Template

Every AI grammar file should document:

File
Purpose
Status
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Public Grammar Contract
AST Contract
Semantic Contract
Type Integration
Effect Integration
Resource Integration
Capability Integration
IR Integration
Compiler Integration
Runtime Integration
Tooling Integration
Cross-Domain Integration
Diagnostics
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Determinism Tests
Compatibility Tests
Security
Hard-Coding Audit
Completion Criteria

This is mandatory for production AI grammar development.

---

168. Required AI Conformance Matrix

The repository should maintain a feature matrix covering at least:

Feature| Spec| Grammar| AST| Semantic| IR| Compiler| Runtime| Tests
Model| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Tensor| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Dataset| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Training| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Inference| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Differentiation| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Pipeline| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Agent| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Accelerator intent| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Quantum integration| ✓| ✓| ✓| ✓| quantum::ir| ✓| ✓| ✓
Distributed AI| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Security| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓

A feature is not complete until every applicable column is satisfied.

---

169. Required Negative Architectural Tests

The repository must reject or flag architectural violations such as:

AI grammar creates an AI IR
AI grammar creates a second quantum IR
AI grammar defines physical qubit IDs
AI grammar defines GPU IDs
AI grammar defines fixed node counts
AI grammar defines fixed accelerator counts
AI grammar defines maximum tensor rank
AI grammar defines maximum parameter count
AI grammar duplicates type syntax
AI grammar duplicates expression syntax
AI grammar executes runtime behavior
AI grammar accesses hardware
AI grammar performs network I/O
AI grammar contains unsafe Rust
AI grammar depends on vendor implementation

---

170. Production Readiness Criteria

"grammar/spec/ai.md" and the AI subsystem are production-ready only when:

Specification

- AI semantics are completely specified.
- AI ownership is unambiguous.
- AI integration contracts exist.
- portability is defined.
- scalability is defined.

Grammar

- all stable AI syntax is represented;
- grammar is modular;
- no duplicate grammar authority exists;
- no artificial limits exist;
- ambiguity is controlled.

AST

- every AI construct has a domain-neutral AST representation;
- source spans are preserved;
- AST does not contain target decisions.

Semantic Analysis

- model validation exists;
- tensor validation exists;
- shape validation exists;
- resource/capability analysis exists;
- effect analysis exists;
- portability analysis exists.

IR

- every stable AI construct has an IR path;
- quantum constructs reach "quantum::ir";
- no duplicate IR exists.

Compiler

- optimization is semantics-preserving;
- specialization is target-aware but source-portable;
- differentiation is implemented;
- tensor/model lowering is implemented.

Runtime

- execution does not parse AI source directly;
- resources are discovered dynamically;
- placement is downstream;
- scaling is resource-driven.

Testing

- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- compatibility tests exist;
- deterministic behavior is tested;
- hard-coding detection exists.

Safety

- Rust 1.97/1.97.1 is supported;
- Rust 2021 is used;
- no "unsafe";
- no parser-side execution;
- no filesystem/network/hardware access from grammar.

---

171. Final AI Architecture

The final AI subsystem is:

                         Zamani Source
                              │
                              ▼
                       grammar/Zamani.g4
                              │
                              ▼
                           Parser
                              │
                              ▼
                    Domain-Neutral AST
                              │
                 ┌────────────┴────────────┐
                 │                         │
                 ▼                         ▼
             AI syntax              Universal syntax
                 │                         │
                 └────────────┬────────────┘
                              ▼
                     Semantic Analysis
                              │
       ┌──────────────────────┼─────────────────────────┐
       │                      │                         │
       ▼                      ▼                         ▼
     Types                 Effects                  Resources
       │                      │                         │
       └──────────────────────┼─────────────────────────┘
                              ▼
                      AI Semantic Model
                              │
          ┌───────────────────┼────────────────────┐
          │                   │                    │
          ▼                   ▼                    ▼
     Classical            Quantum              Hardware
        IR               quantum::ir              IR
          │                   │                    │
          └───────────────────┼────────────────────┘
                              ▼
                       Optimization
                              │
                 ┌────────────┼─────────────┐
                 │            │             │
                 ▼            ▼             ▼
             Parallel      Distributed   Differentiation
                 │            │             │
                 └────────────┼─────────────┘
                              ▼
                     Routing / Scheduling
                              │
                              ▼
                    Resilience / ZQN
                              │
                              ▼
                             HAL
                              │
                              ▼
                     Target Realization
                              │
          ┌──────────┬────────┼────────┬──────────┐
          ▼          ▼        ▼        ▼          ▼
         CPU        GPU      FPGA     QPU      Future
          │          │        │        │       targets
          └──────────┴────────┴────────┴──────────┘
                              │
                              ▼
                           Runtime

The central invariant is:

«Zamani AI describes computation, data, models, mathematical relationships, capabilities, resources, constraints and intent. It does not describe today's machine.»

Therefore:

Model
Tensor
Dataset
Training
Inference
Agent
Pipeline
Differentiation
Accelerator intent
Quantum-AI integration
Distributed AI
Hardware/software co-design

can all scale without changing the language merely because the target machine becomes larger, smaller, different, heterogeneous, distributed, quantum-enabled, accelerator-heavy, or otherwise future hardware.

---

172. Final Non-Negotiable Rules

1. "grammar/spec/ai.md" is the normative AI semantic contract.
2. Existing filenames are retained unless there is a demonstrated technical reason to change them.
3. "grammar/Zamani.g4" remains the canonical root grammar.
4. AI grammar remains modular under "grammar/ai/".
5. AI does not become a separate programming language.
6. AI does not create a second AST architecture.
7. AI does not create a second universal IR.
8. AI does not create a second quantum IR.
9. "quantum::ir" remains the canonical quantum semantic boundary.
10. AI does not own hardware realization.
11. AI does not own resource discovery.
12. AI does not own scheduling.
13. AI does not own routing.
14. AI does not own QEC.
15. AI does not own ZQN.
16. AI does not own HAL.
17. AI does not execute during parsing.
18. AI grammar does not access files, networks, processes or hardware.
19. AI grammar contains no artificial machine limits.
20. AI semantic quantities must not silently overflow host-width integers.
21. AI does not hard-code CPU/GPU/TPU/NPU/FPGA/QPU counts.
22. AI does not hard-code tensor rank.
23. AI does not hard-code model depth.
24. AI does not hard-code parameter count.
25. AI does not hard-code dataset size.
26. AI does not hard-code cluster size.
27. AI does not hard-code network topology.
28. AI framework-specific features belong to interoperability/dialects.
29. AI vendor-specific features must be explicitly target-specific.
30. AI capabilities are not device identifiers.
31. AI requirements are not physical placement decisions.
32. AI preferences are not requirements.
33. AI resource declarations do not allocate resources.
34. AI annotations do not automatically grant permissions.
35. AI agents do not bypass security.
36. AI model syntax does not enumerate every known model architecture.
37. AI tensor syntax does not enumerate every possible tensor rank.
38. AI operations use generic semantic operation mechanisms wherever possible.
39. AI libraries provide algorithms that do not need to become grammar keywords.
40. AI syntax must compose with classical, quantum, HDL, hardware, distributed, networking, data and security domains.
41. Every stable AI construct requires an AST contract.
42. Every stable AI construct requires a semantic contract.
43. Every stable AI construct requires an IR integration contract.
44. Every stable AI construct requires compiler/runtime integration.
45. Every stable AI construct requires positive tests.
46. Every stable AI construct requires negative tests.
47. Every stable AI construct requires boundary tests.
48. Every stable AI construct requires scalability tests.
49. Every stable AI construct requires compatibility tests.
50. Every AI grammar file must be independently completable according to its integration contract.
51. Rust 1.97/1.97.1 is the implementation baseline.
52. Rust 2021 is required.
53. "unsafe" Rust is prohibited.
54. Parser grammar must remain free of runtime execution logic.
55. POCO-REAF remains the governing portability objective.

---

173. Definition of Done

This specification is considered correctly integrated when:

grammar/spec/ai.md
        │
        ├── grammar/ai/ai.g4
        ├── grammar/ai/models.g4
        ├── grammar/ai/tensors.g4
        ├── grammar/ai/datasets.g4
        ├── grammar/ai/training.g4
        ├── grammar/ai/inference.g4
        ├── grammar/ai/agents.g4
        ├── grammar/ai/pipelines.g4
        ├── grammar/ai/differentiation.g4
        └── grammar/ai/ai-accelerators.g4
                    │
                    ▼
             grammar/Zamani.g4
                    │
                    ▼
              Lexer / Parser
                    │
                    ▼
          src/frontend/ast/
                    │
                    ▼
            Semantic Analysis
                    │
                    ▼
             Canonical IR
                    │
          ┌─────────┼──────────┐
          ▼         ▼          ▼
      Classical  quantum::ir  HDL/Hardware
          │         │          │
          └─────────┼──────────┘
                    ▼
              Compiler
                    │
                    ▼
               Runtime

has a traceable, tested contract for every stable AI feature.

At that point, adding a future AI architecture, tensor operation, model family, accelerator, framework interoperability layer, quantum-AI technique, distributed execution strategy, or hardware target should normally require an extension of a semantic/library/dialect contract rather than a redesign of the universal Zamani grammar.

That is the required architectural property for Zamani AI to remain compatible with POCO-REAF while scaling from the smallest computation to arbitrarily large computations subject to actual resources and capabilities.