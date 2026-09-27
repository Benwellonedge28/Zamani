Zamani Execution Grammar

Path: "grammar/execution/"
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Grammar authority: "grammar/Zamani.g4"
Architecture authority: "grammar/DESIGN.md"
Implementation-conformance reference: "grammar/grammar.md"
Extended/historical design reference: "grammar/Zamani-Grammar.md"
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Rust safety policy: Safe Rust only; production Rust MUST NOT use "unsafe"
Primary portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever"
Abbreviation: "POCO-REAF"

---

1. Purpose

"grammar/execution/" defines the source-level execution-intent language of Zamani.

Execution syntax describes how a Zamani program declares or communicates:

- what should execute;
- what execution conditions are required;
- what capabilities are required;
- what resources are required;
- what constraints apply;
- what execution preferences exist;
- what implementation hints are permitted;
- what target classes are acceptable;
- what placement properties matter;
- what scheduling properties matter;
- what synchronization semantics matter;
- how execution may be dispatched;
- what runtime environment is required;
- how execution lifecycle is controlled;
- what failure and recovery policy is requested;
- what checkpoint/restore behavior is permitted;
- what observability is requested;
- what tracing/profiling is requested;
- what resilience behavior is acceptable;
- what deployment relationship exists.

It does not execute a program.

It does not discover hardware.

It does not allocate resources.

It does not schedule operations.

It does not route quantum operations.

It does not perform QEC.

It does not implement ZQN.

It does not implement a runtime.

It does not define physical hardware.

Its job is to express portable execution meaning and intent that later compiler, planner, scheduler, runtime, deployment, and hardware layers can realize.

---

2. Normative Status

This README is the architectural contract for the execution grammar domain.

It defines:

1. execution-domain ownership;
2. execution-domain non-ownership;
3. integration with the canonical language grammar;
4. integration with the canonical lexer;
5. integration with the parser;
6. integration with the frontend AST;
7. integration with semantic analysis;
8. integration with resource and capability analysis;
9. integration with canonical IR;
10. integration with classical computation;
11. integration with "quantum::ir";
12. integration with HDL/hardware;
13. integration with scheduling;
14. integration with placement;
15. integration with routing;
16. integration with resilience;
17. integration with QEC;
18. integration with ZQN;
19. integration with HAL;
20. integration with runtime;
21. integration with deployment;
22. integration with diagnostics;
23. integration with compatibility/versioning;
24. integration with tests;
25. scalability and hard-coding requirements;
26. completion criteria for every execution grammar component.

This README is not a competing language specification.

The normative language specification remains under:

grammar/specification/
grammar/spec/

The canonical ANTLR composition root remains:

grammar/Zamani.g4

The implementation-conformance reference remains:

grammar/grammar.md

The historical/extended design reference remains:

grammar/Zamani-Grammar.md

---

3. Fundamental Execution Principle

Execution syntax expresses:

WHAT
+
UNDER WHAT SEMANTIC CONDITIONS
+
WITH WHAT CAPABILITIES
+
WITH WHAT RESOURCES
+
UNDER WHAT CONSTRAINTS
+
WITH WHAT PREFERENCES
+
WITH WHAT PERMITTED IMPLEMENTATION GUIDANCE

It does not unnecessarily encode:

HOW A PARTICULAR MACHINE HAPPENS TO EXECUTE IT

Therefore execution grammar MUST remain independent of:

- CPU model;
- GPU model;
- QPU model;
- FPGA model;
- ASIC model;
- accelerator vendor;
- simulator vendor;
- operating system;
- physical device identifier;
- physical address;
- fixed memory bank;
- fixed register width;
- fixed topology;
- fixed node count;
- fixed device count;
- fixed CPU count;
- fixed GPU count;
- fixed FPGA count;
- fixed QPU count;
- fixed qubit count;
- fixed thread count;
- fixed queue capacity;
- fixed cluster size;
- fixed network size;
- fixed execution timeline count.

---

4. POCO-REAF

Execution is one of the principal mechanisms protecting:

Program
  ↓
Compile
  ↓
Run

from becoming:

Program
  ↓
Machine-specific rewrite
  ↓
Machine-specific compilation
  ↓
Machine-specific execution

The Zamani objective is:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

POCO-REAF does not mean that every program can run on every target regardless of resources.

Instead:

portable semantics
        ↓
capability requirements
        ↓
resource requirements
        ↓
target resolution
        ↓
realization

A program may be semantically valid while a particular target is unable to execute it.

For example:

requires qubits >= logical_qubits;

does not make the language invalid when the current QPU has insufficient physical resources.

The compiler/runtime may instead:

- select another target;
- distribute the workload;
- use logical qubits;
- use error correction;
- decompose operations;
- route operations;
- schedule execution;
- simulate the computation;
- defer execution;
- use an alternative backend;
- report a precise target/resource incompatibility.

The execution grammar MUST NOT silently rewrite the program merely because one target is insufficient.

---

5. Scalability Contract

Execution grammar must scale from the smallest supported execution to arbitrarily large executions subject only to:

- program semantics;
- representation limits;
- declared semantic constraints;
- compiler resources;
- runtime resources;
- target capabilities;
- deployment resources;
- externally imposed resource policies.

The grammar MUST NOT establish universal hardware ceilings.

The following are prohibited as language-level limits:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_JOBS
MAX_STAGES
MAX_TIMELINES
MAX_REPLICAS
MAX_RETRIES

Equivalent hidden limits are also prohibited.

For example, this is prohibited:

execution grammar accepts at most 32 placement clauses

This is also prohibited:

execution grammar supports at most 8 resources

And:

execution grammar supports at most 16 targets

Repeated syntax must be represented using grammar repetition or recursively composable structures.

Actual limits belong to the implementation/environment and must not become language semantics.

---

6. Execution Domain Scope

The execution domain covers:

execution
├── contexts
├── environments
├── entrypoints
├── dispatch
├── runtime
├── scheduling
├── placement
├── synchronization
├── parallel execution
├── runtime capabilities
├── deployment
├── lifecycle
├── resilience
├── recovery
├── checkpointing
├── observability
├── tracing
└── profiling

The existing execution directory already contains modular execution grammar components, including:

execution.g4
execution-context.g4
dispatch.g4
placement.g4
scheduling.g4
synchronization.g4
runtime-capabilities.g4
parallel-execution.g4
deployment.g4
runtime.g4
entrypoints.g4
environments.g4

Existing files MUST be expanded and integrated rather than unnecessarily renamed.

Additional execution-specific files may be added where necessary, but they must not create competing ownership.

Recommended additional components, only where not already represented elsewhere in the repository, are:

lifecycle.g4
resilience.g4
recovery.g4
checkpointing.g4
observability.g4
tracing.g4
profiling.g4

If an equivalent canonical component already exists elsewhere in "grammar/", the execution directory MUST reference it rather than duplicate it.

---

7. Ownership

"grammar/execution/" owns source-level syntax for:

- execution declarations;
- execution contexts;
- execution environments;
- entry points;
- execution requests;
- execution policies;
- dispatch intent;
- scheduling intent;
- placement intent;
- synchronization intent;
- runtime intent;
- lifecycle intent;
- failure policy;
- recovery policy;
- checkpoint intent;
- observability intent;
- tracing intent;
- profiling intent;
- runtime capability requirements;
- execution resource intent;
- execution-specific constraints;
- execution-specific preferences;
- execution-specific hints;
- execution metadata.

---

8. Non-Ownership

Execution grammar does not own:

Lexical rules

Owned by the canonical lexer.

General expressions

Owned by:

grammar/expressions/

General types

Owned by:

grammar/types/

General declarations

Owned by:

grammar/declarations/

Functions

Owned by:

grammar/functions/

Modules

Owned by:

grammar/modules/

Effects

Owned by:

grammar/effects/

Resource semantics

Owned by:

grammar/resources/

and the corresponding semantic/resource subsystem.

Hardware semantics

Owned by:

grammar/hardware/

HDL

Owned by:

grammar/hdl/

Classical computation

Owned by:

grammar/classical/

Quantum operations

Owned by:

grammar/quantum/

Quantum IR

Owned by the canonical:

quantum::ir

Routing

Owned by routing.

Scheduling algorithms

Owned by scheduling implementation.

Optimization

Owned by optimization.

QEC

Owned by the QEC subsystem.

ZQN

Owned by the Zamani Quantum Noise subsystem.

Calibration

Owned by hardware/calibration.

Hardware discovery

Owned by HAL/hardware infrastructure.

Resource discovery

Owned by resource management.

Runtime implementation

Owned by runtime infrastructure.

Deployment implementation

Owned by deployment infrastructure.

The execution grammar must never absorb these implementations.

---

9. Canonical Architecture

The execution pipeline is:

Zamani source
      |
      v
canonical lexer
      |
      v
canonical parser
      |
      v
domain-neutral frontend AST
      |
      v
structural analysis
      |
      +--> name/module resolution
      +--> type analysis
      +--> effect analysis
      +--> ownership/resource analysis
      +--> capability analysis
      +--> portability analysis
      +--> execution-intent validation
      |
      v
canonical semantic representation
      |
      +-------------------+-------------------+
      |                   |                   |
      v                   v                   v
 classical representation quantum::ir HDL/hardware representation
      |                   |                   |
      +-------------------+-------------------+
                          |
                          v
                     optimization
                          |
              +-----------+-----------+
              |           |           |
              v           v           v
           routing    scheduling   resilience
              |           |           |
              +-----------+-----------+
                          |
                    QEC / ZQN
                          |
                         HAL
                          |
                          v
                   target realization
                          |
                          v
                       dispatch
                          |
                          v
                       runtime
                          |
                          v
                     deployment

The execution grammar participates in this pipeline but does not replace any downstream subsystem.

---

10. Canonical Composition Rule

The root language grammar remains:

grammar/Zamani.g4

"Zamani.g4" is the composition root.

Execution syntax must be reachable from the root through the canonical grammar composition.

There must not be another competing root grammar.

In particular, the repository must not maintain two independently authoritative forms such as:

grammar/Zamani.g4
grammar/antlr/ZamaniParser.g4

where both independently define Zamani.

Any ANTLR helper grammar under "grammar/antlr/" must be treated as an implementation component or transitional artifact, not as a second language authority.

The final authority remains:

grammar/Zamani.g4

---

11. Canonical Dependency Direction

The dependency direction is:

Lexer
  ↓
Core
  ↓
Expressions / Types / Declarations / Statements
  ↓
Execution
  ↓
Semantic analysis
  ↓
Canonical semantic model
  ↓
IR
  ↓
Optimization
  ↓
Routing / Scheduling / Resilience
  ↓
QEC / ZQN / HAL
  ↓
Target
  ↓
Runtime

The following dependency directions are prohibited:

runtime → grammar
hardware → grammar
HAL → grammar
scheduler implementation → grammar
QEC implementation → grammar
ZQN implementation → grammar

Implementation systems consume the semantic result of the grammar.

They do not redefine the grammar.

---

12. Canonical Lexer Boundary

Execution grammars consume the repository's canonical lexical model.

They must not create an independent execution lexer.

The lexer owns:

- identifiers;
- keywords;
- literals;
- punctuation;
- operators;
- comments;
- Unicode;
- source locations;
- lexical diagnostics.

Execution grammar may use canonical tokens such as:

IDENTIFIER
INTEGER
FLOAT
STRING
COLON
SEMICOLON
COMMA
DOT
DOUBLE_COLON
ASSIGN
LBRACE
RBRACE
LPAREN
RPAREN
LBRACKET
RBRACKET

or their canonical repository equivalents.

Execution syntax must not invent duplicate lexical concepts.

---

13. Important Existing Lexer/Parser Integration Issue

The repository's current handwritten lexer/parser exposes overlapping lexical concepts, including constructs such as:

BitAnd
Ampersand
Question
QuestionMark

where applicable.

Execution grammar MUST NOT create another interpretation of these tokens.

The repository-wide lexical normalization must establish one canonical meaning for each lexical token.

Execution files must consume that canonical vocabulary after normalization.

Similarly, execution grammar must not assume that a token exists merely because it appears in a historical ANTLR file.

Every execution token must have a traceable path:

specification
    ↓
canonical token contract
    ↓
lexer
    ↓
ANTLR vocabulary
    ↓
parser
    ↓
AST

This is a repository-wide compatibility requirement, not an execution-only workaround.

---

14. General Expression Integration

Execution syntax must reuse the canonical expression grammar.

It must not define another expression language.

Execution values may therefore be:

literal
name
qualified name
function call
arithmetic expression
comparison
logical expression
range
collection
symbolic expression
domain-specific expression

Examples:

required_qubits
required_qubits(input_size)
problem_size * 2
available_memory >= required_memory
capability("quantum.measurement")

The execution grammar must not redefine operator precedence.

Expression precedence belongs to:

grammar/expressions/

---

15. Requirement / Constraint / Preference / Hint

These four concepts MUST remain semantically distinct.

Requirement

Mandatory.

Example:

requires: capability("quantum.measurement");

Failure means the realization is incompatible.

Constraint

Restricts allowed realizations.

Example:

constraint: latency <= deadline;

Preference

Advisory desired behavior.

Example:

prefer: accelerator("quantum");

Failure to satisfy a preference does not automatically invalidate the program.

Hint

Advisory implementation guidance.

Example:

hint: locality::near;

A backend MUST NOT silently promote a hint to a semantic requirement.

---

16. Resource Integration

Execution grammar may refer to resource intent.

Examples include:

memory
compute
storage
bandwidth
quantum
accelerator
communication
energy
time

Resource quantities are expressions.

For example:

requires: qubits >= logical_qubits;
requires: memory >= required_memory;
requires: bandwidth >= required_bandwidth;

The grammar does not determine whether a resource is physically available.

That belongs to resource analysis and target realization.

---

17. Capability Integration

Execution capability requirements must remain open-ended.

Examples:

capability("tensor.compute")
capability("gpu.compute")
capability("quantum.measurement")
capability("quantum.mid_circuit_measurement")
capability("fault_tolerance")
capability("distributed.communication")
capability("realtime")
capability("secure.execution")

The grammar recognizes the structure.

Semantic analysis resolves whether the capability has meaning under the current language profile.

Runtime/HAL determines whether the capability is actually available.

A future capability must not require a new parser architecture merely because the capability name is new.

---

18. No Device Enumeration in the Grammar

Execution grammar must not encode universal device inventories.

Prohibited as universal grammar assumptions:

gpu0
gpu1
qpu0
qpu1
cpu0
cpu1
node0
node1
fpga0
fpga1

A user may explicitly request a concrete target identity where the language specification permits target-specific deployment semantics.

Such concrete binding is a program portability decision, not a language-wide machine assumption.

Portable execution should instead prefer:

capability(...)
target(...)
resource(...)
constraint(...)
preference(...)

---

19. Runtime Integration

"runtime.g4" owns source-level runtime intent.

It may describe:

- runtime requirements;
- runtime policies;
- runtime lifecycle;
- runtime adaptation;
- runtime capability intent;
- runtime resource intent;
- runtime observability;
- runtime resilience intent;
- runtime portability intent.

It MUST NOT:

- execute code;
- discover hardware;
- call runtime APIs;
- open files;
- access networks;
- allocate memory;
- allocate devices;
- select physical qubits;
- invoke vendor APIs.

The runtime implementation consumes the semantic representation after compilation and validation.

---

20. Runtime Environment Integration

"environments.g4" owns reusable source-level execution-environment contracts.

An environment is:

abstract execution intent

not:

physical machine instance

Environment declarations may contain:

- requirements;
- capabilities;
- constraints;
- preferences;
- hints;
- execution policies;
- portability properties.

They must not perform:

- hardware discovery;
- resource allocation;
- scheduling;
- routing;
- runtime initialization;
- deployment.

---

21. Entrypoint Integration

"entrypoints.g4" owns executable entry-point declarations.

An entry point describes:

what can be invoked externally

not:

how a machine invokes it internally

Entry points must remain independent of:

- CPU count;
- GPU count;
- QPU count;
- node count;
- thread count;
- memory size;
- hardware topology.

There is no universal maximum number of:

- entry points;
- parameters;
- return values;
- requirements;
- capabilities;
- resources;
- policies.

---

22. Dispatch Integration

"dispatch.g4" represents the semantic transition toward execution.

Dispatch must remain distinct from runtime implementation.

The conceptual flow is:

program
  ↓
validated realization
  ↓
dispatch intent
  ↓
runtime

Dispatch may express:

- invocation;
- execution handoff;
- asynchronous dispatch;
- synchronous dispatch;
- deferred dispatch;
- completion behavior;
- dispatch policy.

It must not invoke a backend from the parser.

---

23. Scheduling Integration

"scheduling.g4" owns scheduling intent.

It may represent:

- priority;
- ordering;
- dependencies;
- deadlines;
- latency;
- throughput;
- alignment;
- temporal intent;
- scheduling policy;
- concurrency intent;
- resource intent.

It must not implement:

- list scheduling;
- critical-path scheduling;
- ASAP;
- ALAP;
- resource allocation;
- queue management;
- hardware scheduling;
- pulse scheduling;
- OS scheduling.

The scheduler implementation consumes the semantic scheduling model later.

---

24. Placement Integration

"placement.g4" owns placement intent.

It may express:

- locality;
- affinity;
- anti-affinity;
- co-location;
- separation;
- distribution;
- replication;
- migration;
- elasticity;
- abstract target intent.

It must not:

- allocate hardware;
- select physical qubits;
- construct routes;
- insert SWAP operations;
- select GPU IDs;
- select FPGA tiles;
- select physical memory banks;
- inspect machine topology.

---

25. Synchronization Integration

"synchronization.g4" owns source-level synchronization intent.

It may represent:

- ordering;
- barriers;
- completion;
- dependencies;
- visibility;
- synchronization scopes;
- coordination requirements.

It does not implement:

- OS mutexes;
- hardware locks;
- distributed consensus algorithms;
- scheduler internals;
- memory barriers at a target-specific machine-instruction level.

Those are downstream implementation concerns.

---

26. Parallel Execution Integration

"parallel-execution.g4" describes parallel execution intent.

It must support:

- task parallelism;
- data parallelism;
- pipeline parallelism;
- concurrent execution;
- distributed parallelism;
- accelerator parallelism;
- heterogeneous parallelism.

It must not impose:

8 threads
16 threads
32 cores
4 GPUs
2 nodes

as universal language limits.

If a programmer explicitly requests a resource count, that number is a program-level requirement, not a grammar capacity.

---

27. Distributed Execution

Execution integrates with:

grammar/distributed/

Distributed execution must distinguish:

logical distribution

from:

physical node topology

Execution syntax may express:

- distribution;
- locality;
- replication;
- consistency;
- communication requirements;
- service placement intent;
- migration intent;
- fault-tolerance intent.

Actual nodes and topology are resolved downstream.

There is no universal maximum node count.

---

28. Classical Integration

Classical execution may consume:

grammar/classical/

and the classical IR.

Execution intent can apply to:

- scalar computation;
- vector computation;
- matrix computation;
- tensor computation;
- scientific computation;
- numerical computation;
- symbolic computation;
- HPC;
- embedded computation;
- accelerator computation.

Execution syntax does not need separate constructs for every CPU architecture.

---

29. Quantum Integration

Quantum execution must integrate with:

grammar/quantum/

and the canonical:

quantum::ir

The flow is:

quantum source
      ↓
quantum grammar
      ↓
domain-neutral AST
      ↓
semantic quantum representation
      ↓
quantum::ir
      ↓
optimization
      ↓
routing
      ↓
scheduling
      ↓
QEC / resilience / ZQN
      ↓
HAL
      ↓
target
      ↓
runtime

Execution grammar MUST NOT create another quantum IR.

It must not redefine:

- quantum gates;
- quantum operations;
- qubits;
- quantum states;
- measurements;
- circuits;
- observables;
- quantum channels.

Those belong to the quantum language/semantic system.

---

30. Quantum Operation Extensibility

The execution subsystem must remain compatible with the open quantum-operation model.

The language must not depend on a fixed universal list such as:

H
X
Y
Z
CNOT
...

Execution may reference semantic quantum capabilities such as:

capability("quantum.measurement")
capability("quantum.mid_circuit_measurement")
capability("quantum.dynamic_control")
capability("quantum.error_correction")

The actual operation model remains owned by the quantum subsystem and ultimately "quantum::ir".

This permits:

- custom operations;
- vendor operations;
- future operations;
- parameterized operations;
- logical operations;
- hardware-native operations;

without turning each into a permanent core-language keyword.

---

31. Quantum Resource Scaling

The execution grammar must permit resource requirements derived from:

- algorithm;
- input;
- problem size;
- logical circuit;
- symbolic expressions;
- semantic analysis;
- runtime information.

Example:

requires: qubits >= logical_qubits;

is valid.

The following must never become language limits:

MAX_QUBITS = 1024
MAX_QUBITS = 4096
MAX_QUBITS = 1_000_000

The same rule applies to:

- CPU;
- GPU;
- FPGA;
- memory;
- accelerator;
- network;
- distributed node;
- tensor;
- thread;
- register;
- storage.

---

32. QEC Integration

Execution grammar may express QEC requirements or policies.

For example:

requires: capability("quantum.error_correction");

or equivalent structured execution intent.

Execution grammar does not define:

- stabilizer codes;
- surface codes;
- decoding;
- syndrome extraction;
- logical-qubit construction;
- correction circuits;
- decoder algorithms.

Those remain owned by QEC.

The semantic result may be consumed by:

quantum::ir
    ↓
QEC
    ↓
routing
    ↓
scheduling
    ↓
ZQN
    ↓
HAL

---

33. ZQN Integration

ZQN owns quantum noise/fault semantics.

Execution may express requirements such as:

requires: capability("noise_aware_execution");

or structured fault/noise policy.

Execution must not duplicate:

- noise models;
- fault models;
- correlated faults;
- leakage;
- erasure;
- loss;
- calibration semantics;
- noise channels.

The execution layer expresses desired execution properties.

ZQN determines how those properties are modeled and realized.

---

34. Resilience Integration

Resilience may consume execution intent such as:

retry
recover
resume
restart
rollback
reroute
reschedule
recompile
reoptimize
switch_backend
quarantine
abort

The grammar describes policy.

It does not implement the policy.

The runtime/resilience subsystem determines the actual mechanism.

The canonical resilience states discussed for Zamani remain semantic/runtime concepts:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

The execution grammar may refer to such states through the semantic resilience model without duplicating its implementation.

---

35. Recovery Integration

Recovery intent belongs at the execution boundary, but recovery mechanisms remain downstream.

Recovery may include:

- retry;
- resume;
- restart;
- rollback;
- restore;
- recompile;
- reroute;
- reschedule;
- backend switching;
- degraded execution;
- escalation;
- rejection.

The known Zamani recovery outcome vocabulary remains:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

These are semantic outcomes, not parser algorithms.

The grammar must not hard-code a universal retry count.

---

36. Checkpoint Integration

Checkpointing must distinguish different kinds of execution state.

Possible semantic categories include:

classical_state
compiled_state
logical_state
measurement_boundary
reconstructible_state
provider_supported_state
runtime_checkpoint

The grammar must not assume that arbitrary quantum state can always be serialized.

For example, checkpoint support may depend on:

capability("quantum.checkpoint")

or another canonical capability contract.

The runtime determines whether the requested checkpoint is realizable.

---

37. Lifecycle Integration

Lifecycle syntax may express:

start
run
pause
resume
stop
cancel
complete
restart
recover

Lifecycle syntax describes intent.

The runtime owns lifecycle implementation.

Lifecycle must be compatible with:

entrypoints
dispatch
runtime
resilience
recovery
checkpointing
observability
deployment

Lifecycle states must not imply a particular operating system or runtime engine.

---

38. Observability Integration

Observability intent may express requests for:

- metrics;
- logs;
- events;
- execution state;
- resource observations;
- capability observations;
- fault observations;
- performance measurements.

The grammar must not itself collect telemetry.

Runtime/observability infrastructure owns collection.

Sensitive information must remain subject to security and privacy policy.

---

39. Tracing Integration

Tracing intent belongs to:

tracing

and may express:

- trace enablement;
- trace scope;
- trace categories;
- trace sampling intent;
- correlation;
- distributed trace relationships;
- quantum/classical execution correlation.

The grammar does not implement tracing.

The runtime/tooling layers perform collection.

Tracing must preserve source spans where applicable so execution events can be associated with source-level constructs.

---

40. Profiling Integration

Profiling intent may express:

- performance profiling;
- memory profiling;
- resource profiling;
- accelerator profiling;
- quantum execution profiling;
- communication profiling;
- scheduling profiling.

The grammar does not measure performance.

The runtime/compiler/tooling layers perform measurement.

Profiling requests must not alter the semantic meaning of the program unless explicitly specified by a language-level execution policy.

---

41. HDL and Hardware Integration

Execution grammar integrates with:

grammar/hdl/
grammar/hardware/
grammar/resources/

Execution may express:

requires: capability("accelerator");
requires: capability("hardware.compute");
prefer: target("programmable_logic");

But execution does not define:

- wires;
- ports;
- pins;
- registers;
- clock trees;
- FPGA tiles;
- ASIC cells;
- physical memory banks;
- device routing.

Those remain owned by HDL/hardware.

---

42. Hardware/Software Co-Design

Zamani must allow one source program to describe computation and execution intent across:

software
+
accelerator
+
memory
+
communication
+
hardware

The execution grammar provides the execution contract.

The HDL/hardware grammars provide hardware intent.

They meet through semantic analysis and canonical IR contracts.

Execution MUST NOT duplicate HDL syntax.

HDL MUST NOT redefine execution lifecycle syntax.

---

43. AI/ML Integration

Execution must integrate with:

grammar/ai/
grammar/data/

It must support execution intent for:

- training;
- inference;
- tensor computation;
- model deployment;
- distributed training;
- accelerator execution;
- pipelines.

It must not encode:

- CUDA-specific syntax;
- ROCm-specific syntax;
- framework-specific parser rules;
- a fixed accelerator model.

Framework-specific realization belongs downstream.

---

44. Data Integration

Execution may apply to:

- datasets;
- streams;
- tables;
- tensors;
- data pipelines;
- transformations;
- queries.

Execution must not create another data-language grammar.

It consumes canonical data expressions and types.

---

45. Networking Integration

Execution may express:

- communication requirements;
- locality;
- bandwidth;
- latency;
- service placement;
- distributed execution;
- network capability requirements.

It must not define physical networking protocols.

Networking owns:

grammar/networking/

and the networking semantic/IR layers.

---

46. Security Integration

Execution may express:

- isolation;
- trust requirements;
- secure execution;
- capability requirements;
- authorization requirements;
- provenance;
- confidentiality/integrity constraints.

Security semantics remain owned by:

grammar/security/

Execution grammar must not bypass security policy.

No execution construct may implicitly:

- access secrets;
- bypass authorization;
- enumerate hardware;
- access physical memory;
- execute arbitrary commands.

---

47. Deployment Integration

"deployment.g4" describes deployment-related execution intent.

Deployment is distinct from:

hardware discovery
resource discovery
runtime implementation
scheduling
routing

The conceptual relationship is:

Program
  ↓
Execution intent
  ↓
Compilation
  ↓
Realization
  ↓
Deployment
  ↓
Runtime

Deployment details may be target-specific without making target-specific assumptions part of the core language.

---

48. Target Independence

Execution targets are abstract semantic realization contexts.

A target may represent:

- CPU-capable environment;
- GPU-capable environment;
- FPGA-capable environment;
- QPU-capable environment;
- simulator;
- embedded environment;
- distributed environment;
- cloud environment;
- edge environment;
- heterogeneous environment;
- future execution environment.

The grammar must not require the target to be known at parse time.

---

49. Target Resolution

Target resolution belongs downstream.

The flow is:

source target intent
        ↓
semantic validation
        ↓
capability resolution
        ↓
resource resolution
        ↓
target selection
        ↓
placement
        ↓
routing
        ↓
scheduling
        ↓
runtime realization

The parser must not perform target selection.

---

50. Placement Resolution

Placement syntax expresses:

where execution would preferably/necessarily occur

The placement subsystem determines:

where it can actually occur

This allows:

placement {
    requires: capability("quantum.measurement");
    prefer: locality::near;
}

without specifying a physical QPU.

---

51. Scheduling Resolution

Scheduling syntax expresses:

when/order/how strongly execution timing matters

The scheduler determines the actual execution schedule.

For example:

schedule {
    priority: priority_expression;
    deadline: deadline_expression;
    prefer: throughput;
}

does not prescribe the scheduler algorithm.

---

52. Runtime Capability Resolution

The runtime may discover:

available capabilities
available resources
current state
current load
current health

Execution grammar only expresses the required semantic contract.

The runtime must not mutate source semantics to fit a deficient environment.

---

53. Runtime Adaptation

Runtime adaptation may allow the implementation to:

- change placement;
- reschedule;
- migrate;
- retry;
- recover;
- select another compatible backend;
- change resource allocation;
- use degraded execution;
- recompile when explicitly permitted.

Adaptation MUST remain bounded by the program's semantic contract.

A hint can be changed.

A requirement cannot be silently ignored.

---

54. Determinism

The grammar must be deterministic.

Execution grammar must contain no:

- random parser behavior;
- filesystem access;
- network access;
- hardware queries;
- runtime callbacks;
- mutable external state;
- parser-time resource discovery;
- parser-time scheduling.

Identical token streams must produce identical parse structures.

Semantic analysis may depend on explicitly supplied compilation/runtime state, but that is not parser behavior.

---

55. Source Spans

Every execution AST construct must preserve source locations.

At minimum:

start
end

must be available through the repository's canonical source-span representation.

Execution diagnostics must be capable of pointing to:

- execution declaration;
- entry point;
- environment;
- requirement;
- constraint;
- preference;
- hint;
- scheduling clause;
- placement clause;
- lifecycle clause;
- recovery clause;
- checkpoint clause;
- observability clause.

Execution grammar must not invent a second source-location type.

---

56. AST Contract

The execution grammar maps into the repository's domain-neutral frontend AST.

It must not create hardware-specific AST types merely because execution can target hardware.

Conceptual nodes may include:

ExecutionDecl
ExecutionContext
ExecutionEnvironment
ExecutionEntrypoint
ExecutionRequirement
ExecutionConstraint
ExecutionPreference
ExecutionHint
ExecutionCapability
ExecutionResource
ExecutionTargetIntent
ExecutionPlacement
ExecutionSchedule
ExecutionSynchronization
ExecutionDispatch
ExecutionLifecycle
ExecutionRecovery
ExecutionCheckpoint
ExecutionObservation
ExecutionTrace
ExecutionProfile
ExecutionDeployment

The exact Rust types must follow the repository's canonical AST naming and ownership conventions.

Execution must not introduce:

GpuExecutionAst
QpuExecutionAst
FpgaExecutionAst
CpuExecutionAst
PhysicalQubitExecutionAst

into the domain-neutral AST.

---

57. Semantic Contract

The semantic layer normalizes syntax into execution intent.

Conceptual categories are:

Requirement
Constraint
Preference
Hint
CapabilityRequirement
ResourceRequirement
TargetIntent
PlacementIntent
SchedulingIntent
SynchronizationIntent
LifecycleIntent
RecoveryPolicy
CheckpointPolicy
ObservabilityPolicy
TracingPolicy
ProfilingPolicy
DeploymentIntent

The semantic model must retain the distinction between:

mandatory
restrictive
advisory
informational
target-specific

Unknown properties must never silently become mandatory.

---

58. IR Contract

The execution grammar does not define a new universal execution IR.

Do not create:

ExecutionIR
UniversalExecutionIR
RuntimeIR
QuantumExecutionIR
PlacementIR

merely because execution syntax exists.

Execution semantics must lower into the repository's existing canonical semantic/IR architecture.

For quantum programs:

quantum::ir

remains canonical.

For classical programs, use the repository's classical IR.

For HDL/hardware, use the repository's HDL/hardware representation.

Execution metadata may accompany or constrain those canonical representations.

---

59. Canonical Quantum IR Boundary

This is mandatory:

execution grammar
      ↓
semantic execution intent
      ↓
quantum semantic model
      ↓
quantum::ir

Not:

execution grammar
      ↓
execution quantum IR
      ↓
quantum::ir

There must be no competing quantum IR.

---

60. Compiler Integration

The compiler consumes execution semantics after parsing and semantic analysis.

Compiler responsibilities include:

- specialization;
- target resolution;
- optimization;
- lowering;
- routing;
- scheduling;
- resource planning;
- resilience planning;
- backend selection;
- artifact generation.

The grammar must not perform these operations.

---

61. Runtime Integration

The runtime consumes a validated compiled realization.

Runtime responsibilities include:

- resource acquisition;
- lifecycle;
- dispatch;
- monitoring;
- adaptation;
- recovery;
- observability;
- tracing;
- profiling;
- execution;
- completion;
- error reporting.

The runtime must not depend on parser implementation details.

---

62. Hardware Integration

Hardware discovery occurs after semantic validation.

The hardware subsystem determines:

actual capabilities
actual resources
actual topology
actual health
actual calibration
actual availability

The execution grammar does not.

---

63. Resource Availability

Execution may be valid while resources are unavailable.

For example:

requires: memory >= required_memory;

does not imply:

memory is currently available

The correct distinction is:

program requirement
        ≠
current resource availability

Resource feasibility is determined downstream.

---

64. Failure Classification

Execution diagnostics should distinguish at least:

SyntaxError
SemanticError
TypeError
CapabilityError
ResourceRequirementError
TargetCompatibilityError
PlacementError
SchedulingError
RuntimeError
DeploymentError
RecoveryError
CheckpointError
ObservabilityError

A resource shortage must not be reported as a parser syntax error.

A hardware failure must not be represented as malformed grammar.

---

65. Error Recovery

Parser recovery must remain local and deterministic.

Execution grammar must provide enough structural boundaries for useful recovery:

{
    ...
}
;

and similar canonical delimiters.

A malformed execution clause must not cause arbitrary consumption of unrelated program constructs.

---

66. Security Contract

Parsing execution syntax MUST NOT:

- enumerate hardware;
- discover devices;
- access physical addresses;
- allocate resources;
- reserve resources;
- open network connections;
- read arbitrary files;
- invoke commands;
- load drivers;
- access secrets;
- bypass authorization;
- bypass capability checks;
- mutate runtime state.

Execution semantics are data.

Runtime operations happen only after validation and authorization.

---

67. Rust Contract

The grammar itself contains no Rust execution logic.

The implementation must target:

Rust 1.97
Rust 1.97.1
Rust 2021

Production Rust code must use safe Rust.

"unsafe" is prohibited.

The execution grammar must not require:

- unsafe FFI;
- unsafe parser actions;
- unsafe runtime callbacks;
- unsafe memory manipulation.

Where external systems require unsafe internals, those must remain outside the grammar contract and outside the production execution-language implementation boundary.

---

68. Parser Technology Contract

The repository currently contains a handwritten Rust lexer/parser as well as ANTLR grammar infrastructure.

This must be treated as an implementation-conformance problem, not as permission to create two languages.

The authority remains:

specification
    ↓
Zamani.g4

The handwritten parser and ANTLR-generated parser must converge on the same language contract.

They must not silently accept different execution languages.

Conformance tests must compare them where both are production-supported.

---

69. No Parser-Time Runtime Behavior

The parser must never:

start runtime
allocate thread
allocate GPU
allocate QPU
discover CPU
discover memory
discover FPGA
discover network
perform scheduling
perform routing
perform QEC
perform calibration

Parser output is data.

Semantic analysis validates data.

Compilation transforms validated semantics.

Runtime executes compiled semantics.

---

70. Compatibility

Execution grammar must preserve compatibility with existing source forms wherever those forms are already part of the accepted language.

When syntax is changed:

old syntax
    ↓
compatibility mapping
    ↓
canonical semantic representation

must be defined.

The repository's compatibility system remains authoritative for:

- versions;
- migrations;
- deprecated constructs;
- feature gates;
- compatibility profiles.

Do not silently break existing execution syntax.

---

71. Historical Syntax

"grammar/Zamani-Grammar.md" may contain older execution concepts.

Such concepts are not automatically legal.

Each feature must have an explicit status:

stable
implemented
partial
proposed
experimental
planned
deprecated
historical
not implemented

A historical execution concept becomes production only through:

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

72. Feature Completion Contract

Every execution grammar file must be independently completable.

For each file, the following must be known before implementation begins:

File
Purpose
Status
Owns
Does Not Own
Dependencies
Upstream Contracts
Downstream Consumers
Lexical Contract
Grammar Contract
AST Contract
Semantic Contract
IR Contract
Compiler Integration
Runtime Integration
Cross-Domain Integration
Diagnostics
Security
Compatibility
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Determinism Tests
Hard-Coding Audit
Completion Criteria

This prevents:

finish file A
change file B
return to file A
rewrite file A

The integration contract is established first.

---

73. "execution.g4"

"execution.g4" is the execution-domain composition component.

It owns:

- execution-level dispatch;
- execution-domain composition;
- execution declaration/statement routing;
- integration of specialized execution grammars.

It must not absorb every specialized rule.

It integrates:

execution-context.g4
dispatch.g4
placement.g4
scheduling.g4
synchronization.g4
runtime-capabilities.g4
parallel-execution.g4
deployment.g4
runtime.g4
entrypoints.g4
environments.g4

and any additional canonical execution components.

Its completion criteria are:

- all execution constructs have a single dispatch path;
- no duplicate rule ownership;
- no duplicate expression grammar;
- no duplicate type grammar;
- no duplicate resource grammar;
- no duplicate capability grammar;
- no second execution root;
- root "Zamani.g4" can reach execution syntax;
- AST mappings are documented;
- semantic mappings are documented;
- tests exist for every public execution entry.

---

74. "execution-context.g4"

Owns execution-context syntax.

It represents contextual execution intent without becoming a hardware target description.

Integration:

execution-context
    ↓
domain-neutral AST
    ↓
semantic execution context
    ↓
capability/resource/target analysis

It must not allocate resources.

---

75. "entrypoints.g4"

Owns entry-point syntax.

Must support scalable:

- parameters;
- return values;
- attributes;
- requirements;
- capabilities;
- execution policies.

No fixed number of entry points or parameters may be encoded.

Integration:

entrypoint
    ↓
AST
    ↓
name/type validation
    ↓
execution contract
    ↓
compiler entry artifact
    ↓
dispatch/runtime

---

76. "environments.g4"

Owns reusable execution-environment declarations.

Integration:

environment
    ↓
AST
    ↓
environment semantic contract
    ↓
capability/resource/target analysis
    ↓
compiler/runtime environment resolution

It must remain distinct from deployment and physical hardware.

---

77. "dispatch.g4"

Owns dispatch intent.

Integration:

dispatch
    ↓
AST
    ↓
semantic dispatch contract
    ↓
compiled realization
    ↓
runtime dispatch

No runtime call may occur during parsing.

---

78. "placement.g4"

Owns placement intent.

It must support scalable:

- locality;
- affinity;
- anti-affinity;
- co-location;
- separation;
- replication;
- migration;
- elasticity;
- target intent.

It must remain hardware-independent.

---

79. "scheduling.g4"

Owns scheduling intent.

It may represent:

- priority;
- deadline;
- latency;
- throughput;
- order;
- dependency;
- temporal intent;
- alignment;
- scheduler policy.

It must not implement scheduling algorithms.

---

80. "synchronization.g4"

Owns synchronization intent.

It integrates with:

concurrency
distributed
memory
execution
runtime

It must not become a second concurrency model.

---

81. "runtime-capabilities.g4"

Owns source syntax for runtime capability requirements.

It integrates with:

resources/
hardware/
security/
quantum/
classical/
distributed/
runtime

Capability names remain open-ended.

---

82. "parallel-execution.g4"

Owns parallel execution intent.

It integrates with:

grammar/concurrency/
grammar/distributed/
grammar/classical/
grammar/quantum/
grammar/hardware/
grammar/resources/

No fixed worker count is permitted.

---

83. "deployment.g4"

Owns deployment intent.

It integrates with:

compile/
hardware/
resources/
distributed/
networking/
runtime/

It must not become a cloud-provider-specific deployment language.

---

84. "runtime.g4"

Owns runtime intent.

It integrates with:

entrypoints
environments
dispatch
lifecycle
recovery
checkpointing
observability
tracing
profiling
resilience
deployment

Runtime grammar does not implement runtime behavior.

---

85. Lifecycle Component

If "lifecycle.g4" is retained as a dedicated file, it owns:

start
run
pause
resume
stop
cancel
restart
recover
complete

Integration:

lifecycle
    ↓
AST
    ↓
semantic lifecycle policy
    ↓
runtime

No operating-system lifecycle assumptions are permitted.

---

86. Recovery Component

If "recovery.g4" is retained as a dedicated file, it owns source-level recovery intent.

It integrates with:

resilience
runtime
checkpointing
scheduling
placement
quantum/QEC/ZQN where applicable

It does not implement recovery algorithms.

---

87. Checkpointing Component

If "checkpointing.g4" is retained as a dedicated file, it owns checkpoint intent.

It integrates with:

runtime
memory
quantum
resilience
recovery
storage
deployment

It must distinguish checkpoint semantics from physical serialization.

---

88. Resilience Component

If "resilience.g4" is retained as a dedicated file, it owns execution-level resilience policy syntax.

It integrates with:

runtime
recovery
QEC
ZQN
hardware
scheduling
placement

It does not own resilience algorithms.

---

89. Observability Component

If "observability.g4" is retained as a dedicated file, it owns observability intent.

It integrates with:

runtime
tracing
profiling
security
diagnostics

No telemetry is collected during parsing.

---

90. Tracing Component

If "tracing.g4" is retained as a dedicated file, it owns tracing intent.

It integrates with:

observability
runtime
distributed
diagnostics
source spans

Trace data collection remains a runtime/tooling responsibility.

---

91. Profiling Component

If "profiling.g4" is retained as a dedicated file, it owns profiling intent.

It integrates with:

observability
runtime
compiler
hardware
quantum
classical
distributed

Profiling must not change program semantics unless the language specification explicitly defines such behavior.

---

92. Execution Metadata

Execution metadata must remain distinguishable from executable semantics.

Metadata may include:

- labels;
- annotations;
- tracing names;
- profiling names;
- provenance;
- documentation;
- tooling hints.

Metadata must not silently become a resource requirement.

---

93. Namespaces and Extensions

Execution properties should support qualified names where appropriate.

For example:

quantum::execution::property
vendor::extension::property
future::execution::property

Namespaced extensions must not require changes to the universal parser for every new ecosystem concept.

However, semantic registration and compatibility rules must still apply.

---

94. Dialect Integration

Execution grammar integrates with:

grammar/dialects/

A dialect may extend execution semantics only through an explicit dialect contract.

A dialect must identify:

name
version
syntax extensions
semantic extensions
AST mapping
IR mapping
capabilities
compatibility
feature gates

A dialect must not silently become part of core Zamani.

---

95. Macros

Execution constructs may be generated by macros.

However:

macro expansion
    ↓
normal parsing/validation
    ↓
semantic analysis

must still occur.

Macros must not bypass execution safety or semantic validation.

---

96. Metaprogramming

Metaprogramming may construct execution syntax or execution policies.

The final expanded program must still pass:

- syntax validation;
- type validation;
- capability validation;
- resource validation;
- portability validation;
- security validation.

---

97. Portability Classes

Execution semantics should distinguish at least:

portable
target-constrained
target-specific
implementation-specific
non-portable

A concrete hardware binding must not silently be represented as portable.

The compiler may warn or reject non-portable execution intent according to the selected language profile.

---

98. Requirements Must Not Be Hidden in Hints

This is invalid semantic behavior:

hint: requires_quantum

if the runtime interprets the hint as mandatory.

The semantic category must be explicit.

Correct:

requires: capability("quantum");

or:

prefer: accelerator("quantum");

depending on intent.

---

99. Resource vs Capability

These must remain distinct.

Resource:

requires: qubits >= logical_qubits;

Capability:

requires: capability("quantum.mid_circuit_measurement");

A target may have:

many resources

without having:

the required capability

and may have the capability without enough resources.

Both analyses are required.

---

100. Constraint Solving Boundary

Execution grammar represents constraints.

It does not solve them.

For example:

requires: memory >= required_memory;
constraint: latency <= deadline;

The compiler/resource planner determines feasibility.

The parser only verifies syntax.

Semantic analysis verifies expression/type validity.

---

101. Runtime State Boundary

Runtime state must never be embedded into source syntax implicitly.

Examples of runtime state include:

current device load
current queue length
current calibration
current health
current memory pressure
current network state
current QPU availability

These belong to runtime/HAL/resource discovery.

Source may request policies about them.

It does not directly observe them during parsing.

---

102. Resilience and Current State

The execution model must permit runtime adaptation without changing source semantics.

For example:

if target becomes unavailable:
    recover

is an execution policy.

The actual decision is runtime/resilience behavior.

The grammar does not implement the decision tree.

---

103. No Hidden Retry Limits

The execution language must never silently impose:

retry = 3

or:

max_retries = 5

unless explicitly defined by a source-level program or policy.

Even when a source declares a retry count, the runtime must treat it as a semantic policy and validate it according to the language specification.

---

104. No Hidden Timeout Limits

Likewise, the language implementation must not secretly assume:

maximum execution time = N

A timeout is a program/environment policy.

If no timeout is declared, implementation policy may still impose an operational limit, but that must not be represented as a language semantic limit.

---

105. No Hidden Queue Limits

The grammar must not assume a maximum number of:

- tasks;
- jobs;
- stages;
- dispatches;
- queues;
- dependencies.

Repeated structures are unbounded at the language level.

---

106. No Hidden Timeline Limits

For multi-timeline or temporal execution, the grammar must not assume:

MAX_TIMELINES
MAX_BRANCHES
MAX_FORKS

Timeline cardinality is determined by the program and available resources.

---

107. Nano and Future Computing

Execution grammar must remain compatible with future domains, including nano-oriented computation and computational models not yet known.

The mechanism is:

open semantic names
+
capability model
+
resource model
+
target abstraction
+
dialect mechanism

rather than continually adding parser keywords.

---

108. Interoperability

Execution must integrate with:

grammar/interoperability/

External formats such as:

- QASM;
- QIR;
- LLVM-related representations;
- MLIR-related representations;
- HDL formats;
- foreign-function interfaces;

are interoperability boundaries.

They do not replace Zamani's canonical semantic model.

---

109. Diagnostics

Execution diagnostics must be:

- deterministic;
- source-located;
- structured;
- actionable;
- domain-aware;
- portable.

Examples:

missing execution body
unknown execution property
invalid requirement expression
invalid capability expression
invalid resource expression
conflicting execution constraints
unsupported execution capability
incompatible target
unsatisfied resource requirement
invalid placement
invalid scheduling policy
invalid lifecycle transition
unsupported checkpoint operation
unsupported recovery policy
non-portable target binding

Diagnostics must distinguish:

syntax failure

from:

semantic failure

and:

resource infeasibility

and:

runtime failure

---

110. Positive Tests

Every public execution grammar construct requires positive tests.

At minimum:

minimal execution
execution with environment
execution with entrypoint
execution with capability
execution with resource
execution with requirement
execution with constraint
execution with preference
execution with hint
execution with placement
execution with scheduling
execution with synchronization
execution with dispatch
execution with lifecycle
execution with recovery
execution with checkpoint
execution with observability
execution with tracing
execution with profiling
execution with deployment

---

111. Negative Tests

Negative tests must cover:

missing execution body
malformed property
missing separator
missing terminator
invalid expression
invalid qualified name
invalid lifecycle transition
invalid resource expression
invalid capability expression
conflicting requirements
invalid target syntax
invalid placement syntax
invalid scheduling syntax
invalid recovery syntax
invalid checkpoint syntax

The parser must reject malformed source deterministically.

---

112. Boundary Tests

Boundary tests must include:

empty valid structures where permitted
one property
many properties
deeply nested expressions
deeply qualified names
large value expressions
large requirement sets
large capability sets
large resource sets
large placement sets
large scheduling sets
large distributed execution descriptions
large quantum execution descriptions
large classical execution descriptions
large HDL/co-design descriptions

No boundary test may establish a universal machine maximum.

---

113. Scalability Tests

Scalability testing must progressively increase:

program size
number of execution declarations
number of entry points
number of requirements
number of capabilities
number of resources
number of placement relationships
number of scheduling constraints
number of dependencies
number of distributed components
number of quantum resources
number of classical resources
number of hardware resources

The test harness must distinguish:

language capacity

from:

test machine capacity

A test machine running out of memory is not a language-level grammar limit.

---

114. Determinism Tests

Identical source input must produce identical:

- tokens;
- parse tree;
- source spans;
- syntax diagnostics;
- AST shape.

Where semantic inputs are identical, semantic execution intent must also be deterministic.

---

115. Compatibility Tests

Compatibility tests must verify:

existing execution syntax
new canonical syntax
deprecated syntax
migration behavior
dialect behavior
feature gates
ANTLR parser behavior
handwritten parser behavior
AST behavior
semantic behavior

No execution change is complete without compatibility analysis.

---

116. Hard-Coding Audit

Every execution grammar file must pass a hard-coding audit.

Search for:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_JOBS
MAX_STAGES
MAX_TIMELINES
MAX_RETRIES

Also inspect semantic equivalents such as:

supports exactly 8
maximum of 32
only 4 devices
requires 16 cores
always uses GPU 0
always uses QPU 0

The audit must distinguish:

explicit program value

from:

language implementation limit

An explicit user value is not automatically a prohibited hard-coded language limit.

---

117. Safe Rust Audit

Every implementation touching execution must be checked for:

unsafe
unsafe fn
unsafe block
unsafe trait
unsafe impl
raw-pointer-based runtime assumptions

Production execution infrastructure must remain compatible with the repository requirement:

safe Rust only

---

118. Performance

The execution grammar should remain parser-efficient.

Do not introduce unnecessary:

- exponential ambiguity;
- semantic predicates;
- parser-time lookups;
- backtracking dependencies;
- runtime calls;
- cross-file I/O.

Large execution specifications should remain parseable without requiring hardware discovery.

---

119. Memory Safety

No execution grammar feature may require unsafe memory handling.

Large programs must be handled through normal safe Rust resource management.

The grammar must not impose artificial language limits merely to compensate for an unsafe implementation.

Implementation-level resource controls must remain separate from language semantics.

---

120. Error Isolation

An invalid execution construct must not corrupt unrelated AST domains.

For example, malformed placement must not alter:

- quantum operations;
- classical expressions;
- HDL declarations;
- module declarations.

Parser recovery boundaries must be designed accordingly.

---

121. Cross-Domain Integration Matrix

Execution must integrate with:

Domain| Execution relationship
"core/"| names, blocks, attributes, common syntax
"types/"| typed execution values
"expressions/"| all execution expressions
"declarations/"| execution declarations
"statements/"| execution statements
"functions/"| entrypoints and callable execution
"modules/"| execution visibility/imports
"effects/"| execution effects
"memory/"| memory/resource intent
"concurrency/"| parallel execution
"classical/"| classical execution
"quantum/"| quantum execution
"hybrid/"| classical/quantum coordination
"hdl/"| hardware execution intent
"hardware/"| target realization
"resources/"| resource requirements
"distributed/"| distributed execution
"ai/"| model/training/inference execution
"data/"| data processing
"networking/"| communication
"security/"| secure execution
"compile/"| compilation and target realization
"interoperability/"| external execution formats
"dialects/"| controlled extensions
"validation/"| conformance
"compatibility/"| evolution
"tests/"| execution acceptance

---

122. Integration With "grammar/DESIGN.md"

"DESIGN.md" is authoritative for architecture.

This README specializes that architecture for execution.

If an execution feature appears to conflict with "DESIGN.md", the implementation must resolve the conflict at the architecture/specification level before adding syntax.

Do not solve architectural conflicts by adding another grammar rule.

---

123. Integration With "grammar/README.md"

"grammar/README.md" provides the repository-wide grammar authority model.

This README provides the execution-domain implementation contract.

Therefore:

grammar/README.md
        ↓
domain architecture
        ↓
execution/README.md
        ↓
execution grammar files

Execution must follow the repository-wide authority hierarchy.

---

124. Integration With "grammar/grammar.md"

"grammar.md" records implementation conformance.

Every execution feature must eventually be traceable there as:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

Execution README documentation must not mark a feature "STABLE" merely because the grammar file exists.

---

125. Integration With "grammar/Zamani-Grammar.md"

"Zamani-Grammar.md" remains the historical/extended design reference.

Execution concepts found there are candidates for promotion, not automatic syntax.

Promotion requires:

design
→ specification
→ AST contract
→ grammar
→ semantic implementation
→ IR integration
→ tests
→ stable

---

126. Integration With "src/lexer.rs"

The current handwritten lexer already defines a broad Zamani token vocabulary, including execution-relevant identifiers, operators, punctuation, literals, and keywords.

Execution grammar must consume canonical lexical concepts rather than creating duplicate token meanings.

Any execution keyword introduced into ANTLR must have a corresponding repository-wide lexical decision.

Open-ended semantic concepts should preferentially use identifiers/qualified names instead of requiring a new keyword.

---

127. Integration With "src/parser.rs"

The handwritten parser currently implements substantial Zamani syntax using recursive-descent/Pratt parsing.

Execution support added to the grammar must eventually have a corresponding parser-conformance story.

The parser must produce the same semantic language as the canonical grammar.

Execution-specific parser code must:

- preserve source spans;
- preserve deterministic parsing;
- reuse canonical expressions;
- reuse canonical names;
- avoid hardware discovery;
- avoid runtime calls;
- avoid unsafe Rust.

---

128. Integration With Frontend AST

The frontend AST must remain domain-neutral.

Execution syntax must map to generic execution-intent structures.

Do not make the AST dependent on:

- a particular CPU;
- a particular GPU;
- a particular QPU;
- a particular FPGA;
- a particular cloud provider;
- a particular scheduler implementation.

Domain-specific semantics are introduced after structural parsing.

---

129. Integration With Canonical Semantic Model

Execution semantic analysis combines:

execution intent
+
types
+
effects
+
resources
+
capabilities
+
constraints
+
target compatibility
+
portability

The resulting semantic model must remain target-independent until target resolution.

---

130. Integration With Optimization

Execution policies may constrain optimization.

For example:

require latency <= deadline

can influence optimization.

But optimization must not reinterpret the source requirement.

Optimization is allowed to change implementation while preserving semantic constraints.

---

131. Integration With Routing

Routing consumes placement/resource information.

For quantum execution:

logical execution
        ↓
quantum::ir
        ↓
placement/routing

Execution grammar must never directly produce physical mappings.

---

132. Integration With Scheduling

Scheduling consumes:

- execution constraints;
- temporal intent;
- dependencies;
- resource requirements;
- placement information;
- quantum timing constraints;
- hardware constraints.

Execution grammar supplies intent.

Scheduling implements realization.

---

133. Integration With Resilience

Resilience consumes:

- execution policy;
- runtime state;
- health;
- fault information;
- checkpoint state;
- resource availability.

Execution grammar expresses the allowed policy space.

Resilience selects and performs the actual recovery behavior.

---

134. Integration With ZQN

ZQN receives quantum/noise semantics after quantum IR and relevant execution policy are established.

Execution does not create another noise representation.

---

135. Integration With HAL

HAL is responsible for:

- device discovery;
- capabilities;
- resource state;
- topology;
- calibration;
- hardware access.

Execution grammar must never depend on HAL during parsing.

---

136. Integration With Runtime

The runtime consumes the validated compiled realization.

Runtime may implement:

dispatch
lifecycle
resource acquisition
monitoring
adaptation
recovery
checkpointing
observability
tracing
profiling
completion

The grammar only declares intent.

---

137. Integration With Deployment

Deployment determines how a validated executable realization becomes available in its intended environment.

Execution and deployment must share semantic contracts but not duplicate syntax unnecessarily.

---

138. Future-Proofing

A future accelerator should ideally require:

capability registration
resource model
target profile
backend

rather than:

new core keyword
new root grammar
new parser architecture

Likewise, a new quantum technology should be expressible through:

capability
resource
operation
target
dialect
IR lowering

without rewriting universal execution syntax.

---

139. What Execution Grammar Must Never Become

Execution grammar must never become:

- a scheduler;
- a runtime;
- a device manager;
- a hardware manager;
- a cloud deployment engine;
- a QEC engine;
- a noise simulator;
- a routing engine;
- a resource allocator;
- a driver interface;
- a vendor API;
- a second IR;
- a second language;
- a hardware inventory;
- a fixed machine model.

---

140. Production Acceptance Checklist

"grammar/execution/" is production-ready only when all of the following are true.

Architecture

- [ ] ownership is defined;
- [ ] non-ownership is defined;
- [ ] authority hierarchy is unambiguous;
- [ ] no competing root grammar exists;
- [ ] dependency direction is acyclic.

Lexing

- [ ] all tokens are canonical;
- [ ] no duplicate token concepts remain;
- [ ] keywords are justified;
- [ ] open-ended semantic names are supported.

Grammar

- [ ] all public execution constructs are defined;
- [ ] expression grammar is reused;
- [ ] type grammar is reused;
- [ ] name grammar is reused;
- [ ] no parser actions exist;
- [ ] no semantic predicates requiring external state exist.

AST

- [ ] every public rule has an AST mapping;
- [ ] AST is domain-neutral;
- [ ] source spans are preserved;
- [ ] no hardware-specific AST pollution exists.

Semantics

- [ ] requirements are distinct from constraints;
- [ ] constraints are distinct from preferences;
- [ ] preferences are distinct from hints;
- [ ] capabilities are distinct from resources;
- [ ] target intent is distinct from physical target;
- [ ] portability is explicit.

IR

- [ ] no competing execution IR exists;
- [ ] canonical semantic representation is used;
- [ ] "quantum::ir" remains canonical;
- [ ] classical IR integration is defined;
- [ ] HDL/hardware IR integration is defined.

Runtime

- [ ] runtime integration is defined;
- [ ] lifecycle is defined;
- [ ] dispatch is defined;
- [ ] recovery is defined;
- [ ] checkpointing is defined;
- [ ] observability is defined;
- [ ] tracing is defined;
- [ ] profiling is defined.

Quantum

- [ ] quantum execution integration exists;
- [ ] quantum resource requirements are open-ended;
- [ ] QEC boundary is defined;
- [ ] ZQN boundary is defined;
- [ ] routing boundary is defined;
- [ ] scheduling boundary is defined;
- [ ] no fixed gate enumeration is introduced by execution.

Classical

- [ ] classical execution integrates through common semantics;
- [ ] no CPU-specific universal limit exists.

HDL

- [ ] HDL execution intent integrates with hardware/HDL;
- [ ] no fixed physical hardware assumptions exist.

Distributed

- [ ] distributed execution is supported;
- [ ] no fixed node count exists;
- [ ] topology remains downstream.

Security

- [ ] no parser-time privileged operation exists;
- [ ] execution cannot bypass security policy.

Scalability

- [ ] no universal resource ceilings exist;
- [ ] no fixed topology exists;
- [ ] no fixed worker count exists;
- [ ] no fixed retry count exists;
- [ ] no fixed timeline count exists;
- [ ] no fixed device count exists.

Rust

- [ ] Rust 1.97 compatibility;
- [ ] Rust 1.97.1 compatibility;
- [ ] Rust 2021;
- [ ] no "unsafe";
- [ ] no unsafe parser/runtime contract.

Testing

- [ ] positive tests;
- [ ] negative tests;
- [ ] boundary tests;
- [ ] scalability tests;
- [ ] determinism tests;
- [ ] compatibility tests;
- [ ] diagnostics tests;
- [ ] hard-coding audit.

---

141. Definition of Done for Each Execution File

A file is not complete merely because its grammar parses.

A file is complete only when:

[ ] Purpose defined
[ ] Status defined
[ ] Ownership defined
[ ] Non-ownership defined
[ ] Dependencies defined
[ ] Upstream contracts defined
[ ] Downstream consumers defined
[ ] Lexer contract defined
[ ] Grammar contract defined
[ ] AST contract defined
[ ] Semantic contract defined
[ ] IR contract defined
[ ] Compiler integration defined
[ ] Runtime integration defined
[ ] Cross-domain integration defined
[ ] Source spans defined
[ ] Diagnostics defined
[ ] Security boundary defined
[ ] Compatibility defined
[ ] Positive tests defined
[ ] Negative tests defined
[ ] Boundary tests defined
[ ] Scalability tests defined
[ ] Determinism tests defined
[ ] Hard-coding audit completed
[ ] Rust 1.97 compatibility verified
[ ] Rust 1.97.1 compatibility verified
[ ] No unsafe requirement
[ ] Completion criteria satisfied

This is the required independent-first development rule.

---

142. Execution Domain Completion Matrix

File| Primary ownership| AST| Semantic| IR| Compiler| Runtime
"execution.g4"| composition| dispatch| execution intent| canonical semantic model| yes| yes
"execution-context.g4"| execution contexts| context| context contract| semantic model| yes| yes
"entrypoints.g4"| entrypoints| entrypoint| invocation contract| callable/executable representation| yes| yes
"environments.g4"| environments| environment| environment contract| semantic metadata| yes| yes
"dispatch.g4"| dispatch intent| dispatch| dispatch policy| execution metadata| yes| yes
"placement.g4"| placement intent| placement| placement contract| canonical metadata| routing input| runtime
"scheduling.g4"| scheduling intent| schedule| scheduling contract| scheduler input| scheduling| runtime
"synchronization.g4"| synchronization| synchronization| synchronization contract| canonical semantics| lowering| runtime
"runtime-capabilities.g4"| capability intent| capability| capability requirement| semantic metadata| target analysis| runtime
"parallel-execution.g4"| parallel intent| parallel execution| concurrency contract| canonical semantics| parallel lowering| runtime
"deployment.g4"| deployment intent| deployment| deployment contract| deployment representation| deployment| runtime
"runtime.g4"| runtime intent| runtime| runtime contract| semantic metadata| realization| runtime
"lifecycle.g4"| lifecycle| lifecycle| lifecycle policy| semantic metadata| realization| runtime
"resilience.g4"| resilience policy| resilience| resilience policy| semantic metadata| resilience planning| runtime
"recovery.g4"| recovery intent| recovery| recovery policy| semantic metadata| recovery planning| runtime
"checkpointing.g4"| checkpoint intent| checkpoint| checkpoint policy| semantic metadata| checkpoint planning| runtime
"observability.g4"| observability intent| observability| observation policy| semantic metadata| instrumentation| runtime
"tracing.g4"| tracing intent| tracing| tracing policy| semantic metadata| instrumentation| runtime
"profiling.g4"| profiling intent| profiling| profiling policy| semantic metadata| instrumentation| runtime

Where an existing file already owns a listed concern, a new duplicate file MUST NOT be created merely to follow this table.

---

143. Canonical Execution Integration Graph

The complete execution architecture is:

                         Zamani Source
                              |
                              v
                         Zamani.g4
                              |
                              v
                           Lexer
                              |
                              v
                           Parser
                              |
                              v
                       Domain-Neutral AST
                              |
              +---------------+---------------+
              |               |               |
              v               v               v
             Types          Effects        Resources
              |               |               |
              +---------------+---------------+
                              |
                              v
                       Capability Analysis
                              |
                              v
                       Execution Semantics
                              |
       +----------------------+----------------------+
       |                      |                      |
       v                      v                      v
   Classical             quantum::ir          HDL/Hardware
       |                      |                      |
       +----------------------+----------------------+
                              |
                              v
                         Optimization
                              |
          +-------------------+-------------------+
          |                   |                   |
          v                   v                   v
      Placement          Routing             Scheduling
          |                   |                   |
          +-------------------+-------------------+
                              |
                     Resilience / QEC / ZQN
                              |
                              v
                             HAL
                              |
                              v
                       Target Realization
                              |
                              v
                           Dispatch
                              |
                              v
                           Runtime
                              |
          +-------------------+-------------------+
          |                   |                   |
          v                   v                   v
      Lifecycle          Recovery           Observability
          |                   |                   |
          +-------------------+-------------------+
                              |
                              v
                         Deployment

No execution component may bypass this architecture.

---

144. POCO-REAF Acceptance Examples

The following concepts are valid because they express intent rather than fixed hardware:

requires: qubits >= logical_qubits;

requires: memory >= required_memory;

requires: capability("tensor.compute");

requires: capability("gpu.compute");

requires: capability("quantum.measurement");

prefer: accelerator("quantum");

prefer: accelerator("gpu");

constraint: latency <= required_latency;

constraint: memory >= required_memory;

hint: locality::near;

The following concepts must not become universal language assumptions:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

---

145. Portability Principle

The programmer should primarily describe:

what
why
requirements
capabilities
constraints
preferences
correctness

The compiler/runtime should determine:

where
when
how
which device
which backend
which topology
which physical resource
which schedule
which route

This separation is the central execution-language mechanism enabling POCO-REAF.

---

146. Final Production Contract

The execution grammar is production-ready only when the following statement is true:

«A valid Zamani execution construct can be parsed deterministically, represented in the domain-neutral AST, semantically validated, mapped into the canonical semantic/IR architecture, and consumed by compiler/runtime infrastructure without requiring the grammar to know the physical size, topology, vendor, architecture, or current availability of the target machine.»

For quantum programs:

«Execution semantics ultimately integrate with the canonical "quantum::ir"; execution grammar never creates a competing quantum IR.»

For classical programs:

«Execution semantics remain independent of processor count, instruction set, accelerator count, and physical memory size.»

For HDL/hardware:

«Execution semantics remain separate from physical hardware description and realization.»

For distributed programs:

«Execution semantics remain independent of a fixed node count or fixed network topology.»

For future computing models:

«New capabilities, resources, target classes, and dialects can be introduced without turning every new technology into a new core-language keyword or a new competing grammar.»

For scalability:

«Zamani's execution grammar imposes no artificial universal hardware ceilings. Actual execution is bounded by program semantics, representation limits, declared policies, and resources genuinely available to the compiler, runtime, and target.»

For safety:

«Production Rust integration targets Rust 1.97/1.97.1, Rust 2021, and safe Rust only. "unsafe" is not part of the execution grammar or its production implementation contract.»

Therefore the execution subsystem's architectural invariant is:

             ONE LANGUAGE
                  |
                  v
          ONE EXECUTION MODEL
                  |
                  v
       TARGET-INDEPENDENT INTENT
                  |
        +---------+---------+
        |         |         |
        v         v         v
    classical  quantum    hardware
        |         |         |
        +---------+---------+
                  |
                  v
          canonical semantics
                  |
                  v
                 IR
                  |
        +---------+---------+
        |         |         |
        v         v         v
     routing  scheduling resilience
        |         |         |
        +---------+---------+
                  |
             QEC / ZQN
                  |
                 HAL
                  |
                  v
         actual realization
                  |
                  v
               runtime

Execution grammar describes execution intent.
The compiler determines realization.
The runtime performs execution.
Hardware determines physical capability.
The program's semantics remain authoritative.

That is the execution-domain contract required for Zamani's:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

objective.