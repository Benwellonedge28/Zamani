Zamani Hardware Grammar

Path: "grammar/hardware/"
Primary entry grammar: "hardware.g4"
Language: Zamani Universal Computing Language
Grammar technology: ANTLR
Runtime/compiler baseline: Rust 1.97 / Rust 1.97.1
Safety requirement: No "unsafe" Rust
Status: Production architecture and integration contract

---

1. Purpose

The "grammar/hardware/" subsystem defines the source-language syntax for expressing hardware intent, hardware structure, hardware contracts, capabilities, resources, targets, topology requirements, placement intent, accelerators, and hardware-device classes in Zamani.

It exists to allow Zamani programs to describe computation and hardware relationships without coupling the program's permanent semantics to one particular physical machine.

The hardware grammar is therefore a machine-independent language boundary.

It enables Zamani to express hardware-aware programs that can subsequently be interpreted, compiled, lowered, mapped, scheduled, routed, synthesized, deployed, or executed by downstream systems.

The fundamental design objective is:

«Describe what the program requires or means, not an accidental limitation of the machine currently available.»

This supports:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

and the broader Zamani objective:

«From Atom to Everywhere.»

---

2. Architectural Position

The hardware grammar sits at the syntax layer.

The intended architecture is:

Zamani source
    │
    ▼
Zamani lexer
    │
    ▼
Zamani parser
    │
    ├── classical syntax
    ├── quantum syntax
    ├── HDL syntax
    ├── hardware syntax
    ├── distributed syntax
    ├── AI/data syntax
    └── other language domains
    │
    ▼
Syntax AST
    │
    ▼
Semantic analysis
    │
    ├── type checking
    ├── capability checking
    ├── requirement checking
    ├── effect checking
    ├── resource validation
    └── constraint validation
    │
    ▼
Canonical semantic representations
    │
    ├── classical IR
    ├── quantum::ir
    ├── HDL/hardware semantic model
    ├── resource model
    └── execution/deployment model
    │
    ▼
Compiler pipeline
    │
    ├── optimization
    ├── routing
    ├── scheduling
    ├── synthesis
    ├── resilience
    └── target lowering
    │
    ▼
Hardware abstraction / HAL
    │
    ▼
Runtime / deployment
    │
    ▼
Physical machine

The grammar is therefore upstream of machine realization.

It must never become a replacement for the compiler IR, hardware HAL, scheduler, router, runtime, calibration subsystem, or physical device manager.

---

3. Core Ownership Rule

"grammar/hardware/" owns the syntax of hardware intent and hardware descriptions.

It does not own the implementation of hardware execution.

The distinction is mandatory.

3.1 Hardware grammar owns

The hardware grammar owns syntax for:

- hardware declarations;
- abstract hardware modules;
- hardware interfaces;
- ports;
- hardware resources;
- hardware capabilities;
- hardware requirements;
- hardware constraints;
- hardware preferences;
- target classes;
- topology requirements;
- placement intent;
- logical mappings;
- hardware instances;
- logical connections;
- hardware properties;
- clocks and timing contracts where they belong to the hardware contract layer;
- memories;
- accelerator classes;
- CPU classes;
- GPU classes;
- FPGA classes;
- ASIC classes;
- QPU declarations;
- hardware extension points;
- hardware-specific generic parameters;
- machine-independent hardware composition.

3.2 Hardware grammar does not own

It does not own:

- lexical token definitions;
- general identifier semantics;
- general expressions;
- general type semantics;
- canonical AST implementation;
- semantic type checking;
- hardware discovery;
- physical device enumeration;
- physical device allocation;
- device drivers;
- calibration;
- pulse generation;
- quantum gate semantics;
- "quantum::ir";
- QEC algorithms;
- ZQN noise semantics;
- optimization algorithms;
- routing algorithms;
- scheduling algorithms;
- resilience policy;
- runtime dispatch;
- deployment orchestration;
- physical topology discovery;
- physical addresses;
- vendor-specific runtime behavior.

The current "hardware.g4" already establishes this separation explicitly; this README is the authoritative subsystem-level contract that keeps the specialized grammar files aligned with it.

---

4. Hardware Grammar Is Not a Hardware IR

A critical architectural rule is:

Grammar ≠ AST ≠ semantic model ≠ IR ≠ hardware runtime

The grammar recognizes source syntax.

The parser produces syntax structures.

Semantic analysis interprets those structures.

The compiler produces canonical representations.

Target-specific stages perform realization.

The hardware grammar must therefore never encode a second hardware IR.

For example, the grammar may recognize:

requires capability.quantum;

or:

requires resource.memory >= required_memory;

but it must not itself decide:

use_device("some-device");

unless such a device-specific construct is explicitly part of a separate target/deployment language contract.

Even then, the semantic meaning must remain distinct from physical discovery and allocation.

---

5. POCO-REAF Contract

The hardware grammar is designed around five properties.

5.1 Program Once

The developer describes:

- computation;
- semantic hardware requirements;
- capabilities;
- constraints;
- preferences;
- resource relationships;
- interfaces;
- deployment intent.

The developer should not have to rewrite the computation merely because the machine changes.

5.2 Compile Once

Compilation should preserve architecture-independent meaning wherever possible.

A hardware-aware source declaration should be representable in a target-independent semantic form.

Target realization occurs downstream.

5.3 Run Everywhere

The same semantic program may be mapped to:

- embedded hardware;
- CPUs;
- multicore CPUs;
- GPUs;
- FPGAs;
- ASICs;
- quantum processors;
- simulators;
- accelerators;
- clusters;
- supercomputers;
- distributed systems;
- cloud infrastructure;
- future execution platforms.

5.4 Run Anywhere

The grammar must not assume that execution happens locally.

The source may express requirements compatible with:

- local execution;
- remote execution;
- distributed execution;
- heterogeneous execution;
- cloud execution;
- edge execution;
- accelerator execution;
- quantum execution.

5.5 Run Forever

The language must be evolvable.

Future hardware must be expressible without repeatedly redesigning the entire core grammar.

This requires:

- extensible properties;
- qualified names;
- dialect mechanisms;
- versioning;
- capability-based descriptions;
- generic parameters;
- semantic separation between requirements and implementation;
- reserved extension space.

---

6. Absolute Scalability Rule

The grammar must contain no arbitrary machine-size maximums.

The grammar must never introduce constructs such as:

MAX_CPUS = 64
MAX_CORES = 128
MAX_GPUS = 8
MAX_FPGAS = 4
MAX_QUBITS = 64
MAX_DEVICES = 1024
MAX_PORTS = 256

or equivalent parser restrictions.

The same applies to hidden restrictions encoded through:

- fixed alternatives;
- bounded repetitions;
- fixed-size arrays used to model machine capacity;
- enumerations representing currently known hardware;
- fixed topology assumptions;
- fixed device identifiers;
- fixed register counts;
- fixed memory capacities.

The grammar's role is to describe the syntax.

Actual limits belong to:

- semantic analysis;
- resource resolution;
- compiler policy;
- target capability;
- runtime availability;
- operating-system constraints;
- physical hardware;
- deployment policy.

The current grammar explicitly follows this principle: quantities are expressions and there are no grammar-level maximums for CPUs, cores, GPUs, FPGAs, qubits, memory, devices, ports, connections, or resources.

---

7. Semantic Requirement Versus Physical Realization

Every hardware construct must preserve this distinction:

semantic requirement
        ≠
implementation preference
        ≠
target constraint
        ≠
physical allocation

For example:

requires quantum;

means the computation requires quantum capability.

It does not mean:

use vendor X;
use device Y;
use exactly N physical qubits;
use topology Z;

Similarly:

requires resource.memory >= workload_memory;

describes a requirement.

It does not allocate memory.

And:

prefer accelerator;

is not equivalent to:

must execute on GPU device 7.

This separation is fundamental to POCO-REAF.

---

8. Universal Hardware Contract Model

Hardware syntax must support these distinct concepts:

resource
capability
requirement
constraint
preference
hint
target
placement
mapping
property

They must not collapse into one generic concept.

Resource

Something that can be consumed, reserved, shared, or otherwise relevant to execution.

Examples:

- compute capacity;
- memory;
- bandwidth;
- accelerator capacity;
- storage;
- communication capacity.

Capability

Something a target can do.

Examples:

- quantum computation;
- tensor acceleration;
- floating-point operation;
- programmable logic;
- cryptographic acceleration.

Requirement

Something the program needs for valid execution.

Constraint

A condition that must hold.

Preference

A desirable implementation property that does not change program semantics.

Hint

A non-binding optimization or realization suggestion.

Target

A target class or target contract.

It does not inherently identify a physical machine.

Placement

Where an abstract computation or resource should preferably be realized.

Actual placement belongs downstream.

Mapping

A relationship between abstract resources or semantic entities.

Physical allocation remains downstream.

---

9. Hardware Directory Ownership

The hardware grammar is intentionally decomposed into specialized files.

The ownership model is:

hardware/
├── README.md                 ← subsystem contract
├── hardware.g4               ← composition/root hardware grammar
├── devices.g4                ← device declarations
├── resources.g4              ← resource syntax
├── capabilities.g4           ← capability syntax
├── topology.g4               ← topology contracts
├── placement.g4              ← placement intent
├── targets.g4                ← target declarations
├── accelerators.g4           ← accelerator contracts
├── fpga.g4                   ← FPGA contracts
├── asic.g4                   ← ASIC contracts
├── cpu.g4                    ← CPU contracts
├── gpu.g4                    ← GPU contracts
└── quantum-device.g4         ← quantum-device contracts

Where additional hardware grammar files exist, they must obey the same ownership model.

---

10. "hardware.g4"

Purpose

"hardware.g4" is the composition grammar for the hardware namespace.

It defines the common hardware declaration structure and delegates specialized concepts to their owning grammar files.

Owns

- hardware declaration entry;
- hardware body;
- common modifiers;
- common attributes;
- generic parameters;
- common contracts;
- hardware composition;
- common properties;
- common connection structures;
- integration of specialized hardware declarations.

Does not own

It must not duplicate:

- CPU grammar;
- GPU grammar;
- FPGA grammar;
- ASIC grammar;
- accelerator grammar;
- QPU grammar;
- device grammar;
- resource grammar;
- capability grammar;
- target grammar;
- placement grammar.

The existing grammar already follows this delegation model. For example, specialized accelerator and QPU declarations are intended to be delegated to their dedicated files.

Integration contract

"hardware.g4" consumes rules from the specialized hardware grammar files.

The canonical root grammar must consume the hardware declaration entry point.

No specialized grammar may require "hardware.g4" to know implementation details of the downstream subsystem.

---

11. "devices.g4"

Purpose

Defines syntax for abstract and classified hardware devices.

Owns

- device declarations;
- device identity at the language-contract level;
- device classification;
- abstract device properties;
- device relationships.

Does not own

- physical enumeration;
- operating-system device handles;
- hardware addresses;
- driver APIs;
- runtime discovery.

A device identifier in source is a semantic name or contract unless explicitly defined otherwise by a target/deployment layer.

Existing repository material explicitly positions this file as complementary to "hardware.g4" rather than replacing its ownership of hardware structure.

---

12. "resources.g4"

Purpose

Defines syntax for hardware resource declarations and resource expressions.

Owns

- resource declarations;
- resource quantities;
- resource relationships;
- resource properties;
- resource references.

Does not own

- runtime resource accounting;
- allocation;
- reservation;
- scheduling;
- resource discovery.

Resource quantities must remain expression-based.

This means the language can express:

resource compute;
resource memory;
resource bandwidth;

without imposing a fixed physical capacity.

The existing resource grammar explicitly treats resource syntax as an integration point with "hardware.g4" and the wider resource model.

---

13. "capabilities.g4"

Purpose

Defines capability declarations and references.

Owns

- capability names;
- capability declarations;
- capability values;
- capability relationships.

Does not own

- capability discovery;
- runtime feature probing;
- device drivers;
- target selection.

A capability such as:

capability.quantum

must be interpreted semantically by downstream capability analysis.

---

14. "targets.g4"

Purpose

Defines target classes and target contracts.

Owns

- target declarations;
- target properties;
- target-level contracts;
- target-independent target descriptions.

Does not own

- physical target discovery;
- compiler backend implementation;
- deployment;
- device allocation.

A target declaration must be able to represent a class of machines rather than one machine.

The existing file describes itself as a rule intended for consumption by "hardware.g4", making that delegation contract part of this subsystem architecture.

---

15. "topology.g4"

Purpose

Defines syntax for topology requirements and topology capabilities.

Owns

- topology models;
- connectivity relationships;
- abstract degree requirements;
- abstract distance requirements;
- topology properties.

Does not own

- physical topology discovery;
- routing;
- placement algorithms;
- graph optimization.

Topology syntax describes what is required or declared.

It does not discover a machine's actual topology.

---

16. "placement.g4"

Purpose

Defines placement intent.

Owns

- placement declarations;
- affinity;
- grouping;
- proximity requirements;
- avoidance intent;
- placement properties.

Does not own

- routing;
- physical placement;
- resource allocation;
- scheduling.

Placement is a semantic input to downstream compilation.

The existing placement grammar explicitly describes itself as a rule consumed by "hardware.g4", confirming this integration boundary.

---

17. "accelerators.g4"

Purpose

Defines accelerator contracts.

Owns

- accelerator declarations;
- accelerator capabilities;
- accelerator interfaces;
- accelerator parameters;
- accelerator resource relationships.

Does not own

- CUDA/ROCm/etc. implementation;
- physical GPU allocation;
- accelerator runtime;
- scheduling.

The existing architecture explicitly requires "hardware.g4" to consume the accelerator declaration rule and move accelerator ownership out of the root hardware grammar.

---

18. "cpu.g4"

Purpose

Defines CPU-class hardware contracts.

Owns

- abstract CPU declarations;
- CPU capability descriptions;
- CPU-specific semantic properties;
- CPU resource contracts.

Does not own

- a fixed number of cores;
- fixed register counts;
- fixed ISA assumptions;
- physical CPU enumeration;
- operating-system CPU discovery.

CPU syntax must remain extensible across current and future CPU architectures.

---

19. "gpu.g4"

Purpose

Defines GPU-class hardware contracts.

Owns

- GPU declarations;
- GPU capabilities;
- GPU resource relationships;
- GPU execution properties.

Does not own

- fixed GPU count;
- fixed warp/wavefront assumptions;
- physical GPU identifiers;
- runtime allocation;
- vendor-specific execution.

Vendor-specific information must use qualified extension mechanisms rather than permanently expanding the core language.

The existing GPU grammar already establishes integration with the common hardware/resource layer.

---

20. "fpga.g4"

Purpose

Defines FPGA-class hardware contracts.

Owns

- FPGA declarations;
- programmable-logic capabilities;
- FPGA resource contracts;
- FPGA-specific semantic properties;
- FPGA parameters.

Does not own

- synthesis;
- place-and-route;
- physical FPGA device discovery;
- fixed LUT counts;
- fixed BRAM counts;
- fixed DSP counts.

The existing FPGA grammar explicitly defines its entry point as something that "hardware.g4" must delegate to.

---

21. "asic.g4"

Purpose

Defines ASIC-class hardware contracts.

Owns

- ASIC declarations;
- technology-independent ASIC contracts;
- implementation constraints expressible at the language level;
- ASIC properties.

Does not own

- foundry selection;
- physical process discovery;
- transistor-level synthesis;
- physical design;
- routing;
- timing closure.

---

22. "quantum-device.g4"

Purpose

Defines quantum-device hardware contracts.

Owns

- quantum-device declarations;
- quantum-device capabilities;
- device-level quantum resource relationships;
- abstract QPU properties.

Does not own

- quantum gates;
- quantum circuit semantics;
- "quantum::ir";
- QEC algorithms;
- ZQN noise semantics;
- calibration;
- physical qubit discovery;
- quantum routing;
- quantum scheduling.

Quantum programming syntax belongs to "grammar/quantum/".

Quantum device syntax belongs here.

The canonical semantic boundary remains "quantum::ir".

The existing quantum-device grammar explicitly establishes delegation from "hardware.g4".

---

23. Hardware and HDL Separation

The hardware grammar must not become a duplicate HDL grammar.

The distinction is:

grammar/hardware/
    hardware contracts and hardware realization intent

grammar/hdl/
    hardware behavioral/structural description

For example:

A hardware contract may say:

requires capability.reconfigurable_logic;

while HDL may describe:

module ...
signal ...
process ...

The hardware grammar may reference HDL-related capabilities or contracts, but it must not duplicate HDL behavioral syntax.

This prevents:

hardware.g4
    ↕
hdl.g4

from becoming a circular grammar dependency.

The dependency direction is:

hardware syntax ──► semantic hardware model
HDL syntax      ──► semantic HDL model

semantic models ──► compiler/lowering

---

24. Hardware and Quantum Separation

The hardware grammar must not duplicate quantum programming syntax.

Correct separation:

grammar/quantum/
    quantum computation

grammar/hardware/
    quantum device capability

For example:

quantum circuit
    ↓
quantum::ir
    ↓
routing/scheduling
    ↓
hardware target

The hardware grammar can describe:

- QPU capability;
- quantum-device resource requirements;
- abstract topology;
- device characteristics;
- target constraints.

It must not construct or replace "quantum::ir".

---

25. Hardware and QEC

The hardware grammar may express a requirement or capability associated with error correction.

It must not implement QEC.

For example, syntax may represent:

requires capability.error_correction;

but the implementation of:

- surface codes;
- repetition codes;
- syndrome extraction;
- decoding;
- logical error correction;

belongs to the quantum/QEC subsystem.

The grammar is a declaration layer only.

---

26. Hardware and ZQN

ZQN owns quantum noise and fault semantics.

Hardware grammar may declare capabilities or constraints relevant to noise handling.

It must not define the canonical ZQN fault model.

Correct architecture:

hardware grammar
      │
      │ declares capabilities/requirements
      ▼
semantic analysis
      │
      ├── hardware model
      └── quantum model
               │
               ▼
              ZQN

The hardware grammar must not duplicate:

- noise channels;
- fault classification;
- correlated-fault semantics;
- leakage semantics;
- loss semantics;
- erasure semantics.

---

27. Hardware and Scheduling

The grammar may express:

- timing requirements;
- latency preferences;
- ordering-related constraints;
- placement intent.

It must not implement scheduling.

Correct boundary:

grammar
  ↓
semantic timing/resource requirements
  ↓
scheduling subsystem
  ↓
schedule

Scheduling remains responsible for:

- ASAP;
- ALAP;
- resource-aware scheduling;
- dependency scheduling;
- timing;
- alignment;
- delays;
- dynamic execution ordering.

---

28. Hardware and Routing

The grammar may express topology requirements and placement intent.

It must not implement routing.

Correct boundary:

hardware topology requirement
          ↓
routing subsystem
          ↓
physical realization

No grammar rule may secretly implement:

- shortest-path routing;
- qubit routing;
- network routing;
- physical placement algorithms.

---

29. Hardware and Optimization

Hardware grammar may provide:

- optimization hints;
- target preferences;
- resource preferences.

It must not implement optimization.

Correct boundary:

source preference
      ↓
semantic representation
      ↓
optimization

Preferences must never silently change program semantics.

---

30. Hardware and Resilience

Hardware grammar may describe:

- reliability requirements;
- availability requirements;
- fault-tolerance requirements;
- hardware capability requirements.

It must not implement recovery policy.

Resilience remains responsible for deciding whether to:

- retry;
- restart;
- resume;
- rollback;
- remap;
- reroute;
- reschedule;
- recompile;
- reoptimize;
- change QEC;
- mitigate;
- switch backend;
- quarantine;
- abort.

---

31. Hardware and Runtime

The grammar must not directly execute hardware.

There must be no grammar actions that:

- open devices;
- allocate resources;
- send network requests;
- access files;
- communicate with drivers;
- mutate runtime state;
- invoke hardware APIs.

The grammar must remain action-free.

This is especially important because the Rust implementation must remain safe and compatible with Rust 1.97 / 1.97.1.

---

32. ANTLR Contract

The grammar subsystem must remain compatible with the repository's ANTLR architecture.

The hardware grammar must use the canonical token vocabulary.

The current architecture identifies "ZamaniTokens" as the lexer vocabulary consumed by "hardware.g4".

The hardware grammar must therefore not introduce an independent lexer.

The architecture must remain:

ZamaniTokens
      ↓
hardware parser rules

rather than:

HardwareLexer
      ↓
HardwareParser

unless the entire repository intentionally adopts a different unified lexer architecture.

---

33. Expression Integration

Hardware expressions must reuse the canonical expression grammar.

Hardware-specific syntax must not create an independent expression language.

The following concepts should therefore eventually lower through the common expression system:

- quantities;
- comparisons;
- arithmetic;
- capability values;
- resource values;
- generic arguments;
- properties;
- constraints;
- preferences.

This avoids duplicated operator precedence and inconsistent semantics.

---

34. Type Integration

Hardware-specific type references must integrate with the canonical type system.

Hardware syntax may introduce hardware-domain type names, but it must not redefine:

- primitive types;
- arrays;
- tuples;
- generics;
- references;
- options;
- results.

Hardware types should become semantic types through the type-checking layer.

---

35. Name Integration

Hardware names must use the canonical naming system.

Do not create a second identifier model.

Qualified names must be compatible with:

grammar/core/names.g4
grammar/core/paths.g4
grammar/core/qualified-names.g4

The existing core grammar documentation already treats names and qualified identities as shared infrastructure across hardware and quantum domains.

---

36. Generic Hardware Parameters

Hardware generics are semantic parameters.

They may represent concepts such as:

Width
Lanes
Capacity
Precision
Dimensions
Throughput
Latency

but must not be interpreted as fixed machine constants.

For example:

hardware Accelerator<Width, Lanes> {
    ...
}

does not mean the grammar has a fixed value for either parameter.

Values may be supplied by:

- source-level specialization;
- compilation;
- target selection;
- resource discovery;
- runtime configuration.

---

37. Resource Quantities

Resource quantities must be expressions.

This allows:

resource memory >= required_memory;
resource compute >= required_compute;
resource bandwidth >= required_bandwidth;

without hard-coding physical capacity.

Quantities may depend on:

- generic parameters;
- compile-time expressions;
- input characteristics;
- workload properties;
- target capabilities;
- runtime resource availability.

---

38. Target Independence

The grammar must distinguish:

target class

from:

physical target

For example:

target quantum;

may identify a target category.

It must not silently identify a physical QPU.

Likewise:

target gpu;

must not imply a specific GPU vendor, model, memory capacity, or device identifier.

---

39. Vendor Extensions

Vendor-specific features must not pollute the permanent core grammar.

Prefer:

vendor.feature

or another qualified extension mechanism over introducing permanent keywords for every vendor.

Vendor extensions must remain:

- namespaced;
- versionable;
- capability-based;
- optional;
- semantically validated downstream.

A vendor extension must never become a hidden dependency of portable core syntax.

---

40. Future Hardware

The grammar must support future technologies without requiring a redesign of the entire grammar.

Potential future classes include:

- neuromorphic processors;
- photonic processors;
- molecular computing;
- biological computing;
- analog accelerators;
- optical accelerators;
- quantum annealers;
- fault-tolerant QPUs;
- reconfigurable compute fabrics;
- future unknown architectures.

The grammar should therefore describe capabilities and contracts rather than attempting to enumerate every possible machine.

---

41. Physical Identity Boundary

Physical identifiers must be downstream concepts unless explicitly required by a deployment-specific language layer.

The core grammar must not hard-code:

device0
device1
gpu0
gpu1
qpu0
qpu1

as intrinsic language entities.

Likewise, it must not encode:

- PCI addresses;
- MMIO addresses;
- physical socket numbers;
- fixed node names;
- cloud instance IDs.

Those belong to deployment, target configuration, or runtime layers.

---

42. Topology Boundary

Topology declarations represent:

what topology is required

or:

what topology is described

They do not perform:

discover topology

or:

allocate topology

This permits the same program to operate on:

- line topology;
- grid topology;
- mesh;
- torus;
- arbitrary graph;
- fully connected resources;
- dynamically changing resources.

---

43. Placement Boundary

Placement is intent.

For example:

near = memory;
affinity = accelerator;
avoid = congested_resource;

must be interpreted downstream.

The grammar must not require a fixed number of placement slots.

---

44. Timing Boundary

Hardware timing syntax may express semantic timing contracts.

Examples include:

- latency;
- duration;
- timing relationships;
- clock relationships;
- synchronization requirements.

The grammar must not encode one hardware clock frequency as a universal language assumption.

Physical timing is target-dependent.

---

45. Memory Boundary

Hardware memory declarations describe memory contracts.

They must not assume:

64 KB
1 MB
8 GB
80 GB

as universal machine properties.

Memory requirements should be expressible through:

- expressions;
- resource constraints;
- capabilities;
- target properties.

---

46. Accelerator Boundary

An accelerator is a semantic execution resource.

The grammar must not assume:

- one accelerator;
- two accelerators;
- a fixed accelerator topology;
- fixed accelerator memory;
- fixed accelerator lanes.

The number and characteristics of available accelerators are determined downstream.

---

47. Hardware Properties

Properties must be extensible.

Core syntax should provide a stable way to represent properties without turning every possible future property into a keyword.

Properties should support:

- qualified names;
- typed values where semantic analysis permits;
- expressions;
- versioning;
- extension namespaces.

Unknown properties must be handled according to the language's semantic validation policy rather than silently changing meaning.

---

48. Attributes

Attributes provide metadata and annotations.

They must remain separate from semantic hardware declarations.

Attributes may be used for:

- documentation;
- compilation metadata;
- dialect information;
- optimization hints;
- tooling;
- compatibility;
- source mapping.

Attributes must not become a hidden replacement for formal semantic requirements.

---

49. Error Handling

Grammar errors must be deterministic and diagnosable.

The parser must report:

- unexpected tokens;
- malformed declarations;
- malformed resource expressions;
- malformed capability expressions;
- malformed topology declarations;
- malformed target declarations;
- invalid delimiters;
- incomplete constructs.

Semantic errors belong to semantic analysis.

For example:

requires capability.foo;

may be syntactically valid while semantically invalid because "capability.foo" is unavailable or undefined.

The parser must not attempt to solve that semantic problem.

---

50. Determinism

Given identical source and identical lexer/parser configuration:

source A
    ↓
lexer
    ↓
parser
    ↓
syntax tree

must produce deterministic results.

There must be no:

- random parser behavior;
- runtime hardware discovery;
- environment-dependent grammar actions;
- network-dependent parsing;
- filesystem-dependent parsing.

---

51. Security

The grammar subsystem must be passive.

Parsing source must not:

- execute arbitrary code;
- access the filesystem;
- access the network;
- execute shell commands;
- access hardware;
- load arbitrary native libraries.

The grammar itself must remain action-free.

Rust integration must use safe Rust only.

No "unsafe" code is permitted in the grammar infrastructure.

---

52. Compatibility

The hardware grammar must be versioned with the Zamani language.

Compatibility must distinguish:

source-language compatibility
grammar compatibility
AST compatibility
semantic compatibility
IR compatibility
target compatibility
runtime compatibility

A hardware syntax change must not silently alter the meaning of existing source programs.

Breaking changes require:

- version identification;
- migration documentation;
- deprecation period where appropriate;
- compatibility tests.

---

53. Backward Compatibility

Existing valid hardware syntax must be preserved unless it is:

- demonstrably incorrect;
- ambiguous;
- unsafe;
- incompatible with the language specification;
- architecturally duplicated;
- impossible to preserve without violating the semantic model.

When a construct is replaced:

old syntax
    ↓
compatibility layer
    ↓
new semantic representation

must be preferred over silent removal.

---

54. AST Contract

The hardware grammar produces syntax that must be representable in the canonical AST.

The AST must preserve:

- source spans;
- declaration identity;
- modifiers;
- generic parameters;
- properties;
- resources;
- capabilities;
- requirements;
- constraints;
- preferences;
- target information;
- topology information;
- placement information;
- mappings;
- hardware composition.

The grammar must not require the AST to contain physical device state.

---

55. Semantic Contract

Semantic analysis is responsible for determining whether a parsed hardware declaration is meaningful.

Examples:

resource.memory >= x

must be checked for:

- valid resource identity;
- valid expression;
- compatible units/types;
- legal comparison.

Likewise:

requires capability.quantum

must be checked against the semantic capability model.

The parser does not perform these checks.

---

56. IR Integration

The hardware grammar must lower indirectly.

Correct:

source
  ↓
grammar
  ↓
AST
  ↓
semantic model
  ↓
canonical IR / hardware model

Incorrect:

source
  ↓
hardware.g4
  ↓
quantum::ir

The grammar must never construct "quantum::ir".

For quantum operations, the canonical quantum semantic boundary remains "quantum::ir".

For hardware contracts, the repository's hardware semantic representation becomes the downstream boundary.

---

57. Compiler Integration

The compiler may consume hardware semantic information for:

- target selection;
- capability validation;
- resource analysis;
- lowering;
- optimization;
- routing;
- scheduling;
- synthesis;
- code generation.

The grammar must expose enough source information for these stages without embedding their algorithms.

---

58. Runtime Integration

Runtime systems may use compiled hardware metadata to:

- select available resources;
- query capabilities;
- allocate resources;
- dispatch execution;
- adapt to runtime conditions.

The runtime must not need to reparse source merely to discover hardware properties.

The compiler/semantic layer should produce the appropriate representation.

---

59. Tooling Integration

The hardware grammar must support:

- syntax highlighting;
- parser diagnostics;
- IDE completion;
- source navigation;
- documentation generation;
- formatting;
- semantic inspection;
- syntax tree inspection;
- language-server integration.

Stable rule names and predictable syntax are therefore public compatibility surfaces.

---

60. Cross-Domain Integration

Hardware syntax must support combinations such as:

classical + hardware
quantum + hardware
HDL + hardware
classical + quantum + hardware
quantum + HDL + hardware
AI + hardware
distributed + hardware
AI + quantum + hardware
classical + quantum + HDL + hardware

The domains must compose through shared:

- names;
- types;
- expressions;
- capabilities;
- resources;
- requirements;
- constraints;
- effects;
- semantic models.

They must not create mutually incompatible mini-languages.

---

61. No Circular Dependencies

The hardware grammar dependency graph must remain acyclic.

Preferred direction:

lexer
  ↓
core names/types/expressions
  ↓
hardware grammar
  ↓
AST
  ↓
semantic analysis
  ↓
IR
  ↓
compiler
  ↓
runtime

Not:

hardware grammar
    ↕
runtime

or:

hardware grammar
    ↕
quantum IR

or:

hardware grammar
    ↕
HDL grammar

Specialized grammars may share common grammar infrastructure, but must not recursively redefine one another.

---

62. Dependency Rules Between Hardware Files

The intended dependency direction is:

common lexer
    ↓
core names/types/expressions
    ↓
hardware common grammar
    ↓
resources / capabilities / devices / targets
    ↓
topology / placement / accelerators
    ↓
CPU / GPU / FPGA / ASIC / QPU

"hardware.g4" is the composition point.

Specialized grammars must not depend on physical runtime implementations.

---

63. Completion Contract for Every Hardware Grammar File

Every ".g4" file in this directory is complete only when all of the following are true:

Purpose

Its semantic purpose is explicitly documented.

Ownership

Every rule has one owner.

Non-ownership

Duplicated responsibilities are explicitly rejected.

Inputs

The source tokens and grammar rules consumed are identified.

Outputs

The parser rules exported to consumers are identified.

Dependencies

Every grammar dependency is documented.

Upstream contract

The file states which shared grammar infrastructure it consumes.

Downstream contract

The file states which parser/semantic components consume its rules.

AST contract

Every construct has a defined AST representation.

Semantic contract

Every construct has a defined semantic interpretation.

IR contract

The downstream representation is identified.

Compiler contract

Compiler consumers are identified.

Runtime contract

Runtime relevance is documented without introducing runtime behavior into the grammar.

Tooling contract

IDE/formatter/diagnostic implications are known.

Cross-domain contract

Interactions with other domains are explicitly defined.

Tests

Positive, negative, boundary, compatibility, and integration tests exist.

Scalability

No artificial machine-size limits exist.

Hard-coding audit

All fixed values are justified or removed.

Determinism

Parsing is deterministic.

Safety

No embedded executable actions or unsafe Rust exist.

---

64. Required Testing Model

Testing must exist at multiple levels.

Lexer tests

Verify:

- hardware keywords;
- identifiers;
- qualified names;
- literals;
- operators;
- punctuation.

Parser tests

Verify every public rule.

Positive tests

Examples must cover:

- minimal hardware;
- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- accelerator;
- resources;
- capabilities;
- targets;
- topology;
- placement;
- mappings;
- connections.

Negative tests

Verify rejection of:

- malformed declarations;
- missing identifiers;
- invalid delimiters;
- malformed expressions;
- invalid generic syntax;
- invalid topology syntax;
- invalid placement syntax.

Boundary tests

Test:

- zero-length collections where legal;
- one-element collections;
- very large declarations;
- deeply nested expressions;
- large generic lists;
- large property sets;
- large hardware compositions.

The tests must not impose artificial maximums merely because the test fixture is small.

---

65. Scalability Tests

The grammar must explicitly test that it can parse source describing arbitrarily large conceptual systems, limited only by available parser/compiler memory and execution resources.

Test dimensions should include:

number of hardware declarations
number of resources
number of capabilities
number of ports
number of connections
number of instances
number of topology relationships
number of generic parameters
number of properties

The tests must verify absence of grammar-defined limits.

A test must never establish a language rule such as:

only 64 resources allowed

merely because the test uses 64 resources.

---

66. POCO-REAF Scalability Test

A representative test must establish that one source description can remain semantically valid while varying the available target resources.

Conceptually:

same source
    │
    ├── tiny target
    ├── CPU target
    ├── GPU target
    ├── FPGA target
    ├── QPU target
    ├── heterogeneous target
    └── distributed target

The source semantics remain unchanged.

Only target realization changes.

---

67. Cross-Domain Tests

Mandatory combinations include:

classical + hardware
quantum + hardware
HDL + hardware
classical + quantum + hardware
quantum + HDL + hardware
AI + hardware
distributed + hardware
AI + quantum + hardware
classical + quantum + HDL + hardware

The tests must verify that grammar composition does not introduce ambiguity.

---

68. Round-Trip Tests

Where a canonical formatter/printer exists:

source
 ↓
lexer
 ↓
parser
 ↓
AST
 ↓
printer
 ↓
parser

must preserve semantic meaning.

Whitespace and formatting may change.

Meaning must not.

---

69. Hard-Coding Audit

Every hardware grammar change must be checked for:

- fixed machine counts;
- fixed device counts;
- fixed CPU counts;
- fixed core counts;
- fixed GPU counts;
- fixed FPGA counts;
- fixed qubit counts;
- fixed memory sizes;
- fixed port counts;
- fixed topology;
- fixed addresses;
- fixed device IDs;
- fixed vendor assumptions;
- fixed accelerator counts;
- fixed deployment topology.

Each fixed value must be classified as:

1. language semantic requirement;
2. target-specific requirement;
3. resource constraint;
4. implementation limitation;
5. accidental hard-coding;
6. test-only limitation;
7. documentation-only limitation.

Only categories 1–3 may normally survive, and even then the value must not become an accidental grammar-level universal limit.

---

70. Examples of Forbidden Design

The hardware grammar must never evolve toward:

hardware GPU<8>

if "8" is intended as a universal maximum.

Nor:

hardware QPU<64 qubits>

as a grammar restriction.

Nor:

device gpu0;
device gpu1;

as the only representable hardware model.

Nor:

connect q0 to q1;

if this implicitly requires exactly two physical qubits.

Instead, quantities and relationships should remain semantic and parameterized.

---

71. Acceptable Design

The grammar should support concepts equivalent to:

hardware Accelerator<Capacity> {
    requires capability.parallel_compute;
    resource compute >= Capacity;
}

where "Capacity" is a semantic parameter.

Likewise:

hardware QuantumTarget<QubitCount> {
    requires capability.quantum;
    resource qubits >= QubitCount;
}

The actual available capacity is determined by target and runtime systems.

---

72. Hardware Abstraction Principle

The language should allow this conceptual flow:

program requirement
        ↓
abstract hardware capability
        ↓
available target
        ↓
resource matching
        ↓
mapping
        ↓
routing
        ↓
scheduling
        ↓
runtime
        ↓
physical hardware

No earlier layer should silently absorb the responsibility of a later layer.

---

73. Production Documentation Requirements

This README is the subsystem-level architectural contract.

Each specialized grammar file must additionally contain its own header documenting:

File
Purpose
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
AST Contract
Semantic Contract
IR Integration
Compiler Integration
Runtime Integration
Tooling Integration
Cross-Domain Integration
Tests
Scalability
Hard-Coding Audit
Completion Criteria

A specialized grammar file is not considered complete merely because ANTLR accepts it.

---

74. Repository Integration Matrix

Layer| Hardware grammar relationship
Lexer| Consumes canonical Zamani tokens
Core names| Reuses canonical identifiers and qualified names
Types| Reuses canonical type system
Expressions| Reuses canonical expressions
AST| Produces hardware syntax nodes
Semantic analysis| Interprets hardware contracts
Capability system| Validates capability requirements
Resource system| Resolves resource requirements
Classical IR| Receives classical portions downstream
"quantum::ir"| Receives quantum semantics downstream; grammar never creates it
QEC| Consumes relevant semantic requirements downstream
ZQN| Owns noise/fault semantics; hardware grammar does not duplicate them
Optimization| Consumes hardware constraints/preferences
Routing| Consumes topology/placement information
Scheduling| Consumes resource/timing information
Hardware HAL| Realizes abstract hardware requirements
Calibration| Supplies runtime/target data downstream
Resilience| Uses hardware health/capability information downstream
Compiler| Performs lowering and target realization
Runtime| Performs execution
Tooling| Uses grammar/parser contracts
Tests| Validate syntax and integration
Documentation| Defines language-facing behavior

---

75. Ownership Matrix

Concept| Owner
Identifiers| "grammar/core"
Expressions| "grammar/expressions"
General types| "grammar/types"
Hardware declarations| "hardware/hardware.g4"
Hardware devices| "hardware/devices.g4"
Resources| "hardware/resources.g4"
Capabilities| "hardware/capabilities.g4"
Targets| "hardware/targets.g4"
Topology| "hardware/topology.g4"
Placement intent| "hardware/placement.g4"
Accelerators| "hardware/accelerators.g4"
CPU| "hardware/cpu.g4"
GPU| "hardware/gpu.g4"
FPGA| "hardware/fpga.g4"
ASIC| "hardware/asic.g4"
Quantum devices| "hardware/quantum-device.g4"
Quantum computation| "grammar/quantum"
Quantum canonical semantics| "quantum::ir"
QEC| quantum/QEC subsystem
Noise/fault model| ZQN
Routing| routing subsystem
Scheduling| scheduling subsystem
Optimization| optimization subsystem
Physical discovery| hardware HAL
Runtime execution| runtime
Recovery decisions| resilience

---

76. Implementation Order

Hardware grammar implementation must proceed only after its upstream contracts are stable.

Recommended order:

1. specification/grammar-authority.md
        ↓
2. lexer token contract
        ↓
3. core names
        ↓
4. core paths/qualified names
        ↓
5. core expressions
        ↓
6. core types
        ↓
7. hardware README contract
        ↓
8. resources.g4
        ↓
9. capabilities.g4
        ↓
10. devices.g4
        ↓
11. targets.g4
        ↓
12. topology.g4
        ↓
13. placement.g4
        ↓
14. accelerators.g4
        ↓
15. cpu.g4
        ↓
16. gpu.g4
        ↓
17. fpga.g4
        ↓
18. asic.g4
        ↓
19. quantum-device.g4
        ↓
20. hardware.g4 composition
        ↓
21. hardware semantic AST integration
        ↓
22. semantic validation
        ↓
23. compiler integration
        ↓
24. target/HAL integration
        ↓
25. runtime integration
        ↓
26. cross-domain tests

The README must be completed before implementing or substantially changing the dependent grammar files because it establishes their ownership contracts.

---

77. Definition of Done

"grammar/hardware/" is production-ready only when:

- the grammar authority is unambiguous;
- every grammar file has exactly defined ownership;
- no duplicate hardware grammar exists;
- "hardware.g4" is a composition grammar;
- specialized declarations are delegated correctly;
- the canonical lexer is reused;
- canonical names are reused;
- canonical expressions are reused;
- canonical types are reused;
- hardware syntax is separate from HDL behavior;
- hardware syntax is separate from quantum computation;
- hardware syntax does not create "quantum::ir";
- QEC remains outside grammar ownership;
- ZQN remains outside grammar ownership;
- routing remains outside grammar ownership;
- scheduling remains outside grammar ownership;
- optimization remains outside grammar ownership;
- resilience remains outside grammar ownership;
- physical discovery remains outside grammar ownership;
- runtime execution remains outside grammar ownership;
- no unsafe code is introduced;
- grammar actions are absent;
- no filesystem/network/device access occurs during parsing;
- no fixed machine-size limits exist;
- resource quantities are scalable;
- target descriptions are abstract;
- capabilities and requirements remain distinct;
- constraints and preferences remain distinct;
- placement remains intent;
- topology remains declarative;
- vendor extensions remain extensible;
- versioning is defined;
- diagnostics are deterministic;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- cross-domain tests exist;
- round-trip tests exist where applicable;
- hard-coding audits pass;
- ANTLR generation succeeds;
- Rust 1.97 / 1.97.1 repository builds succeed;
- repository tests succeed;
- semantic integration succeeds;
- compiler integration succeeds;
- target integration succeeds;
- runtime integration succeeds.

---

78. Final Architectural Rule

The hardware grammar must always preserve the following invariant:

Zamani source
    describes
        computation
        intent
        requirements
        capabilities
        constraints
        preferences
        abstract hardware relationships

NOT

    a frozen description of today's machine.

Therefore:

One program
    ↓
One semantic meaning
    ↓
Many target classes
    ↓
Many architectures
    ↓
Many hardware configurations
    ↓
Many scales
    ↓
Many execution environments
    ↓
Future platforms

The hardware grammar is successful when a Zamani developer can express the hardware relationship required by a computation without rewriting the computation merely because the available machine changes.

That is the hardware-language foundation required for:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever»

and for:

«Zamani — From Atom to Everywhere.»

---

79. Required Maintenance Rule

Whenever a new hardware technology is added, developers must first determine whether it can be represented using existing:

- capabilities;
- resources;
- requirements;
- constraints;
- preferences;
- targets;
- properties;
- generic parameters;
- dialect extensions.

A new core grammar file or keyword must only be introduced when existing abstractions cannot represent the technology without loss of semantic clarity.

This prevents the grammar from growing through unnecessary permanent special cases.

---

80. Required Change Review

Every future modification under "grammar/hardware/" must answer:

1. What new syntax is introduced?
2. Which file owns it?
3. Why does that file own it?
4. Does another grammar already own the concept?
5. Does the change introduce a new keyword?
6. Could a qualified name or property represent it instead?
7. Does it introduce a physical-machine assumption?
8. Does it introduce a fixed capacity?
9. Does it create a dependency cycle?
10. Does it alter AST requirements?
11. Does it alter semantic meaning?
12. Does it affect "quantum::ir"?
13. Does it affect QEC?
14. Does it affect ZQN?
15. Does it affect routing?
16. Does it affect scheduling?
17. Does it affect optimization?
18. Does it affect hardware HAL?
19. Does it affect runtime?
20. Does it preserve POCO-REAF?
21. Does it preserve deterministic parsing?
22. Does it preserve backward compatibility?
23. Are positive and negative tests added?
24. Are scalability tests added?
25. Has the hard-coding audit passed?

A change is not complete until all applicable questions have explicit answers.

---

81. Non-Negotiable Invariants

The following are permanent invariants of "grammar/hardware/":

NO fixed machine-size limits
NO fixed qubit limits
NO fixed CPU limits
NO fixed GPU limits
NO fixed FPGA limits
NO fixed accelerator limits
NO physical device enumeration
NO physical addresses in the core hardware grammar
NO runtime execution in grammar actions
NO unsafe Rust
NO duplicated quantum IR
NO duplicated QEC model
NO duplicated ZQN model
NO scheduling implementation
NO routing implementation
NO optimization implementation
NO calibration implementation
NO hardware discovery
NO arbitrary vendor keywords
NO circular grammar dependencies
NO hidden machine assumptions

And:

YES capability-based design
YES resource-based design
YES generic parameters
YES target independence
YES extensibility
YES deterministic parsing
YES strong diagnostics
YES semantic separation
YES repository-wide integration
YES cross-domain composition
YES POCO-REAF
YES scalability limited only by available implementation resources

---

82. Final Status

This README is the architecture and ownership contract for "grammar/hardware/".

It does not replace the individual ".g4" files.

Instead, every individual ".g4" file must conform to this contract.

The implementation rule is:

README contract
      ↓
individual grammar contract
      ↓
grammar implementation
      ↓
tests
      ↓
semantic integration
      ↓
compiler integration
      ↓
runtime/target integration

No specialized hardware grammar should be considered independently complete if its ownership or integration contract contradicts this document.