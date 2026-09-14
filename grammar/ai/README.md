Zamani AI Grammar

1. Purpose

"grammar/ai/" defines the Zamani source-language syntax for artificial intelligence, machine learning, differentiable computation, model execution, data-driven computation, AI pipelines, agents, and AI-oriented accelerator intent.

The subsystem is a syntax layer only.

It allows a Zamani program to express AI computation without embedding assumptions about a particular:

- CPU
- GPU
- TPU
- NPU
- FPGA
- ASIC
- quantum processor
- accelerator
- memory capacity
- tensor size
- device count
- cluster size
- network topology
- execution provider
- vendor
- runtime
- deployment environment

The central rule is:

«AI syntax describes computation and intent; compilation, resource selection, placement, scheduling, optimization, and execution determine how that computation is realized.»

The AI grammar therefore participates in Zamani's:

Program Once → Compile Once → Run Everywhere → Run Anywhere → Run Forever (POCO-REAF)

model.

---

2. Scope

This directory owns syntax for:

1. AI declarations
2. AI models
3. model parameters
4. tensors
5. tensor-oriented operations
6. datasets
7. training
8. inference
9. evaluation
10. differentiation
11. optimization intent
12. AI pipelines
13. AI agents
14. AI accelerator intent
15. AI-specific annotations and metadata
16. AI resource requirements
17. AI capability requirements
18. AI portability declarations
19. AI/classical integration
20. AI/quantum integration points
21. AI/hardware integration points
22. AI/distributed execution integration points

The directory does not own the semantic implementation of these concepts.

---

3. Ownership

3.1 This directory owns

"grammar/ai/" owns:

- lexical references required by AI syntax
- AI grammar productions
- AI-specific syntactic structure
- syntactic composition of AI constructs
- AI syntax versioning hooks
- AI syntax documentation
- AI grammar fixtures
- AI grammar-specific parser tests

---

3.2 This directory does not own

"grammar/ai/" does not own:

- the canonical AST implementation
- type checking
- tensor runtime implementation
- numerical kernels
- automatic differentiation implementation
- model execution
- GPU discovery
- accelerator discovery
- hardware discovery
- hardware topology
- scheduling
- optimization algorithms
- distributed execution
- model storage
- dataset storage
- AI model serialization formats
- quantum IR
- quantum execution
- QEC
- ZQN
- runtime policy
- hardware calibration
- device selection
- compiler backend implementation

Those responsibilities belong to the appropriate repository subsystems.

---

4. Architectural Position

The intended architecture is:

Zamani Source
     │
     ▼
ANTLR Lexer
     │
     ▼
ANTLR Parser
     │
     ▼
AI Grammar
     │
     ▼
Language AST
     │
     ▼
Semantic Analysis
     │
     ├── Type Checking
     ├── Effect Checking
     ├── Capability Checking
     ├── Resource Checking
     └── Domain Validation
     │
     ▼
Canonical Semantic IR
     │
     ├── Classical IR
     ├── Quantum IR
     ├── Hardware IR
     └── Other domain IRs
     │
     ▼
Optimization
     │
     ▼
Scheduling
     │
     ▼
Routing / Placement
     │
     ▼
Target / Hardware Abstraction
     │
     ▼
Runtime

The grammar must never reverse this dependency.

In particular:

grammar → IR
grammar → compiler
grammar → runtime

is permitted as an architectural integration relationship only through parsing/compiler interfaces.

The following are prohibited:

grammar → runtime implementation → grammar
grammar → hardware implementation → grammar
grammar → quantum::ir → grammar

There must be no circular semantic dependency.

---

5. AI Grammar Files

The AI grammar subsystem consists of:

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

Every file has a predefined integration contract.

---

6. "ai.g4"

Purpose

"ai.g4" is the AI grammar composition root.

It establishes the top-level AI syntactic namespace and composes the specialized AI grammar fragments.

Owns

- AI declaration entry points
- AI domain markers
- AI construct composition
- AI-specific top-level syntactic dispatch

Does not own

- model internals
- tensor internals
- training syntax
- inference syntax
- accelerator implementation
- semantic validation

Those belong to specialized grammar files.

Inputs

It consumes grammar productions from:

- core grammar
- types
- expressions
- declarations
- functions
- effects
- resources
- classical grammar
- quantum grammar where explicitly integrated

Outputs

It produces parser-visible AI grammar productions.

Dependencies

Conceptually:

core
types
expressions
declarations
functions
effects
resources
classical

Quantum dependencies are optional syntactic integration points and must not make AI syntax dependent on a physical quantum implementation.

Upstream Contract

The core grammar must already define:

- identifiers
- qualified names
- attributes
- annotations
- paths
- literals
- expressions
- declarations

Downstream Consumers

- parser
- AST builder
- semantic analyzer
- AI lowering
- compiler front end

Public Grammar Contract

AI constructs must be syntactically composable with ordinary Zamani programs.

For example, an AI operation must be usable from ordinary functions, modules, control flow, concurrency constructs, and hybrid programs.

AST Contract

Every parsed AI construct must have source locations and sufficient structural information for semantic lowering.

The grammar must not encode target-specific execution decisions into AST structure.

Semantic Contract

AI syntax expresses:

- what computation is requested
- data relationships
- model relationships
- training/inference intent
- differentiability intent
- resource requirements
- capabilities
- preferences
- constraints

It does not determine:

- where computation executes
- which accelerator is used
- how tensors are physically laid out
- how kernels are scheduled

IR Integration

The resulting AST is lowered into the repository's appropriate semantic representation.

AI grammar must not create an independent AI IR merely because AI syntax exists.

Where existing classical IR is appropriate, AI constructs may lower through it.

Where a dedicated AI semantic representation is required, that representation belongs outside "grammar/".

Compiler Integration

The compiler determines:

- specialization
- lowering
- optimization
- accelerator selection
- execution strategy
- target adaptation

Runtime Integration

Runtime receives compiled semantic/execution representations rather than parsing AI grammar directly.

Tooling Integration

Must support:

- syntax highlighting
- parser diagnostics
- formatting
- language-server parsing
- documentation extraction
- source locations

Cross-Domain Integration

AI must compose with:

- classical
- quantum
- HDL
- hardware
- distributed
- networking
- security
- data
- resources

without creating mutually dependent grammar modules.

Tests

Test:

- minimal AI declarations
- nested AI constructs
- AI inside functions
- AI inside modules
- AI/classical composition
- AI/quantum composition
- AI/hardware composition

Negative Tests

Reject:

- malformed AI declarations
- incomplete constructs
- invalid delimiter structure
- malformed model specifications

Boundary Tests

Use arbitrarily large syntactic structures generated by tests.

No artificial AI size limit may appear in grammar rules.

Compatibility

Grammar changes require language-version tracking.

Scalability

No machine-specific cardinality may occur.

Hard-Coding Audit

Reject grammar constructs that encode:

GPU_COUNT
TPU_COUNT
MAX_TENSORS
MAX_LAYERS
MAX_PARAMETERS
MAX_DEVICES

or equivalent semantic limits.

Completion Criteria

"ai.g4" is complete when every AI grammar fragment can be composed through it without requiring future structural changes caused by model, tensor, training, inference, agent, or accelerator grammar additions.

---

7. "models.g4"

Purpose

Defines syntax for AI model declarations and model composition.

Owns

- model declarations
- model identity
- model interfaces
- model inputs
- model outputs
- model parameters
- model composition
- model configuration syntax

Does not own

- tensor implementation
- numerical kernels
- training algorithms
- model storage
- device selection

Inputs

Consumes:

- identifiers
- types
- expressions
- declarations
- functions
- generics
- attributes

Outputs

Structured model syntax.

Integration

Models must be usable by:

- training
- inference
- pipelines
- agents
- differentiation
- distributed execution

AST Contract

Model nodes must preserve:

- name
- interface
- parameters
- input/output declarations
- source spans
- annotations

Semantic Contract

A model describes a computational structure.

It does not imply a particular implementation.

IR Integration

Lowering decides whether a model becomes:

- classical computation
- tensor computation
- accelerator computation
- hybrid computation
- another supported representation

Quantum Integration

A model may reference quantum computation through established quantum semantic boundaries, but this file must never define quantum gates or duplicate "quantum::ir".

Scalability

Model depth, parameter count, input count, and output count are unbounded by grammar design.

Tests

Include:

- empty/minimal model
- single-input model
- multi-input model
- generic model
- nested/composed models
- arbitrarily parameterized models

Completion

No later AI grammar file should require modification to the fundamental model declaration syntax.

---

8. "tensors.g4"

Purpose

Defines tensor-oriented AI syntax.

Owns

- tensor declarations
- tensor shapes
- dimensions
- symbolic dimensions
- tensor indexing
- tensor expressions
- tensor transformations
- tensor metadata

Does not own

- tensor allocation
- device memory
- kernel implementation
- fixed hardware vector widths

Critical Scalability Rule

Tensor dimensions must support:

- compile-time constants
- runtime dimensions
- symbolic dimensions
- dependent dimensions where supported
- dynamically discovered dimensions

The grammar must not impose a maximum rank or dimension count.

A tensor shape must not inherently require a specific device.

For example, syntax representing a tensor of shape:

[batch, sequence, features]

must remain valid without requiring a fixed numerical value for any dimension.

Resource Integration

Large tensor requirements belong in:

resources/

rather than being encoded as grammar limits.

Classical Integration

Tensor computation may lower to classical numerical/vector/matrix/tensor representations.

Accelerator Integration

Physical tensor layouts belong to target lowering.

Quantum Integration

Quantum-state tensors, if exposed by language semantics, must not be confused with quantum register or state representations owned by the quantum subsystem.

Tests

Include:

- scalar tensor
- vector
- matrix
- higher-dimensional tensor
- symbolic shape
- dynamic shape
- slicing
- indexing
- reshaping
- broadcasting

Negative Tests

Reject malformed dimensions and invalid syntax.

Completion

No later accelerator or hardware grammar may require changing tensor syntax to accommodate a new hardware width.

---

9. "datasets.g4"

Purpose

Defines dataset declarations and dataset-processing syntax.

Owns

- dataset declarations
- schemas
- feature declarations
- labels
- splits
- transformations
- streaming dataset references
- dataset metadata

Does not own

- file-system implementation
- cloud storage
- database implementation
- network transport

Scalability

Dataset size is never represented as a grammar maximum.

Dataset cardinality may be:

- static
- dynamic
- streaming
- externally determined

Security

Access policies must integrate with the security/capability subsystem.

The grammar must not itself authorize access to data.

Distributed Integration

Datasets may be distributed without requiring syntax changes.

Tests

Include local, remote-reference, streaming, partitioned, and dynamically sized datasets.

---

10. "training.g4"

Purpose

Defines model-training syntax.

Owns

- training declarations
- training inputs
- training objectives
- loss expressions
- optimization intent
- epochs/steps where semantically meaningful
- evaluation hooks
- checkpoints as semantic training concepts
- training configuration

Does not own

- optimizer implementation
- scheduler implementation
- checkpoint serialization implementation
- hardware placement
- GPU scheduling
- distributed runtime

Scalability

Training syntax must not assume:

- a fixed number of epochs
- a fixed batch size
- a fixed number of devices
- a fixed model size
- a fixed memory size

Values may be expressions or runtime-derived values where language semantics permit.

Resource Integration

Training may express:

requires
constrains
prefers
hints

through the universal resource model.

A requirement such as:

requires accelerator

must not mean:

requires NVIDIA GPU

unless the program explicitly expresses a vendor-specific semantic dependency.

Distributed Integration

Training may be mapped to distributed execution without changing the model semantics.

Tests

Include:

- minimal training
- training with validation
- training with symbolic batch size
- distributed training intent
- resource-constrained training
- dynamically sized training

---

11. "inference.g4"

Purpose

Defines inference-oriented AI syntax.

Owns

- inference declarations
- model invocation
- inference inputs
- outputs
- serving intent
- batching intent
- latency/performance preferences

Does not own

- runtime serving infrastructure
- networking
- accelerator scheduling
- model deployment implementation

Scalability

Inference syntax must support:

- one input
- streams
- batches
- distributed requests
- dynamically sized requests

without fixed capacity.

Runtime Integration

The runtime resolves:

- execution location
- resource allocation
- batching
- scheduling
- deployment

Tests

Include local, streaming, batched, distributed, and heterogeneous inference syntax.

---

12. "agents.g4"

Purpose

Defines AI-agent syntax.

Owns

- agent declarations
- agent goals
- tools
- actions
- observations
- state declarations
- policies
- agent workflows
- interaction declarations

Does not own

- model implementation
- networking implementation
- security enforcement
- tool execution
- distributed scheduling

Security Integration

Agent capabilities must integrate with:

security/
effects/

An agent declaring a capability does not automatically receive that capability.

Authorization remains outside grammar.

Scalability

No fixed:

- number of agents
- number of tools
- number of interactions
- number of states

may be encoded.

Tests

Include:

- minimal agent
- multi-tool agent
- stateful agent
- distributed agent
- agent + quantum computation
- agent + hardware interaction

---

13. "pipelines.g4"

Purpose

Defines AI computation pipelines.

Owns

- pipeline declarations
- stages
- dependencies
- data flow
- model composition
- execution intent
- pipeline parameters

Does not own

- scheduling algorithms
- dependency graph implementation
- runtime execution
- physical placement

Integration

Pipeline dependencies become semantic dependency information for compiler/scheduler layers.

The grammar must not implement scheduling itself.

Cross-Domain

Pipeline stages may contain:

- classical computation
- AI computation
- quantum computation
- hardware operations
- distributed operations

provided those constructs are semantically compatible.

Scalability

Pipeline stage count is not bounded by grammar.

Pipeline topology is semantic unless explicitly declared as a deployment constraint.

---

14. "differentiation.g4"

Purpose

Defines syntax for differentiable computation.

Owns

- differentiation declarations
- derivative expressions
- gradient requests
- Jacobian requests
- Hessian requests
- differentiation boundaries
- differentiability annotations

Does not own

- automatic differentiation algorithm
- symbolic differentiation engine
- reverse-mode implementation
- forward-mode implementation
- numerical differentiation implementation

Semantic Contract

The source declares differentiation intent.

Compiler infrastructure determines the implementation.

Quantum Integration

Quantum differentiation syntax must not define its own quantum semantics.

Where quantum gradients are supported, they must lower through the established quantum semantic pipeline.

Tests

Include:

- scalar derivative
- vector derivative
- nested differentiation
- parameterized model differentiation
- classical/quantum differentiation integration

---

15. "ai-accelerators.g4"

Purpose

Defines syntax for accelerator intent and capability requirements.

Owns

- accelerator requirement syntax
- accelerator capability references
- accelerator preferences
- accelerator constraints
- accelerator portability annotations

Does not own

- device enumeration
- device identifiers
- vendor APIs
- physical topology
- device discovery
- hardware scheduling

Critical Rule

This is invalid architecture:

use_gpu(0)

when "0" represents a physical device selected outside the program's semantics.

This is preferable:

requires accelerator

or a capability-based requirement.

The exact syntax must be defined consistently with:

grammar/resources/
grammar/hardware/
grammar/effects/

and must not duplicate their semantic models.

Hardware Integration

Hardware abstraction determines whether a target satisfies the requirement.

Quantum Integration

Quantum accelerators are selected through capabilities and target descriptions, not through fixed quantum device identifiers.

Tests

Include:

- generic accelerator requirement
- capability requirement
- preference
- constraint
- portable accelerator request
- incompatible capability expression

---

16. AI and Universal Resource Semantics

AI syntax must distinguish five concepts:

requirement
constraint
preference
hint
capability

They must never be collapsed into one mechanism.

For example:

requires tensor_acceleration

means the program requires an execution capability.

It does not mean:

use a particular GPU

Similarly:

prefers low_latency

does not mean the compiler must violate semantic correctness to achieve low latency.

Resource semantics are interpreted by the compiler, scheduler, hardware abstraction, and runtime.

---

17. AI + Classical Integration

AI is fundamentally capable of classical computation.

AI constructs therefore compose with:

- functions
- loops
- conditionals
- generic types
- numerical operations
- vectors
- matrices
- tensors
- concurrency
- parallelism
- memory abstractions

The AI grammar must reuse common grammar productions instead of redefining them.

There must not be:

AIExpression
ClassicalExpression

with incompatible meanings for the same underlying language concept unless a genuine semantic distinction requires it.

---

18. AI + Quantum Integration

AI and quantum computation may form hybrid programs.

Examples include:

classical data
      ↓
AI model
      ↓
quantum computation
      ↓
measurement
      ↓
classical/AI processing

The AI grammar may provide syntactic integration points.

It must not:

- define quantum gates
- define qubit IDs
- define quantum topology
- define quantum error correction
- define ZQN noise
- duplicate quantum IR

The canonical boundary remains:

AI syntax
   ↓
AST / semantic analysis
   ↓
quantum::ir

where quantum semantics are actually involved.

---

19. AI + HDL Integration

AI computation may target hardware described using the HDL subsystem.

The relationship is:

AI intent
   ↓
compiler lowering
   ↓
hardware-aware representation
   ↓
HDL / hardware lowering

The AI grammar must not directly encode:

- FPGA routing
- LUT counts
- physical pins
- ASIC cell counts
- clock-tree implementation
- device-specific memory layout

unless these are explicitly part of a hardware program's semantic contract.

---

20. AI + Distributed Computing

AI syntax must permit computation to be distributed.

The grammar must not assume:

- number of nodes
- number of workers
- number of accelerators
- cluster topology
- network bandwidth
- memory per node

These belong to:

- resource models
- capabilities
- deployment configuration
- scheduler
- runtime
- hardware abstraction

The same AI source must remain valid from a single local machine through arbitrarily large available distributed resources.

---

21. AI + Security

AI syntax may declare security requirements and capabilities.

It must not itself enforce authorization.

For example:

requires capability(...)

is a declaration.

The security subsystem decides whether that capability is actually available.

Sensitive model/data access must therefore flow through the security/effect architecture.

---

22. AI + Data

AI data syntax must integrate with:

grammar/data/

rather than duplicate:

- collections
- schemas
- serialization
- streams
- records

AI-specific syntax should only exist where AI semantics genuinely require it.

---

23. AI + Concurrency

Training and inference may execute concurrently or in parallel.

The AI grammar should reuse:

grammar/concurrency/

for:

- tasks
- futures
- channels
- synchronization
- parallel execution
- cancellation

AI grammar must not create an independent concurrency model.

---

24. AI + Effects

AI operations may have effects such as:

- I/O
- network access
- accelerator access
- external model access
- data access
- distributed execution

These effects belong to:

grammar/effects/

The AI grammar may reference effect syntax but must not duplicate the effect system.

---

25. Versioning

AI syntax must be versioned with the Zamani language.

A new AI construct must not silently change the meaning of existing valid source.

Compatibility policy belongs to:

grammar/compatibility/

The AI README documents the contract but does not become the version authority.

---

26. ANTLR Integration

The AI grammar is intended for ANTLR-based parsing.

Grammar fragments must:

- have deterministic production behavior
- avoid unnecessary ambiguity
- avoid left-recursive constructs that conflict with parser generation strategy
- use shared tokens
- reuse common productions
- avoid redefining global lexer tokens
- preserve source positions
- generate deterministic parse structures

The composition root must ensure that AI grammar fragments are available to the authoritative "grammar/Zamani.g4".

---

27. Lexer Boundary

AI grammar files should not independently redefine common lexical tokens.

Identifiers, literals, operators, keywords, annotations, and punctuation must follow the central lexer architecture.

AI-specific keywords must be deliberately registered in the language keyword authority.

Potential contextual keywords should be preferred over unnecessarily reserving identifiers when doing so preserves compatibility.

---

28. AST Boundary

The grammar produces syntax.

The AST layer determines semantic node representation.

The grammar must provide enough structure to distinguish:

- model declaration
- tensor expression
- dataset declaration
- training operation
- inference operation
- agent declaration
- pipeline
- differentiation
- accelerator intent

but must not embed compiler decisions into parser actions.

---

29. No Semantic Actions in Grammar

The grammar must not perform:

- hardware discovery
- model execution
- tensor computation
- network access
- file access
- quantum execution
- compilation
- scheduling
- optimization

Parser behavior must remain deterministic and side-effect free.

---

30. Rust Compatibility

The repository's implementation surrounding this grammar must remain compatible with:

Rust 1.97
Rust 1.97.1

as required by the repository's selected toolchain policy.

No "unsafe" code is permitted in supporting Rust grammar/parser infrastructure.

Where Rust-side integration is needed, prefer:

- safe ownership
- borrowing
- standard collections
- explicit error types
- deterministic parsing
- checked conversions
- bounded resource use where the runtime requires operational limits

Operational limits must not be confused with language-level scalability limits.

---

31. Infinite-Scale Principle

"Infinity" is treated as a semantic scalability objective, not as a promise that finite hardware has infinite resources.

The grammar therefore imposes no arbitrary maximum on:

- models
- tensors
- tensor dimensions
- layers
- parameters
- datasets
- pipeline stages
- agents
- tools
- training steps
- inference requests
- accelerators
- nodes
- devices

Actual execution remains constrained by available resources.

When a resource is insufficient, the compiler/runtime must report or negotiate that limitation rather than the grammar having rejected the program because of an arbitrary built-in maximum.

---

32. Hard-Coding Audit

The AI grammar must be continuously checked for:

- "MAX_*"
- fixed tensor rank
- fixed dimension count
- fixed model layers
- fixed parameter counts
- fixed accelerator counts
- fixed GPU IDs
- fixed device IDs
- fixed worker counts
- fixed cluster sizes
- fixed memory sizes
- fixed hardware architectures
- fixed vendor names embedded as universal semantics

Every occurrence must be classified as:

1. language semantic requirement
2. target requirement
3. resource constraint
4. implementation limitation
5. accidental hard-coding
6. test-only limitation
7. documentation-only limitation

Only categories 1–3 may be legitimate, and even then the representation must occur at the correct architectural layer.

---

33. Repository Integration Matrix

Subsystem| AI Grammar Relationship
Lexer| Consumes shared tokens
Parser| Parses AI productions
AST| Receives AI syntax structure
Type system| Validates AI types
Effects| Validates AI effects
Resources| Expresses AI resource requirements
Classical| Provides classical computation
Quantum| Provides quantum semantic integration
"quantum::ir"| Canonical quantum semantic boundary
QEC| Consumes applicable quantum semantics; not defined here
ZQN| Supplies quantum fault/noise semantics; not defined here
Optimization| Optimizes lowered AI computation
Scheduling| Schedules executable work
Hardware| Provides target capabilities
Routing| Determines physical realization where applicable
Runtime| Executes compiled representation
Distributed| Provides distributed execution
Security| Enforces capabilities/policies
Data| Provides general data abstractions
HDL| Provides hardware-description integration
Interoperability| Provides external framework/API integration
Dialects| Provides extensible AI dialect mechanisms
Tests| Validates syntax and integration

---

34. Dependency Direction

The dependency direction must remain:

lexer
  ↓
core
  ↓
types
  ↓
expressions
  ↓
declarations/functions/modules
  ↓
effects/resources
  ↓
classical
  ↓
quantum/hybrid
  ↓
AI
  ↓
hardware/distributed/etc.
  ↓
compiler
  ↓
runtime

This is a conceptual dependency ordering.

Where common grammar fragments are reused, the actual ANTLR imports/dependencies must preserve an acyclic structure.

AI must not become a foundational dependency of core language syntax.

---

35. Required Tests

The AI grammar test suite must eventually include:

Positive

- minimal AI program
- model declaration
- tensor declaration
- dataset
- training
- inference
- agent
- pipeline
- differentiation
- accelerator requirement

Classical + AI

- scalar + tensor
- function + model
- loop + training
- generic function + inference
- concurrency + inference

Quantum + AI

- model + quantum computation
- training + quantum computation
- inference + quantum computation
- hybrid classical/AI/quantum workflow
- measurement feeding AI computation

Hardware + AI

- AI accelerator intent
- AI + hardware capability
- AI + HDL workflow

Distributed + AI

- distributed training
- distributed inference
- pipeline distribution

Security + AI

- protected model
- protected dataset
- capability-restricted agent

---

36. Negative Tests

Tests must reject:

- malformed model declarations
- malformed tensor shapes
- malformed training blocks
- malformed inference blocks
- malformed pipeline dependencies
- malformed agent definitions
- invalid differentiation syntax
- invalid accelerator syntax
- incomplete AI expressions
- invalid delimiters
- ambiguous constructs that violate language rules

Semantic invalidity should be rejected by semantic analysis rather than incorrectly encoded into the grammar whenever the condition cannot be determined syntactically.

---

37. Scalability Tests

The test suite must generate AI programs whose sizes grow independently of grammar constants.

Examples:

1 tensor
10 tensors
100 tensors
N tensors

and:

1 pipeline stage
N pipeline stages

without a source-level maximum.

The test harness itself may impose practical limits for CI execution. Such limits are test infrastructure limits and must never become language semantics.

---

38. Determinism Tests

The same source must produce:

same token sequence
same parse structure
same diagnostics
same source locations

across repeated parser executions under the same language version and parser configuration.

The grammar must not depend on:

- current time
- random values
- hardware
- environment variables
- filesystem state
- network state
- runtime device discovery

for parsing.

---

39. Round-Trip Tests

Where the repository provides a canonical formatter/printer:

source
  ↓
lexer
  ↓
parser
  ↓
AST
  ↓
formatter
  ↓
parser

must preserve semantic structure.

Formatting changes must not change AI semantics.

---

40. Compatibility

AI language evolution must distinguish:

Addition

Adding new syntax that cannot reinterpret existing valid programs.

Deprecation

Existing syntax remains accepted but receives migration guidance.

Migration

An explicit source transformation or compatibility mechanism exists.

Removal

Only permitted through the language compatibility policy.

No AI construct may be silently removed because a newer accelerator, model framework, or AI paradigm becomes available.

---

41. Vendor Independence

Vendor-specific AI frameworks may be represented through:

- interoperability
- dialects
- capabilities
- target descriptions
- foreign interfaces
- explicit vendor-specific extensions

They must not become universal Zamani AI semantics.

For example, the grammar must not make a particular accelerator API the definition of "AI execution."

---

42. Future Extensibility

The AI grammar must accommodate future computational models without redesigning the language core.

Potential future domains include:

- neuromorphic computing
- photonic computing
- analog computing
- biological computing
- quantum machine learning
- distributed intelligence
- new accelerator classes
- future model architectures
- new differentiation systems

New implementations should normally be introduced through:

dialect
capability
effect
resource
target
interoperability

rather than by modifying universal semantics unnecessarily.

---

43. Documentation Relationship

This README is the architectural contract for "grammar/ai/".

The authoritative syntax remains the designated grammar authority established by:

grammar/specification/grammar-authority.md

The individual ".g4" files define syntax.

The README explains:

- ownership
- boundaries
- integration
- scalability
- compatibility
- testing
- architectural intent

Documentation must never silently define syntax that the authoritative grammar does not implement.

---

44. Completion Requirements

"grammar/ai/" is production-ready only when all of the following are true:

- [ ] AI grammar authority is established.
- [ ] All AI grammar fragments are connected to the authoritative parser.
- [ ] Shared lexer tokens are reused.
- [ ] No duplicated core grammar exists.
- [ ] AI syntax has defined AST mappings.
- [ ] AI syntax has semantic-analysis contracts.
- [ ] AI syntax has resource/capability integration.
- [ ] Classical integration works.
- [ ] Quantum integration works.
- [ ] HDL/hardware integration works.
- [ ] Distributed integration works.
- [ ] Data integration works.
- [ ] Security/effect integration works.
- [ ] Accelerator syntax is capability-based.
- [ ] No physical device IDs are required by portable AI syntax.
- [ ] No arbitrary machine-size limits exist.
- [ ] No arbitrary tensor limits exist.
- [ ] No arbitrary model-size limits exist.
- [ ] No arbitrary pipeline-size limits exist.
- [ ] No arbitrary accelerator-count limits exist.
- [ ] Parser behavior is deterministic.
- [ ] Diagnostics identify source locations.
- [ ] Positive tests exist.
- [ ] Negative tests exist.
- [ ] Boundary tests exist.
- [ ] Scalability tests exist.
- [ ] Cross-domain tests exist.
- [ ] Round-trip tests exist where supported.
- [ ] Compatibility rules are documented.
- [ ] Hard-coding audit passes.
- [ ] No "unsafe" Rust is introduced.
- [ ] Rust integration is compatible with the repository's Rust 1.97/1.97.1 policy.
- [ ] No AI grammar file depends directly on runtime implementation.
- [ ] No AI grammar file duplicates "quantum::ir".
- [ ] No circular grammar architecture exists.
- [ ] Documentation agrees with the authoritative grammar.
- [ ] Existing valid Zamani functionality has been preserved or explicitly migrated.

---

45. Definition of Done for the Directory

The directory is complete when an AI program can be expressed once at the Zamani semantic level and subsequently be lowered according to available capabilities and resources without changing the source merely because the execution environment changes.

Conceptually:

                 ONE ZAMANI PROGRAM
                         │
                         ▼
                 AI SEMANTIC INTENT
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
       Classical      Quantum        Hardware
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                  Hybrid Semantics
                         │
                         ▼
               Capability / Resource
                     Resolution
                         │
                         ▼
             Optimization / Scheduling
                         │
                         ▼
             Target / Hardware Mapping
                         │
                         ▼
                  Runtime Execution

The same source-level semantics must remain valid across:

atom-scale systems
embedded systems
CPUs
multicore systems
GPUs
NPUs
FPGAs
ASICs
quantum processors
quantum simulators
heterogeneous systems
clusters
supercomputers
distributed systems
cloud systems
future architectures

subject only to the actual semantic requirements of the program and the capabilities/resources available at execution time.

The governing principle is:

«Zamani AI grammar describes AI computation, intent, relationships, capabilities, and constraints—not the accidental limitations of today's hardware.»

Therefore:

ONE PROGRAM
     ↓
ONE SEMANTIC MEANING
     ↓
MANY COMPILATION TARGETS
     ↓
MANY HARDWARE CONFIGURATIONS
     ↓
MANY SCALES
     ↓
MANY EXECUTION ENVIRONMENTS
     ↓
FUTURE COMPUTING PLATFORMS

This is the AI grammar subsystem's contribution to:

Zamani — From Atom to Everywhere

POCO-REAF

Program Once → Compile Once → Run Everywhere → Anywhere → Forever