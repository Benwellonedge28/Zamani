Zamani Semantic Boundaries

Path: "grammar/validation/semantic-boundaries.md"
Status: Normative architecture and validation specification
Language: Zamani
Compiler implementation baseline: Rust 1.97 / Rust 1.97.1
Rust safety: "unsafe" is prohibited
Scope: Grammar, parser, AST, semantic analysis, IR boundaries, compilation, execution, quantum, classical, HDL, hardware, distributed and future computing domains

---

1. Purpose

This document defines the authoritative semantic boundaries for the Zamani language.

Its purpose is to ensure that Zamani remains:

- hardware-independent;
- architecture-independent;
- quantum/classical interoperable;
- HDL-capable;
- scalable from the smallest supported computation to arbitrarily large computations permitted by available resources;
- deterministic where the language requires determinism;
- extensible without destabilizing existing semantics;
- compatible with future hardware and execution models;
- free of accidental machine-specific assumptions;
- cleanly separated from compiler, scheduler, optimizer, runtime, hardware and backend implementation details.

The central principle is:

«Zamani source code describes computation, intent, semantics, requirements, constraints and permitted effects. It does not accidentally describe the temporary physical machine on which that computation happens to execute.»

The target programming model is:

«Program Once → Compile Once → Run Everywhere → Anywhere → Forever (POCO-REAF).»

POCO-REAF does not mean that every program is physically executable on every machine.

It means that the program's semantic description remains portable, while compilation, resource selection, routing, scheduling, lowering, execution and adaptation determine how that semantic program is realized on a particular available target.

---

2. Normative status

This document is normative for semantic ownership.

The following priority order applies:

1. Language semantic specification
2. Canonical grammar
3. Canonical AST/semantic model
4. Canonical IR contracts
5. Compiler/lowering contracts
6. Runtime/hardware contracts
7. Explanatory documentation
8. Examples

If an explanatory document contradicts a normative semantic contract, the explanatory document is wrong.

If grammar accepts syntax that violates a semantic invariant, accepting the syntax does not make the semantic construct valid.

If an implementation cannot currently support a valid semantic construct, that is an implementation limitation rather than permission to redefine the language around that limitation.

---

3. Core semantic law

Zamani has four fundamentally different layers:

SOURCE MEANING
      |
      v
SEMANTIC REPRESENTATION
      |
      v
TARGET REALIZATION
      |
      v
PHYSICAL EXECUTION

These layers must never be conflated.

3.1 Source meaning

Source meaning includes:

- values;
- types;
- functions;
- control flow;
- data dependencies;
- quantum operations;
- measurements;
- hardware behavior when hardware behavior is explicitly the program's subject;
- resource requirements;
- capabilities;
- semantic constraints;
- effects;
- correctness requirements;
- security properties;
- communication semantics;
- timing semantics when timing is semantically observable;
- explicit approximation/error contracts;
- explicit nondeterminism;
- explicit distribution semantics.

3.2 Semantic representation

The semantic representation records what the program means independently of a particular execution device.

Examples include:

- AST;
- typed AST;
- classical IR;
- canonical quantum IR;
- hardware-independent operation descriptions;
- resource requirements;
- effect descriptions;
- capability requirements.

3.3 Target realization

Target realization determines how the semantic program is implemented.

Examples:

- instruction selection;
- gate decomposition;
- qubit mapping;
- routing;
- scheduling;
- optimization;
- FPGA synthesis;
- GPU lowering;
- CPU code generation;
- distributed placement;
- accelerator selection.

3.4 Physical execution

Physical execution includes:

- actual devices;
- physical qubits;
- physical memory;
- processor cores;
- accelerator instances;
- network links;
- clocks;
- physical topology;
- calibration;
- noise;
- queue state;
- runtime resource availability.

These properties must not silently become source semantics.

---

4. The canonical dependency direction

The architecture MUST follow:

Zamani source
    |
    v
Lexer
    |
    v
Parser
    |
    v
AST
    |
    v
Name / type / effect / capability / semantic analysis
    |
    +------------------+
    |                  |
    v                  v
Classical semantic IR  Canonical quantum::ir
    |                  |
    +--------+---------+
             |
             v
      Universal compilation
             |
      +------+------+---------+----------+
      |             |          |          |
      v             v          v          v
 optimization    routing    scheduling   lowering
      |             |          |          |
      +-------------+----------+----------+
                            |
                            v
                    Hardware abstraction
                            |
                            v
                         Runtime
                            |
                            v
                         Backend
                            |
                            v
                     Physical execution

No downstream subsystem may require the grammar to understand its internal implementation representation.

No grammar subsystem may create a competing IR merely because a domain is complicated.

---

5. Ownership boundary

5.1 Grammar owns

Grammar owns:

- lexical syntax;
- tokenization structure;
- syntactic productions;
- precedence;
- associativity;
- source-level declarations;
- source-level expressions;
- source-level statements;
- source-level domain constructs;
- syntactic metadata;
- syntactic attributes;
- syntactic resource declarations;
- syntactic capability requirements;
- syntactic constraints;
- syntactic target descriptions where target descriptions are explicitly part of source semantics.

5.2 Grammar does not own

Grammar does not own:

- type inference algorithms;
- semantic validation algorithms;
- quantum circuit optimization;
- QEC algorithms;
- noise simulation;
- hardware discovery;
- hardware calibration;
- physical qubit allocation;
- routing;
- scheduling;
- backend selection;
- runtime resource discovery;
- physical device management;
- execution;
- optimizer internals;
- compiler backend internals;
- simulator internals.

---

6. AST boundary

The AST is the structural semantic bridge between syntax and semantic analysis.

The AST MUST preserve information required to reconstruct source meaning.

It MUST NOT prematurely encode implementation decisions.

For example, a quantum AST may contain:

quantum operation
target operands
control operands
parameters
condition
source span
attributes

It must not silently contain:

physical qubit 7
IBM-style instruction ID
specific pulse schedule
device calibration value
backend-specific gate decomposition

unless those things were explicitly present in source syntax and are semantically meaningful.

---

7. Canonical IR boundary

7.1 Quantum

"quantum::ir" is the canonical semantic quantum boundary.

The grammar MUST NOT create a competing quantum representation.

The intended flow is:

Zamani quantum syntax
        |
        v
Quantum AST
        |
        v
Semantic validation
        |
        v
quantum::ir
        |
        +--> optimization
        +--> QEC integration
        +--> routing
        +--> scheduling
        +--> ZQN/fault-aware compilation
        +--> hardware realization

The quantum grammar therefore describes quantum intent.

It does not own:

- decoder algorithms;
- syndrome extraction algorithms;
- noise mathematics;
- physical calibration;
- coupling-map construction;
- pulse generation;
- hardware-specific scheduling.

Existing QEC architecture similarly establishes explicit ownership boundaries and does not duplicate partition geometry, decoder mathematics, transport or global scheduling policy.

7.2 Classical

The classical grammar must lower into the repository's canonical classical semantic representation.

If the repository has more than one classical representation, they must be explicitly classified as:

- source AST;
- typed semantic representation;
- canonical classical IR;
- target-specific representation.

They must not silently compete.

7.3 HDL

HDL constructs must lower into an appropriate hardware semantic representation.

The grammar must not turn Verilog/VHDL implementation details into universal Zamani semantics unless explicitly requested.

---

8. Source semantics versus target realization

The following distinction is mandatory.

Concept| Source semantic?| Target realization?
Compute a matrix product| Yes| 
Use quantum interference| Yes| 
Measure a quantum observable| Yes| 
Require a quantum capability| Yes| 
Require fault tolerance| Yes| 
Require a particular fidelity bound| Yes, if explicitly required| 
Use 32 qubits merely because a backend has 32| No| Yes
Physical qubit 7| Usually no| Yes
CPU core 3| No| Yes
GPU device 0| No| Yes
FPGA placement region| No| Yes
Coupling-map edge| No| Yes
Physical gate duration| Usually| Yes
Logical operation duration| Yes when semantically observable| 
Exact physical pulse| No| Yes
Network endpoint explicitly required by the application| Yes| 
Temporary network route| No| Yes
Device calibration| No| Yes
Runtime queue position| No| Yes

---

9. Requirements, constraints, capabilities and preferences

These concepts MUST remain distinct.

9.1 Requirement

A requirement states something the computation needs.

Example:

requires quantum;

means the computation requires a quantum execution capability.

It does not mean:

use vendor X;
use processor Y;
use N physical qubits;
use topology Z;

9.2 Constraint

A constraint restricts legal realizations.

Examples:

constraint logical_error_rate < bound;
constraint latency <= bound;
constraint energy <= bound;

Constraints may prevent execution on some targets.

They must not be silently transformed into fixed hardware choices.

9.3 Capability

A capability describes what a target can provide.

Examples:

- quantum computation;
- reversible computation;
- floating-point computation;
- tensor acceleration;
- FPGA synthesis;
- cryptographic acceleration;
- distributed execution.

Capabilities belong primarily to target/resource descriptions.

9.4 Preference

A preference is not a correctness requirement.

For example:

prefer accelerator;

must not mean:

accelerator is mandatory.

9.5 Hint

A hint may guide compilation.

A hint MUST NOT change program semantics unless explicitly specified as semantic behavior.

---

10. No implicit hardware commitments

The following are forbidden as implicit semantics:

q[0]
q[1]

as assumptions about available physical qubits.

Likewise forbidden:

cpu0
gpu0
core0
node0
fpga0
device0

unless explicitly declared as a target-specific program requirement.

The parser may parse identifiers containing such names.

The semantic system must distinguish an ordinary user identifier from a physical resource identity.

---

11. Resource cardinality

Resource cardinality MUST be represented symbolically or dynamically where possible.

Forbidden:

MAX_QUBITS = 32
MAX_CORES = 64
MAX_NODES = 1024

as language-level scalability restrictions.

Valid forms include:

requires qubits(expr);
requires memory(expr);
requires compute(expr);
requires nodes(expr);

where "expr" is evaluated against program requirements and available target resources.

The implementation may impose operational limits for safety or resource management, but those limits MUST NOT become language semantics.

---

12. Resource limits versus language limits

This distinction is critical.

A runtime may reject a program because:

available_memory < required_memory

without implying that Zamani has a maximum memory size.

Likewise:

available_qubits < required_qubits

does not mean Zamani supports only a fixed number of qubits.

The correct model is:

Language:
    potentially unbounded semantic domain

Compilation:
    target-dependent realization

Runtime:
    resource-bounded execution

The practical ceiling is therefore determined by available resources and implementation capacity, not by arbitrary grammar constants.

---

13. Infinite scalability interpretation

"Infinity" is a semantic scalability objective, not a claim that physical machines have infinite resources.

Zamani must avoid artificial finite language limits.

Therefore:

semantic domain
    >
implementation capacity
    >
current physical capacity

A finite implementation may necessarily encounter:

- memory exhaustion;
- integer overflow;
- compilation timeout;
- recursion/resource limits;
- backend capacity limits;
- execution time limits.

Such limitations MUST be represented as implementation/resource failures rather than encoded as permanent language maximums.

---

14. Quantum semantic boundary

Quantum syntax must distinguish:

logical resource
physical resource
semantic operation
physical realization

14.1 Logical qubit

A logical qubit belongs to the computation's semantic model.

14.2 Physical qubit

A physical qubit belongs to hardware realization.

14.3 Quantum register

A register may be a source-level grouping of quantum resources.

Its size may be:

- explicit;
- generic;
- parameterized;
- inferred where the language permits;
- runtime/resource dependent where semantically valid.

It must not have a hidden global maximum.

14.4 Gate

A gate in source describes a semantic quantum operation.

The optimizer/router/backend may later decompose it.

The grammar must not force a source program to express a backend's native gate set merely because that backend exists.

---

15. Quantum measurement boundary

Measurement is semantically observable.

Therefore measurement belongs in the source semantic model.

The compiler may decide:

- when measurement occurs;
- how measurement is implemented;

only when such transformations preserve the declared semantics.

The compiler MUST NOT automatically insert measurements merely to satisfy a backend implementation.

In particular, "measure every qubit at program termination" must not be an implicit universal language rule.

---

16. Dynamic quantum computation

Dynamic quantum programs may contain:

- mid-circuit measurement;
- classical conditions;
- conditional quantum operations;
- loops dependent on measurement results;
- runtime-controlled execution.

The semantic boundary is:

source control semantics
        |
        v
quantum/classical semantic representation
        |
        v
backend capability validation

If a backend cannot support the required dynamic behavior, the compiler must:

1. find an equivalent supported realization if one exists;
2. transform only under proven semantic preservation;
3. report target incompatibility if no valid realization exists.

It must not silently alter program meaning.

---

17. QEC boundary

The grammar may describe:

- logical qubits;
- error-correction requirements;
- fault-tolerance requirements;
- logical-error constraints;
- QEC policies or declarations where such syntax is part of the language.

The grammar does NOT own:

- decoding algorithms;
- MWPM;
- Union-Find;
- syndrome extraction mathematics;
- decoder implementation;
- correction graph storage;
- QEC resource accounting.

Those belong to QEC.

Existing QEC implementation contracts explicitly separate decoder responsibilities from syndrome extraction, logical classification and other layers.

---

18. ZQN boundary

ZQN owns fault/noise semantics.

The grammar may express:

- noise-awareness requirements;
- fault models as source-level specifications;
- resilience requirements;
- acceptable error bounds;
- execution requirements related to faults.

The grammar MUST NOT duplicate ZQN's canonical noise/fault representation.

The flow is:

Zamani syntax
     |
     v
semantic fault/noise requirement
     |
     v
ZQN integration
     |
     v
fault-aware compilation/execution

---

19. Resilience boundary

Resilience is a decision/orchestration layer.

The grammar may describe resilience requirements or policies where such concepts are intentionally part of Zamani source semantics.

It must not implement:

- retry algorithms;
- recovery state machines;
- backend discovery;
- routing;
- scheduling;
- QEC;
- noise modeling.

Those remain separate repository subsystems.

A semantic statement such as:

require fault_tolerance;

does not mean the grammar chooses a decoder, backend, scheduler or recovery algorithm.

---

20. Scheduling boundary

Scheduling is a target realization concern except where timing is explicitly semantic.

Source-level:

operation A must happen before B

is semantic.

Target-level:

start A at physical timestamp T

is generally realization.

The scheduler owns:

- dependency ordering;
- resource conflicts;
- timing;
- alignment;
- placement-aware scheduling;
- delays;
- dynamical decoupling;
- target timing.

The grammar owns only the source-level syntax necessary to express semantic timing requirements.

---

21. Optimization boundary

Optimization may transform implementation while preserving semantics.

The grammar must not contain optimizer internals.

A source-level optimization directive is permitted only if its semantics are defined.

Examples:

prefer latency;
prefer energy;
prefer throughput;

are semantic preferences.

They are not instructions to execute a specific optimization pass.

The optimizer chooses the realization.

---

22. Hardware boundary

Hardware declarations must distinguish:

hardware abstraction

from:

physical hardware instance

A source program may declare that it requires:

capability quantum;
capability tensor;
capability fpga;

without binding itself to:

device = vendor.model.serial;

A physical identity becomes semantic only when the application genuinely requires that identity.

---

23. HDL boundary

HDL is inherently closer to hardware semantics than ordinary application code.

Therefore Zamani must distinguish:

Hardware behavior

Examples:

- combinational relation;
- sequential state;
- signal propagation;
- clocked behavior;
- protocol behavior.

from:

Physical implementation

Examples:

- FPGA LUT location;
- routing channel;
- ASIC cell;
- transistor;
- vendor-specific primitive;
- physical clock tree.

The first may be semantic.

The second belongs to synthesis/target realization unless explicitly requested.

---

24. Timing boundary

Timing has three categories.

24.1 Semantic timing

Timing observable by the program or required for correctness.

Example:

operation B occurs after event A

This belongs to semantic representation.

24.2 Logical timing

Timing used by the abstract computational model.

Example:

duration = expression

This may belong to semantics.

24.3 Physical timing

Actual target-specific timing.

Examples:

- pulse duration;
- hardware propagation delay;
- device clock period;
- backend alignment slot.

These belong to target realization unless explicitly exposed by the program.

---

25. Distributed boundary

Distributed syntax may describe:

- logical workers;
- data ownership;
- communication semantics;
- consistency requirements;
- replication semantics;
- fault tolerance;
- placement constraints.

It must not assume:

N nodes
N network links
fixed cluster topology
fixed machine addresses

unless explicitly required.

Physical node discovery belongs to runtime/hardware/deployment infrastructure.

---

26. Networking boundary

Networking syntax may describe semantic communication.

For example:

send message;
receive message;
require reliable_delivery;

The implementation may choose:

- TCP;
- QUIC;
- RDMA;
- shared memory;
- hardware messaging;
- another supported mechanism.

A protocol may become semantically fixed only when protocol choice itself is part of the program's meaning.

---

27. Memory boundary

The language may express:

- ownership;
- borrowing;
- lifetime;
- allocation;
- sharing;
- locality;
- persistence;
- memory requirements.

It must not silently bind ordinary memory semantics to:

- a specific NUMA node;
- a physical address;
- a cache level;
- a fixed RAM capacity.

Physical memory placement belongs to compilation/runtime.

---

28. Concurrency boundary

The language may define:

- concurrency;
- ordering;
- synchronization;
- atomicity;
- communication;
- structured concurrency;
- cancellation;
- task dependencies.

The implementation may select:

- threads;
- fibers;
- actors;
- processes;
- accelerator kernels;
- distributed workers.

The source semantics must not depend on the number of worker threads unless explicitly required.

---

29. AI/tensor boundary

Tensor dimensions that are genuinely part of a model's mathematical meaning are semantic.

For example:

matrix<rows, columns>

may be semantically meaningful.

However:

GPU has exactly 32 tensor lanes

is target-specific.

The language must distinguish mathematical shape from accelerator implementation width.

---

30. Security boundary

Security properties are semantic when they affect program correctness or allowed behavior.

Examples:

- confidentiality requirement;
- authentication requirement;
- integrity requirement;
- cryptographic algorithm requirement;
- information-flow constraint.

The grammar must not silently select a hardware security mechanism unless explicitly specified.

Security implementation belongs to security/compiler/runtime layers.

---

31. Capability boundary

Capabilities answer:

«"What must the execution environment be capable of doing?"»

They do not answer:

«"Which exact machine must be used?"»

Therefore:

requires quantum;

is valid abstraction.

requires device "specific-device";

is a much stronger target binding and must remain explicit.

---

32. Target boundary

Target declarations are allowed where target-specific compilation is genuinely intended.

However, target declarations MUST be visibly distinguishable from universal semantics.

A useful conceptual distinction is:

program
program requirements
program preferences
target profile
target realization

A target profile must never be confused with the program itself.

---

33. Portability levels

Zamani should classify programs by portability.

Level 0 — Universal semantic program

No target-specific assumptions.

Level 1 — Capability-constrained

Requires abstract capabilities.

Level 2 — Resource-constrained

Requires resource properties.

Level 3 — Architecture-constrained

Requires architectural characteristics.

Level 4 — Device-constrained

Requires a specific device/class of device.

Level 5 — Physical implementation constrained

Requires concrete physical properties.

POCO-REAF is strongest at Levels 0–2.

Levels 3–5 remain valid Zamani programming, but they intentionally trade portability for control.

The language must not accidentally move a program from Level 0 to Level 5.

---

34. Explicit specialization

Target specialization MUST be explicit.

Valid conceptual structure:

universal computation
    |
    +-- generic realization
    |
    +-- cpu specialization
    |
    +-- gpu specialization
    |
    +-- quantum specialization
    |
    +-- fpga specialization

Specialization must preserve the universal program's semantic contract.

A specialization that changes semantics must be treated as a different program or explicitly defined semantic variant.

---

35. Conditional compilation boundary

Conditional compilation may depend on:

- declared capabilities;
- target properties;
- compiler features;
- language version;
- explicitly selected configuration.

It must not silently alter universal program semantics.

Conditional compilation must never become the only way to make a fundamentally portable program executable.

---

36. Compile-time versus runtime information

The language must distinguish:

compile-time known
runtime known
target-discovered
execution-observed

A value that is unknown at compile time must not require hard-coded source placeholders.

Examples:

available_memory
available_qubits
available_accelerators
available_nodes

are runtime/target information unless explicitly materialized into compile-time configuration.

---

37. Semantic preservation

Every compiler transformation must satisfy:

Meaning(source)
==
Meaning(transformed_program)

subject to explicitly declared:

- approximation;
- nondeterminism;
- numerical tolerance;
- probabilistic behavior;
- target-specific observable differences permitted by the language.

This requirement applies to:

- optimization;
- lowering;
- gate decomposition;
- scheduling;
- routing;
- hardware mapping;
- parallelization;
- vectorization;
- distributed partitioning;
- accelerator offloading.

---

38. Approximation boundary

Approximate computation must be explicit.

The compiler MUST NOT silently weaken:

- precision;
- fidelity;
- correctness;
- error tolerance;
- convergence requirements.

If the language supports approximate computation, the semantic contract must explicitly identify:

- permitted error;
- quality requirement;
- metric;
- validation rule.

---

39. Nondeterminism boundary

Nondeterminism must be explicit.

Examples:

- quantum measurement;
- concurrency races where intentionally exposed;
- randomized algorithms;
- distributed scheduling;
- stochastic algorithms.

The compiler may choose any valid implementation whose observable behavior remains within the specified semantic contract.

---

40. Determinism boundary

Parsing MUST be deterministic.

For identical:

source
language version
dialect configuration

the parser must produce the same structural result.

Semantic analysis should also be deterministic unless nondeterminism is explicitly part of the compiler architecture.

Compiler passes should have deterministic ordering where reproducibility is required.

---

41. Source locations

All semantic diagnostics originating from source constructs must retain source location information sufficient to identify:

- file;
- line;
- column;
- span;
- construct;
- relevant semantic context.

AST transformations must preserve provenance.

Lowering must not discard the source relationship required for diagnostics and verification.

---

42. Provenance boundary

Every semantic object that can produce a user-visible diagnostic or verification result should retain provenance where practical.

Provenance should permit:

runtime/backend result
        |
        v
compiled operation
        |
        v
IR operation
        |
        v
AST node
        |
        v
source span

Provenance is not a license to place source-language dependencies into backend internals.

---

43. Error boundary

Errors must remain at the appropriate layer.

Grammar errors

Examples:

- malformed token;
- malformed expression;
- missing delimiter;
- invalid syntactic production.

Semantic errors

Examples:

- type mismatch;
- invalid resource requirement;
- illegal quantum/classical interaction;
- invalid effect.

Compilation errors

Examples:

- no valid lowering;
- unsupported target capability;
- impossible constraint.

Runtime errors

Examples:

- resource unavailable;
- backend failure;
- execution failure.

Hardware errors

Examples:

- physical device fault;
- calibration failure;
- unavailable resource.

These categories must not be collapsed into generic parser errors.

---

44. Security boundary

Grammar parsing MUST be treated as untrusted input processing.

The grammar implementation must not:

- execute source code;
- access arbitrary files;
- access arbitrary network resources;
- execute shell commands;
- mutate hardware;
- perform backend discovery.

Parsing must remain a deterministic transformation from source input to syntax representation.

Compile-time execution, if supported by Zamani, must have an explicitly isolated security model.

---

45. Unsafe-code boundary

The Rust implementation must use:

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

where appropriate for grammar-related Rust modules.

No grammar implementation feature may require "unsafe".

The language's ability to express hardware-level operations does not justify unsafe Rust in the compiler implementation.

Hardware unsafety and compiler implementation unsafety are separate concepts.

---

46. Rust compatibility

Grammar tooling and semantic validation infrastructure must target:

- Rust 1.97;
- preferably Rust 1.97.1 where repository policy specifies the patch release;
- Rust 2021 edition where applicable.

The grammar design must not depend on newer Rust language features unavailable to the declared compiler baseline.

---

47. Genericity boundary

Generics must describe reusable semantics.

They must not be used merely as disguised hardware constants.

Valid:

fn transform<T>(value: T) -> T

Valid:

fn compute<const N>(...)

where "N" is a program-level mathematical or structural parameter.

Invalid architectural assumption:

const MAX_QUBITS: usize = 32;

when used as a universal language ceiling.

---

48. Compile-time numeric limits

Numeric literals may have implementation limits.

Such limits must be treated as lexer/parser implementation constraints and documented.

They must not impose arbitrary semantic limits on:

- tensor dimensions;
- quantum register sizes;
- data collection sizes;
- distributed worker counts;
- hardware resources.

Where arbitrary-precision or symbolic representation is required, the appropriate semantic layer should provide it.

---

49. Identifier boundary

Identifiers are semantic names.

An identifier such as:

gpu0
q7
device42
node3

does not automatically identify a physical resource.

Physical identity requires explicit semantic typing/context.

This prevents accidental coupling between naming conventions and hardware.

---

50. Namespace boundary

Namespaces must prevent collisions between:

- source symbols;
- capabilities;
- resource classes;
- hardware identities;
- dialect symbols;
- backend-specific names.

Vendor-specific names must not become global universal keywords without language-governance approval.

---

51. Dialect boundary

Dialects may extend syntax and semantics.

A dialect MUST declare:

- namespace;
- version;
- ownership;
- capabilities;
- semantic extensions;
- compatibility policy;
- lowering boundary;
- unsupported combinations.

A dialect must not silently redefine a core Zamani construct.

Dialect syntax must remain distinguishable from core language semantics.

---

52. Vendor extension boundary

Vendor extensions must be isolated.

For example:

vendor::<namespace>

or an equivalent explicit dialect mechanism may be used.

Vendor syntax must not become mandatory for portable programs.

A vendor backend may consume a portable Zamani program without requiring vendor syntax.

---

53. Interoperability boundary

Foreign formats are representations, not automatically Zamani semantics.

Examples:

- OpenQASM;
- Verilog;
- VHDL;
- C;
- C++;
- Python;
- LLVM IR;
- MLIR;
- backend-specific formats.

The interoperability layer owns:

foreign representation <-> Zamani representation

The core grammar must not become a foreign-language grammar simply because an importer/exporter exists.

---

54. OpenQASM boundary

OpenQASM input must pass through the frontend/interoperability layer.

Conceptually:

OpenQASM
   |
   v
OpenQASM frontend
   |
   v
Zamani semantic representation
   |
   v
quantum::ir

OpenQASM syntax must not become the canonical Zamani quantum grammar.

The same rule applies to HDL and other foreign representations.

---

55. Hardware address boundary

Physical addresses must never be implicit universal semantics.

Examples:

- memory addresses;
- MMIO addresses;
- device BARs;
- PCI identifiers;
- physical qubit indices;
- FPGA routing coordinates.

If such information is required, it belongs in an explicitly target-specific representation.

---

56. Scheduling and topology boundary

Topology is generally target information.

The program may express:

operation A requires communication with B

The routing system determines:

physical path A -> B

The scheduler determines:

when operations execute

The grammar must not encode a backend's topology as universal language semantics.

---

57. Capability negotiation

The compiler/runtime may perform:

program requirements
        |
        v
available capabilities
        |
        v
candidate realization
        |
        v
verification

If several targets satisfy the requirements, the program remains unchanged.

This is central to POCO-REAF.

---

58. Resource negotiation

Resource selection must be dynamic where practical.

For example:

required qubits = semantic analysis result
available qubits = target discovery

Then:

if available >= required:
    continue
else:
    reject or choose another valid realization

The grammar must not encode the available quantity.

---

59. No grammar-to-runtime cycle

Forbidden dependency:

grammar
  -> runtime
  -> grammar

Runtime may consume compiled semantic information derived from grammar.

Runtime must not redefine grammar syntax.

---

60. No grammar-to-IR cycle

Forbidden:

grammar -> quantum::ir -> grammar

Correct:

grammar -> AST -> semantic lowering -> quantum::ir

IR may retain source provenance, but provenance does not make IR dependent on parser implementation.

---

61. No grammar-to-hardware cycle

Forbidden:

grammar
  -> hardware discovery
  -> grammar

Correct:

grammar
  -> requirements
  -> compiler
  -> hardware capabilities
  -> realization

---

62. Semantic ownership matrix

Concern| Grammar| AST| Semantic Analysis| IR| Optimizer| Scheduler| Routing| Hardware| Runtime
Syntax| Owns| Receives| | | | | | | 
Names| Parses| Stores| Owns| Uses| | | | | 
Types| Parses| Represents| Owns| Represents| | | | | 
Effects| Parses| Represents| Owns| Represents| | | | | 
Quantum semantics| Parses| Represents| Validates| Canonical| Consumes| Consumes| Consumes| Consumes| Consumes
QEC algorithms| No| No| No| No| No| No| No| No| QEC
Noise model| No| No| No| Referenced| Consumes| Consumes| Consumes| Provides| Observes
Topology| Syntax only when explicit| Represents| Validates| Abstract| Consumes| Consumes| Owns realization| Provides| Observes
Timing| Syntax where semantic| Represents| Validates| Abstract| Consumes| Owns schedule| May constrain| Provides| Executes
Hardware discovery| No| No| No| No| No| No| No| Owns| Uses
Backend selection| No| No| No| No| No| No| No| Provides capabilities| Executes
Execution| No| No| No| No| No| No| No| No| Owns

---

63. Semantic categories

Every new grammar construct MUST be classified as one of:

1. Pure syntax
2. Value semantics
3. Type semantics
4. Control semantics
5. Effect semantics
6. Resource requirement
7. Constraint
8. Capability requirement
9. Preference
10. Hint
11. Target-specific realization
12. Foreign/interoperability construct
13. Metadata
14. Compile-time semantics
15. Runtime semantics
16. Hardware semantics

A construct MUST NOT remain semantically ambiguous.

---

64. New construct review

Before adding any grammar construct, answer:

What does it mean?

Which layer owns that meaning?

Can the meaning exist without a physical machine?

Is it portable?

Does it alter program semantics?

Does it represent a requirement or a realization?

Does another subsystem already own it?

Does it duplicate an existing IR concept?

Does it introduce a machine-size assumption?

Does it introduce a vendor dependency?

Can it be expressed using an existing abstraction?

If the answer reveals duplicate ownership, the construct must be redesigned before implementation.

---

65. Existing-feature migration

Existing grammar features MUST be classified before modification:

KEEP
REWRITE
SPLIT
MERGE
MOVE
DEPRECATE
REMOVE

No feature may be silently removed.

Migration must preserve semantics where compatibility requires it.

---

66. Hard-coding audit

Every grammar and semantic component must be checked for:

- "MAX_QUBITS";
- "MAX_CORES";
- "MAX_THREADS";
- fixed GPU counts;
- fixed FPGA counts;
- fixed node counts;
- fixed memory capacities;
- fixed topology;
- fixed device IDs;
- fixed physical addresses;
- fixed tensor dimensions;
- fixed accelerator widths;
- fixed timing grids;
- fixed backend assumptions.

Every occurrence must be classified as:

1. Semantic requirement
2. Resource policy
3. Target property
4. Implementation safety limit
5. Test fixture
6. Documentation example
7. Accidental hard-code

Only category 7 is automatically unacceptable.

Categories 2–5 must be kept outside universal language semantics.

---

67. Operational safety limits

Production implementations may require safety limits to prevent:

- memory exhaustion;
- parser denial-of-service;
- stack exhaustion;
- pathological recursion;
- excessive diagnostics;
- enormous token streams;
- excessive compile-time execution.

These are operational safeguards.

They MUST be:

- configurable where appropriate;
- explicit;
- documented;
- separate from language semantics;
- incapable of changing the meaning of valid source code.

A parser safety limit must never be described as:

«"Zamani supports only N constructs."»

It should be described as:

«"This implementation invocation permits at most N units of parser work."»

---

68. Diagnostics boundary

Diagnostics must distinguish:

syntax error
semantic error
capability error
resource error
target incompatibility
compiler limitation
runtime failure
hardware failure

A source program that is semantically valid but cannot execute on a particular target is not necessarily an invalid Zamani program.

---

69. Capability failure

If a program requires:

capability quantum

and the selected target has no quantum capability, the compiler/runtime should report target incompatibility.

It must not rewrite the program into classical computation unless an explicitly valid semantics-preserving fallback exists.

---

70. Resource failure

If a program requires more resources than are currently available:

required > available

the system may:

- select another target;
- choose another valid realization;
- defer execution;
- report resource exhaustion.

It must not alter the source program.

---

71. Semantic fallback

Fallback is legal only when semantic equivalence is established.

Examples:

GPU -> CPU
quantum simulator -> quantum hardware
accelerator -> scalar implementation
distributed -> single-node implementation

may be valid in some programs.

But:

quantum -> classical approximation

is not automatically semantics-preserving.

Fallback must therefore be capability-aware and semantics-aware.

---

72. POCO-REAF contract

A Zamani program intended for universal execution should satisfy:

Source semantics
       |
       | independent of temporary hardware
       v
Canonical semantic representation
       |
       | target-independent as far as possible
       v
Portable compiled artifact
       |
       | target realization
       v
Execution

The program may be compiled once into a sufficiently abstract artifact when the repository's compilation architecture supports that.

Target-specific lowering may happen later without changing the original program.

---

73. Compile-once caveat

"Compile once" must not be interpreted as requiring one immutable machine-code binary to execute natively on every future architecture.

Instead:

«Compile once means preserve the program's portable semantic/compiled representation so target realization can occur without rewriting the source semantics.»

This distinction is essential for future architectures.

A native x86 binary cannot magically become a native quantum circuit.

A universal semantic artifact can, however, be lowered into different valid realizations.

---

74. Forever compatibility

Future compatibility requires:

- versioned grammar;
- versioned semantic contracts;
- stable core constructs;
- reserved syntax space;
- dialect versioning;
- explicit deprecation;
- migration rules;
- semantic compatibility tests;
- canonical IR versioning;
- provenance.

The language must evolve without making old semantic programs meaningless merely because new hardware appeared.

---

75. Version boundary

Grammar version is not hardware version.

These must remain separate:

language_version
grammar_version
semantic_model_version
IR_version
backend_version
hardware_version

A backend upgrade must not automatically change language semantics.

---

76. Serialization boundary

Serialized AST/IR artifacts must identify their schema/version.

A serialized artifact must not depend on:

- memory addresses;
- process-local identifiers;
- pointer identity;
- temporary hardware IDs,

unless explicitly represented as target-bound metadata.

---

77. Checkpoint boundary

The grammar must not imply that arbitrary quantum state can always be serialized.

Quantum checkpoint semantics must distinguish:

- classical execution state;
- compiled program state;
- logical checkpoint;
- measurement boundary;
- QEC-supported state;
- reconstructible state;
- provider-supported state.

An arbitrary unknown quantum state cannot simply be assumed to be serializable.

---

78. Metadata boundary

Metadata may describe:

- documentation;
- provenance;
- optimization preferences;
- source annotations;
- debugging;
- compilation hints.

Metadata must not silently become executable semantics.

If metadata affects semantics, it must be promoted into an explicitly defined semantic construct.

---

79. Annotation boundary

Annotations must be classified.

Semantic annotation

Changes or constrains program meaning.

Compiler annotation

Guides compilation but does not change meaning.

Tooling annotation

Used by editors/documentation.

Diagnostic annotation

Affects diagnostics.

The implementation must not treat all annotations as equivalent.

---

80. Effect boundary

Effects describe observable interactions such as:

- IO;
- quantum execution;
- hardware interaction;
- networking;
- distributed communication;
- security operations;
- external state.

Effects belong to semantic analysis and IR.

They must not be confused with target implementation mechanisms.

---

81. Hardware effect

A source-level hardware effect may express:

interact with hardware

but must not automatically expose physical implementation.

For example:

hardware.read(sensor)

does not imply:

MMIO address 0x...

The latter belongs to the hardware binding layer.

---

82. Quantum/classical boundary

Quantum and classical domains must interoperate through explicit semantic constructs.

The compiler must preserve:

classical value
quantum value
measurement result
classical control
quantum operation

as distinct semantic categories.

A measurement result may enter classical computation.

A classical expression may control a quantum operation where the quantum execution model permits it.

The grammar must not collapse these into one untyped "value" abstraction.

---

83. Quantum/HDL boundary

Quantum hardware descriptions may interact with HDL semantics.

However:

quantum algorithm

must not automatically mean:

specific pulse generator implementation

Likewise:

hardware module

must not automatically become a quantum circuit.

The hybrid layer explicitly connects the two semantic domains.

---

84. Classical/accelerator boundary

Offloading is a realization decision unless explicitly semantic.

Source:

compute(data)

may execute on:

- CPU;
- GPU;
- NPU;
- FPGA;
- quantum-classical accelerator;

if semantics permit.

The compiler/runtime chooses the realization according to capabilities, requirements, constraints and preferences.

---

85. Parallelism boundary

A source-level parallel operation expresses parallel semantics.

The implementation decides:

- number of threads;
- number of workers;
- vector width;
- accelerator dimensions;
- node placement.

No fixed parallelism ceiling may be embedded in grammar semantics.

---

86. Data-size boundary

Data structures must be capable of representing arbitrary supported sizes subject to resource availability.

Do not encode:

array has at most N elements

unless N is an intrinsic semantic property of the type.

A fixed mathematical dimension is semantic.

A fixed machine capacity is not.

---

87. Type-level size boundary

Size parameters may be semantic:

Matrix<R, C>

where "R" and "C" describe the mathematical object.

They must not be interpreted as a machine capability.

The compiler determines whether a target can realize the object.

---

88. Compile-time evaluation boundary

Compile-time evaluation must not accidentally depend on:

- host machine CPU count;
- host memory size;
- filesystem state;
- network state;
- hardware availability;

unless those are explicitly declared inputs to compilation.

This preserves reproducibility.

---

89. Reproducibility

A reproducible build should record:

- source;
- language version;
- grammar version;
- dialect versions;
- compiler version;
- semantic configuration;
- relevant target profile;
- deterministic compilation settings.

Undeclared host properties must not silently change semantics.

---

90. Host independence

The machine compiling Zamani is not automatically the machine executing Zamani.

Therefore:

host
target
execution environment

must be separate concepts.

Cross-compilation must remain possible.

---

91. Compiler self-description boundary

If Zamani supports compiler/self-hosting declarations, these constructs describe compiler behavior.

They must not make the language grammar depend on the compiler's current host machine.

Self-hosting metadata must remain versioned and reproducible.

---

92. Runtime capability discovery

Runtime discovery may provide:

available resources
available capabilities
current health
current topology
current calibration
current queue state

These values must flow into runtime/compiler decision systems.

They must not mutate the source program's semantic meaning.

---

93. Dynamic adaptation

Dynamic adaptation is legal when the source program explicitly permits it or when the adaptation is semantics-preserving.

Examples:

choose available accelerator
retry execution
reroute operation
reschedule workload
switch equivalent backend

must be governed by the appropriate subsystem.

The grammar describes the policy/permission if that policy is source-visible.

It does not implement the adaptation algorithm.

---

94. Failure semantics

A target failure must not be confused with a source error.

For example:

valid program
+
backend unavailable
=
execution failure

not:

invalid program

This distinction is essential to portability.

---

95. Verification boundary

Verification may occur at multiple levels:

syntax verification
semantic verification
type verification
IR verification
optimization preservation
schedule validation
hardware constraint validation
runtime result validation

Each verifier owns its layer.

No single grammar validator should attempt to validate every downstream property.

---

96. Cross-domain semantic preservation

For:

classical + quantum

preserve both domains.

For:

quantum + HDL

preserve both semantic contracts.

For:

quantum + distributed + hardware

preserve:

- quantum semantics;
- communication semantics;
- hardware realization constraints.

No domain may silently reinterpret another domain.

---

97. Grammar modularity

The grammar directory may be physically divided into:

core/
types/
expressions/
statements/
quantum/
classical/
hdl/
hardware/
distributed/
...

but semantic ownership must remain coherent.

Splitting grammar files does not create independent language semantics.

All grammar fragments must ultimately participate in one authoritative Zamani grammar.

---

98. ANTLR boundary

ANTLR is a parsing technology.

It is not the semantic authority.

The grammar may use ANTLR rules to recognize syntax.

Semantic meaning belongs to the subsequent semantic layer.

Therefore:

ANTLR grammar

must not contain large amounts of semantic execution logic.

---

99. Parser boundary

The parser must:

- recognize valid syntax;
- reject malformed syntax;
- produce deterministic parse structures;
- retain source locations;
- produce useful syntax diagnostics.

The parser must not:

- discover hardware;
- select a backend;
- run quantum algorithms;
- schedule operations;
- optimize circuits;
- execute code.

---

100. Semantic analyzer boundary

Semantic analysis owns:

- name resolution;
- type validation;
- effect validation;
- capability validation;
- resource-expression validation;
- cross-domain legality;
- semantic invariants.

It does not own physical target scheduling.

---

101. IR lowering boundary

Lowering translates validated semantic constructs into canonical representations.

It must:

- preserve semantics;
- preserve provenance;
- reject unsupported semantics explicitly;
- avoid target-specific assumptions in target-independent IR.

---

102. Optimization boundary

Optimization consumes canonical representations.

It must never require the parser to know optimizer implementation details.

---

103. Scheduling boundary

Scheduling consumes operations plus target/resource information.

It must never require the grammar to encode the scheduler's internal representation.

---

104. Routing boundary

Routing consumes:

- logical operations;
- hardware connectivity;
- mapping constraints.

The grammar should not duplicate the router's topology representation.

---

105. Hardware abstraction boundary

Hardware abstraction exposes capabilities/resources to compiler and runtime layers.

It must not require source code to encode every physical property.

---

106. Runtime boundary

Runtime consumes compiled executable representations.

Runtime may:

- acquire resources;
- dispatch;
- observe failures;
- adapt execution;
- report telemetry.

Runtime must not redefine source semantics.

---

107. Integration contract for grammar files

Every grammar file MUST document:

Purpose
Owns
Does Not Own
Imports/Includes
Produces
AST nodes
Semantic categories
Downstream consumers
Cross-domain dependencies
Compatibility rules
Scalability rules
Hard-coding audit
Tests
Completion criteria

A grammar file is not complete merely because ANTLR accepts it.

---

108. Integration contract for domain grammar

Every domain grammar must answer:

What semantic domain does this represent?

What is the canonical AST representation?

What semantic analyzer validates it?

What canonical IR consumes it?

Which existing repository subsystem owns the resulting semantics?

Which concepts must NOT be duplicated?

Which resource properties are abstract?

Which properties are target-specific?

Which tests prove portability?

---

109. Completion criteria

A semantic grammar component is complete only when:

- syntax is defined;
- ambiguity is resolved;
- ownership is documented;
- AST mapping is defined;
- semantic meaning is defined;
- IR mapping is defined;
- downstream integration is defined;
- compatibility is defined;
- hard-coding audit passes;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- cross-domain tests exist where relevant;
- deterministic parsing is verified;
- diagnostics are verified;
- no circular dependency exists.

---

110. Required semantic tests

110.1 Tiny programs

Examples must include:

- zero/one classical value;
- smallest valid classical computation;
- smallest valid quantum computation;
- smallest valid HDL module;
- smallest valid resource declaration.

110.2 Large programs

Test generation must scale program size without requiring grammar changes.

110.3 Resource scaling

Generate equivalent semantic programs at increasing:

- qubit counts;
- data sizes;
- tensor dimensions;
- worker counts;
- operation counts;
- hardware resource counts.

The grammar must not impose arbitrary ceilings.

---

111. Quantum scalability tests

Tests must verify that syntax remains valid for generated quantum programs with resource sizes chosen by the test configuration.

The test harness may choose practical bounds.

Those test bounds are not language limits.

The test suite MUST NOT encode assertions such as:

Zamani supports at most 32 qubits.

unless testing a deliberately target-specific backend contract.

---

112. Classical scalability tests

Perform equivalent scaling tests for:

- arrays;
- matrices;
- tensors;
- nested structures;
- function call graphs;
- concurrent tasks;
- distributed workers.

---

113. HDL scalability tests

Generate hardware descriptions with configurable:

- port counts;
- signal counts;
- register counts;
- pipeline stages;
- state-machine states;
- module hierarchy.

The grammar must not encode fixed hardware sizes.

---

114. Cross-domain tests

Required combinations include:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

The purpose is to detect accidental semantic ownership collisions.

---

115. Negative semantic tests

Negative tests must verify rejection of:

- quantum operations on classical-only values;
- invalid resource expressions;
- invalid capability requirements;
- impossible semantic combinations;
- unsupported dynamic behavior;
- illegal hardware semantics;
- invalid cross-domain conversions;
- ambiguous dialect constructs;
- illegal target binding;
- malformed version constraints.

---

116. Target-specific negative tests

A semantically valid program should not be rejected merely because the current target cannot execute it.

Instead, target validation should report:

valid program
target incompatibility

where appropriate.

---

117. Round-trip requirement

Where a canonical printer exists:

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
printer
  |
  v
parser

must preserve semantic meaning.

Formatting differences are acceptable.

Semantic changes are not.

---

118. Semantic equivalence

Two source programs may have different syntax while representing the same semantics.

Therefore semantic tests must not rely only on textual equality.

The preferred validation is:

source A
    |
    v
semantic representation A

source B
    |
    v
semantic representation B

compare semantic meaning

---

119. Future computing boundary

The core language must be extensible to future computational paradigms without requiring core semantic redesign.

Potential future domains include:

- neuromorphic computing;
- photonic computing;
- reversible computing;
- analog computing;
- molecular computing;
- optical computing;
- biological computing;
- quantum networks;
- future accelerators;
- architectures not yet invented.

The extension mechanism should add capabilities and dialects rather than hard-code every future technology into core semantics.

---

120. "More computing" principle

Zamani should not attempt to predict every future machine.

Instead it should define stable abstractions:

value
type
computation
resource
capability
constraint
effect
communication
state
time
execution
target
provenance
verification

New technologies should map onto these abstractions wherever possible.

---

121. Semantic stability rule

A new backend must not require changing the meaning of an existing core construct merely because the backend has different hardware.

If a new machine cannot implement a construct, the backend must either:

1. provide a valid lowering;
2. provide a valid emulation;
3. provide a valid fallback;
4. report unsupported capability.

It must not silently redefine the language.

---

122. Grammar stability rule

Adding a target must not require adding target-specific syntax to ordinary portable programs.

Bad:

quantum_for_vendor_x ...

Preferred:

quantum computation

with target-specific realization handled later.

---

123. Vendor-neutral core

Core Zamani must remain vendor-neutral.

Vendor-specific functionality belongs to:

dialects/
interoperability/
hardware/

or another explicitly governed extension boundary.

---

124. Semantic boundary checklist

For every construct, verify:

- [ ] Is its meaning explicitly defined?
- [ ] Is its owner identified?
- [ ] Is its non-owner identified?
- [ ] Is its AST representation defined?
- [ ] Is its semantic representation defined?
- [ ] Is its canonical IR mapping defined?
- [ ] Is its target realization separated?
- [ ] Is hardware dependence explicit?
- [ ] Is resource dependence explicit?
- [ ] Is capability dependence explicit?
- [ ] Is provenance preserved?
- [ ] Are errors classified correctly?
- [ ] Is compatibility defined?
- [ ] Is scalability preserved?
- [ ] Has hard-coding been audited?
- [ ] Are positive tests present?
- [ ] Are negative tests present?
- [ ] Are boundary tests present?
- [ ] Are cross-domain tests present where applicable?
- [ ] Is deterministic parsing demonstrated?
- [ ] Is circular dependency absent?

---

125. Repository integration requirements

The semantic-boundary specification must integrate with the following repository areas.

Grammar

grammar/Zamani.g4
grammar/specification/*
grammar/core/*
grammar/types/*
grammar/expressions/*
grammar/statements/*
grammar/quantum/*
grammar/classical/*
grammar/hdl/*
grammar/hardware/*
grammar/resources/*
grammar/compile/*
grammar/execution/*
grammar/interoperability/*
grammar/dialects/*

The grammar defines syntax only within its ownership boundary.

Quantum

The grammar lowers through semantic analysis into:

quantum::ir

rather than creating another quantum operation representation.

QEC

Grammar-level QEC declarations become semantic requirements/configuration.

QEC algorithms remain in QEC.

Existing QEC components demonstrate the intended pattern of explicit ownership and cross-module integration rather than duplicated abstractions.

ZQN

Noise/fault semantics remain in ZQN.

The grammar may express requirements that ZQN later consumes.

Scheduling

Timing/order requirements flow to scheduling.

The grammar does not implement scheduling.

Optimization

Optimization consumes canonical semantic/IR representations.

Hardware

Hardware capabilities and target properties are supplied by hardware abstraction.

Runtime

Runtime performs execution and resource acquisition.

---

126. Existing repository implementation policy

Repository implementations that already enforce:

#![deny(unsafe_code)]

or equivalent safety policy must retain that policy.

Existing QEC components explicitly target Rust 1.97.1 and deny unsafe code.

Grammar-related Rust implementation must follow the same safety posture.

---

127. Separation from implementation limits

An implementation may contain constants such as:

maximum diagnostic length
maximum parser recursion
maximum allocation
maximum input bytes

when required for safety.

Such constants must never be interpreted as:

maximum Zamani program size
maximum number of qubits
maximum number of devices
maximum machine size

unless they are explicitly part of a target-specific or operational contract.

---

128. Anti-patterns

The following are prohibited.

Fixed quantum capacity

MAX_QUBITS = 32

Fixed machine topology

q[0] connects q[1]

as universal semantics.

Automatic measurement

Adding measurements that the source did not request.

Backend leakage

Putting vendor gate names into core semantic types.

Scheduler leakage

Encoding scheduler internals in grammar.

Optimizer leakage

Encoding optimizer passes as core semantics.

Hardware discovery in parser

Querying hardware during parsing.

Runtime semantics in grammar

Making parser behavior depend on current machine state.

Duplicate IR

Creating a second quantum gate/qubit model unrelated to "quantum::ir".

Silent fallback

Changing quantum computation into classical approximation without an explicit valid semantic contract.

Hidden limits

Using parser constants as language maximums.

---

129. Correct architecture examples

Portable quantum program

program
    |
    v
quantum intent
    |
    v
quantum::ir
    |
    +--> simulator
    +--> QPU A
    +--> QPU B
    +--> future QPU

The source remains unchanged.

Portable accelerator program

computation
    |
    v
resource requirement
    |
    +--> CPU
    +--> GPU
    +--> FPGA
    +--> NPU
    +--> future accelerator

Portable distributed program

logical workers
    |
    v
distributed semantics
    |
    +--> local execution
    +--> cluster
    +--> cloud
    +--> future distributed substrate

---

130. Semantic boundary decision procedure

When a new feature is proposed, apply this sequence:

1. Identify the source meaning.
2. Determine whether the meaning is machine-independent.
3. Identify the semantic owner.
4. Check whether an existing repository subsystem already owns it.
5. Define AST representation.
6. Define semantic representation.
7. Define canonical IR mapping.
8. Define target realization.
9. Define capability/resource requirements.
10. Define diagnostics.
11. Define compatibility.
12. Perform hard-coding audit.
13. Define positive/negative/boundary tests.
14. Verify no dependency cycle.
15. Approve grammar syntax only after the semantic contract is complete.

---

131. File-completion contract

This document itself is complete only when every normative boundary described here is reflected consistently in:

- grammar;
- AST;
- semantic analysis;
- canonical IR;
- compiler;
- hardware abstraction;
- runtime;
- tests;
- documentation.

If implementation discovers a contradiction, implementation must stop and the semantic contract must be resolved first.

The implementation must not silently invent semantics.

---

132. Required companion files

This specification integrates directly with:

grammar/validation/grammar-validation.md
grammar/validation/ambiguity-rules.md
grammar/validation/scalability-rules.md
grammar/validation/compatibility-rules.md
grammar/validation/naming-rules.md
grammar/validation/hardcoding-audit.md

Recommended dependency order:

semantic-boundaries.md
        |
        +--> grammar-validation.md
        |
        +--> ambiguity-rules.md
        |
        +--> scalability-rules.md
        |
        +--> compatibility-rules.md
        |
        +--> naming-rules.md
        |
        +--> hardcoding-audit.md

"semantic-boundaries.md" therefore establishes the semantic ownership rules consumed by the other validation documents.

---

133. Required companion grammar integration

The authoritative grammar must implement syntax consistent with these boundaries.

In particular:

grammar
    !=
semantic analyzer
    !=
IR
    !=
optimizer
    !=
scheduler
    !=
routing
    !=
hardware
    !=
runtime

Each layer consumes the contract produced by the preceding layer.

---

134. Required companion IR integration

The semantic layer must lower quantum constructs into the repository's canonical:

quantum::ir

and must not create:

GrammarQuantumGate
GrammarQubit
GrammarCircuit

as a competing canonical representation.

Temporary AST structures are permitted.

Competing semantic IRs are not.

---

135. Required companion resource integration

Resource requirements must eventually be consumable by the repository's resource-management and hardware systems.

The flow is:

source requirement
      |
      v
semantic requirement
      |
      v
resource model
      |
      v
available resource snapshot
      |
      v
admission / planning

The source program does not own the resource snapshot.

---

136. Required companion scheduling integration

Semantic ordering constraints flow into scheduling:

semantic dependency
      |
      v
IR dependency
      |
      v
scheduler
      |
      v
target schedule

The grammar does not generate final timestamps.

---

137. Required companion QEC integration

The grammar may state:

fault_tolerant;
logical_error_rate < requirement;

but the QEC subsystem decides:

- code;
- decoder;
- syndrome method;
- correction;
- resource allocation.

This maintains the repository's existing separation of QEC algorithms and orchestration.

---

138. Required companion ZQN integration

The grammar may express:

noise_aware;
fault_tolerance required;
error_bound expression;

but ZQN owns fault/noise modeling.

The grammar must not duplicate noise channels or fault objects.

---

139. Required companion hardware integration

The hardware layer supplies:

capabilities
resources
topology
timing
calibration
availability

The grammar supplies:

requirements
constraints
preferences
semantic intent

The compiler combines them.

---

140. Production-readiness gate

"semantic-boundaries.md" is considered production-ready only when all of the following hold:

Architecture

- [ ] Semantic ownership is explicit.
- [ ] No circular dependency exists.
- [ ] Canonical IR boundaries are defined.
- [ ] Quantum semantics terminate at "quantum::ir".
- [ ] Hardware realization remains downstream.

Scalability

- [ ] No fixed machine-size semantics exist.
- [ ] No fixed qubit ceiling exists.
- [ ] No fixed CPU/GPU/FPGA/node ceiling exists.
- [ ] Resource availability remains external to language meaning.
- [ ] Large programs can be represented subject only to implementation/resource limits.

Portability

- [ ] Portable programs do not require vendor syntax.
- [ ] Target bindings are explicit.
- [ ] Capability requirements are distinct from target identity.
- [ ] Preferences are distinct from requirements.

Quantum

- [ ] Logical and physical resources are separated.
- [ ] Measurement is explicit.
- [ ] Dynamic circuits are semantically defined.
- [ ] QEC is not duplicated.
- [ ] ZQN is not duplicated.
- [ ] Routing is not duplicated.
- [ ] Scheduling is not duplicated.

Classical

- [ ] Classical computation has an independent semantic model.
- [ ] Accelerator selection remains downstream.
- [ ] Parallelism is not hard-coded.

HDL

- [ ] Hardware behavior is separated from physical implementation.
- [ ] Synthesis details remain downstream.
- [ ] Hardware parameters remain scalable.

Safety

- [ ] Rust implementation uses no "unsafe".
- [ ] Rust 1.97/1.97.1 compatibility is maintained.
- [ ] Parser does not execute arbitrary source.
- [ ] Compile-time execution is separately controlled.

Compatibility

- [ ] Language versions are distinct from target versions.
- [ ] Dialects are versioned.
- [ ] Deprecation is explicit.
- [ ] Semantic compatibility is tested.

Testing

- [ ] Positive tests exist.
- [ ] Negative tests exist.
- [ ] Boundary tests exist.
- [ ] Cross-domain tests exist.
- [ ] Scalability tests exist.
- [ ] Determinism tests exist.
- [ ] Round-trip tests exist where supported.
- [ ] Hard-coding audits exist.

---

141. Final invariant

The following invariant is absolute:

«A physical limitation must never become a language limitation merely because a current implementation has that physical limitation.»

Likewise:

«A backend implementation detail must never become universal source semantics merely because one backend currently requires it.»

And:

«A new hardware architecture must not require rewriting a portable Zamani program merely because its physical realization differs.»

---

142. Final POCO-REAF invariant

For a portable Zamani program:

PROGRAM ONCE
     |
     v
ONE SEMANTIC MEANING
     |
     v
ONE PORTABLE PROGRAM REPRESENTATION
     |
     +----------------+----------------+----------------+
     |                |                |                |
     v                v                v                v
   CPU              GPU             FPGA            Quantum
     |                |                |                |
     +----------------+----------------+----------------+
                              |
                              v
                       DISTRIBUTED / CLOUD /
                       EMBEDDED / FUTURE

The source semantics remain stable.

The realization changes.

---

143. Ultimate Zamani principle

Zamani is not a language for one machine.

It is a language for expressing computation independently of temporary machine boundaries.

Therefore:

source
  !=
machine

program
  !=
target

semantic requirement
  !=
physical resource

logical qubit
  !=
physical qubit

capability
  !=
device identity

optimization
  !=
program meaning

schedule
  !=
computation

routing
  !=
quantum algorithm

noise model
  !=
quantum semantics

QEC algorithm
  !=
quantum language

runtime resource availability
  !=
language capability

These separations are the foundation of:

«Zamani — From Atom to Everywhere»

and:

«Program Once → Compile Once → Run Everywhere → Anywhere → Forever.»

---

144. Acceptance statement

This document establishes the semantic-boundary contract that all subsequent Zamani grammar, AST, semantic-analysis, IR, compiler, scheduling, routing, hardware and runtime work must satisfy.

No downstream implementation may redefine these boundaries implicitly.

Any intentional boundary change requires:

1. semantic specification change;
2. compatibility analysis;
3. affected-file identification;
4. migration plan;
5. tests;
6. documentation update;
7. explicit review.

Until such a change is approved, these boundaries are normative.

End of "grammar/validation/semantic-boundaries.md".