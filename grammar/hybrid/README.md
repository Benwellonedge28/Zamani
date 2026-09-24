Zamani Hybrid Grammar

Path: "grammar/hybrid/README.md"
Domain: Hybrid classical/quantum/accelerator/HDL/hardware computation
Status: Production architecture and integration contract
Language: Zamani
Grammar technology: ANTLR4
Compiler baseline: Rust 1.97 / Rust 1.97.1
Rust edition: Rust 2021
Safety: Safe Rust only; "unsafe" Rust is prohibited
Portability model: Program Once → Compile Once → Run Everywhere → Anywhere → Forever
Canonical quantum semantic boundary: "quantum::ir"

---

1. Purpose

"grammar/hybrid/" defines the hybrid-computation grammar domain of Zamani.

Hybrid computation means that one Zamani program can express computation whose semantic dependency graph crosses multiple computational domains, including:

- classical computation;
- quantum computation;
- classical control of quantum computation;
- quantum results consumed by classical computation;
- accelerator computation;
- CPU/GPU/FPGA/ASIC-oriented computation;
- hardware/software co-design;
- HDL-related computation;
- AI and tensor computation;
- distributed computation;
- data processing;
- networking;
- future computational domains.

Hybrid is not a second language.

Hybrid is not a second type system.

Hybrid is not a second expression language.

Hybrid is not a second resource language.

Hybrid is not a second quantum language.

Hybrid is not a second quantum IR.

Hybrid is a composition domain that connects otherwise independent Zamani language domains while preserving one language-wide semantic model.

The fundamental responsibility of this directory is therefore:

«Express relationships between computational domains without embedding the accidental limitations or topology of a particular machine.»

---

2. Governing Principle

Hybrid source code describes:

- what computation is performed;
- what domains participate;
- what values cross domain boundaries;
- what dependencies exist;
- what controls what;
- what results are produced;
- what capabilities are required;
- what resources are required;
- what constraints must hold;
- what preferences or hints may guide implementation.

Hybrid source code must not unnecessarily describe:

- which physical CPU executes an operation;
- which GPU executes an operation;
- which QPU executes an operation;
- which physical qubit is selected;
- which FPGA fabric is selected;
- which memory bank is selected;
- which network node is selected;
- which accelerator instance is selected;
- how quantum operations are routed;
- how operations are scheduled;
- how QEC is implemented;
- how noise is modeled;
- how calibration is performed;
- how hardware is discovered.

Those concerns belong to downstream semantic, compilation, runtime, hardware, routing, scheduling, resilience, ZQN, and HAL systems.

---

3. POCO-REAF Contract

Zamani hybrid syntax is governed by:

«"Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever"»

POCO-REAF means that a program expresses portable computational intent and can subsequently be realized on different targets when those targets satisfy the program's semantic requirements or an explicitly supported implementation strategy exists.

The same hybrid source model must be capable of representing computation ranging from:

tiny
  ↓
small embedded system
  ↓
single CPU
  ↓
multicore CPU
  ↓
GPU
  ↓
FPGA
  ↓
ASIC
  ↓
QPU
  ↓
heterogeneous system
  ↓
cluster
  ↓
distributed system
  ↓
cloud
  ↓
future computational substrates

The grammar does not promise that every target can execute every program.

It promises that target capacity is not accidentally turned into a language-level limit.

---

4. Meaning of "Scalable to Infinity"

"Infinity" is an architectural statement, not a claim that physical machines have infinite resources.

The grammar must not impose artificial finite limits on:

- hybrid regions;
- domain crossings;
- classical values;
- quantum values;
- qubits;
- registers;
- tensors;
- accelerators;
- GPUs;
- FPGAs;
- CPUs;
- cores;
- threads;
- nodes;
- processes;
- tasks;
- channels;
- memory;
- storage;
- tensor dimensions;
- tensor rank;
- network size;
- devices;
- timelines;
- operations;
- circuit depth;
- source size.

Actual limits belong to the implementation and target environment.

The correct model is:

Zamani program
    ↓
semantic requirements
    ↓
available capabilities/resources
    ↓
compiler planning
    ↓
target realization

not:

Zamani grammar
    ↓
today's hardware limits

---

5. Absolute Hard-Coding Prohibition

No hybrid grammar file may introduce universal limits such as:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_QPUS
MAX_ACCELERATORS
MAX_NODES
MAX_DEVICES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_TIMELINES
MAX_OPERATIONS

Equivalent disguised limits are also prohibited.

For example, the following concepts must not become universal grammar restrictions:

only 32 qubits
only 8 GPUs
only 16 nodes
only 64 threads
only 32-bit registers
only 24 GB accelerator memory
only 1024 tensor dimensions

A numeric literal in a Zamani program is allowed when it is program semantics.

For example:

let n = 1024;

does not constitute a hardware limit.

Likewise:

requires qubits >= n;

expresses a resource requirement rather than defining a maximum number of qubits.

---

6. Architecture

The hybrid grammar sits inside the following pipeline:

                    Zamani Source
                         │
                         ▼
                 Canonical Lexer
                         │
                         ▼
                 Canonical Parser
                         │
                         ▼
                  Hybrid Syntax
                         │
                         ▼
              Domain-Neutral AST
                         │
                         ▼
                Name Resolution
                         │
              ┌──────────┼──────────┐
              ▼          ▼          ▼
          Type System  Effects   Ownership
              │          │          │
              └──────────┼──────────┘
                         ▼
              Resource/Capability
                    Analysis
                         │
                         ▼
                Semantic Validation
                         │
                         ▼
              Canonical Semantic Model
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
      Classical      quantum::ir    HDL/Hardware
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                    Optimization
                         │
              ┌──────────┼──────────┐
              ▼          ▼          ▼
           Routing   Scheduling  Resilience
              │          │          │
              └──────────┼──────────┘
                         ▼
                        ZQN
                         │
                         ▼
                        HAL
                         │
                         ▼
                 Target Realization

Hybrid grammar is therefore an upstream syntax layer.

It must never become a hidden replacement for any downstream subsystem.

---

7. Ownership

"grammar/hybrid/" owns:

- hybrid-domain syntax;
- syntax for explicit cross-domain computation;
- syntax for domain-qualified operations where defined by the canonical grammar;
- syntax for classical/quantum dependency relationships;
- syntax for hybrid control relationships;
- syntax for hybrid dataflow;
- syntax for accelerator interoperability;
- syntax for hybrid resource intent;
- syntax needed to preserve cross-domain dependency information;
- hybrid-specific parser composition.

It does not own the implementation of those semantics.

---

8. Non-Ownership

The hybrid directory does not own:

8.1 Lexer

Owned by the canonical Zamani lexer vocabulary.

Hybrid files must not create competing lexer rules.

---

8.2 General expressions

Owned by:

grammar/expressions/

Hybrid syntax reuses the canonical expression system.

---

8.3 General types

Owned by:

grammar/types/

Hybrid does not create a separate hybrid type system.

---

8.4 General statements

Owned by:

grammar/statements/

Hybrid introduces only genuinely hybrid composition constructs.

---

8.5 Classical semantics

Owned downstream by the classical semantic/IR pipeline.

---

8.6 Quantum semantics

Owned downstream by the quantum semantic pipeline.

The canonical quantum semantic boundary remains:

quantum::ir

---

8.7 Quantum error correction

Owned by QEC/resilience infrastructure.

Hybrid syntax may express QEC-related intent where the language specification requires it, but it must not implement:

- code construction;
- syndrome extraction;
- decoding;
- correction;
- physical fault handling.

---

8.8 ZQN

ZQN owns quantum noise/fault semantics.

Hybrid grammar must not create a competing noise model.

---

8.9 Routing

Routing determines physical realization.

Hybrid grammar does not assign physical topology.

---

8.10 Scheduling

Scheduling determines execution order and timing.

Hybrid grammar does not assign physical timestamps.

---

8.11 Calibration

Calibration belongs downstream.

Hybrid grammar does not contain calibration algorithms.

---

8.12 HAL

HAL resolves logical requirements against actual hardware.

Hybrid grammar does not perform hardware discovery.

---

8.13 Runtime

Runtime performs execution, synchronization, transport, recovery, monitoring, and dispatch.

Hybrid grammar only describes source-level intent.

---

9. Directory Contract

The hybrid directory must contain one authoritative README and only grammar components with independently defined ownership.

The intended structure is:

grammar/hybrid/
├── README.md
├── hybrid.g4
├── classical-quantum.g4
├── quantum-classical-control.g4
├── accelerator-interoperability.g4
├── hybrid-resources.g4
├── synchronization.g4
├── shared-data.g4
└── hybrid-functions.g4

Existing repository files should be retained when they already provide the required responsibility.

Do not create parallel replacements merely because a filename is inconvenient.

Where an existing filename does not conform to the ANTLR grammar-name/file-name requirement, the correction must be made deliberately at the repository/build level rather than silently documented as though both forms were canonical.

There must be exactly one authoritative grammar identity for each component.

---

10. "hybrid.g4"

Purpose

"hybrid.g4" is the hybrid composition grammar.

It combines hybrid-specific constructs without redefining universal language syntax.

Owns

- hybrid declaration composition;
- hybrid construct composition;
- hybrid region composition;
- hybrid invocation composition;
- hybrid-domain composition;
- integration points for hybrid resources;
- integration points for hybrid functions;
- integration points for hybrid synchronization/dataflow.

Does not own

- lexer rules;
- general expressions;
- general statements;
- general types;
- quantum gates;
- quantum measurement semantics;
- resource implementation;
- accelerator implementation;
- physical hardware;
- AST definitions;
- IR definitions.

Required integration direction

The dependency direction must remain:

leaf/domain grammar
       ↓
Hybrid
       ↓
ZamaniParser
       ↓
Zamani

Never:

Hybrid
   ↓
ZamaniParser

The hybrid grammar must not import the completed parser composition root.

---

11. Canonical Lexer Integration

Hybrid grammar components consume the canonical lexer vocabulary.

The canonical lexer is the single lexical authority.

Hybrid must not define:

- hybrid-specific lexer aliases;
- parser-local keyword tokens;
- duplicate operator tokens;
- duplicate punctuation tokens;
- vendor/device tokens;
- quantum gate tokens.

If a future language version promotes a contextual word to a globally reserved keyword, that change must happen in the canonical lexical specification and lexer.

Hybrid syntax must not silently create a second lexical policy.

---

12. Contextual "hybrid" Marker

The current repository design treats "hybrid" as a contextual identifier rather than introducing an unnecessary new lexer token.

That policy may remain valid.

If the source form is:

hybrid {
    ...
}

then the grammar may recognize the lexical identifier and semantic validation must establish that its spelling is exactly:

hybrid

This is preferable to inventing a hybrid-only lexical vocabulary.

If a future language version reserves "hybrid", that is a language-version lexical change.

The hybrid grammar must not contain two parallel mechanisms for the same source spelling.

---

13. "classical-quantum.g4"

Purpose

Defines source-level relationships where classical and quantum computation exchange information.

It must support the semantic model:

classical value
      ↓
quantum operation

and:

quantum operation
      ↓
measurement/result
      ↓
classical value

It may also support:

classical
   ↓
quantum
   ↓
measurement
   ↓
classical
   ↓
quantum

without imposing a fixed number of crossings.

Owns

- classical-to-quantum invocation relationships;
- quantum-to-classical result relationships;
- parameter passing across the classical/quantum boundary;
- result binding;
- cross-domain dependency syntax;
- explicit hybrid data dependencies.

Does not own

- general classical expressions;
- quantum operation definitions;
- measurement implementation;
- quantum state simulation;
- type checking;
- QEC;
- scheduling;
- routing;
- hardware realization.

---

14. Classical-to-Quantum Contract

A classical value may become:

- a quantum operation parameter;
- a control value;
- a measurement configuration;
- a runtime parameter;
- a semantic resource expression.

The grammar preserves the relationship.

Semantic analysis determines whether the value is legal.

For example, conceptually:

quantum::operation(parameter);

may carry a classical parameter into a quantum semantic operation.

The grammar does not determine:

- how that value is encoded physically;
- which control electronics represent it;
- whether the target supports the operation;
- whether compilation specializes it;
- whether the parameter is static or dynamic.

Those are downstream decisions.

---

15. Quantum-to-Classical Contract

A quantum result may cross into classical computation through mechanisms such as measurement.

The grammar must preserve:

quantum producer
      ↓
result
      ↓
classical consumer

The semantic layer determines:

- result type;
- ownership;
- lifetime;
- measurement semantics;
- synchronization requirements;
- capability requirements;
- whether the operation is statically resolvable;
- whether runtime feed-forward is required.

The grammar must not implement measurement.

---

16. "quantum-classical-control.g4"

Purpose

Defines classical control relationships over quantum computation.

It must support dynamic relationships such as:

measurement
    ↓
classical condition
    ↓
quantum operation

and:

classical computation
    ↓
condition
    ↓
quantum region

Owns

- control expressions attached to hybrid computation;
- measurement-dependent control structure;
- dynamic classical/quantum dependency syntax;
- hybrid branch composition.

Does not own

- general control-flow semantics;
- boolean type semantics;
- measurement implementation;
- target capability detection;
- scheduling;
- runtime dispatch.

---

17. Dynamic-Control Semantics

A target may or may not support a particular dynamic control construct.

That fact must not be hard-coded into the grammar.

The correct pipeline is:

source syntax
     ↓
AST
     ↓
semantic validation
     ↓
capability analysis
     ↓
target-aware lowering

For example, a target without a required dynamic-control capability may cause:

- a compilation diagnostic;
- a transformation;
- a fallback implementation;
- a rejected target realization.

It must not cause the grammar itself to declare that the source language has only the capabilities of that target.

---

18. "accelerator-interoperability.g4"

Purpose

Defines portable interaction with accelerator domains.

The accelerator model must remain open to:

- GPU;
- FPGA;
- ASIC accelerator;
- tensor accelerator;
- AI accelerator;
- DSP;
- quantum-control accelerator;
- future accelerator classes.

Owns

- accelerator computation regions;
- accelerator invocation relationships;
- accelerator dataflow boundaries;
- accelerator capability requirements;
- accelerator interoperability intent.

Does not own

- CUDA syntax as core Zamani syntax;
- ROCm syntax as core Zamani syntax;
- FPGA vendor primitives;
- ASIC-specific instructions;
- device identifiers;
- physical memory addresses;
- physical topology;
- device inventory.

---

19. Accelerator Portability

The source program may express:

requires capability("accelerated.compute");

or an equivalent canonical resource/capability construct.

It must not require the source language to know that the machine contains:

GPU 0
GPU 1
GPU 2

or any other fixed inventory.

The compiler and runtime determine realization.

---

20. "hybrid-resources.g4"

Purpose

"hybrid-resources.g4" is an adapter to the canonical resource grammar.

It must not create a second resource language.

The universal resource authority remains under:

grammar/resources/

The relationship is:

Hybrid
   ↓
HybridResources
   ↓
Resources
   ↓
semantic resource model

---

21. Resource Concept Separation

The semantic system must distinguish:

requirement
constraint
capability
preference
hint
target
placement
resource

These are not interchangeable.

For example:

requires capability("quantum.measurement");

means that the program requires a capability.

It does not mean:

use qpu0

Likewise:

prefer capability("accelerated.compute");

is not equivalent to selecting a physical accelerator.

---

22. Requirements

Requirements are mandatory semantic intent.

Examples include:

requires qubits >= required_qubits;

or:

requires capability("quantum.measurement");

The exact canonical resource syntax must remain owned by "grammar/resources/".

HybridResources only adapts it.

---

23. Constraints

Constraints describe conditions that must remain satisfied.

Examples conceptually include:

constraint latency <= required_latency;

or an equivalent canonical resource expression.

Constraints must not silently become physical placement directives.

---

24. Preferences

Preferences are advisory.

They may guide:

- optimization;
- target selection;
- resource planning;
- scheduling;
- deployment.

A preference must not change program semantics merely because a compiler chooses not to satisfy it.

---

25. Hints

Hints provide implementation guidance.

Hints must be:

- explicitly advisory;
- semantically distinguishable from requirements;
- incapable of silently changing program meaning.

---

26. Capabilities

Capability names must be open-world semantic identifiers.

Examples:

quantum.measurement
quantum.mid_circuit_measurement
tensor.compute
accelerated.compute
distributed.collective
hardware.reconfiguration

The grammar must not enumerate every possible future capability.

Capability registries belong to the semantic/resource/target systems.

---

27. Targets

A target expression represents an abstract realization category or target constraint where the language specification permits it.

It must not automatically mean a concrete physical device.

The following distinction is mandatory:

target class

is different from:

physical device

and:

logical resource

is different from:

physical resource

---

28. "synchronization.g4"

Purpose

Defines source-level synchronization relationships required when multiple computational domains interact.

Synchronization may be required between:

- classical and quantum execution;
- CPU and accelerator execution;
- host and device;
- distributed domains;
- hardware and software;
- data producers and consumers.

Owns

- explicit synchronization constructs;
- source-level synchronization boundaries;
- dependency synchronization markers;
- ordering relationships where the language defines them.

Does not own

- physical clock implementation;
- device scheduling;
- timestamp allocation;
- transport implementation;
- runtime synchronization primitives.

---

29. Synchronization Semantics

A synchronization point means:

«A semantic dependency exists that downstream execution must preserve.»

It does not mean:

«Execute on machine X at physical time Y.»

The scheduler decides actual timing.

The runtime decides actual synchronization mechanisms.

The hardware layer decides physical realization.

---

30. "shared-data.g4"

Purpose

Defines hybrid dataflow where values participate in multiple computational domains.

The grammar must preserve:

producer
   ↓
value
   ↓
consumer

including:

classical → quantum
quantum → classical
classical → accelerator
accelerator → classical
quantum → accelerator
accelerator → quantum

where the corresponding semantics are supported.

Owns

- data-boundary syntax;
- hybrid dataflow constructs;
- explicit transfer intent;
- producer/consumer relationships.

Does not own

- physical DMA;
- PCIe;
- memory-controller implementation;
- device memory allocation;
- physical bus selection;
- network transport.

---

31. Data Movement

Hybrid source syntax may express that data must cross a computational boundary.

It must not hard-code how that movement occurs.

For example, source semantics may require:

value available to accelerator

without requiring:

PCIe

or:

specific memory bank

or:

specific device address

unless such a physical property is intentionally part of a target-specific dialect.

Portable Zamani remains target-independent.

---

32. "hybrid-functions.g4"

Purpose

Defines hybrid-specific function composition where one function can contain or coordinate multiple computational domains.

A hybrid function may conceptually:

accept classical input
        ↓
prepare quantum state
        ↓
execute quantum computation
        ↓
measure
        ↓
perform classical post-processing
        ↓
return classical result

or:

classical algorithm
        ↓
accelerator computation
        ↓
quantum computation
        ↓
classical result

Owns

- hybrid function markers;
- hybrid function composition;
- hybrid-specific function attributes;
- hybrid domain participation metadata.

Does not own

- general function syntax;
- parameter syntax;
- generic type syntax;
- calling convention implementation;
- ABI implementation.

General function constructs remain owned by:

grammar/functions/

---

33. Domain-Neutral AST Requirement

Hybrid grammar must map into the repository's domain-neutral frontend AST.

The AST must not become a mirror of every hybrid grammar filename.

Avoid structures such as:

HybridQuantumAst
HybridGpuAst
HybridFpgaAst
HybridQpuAst
HybridResourceAst

when the underlying semantics are already representable by canonical AST concepts.

Instead, hybrid syntax should preserve the information needed to represent:

- operation;
- producer;
- consumer;
- domain;
- operands;
- results;
- dependencies;
- controls;
- attributes;
- source spans;
- resource intent;
- capabilities.

---

34. AST Integration Contract

Every hybrid construct must have a predetermined mapping:

grammar rule
    ↓
AST representation
    ↓
semantic representation
    ↓
canonical IR

No hybrid grammar feature is complete merely because ANTLR can parse it.

Before a grammar rule is considered production-ready, its AST contract must already be defined.

---

35. Source Span Preservation

Every hybrid AST construct must preserve sufficient source-location information for diagnostics and tooling.

At minimum, semantic consumers must be able to identify:

- construct start;
- construct end;
- relevant operator/domain marker;
- relevant operands;
- relevant resource/capability expression.

Source spans must survive:

lexer
 ↓
parser
 ↓
AST
 ↓
semantic analysis
 ↓
IR/lowering

where diagnostics require the original source location.

---

36. Semantic Analysis Contract

Semantic analysis determines whether a syntactically valid hybrid construct is meaningful.

It is responsible for:

- domain compatibility;
- type compatibility;
- ownership;
- lifetime;
- effect compatibility;
- capability requirements;
- resource requirements;
- dependency legality;
- synchronization legality;
- classical/quantum boundary legality;
- accelerator compatibility;
- target-independent semantic validation.

The grammar should not encode these decisions as arbitrary parser restrictions.

---

37. Type Integration

Hybrid does not create a second type system.

It consumes canonical types.

Potential types may include:

classical values
quantum values
Qubit
Qubit[n]
Tensor<T, shape>
Memory<T, size>
resource types
capability types
function types
effectful types

The meaning of these types belongs to the canonical type/semantic system.

The hybrid grammar only provides syntax for their use where necessary.

---

38. Effects Integration

Cross-domain computation can introduce effects.

Examples include:

- quantum measurement;
- external device interaction;
- accelerator execution;
- synchronization;
- distributed communication;
- nondeterministic hardware interaction;
- resource acquisition.

Hybrid grammar must preserve the syntax necessary for semantic effect analysis.

The effect system determines whether a combination is legal.

Hybrid must not create a parallel effect model.

---

39. Ownership and Lifetime

Cross-domain values must have well-defined ownership/lifetime semantics.

For example:

classical value
      ↓
accelerator

must not leave the semantic model unable to determine:

- who owns the value;
- when it becomes available;
- when it can be released;
- whether it is copied or borrowed;
- whether it is immutable;
- whether synchronization is required.

The grammar itself does not perform ownership analysis.

It must, however, preserve the syntax needed for the semantic layer to perform it.

---

40. Quantum IR Invariant

Hybrid must never create a second quantum IR.

The mandatory quantum path is:

Zamani source
      ↓
domain-neutral AST
      ↓
semantic analysis
      ↓
quantum::ir

Then:

quantum::ir
      ↓
optimization
      ↓
routing
      ↓
scheduling
      ↓
QEC/resilience
      ↓
ZQN
      ↓
HAL
      ↓
target

Hybrid is not allowed to introduce:

HybridQuantumIR
HybridCircuitIR
HybridGateIR
HybridQpuIR

as competing canonical representations.

---

41. Classical IR Integration

Classical hybrid computation must lower through the repository's canonical classical representation.

Hybrid relationships must preserve dependencies between classical and quantum/accelerator regions.

The hybrid grammar does not define a new classical IR.

---

42. HDL and Hardware Integration

Hybrid programs may combine software computation with hardware intent.

The dependency model is:

Hybrid
   ↓
HDL/Hardware semantic model
   ↓
canonical hardware representation
   ↓
synthesis/lowering

Hybrid grammar must not introduce:

wire [31:0]

as a universal hardware assumption.

Widths may be program semantics when explicitly specified, but a particular width must never become a universal language capacity.

---

43. Resource Integration

Hybrid resource intent integrates with:

grammar/resources/
grammar/hardware/
grammar/compile/
grammar/execution/

The roles are:

grammar/hybrid/
        ↓
express hybrid resource intent
        ↓
resource semantic model
        ↓
capability analysis
        ↓
compiler planning
        ↓
hardware/runtime realization

Hybrid grammar does not discover available resources.

---

44. Capability Integration

Capabilities describe properties required by computation.

For example:

requires capability("quantum.measurement");
requires capability("tensor.compute");
requires capability("accelerated.compute");

The capability system determines whether the target environment satisfies them.

No finite capability list is embedded into hybrid grammar.

New capabilities must be addable without redesigning the hybrid grammar's foundational structure.

---

45. Routing Integration

Hybrid syntax must preserve logical dependencies needed by routing.

For quantum computation:

logical quantum operation
        ↓
quantum::ir
        ↓
routing

Routing decides physical realization.

Hybrid grammar does not specify:

physical_qubit(17)

as a universal mechanism.

A target-specific dialect may provide explicit physical mapping when required, but that must be explicitly identified as target-specific and must not contaminate portable core Zamani syntax.

---

46. Scheduling Integration

Hybrid syntax must preserve dependencies such as:

A
 ↓
measurement
 ↓
classical condition
 ↓
B

The scheduler can then determine a valid execution schedule.

The grammar must not assign:

- physical clock cycles;
- absolute timestamps;
- machine-specific pipeline slots;
- fixed execution intervals.

Those are downstream realization details.

---

47. QEC and Resilience Integration

Hybrid syntax may eventually express source-level intent such as:

requires fault_tolerance(...);
requires error_correction(...);
requires reliability(...);

but those expressions are semantic requirements.

QEC/resilience systems determine:

- code;
- logical encoding;
- syndrome handling;
- recovery;
- retry;
- degradation;
- escalation.

The hybrid grammar must not implement those mechanisms.

---

48. ZQN Integration

ZQN may consume semantic information concerning:

- noise;
- faults;
- execution conditions;
- reliability;
- quantum operation behavior;
- resilience requirements.

Hybrid grammar does not define a second noise model.

The pipeline remains:

hybrid source
     ↓
AST
     ↓
semantic model
     ↓
quantum::ir
     ↓
ZQN

where appropriate.

---

49. Resilience States

Where hybrid source-level resilience metadata is supported, the semantic vocabulary must remain compatible with the repository's established resilience model:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

The grammar must not implement state transitions.

---

50. Resilience Outcomes

Where source-level policies require outcome vocabulary, the established semantic outcomes remain:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

The grammar may represent policy intent where specified.

Runtime/resilience infrastructure performs the actual transition.

---

51. Distributed Integration

Hybrid computation may span distributed resources.

The grammar must therefore not assume that all participating domains share:

- one CPU;
- one memory space;
- one accelerator;
- one QPU;
- one clock;
- one node.

Distributed semantics belong to:

grammar/distributed/

Hybrid only expresses the cross-domain composition.

---

52. AI and Data Integration

Hybrid programs may combine:

classical
+
quantum
+
tensor
+
AI
+
accelerator

The hybrid grammar must reuse canonical AI/data/tensor constructs.

It must not encode:

- PyTorch;
- TensorFlow;
- JAX;
- CUDA;
- vendor-specific frameworks

as core Zamani grammar concepts.

Framework integration belongs to interoperability and backend layers.

---

53. Networking Integration

If a hybrid operation crosses network boundaries, hybrid syntax may preserve the logical relationship.

It must not automatically prescribe:

- TCP;
- UDP;
- InfiniBand;
- Ethernet;
- a particular NIC;
- a particular IP address;
- a particular network topology.

Those belong to networking and target realization.

---

54. Interoperability Integration

Hybrid may interact with external representations such as:

- OpenQASM;
- QIR;
- HDL formats;
- LLVM-related representations;
- MLIR-related representations;
- foreign functions;
- external accelerators.

These are interoperability boundaries.

They must not become the canonical Zamani semantic model.

The architecture remains:

Zamani
  ↓
canonical semantic representation
  ↓
interoperability lowering
  ↓
external format

not:

external format
  ↓
canonical Zamani semantics

unless an explicitly defined import mechanism is being implemented.

---

55. Dialect Integration

Hybrid must work with the dialect system without becoming a separate language.

A hybrid dialect must declare:

- dialect name;
- version;
- owner;
- syntax extensions;
- semantic extensions;
- AST mapping;
- IR mapping;
- capabilities;
- compatibility requirements;
- feature status.

Dialect syntax must not silently alter core hybrid semantics.

---

56. Macro Integration

Macros may generate hybrid syntax.

However:

macro expansion
      ↓
canonical parsing/AST
      ↓
semantic validation

must remain intact.

Macros must not bypass:

- type checking;
- capability checking;
- resource checking;
- effect checking;
- ownership;
- safety;
- semantic validation.

---

57. Metaprogramming Integration

Metaprogramming may inspect or generate hybrid constructs where the language permits it.

Generated hybrid syntax must still pass the normal semantic pipeline.

Metaprogramming must not introduce a privileged path around:

AST
→ semantics
→ IR

---

58. Determinism

Parsing must be deterministic.

Given:

same source
+
same language version
+
same lexical configuration
+
same grammar configuration

the parser must produce the same structural result.

Parsing must not depend on:

- hardware availability;
- current time;
- randomness;
- filesystem state;
- network state;
- environment variables;
- runtime state;
- available QPUs;
- available GPUs;
- available CPUs.

Hardware availability belongs downstream.

---

59. Parser Recovery

Hybrid grammar must provide predictable parser diagnostics for malformed syntax.

Recovery must not reinterpret malformed source as a different valid domain.

For example, a malformed hybrid boundary must not accidentally become an ordinary classical statement merely because the hybrid construct failed.

Diagnostics must identify:

- expected syntax;
- actual syntax;
- source location;
- relevant hybrid construct where possible.

---

60. Semantic Errors vs Syntax Errors

The grammar must reject syntax errors.

The semantic analyzer must reject semantic errors.

For example:

quantum::operation(valid_syntax)

may parse correctly while still being semantically invalid because:

- the operation does not exist;
- the argument type is wrong;
- the target lacks a required capability;
- a value is unavailable at that point;
- a resource requirement is unsatisfied.

Do not move every semantic rule into the parser.

---

61. Open-World Operations

Hybrid grammar must remain open to future operations.

It must not enumerate:

H
X
Y
Z
CNOT
RX
RY
RZ

as the universal set of quantum operations.

Likewise it must not enumerate every:

- GPU operation;
- FPGA primitive;
- tensor operation;
- AI operation;
- future accelerator operation.

Operation identity is semantic data.

This is essential for long-term scalability.

---

62. Domain Names

Domain names must be represented through the canonical name/identifier system.

The grammar may recognize canonical reserved domain tokens where they already exist.

Future or user-defined domains must not require a complete rewrite of the hybrid architecture merely because a new computational domain is introduced.

This supports future domains without creating:

one grammar branch per machine type

---

63. Domain Crossing Model

A hybrid boundary must conceptually represent:

producer domain
consumer domain
value/dependency
operation
control
resource/capability requirements

For example:

classical → quantum

means that classical information influences quantum computation.

It does not inherently specify:

CPU → QPU physical transport

Likewise:

quantum → classical

means that quantum computation produces information consumed by classical semantics.

It does not prescribe a physical readout implementation.

---

64. Hybrid Dependency Graph

The semantic representation should be capable of constructing a graph such as:

        classical input
              │
              ▼
       parameter computation
              │
              ▼
        quantum operation
              │
              ▼
          measurement
              │
              ▼
       classical condition
          ┌───┴───┐
          ▼       ▼
      quantum   classical
      branch    branch
          │       │
          └───┬───┘
              ▼
       accelerator stage
              │
              ▼
          final result

The grammar supplies syntax.

The semantic layer builds the actual dependency model.

---

65. No Hidden Synchronization

A domain crossing must not silently imply arbitrary runtime synchronization unless the language specification explicitly defines that semantic behavior.

Where synchronization matters, it must be represented semantically.

This prevents the compiler from making accidental assumptions such as:

every quantum call blocks the entire host

or:

every accelerator call is synchronous

unless that is explicitly part of the language semantics.

---

66. Blocking vs Asynchronous Semantics

Hybrid computation must be able to distinguish where required between:

- synchronous invocation;
- asynchronous invocation;
- deferred result;
- event/future;
- explicit synchronization;
- dependency-driven synchronization.

The implementation must not assume one execution model for every target.

---

67. Data Availability

A value crossing a domain boundary must have a semantic availability point.

For example:

measurement
    ↓
result available
    ↓
classical computation

The compiler/runtime may implement that through:

- synchronization;
- event;
- buffer;
- message;
- hardware signal;
- runtime handle;
- future;
- other target mechanism.

The grammar does not choose the implementation.

---

68. Error and Failure Propagation

Hybrid semantics must account for failures across domains.

Potential sources include:

- unavailable accelerator;
- failed quantum execution;
- degraded hardware;
- communication failure;
- runtime cancellation;
- resource exhaustion;
- unsupported capability.

The grammar may express source-level failure policy where specified.

Actual recovery remains downstream.

---

69. Security Boundary

Cross-domain data may cross security boundaries.

Hybrid grammar must integrate with:

grammar/security/

for:

- authorization;
- capability restrictions;
- trusted execution;
- secure computation;
- data policies.

The grammar must not bypass security validation merely because an operation is hybrid.

---

70. No Vendor Lock-In

Core hybrid syntax must not contain vendor-specific device identifiers.

Avoid universal constructs such as:

use_nvidia_gpu_0
use_ibm_qpu_7
use_intel_fpga_2
use_vendor_memory_bank_3

as core language semantics.

Vendor-specific mechanisms belong in:

- interoperability;
- dialects;
- target configuration;
- backend metadata.

---

71. Physical Mapping

Physical mapping is downstream.

The distinction is:

portable source intent
        ↓
logical operation
        ↓
semantic representation
        ↓
target planning
        ↓
physical mapping

Hybrid grammar must not collapse these layers.

---

72. Compile-Time vs Runtime Knowledge

Hybrid source must be able to participate in compilation without assuming that every resource property is known statically.

Some information may be:

- compile-time known;
- link-time known;
- deployment-time known;
- runtime known.

The semantic model must preserve this distinction where required.

The grammar must not force hardware discovery into parsing.

---

73. Resource Negotiation

When a hybrid program can execute on multiple possible targets, resource negotiation belongs downstream.

The source may state requirements and preferences.

For example:

required capability
preferred capability
minimum resource
maximum tolerated latency
required reliability

The compiler/runtime determines an acceptable realization.

---

74. Target Fallback

A portable hybrid program may have multiple valid implementations.

For example:

logical accelerated computation

could potentially lower to:

CPU
GPU
FPGA
ASIC
future accelerator

when semantic equivalence permits.

The hybrid grammar must not force one implementation.

---

75. Semantic Equivalence

When multiple target implementations exist, they must preserve the required Zamani semantics.

The compiler may transform:

high-level hybrid intent

into:

target-specific implementation

only when the required semantics remain valid.

This is essential to POCO-REAF.

---

76. Numerical Semantics

Hybrid grammar must not impose physical numerical widths unless the language specification explicitly makes a width part of program meaning.

Do not assume:

int = 32-bit
register = 32-bit
float = one hardware representation

unless those are explicitly defined by the canonical type system.

---

77. Tensor and Data Scalability

Hybrid computation may involve tensors and large data structures.

The hybrid grammar must not impose:

MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_DATASET_SIZE
MAX_BATCH_SIZE

as universal limits.

Actual limits belong to type checking, resource analysis, compilation, runtime, and targets.

---

78. Quantum Scalability

Hybrid syntax must support the same semantic model for:

one qubit
many qubits
parameterized qubit collections
logical qubits
large quantum systems
future quantum substrates

without changing the grammar because a larger machine becomes available.

The grammar must not contain a fixed qubit ceiling.

---

79. Classical Scalability

The same hybrid syntax must support:

one operation
many operations
single-core execution
multicore execution
vector execution
parallel execution
distributed execution

without introducing fixed processor counts.

---

80. Accelerator Scalability

The same source model must support:

no accelerator
one accelerator
multiple accelerators
heterogeneous accelerators
distributed accelerators
future accelerators

provided the semantic requirements are satisfied.

---

81. Repository Integration

The hybrid directory integrates with the repository as follows:

grammar/core/
grammar/types/
grammar/expressions/
grammar/statements/
grammar/functions/
grammar/effects/
grammar/memory/
grammar/concurrency/
        │
        ▼
grammar/classical/
grammar/quantum/
grammar/hdl/
grammar/hardware/
grammar/resources/
        │
        ▼
grammar/hybrid/
        │
        ▼
grammar/distributed/
grammar/ai/
grammar/data/
grammar/networking/
grammar/security/
        │
        ▼
grammar/compile/
grammar/execution/
grammar/interoperability/
grammar/dialects/
        │
        ▼
canonical Zamani parser composition

This is a dependency architecture, not a collection of competing languages.

---

82. Canonical Root Integration

The final root path must remain conceptually:

Zamani.g4
    ↓
canonical parser composition
    ↓
Hybrid

"hybrid.g4" must not import the root grammar.

The root must not duplicate hybrid productions.

The parser composition layer is responsible for connecting the hybrid domain to the complete language.

---

83. Lexer/Parser/AST/IR Traceability

Every public hybrid grammar rule must have a traceability record:

Source construct
      ↓
Lexer tokens
      ↓
Parser rule
      ↓
AST node/representation
      ↓
Semantic rule
      ↓
Canonical IR
      ↓
Compiler consumer
      ↓
Runtime/target consumer

This traceability must be established before the rule is marked production-ready.

---

84. Feature Completion Rule

A hybrid feature is not complete when:

ANTLR parses it

It is complete only when all required layers exist:

syntax
lexer compatibility
parser integration
AST mapping
semantic contract
type/effect/resource integration
capability integration
IR mapping
compiler integration
runtime integration
diagnostics
positive tests
negative tests
boundary tests
scalability tests
determinism tests
compatibility tests
hard-coding audit

---

85. Independent-File Completion Contract

Every hybrid grammar file must be independently completable.

Before implementation begins, the file must define:

Purpose

What the file exists to represent.

Owns

Exactly what syntax it owns.

Does Not Own

Exactly what belongs elsewhere.

Inputs

Grammar rules/tokens it consumes.

Outputs

Parser contexts it exposes.

Upstream dependencies

Canonical grammar contracts it requires.

Downstream consumers

AST, semantic, IR, compiler, runtime, or tooling consumers.

AST contract

What information must survive parsing.

Semantic contract

What semantic analysis must determine.

IR contract

Where the information ultimately lowers.

Resource contract

What resource/capability information may be attached.

Diagnostics contract

What syntax errors must be identifiable.

Test contract

Positive, negative, boundary, scalability, determinism, and compatibility cases.

Hard-coding audit

What forbidden assumptions must be checked.

Completion criteria

What must be true before the file is considered finished.

This prevents later repository changes from forcing redesign of an already-completed file merely because its integration contract was unspecified.

---

86. Dependency Discipline

Hybrid grammar files should depend on the smallest canonical contracts necessary.

Do not import the complete parser into a leaf grammar.

Do not import the same grammar through multiple competing paths.

Avoid circular composition.

Preferred direction:

Core
Types
Expressions
Statements
Functions
Resources
Classical
Quantum
Hardware
        ↓
Hybrid components
        ↓
Hybrid
        ↓
ZamaniParser
        ↓
Zamani

---

87. No Duplicate Universal Rules

The hybrid directory must never redefine:

identifier
qualifiedName
expression
statement
block
typeExpression
argumentList
functionDeclaration
resourceItem

when those already have canonical owners.

If a hybrid file needs one of those concepts, it consumes the canonical rule.

---

88. No Duplicate Resource Language

"hybrid-resources.g4" must remain a thin adapter.

The universal resource language remains owned by:

grammar/resources/

Therefore hybrid must not develop an independent grammar for:

- requirements;
- capabilities;
- constraints;
- preferences;
- hints;
- targets;
- resources.

---

89. No Duplicate Quantum Language

Hybrid must not redefine:

- quantum gates;
- quantum states;
- measurement;
- quantum registers;
- quantum operation semantics.

It composes canonical quantum constructs.

Quantum operation semantics ultimately belong to:

quantum::ir

---

90. No Duplicate Classical Language

Hybrid must not redefine:

- arithmetic;
- loops;
- functions;
- ordinary variables;
- ordinary expressions;
- ordinary control flow.

Those remain canonical language features.

---

91. No Duplicate HDL Language

Hybrid may compose HDL computation but must not duplicate:

- signal syntax;
- module syntax;
- port syntax;
- timing syntax;
- register syntax;
- synthesis semantics.

Those belong to:

grammar/hdl/

---

92. No Duplicate Hardware Language

Hybrid may express hardware-related intent but must not become a hardware-target language.

Hardware realization belongs to:

grammar/hardware/

and downstream target infrastructure.

---

93. Safety

The grammar contains no executable target-language actions.

It must perform no:

- filesystem access;
- network access;
- hardware discovery;
- environment inspection;
- secret access;
- runtime execution;
- random selection.

The Rust implementation consuming this grammar must remain:

Rust 2021
Rust 1.97 / 1.97.1
safe Rust

with:

unsafe

prohibited.

---

94. Rust Integration

The grammar itself is language-independent ANTLR syntax.

Rust integration must occur through the repository's parser/frontend pipeline.

The Rust implementation must not add a hybrid-specific unsafe escape hatch.

Any AST or semantic representation introduced for hybrid must remain compatible with the repository's safe Rust architecture.

---

95. Tooling Integration

Hybrid syntax must be usable by:

- formatter;
- syntax highlighter;
- IDE tooling;
- source navigation;
- diagnostics;
- refactoring tools;
- documentation tooling;
- grammar conformance tooling.

The parser must preserve enough structural information for these tools.

---

96. Documentation Integration

This README is an architectural contract.

It is not the complete language specification.

The authority hierarchy remains:

grammar/specification/
        ↓
normative language specification

grammar/Zamani.g4
        ↓
canonical ANTLR composition

grammar/hybrid/
        ↓
hybrid-domain grammar components

grammar/grammar.md
        ↓
implementation-conformance reference

grammar/Zamani-Grammar.md
        ↓
historical/extended design reference

"Zamani-Grammar.md" must not silently introduce implemented hybrid syntax.

---

97. Feature Lifecycle

A proposed hybrid feature follows:

Zamani-Grammar.md / proposal
        ↓
semantic design
        ↓
AST contract
        ↓
grammar contract
        ↓
ANTLR grammar
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
compatibility validation
        ↓
stable

A feature does not become stable merely because it appears in "Zamani-Grammar.md".

---

98. Status Classification

Hybrid features must use the repository's established status vocabulary where applicable:

stable
proposed
experimental
deprecated
historical
not implemented

Documentation status and implementation status must not be confused.

---

99. Positive Test Requirements

At minimum, hybrid tests must cover:

minimal hybrid program
classical → quantum
quantum → classical
classical → accelerator
quantum → accelerator
accelerator → classical
accelerator → quantum
classical → quantum → classical
classical → accelerator → quantum
quantum → accelerator → classical
nested hybrid regions
multiple domain crossings
parameterized operations
measurement-driven control
resource requirements
capability requirements
preferences
constraints
hints
hybrid functions
synchronization
shared data
distributed hybrid computation

---

100. Negative Tests

Negative tests must include:

- malformed hybrid marker;
- malformed hybrid region;
- missing block delimiter;
- malformed domain qualifier;
- malformed invocation;
- malformed cross-domain binding;
- malformed resource expression;
- malformed synchronization expression;
- malformed capability expression;
- malformed function construct;
- malformed data-boundary construct.

Semantic invalidity must be tested separately from syntax invalidity.

---

101. Boundary Tests

Boundary tests must verify that hybrid grammar does not accidentally introduce finite limits.

Test:

- one domain crossing;
- many domain crossings;
- deeply nested hybrid regions;
- large operation lists;
- large expressions;
- large resource expressions;
- large tensor shapes;
- large quantum resource requirements;
- symbolic quantities;
- arbitrary identifiers;
- arbitrary domain names where allowed.

---

102. Scalability Tests

The scalability suite must specifically search for accidental limits.

It must establish that no grammar-level restriction exists for:

number of qubits
number of CPUs
number of GPUs
number of FPGAs
number of accelerators
number of nodes
number of threads
number of resources
number of domain crossings
tensor rank
register width
network size
memory capacity
device count
timeline count

Actual parser/compiler implementation limits must be documented separately from language semantics.

---

103. Determinism Tests

Given identical:

source
grammar version
lexer configuration
dialect configuration

parsing must be deterministic.

Tests must ensure that parsing does not change because:

- a GPU exists;
- a QPU exists;
- memory is available;
- a runtime is running;
- a network is available.

---

104. Compatibility Tests

Compatibility tests must compare:

specification
Zamani.g4
ZamaniParser
ZamaniLexer
Rust lexer
Rust parser
AST
semantic analysis
IR
compiler
runtime

for every stable hybrid construct.

A grammar change is incomplete if downstream implementations disagree about the construct's meaning.

---

105. Hard-Coding Audit

The hybrid validation suite must scan for forbidden universal assumptions.

At minimum, search for:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ACCELERATORS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

Also inspect for disguised equivalents such as:

gpu0
qpu0
cpu0
fpga0
node0
memory_bank0
physical_qubit0

A concrete identifier is not automatically invalid, but it must never accidentally become a universal language capacity or portable semantic assumption.

---

106. Security Audit

Hybrid validation must ensure that cross-domain constructs cannot bypass:

- authorization;
- capability checks;
- resource policies;
- ownership;
- type checking;
- effect checking;
- sandbox boundaries;
- interoperability validation.

A hybrid boundary is not a security bypass.

---

107. Performance Audit

Grammar composition must avoid unnecessary duplication.

Avoid:

- duplicate expression precedence;
- duplicate type parsing;
- duplicate resource parsing;
- duplicate quantum parsing;
- excessive parser alternatives;
- unnecessary ambiguity;
- left-recursive constructs that conflict with the canonical grammar;
- redundant domain branches.

Performance optimizations must not change language semantics.

---

108. Ambiguity Policy

Hybrid grammar must not rely on semantic guesses to resolve syntactic ambiguity where a deterministic grammar can resolve it cleanly.

Where contextual interpretation is intentionally required, the semantic contract must explicitly document it.

The parser must not depend on:

- hardware state;
- target discovery;
- runtime state.

---

109. Error Stability

Public hybrid parser rules should have stable meanings.

Changing a public rule in a way that changes accepted source syntax requires:

- compatibility review;
- specification update;
- conformance tests;
- migration documentation where necessary.

---

110. Public Rule Stability

The hybrid composition should expose stable entry points such as:

hybridDeclaration
hybridStatement
hybridExpression
hybridConstruct

where these are actually required by the canonical parser.

Rule names must not be multiplied merely to expose implementation details.

---

111. Domain Extension

Adding a new computational domain must not require redesigning the hybrid architecture.

For example, adding:

future_accelerator

should be possible through the established domain/capability/semantic mechanisms rather than requiring:

new grammar root
new hybrid IR
new resource grammar
new type system

The domain becomes another participant in the existing composition model.

---

112. Future-Proofing

The grammar should be able to accommodate future computational models including:

- new quantum modalities;
- new accelerator architectures;
- optical computing;
- neuromorphic computing;
- molecular computing;
- biological computing;
- future hardware substrates;
- new distributed execution models.

Future-proofing comes from semantic openness, not from enumerating every future technology.

---

113. Program Meaning vs Target Realization

The central distinction is:

PROGRAM MEANING
       ≠
TARGET REALIZATION

For example:

requires capability("tensor.compute")

is program intent.

Which tensor accelerator is used is target realization.

Likewise:

requires qubits >= n

is a semantic resource requirement.

Which physical qubits are selected is target realization.

---

114. Hybrid Compiler Contract

The compiler must be able to:

1. parse hybrid syntax;
2. construct domain-neutral AST;
3. resolve names;
4. check types;
5. check effects;
6. analyze ownership;
7. analyze resource requirements;
8. analyze capabilities;
9. validate domain crossings;
10. construct canonical semantic representation;
11. lower quantum portions through "quantum::ir";
12. lower classical portions through the canonical classical representation;
13. lower hardware/HDL portions through their canonical semantic paths;
14. preserve cross-domain dependencies;
15. optimize;
16. route where applicable;
17. schedule;
18. apply resilience/QEC where applicable;
19. pass through ZQN where applicable;
20. lower through HAL/backend infrastructure;
21. generate the target realization.

---

115. Runtime Contract

Runtime may be responsible for:

- resource discovery;
- capability discovery;
- synchronization;
- data movement;
- dispatch;
- monitoring;
- failure handling;
- recovery;
- dynamic scheduling;
- target selection.

None of those responsibilities belong in the hybrid parser.

---

116. HAL Contract

HAL is responsible for translating semantic target requirements into actual device capabilities.

Hybrid grammar must never become a hardware discovery mechanism.

The source may express:

requires capability(...)

The HAL determines whether an available target satisfies it.

---

117. No Physical Topology in Core Hybrid Grammar

The hybrid grammar must not encode a universal topology.

Do not assume:

linear
grid
mesh
ring
star
fully connected
specific QPU topology
specific GPU interconnect
specific FPGA fabric

as core semantics.

Topology belongs to hardware/routing systems.

---

118. No Physical Memory Assumptions

Do not encode:

RAM = 64 GB
VRAM = 24 GB
register = 32 bit

as language assumptions.

The program may require memory semantically.

The resource/target system determines whether the environment can satisfy it.

---

119. No Fixed Device Inventory

Do not assume:

one CPU
one GPU
one QPU
one FPGA
one accelerator

or any fixed number.

A hybrid program may execute in any supported resource configuration satisfying its semantics.

---

120. No Fixed Thread Model

Hybrid grammar must not assume:

8 threads
16 threads
32 threads

as a universal implementation model.

Concurrency intent belongs to the canonical concurrency system.

Actual execution width belongs downstream.

---

121. No Fixed Timeline Model

Hybrid grammar must not assume a fixed number of:

- execution timelines;
- speculative branches;
- distributed participants;
- concurrent regions.

The semantic model must remain scalable.

---

122. Interaction with MTS

If Zamani's Multi-Timeline System is used with hybrid computation, the hybrid grammar must preserve the semantic relationship without imposing a fixed timeline count.

MTS semantics remain owned by the execution/timeline subsystem.

Hybrid only composes the computation.

---

123. Interaction with Sankofa

Where Sankofa-related memory/temporal/learning constructs participate in a hybrid computation, the hybrid grammar must consume their canonical semantic representations.

It must not implement memory, learning, recall, history, or temporal reasoning itself.

---

124. Interaction with Nano Computing

If nano computing becomes a first-class Zamani domain, it may participate in hybrid computation through the same model:

domain
+
operation
+
data
+
dependency
+
capability
+
resource

No hybrid-specific nano IR should be introduced.

---

125. Inter-Domain Value Model

A value crossing domains must remain semantically identifiable.

The semantic representation should be able to answer:

Where was the value produced?
What is its type?
Who owns it?
When is it available?
Who consumes it?
What effects apply?
What capabilities are required?
What synchronization is required?
What resource constraints apply?

The parser must preserve the information needed to answer these questions.

---

126. Hybrid Operation Model

Where a generic hybrid operation is represented, its semantic information should be extensible enough to preserve:

name
namespace/domain
operands
parameters
results
attributes
modifiers
effects
capabilities
resource requirements
source span
dependencies

The grammar must not reduce future operations to a fixed enumeration.

---

127. No Backend Leakage

A core hybrid grammar file must not require knowledge of:

- LLVM internals;
- CUDA internals;
- ROCm internals;
- vendor QPU APIs;
- FPGA vendor primitives;
- device driver APIs;
- runtime-specific handles.

Those are downstream interoperability/backend concerns.

---

128. Build Integration

The build system must treat the canonical ANTLR grammar hierarchy as one composition graph.

It must:

- generate parser artifacts deterministically;
- validate grammar imports;
- reject missing grammar dependencies;
- reject duplicate grammar authorities;
- reject conflicting token definitions;
- verify grammar names and filenames;
- run parser generation;
- run Rust frontend conformance tests.

Hyphenated repository filenames must not be assumed to be valid ANTLR grammar identities when ANTLR requires the grammar filename to correspond to the grammar declaration.

---

129. Generated Artifact Policy

Generated ANTLR artifacts must not become a second source authority.

The source of truth remains:

grammar/*.g4

Generated parser code is derived output.

Generated documentation is derived output.

"grammar/grammar.md" is a conformance reference, not a grammar authority.

---

130. CI Requirements

CI must validate:

ANTLR grammar generation
lexer generation
parser generation
grammar ambiguity
duplicate tokens
undefined rules
unused rules where applicable
Rust compilation
safe-Rust requirement
AST conformance
semantic conformance
IR conformance
hybrid positive tests
hybrid negative tests
hybrid boundary tests
hybrid scalability tests
determinism tests
compatibility tests
hard-coding audit
documentation consistency

---

131. Production Gate

"grammar/hybrid/" must not be declared production-ready merely because its ".g4" files compile.

Production readiness requires:

Specification
      ✓
Lexer
      ✓
Parser
      ✓
AST
      ✓
Semantics
      ✓
Resources
      ✓
Capabilities
      ✓
IR
      ✓
Compiler
      ✓
Runtime
      ✓
Diagnostics
      ✓
Tests
      ✓
Compatibility
      ✓
Scalability
      ✓
Hard-coding audit
      ✓
Safe Rust
      ✓

---

132. Required Test Matrix

Every stable hybrid feature should be tested across:

Dimension| Required
Lexical| Yes
Syntax| Yes
AST| Yes
Semantic| Yes
Type| Where applicable
Effects| Where applicable
Resources| Where applicable
Capabilities| Where applicable
Classical| Where applicable
Quantum| Where applicable
Accelerator| Where applicable
HDL| Where applicable
Distributed| Where applicable
IR| Yes
Compiler| Yes
Runtime| Where applicable
Diagnostics| Yes
Compatibility| Yes
Scalability| Yes
Determinism| Yes
Hard-coding| Yes

---

133. Example Semantic Scenarios

The following are architectural scenarios, not additional grammar authorities.

Classical controls quantum computation

classical value
      ↓
quantum operation parameter/control

Quantum result controls classical computation

quantum operation
      ↓
measurement
      ↓
classical condition

Classical + accelerator

classical computation
      ↓
accelerated region
      ↓
classical result

Quantum + accelerator

quantum computation
      ↓
accelerator computation
      ↓
quantum/classical continuation

Full hybrid

classical
   ↓
quantum
   ↓
measurement
   ↓
classical
   ↓
accelerator
   ↓
distributed computation
   ↓
result

All use one Zamani language.

---

134. What Must Never Be Encoded Here

The following do not belong as universal hybrid grammar semantics:

GPU 0
QPU 0
CPU 0
FPGA 0
physical_qubit(0)
MAX_QUBITS
MAX_GPUS
MAX_CPUS
MAX_FPGAS
MAX_THREADS
MAX_NODES
MAX_MEMORY
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
fixed topology
fixed memory size
fixed device inventory
fixed accelerator inventory
vendor-specific instruction sets
QEC implementation
noise implementation
routing implementation
scheduler implementation
calibration implementation
hardware discovery
runtime recovery implementation

---

135. What Belongs in Source Semantics

Source semantics may express:

requires capability(...)
requires resource(...)
requires qubits >= n
requires memory >= required_memory
requires topology(...)
prefer capability(...)
constraint ...
hint ...

where those constructs are part of the canonical resource/capability language.

The critical distinction remains:

requirement ≠ physical allocation
capability ≠ device selection
preference ≠ requirement
hint ≠ semantic guarantee
logical resource ≠ physical resource

---

136. Definition of Done for "hybrid.g4"

"hybrid.g4" is complete only when:

- its ownership is singular;
- it imports only permitted lower-level grammars;
- it does not import the parser root;
- it contains no duplicate universal grammar;
- it exposes stable public hybrid entry points;
- AST mapping is documented;
- semantic mapping is documented;
- IR mapping is documented;
- resource integration is documented;
- capability integration is documented;
- compiler consumers are identified;
- runtime consumers are identified;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- compatibility tests exist;
- determinism tests exist;
- hard-coding audit passes.

---

137. Definition of Done for "classical-quantum.g4"

Complete only when:

- classical→quantum syntax is defined;
- quantum→classical syntax is defined where supported;
- parameter flow is preserved;
- result flow is preserved;
- dependencies are preserved;
- AST mapping exists;
- semantic validation exists;
- type integration exists;
- effect integration exists;
- resource/capability integration exists;
- quantum lowering uses "quantum::ir";
- scheduling dependencies are preserved;
- runtime semantics are defined;
- positive/negative/boundary/scalability tests exist.

---

138. Definition of Done for "quantum-classical-control.g4"

Complete only when:

- measurement-dependent control is represented;
- classical conditions are preserved;
- dynamic dependencies are preserved;
- semantic capability checking is defined;
- scheduling integration is defined;
- no backend capability is hard-coded;
- AST/semantic/IR contracts exist;
- tests cover nested and repeated control;
- no fixed control count exists.

---

139. Definition of Done for "accelerator-interoperability.g4"

Complete only when:

- accelerator intent is represented generically;
- no vendor-specific core syntax is required;
- accelerator requirements use canonical capabilities/resources;
- AST mapping exists;
- semantic mapping exists;
- compiler lowering is defined;
- runtime realization is defined;
- CPU/GPU/FPGA/ASIC/future alternatives remain possible where semantically valid;
- no fixed accelerator count exists;
- scalability tests pass.

---

140. Definition of Done for "hybrid-resources.g4"

Complete only when:

- canonical "Resources" remains the sole resource syntax owner;
- no resource syntax is duplicated;
- requirements remain distinct from constraints;
- constraints remain distinct from preferences;
- preferences remain distinct from hints;
- capabilities remain distinct from targets;
- target expressions do not imply physical device selection;
- no physical inventory is encoded;
- no fixed capacity is encoded;
- semantic resource analysis is downstream;
- no second resource IR exists.

---

141. Definition of Done for "synchronization.g4"

Complete only when:

- semantic dependencies can be represented;
- synchronization intent is distinguishable from physical timing;
- asynchronous and synchronous semantics are distinguishable where required;
- scheduler integration is defined;
- runtime integration is defined;
- no physical clock assumptions exist.

---

142. Definition of Done for "shared-data.g4"

Complete only when:

- producer/consumer relationships are preserved;
- domain crossings are represented;
- ownership/lifetime information is preservable;
- type checking is delegated correctly;
- physical transport is not hard-coded;
- runtime data movement is downstream;
- resource/capability integration is defined.

---

143. Definition of Done for "hybrid-functions.g4"

Complete only when:

- hybrid function composition is represented;
- canonical function syntax is reused;
- domain participation can be represented;
- parameters and results remain canonical;
- effects are preserved;
- resource/capability requirements are preservable;
- AST/semantic/IR contracts exist;
- no second function system is created.

---

144. Repository-Wide Integration Checklist

Before declaring hybrid production-ready, verify:

Specification

- [ ] "grammar/specification/" defines hybrid semantics.
- [ ] "grammar/spec/" defines hybrid contracts.
- [ ] "Zamani-Grammar.md" status is correctly classified.
- [ ] "grammar.md" reflects implementation reality.

Lexer

- [ ] Every referenced token exists in the canonical lexer.
- [ ] No duplicate hybrid token vocabulary exists.
- [ ] Contextual words are documented.
- [ ] Unicode behavior is consistent.

Parser

- [ ] Hybrid grammar names match their files.
- [ ] Import graph is acyclic.
- [ ] "ZamaniParser" composes Hybrid.
- [ ] Hybrid does not import "ZamaniParser".
- [ ] Root grammar does not duplicate Hybrid.

AST

- [ ] Every public hybrid construct has an AST mapping.
- [ ] Source spans are preserved.
- [ ] No hybrid-only duplicate universal AST exists.

Semantics

- [ ] Domain compatibility is checked.
- [ ] Types are checked.
- [ ] Effects are checked.
- [ ] Ownership is checked.
- [ ] Resource requirements are checked.
- [ ] Capabilities are checked.
- [ ] Synchronization dependencies are checked.

IR

- [ ] No Hybrid IR is introduced.
- [ ] Quantum semantics use "quantum::ir".
- [ ] Classical semantics use the canonical classical representation.
- [ ] Hardware/HDL semantics use their canonical representation.

Compiler

- [ ] Cross-domain dependencies survive lowering.
- [ ] Optimization can operate on hybrid semantics.
- [ ] Routing can consume quantum dependencies.
- [ ] Scheduling can consume hybrid dependencies.
- [ ] Resilience/QEC can consume relevant metadata.
- [ ] ZQN can consume relevant quantum semantics.
- [ ] HAL can consume target requirements.

Runtime

- [ ] Runtime can realize synchronization.
- [ ] Runtime can handle data movement.
- [ ] Runtime can discover resources where required.
- [ ] Runtime can handle failure/recovery policies.

Scalability

- [ ] No fixed resource limits exist.
- [ ] No fixed device inventory exists.
- [ ] No fixed qubit limit exists.
- [ ] No fixed accelerator limit exists.
- [ ] No fixed node limit exists.
- [ ] No fixed tensor rank limit exists.
- [ ] No fixed register width is imposed by hybrid.

Safety

- [ ] No unsafe Rust is required.
- [ ] No grammar actions execute Rust.
- [ ] No filesystem access occurs during parsing.
- [ ] No network access occurs during parsing.
- [ ] No hardware discovery occurs during parsing.
- [ ] No runtime execution occurs during parsing.

---

145. Final Hybrid Architecture

The production architecture is:

                         ZAMANI
                           │
                           ▼
                      Canonical AST
                           │
                           ▼
                   Hybrid semantic graph
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
        ▼                  ▼                  ▼
    Classical           Quantum          Accelerator
        │                  │                  │
        │                  ▼                  │
        │             quantum::ir            │
        │                  │                  │
        └──────────────────┼──────────────────┘
                           │
                           ▼
                  Canonical semantics
                           │
              ┌────────────┼────────────┐
              ▼            ▼            ▼
         Optimization    Routing     Scheduling
              │            │            │
              └────────────┼────────────┘
                           ▼
                    QEC / Resilience
                           │
                           ▼
                          ZQN
                           │
                           ▼
                          HAL
                           │
                           ▼
                   Target realization
                           │
          ┌────────────────┼────────────────┐
          ▼                ▼                ▼
         CPU              GPU              FPGA
          │                │                │
          └────────────────┼────────────────┘
                           │
                    QPU / ASIC / future

Hybrid is therefore the composition layer, not the target layer.

---

146. Core Invariant

The most important invariant for this directory is:

«Hybrid syntax describes relationships between computations; it does not describe the accidental physical arrangement of the machine that realizes those computations.»

Therefore:

classical
quantum
HDL
accelerator
distributed
AI
data
network
future domains

can participate in one program without requiring separate languages or separate semantic universes.

---

147. POCO-REAF Invariant

The hybrid source should describe:

WHAT

and, where necessary:

WHY
REQUIRED
PREFERRED
CONSTRAINED
CAPABLE

The compiler/runtime determines:

WHERE
WHEN
HOW
ON WHICH DEVICE
ON WHICH RESOURCE
WITH WHICH ROUTING
WITH WHICH SCHEDULE
WITH WHICH IMPLEMENTATION

This separation is the foundation of:

Program Once
    ↓
Compile Once
    ↓
Run Everywhere
    ↓
Anywhere
    ↓
Forever

subject to the actual semantic capabilities and resources of the realization target.

---

148. Final Non-Negotiable Rules

1. "grammar/hybrid/" is one domain of one Zamani language.
2. It must not become a second language.
3. It must not create a second type system.
4. It must not create a second expression system.
5. It must not create a second resource system.
6. It must not create a second quantum language.
7. It must not create a second quantum IR.
8. "quantum::ir" remains the canonical quantum semantic boundary.
9. General syntax must be reused rather than duplicated.
10. The canonical lexer remains the only lexer authority.
11. Hybrid must not import the parser root.
12. The parser root composes Hybrid.
13. Domain-specific operations remain semantically open.
14. Quantum gates must not become a fixed universal enumeration.
15. Vendor hardware must not become core language syntax.
16. Physical topology must remain downstream.
17. Routing must remain downstream.
18. Scheduling must remain downstream.
19. QEC must remain downstream.
20. ZQN must remain downstream.
21. Calibration must remain downstream.
22. HAL must remain downstream.
23. Runtime resource discovery must remain downstream.
24. Resource requirements must not become physical allocations.
25. Capabilities must not become physical device selections.
26. Preferences must not become requirements.
27. Hints must not silently change semantics.
28. No universal hardware capacity may be hard-coded.
29. No fixed qubit count may be encoded.
30. No fixed CPU/core/thread count may be encoded.
31. No fixed GPU/FPGA/accelerator count may be encoded.
32. No fixed node/device count may be encoded.
33. No fixed memory capacity may be encoded.
34. No fixed tensor rank may be encoded.
35. No fixed register width may be imposed by hybrid.
36. No fixed network size may be imposed.
37. No fixed timeline count may be imposed.
38. Source spans must survive into semantic diagnostics.
39. Syntax errors must remain distinct from semantic errors.
40. Every public hybrid construct must have an AST contract.
41. Every public hybrid construct must have a semantic contract.
42. Every public hybrid construct must have an IR contract.
43. Every production feature must identify downstream consumers before implementation.
44. Every production feature requires positive tests.
45. Every production feature requires negative tests.
46. Every production feature requires boundary tests.
47. Every production feature requires scalability tests.
48. Every production feature requires compatibility tests.
49. Deterministic parsing is mandatory.
50. Safe Rust is mandatory.
51. "unsafe" Rust is prohibited.
52. Rust 2021 with Rust 1.97 / 1.97.1 is the implementation baseline.
53. "Zamani-Grammar.md" is not an implicit implementation authority.
54. "grammar.md" is not a competing specification.
55. Generated artifacts are not source authorities.
56. Dialects must not silently fork Zamani.
57. Macros must not bypass semantic validation.
58. Metaprogramming must not bypass safety and semantic validation.
59. New computational domains must be addable without redesigning the hybrid foundation.
60. The same semantic model must scale from tiny computation to arbitrarily large computation subject to available implementation resources.

---

149. Final Definition of Production-Ready Hybrid

"grammar/hybrid/" is production-ready only when the complete contract is true:

                         SOURCE
                           │
                           ▼
                         LEXER
                           │
                           ▼
                         PARSER
                           │
                           ▼
                    HYBRID COMPOSITION
                           │
                           ▼
                    DOMAIN-NEUTRAL AST
                           │
                           ▼
                TYPE / EFFECT / OWNERSHIP
                           │
                           ▼
              RESOURCE / CAPABILITY ANALYSIS
                           │
                           ▼
                  SEMANTIC VALIDATION
                           │
                           ▼
                 CANONICAL SEMANTICS
                           │
             ┌─────────────┼─────────────┐
             ▼             ▼             ▼
         CLASSICAL     quantum::ir    HDL/HARDWARE
             │             │             │
             └─────────────┼─────────────┘
                           ▼
                      OPTIMIZATION
                           │
                    ┌──────┼──────┐
                    ▼      ▼      ▼
                  ROUTE  SCHEDULE RESILIENCE
                    │      │      │
                    └──────┼──────┘
                           ▼
                          ZQN
                           │
                           ▼
                          HAL
                           │
                           ▼
                    TARGET BACKEND
                           │
             ┌─────────────┼─────────────┐
             ▼             ▼             ▼
            CPU           GPU           FPGA
             │             │             │
             └─────────────┼─────────────┘
                           ▼
                    QPU / ASIC / FUTURE

The defining rule is:

«Zamani hybrid syntax expresses portable cross-domain computation and intent. Semantic analysis determines whether that intent is valid. The canonical IR represents it. Compiler and runtime infrastructure determine how it is realized on available resources.»

That is the required foundation for production-grade hybrid Zamani and for POCO-REAF.

---

150. Completion Statement

This README is complete as the hybrid-domain architectural contract when the implementation files conform to it.

It does not authorize syntax by itself.

A hybrid feature becomes implemented only after the corresponding:

specification
→ lexer compatibility
→ grammar
→ AST
→ semantic analysis
→ resource/capability analysis
→ canonical IR
→ compiler
→ runtime
→ tests

chain is complete.

No later grammar-domain addition should require rewriting this contract merely to discover where the new feature belongs.

The governing architecture remains:

«One Zamani language. One canonical semantic model. One canonical quantum IR boundary. Open-world operations and capabilities. Target-independent source intent. Target-aware downstream realization. No artificial hardware limits. Safe Rust only. POCO-REAF.»