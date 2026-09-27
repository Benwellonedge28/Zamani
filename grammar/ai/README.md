Zamani AI Grammar

Path: "grammar/ai/README.md"
Domain: Artificial intelligence, machine learning, differentiable computation, models, tensors, datasets, training, inference, agents, AI pipelines, and AI-oriented execution intent
Language: Zamani
Grammar technology: ANTLR4
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Rust safety policy: Safe Rust only; "unsafe" Rust is prohibited
Primary portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)
Scalability objective: From the smallest supported computation to arbitrarily large computation subject only to program semantics, representational constraints explicitly defined by the language, and actual available resources/capabilities
Status: Canonical AI grammar subsystem contract

---

1. Purpose

"grammar/ai/" defines the source-language syntax required for Zamani programs to express AI and machine-learning computation while remaining part of one universal Zamani language.

The subsystem covers syntax for:

- models;
- model interfaces;
- model parameters;
- tensors;
- symbolic and dynamic shapes;
- datasets;
- data/model pipelines;
- training;
- inference;
- evaluation;
- differentiation;
- optimization intent;
- agents;
- AI workflows;
- AI resource requirements;
- AI capability requirements;
- AI execution constraints;
- AI preferences;
- AI hints;
- AI/classical integration;
- AI/quantum integration;
- AI/HDL integration;
- AI/hardware integration;
- AI/distributed integration;
- AI/security integration;
- AI interoperability;
- AI dialect extensions.

The fundamental architectural rule is:

«AI grammar describes AI computation and source-level intent. It does not describe today's hardware.»

AI syntax must therefore remain independent of:

- CPU model;
- CPU count;
- core count;
- thread count;
- GPU model;
- GPU count;
- TPU/NPU count;
- FPGA capacity;
- ASIC implementation;
- QPU model;
- QPU count;
- physical qubit numbering;
- memory capacity;
- VRAM capacity;
- register width;
- SIMD width;
- tensor-core width;
- cluster size;
- node count;
- network topology;
- cloud provider;
- runtime implementation;
- AI framework;
- vendor SDK;
- model-serving provider.

Those concerns are resolved downstream.

---

2. POCO-REAF

AI is a domain of the universal Zamani language.

The intended compilation architecture is:

Zamani Source
      |
      v
Canonical Lexer
      |
      v
Canonical Parser
      |
      v
AI Grammar
      |
      v
Domain-Neutral Frontend AST
      |
      v
Semantic Analysis
      |
      +---- Types
      +---- Effects
      +---- Resources
      +---- Capabilities
      +---- Ownership
      +---- Security
      +---- Differentiability
      +---- Shape/quantity validation
      +---- Portability
      |
      v
Canonical Semantic Representation
      |
      +---- Classical computation
      +---- Tensor/data computation
      +---- quantum::ir
      +---- HDL/hardware representation
      +---- Distributed execution representation
      |
      v
Optimization / Lowering
      |
      +---- Specialization
      +---- Parallelization
      +---- Distribution
      +---- Differentiation
      +---- Accelerator lowering
      |
      v
Routing / Placement / Scheduling
      |
      v
Resilience / QEC / ZQN where applicable
      |
      v
HAL / Target abstraction
      |
      v
Target realization
      |
      +---- CPU
      +---- GPU
      +---- FPGA
      +---- ASIC
      +---- QPU
      +---- accelerator
      +---- distributed system
      +---- future target
      |
      v
Runtime

The AI grammar must not bypass this architecture.

In particular:

grammar -> runtime
grammar -> hardware
grammar -> device discovery
grammar -> physical placement

are prohibited as direct dependencies.

The correct relationship is:

grammar
   ↓
AST
   ↓
semantic analysis
   ↓
canonical representation / IR
   ↓
compiler
   ↓
runtime / target

---

3. Relationship to the Existing Repository

The AI grammar must integrate with the existing repository rather than create an independent AI compiler architecture.

The principal integration boundaries are:

grammar/Zamani.g4
grammar/antlr/ZamaniParser.g4
grammar/antlr/ZamaniLexer.g4

grammar/core/
grammar/types/
grammar/expressions/
grammar/declarations/
grammar/statements/
grammar/functions/
grammar/modules/
grammar/effects/
grammar/memory/
grammar/concurrency/

grammar/classical/
grammar/quantum/
grammar/hybrid/
grammar/hdl/
grammar/hardware/
grammar/resources/
grammar/distributed/
grammar/data/
grammar/networking/
grammar/security/
grammar/compile/
grammar/execution/
grammar/interoperability/
grammar/dialects/

src/lexer.rs
src/parser.rs
src/frontend/ast/

semantic analysis
canonical IR
quantum::ir
compiler
runtime

AI does not replace any of these systems.

AI extends them.

---

4. Authority

The AI subsystem follows the repository-wide authority model.

4.1 "grammar/DESIGN.md"

Owns:

- architecture;
- authority;
- integration;
- portability;
- scalability;
- dependency direction;
- production-readiness requirements.

4.2 "grammar/Zamani.g4"

Owns the canonical ANTLR root composition.

It remains the single public grammar entry point.

AI syntax must eventually be reachable through the canonical parser hierarchy.

4.3 "grammar/antlr/ZamaniParser.g4"

Owns parser composition below "Zamani.g4".

AI grammar components integrate through this parser hierarchy.

4.4 "grammar/specification/"

Owns normative language specification.

AI syntax cannot become stable merely because it exists in an AI ".g4" file.

4.5 "grammar/spec/"

Owns machine-oriented semantic contracts.

AI must follow universal contracts for:

- types;
- semantics;
- resources;
- effects;
- portability;
- compatibility;
- diagnostics.

4.6 "grammar/grammar.md"

Describes implementation conformance.

It must distinguish at least:

SPECIFIED
IMPLEMENTED
PARTIALLY_IMPLEMENTED
PLANNED
DEPRECATED

4.7 "grammar/Zamani-Grammar.md"

Remains the historical/extended design reference.

Features appearing there are not automatically valid Zamani syntax.

AI features must follow the repository's promotion path:

Zamani-Grammar.md
        ↓
proposal
        ↓
semantic design
        ↓
AST contract
        ↓
canonical grammar
        ↓
implementation
        ↓
IR contract
        ↓
tests
        ↓
stable

---

5. AI Directory Ownership

The canonical AI grammar directory is:

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

Existing files must be retained unless an actual technical conflict requires otherwise.

No second AI grammar tree should be created.

No "ai-everything.g4" or equivalent monolithic replacement should be introduced.

---

6. File Ownership Matrix

File| Owns
"README.md"| AI architecture and integration contract
"ai.g4"| AI grammar composition
"models.g4"| Model declarations and interfaces
"tensors.g4"| Tensor syntax and shape expressions
"datasets.g4"| Dataset declarations and dataset structure
"training.g4"| Training syntax
"inference.g4"| Inference syntax
"agents.g4"| Agent syntax
"pipelines.g4"| AI/data/model pipeline syntax
"differentiation.g4"| Differentiation intent
"ai-accelerators.g4"| Accelerator requirements/preferences/capabilities

Each file has one primary responsibility.

Cross-domain semantics belong to their owning subsystem.

---

7. Universal AI File Completion Contract

Every AI grammar file must be independently completable.

Before an AI grammar file is considered complete, the following must already be known:

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
Expression Integration
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

A future change to another AI file must not require redesigning the already-completed file.

New functionality should normally be added by extending the appropriate semantic contract or by adding a new specialized component.

---

8. "ai.g4"

Purpose

"ai.g4" is the AI grammar composition facade.

It composes the specialized AI grammar components.

Owns

- AI-domain entry points;
- AI declaration dispatch;
- AI construct composition;
- AI grammar integration;
- AI-specific composition boundaries.

Does not own

- tensor internals;
- model internals;
- training internals;
- inference internals;
- agent internals;
- differentiation internals;
- accelerator implementation;
- runtime behavior.

Dependencies

It consumes established contracts from:

core
types
expressions
declarations
statements
functions
effects
resources
classical
data

and optional integration points from:

quantum
hardware
distributed
security

AST Contract

AI grammar must lower into the repository's domain-neutral frontend AST.

It must not require a second AI-specific AST hierarchy that duplicates universal AST concepts.

An AI node must preserve at minimum:

- source span;
- syntactic category;
- identifier/name;
- child expressions/declarations;
- attributes;
- modifiers;
- generic parameters where applicable;
- semantic arguments.

Completion Criterion

"ai.g4" is complete when adding a new AI model family, tensor operation, training algorithm, accelerator, deployment backend, or framework integration does not require restructuring its fundamental composition model.

---

9. "models.g4"

Purpose

Defines model-level source syntax.

A model is a description of computational structure and interface.

Owns

- model declarations;
- model names;
- model parameters;
- model inputs;
- model outputs;
- model interfaces;
- model composition;
- model attributes;
- generic model parameters;
- model contracts.

Does not own

- tensor implementation;
- numerical kernels;
- optimizer algorithms;
- accelerator selection;
- storage;
- serialization implementation;
- model serving infrastructure.

Model Representation

A model may contain:

inputs
outputs
parameters
state
operations
submodels
constraints
capabilities
effects
resources

The grammar must not enumerate every possible AI architecture.

Do not make the grammar a closed list of:

Transformer
CNN
RNN
GAN
VAE
...

A model architecture should normally be expressible through generic model composition.

Framework-specific architecture names belong in libraries or explicit dialects where necessary.

Type Integration

Model types use the canonical type system.

For example, model interfaces may reference:

Tensor<T, Shape>
Stream<T>
Dataset<T>
Function<...>

using the universal type contracts.

AST Contract

The AST must preserve:

model name
parameters
inputs
outputs
body/composition
attributes
source spans

It must not contain:

GPU id
CPU id
device id
physical memory address
vendor runtime handle

Scalability

The grammar must not impose maximum:

- model depth;
- model parameter count;
- input count;
- output count;
- submodel count;
- layer count.

---

10. "tensors.g4"

Purpose

Defines tensor-oriented source syntax.

Tensor syntax is central to AI but must remain compatible with the universal Zamani type and expression systems.

Owns

- tensor declarations;
- tensor shape expressions;
- symbolic dimensions;
- dynamic dimensions;
- tensor indexing;
- tensor slicing;
- tensor transformations;
- tensor metadata;
- tensor-oriented syntax.

Does not own

- memory allocation;
- device memory;
- GPU memory;
- accelerator memory;
- physical tensor layout;
- kernel implementation;
- SIMD width;
- hardware tensor-core width.

Shape Model

Tensor shapes must support, where permitted by the type/semantic system:

constant dimensions
symbolic dimensions
runtime dimensions
dependent dimensions
derived dimensions
dynamic dimensions

Example:

Tensor<Float, [batch, sequence, features]>

must not require the compiler to know the numerical values of all dimensions during parsing.

Critical Rule

There must be no grammar-level:

MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_TENSOR_SIZE
MAX_ELEMENTS
MAX_PARAMETERS

or equivalent artificial language limit.

Host-Width Independence

A semantic tensor dimension must not silently become limited to a host implementation type merely because the implementation happens to use:

usize
u32
u64

internally.

Host indexing may use an appropriate safe representation internally, but that implementation detail must not redefine Zamani semantics.

Tensor Operations

Prefer generic semantic operations over a permanently expanding keyword list.

For example:

reshape
transpose
slice
broadcast
reduce
map
contract

should be represented through generic operation semantics where appropriate.

New mathematical/tensor algorithms should normally be library/intrinsic operations rather than new language keywords.

Quantum Boundary

A tensor representing mathematical data is not automatically a quantum state.

Quantum semantics remain owned by:

grammar/quantum/
quantum::ir

AI tensor syntax must not create a second quantum-state representation.

---

11. "datasets.g4"

Purpose

Defines logical datasets and dataset-processing structures.

Owns

- dataset declarations;
- dataset schemas;
- features;
- labels;
- logical splits;
- metadata;
- dataset transformations;
- streaming dataset references.

Does not own

- filesystem implementation;
- database implementation;
- cloud storage;
- network transport;
- physical partitioning;
- serialization implementation.

Scalability

Dataset cardinality may be:

- static;
- dynamic;
- unknown until runtime;
- streaming;
- externally determined.

The grammar must not contain:

MAX_DATASET_SIZE
MAX_RECORDS
MAX_FEATURES
MAX_PARTITIONS

as universal language restrictions.

Data Integration

Where generic data syntax already exists under:

grammar/data/

AI must reuse it instead of duplicating it.

AI-specific syntax should exist only where AI semantics genuinely require it.

---

12. "training.g4"

Purpose

Defines source syntax expressing training intent.

Owns

- training declarations;
- model selection;
- training inputs;
- objective expressions;
- loss expressions;
- optimization intent;
- validation;
- evaluation hooks;
- checkpoint intent;
- training configuration.

Does not own

- optimizer implementation;
- automatic differentiation implementation;
- scheduler implementation;
- checkpoint storage;
- accelerator scheduling;
- distributed training runtime;
- hardware placement.

Training Quantities

The grammar must not impose universal maximums for:

epochs
steps
batch size
model size
parameter count
dataset size
device count
worker count

These are program values or resource/semantic constraints.

Requirement vs Preference

These are different concepts:

requires capability(...)
requires resource(...)
constrains ...
prefers ...
hint ...

A preference must not silently become a requirement.

Distributed Integration

Training may express distributed intent.

The distributed subsystem determines:

- partitioning;
- communication;
- placement;
- replication;
- scheduling;
- failure handling.

AI grammar must not duplicate distributed semantics.

---

13. "inference.g4"

Purpose

Defines model inference syntax.

Owns

- inference declarations;
- model invocation;
- inputs;
- outputs;
- serving intent;
- batching intent;
- streaming inference;
- inference constraints;
- inference preferences.

Does not own

- serving infrastructure;
- networking;
- deployment;
- load balancing;
- accelerator selection;
- runtime scheduling.

Scalability

Inference syntax must support:

single input
batch
stream
distributed requests
dynamic requests
heterogeneous execution

without embedding fixed capacities.

---

14. "agents.g4"

Purpose

Defines AI-agent source syntax.

Owns

- agent declarations;
- goals;
- observations;
- actions;
- tools;
- state;
- policies;
- workflows;
- agent relationships;
- agent capabilities as declarations/requirements.

Does not own

- authorization;
- model execution;
- networking;
- tool implementation;
- runtime scheduling;
- distributed placement.

Security Boundary

An agent declaring:

requires capability("...")

does not grant itself that capability.

Capability authorization belongs to:

security
effects
resource/capability analysis
runtime policy

Scalability

No universal maximum may be imposed on:

- agents;
- tools;
- actions;
- observations;
- states;
- interactions;
- workflows.

---

15. "pipelines.g4"

Purpose

Defines composition of AI/data/model computation.

Owns

- pipeline declarations;
- stages;
- stage dependencies;
- data flow;
- model composition;
- pipeline parameters;
- pipeline-level constraints;
- pipeline-level requirements.

Does not own

- scheduler implementation;
- graph execution engine;
- placement;
- distributed transport;
- accelerator allocation.

Cross-Domain Stages

A pipeline may compose:

classical
AI
quantum
HDL/hardware intent
data
networking
distributed
security

provided semantic contracts permit the composition.

For example:

data
  ↓
classical preprocessing
  ↓
AI model
  ↓
quantum computation
  ↓
measurement
  ↓
classical postprocessing

is a single Zamani program.

AI grammar must not create a separate language boundary between those stages.

---

16. "differentiation.g4"

Purpose

Defines source-level differentiation intent.

Owns

- differentiation requests;
- gradient requests;
- Jacobian requests;
- Hessian requests;
- derivative expressions;
- differentiability annotations;
- differentiation boundaries.

Does not own

- automatic differentiation algorithms;
- symbolic differentiation engine;
- forward-mode implementation;
- reverse-mode implementation;
- numerical differentiation;
- accelerator kernels.

The source expresses:

what should be differentiated

The compiler determines:

how differentiation is implemented

Quantum Integration

Where differentiation crosses a quantum computation boundary, the construct must lower through the established quantum semantic pipeline.

AI must not create:

ai::quantum::ir

or another competing quantum IR.

The canonical path remains:

AI source
   ↓
domain-neutral AST
   ↓
semantic analysis
   ↓
quantum::ir
   ↓
quantum optimization/routing/scheduling/resilience

where quantum semantics are actually involved.

---

17. "ai-accelerators.g4"

Purpose

Defines accelerator intent.

This file is one of the most important portability boundaries in the AI grammar.

Owns

- generic accelerator requirements;
- capability requirements;
- accelerator preferences;
- accelerator constraints;
- accelerator hints;
- portable accelerator intent.

Does not own

- device discovery;
- device enumeration;
- device identifiers;
- vendor APIs;
- driver APIs;
- physical topology;
- scheduling;
- memory allocation;
- kernel execution.

Correct Model

Prefer semantic requirements such as:

requires capability("tensor.compute")

or:

requires capability("ai.acceleration")

rather than source-level physical device selection.

A requirement is not a placement decision.

Incorrect Universal Model

Avoid making core AI syntax depend on constructs such as:

use_gpu(0)
use_device(3)
use_gpu("specific-vendor-device")

when those identify physical resources.

Target-specific mechanisms belong in explicit target/dialect/deployment layers.

---

18. Universal Resource Model

AI must reuse the repository-wide resource model.

The following are distinct:

Concept| Meaning
Requirement| Necessary semantic/resource property
Capability| Ability required from a target
Constraint| Property that must be respected
Preference| Desired property
Hint| Information that may improve realization
Budget| Resource/performance boundary
Realization| Concrete downstream mapping

For example:

requires capability("tensor.compute")

does not mean:

use GPU #0

Likewise:

prefer accelerator

does not mean:

accelerator is mandatory

And:

requires memory >= required_memory

does not mean:

RAM must be exactly a particular capacity

---

19. No Artificial Hardware Limits

The AI grammar must never define universal limits such as:

MAX_GPUS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_ACCELERATORS
MAX_TENSORS
MAX_TENSOR_RANK
MAX_TENSOR_ELEMENTS
MAX_LAYERS
MAX_PARAMETERS
MAX_DATASET_SIZE
MAX_AGENTS
MAX_NODES
MAX_DEVICES
MAX_MEMORY
MAX_VRAM
MAX_REGISTER_WIDTH

The same prohibition applies under different names.

The language must not encode current hardware capabilities as permanent language capabilities.

---

20. Program Values Are Not Language Limits

The prohibition against hard-coding does not prohibit ordinary program constants.

This is valid:

let batch_size = 1024;

This can also be valid:

Tensor<Float, [1024, 1024]>

because these values are program semantics.

The prohibited architecture is:

MAX_TENSOR_RANK = 8

when "8" is being used as an artificial compiler-wide limit.

The distinction is:

program value
    ≠
language implementation ceiling

---

21. Generic Operations Instead of Keyword Explosion

AI must not become a dictionary of every known algorithm.

Do not continuously add keywords for every:

- model;
- optimizer;
- activation;
- loss;
- tensor operation;
- architecture;
- framework;
- vendor API;
- accelerator.

Prefer:

generic syntax
+
types
+
expressions
+
semantic operations
+
libraries/intrinsics
+
capabilities
+
dialects where genuinely necessary

This permits future AI algorithms without redesigning the core grammar.

---

22. Framework Neutrality

Core Zamani AI syntax must not depend on:

- PyTorch;
- TensorFlow;
- JAX;
- ONNX;
- CUDA;
- ROCm;
- oneDNN;
- vendor-specific accelerator APIs;
- any single model-serving runtime;
- any single model format.

Framework interoperability belongs under:

grammar/interoperability/

Framework-specific language extensions belong under:

grammar/dialects/

Core AI semantics remain framework-neutral.

---

23. Vendor Neutrality

Vendor-specific functionality must never silently become universal Zamani syntax.

If a feature genuinely depends on a vendor or target, that dependency must be explicit.

The distinction is:

portable capability

versus:

vendor-specific capability

A portable program must not accidentally become tied to a vendor merely because the compiler selected that vendor during lowering.

---

24. AI + Classical Integration

AI frequently lowers into classical computation.

AI must therefore reuse:

grammar/classical/
grammar/types/
grammar/expressions/
grammar/functions/

for:

- arithmetic;
- numerical values;
- vectors;
- matrices;
- tensors;
- symbolic computation;
- control flow;
- functions.

AI must not create duplicate definitions of:

integer
float
vector
matrix
function
expression

unless a genuinely new semantic type is required and formally specified.

---

25. AI + Quantum Integration

AI may interact with quantum computation.

Examples include:

AI
 ↓
quantum operation
 ↓
measurement
 ↓
AI/classical processing

or:

AI model
 ↓
quantum-assisted computation
 ↓
classical result

The AI grammar must not redefine quantum semantics.

Quantum constructs belong to:

grammar/quantum/

and their canonical semantic boundary remains:

quantum::ir

The AI layer may reference quantum constructs through the universal grammar composition model.

It must never introduce a competing quantum gate enumeration or quantum IR.

---

26. AI + HDL Integration

AI may express hardware/software co-design intent.

For example, an AI workload may express that it benefits from:

tensor.compute
parallel.compute
low_latency
high_bandwidth
accelerated.memory

The hardware subsystem determines how those requirements are realized.

AI must not redefine:

- ports;
- signals;
- wires;
- clocks;
- registers;
- HDL timing;
- hardware modules.

Those belong to:

grammar/hdl/
grammar/hardware/

---

27. AI + Distributed Integration

AI programs may scale across:

one resource
multiple resources
multiple processes
multiple machines
clusters
heterogeneous systems
federated systems
cloud systems
future distributed architectures

AI grammar expresses distributed intent.

The distributed subsystem determines:

- partitioning;
- placement;
- communication;
- replication;
- consistency;
- scheduling;
- recovery;
- topology;
- deployment.

No fixed number of workers/nodes/devices may be encoded in the AI grammar.

---

28. AI + Data Integration

AI depends heavily on data.

The AI grammar must reuse:

grammar/data/

for universal concepts such as:

- records;
- schemas;
- collections;
- streams;
- transformations;
- serialization.

AI-specific dataset syntax should exist only when required for AI semantics.

This avoids two competing data languages.

---

29. AI + Networking Integration

AI may involve:

- remote inference;
- distributed training;
- data streaming;
- service interaction;
- model distribution.

The AI grammar does not define networking protocols.

Networking belongs to:

grammar/networking/

AI may express semantic requirements or references to networked resources through established interfaces.

---

30. AI + Security Integration

AI programs may require:

- protected datasets;
- identity;
- authorization;
- secrets;
- provenance;
- secure execution;
- privacy-preserving computation.

AI grammar does not grant access.

Security remains owned by:

grammar/security/
grammar/effects/

An AI construct that requests access must be validated by semantic/security analysis.

---

31. Effects

AI operations may have effects including:

data access
network access
randomness
model loading
model persistence
checkpointing
accelerator interaction
distributed communication
external service interaction

These must integrate with the universal effects system.

AI must not create a second effect system.

Parsing must never execute those effects.

---

32. Parser Purity

The AI grammar must remain declarative.

It must not:

- load a model;
- inspect a dataset;
- query a GPU;
- query a QPU;
- inspect hardware;
- access the filesystem;
- access the network;
- execute inference;
- execute training;
- allocate device memory;
- discover resources.

The parser produces syntax.

Semantic analysis interprets syntax.

Compilation realizes semantics.

Runtime executes compiled behavior.

---

33. No Embedded Rust Semantics

AI ".g4" files must not contain embedded Rust implementation logic such as:

@members
@parser::members

for AI semantics.

Do not put:

- runtime execution;
- resource discovery;
- hardware selection;
- model loading;
- network I/O;
- semantic state mutation

inside the grammar.

Rust implementation remains outside the grammar.

---

34. AST Contract

AI syntax must integrate into:

src/frontend/ast/

The frontend AST is domain-neutral.

AI grammar must not force the frontend AST to become:

AiTensorNode
AiGpuNode
AiCudaNode
AiQuantumGpuNode

merely because a source construct happens to belong to AI.

Where an AI-specific semantic distinction is genuinely necessary, the AST representation must be specified before grammar stabilization.

Every AI AST construct must have:

- source span;
- stable node identity/category;
- children;
- names;
- attributes;
- generic parameters;
- expressions;
- declarations;
- semantic payload where necessary.

The AST must not contain physical target decisions.

---

35. Semantic Contract

Semantic analysis is responsible for validating:

- names;
- types;
- shapes;
- generic parameters;
- dimensions;
- differentiability;
- effects;
- capabilities;
- resource requirements;
- constraints;
- security;
- ownership;
- portability;
- domain compatibility;
- target-independent correctness.

The grammar is not responsible for these decisions.

---

36. IR Contract

AI grammar does not create an independent universal AI IR.

AI constructs lower into the repository's canonical semantic/IR architecture.

Depending on the computation, AI may lower toward:

classical representation
tensor/data representation
quantum::ir
HDL/hardware representation
distributed representation

The choice is semantic and compiler-owned.

There must not be:

grammar/ai -> ai::ir

merely because AI is a grammar domain.

If an AI-specific intermediate representation is ever required, it must be defined outside "grammar/" and integrated into the canonical compiler architecture.

---

37. Differentiation Integration

Differentiation is a semantic transformation.

The source may express:

differentiate f
gradient loss
jacobian model
hessian objective

but the compiler determines the implementation strategy.

Possible implementations may include:

- symbolic differentiation;
- automatic differentiation;
- forward mode;
- reverse mode;
- mixed mode;
- numerical methods;
- specialized accelerator lowering;
- quantum-compatible differentiation.

The grammar does not choose the implementation merely by parsing the construct.

---

38. Resource Discovery

Resource discovery is never performed by AI grammar.

The compiler/runtime may discover:

available memory
available accelerators
available CPU resources
available GPU resources
available QPU resources
available network resources
available distributed resources

and compare them with program requirements.

This is the correct direction:

program requirement
       ↓
semantic analysis
       ↓
resource capability query
       ↓
target selection

not:

parser
  ↓
inspect current machine
  ↓
change language meaning

---

39. Resource Failure Semantics

A valid AI program must not become syntactically invalid merely because the current target lacks sufficient resources.

Distinguish:

syntax error
semantic error
unsupported language feature
missing capability
insufficient resources
incompatible target
runtime failure

For example:

requires capability("tensor.compute")

may be semantically valid even when the currently selected target does not provide that capability.

The target compatibility layer determines whether execution is possible.

---

40. Scaling Model

The AI grammar is intentionally open-ended with respect to:

- tensor rank;
- tensor dimensions;
- model size;
- parameter count;
- dataset cardinality;
- pipeline stages;
- agents;
- tools;
- distributed workers;
- devices;
- accelerator count;
- memory;
- execution resources.

"Infinity" here means no artificial grammar ceiling.

Actual execution remains bounded by:

program semantics
compiler representation
available memory
available compute
available devices
target capabilities
runtime constraints
explicit resource policies

Those are not equivalent to a hard-coded language maximum.

---

41. Determinism

Parsing the same AI source must produce the same syntactic result when given the same:

- source;
- language version;
- grammar version;
- explicitly selected dialect configuration.

Parsing must not depend on:

- current hardware;
- current GPU;
- current QPU;
- current memory;
- wall-clock time;
- randomness;
- network state;
- filesystem state;
- runtime state.

---

42. Diagnostics

AI grammar diagnostics must preserve source locations.

Errors should identify:

- source file;
- line;
- column;
- relevant construct;
- expected syntax;
- actual syntax.

Semantic diagnostics must be distinguished from parser diagnostics.

For example:

syntax:
unexpected token

semantic:
tensor shape mismatch

resource:
required capability unavailable

portability:
target-specific requirement prevents selected target

security:
required capability is not authorized

These are different failure classes.

---

43. Compatibility

AI grammar changes must participate in the repository-wide compatibility system.

Every stable AI feature must have:

language version
grammar status
implementation status
AST status
semantic status
IR status
compiler status
runtime status
test status
deprecation policy
migration policy where applicable

A grammar construct must not silently change meaning between language versions.

---

44. Dialects

AI dialects are permitted.

They must be explicit.

A dialect must identify:

name
version
purpose
owner
syntax extensions
semantic extensions
AST mapping
IR mapping
capabilities
compatibility
portability

Vendor/framework-specific AI functionality belongs here when it cannot be represented through portable core semantics.

Dialects must not silently redefine core Zamani meaning.

---

45. Interoperability

AI interoperability belongs under:

grammar/interoperability/

Possible external formats include model/data representations and execution ecosystems.

Interoperability formats are boundaries, not alternate definitions of Zamani semantics.

The canonical direction is:

Zamani semantics
      ↓
interoperability lowering/import/export
      ↓
external format

not:

external framework
      ↓
becomes Zamani's semantic authority

---

46. Hard-Coding Audit

Every AI grammar change must be audited for accidental hard-coding.

The audit must search for concepts equivalent to:

MAX_GPU
MAX_CPU
MAX_THREAD
MAX_NODE
MAX_DEVICE
MAX_TENSOR
MAX_RANK
MAX_LAYER
MAX_PARAMETER
MAX_DATASET
MAX_MEMORY
MAX_VRAM
MAX_ACCELERATOR
MAX_AGENT
MAX_MODEL_SIZE

It must also detect physical enumeration patterns such as:

gpu0
gpu1
qpu0
qpu1
device0
device1
node0
node1

when they are being used as universal language concepts.

Explicit program values are permitted.

Universal implementation ceilings are not.

---

47. No Host-Architecture Leakage

AI semantic meaning must not depend on:

- host pointer width;
- CPU word size;
- GPU warp size;
- SIMD width;
- register width;
- cache size;
- memory size;
- device count.

The compiler may specialize to those properties.

The source language must not silently acquire those limitations.

---

48. Rust Safety Contract

AI-related Rust implementation must follow:

Rust 1.97 / Rust 1.97.1
Rust 2021
safe Rust
no unsafe

The grammar subsystem must not require:

unsafe

for parsing, AST construction, validation, or AI semantic integration.

Implementations must use explicit error handling.

Resource exhaustion must not be confused with semantic invalidity.

Large quantities must not silently overflow merely because the host implementation uses a narrower integer representation.

---

49. Testing Requirements

Every AI grammar feature requires at least:

positive tests
negative tests
boundary tests
scalability tests
determinism tests
compatibility tests
integration tests

Positive examples

Test:

- minimal model;
- tensor declaration;
- symbolic tensor;
- dataset;
- training;
- inference;
- agent;
- pipeline;
- differentiation;
- accelerator capability;
- classical/AI composition;
- quantum/AI composition;
- distributed AI;
- hardware/AI intent.

Negative examples

Test:

- malformed model;
- malformed tensor shape;
- malformed dataset;
- invalid generic syntax;
- invalid delimiters;
- malformed training declaration;
- malformed inference;
- invalid differentiation structure;
- malformed capability requirement.

Boundary examples

Test:

- empty collections where legal;
- one-dimensional tensor;
- higher-dimensional tensor;
- symbolic shape;
- dynamic shape;
- nested model;
- deeply composed pipeline;
- large generated source;
- large expression trees.

Scalability tests

Tests must demonstrate that grammar behavior does not depend on artificial limits.

Generated tests should vary:

model depth
tensor rank
dimension count
pipeline stages
agent tools
dataset structure
distributed stages

without using those tests to establish a maximum language capacity.

---

50. Architectural Negative Tests

The repository should explicitly detect violations such as:

AI grammar creates a second AST architecture
AI grammar creates a second AI universal IR
AI grammar creates a second quantum IR
AI grammar enumerates physical GPUs
AI grammar enumerates physical QPUs
AI grammar defines fixed tensor rank
AI grammar defines fixed model depth
AI grammar defines fixed parameter count
AI grammar defines fixed dataset size
AI grammar defines fixed node count
AI grammar performs hardware discovery
AI grammar performs network I/O
AI grammar loads models
AI grammar accesses datasets
AI grammar executes inference
AI grammar executes training
AI grammar grants security permissions
AI grammar contains unsafe Rust
AI grammar depends on a vendor implementation
AI grammar duplicates universal type syntax
AI grammar duplicates universal expression syntax

These should be treated as architectural regressions.

---

51. AI Feature Traceability

Every stable AI feature must be traceable:

Specification
     ↓
AI grammar file
     ↓
Lexer tokens if required
     ↓
Parser rule
     ↓
AST contract
     ↓
Semantic contract
     ↓
Resource/effect/capability contract
     ↓
IR mapping
     ↓
Compiler consumer
     ↓
Runtime consumer where required
     ↓
Positive tests
     ↓
Negative tests
     ↓
Boundary tests
     ↓
Scalability tests
     ↓
Compatibility tests

A syntax-only implementation is not a production-ready feature.

---

52. AI Conformance Matrix

The repository should maintain a machine-readable or generated matrix equivalent to:

Feature| Spec| Grammar| Lexer| AST| Semantic| IR| Compiler| Runtime| Tests
Models| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Tensors| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Datasets| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Training| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Inference| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Agents| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Pipelines| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Differentiation| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Accelerator intent| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Distributed AI| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Quantum-AI| ✓| ✓| ✓| ✓| ✓| "quantum::ir"| ✓| ✓| ✓
HDL/hardware AI| ✓| ✓| ✓| ✓| ✓| canonical hardware path| ✓| ✓| ✓
Security| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓

A feature is stable only when all applicable stages have a defined contract and implementation.

---

53. Independent Completion Rule

The following must be possible:

Completing "models.g4"

must not require redesigning:

tensors.g4
training.g4
inference.g4
agents.g4

because their public contracts are already established.

Completing "tensors.g4"

must not require modifying:

models.g4
training.g4
inference.g4

because tensor consumers use the already-defined tensor contract.

Completing "training.g4"

must not require modifying:

models.g4
tensors.g4

except for a separately justified contract evolution.

Completing "ai-accelerators.g4"

must not require modifying tensor/model syntax merely because a new accelerator appears.

This is the required property for maintainability.

---

54. Future Extension Rule

Adding a future AI technology should normally require:

new semantic/library capability
or
new dialect
or
new specialized grammar component

rather than:

rewrite Zamani.g4
rewrite the universal type system
rewrite expressions
rewrite quantum grammar
rewrite hardware grammar

Examples of future technologies that must be able to fit without redesigning the language include:

- new model architectures;
- new training algorithms;
- new differentiation techniques;
- new tensor representations;
- new accelerators;
- new AI hardware;
- new distributed training methods;
- new quantum-AI algorithms;
- new inference strategies;
- new model formats;
- new deployment environments.

---

55. AI Does Not Own the Runtime

Runtime responsibilities remain outside "grammar/ai/".

Runtime may determine:

- execution location;
- resource allocation;
- scheduling;
- batching;
- model loading;
- device selection;
- memory allocation;
- fault recovery;
- distributed execution;
- observability;
- checkpoint restoration.

The AI grammar only provides source-level information required to make those downstream decisions.

---

56. AI Does Not Own Scheduling

AI may express:

requires low_latency
prefers throughput
hint locality

but it must not implement scheduling.

Scheduling belongs downstream.

A source-level performance preference must never silently override correctness or semantic requirements.

---

57. AI Does Not Own Placement

AI source may express:

requires capability("tensor.compute")

but physical placement is determined later.

The source does not inherently select:

GPU 0
GPU 1
QPU 2
node 17
memory bank 4

unless the programmer is deliberately using an explicitly target-specific mechanism.

Such target-specific behavior must not become the portable core language.

---

58. AI Does Not Own QEC or ZQN

When AI computation invokes quantum computation:

AI
 ↓
quantum::ir
 ↓
optimization
 ↓
routing
 ↓
scheduling
 ↓
QEC / resilience
 ↓
ZQN
 ↓
HAL

AI grammar does not implement:

- QEC;
- noise models;
- calibration;
- physical routing;
- physical scheduling;
- ZQN.

This maintains the repository's canonical quantum architecture.

---

59. AI Does Not Own HDL Semantics

AI hardware acceleration may eventually produce hardware intent, but:

AI grammar
      ↓
semantic intent
      ↓
hardware/HDL lowering

is the correct direction.

AI must not duplicate:

wire
signal
register
clock
port
module
pipeline
timing

from "grammar/hdl/".

---

60. AI Does Not Own Data Storage

AI dataset syntax does not define:

filesystem
database
cloud bucket
object store
memory layout
network storage

Those belong to the appropriate data/storage/interoperability/runtime layers.

---

61. AI Does Not Own Security Authorization

An AI agent or model may request capabilities.

It cannot grant them.

The distinction is:

declared capability requirement
        ≠
authorized capability

This is essential for safe agent execution.

---

62. AI Semantic Quantity Rules

AI quantities such as:

- tensor dimensions;
- parameter counts;
- dataset cardinalities;
- model sizes;
- batch sizes;
- sequence lengths;
- iteration counts;

must use the repository's general quantity/type rules.

The grammar must not introduce a separate AI quantity system.

Where symbolic quantities are supported, they remain symbolic until the semantic phase that requires resolution.

---

63. Dynamic and Symbolic Shapes

AI tensor syntax should support shape information at different stages:

known at parse time
known at compile time
known through type-level computation
known at runtime
derived from data
symbolically constrained

The parser must not require every dimension to be a literal integer.

The semantic analyzer determines whether a given operation is valid under the available shape information.

---

64. Shape Compatibility

Tensor shape compatibility is a semantic concern.

The grammar establishes structural syntax.

For example, syntax may represent:

a + b

but semantic analysis determines whether the types and shapes make that operation valid.

Do not encode all possible shape arithmetic as parser alternatives.

---

65. Model Composition

Model composition must remain generic.

A model may contain:

submodels
functions
tensor operations
classical operations
quantum operations
data transformations
control flow

where permitted by semantic rules.

The grammar must not assume that a model is necessarily a neural network.

---

66. AI Agents and General Computation

Agents are programs, not magical runtime objects.

Agent syntax must integrate with:

functions
effects
security
data
networking
concurrency
distributed
AI models

An agent must remain subject to ordinary Zamani semantic rules.

---

67. AI and Concurrency

AI workloads may use:

async
await
parallel
pipeline
spawn
tasks
channels

from the universal concurrency system.

AI must not create separate versions of those constructs.

AI-specific parallelism should express semantic intent only where necessary.

---

68. AI and Memory

AI may require large or distributed memory.

Memory semantics remain owned by:

grammar/memory/

AI may express resource requirements through the universal resource model.

Do not encode:

VRAM = 24GB
RAM = 64GB

as language assumptions.

---

69. AI and Compilation

AI compilation may involve:

specialization
constant propagation
shape specialization
automatic differentiation
fusion
vectorization
parallelization
distribution
accelerator lowering
quantization
layout selection
kernel generation

These are compiler responsibilities.

The grammar must not attempt to implement them.

---

70. AI and Optimization

An AI source construct may express optimization intent.

The compiler determines whether and how to perform:

operator fusion
parallel execution
memory reuse
specialization
tiling
vectorization
distribution
accelerator mapping

Optimization must preserve program semantics.

---

71. AI and Interoperability

External AI ecosystems must remain boundaries.

The language may provide explicit interoperability constructs, but importing an external framework must not silently redefine Zamani semantics.

The canonical semantic direction remains:

Zamani
 ↓
semantic model
 ↓
interop adapter
 ↓
external representation

---

72. Production Safety Requirements

AI grammar production code must satisfy:

- Rust 2021 compatibility;
- Rust 1.97 compatibility;
- Rust 1.97.1 compatibility;
- safe Rust only;
- no "unsafe";
- no parser-time execution;
- no filesystem access;
- no network access;
- no hardware discovery;
- no environment-dependent parsing;
- deterministic parsing;
- explicit diagnostics;
- source-span preservation;
- no silent semantic overflow.

---

73. Required Repository Integration Tests

At minimum, the AI grammar must be tested through:

grammar/Zamani.g4
        ↓
ANTLR generation
        ↓
lexer
        ↓
parser
        ↓
src/frontend/ast/
        ↓
semantic analysis
        ↓
canonical IR

Representative integration programs should cover:

minimal.zm
classical.zm
ai.zm
quantum.zm
hybrid.zm
hdl.zm
distributed.zm
poco-reaf.zm

The AI-specific tests must also include combinations such as:

AI + classical
AI + quantum
AI + HDL
AI + hardware
AI + distributed
AI + data
AI + networking
AI + security
AI + concurrency
AI + resources

---

74. POCO-REAF Acceptance Tests

AI grammar is not considered production-ready merely because the same source parses.

The repository should verify that portable AI source semantics can be lowered without rewriting the source for different target classes.

Conceptually:

same Zamani source
       |
       +---- CPU
       +---- GPU
       +---- FPGA
       +---- ASIC
       +---- QPU where semantically applicable
       +---- distributed system
       +---- accelerator
       +---- future target

The target-specific compiler is responsible for realization.

The source language should not have to encode today's hardware topology.

---

75. Production Definition of Done

The AI grammar subsystem is production-ready only when all of the following hold.

Specification

- AI language semantics are specified.
- AI ownership is unambiguous.
- Cross-domain integration is specified.
- portability is specified.
- scalability is specified.

Grammar

- AI grammar is modular.
- "ai.g4" is the composition facade.
- no competing AI root grammar exists.
- generic operations are preferred over keyword explosion.
- no artificial hardware limits exist.
- no physical resource enumeration is required.
- parser ambiguity is controlled.

Lexer

- only genuinely lexical AI tokens are introduced.
- generic identifiers remain available.
- AI does not cause unnecessary keyword explosion.
- token ownership is centralized.

AST

- every stable AI construct has an AST contract.
- AST remains domain-neutral.
- source spans are preserved.
- no target-specific decisions are embedded.

Semantics

- types are validated.
- tensor shapes are validated.
- model interfaces are validated.
- effects are validated.
- capabilities are validated.
- resources are validated.
- differentiability is validated.
- security is validated.
- portability is validated.

IR

- every stable AI construct has a defined lowering path.
- no unnecessary AI IR is created.
- quantum computation reaches "quantum::ir".
- hardware computation reaches the canonical hardware path.

Compiler

- AI lowering exists.
- optimization preserves semantics.
- differentiation is implemented where advertised.
- target specialization is downstream.
- resource-aware compilation exists.

Runtime

- runtime does not parse AI source.
- resource discovery is dynamic.
- placement is downstream.
- execution is target-aware.
- source semantics remain target-independent.

Testing

- positive tests exist.
- negative tests exist.
- boundary tests exist.
- scalability tests exist.
- determinism tests exist.
- compatibility tests exist.
- cross-domain tests exist.
- hard-coding tests exist.

Safety

- Rust 1.97 / 1.97.1 is supported.
- Rust 2021 is used.
- no "unsafe".
- no parser-side execution.
- no hardware access from grammar.
- no network access from grammar.
- no filesystem access from grammar.

---

76. Final AI Architecture

The production architecture is:

                         Zamani Source
                              |
                              v
                       grammar/Zamani.g4
                              |
                              v
                      Zamani Parser
                              |
                              v
                       AI Grammar
                              |
              +---------------+---------------+
              |               |               |
              v               v               v
           Models         Tensors         Datasets
              |               |               |
              +---------------+---------------+
                              |
              +---------------+---------------+
              |               |               |
              v               v               v
          Training        Inference       Pipelines
              |               |               |
              +---------------+---------------+
                              |
              +---------------+---------------+
              |               |               |
              v               v               v
           Agents       Differentiation   Accelerator Intent
                              |
                              v
                    Domain-Neutral AST
                              |
                              v
                     Semantic Analysis
                              |
        +---------------------+----------------------+
        |            |          |          |         |
        v            v          v          v         v
      Types       Effects   Resources  Security  Capabilities
        |            |          |          |         |
        +------------+----------+----------+---------+
                              |
                              v
                     Canonical Semantics
                              |
        +---------------------+----------------------+
        |                     |                      |
        v                     v                      v
    Classical             quantum::ir          HDL/Hardware
        |                     |                      |
        +---------------------+----------------------+
                              |
                              v
                    Optimization / Lowering
                              |
        +---------------------+----------------------+
        |                     |                      |
        v                     v                      v
 Differentiation         Parallelization        Distribution
        |                     |                      |
        +---------------------+----------------------+
                              |
                              v
                     Routing / Scheduling
                              |
                              v
                    Resilience / ZQN
                              |
                              v
                             HAL
                              |
                              v
                     Target Realization
                              |
          +---------+---------+---------+---------+
          |         |         |         |         |
         CPU       GPU       FPGA      QPU     Future
          |         |         |         |      targets
          +---------+---------+---------+---------+
                              |
                              v
                           Runtime

---

77. Non-Negotiable AI Invariants

1. Zamani remains one language.
2. "grammar/Zamani.g4" remains the canonical root grammar.
3. "grammar/ai/" remains a modular domain, not a separate language.
4. "ai.g4" remains the AI composition facade.
5. AI grammar does not create a second universal AST.
6. AI grammar does not create a second universal IR.
7. AI grammar does not create a second quantum IR.
8. "quantum::ir" remains the canonical quantum semantic boundary.
9. AI grammar does not own hardware realization.
10. AI grammar does not own device discovery.
11. AI grammar does not own resource allocation.
12. AI grammar does not own scheduling.
13. AI grammar does not own routing.
14. AI grammar does not own QEC.
15. AI grammar does not own ZQN.
16. AI grammar does not own HAL.
17. AI grammar does not execute code.
18. AI grammar does not access the filesystem.
19. AI grammar does not access the network.
20. AI grammar does not access hardware.
21. AI grammar does not load models.
22. AI grammar does not execute training.
23. AI grammar does not execute inference.
24. AI grammar does not grant security permissions.
25. AI grammar does not hard-code machine capacities.
26. AI grammar does not hard-code tensor rank.
27. AI grammar does not hard-code model depth.
28. AI grammar does not hard-code parameter count.
29. AI grammar does not hard-code dataset size.
30. AI grammar does not hard-code device count.
31. AI grammar does not hard-code node count.
32. AI grammar does not hard-code accelerator count.
33. AI grammar does not hard-code memory capacity.
34. AI grammar does not hard-code register width.
35. AI grammar does not hard-code network topology.
36. AI grammar does not silently depend on a vendor.
37. AI grammar does not silently depend on a framework.
38. AI grammar reuses the universal type system.
39. AI grammar reuses the universal expression system.
40. AI grammar reuses the universal resource system.
41. AI grammar reuses the universal capability system.
42. AI grammar reuses the universal effects system.
43. AI grammar reuses the universal security system.
44. AI grammar reuses the universal concurrency system.
45. AI grammar reuses the universal distributed system.
46. AI grammar reuses the universal data system.
47. AI tensor quantities remain independent of host integer width.
48. Symbolic and dynamic dimensions are not parser-level errors merely because their values are unknown.
49. Requirements are distinct from preferences.
50. Capabilities are distinct from physical devices.
51. Preferences are distinct from requirements.
52. Hints are distinct from semantic requirements.
53. Resource requirements do not allocate resources.
54. Capability declarations do not grant authorization.
55. Target realization is downstream.
56. Vendor/framework extensions belong in explicit dialect/interoperability layers.
57. New AI algorithms should not require universal keyword expansion.
58. Every stable AI construct requires a complete AST contract.
59. Every stable AI construct requires a semantic contract.
60. Every stable AI construct requires an IR/lowering contract.
61. Every stable AI construct requires integration tests.
62. Every stable AI construct requires negative tests.
63. Every stable AI construct requires boundary tests.
64. Every stable AI construct requires scalability tests.
65. Every stable AI construct requires compatibility tests.
66. Every AI grammar file must be independently completable.
67. Rust 1.97 / 1.97.1 remains the implementation baseline.
68. Rust 2021 remains required.
69. "unsafe" Rust remains prohibited.
70. POCO-REAF remains the governing portability objective.

---

78. Final Design Principle

The AI subsystem must implement this invariant:

AI source describes:

    computation
    models
    tensors
    data
    transformations
    training intent
    inference intent
    differentiation intent
    agent behavior
    pipeline structure
    capabilities
    resources
    constraints
    preferences
    effects
    correctness requirements

AI source does NOT inherently describe:

    a particular CPU
    a particular GPU
    a particular accelerator
    a particular QPU
    a particular FPGA
    a particular ASIC
    a particular node
    a particular memory bank
    a particular register width
    a particular network topology
    a particular vendor
    a particular runtime

Therefore the intended property is:

                 ONE ZAMANI PROGRAM
                         |
                         v
              ONE SOURCE SEMANTICS
                         |
              +----------+----------+
              |          |          |
              v          v          v
             CPU        GPU       FPGA
              |          |          |
              +----------+----------+
                         |
                         +------ QPU where applicable
                         |
                         +------ distributed systems
                         |
                         +------ accelerators
                         |
                         +------ future targets

The source remains stable while the compiler determines the realization appropriate to the available resources and capabilities.

That is the AI grammar's required contribution to:

"Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)"

and to Zamani's larger goal of scaling from the smallest supported computation to arbitrarily large heterogeneous computation without turning today's hardware limitations into tomorrow's language limitations.