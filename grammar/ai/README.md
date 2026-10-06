Zamani AI Grammar

Path: "grammar/ai/"
Domain: AI, machine learning, symbolic computation, reasoning, knowledge, probabilistic computation, differentiable computation, agents, model computation, tensor/data computation, adaptive execution intent, and AI-oriented computational composition
Grammar technology: ANTLR4
Rust baseline: Rust 1.97 or later
Rust edition: 2021
Rust safety policy: Safe Rust only; "unsafe" Rust is prohibited
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: No language-level artificial capacity ceiling; actual execution is constrained only by program semantics, declared requirements, implementation-defined representation limits where explicitly specified, available resources, capabilities, policies, and physical feasibility.

---

1. Purpose

"grammar/ai/" provides the AI-oriented grammar surface of the universal Zamani language.

It does not define a separate AI language.

It does not define a separate compiler.

It does not define a separate AST.

It does not define a separate semantic universe.

It does not define a separate IR.

It does not select hardware.

It does not allocate resources.

It does not execute models.

It does not discover devices.

It does not perform quantum routing.

It does not perform scheduling.

It does not authorize capabilities.

Instead, this directory defines source syntax that expresses AI-related computational intent and feeds that intent into the repository-wide compilation architecture.

The fundamental boundary is:

AI source syntax
        |
        v
domain-neutral frontend AST
        |
        v
semantic analysis
        |
        +--> types
        +--> effects
        +--> resources
        +--> capabilities
        +--> contracts
        +--> policies
        +--> provenance
        +--> security
        +--> determinism
        |
        v
canonical semantic representation
        |
        +--> classical computation
        +--> tensor/data computation
        +--> quantum::ir
        +--> HDL/hardware representation
        +--> distributed computation
        +--> other domain representations
        |
        v
optimization / lowering
        |
        v
routing / placement / scheduling
        |
        v
resilience / recovery / QEC where applicable
        |
        v
ZQN / HAL / target realization

The AI grammar must remain compatible with every stage of this pipeline.

---

2. Production Invariant

The AI subsystem is production-ready only when every public construct has a complete path through:

Specification
    |
    v
Lexer
    |
    v
ANTLR grammar
    |
    v
AST
    |
    v
Structural validation
    |
    v
Semantic analysis
    |
    +--> Type checking
    +--> Effect checking
    +--> Capability checking
    +--> Resource checking
    +--> Contract checking
    +--> Policy checking
    +--> Security checking
    +--> Provenance
    +--> Determinism / reproducibility
    |
    v
Canonical semantic model
    |
    v
IR
    |
    +--> classical IR
    +--> quantum::ir
    +--> other domain IRs
    |
    v
Optimization
    |
    v
Lowering
    |
    v
Routing / placement
    |
    v
Scheduling
    |
    v
Resilience / recovery
    |
    v
ZQN / HAL
    |
    v
target realization

A grammar file is not considered complete merely because ANTLR accepts its syntax.

---

3. Core Architectural Rule

The AI subsystem describes computational intent.

The target system determines physical realization.

Therefore:

source intent
    !=
hardware realization

AI source code must not encode assumptions such as:

CPU 0
GPU 3
specific device ID
specific QPU
specific physical qubit
fixed node count
fixed worker count
fixed memory size
fixed VRAM size
fixed tensor-core width
fixed SIMD width
fixed register width
fixed network topology

Those concerns belong downstream.

The correct model is:

program requirement
        |
        v
capability negotiation
        |
        v
resource analysis
        |
        v
execution planning
        |
        v
target realization

---

4. POCO-REAF

AI programs must participate fully in POCO-REAF.

A program expressing:

model intent
data intent
reasoning intent
learning intent
adaptation intent
resource requirements
capability requirements
contracts
policies
effects
provenance

must remain source-level portable across different realizations.

Potential realizations include:

tiny embedded systems
CPU
multicore CPU
GPU
FPGA
ASIC
AI accelerator
quantum processor
quantum simulator
heterogeneous accelerator
HPC system
cluster
distributed system
cloud infrastructure
future computational targets

The source program must not need to be rewritten merely because the available machine becomes larger, smaller, faster, slower, heterogeneous, distributed, or quantum-enabled.

This does not mean every program is executable on every target.

If a target cannot satisfy the program's semantic requirements, the compiler/runtime must report that incompatibility explicitly.

It must not silently change the meaning of the program.

---

5. Scalability Contract

"grammar/ai/" is an open-world grammar.

It must not impose artificial universal ceilings.

The following concepts must remain unbounded by grammar-level constants:

models
model parameters
model layers
model composition depth
datasets
dataset records
features
tensor rank
tensor dimensions
tensor elements
training steps
epochs
batches
inference requests
agents
tools
observations
actions
knowledge assertions
knowledge relations
reasoning premises
reasoning conclusions
evidence
policies
decisions
distributed participants
workers
accelerators
devices
quantum resources
classical resources
memory
network participants
pipeline stages
pipeline dependencies

The following patterns are prohibited as universal AI language limits:

MAX_MODELS
MAX_MODEL_PARAMETERS
MAX_LAYERS
MAX_DATASET_SIZE
MAX_TENSOR_RANK
MAX_TENSOR_ELEMENTS
MAX_AGENTS
MAX_WORKERS
MAX_ACCELERATORS
MAX_DEVICES
MAX_TRAINING_STEPS
MAX_FEATURES
MAX_PIPELINE_STAGES

This does not prohibit implementation-specific safety limits.

An implementation may have explicit operational limits for:

memory safety
parser resource exhaustion
process quotas
runtime budgets
network policy
sandbox policy
target limitations
security policy

Such limits must remain implementation, policy, resource, or target constraints.

They must never redefine the universal language's semantic capacity.

---

6. Existing AI Grammar Surface

The current directory contains the following grammar modules:

abduction.g4
adaptation.g4
agents.g4
ai-accelerators.g4
ai-capabilities.g4
ai.g4
assertions.g4
capabilities.g4
causality.g4
cognitive.g4
communication.g4
confidence.g4
datasets.g4
decisions.g4
deduction.g4
differentiable.g4
differentiation.g4
distributed-swarm.g4
distributed-training.g4
distributions.g4
effects.g4
embedded.g4
evidence.g4
explanations.g4
facts.g4
federated-learning.g4
feedback.g4
induction.g4
inference.g4
knowledge.g4
learning.g4
mind.g4
model-deployment.g4
model.g4
models.g4
multi-agent.g4
neural-symbolic.g4
neural.g4
optimization.g4
parameters.g4
pipelines.g4
planning.g4
policies.g4
probabilistic.g4
probability.g4
provenance.g4
queries.g4
reasoning.g4
reinforcement.g4
retraction.g4
swarm.g4
symbolic.g4
tensors.g4
training.g4
transfer-learning.g4
uncertainty.g4

This surface must be retained and rationalized through explicit ownership.

A file must not exist merely because a concept sounds useful.

Every file must have:

one primary owner
one canonical semantic responsibility
one documented AST boundary
one documented downstream path
one test owner
one specification owner

Where two files currently describe the same semantic concept, one becomes the canonical owner and the other becomes an adapter, compatibility surface, or candidate for consolidation.

---

7. Ownership Model

The following ownership rules are normative.

Concern| Canonical owner
AI grammar composition| "ai.g4"
Models| "models.g4" / "model.g4"
Model parameters| "parameters.g4"
Tensors| "tensors.g4"
Datasets| "datasets.g4"
Training| "training.g4"
Inference| "inference.g4"
Learning semantics| "learning.g4"
Adaptation| "adaptation.g4"
Reasoning| "reasoning.g4"
Deduction| "deduction.g4"
Induction| "induction.g4"
Abduction| "abduction.g4"
Knowledge| "knowledge.g4"
Facts| "facts.g4"
Queries| "queries.g4"
Retraction| "retraction.g4"
Evidence| "evidence.g4"
Explanations| "explanations.g4"
Decisions| "decisions.g4"
Provenance| "provenance.g4"
Causality| "causality.g4"
Uncertainty| "uncertainty.g4"
Probability| "probability.g4"
Distributions| "distributions.g4"
Confidence| "confidence.g4"
Differentiation| "differentiation.g4"
Differentiable declarations| "differentiable.g4"
Agents| "agents.g4"
Multi-agent semantics| "multi-agent.g4"
Agent communication| "communication.g4"
Planning| "planning.g4"
Cognitive composition| "cognitive.g4"
Neural computation| "neural.g4"
Neural-symbolic composition| "neural-symbolic.g4"
Reinforcement learning| "reinforcement.g4"
Transfer learning| "transfer-learning.g4"
Federated learning| "federated-learning.g4"
Feedback| "feedback.g4"
AI pipelines| "pipelines.g4"
Optimization intent| "optimization.g4"
Model deployment intent| "model-deployment.g4"
AI capabilities| "ai-capabilities.g4"
AI accelerators| "ai-accelerators.g4"
Distributed training| "distributed-training.g4"
Distributed/swarm AI| "distributed-swarm.g4"
AI effects| "effects.g4"
AI policies| "policies.g4"
AI assertions| "assertions.g4"
AI embedded integration| "embedded.g4"
Shared resource capabilities| "grammar/resources/"
Universal effects| "grammar/effects/"
Universal contracts| "grammar/validation/"
Universal policies| "grammar/policies/"
Universal provenance| repository-wide provenance model
Actor lifecycle| "grammar/concurrency/"
Execution adaptation| "grammar/execution/"
Quantum semantics| "grammar/quantum/"
Quantum canonical IR| "src/quantum/ir/"
Hardware semantics| "grammar/hardware/"
HDL semantics| "grammar/hdl/"
Interoperability| "grammar/interoperability/"
Vendor/framework extensions| "grammar/dialects/"

This matrix prevents semantic duplication.

---

8. "ai.g4"

Purpose

"ai.g4" is the composition facade for the AI grammar.

It must not become a monolithic AI grammar.

Owns

- AI construct dispatch;
- AI grammar imports;
- AI-domain composition;
- AI integration boundaries.

Does not own

- tensor internals;
- model internals;
- reasoning internals;
- learning internals;
- resource semantics;
- effect semantics;
- policy semantics;
- quantum operations;
- hardware operations;
- runtime execution.

Integration

The intended hierarchy is:

Zamani.g4
    |
    v
canonical parser
    |
    v
AI
    |
    +--> models
    +--> tensors
    +--> datasets
    +--> reasoning
    +--> knowledge
    +--> learning
    +--> adaptation
    +--> inference
    +--> agents
    +--> pipelines
    +--> differentiation
    +--> uncertainty
    +--> provenance
    +--> neural-symbolic
    +--> deployment
    +--> accelerator intent

The root grammar must not independently import every AI subgrammar.

This keeps root-level coupling bounded.

---

9. Model Grammar

"models.g4" and "model.g4" must have a clearly separated relationship.

There must not be two competing model declaration systems.

Recommended authority:

model.g4
    |
    v
canonical model declaration rules

models.g4
    |
    v
model composition / collection / integration rules

If current contents show overlapping declarations, one must be converted into an adapter or compatibility grammar.

Model semantics

A model may contain:

inputs
outputs
parameters
state
operations
submodels
constraints
contracts
capabilities
effects
policies
metadata
provenance

The grammar must not enumerate every architecture.

The language must not become a closed registry of:

Transformer
CNN
RNN
GAN
VAE
...

Architectures should normally be constructed through generic model semantics.

Framework-specific architectures belong in libraries or dialects.

---

10. "parameters.g4"

Owns model parameter syntax.

It must distinguish:

parameter declaration
parameter value
parameter constraint
parameter metadata
parameter binding
parameter reference

It does not own:

optimizer implementation
memory allocation
device placement
checkpoint storage
hardware layout

Parameter counts and dimensions remain semantic values, not grammar-level capacities.

---

11. "tensors.g4"

Tensor syntax must remain target-neutral.

It owns:

tensor declaration
tensor shape
symbolic dimensions
dynamic dimensions
indexing
slicing
shape expressions
tensor metadata
tensor-oriented structural operations

It does not own:

GPU memory
SIMD width
tensor-core width
physical layout
kernel implementation
device placement
memory allocation

Shapes may be:

constant
symbolic
dependent
runtime-derived
dynamic
unknown until execution

For example, a tensor may semantically depend on:

batch
sequence
features
channels
height
width

without requiring those values to be compile-time constants.

No grammar-level tensor rank or tensor-size ceiling is permitted.

---

12. "datasets.g4"

Owns logical dataset syntax.

It must support:

dataset declaration
schema
features
labels
metadata
logical partitions
streaming
transformations
references
provenance

It does not own:

filesystem
database engine
cloud storage
network transport
physical partitioning
serialization implementation

Existing generic data facilities under "grammar/data/" must be reused.

The AI subsystem must not create a second data language.

---

13. "reasoning.g4"

Reasoning is a generic computational capability.

It must support composition with:

premises
evidence
knowledge
observations
rules
conditions
hypotheses
conclusions
confidence
provenance
policies
contracts

Reasoning must not be restricted to AI models.

The semantic model must permit reasoning to support:

AI
scientific computation
compiler analysis
security analysis
hardware analysis
resource planning
quantum execution planning
distributed decisions

Reasoning syntax must not encode a particular reasoning algorithm.

---

14. "deduction.g4"

Deduction is a specialization of reasoning.

It owns deduction-specific source structure.

It does not create a second reasoning semantic model.

The semantic relationship is:

deduction
    |
    v
reasoning
    |
    v
generic semantic reasoning representation

---

15. "induction.g4"

Induction is similarly a specialization of reasoning.

It may express:

observations
examples
patterns
hypotheses
generalization
confidence
evidence

The grammar must not prescribe one machine-learning algorithm.

---

16. "abduction.g4"

Abduction represents inference toward plausible explanations.

It may consume:

observations
evidence
candidate explanations
constraints
confidence
provenance

It must lower into the shared reasoning semantic model.

---

17. "knowledge.g4"

Knowledge is a universal semantic facility.

It must support knowledge without assuming that knowledge is necessarily AI-specific.

A knowledge item may contain:

subject
relation
object
value
context
validity
confidence
evidence
provenance
metadata

Knowledge can represent:

AI facts
scientific facts
hardware capabilities
security facts
configuration
compiler facts
data relationships
runtime observations

---

18. "facts.g4"

Facts are atomic or structured knowledge assertions.

They must integrate with:

knowledge.g4
assertions.g4
evidence.g4
provenance.g4
queries.g4
retraction.g4

A fact must not become an isolated AI-only AST type.

---

19. "assertions.g4"

Assertions must be distinguished from universal contracts.

AI assertions describe AI/knowledge-domain claims where appropriate.

Universal correctness contracts belong to:

grammar/validation/

Therefore:

AI assertion
    !=
program contract

The two may interact semantically but must not be duplicated syntactically.

---

20. "queries.g4"

Queries represent requests against knowledge/data/model state.

Queries must integrate with:

grammar/data/
grammar/expressions/
grammar/knowledge/
grammar/interoperability/

A query is not inherently a database query.

The same semantic query model may be realized against:

knowledge graph
dataset
stream
memory
distributed data
external data source
runtime state

SQL remains a dialect/interoperability concern rather than universal AI syntax.

---

21. "retraction.g4"

Retraction represents removal or invalidation of knowledge/assertions.

It must preserve provenance.

Retraction must not erase historical provenance automatically.

The semantic system must be capable of distinguishing:

assertion exists
assertion invalidated
assertion superseded
assertion retracted
assertion disputed

from physical deletion.

---

22. "learning.g4"

Learning expresses learning intent.

It owns:

learning declaration
inputs
targets
data
model
objective
evaluation
feedback
constraints
requirements
policies
provenance

It does not own optimizer implementations.

Algorithms should be represented through:

libraries
intrinsics
semantic operation identifiers
dialects
compiler-selected implementations

rather than permanent grammar keywords.

Learning must participate in the effect system.

---

23. "adaptation.g4"

Adaptation is controlled change of computational state, strategy, model, policy-selected behavior, or execution plan.

It must never mean unrestricted self-modifying execution.

The semantic pipeline is:

adaptation request
    |
    v
policy
    |
    v
authorization
    |
    v
capability check
    |
    v
effect check
    |
    v
resource check
    |
    v
provenance
    |
    v
validation
    |
    v
authorized adaptation

Adaptation may concern:

model state
model parameters
strategy
execution plan
resource selection
fallback
retry
recovery
migration
rerouting
rescheduling
runtime state

The grammar must not decide which mechanism performs the adaptation.

---

24. "feedback.g4"

Feedback represents information returned to a learning, reasoning, planning, or adaptation process.

Feedback must integrate with:

learning
adaptation
agents
reasoning
provenance
uncertainty
execution

It must not duplicate the universal event/stream model where one already exists.

---

25. "uncertainty.g4"

Uncertainty must be represented independently of a specific probabilistic implementation.

It may express:

uncertain value
belief
confidence
interval
distribution reference
epistemic state
aleatoric state

The grammar must not assume a specific probability representation.

---

26. "probability.g4"

Probability is a mathematical semantic capability.

It must not become a list of implementation-specific probability algorithms.

The grammar should represent mathematical intent.

---

27. "distributions.g4"

Distributions represent semantic probability distributions.

They must integrate with:

probability
uncertainty
confidence
types
expressions
data

A distribution must remain independent of its eventual representation.

---

28. "confidence.g4"

Confidence represents an explicit confidence/strength value attached to an appropriate semantic result.

Confidence must be:

typed
validated
provenance-aware
composable where mathematically valid

It must not automatically be interpreted as probability.

---

29. "evidence.g4"

Evidence is a universal cross-domain concept.

Evidence may support:

claim
decision
reasoning result
model result
security decision
resource decision
compiler transformation
quantum execution decision
hardware selection

Evidence should preserve:

source
claim
relation
strength
confidence
derivation
verification
provenance

Evidence must not be AI-only at the semantic layer.

---

30. "explanations.g4"

Explanations provide structured reasons for semantic results.

The same explanation model may explain:

AI decision
reasoning result
resource selection
compiler transformation
optimization
quantum routing
hardware placement
security decision
adaptation

An explanation must not expose or require a particular internal model implementation.

---

31. "decisions.g4"

Decisions represent selected outcomes.

A decision should preserve:

options
selected result
criteria
constraints
evidence
confidence
policy
provenance
explanation

Decision-making must not automatically grant authority.

Authorization remains owned by security/policy systems.

---

32. "provenance.g4"

AI provenance must use the repository-wide provenance architecture.

Do not create a competing provenance model.

A provenance record may describe:

source
derived artifact
transformation
model
data
evidence
decision
reason
verification
version
execution context
policy

Provenance must remain available across:

AI
classical
quantum
HDL
hardware
distributed
security
data
compiler
execution

---

33. "causality.g4"

Causal reasoning may represent:

cause
effect
observation
intervention
dependency
counterfactual
causal relation

Causality must remain distinct from ordinary correlation.

The grammar expresses causal intent; the semantic layer determines whether the requested causal operation is valid.

---

34. "neural.g4"

"neural.g4" defines neural-computation-oriented source constructs.

It must not become a permanent registry of every neural architecture.

Generic composition must remain possible.

Architecture-specific functionality belongs in libraries or dialects.

---

35. "neural-symbolic.g4"

Neural-symbolic computation is a first-class integration boundary.

It combines:

learned computation
+
symbolic computation
+
reasoning
+
knowledge

The architecture must remain:

neural operation
      |
      +--> classical/tensor semantics
      |
      v
shared semantic model
      ^
      |
symbolic operation
      |
      +--> reasoning
      +--> knowledge
      +--> contracts

There must not be separate execution universes for neural and symbolic computation.

---

36. "differentiable.g4"

This file owns declarations/annotations describing differentiability.

It must not implement differentiation.

It must integrate with:

differentiation.g4
types
expressions
effects
semantic analysis

---

37. "differentiation.g4"

Differentiation expresses what should be differentiated.

It does not prescribe:

forward mode
reverse mode
symbolic differentiation
automatic differentiation implementation
numerical differentiation
specific accelerator kernels

The compiler may choose the appropriate implementation based on semantic requirements and target capabilities.

---

38. "training.g4"

Training syntax must remain declarative.

It may express:

model
data
objective
loss
evaluation
validation
feedback
checkpoint intent
requirements
capabilities
constraints
preferences
policies

It must not encode:

fixed worker count
fixed accelerator count
fixed memory
fixed batch capacity
fixed training duration

Distributed training is delegated to:

grammar/distributed/
grammar/ai/distributed-training.g4

and downstream execution.

---

39. "inference.g4"

Inference expresses model execution intent.

It must support:

single invocation
batch invocation
streaming invocation
dynamic input
pipeline invocation
distributed invocation

without encoding physical deployment.

---

40. "agents.g4"

An AI agent is a semantic composition of:

state
observation
goal
reasoning
planning
learning
action
tool usage
policy
memory
provenance

An agent is not a second concurrency system.

---

41. Agent / Concurrency Integration

The canonical relationship is:

AI agent
    |
    v
actor/task/process semantic model
    |
    v
grammar/concurrency/
    |
    v
scheduler/runtime

"agents.g4" owns AI-agent meaning.

"grammar/concurrency/" owns:

actors
tasks
channels
messages
synchronization
scheduling semantics

AI must not duplicate those constructs.

---

42. "multi-agent.g4"

Multi-agent semantics must build on existing concurrency/distributed semantics.

It may describe:

agent groups
roles
coordination
delegation
interaction
collective decisions
agent relationships

It must not create a separate:

agent scheduler
agent actor runtime
agent message transport

---

43. "communication.g4"

AI communication syntax must reuse the universal communication model where possible.

Transport concerns belong to:

grammar/networking/
grammar/distributed/
grammar/interoperability/

---

44. "planning.g4"

Planning expresses goals, states, actions, constraints, and plans.

Planning can consume:

knowledge
reasoning
uncertainty
policies
resources
capabilities
agents

Planning does not own scheduling.

---

45. "cognitive.g4"

Cognitive composition is a semantic composition layer.

It must not create an independent cognitive runtime.

It may compose:

perception
memory
knowledge
reasoning
planning
learning
decision
action

using the canonical underlying semantic constructs.

---

46. "mind.g4"

"mind.g4" must remain an integration/abstraction layer if retained.

It must not create a second type system or execution model.

If any rule duplicates:

reasoning
memory
knowledge
planning
learning
decision
agent

the duplicated rule must be replaced by references to the canonical owner.

---

47. "policies.g4"

AI policies must consume the universal policy architecture.

Policies can govern:

learning
adaptation
agent actions
resource use
data access
model deployment
execution
security
explanation
provenance

Policy is not authorization by itself.

Authorization is resolved by the security and execution layers.

---

48. "effects.g4"

AI effects must integrate with "grammar/effects/".

AI-related effects can include semantic categories such as:

learning
adaptation
inference
measurement
randomness
network
foreign
native
reflection
code generation
distributed
simulation

The AI grammar must not create a second effect system.

---

49. Resource and Capability Integration

AI resource requirements must use:

grammar/resources/

Examples of semantic intent include:

requires capability("tensor.compute")
requires capability("ai.inference")
requires capability("quantum.measurement")
requires memory >= required_memory
requires topology(required_topology)

These express requirements.

They do not select a physical device.

The semantic pipeline is:

AI requirement
    |
    v
resource semantic model
    |
    v
capability negotiation
    |
    v
execution planning
    |
    v
target realization

---

50. "ai-capabilities.g4"

This file describes AI-domain capability requirements or declarations where they are genuinely AI-specific.

It must not duplicate generic capability syntax.

Generic capabilities belong to:

grammar/resources/
grammar/security/

AI-specific capability identifiers should remain extensible.

No finite universal registry is required.

---

51. "capabilities.g4"

If retained as an AI adapter, "capabilities.g4" must delegate generic capability semantics to the universal resource/capability subsystem.

It must not create:

AI capability type system
AI capability authorization system
AI capability registry

as competing architectures.

---

52. "ai-accelerators.g4"

This file expresses accelerator intent.

Correct:

requires capability("tensor.compute")
requires capability("ai.acceleration")
prefer capability("parallel.compute")

Incorrect as universal syntax:

use_gpu(0)
use_device(3)
use_specific_vendor_device(...)

Physical device selection belongs downstream.

---

53. "optimization.g4"

Optimization grammar expresses optimization intent.

It must not permanently enumerate compiler optimization algorithms.

The compiler may select:

fusion
tiling
vectorization
parallelization
distribution
specialization
quantization
lowering

or future strategies according to semantics and capabilities.

---

54. "pipelines.g4"

Pipelines compose stages.

Stages may cross domains:

data
    ->
classical
    ->
AI
    ->
quantum
    ->
classical
    ->
AI
    ->
distributed

This must remain one program-level semantic composition.

AI must not create a separate pipeline runtime.

---

55. "model-deployment.g4"

Deployment intent must remain separate from execution implementation.

It may express:

deployment requirements
availability requirements
latency intent
throughput intent
security policy
resource requirements
capability requirements
portability constraints

It must not hard-code a cloud provider or physical deployment topology.

---

56. Distributed Training

"distributed-training.g4" owns AI-specific distributed-training intent.

Generic distributed semantics remain under:

grammar/distributed/

The AI layer may describe:

distributed training
replication intent
partitioning intent
collective requirements
fault/recovery intent

but actual scheduling and transport remain downstream.

No maximum number of workers or nodes may be encoded.

---

57. Federated Learning

"federated-learning.g4" may describe federated-learning semantics such as:

participants
local learning
aggregation intent
privacy requirements
communication policy
provenance

It must not hard-code:

number of participants
specific aggregation implementation
specific cryptographic backend
specific network topology

Cryptographic implementation belongs to the security/cryptography architecture.

---

58. Swarm and Distributed Agents

"distributed-swarm.g4" and "swarm.g4" must integrate with:

agents.g4
multi-agent.g4
grammar/distributed/
grammar/concurrency/
grammar/networking/

They must not create another distributed runtime.

---

59. Embedded AI

"embedded.g4" describes embedded AI intent.

It must remain compatible with:

grammar/hardware/
grammar/classical/
grammar/resources/
grammar/execution/

Embedded syntax must not assume a particular microcontroller, memory size, instruction width, accelerator, or bus.

---

60. AI / Quantum Integration

AI and quantum computation must interoperate through the universal hybrid architecture.

The correct model is:

AI source
    |
    v
domain-neutral AST
    |
    v
semantic analysis
    |
    +--> AI/tensor semantics
    |
    +--> classical semantics
    |
    +--> quantum semantics
             |
             v
         quantum::ir
             |
             v
         optimization
             |
             v
         decomposition
             |
             v
         routing
             |
             v
         scheduling
             |
             v
         QEC/resilience
             |
             v
         ZQN
             |
             v
         HAL

AI must never create a competing quantum IR.

In particular, these are prohibited architectural patterns:

ai::quantum::ir
ai::qml::ir
ai::model::quantum_ir

when they duplicate "quantum::ir".

---

61. Quantum Operations

AI grammar must not enumerate quantum gates.

It must consume the generic quantum operation abstraction.

Quantum operation semantics belong to:

grammar/quantum/

and the canonical implementation boundary:

src/quantum/ir/

This permits future quantum operations without repeatedly modifying AI grammar.

---

62. Quantum-Classical Hybrid Computation

Hybrid AI/quantum programs must permit semantic flow such as:

classical data
    |
    v
AI preprocessing
    |
    v
quantum computation
    |
    v
measurement
    |
    v
classical result
    |
    v
AI reasoning / learning

Measurement results remain classical semantic values after crossing the appropriate quantum boundary.

Quantum resource mapping remains downstream.

---

63. Neural-Symbolic Quantum Integration

The architecture must support:

neural computation
+
symbolic reasoning
+
knowledge
+
quantum computation
+
classical control

without creating a second compiler.

All components converge on the shared semantic model and appropriate domain IR.

---

64. HDL and Hardware Integration

AI grammar may express hardware-relevant intent through:

capabilities
resources
performance constraints
power constraints
latency constraints
data movement requirements
accelerator requirements

Actual HDL semantics belong to:

grammar/hdl/

Hardware realization belongs to:

grammar/hardware/

AI grammar must not embed RTL syntax.

---

65. Data and Interoperability

AI must reuse:

grammar/data/
grammar/interoperability/
grammar/dialects/

for:

JSON
XML
SQL
external datasets
foreign APIs
serialization
schemas
FFI
ABI

SQL is not universal AI syntax.

JSON is not universal AI syntax.

XML is not universal AI syntax.

They are interoperability/data representations.

---

66. FFI and ABI

AI models may interface with external implementations.

The architecture is:

AI source
    |
    v
foreign declaration
    |
    v
effect/capability analysis
    |
    v
ABI
    |
    v
foreign implementation

FFI must participate in:

effects
security
capabilities
provenance
compatibility

AI grammar must not assume a particular foreign runtime.

---

67. Provenance Across AI and Compilation

AI provenance must survive semantic lowering.

For example:

source expression
    |
    v
reasoning result
    |
    v
model operation
    |
    v
classical/tensor operation
    |
    v
quantum operation
    |
    v
optimized operation
    |
    v
target operation

The provenance chain must remain traceable where the selected compilation mode requires it.

This is essential for:

explainability
debugging
audit
scientific reproducibility
security
verification
model governance
compiler diagnostics

---

68. Determinism and Reproducibility

Grammar parsing must be deterministic.

The grammar must not perform:

filesystem access
network access
hardware discovery
resource discovery
random selection
model loading
dataset loading
runtime execution

For a fixed:

source
lexer version
grammar version
parser configuration
enabled dialect configuration

the parse result must be deterministic.

Runtime learning or adaptation may be nondeterministic when explicitly permitted by semantic effects and policies.

Reproducibility belongs to:

execution
provenance
compile
compatibility
runtime

rather than parser actions.

---

69. Security

AI syntax must never grant itself authority.

These are distinct:

requires capability(...)

and:

has capability(...)

and:

authorized capability(...)

A source program can request a capability.

The compiler/runtime/security architecture decides whether that capability is available and authorized.

AI operations involving:

learning
adaptation
reflection
network
FFI
native calls
code generation
model deployment
data access

must participate in the appropriate effect and security analysis.

---

70. Sandboxing

Sandboxing is not an AI-only facility.

AI syntax must consume the shared sandbox/security model.

A sandbox may constrain:

effects
capabilities
resources
network
filesystem
FFI
native execution
reflection
adaptation
data access
model loading

The grammar expresses policy-relevant structure.

The security system enforces it.

---

71. Contracts

AI constructs must participate in universal contracts.

The canonical contract system belongs to:

grammar/validation/

AI operations may therefore interact with:

requires
ensures
invariant
assume
guarantee
property
assertion
evidence
proof

AI grammar must not create a separate contract language.

---

72. Policy

Policy can constrain:

learning
adaptation
agent actions
model deployment
data access
reasoning
resource use
quantum execution
hardware realization
network use
FFI

The policy architecture remains shared.

---

73. Application-Specific Features

The AI grammar must not become an application-keyword catalog.

The following kinds of concepts should normally remain outside core grammar:

computer vision application names
sentiment-analysis keywords
robotics application names
payment operations
administrative actions
specific legal workflows
specific business processes
specific blockchain operations
specific VR/AR application concepts
specific cloud-provider operations
specific model-provider APIs

These belong in:

libraries
dialects
capabilities
policies
services
applications
interoperability adapters

The universal language should provide the primitives from which such systems are built.

---

74. Generic Reasoning and Knowledge Are More Important Than Keyword Count

The AI subsystem should prefer a small number of composable semantic primitives over an enormous list of application-specific keywords.

The central semantic capabilities are:

reason
infer
deduce
induce
abduce
assert
query
retract
learn
adapt
explain
evaluate
decide
observe
plan

These capabilities should compose with:

types
expressions
effects
resources
capabilities
contracts
policies
provenance

A new AI algorithm should normally not require a new keyword.

---

75. AST Contract

The frontend AST is domain-neutral.

AI grammar must not force an AI-specific AST hierarchy that duplicates universal nodes.

AI-specific information must be represented using the existing AST architecture for:

declarations
expressions
statements
types
operations
attributes
modifiers
effects
capabilities
resources
contracts
policies
metadata
source spans

Where an AI-specific semantic distinction is genuinely necessary, it must be documented and justified before introducing a new AST node.

Every AST node must preserve source provenance.

---

76. Semantic Contract

After parsing, AI constructs must be resolved into semantic structures.

Semantic analysis is responsible for:

name resolution
scope resolution
type checking
generic resolution
shape checking
effect checking
capability checking
resource checking
contract checking
policy checking
provenance
security
differentiability
quantum boundary validation
distributed boundary validation
interop validation

The grammar must not perform these operations.

---

77. IR Contract

AI grammar does not own a universal AI IR.

AI semantics may lower into:

classical IR
tensor/data representation
quantum::ir
distributed representation
HDL/hardware representation
other canonical domain representations

according to semantic requirements.

The IR boundary is determined by the compiler architecture.

---

78. Quantum IR Contract

When AI computation contains quantum semantics:

AI semantic model
      |
      v
quantum semantic boundary
      |
      v
quantum::ir

"quantum::ir" is canonical.

AI must not duplicate:

quantum operation model
qubit model
quantum resource model
quantum routing model
quantum scheduling model
QEC model
ZQN model

---

79. Execution Contract

The AI grammar does not execute anything.

Execution may involve:

CPU
GPU
FPGA
ASIC
AI accelerator
QPU
simulator
HPC
cluster
distributed runtime
cloud runtime
future targets

The compiler/runtime chooses a valid realization based on:

semantics
requirements
capabilities
resources
constraints
preferences
policies
target availability

---

80. Adaptive Execution

AI adaptation can interact with execution adaptation.

The execution subsystem may perform:

detect
evaluate
select
fallback
retry
recover
reroute
reschedule
migrate
adapt

The AI grammar does not determine whether adaptation is compile-time or runtime.

---

81. Simulation

AI programs must be simulatable where their semantics permit it.

Simulation may represent:

AI simulation
classical simulation
quantum simulation
hardware simulation
distributed simulation
fault simulation
performance simulation

Simulation is an execution strategy, not a second grammar.

---

82. Resilience

AI execution may interact with resilience.

The AI grammar must not duplicate resilience state machines.

Existing execution/quantum resilience semantics remain authoritative.

AI may consume observations such as:

healthy
degraded
unstable
unavailable
recovering
quarantined
retired

where exposed through the semantic/runtime boundary.

---

83. Compatibility

Every AI feature must have:

introduced version
semantic version
grammar compatibility
AST compatibility
IR compatibility
dialect compatibility
deprecation policy
migration path

A feature must not silently change meaning.

Deprecated syntax must have a documented migration path.

---

84. Dialects

Vendor-specific and framework-specific AI features belong under:

grammar/dialects/

when they require syntax.

Examples include:

framework-specific model syntax
vendor accelerator extensions
provider-specific deployment
specialized inference APIs
specialized training configuration

Dialect extensions must map back into the canonical semantic model.

A dialect must not create a second universal language.

---

85. File-Level Completion Contract

Every ".g4" file under "grammar/ai/" must document, either in its own header or its corresponding specification, all of the following:

Purpose
Status
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Public Rules
Private Rules
Lexer Dependencies
Grammar Dependencies
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
Compiler Integration
Runtime Integration
Tooling Integration
Security
Diagnostics
Positive Tests
Negative Tests
Boundary Tests
Cross-Domain Tests
Scalability Tests
Determinism Tests
Reproducibility Tests
Compatibility Tests
Hard-Coding Audit
Completion Criteria

This is mandatory because a file should be independently completable.

---

86. Dependency Declaration

Each AI grammar file should identify:

DEPENDS_ON
EXPORTS
CONSUMED_BY
AST_OWNER
SEMANTIC_OWNER
IR_OWNER
SPEC_OWNER
TEST_OWNER

Example:

File:
grammar/ai/reasoning.g4

DEPENDS_ON:
    core expressions
    types
    evidence
    knowledge
    provenance
    validation

EXPORTS:
    AI reasoning composition rules

CONSUMED_BY:
    ai.g4
    planning.g4
    cognitive.g4
    neural-symbolic.g4
    agents.g4

AST_OWNER:
    domain-neutral frontend AST

SEMANTIC_OWNER:
    universal reasoning semantic model

IR_OWNER:
    canonical semantic/IR layer

SPEC_OWNER:
    grammar/spec/ai.md

TEST_OWNER:
    grammar/tests/ai/reasoning/

The exact filenames must be kept synchronized with the actual repository.

---

87. Diagnostics Contract

AI grammar diagnostics must be structural.

The parser must report:

unexpected token
missing construct
malformed expression
invalid delimiter
invalid grammar composition
ambiguous syntax

Semantic diagnostics must report:

unknown model
invalid type
invalid tensor shape
invalid capability
unsatisfied requirement
effect violation
contract violation
policy violation
invalid quantum boundary
invalid differentiation boundary
invalid interoperability boundary

The grammar must not attempt to solve semantic errors through parser hacks.

---

88. Negative Testing

Every AI feature requires negative tests.

Examples include:

missing required model
invalid model argument
invalid tensor shape syntax
invalid query
invalid reasoning structure
invalid evidence structure
invalid adaptation
invalid policy
invalid capability requirement
invalid contract
invalid provenance
invalid agent structure
invalid pipeline dependency
invalid quantum boundary
invalid differentiation

Negative tests must verify that malformed constructs are rejected rather than silently reinterpreted.

---

89. Boundary Testing

AI tests must cross boundaries deliberately.

Required combinations include:

AI + classical
AI + data
AI + concurrency
AI + distributed
AI + networking
AI + security
AI + contracts
AI + policies
AI + provenance
AI + hardware
AI + HDL
AI + quantum
AI + hybrid
AI + interoperability
AI + metaprogramming

---

90. Scalability Testing

Scalability tests must verify absence of artificial grammar ceilings.

Tests should vary semantic quantities without changing grammar architecture:

small tensor
large tensor
symbolic tensor
dynamic tensor

small model
deep model
composed model

small dataset
large dataset
streaming dataset

one agent
many agents

single pipeline
large pipeline

single quantum operation
large quantum operation graph

single target
heterogeneous target set
distributed target set

The tests must not depend on arbitrary constants embedded in the grammar.

---

91. Determinism Testing

Given identical:

source
lexer configuration
grammar version
dialect configuration

the parser must produce deterministic output.

The grammar must not contain:

randomness
filesystem access
network access
hardware discovery
environment-dependent semantic actions

---

92. Rust Contract

The grammar itself is language-neutral, but generated and supporting frontend code must comply with:

Rust >= 1.97
Rust 2021
safe Rust only
no unsafe code
no unsafe operations

Repository Rust crates implementing AI parsing, AST conversion, semantic analysis, or related tooling should use appropriate crate-level safety enforcement such as:

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

where applicable.

No AI grammar feature may require unsafe Rust.

---

93. Memory Safety

AI grammar support must not require unsafe memory access.

Large programs, models, datasets, or tensor descriptions must be represented using safe abstractions.

Parser resource protection must be implemented through explicit safe mechanisms such as:

configuration
budgets
cancellation
bounded work where operationally required
streaming
incremental processing
resource policies

Operational safeguards are not language semantic ceilings.

---

94. Incremental and Large-Scale Compilation

The AI grammar must remain compatible with:

incremental parsing
incremental semantic analysis
incremental compilation
parallel analysis
content-addressed artifacts
caching
streaming data
large model descriptions
large pipelines

No feature should require the complete runtime dataset, model weights, or hardware inventory to be loaded merely to parse source syntax.

---

95. Model Weights

Model weights are not grammar syntax.

The grammar may reference model artifacts, but model storage and loading belong to:

data
interoperability
execution
runtime
model deployment

The grammar must not embed model weights into parser-specific structures.

---

96. Dataset Loading

Dataset loading is a runtime/data concern.

The grammar describes:

what data is required
how it is logically shaped
how it participates in computation

It does not perform data loading.

---

97. Framework Independence

Core AI grammar must not depend on a particular AI framework.

Framework integration belongs to:

dialects
interoperability
libraries
compiler backends
runtime adapters

This prevents framework churn from changing the core language.

---

98. Vendor Independence

Core AI grammar must not contain vendor-specific physical assumptions.

Vendor-specific features must be represented through:

dialect
capability
resource
policy
interoperability
backend

where appropriate.

---

99. Future AI Technologies

The grammar must remain open to future computational techniques.

Adding a future AI technique should normally require:

new library
new semantic operation
new dialect
new intrinsic
new compiler implementation

rather than changing the universal grammar.

Only a genuinely new source-language abstraction should require a new core grammar rule.

---

100. No Keyword Explosion

The following rule is mandatory:

«If an application concept can be expressed using existing generic semantic primitives, it must not become a new universal keyword.»

For example, the language should not require a new keyword for every:

vision algorithm
language model
robotics algorithm
optimization algorithm
neural architecture
reasoning algorithm
probability distribution
deployment provider
hardware accelerator

Generic semantic operations are preferred.

---

101. Semantic Extensibility

The AI subsystem should be extensible through:

generic operation identifiers
namespaces
attributes
parameters
types
capabilities
effects
resources
dialects
libraries
intrinsics

This allows the ecosystem to grow without repeatedly changing the root grammar.

---

102. Namespace Discipline

AI operations and extensions must be namespace-safe.

Vendor/framework/application-specific operations should not pollute the universal namespace.

Prefer semantic qualification such as:

namespace.operation

where the existing language architecture supports it.

The exact namespace syntax remains owned by the core/name grammar.

---

103. Metadata

AI constructs may carry metadata such as:

documentation
provenance
model identity
data identity
version
annotations
optimization hints
deployment hints

Metadata must not change semantic meaning unless the specification explicitly defines it as semantic.

---

104. Hints vs Requirements

This distinction is mandatory.

requires capability("tensor.compute")

means the capability is necessary.

A preference means:

prefer ...

A hint means:

hint ...

A hint must never silently become a requirement.

A preference must not silently become a requirement.

---

105. Resource Negotiation

AI resource negotiation follows:

intent
    |
    v
requirements
    |
    v
capabilities
    |
    v
constraints
    |
    v
preferences
    |
    v
target negotiation
    |
    v
execution plan

The grammar does not perform negotiation.

---

106. Target Independence

AI source syntax must remain independent of:

CPU architecture
GPU architecture
FPGA family
ASIC design
QPU vendor
cloud provider
cluster manager
operating system
driver
runtime implementation

Target-specific information belongs downstream.

---

107. Classical Integration

AI computation that can be expressed as classical computation must integrate with the classical subsystem.

No AI-only arithmetic system should be created.

Use:

grammar/classical/
grammar/types/
grammar/expressions/

for universal mathematical and computational semantics.

---

108. Quantum Integration

AI computation that requires quantum semantics must integrate with:

grammar/quantum/
src/quantum/ir/

The canonical pipeline remains:

source
    ->
AST
    ->
semantic model
    ->
quantum::ir
    ->
optimization
    ->
decomposition
    ->
routing
    ->
scheduling
    ->
resilience/QEC
    ->
ZQN
    ->
HAL

No AI-specific quantum backend may bypass this path.

---

109. HDL Integration

AI hardware intent must integrate with:

grammar/hdl/
grammar/hardware/

The AI layer describes intent.

The HDL layer describes hardware semantics.

The hardware/compiler layer determines realization.

---

110. Distributed Integration

AI distributed computation must integrate with:

grammar/distributed/
grammar/concurrency/
grammar/networking/

No fixed node count is permitted.

---

111. Security Integration

AI security must integrate with:

grammar/security/
grammar/policies/
grammar/resources/
grammar/effects/

Security must be enforceable independently of AI semantics.

---

112. Interoperability Integration

AI interoperability must integrate with:

grammar/interoperability/
grammar/dialects/

This includes:

FFI
ABI
external models
external data
external runtimes
serialization
schemas
query systems

---

113. Metaprogramming Integration

AI-related generated code or reflection must use:

grammar/metaprogramming/
grammar/macros/

AI must not create a private reflection/metaprogramming mechanism.

Reflection and code generation must carry the appropriate effects and security requirements.

---

114. Compatibility With Existing Files

The following repository-level files/subsystems are integration authorities:

grammar/DESIGN.md
grammar/README.md
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/Zamani.g4

grammar/core/
grammar/types/
grammar/expressions/
grammar/statements/
grammar/declarations/
grammar/functions/
grammar/modules/

grammar/effects/
grammar/resources/
grammar/validation/
grammar/policies/
grammar/security/
grammar/provenance/

grammar/classical/
grammar/data/
grammar/concurrency/
grammar/distributed/
grammar/execution/
grammar/hardware/
grammar/hdl/
grammar/hybrid/
grammar/interoperability/
grammar/dialects/

grammar/quantum/

src/lexer.rs
src/parser.rs
src/semantic.rs
src/optimizer.rs
src/quantum/
src/quantum/ir/

AI grammar changes must respect these ownership boundaries.

---

115. Generated Parser Boundary

ANTLR grammar is a source representation.

Generated parser code is an implementation artifact.

The AI grammar must not rely on handwritten parser behavior that exists only outside the grammar unless that behavior is explicitly documented as part of the parser architecture.

No grammar feature is considered complete if:

ANTLR grammar accepts it

but:

frontend parser cannot construct the required AST

or:

semantic analyzer cannot interpret it

---

116. Source Span Preservation

Every AI construct must preserve source location information.

At minimum:

file
start offset
end offset
line
column

or the repository's canonical source-span representation.

This is required for:

diagnostics
provenance
debugging
IDE tooling
explanations
semantic errors
cross-domain lowering

---

117. Tooling Integration

AI grammar must support tooling without creating a second parser.

Tooling should consume the same canonical structures for:

syntax highlighting
completion
navigation
documentation
diagnostics
refactoring
semantic inspection
model inspection
pipeline visualization
provenance inspection

---

118. IDE Stability

AI syntax should be sufficiently structured that partially written source can be diagnosed without causing cascading parser failures throughout unrelated constructs.

Grammar design should therefore prefer:

clear delimiters
predictable composition
explicit optional sections
non-ambiguous prefixes

over deeply ambiguous alternatives.

---

119. Testing Layout

AI tests should ultimately be organized by semantic owner:

grammar/tests/ai/
├── lexical/
├── parser/
├── ast/
├── semantic/
├── models/
├── parameters/
├── tensors/
├── datasets/
├── reasoning/
├── deduction/
├── induction/
├── abduction/
├── knowledge/
├── assertions/
├── queries/
├── retraction/
├── learning/
├── adaptation/
├── feedback/
├── uncertainty/
├── probability/
├── distributions/
├── confidence/
├── evidence/
├── explanations/
├── decisions/
├── provenance/
├── causality/
├── agents/
├── multi-agent/
├── planning/
├── cognitive/
├── neural/
├── neural-symbolic/
├── differentiation/
├── training/
├── inference/
├── pipelines/
├── distributed/
├── federated/
├── deployment/
├── accelerators/
├── hybrid/
├── quantum/
├── hdl/
├── security/
├── interoperability/
├── scalability/
├── determinism/
├── compatibility/
├── negative/
└── boundary/

Only directories corresponding to actual maintained test ownership should be created.

---

120. Required Integration Programs

The AI subsystem must have representative integration programs covering:

minimal AI
classical AI
reasoning
knowledge
learning
adaptation
uncertainty
evidence
explanation
decision
agent
multi-agent
neural-symbolic
tensor
dataset
pipeline
differentiation
distributed learning
federated learning
simulation
sandbox
FFI
interoperability
hybrid
quantum
HDL/hardware intent
POCO-REAF

These must exercise the complete frontend pipeline, not merely individual parser rules.

---

121. Mandatory Cross-Domain Program

At least one integration program must combine:

data
+
classical computation
+
tensor computation
+
reasoning
+
knowledge
+
learning
+
adaptation
+
uncertainty
+
evidence
+
decision
+
agent
+
parallelism
+
resource requirements
+
capabilities
+
effects
+
contracts
+
policies
+
provenance
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
    |
    v
lexer
    |
    v
parser
    |
    v
AST
    |
    v
structural validation
    |
    v
semantic model
    |
    +--> type analysis
    +--> effect analysis
    +--> capability analysis
    +--> resource analysis
    +--> contract analysis
    +--> policy analysis
    +--> provenance
    |
    v
canonical IR
    |
    +--> classical representation
    +--> quantum::ir
    |
    v
optimization
    |
    v
lowering
    |
    v
routing
    |
    v
scheduling
    |
    v
resilience
    |
    v
target realization

This test is the principal proof that the AI grammar is integrated rather than isolated.

---

122. Hard-Coding Audit

Before declaring the AI grammar production-ready, search the complete directory for artificial capacity definitions.

Reject patterns representing universal ceilings for:

CPU count
GPU count
FPGA count
QPU count
node count
device count
memory
threads
tensor rank
tensor dimensions
model size
dataset size
agent count
worker count
pipeline count

Also reject hard-coded target assumptions such as:

specific vendor as universal
specific accelerator as universal
specific QPU as universal
specific cloud as universal
specific model provider as universal

---

123. Semantic Limits vs Implementation Limits

The project must explicitly distinguish:

language semantics
implementation representation
resource policy
runtime budget
target capability
physical feasibility

For example, an implementation may choose an integer representation internally for a tensor dimension.

That does not automatically mean the language defines that integer width as the universal semantic limit.

The distinction must be documented wherever representation limits matter.

---

124. No Hidden Limits

Limits must never be hidden in:

grammar alternatives
parser loops
AST containers
semantic validation
IR builders
test fixtures
runtime defaults

without documentation.

If an operational limit is necessary, it must be:

explicit
named
documented
configurable where appropriate
error-reporting
semantically distinguished from language capacity

---

125. Performance

Production readiness includes grammar performance.

The AI grammar should avoid pathological ambiguity.

Tests should measure:

lexing
parsing
large model declarations
large tensor shapes
large pipeline structures
large knowledge expressions
large agent compositions
large distributed descriptions

Performance optimization must not change language semantics.

---

126. Incremental Evolution

When a new AI capability is proposed, use this sequence:

1. Identify whether an existing semantic primitive already represents it.
2. Identify the canonical owner.
3. Determine whether syntax is actually required.
4. Define the semantic contract.
5. Define AST mapping.
6. Define effect/resource/capability interactions.
7. Define policy/security interactions.
8. Define provenance.
9. Define IR destination.
10. Add positive/negative/boundary tests.
11. Add compatibility information.
12. Integrate into ai.g4 only if composition requires it.
13. Update specifications.
14. Update conformance status.

Do not begin by adding a keyword.

---

127. New File Rule

A new file under "grammar/ai/" is justified only when at least one of the following is true:

the concept has independent grammar ownership
the concept has independent semantic ownership
the concept requires independent testing
the concept has independent compatibility lifecycle
the existing file would otherwise become a conflicting monolith

Otherwise extend the existing owner.

---

128. Duplicate File Rule

If two files own substantially the same syntax or semantics:

do not maintain two independent implementations

Instead:

choose canonical owner
        |
        +--> adapter
        |
        +--> compatibility layer
        |
        +--> migration
        |
        +--> deprecation

This is especially important for:

model / models
capabilities / ai-capabilities
differentiable / differentiation
adaptation
effects
policies
assertions
reasoning specializations
distributed/swarm
agent/multi-agent

---

129. Documentation Synchronization

Whenever an AI grammar file changes, its associated contracts must remain synchronized:

grammar/specification/
grammar/spec/
grammar/grammar.md
grammar/tests/
AST implementation
semantic implementation
IR integration

The README is the architectural contract.

It does not replace normative specifications.

---

130. Completion Criteria for "grammar/ai/"

The directory is production-ready only when:

[ ] Every AI grammar file has one clear owner.
[ ] No two files define competing AI semantic systems.
[ ] ai.g4 is the composition facade.
[ ] AI syntax is reachable from the canonical parser.
[ ] No AI grammar bypasses the domain-neutral AST.
[ ] No AI grammar owns runtime behavior.
[ ] No AI grammar selects physical hardware.
[ ] No AI grammar allocates resources.
[ ] No AI grammar performs device discovery.
[ ] No AI grammar performs network access.
[ ] No AI grammar performs filesystem access.
[ ] No AI grammar performs model loading.
[ ] No AI grammar performs dataset loading.
[ ] No AI grammar contains embedded Rust actions.
[ ] No AI feature requires unsafe Rust.
[ ] Rust integration supports Rust 1.97 or later.
[ ] AI types use the universal type system.
[ ] AI effects use the universal effect system.
[ ] AI requirements use the universal resource system.
[ ] AI capabilities use the universal capability system.
[ ] AI contracts use the universal validation system.
[ ] AI policies use the universal policy system.
[ ] AI provenance uses the universal provenance system.
[ ] AI agents use the existing concurrency model.
[ ] AI distributed computation uses the distributed model.
[ ] AI networking uses the networking model.
[ ] AI quantum computation uses quantum semantics.
[ ] Quantum computation lowers through quantum::ir.
[ ] AI HDL intent uses the HDL/hardware boundary.
[ ] AI interoperability uses the interoperability architecture.
[ ] Framework-specific features use dialects/libraries.
[ ] Vendor-specific features do not pollute core syntax.
[ ] No artificial universal capacity constants exist.
[ ] Symbolic/dynamic quantities are supported where semantically valid.
[ ] Positive tests exist.
[ ] Negative tests exist.
[ ] Boundary tests exist.
[ ] Cross-domain tests exist.
[ ] Scalability tests exist.
[ ] Determinism tests exist.
[ ] Compatibility tests exist.
[ ] Reproducibility tests exist.
[ ] Hard-coding audit passes.
[ ] Full frontend integration passes.
[ ] POCO-REAF integration passes.

---

131. Definition of DONE for an Individual AI File

An individual file is DONE only when:

Purpose is fixed.
Ownership is fixed.
Non-ownership is fixed.
Dependencies are fixed.
Public rules are fixed.
AST mapping is fixed.
Semantic mapping is fixed.
Type behavior is fixed.
Effect behavior is fixed.
Capability behavior is fixed.
Resource behavior is fixed.
Contract behavior is fixed.
Policy behavior is fixed.
Provenance behavior is fixed.
IR destination is fixed.
Compiler integration is fixed.
Runtime boundary is fixed.
Tooling boundary is fixed.
Diagnostics are defined.
Positive tests exist.
Negative tests exist.
Boundary tests exist.
Scalability tests exist.
Determinism tests exist.
Compatibility is defined.
Hard-coding audit passes.

A change elsewhere must not force a completed file to be redesigned merely because another subsystem was implemented later.

If a later change genuinely alters a shared contract, that is a contract migration, not an accidental dependency.

---

132. Definition of DONE for "grammar/ai/"

"grammar/ai/README.md" is complete as the architectural contract when it accurately describes the current repository and establishes stable integration boundaries.

"grammar/ai/" itself is complete only after the implementation satisfies the contracts documented here.

The following distinction is mandatory:

README complete
    !=
grammar implementation complete

The README defines what production readiness means.

The grammar, AST, semantic analyzer, compiler, IR, runtime, and tests must subsequently satisfy it.

---

133. Final Architecture

The production architecture is:

                         ZAMANI SOURCE
                              |
                              v
                         CANONICAL LEXER
                              |
                              v
                       CANONICAL PARSER
                              |
                +-------------+-------------+
                |                           |
                v                           v
          UNIVERSAL CORE                    AI
                |                           |
                |       +-------------------+-------------------+
                |       |       |       |       |       |       |
                |       v       v       v       v       v       v
                |   Reason   Knowledge Learn Adapt  Agents  Models
                |       |       |       |       |       |       |
                |       +-------+-------+-------+-------+-------+
                |                           |
                +-------------+-------------+
                              |
                              v
                     DOMAIN-NEUTRAL AST
                              |
                              v
                      STRUCTURAL VALIDATION
                              |
             +----------------+----------------+
             |                |                |
             v                v                v
           Types           Effects        Contracts
             |                |                |
             +----------------+----------------+
                              |
             +----------------+----------------+
             |                |                |
             v                v                v
       Capabilities        Resources        Policies
             |                |                |
             +----------------+----------------+
                              |
                              v
                         PROVENANCE
                              |
                              v
                       SEMANTIC MODEL
                              |
             +----------------+----------------+
             |                |                |
             v                v                v
       Classical/Tensor   quantum::ir     HDL/Hardware
             |                |                |
             |                v                |
             |          Optimization           |
             |                |                |
             |          Decomposition           |
             |                |                |
             |             Routing               |
             |                |                |
             |           Scheduling              |
             |                |                |
             |        QEC / Resilience          |
             |                |                |
             +----------------+----------------+
                              |
                              v
                            ZQN
                              |
                              v
                             HAL
                              |
             +----------------+----------------+
             |        |       |       |        |
            CPU      GPU     FPGA    ASIC     QPU
             |        |       |       |        |
             +--------+-------+-------+--------+
                              |
                   heterogeneous/distributed
                              |
                              v
                         FUTURE TARGETS

The essential invariant is:

AI is not a separate language.
AI is not a separate compiler.
AI is not a separate IR.
AI is not a hardware selector.

AI is a composable semantic domain inside the universal language.

The most important consequence is that reasoning, knowledge, learning, adaptation, uncertainty, evidence, explanations, decisions, agents, neural-symbolic computation, tensor computation, and quantum-classical computation all become composable capabilities of one language architecture rather than a collection of unrelated AI keywords.

The scalability invariant is:

No artificial grammar ceiling
        +
symbolic resource requirements
        +
capability negotiation
        +
target-independent semantics
        +
canonical IR boundaries
        +
dynamic execution planning
        =
POCO-REAF

The physical machine may scale from extremely small to extremely large, heterogeneous, distributed, or future computational substrates without requiring the source language to contain a new fixed-size grammar for every possible machine.

Actual execution remains subject to:

program semantics
declared requirements
available capabilities
available resources
security policy
execution policy
implementation feasibility
target feasibility
physical reality

Those are constraints on realization, not artificial ceilings on the language itself.