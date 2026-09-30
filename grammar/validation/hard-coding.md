Zamani Grammar Hard-Coding Policy

Path: "grammar/validation/hard-coding.md"
Status: Normative production specification
Language: Zamani
Grammar technology: ANTLR4-compatible grammar architecture
Implementation baseline: Rust 1.97 / Rust 1.97.1
Safety requirement: Rust "unsafe" is prohibited
Scope: Repository-wide
Primary objective: Preserve target-independent semantics and POCO-REAF scalability from the smallest supported computation to arbitrarily larger computations subject only to actual resources, capabilities, constraints, and implementation availability.

---

1. Purpose

This document defines the normative policy for detecting, classifying, rejecting, permitting, and documenting hard-coded values throughout the Zamani grammar architecture and its directly integrated validation/tooling layers.

It exists to prevent an implementation detail, current hardware characteristic, compiler limitation, test fixture, backend assumption, or historical design decision from accidentally becoming a Zamani language limitation.

The central invariant is:

«A Zamani program describes computation, semantics, requirements, capabilities, constraints, correctness, and intent. It does not silently encode the finite characteristics of today's implementation targets.»

The policy applies across:

- lexical specifications;
- ANTLR grammar files;
- grammar documentation;
- language specifications;
- AST contracts;
- semantic contracts;
- resource contracts;
- capability contracts;
- quantum contracts;
- classical contracts;
- HDL contracts;
- hardware contracts;
- compilation contracts;
- execution contracts;
- interoperability contracts;
- validation tooling;
- generated artifacts;
- conformance tests;
- compatibility material;
- repository-level validation.

The policy is specifically designed to preserve:

"Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever"

abbreviated as:

"POCO-REAF".

---

2. Authority and integration model

This file is the normative policy for determining what constitutes hard-coding and whether a detected fixed value is semantically acceptable.

It must not become a competing grammar authority.

The authority chain is:

grammar/DESIGN.md
        │
        ▼
language/specification contracts
        │
        ▼
grammar/Zamani.g4
        │
        ▼
lexer / parser
        │
        ▼
frontend AST
        │
        ▼
semantic analysis
        │
        ▼
canonical semantic IR
        │
        ├───────────────┐
        ▼               ▼
 classical IR       quantum::ir
        │               │
        └───────┬───────┘
                ▼
        optimization / lowering
                │
        routing / scheduling
                │
        resilience / QEC / ZQN
                │
                ▼
               HAL
                │
                ▼
          target realization

Hard-coding validation must enforce the boundaries in that architecture.

It must not move target-specific knowledge upward into grammar or AST syntax merely because such knowledge exists downstream.

---

3. Relationship to existing validation files

The repository already contains related validation specifications.

This file does not replace them.

The responsibilities are:

File| Responsibility
"grammar/validation/hard-coding.md"| Normative classification and acceptance policy for hard-coded values
"grammar/validation/hardcoding-audit.md"| Repository hard-coding audit procedure, evidence, and audit execution
"grammar/validation/scalability-rules.md"| Determines whether an otherwise detected fixed value violates scalability requirements
"grammar/validation/grammar-validator.md"| General grammar validation and validator integration
"grammar/validation/semantic-boundaries.md"| Prevents semantic/runtime/backend boundary violations
"grammar/validation/duplicate-tokens.md"| Prevents duplicate lexical authority
"grammar/validation/keyword-collisions.md"| Prevents keyword/token namespace collisions
"grammar/validation/source-spans.md"| Source-location correctness and scalable diagnostic metadata
"grammar/validation/semantic-coverage.md"| Grammar → AST → semantics → IR coverage
"grammar/specification/portability.md"| Normative portability/POCO-REAF requirements
"grammar/spec/quantum.md"| Quantum-specific language contract
"grammar/spec/hdl.md"| HDL-specific language contract
"grammar/resources/requirements.g4"| Resource requirement syntax
"grammar/hardware/capabilities.g4"| Hardware capability syntax
"grammar/Zamani.g4"| Canonical ANTLR composition root

If wording in an older validation document conflicts with this policy, the conflict must be resolved by making the older document defer to this policy rather than creating two independent hard-coding definitions.

Existing filenames MUST NOT be renamed merely to implement this policy.

---

4. Definition of hard-coding

For Zamani, hard-coding means:

«Encoding a value, finite set, range, topology, implementation characteristic, machine identity, resource capacity, or backend assumption into a layer where that information is not semantically owned.»

Hard-coding is therefore contextual.

The presence of a literal does not automatically constitute a violation.

For example:

let iterations = 1000;

is ordinary program data.

By contrast:

grammar only accepts <= 1000 iterations

is a language hard limit.

Likewise:

requires qubits >= n

is portable semantic intent.

But:

compiler supports at most 32 qubits

is an implementation limitation and MUST NOT be represented as a Zamani language limit.

---

5. Fundamental distinction

Every detected fixed value MUST be classified into exactly one primary category before it is accepted.

The categories are:

1. Semantic constant
2. Program data
3. Resource requirement
4. Resource constraint
5. Capability requirement
6. Preference
7. Implementation limitation
8. Target-specific realization
9. Safety limit
10. Test-only fixture
11. Documentation-only example
12. Generated artifact
13. Accidental hard-coding

Only the first six are normally allowed to become portable language semantics.

Categories 7–12 may exist in appropriate layers, but MUST NOT silently become language semantics.

Category 13 is a production validation failure.

---

6. Absolute prohibition

The Zamani grammar architecture MUST NOT impose universal hardware or scalability limits through hard-coded constants, enumerations, ranges, parser alternatives, semantic validation rules, or implicit assumptions.

In particular, the language MUST NOT introduce universal limits such as:

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

The prohibition also applies to semantically equivalent spellings.

Examples include:

MAX_QUBIT_COUNT
MAX_CPU_COUNT
MAX_GPU_COUNT
MAX_NODE_COUNT
MAX_MEMORY_BYTES
MAX_REGISTER_BITS
MAX_TENSOR_DIMENSIONS
MAX_DEVICES
MAX_THREADS_SUPPORTED
SUPPORTED_QUBITS = 32
SUPPORTED_QUBITS = 64
MAX_QUBITS = 128

Changing the identifier does not evade the policy.

---

7. Equivalent hard-coded limits are also forbidden

The validator MUST detect semantic equivalents even when they do not use obvious "MAX_*" names.

Forbidden examples include:

qubits <= 32
qubits < 33

cpu_count <= 8

gpu_count == 1

nodes in 0..16

register_width <= 32

tensor_rank <= 8

device_count <= 4

network_size <= 1024

when those expressions are used to define a universal language or compiler capability.

The validator MUST reason about the role of the value rather than merely matching identifier names.

---

8. Explicit forbidden hardware limits

The following classes are universally prohibited as language limits.

8.1 Quantum

The grammar MUST NOT hard-code:

- maximum qubits;
- maximum logical qubits;
- maximum physical qubits;
- maximum registers;
- maximum register width;
- maximum circuit depth;
- maximum gate count;
- maximum measurement count;
- maximum control count;
- maximum ancilla count;
- maximum QEC distance;
- maximum syndrome rounds;
- maximum shots;
- maximum quantum states;
- maximum tensor dimensions;
- fixed physical qubit identifiers as universal semantics.

Examples of prohibited constructs:

MAX_QUBITS
MAX_PHYSICAL_QUBITS
MAX_LOGICAL_QUBITS
QUBITS_0_TO_31
QUBIT0
QUBIT1
QUBIT2
...

A concrete qubit identifier may exist in a target-specific physical mapping artifact, but it MUST NOT become a universal source-language restriction.

---

9. Classical computing

The grammar MUST NOT impose:

- maximum CPUs;
- maximum cores;
- maximum threads;
- maximum vector lanes;
- maximum SIMD width;
- maximum register width;
- maximum process count;
- maximum task count;
- maximum memory;
- maximum stack size;
- maximum heap size;
- maximum address-space size;
- maximum array size;
- maximum matrix dimensions;
- maximum tensor dimensions.

The source language may express a specific algorithmic dimension.

For example:

Tensor<f64, [1024, 1024]>

is program semantics.

The grammar MUST NOT state that Zamani tensors can never exceed a fixed dimension.

---

10. GPU and accelerator computing

The grammar MUST NOT hard-code:

- maximum GPUs;
- maximum accelerator count;
- maximum blocks;
- maximum grids;
- maximum workgroups;
- maximum lanes;
- maximum shared memory;
- maximum local memory;
- maximum VRAM;
- maximum device queues;
- fixed vendor device IDs.

Examples of prohibited universal assumptions:

GPU_COUNT = 1
MAX_GPUS = 8
MAX_WORKGROUP_SIZE = 1024
VRAM = 24GB
DEVICE_0
DEVICE_1

A backend may discover that a particular target has a certain capability.

That fact belongs to target discovery/capability negotiation, not universal grammar semantics.

---

11. FPGA and ASIC

The grammar MUST NOT hard-code:

- LUT count;
- DSP count;
- BRAM count;
- URAM count;
- logic-cell count;
- routing-resource count;
- clock count;
- I/O count;
- pin count;
- physical region count;
- fixed synthesis dimensions;
- fixed ASIC process characteristics.

HDL may describe:

- parameterized widths;
- modules;
- interfaces;
- timing intent;
- state machines;
- pipelines;
- memories;
- physical intent;
- synthesis intent.

It MUST NOT silently convert one particular FPGA or ASIC into the definition of Zamani HDL.

---

12. Distributed systems

The grammar MUST NOT impose:

- maximum node count;
- maximum service count;
- maximum process count;
- maximum replica count;
- maximum partition count;
- maximum cluster size;
- maximum message size;
- maximum topology size;
- maximum network links;
- maximum peers.

For example:

MAX_NODES = 1024

is prohibited as a universal language constant.

A program may legitimately state:

requires nodes >= required_nodes

when that is semantic resource intent.

---

13. Networking

The grammar MUST NOT impose:

- maximum endpoints;
- maximum connections;
- maximum channels;
- maximum packets;
- maximum route entries;
- maximum peers;
- maximum topology size;
- fixed IP addresses as universal semantics;
- fixed port numbers unless the port itself is part of the program's protocol semantics.

A protocol may legitimately specify a protocol-defined field width.

That does not authorize Zamani to impose the same width on unrelated networking constructs.

---

14. Memory and storage

The grammar MUST NOT encode a particular machine's memory configuration.

Forbidden examples include:

64GB RAM
24GB VRAM
32-bit registers
128KB cache
1TB storage

as universal compiler or language limits.

A source program may require:

requires memory >= required_memory

or describe an algorithmically meaningful object size.

The distinction is:

program requirement

versus:

compiler capacity

Only the first is portable language semantics.

---

15. Tensor and mathematical scalability

Mathematical types MUST NOT acquire artificial compiler limits.

Forbidden:

MAX_TENSOR_RANK = 8
MAX_MATRIX_DIMENSION = 4096
MAX_VECTOR_WIDTH = 1024
MAX_SYMBOLS = ...

unless the value is explicitly part of an externally defined mathematical format or protocol.

For example, a protocol may define a fixed field width.

That fixed protocol definition is semantic.

It MUST NOT be generalized into a Zamani implementation maximum.

---

16. Timelines, histories, and temporal computation

Zamani's temporal and Sankofa-related features MUST NOT hard-code:

- maximum timeline count;
- maximum history depth;
- maximum branches;
- maximum events;
- maximum temporal precision;
- maximum rewind depth;
- maximum retained states.

The number of timelines or historical records is a runtime/resource property unless explicitly constrained by a semantic model.

---

17. Nano-scale computing

Nano or atom-scale computation MUST NOT be limited by fixed grammar enumerations.

The language may express:

- atoms;
- molecules;
- materials;
- interactions;
- physical properties;
- capabilities;
- constraints;
- domains.

It MUST NOT encode the currently known set of physical devices as the complete language universe.

---

18. Quantum operation enumeration

A particularly important prohibition applies to quantum operations.

The grammar MUST NOT establish a closed universal enumeration such as:

quantumGate
    : H
    | X
    | Y
    | Z
    | CNOT
    | ...

as the complete quantum operation language.

The preferred semantic shape is data-driven:

quantumOperation
    : operationSpecifier quantumTargetList
    ;

The operation specification may identify:

H
X
Y
Z
CNOT
custom_gate
vendor.operation
operation(parameter)
future.operation

without requiring a grammar edit for every future operation.

This is essential for POCO-REAF.

---

19. Physical quantum mapping

Physical mapping is downstream.

The source program may express:

logical qubit

and:

requires capability("quantum.measurement")

A later target-specific stage may establish:

logical q0 -> physical resource R

The mapping MUST NOT become the universal definition of the source language.

The intended flow remains:

Zamani source
    ↓
frontend AST
    ↓
semantic quantum operation
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
physical realization

---

20. Capability versus hard-coding

The following is allowed:

requires capability("gpu.compute")

requires capability("quantum.measurement")

requires capability("tensor.compute")

The following is not equivalent:

use GPU 0

when "GPU 0" is being treated as a universal target.

Capability describes what is required.

Hard-coding describes which current implementation is assumed.

The former is portable.

The latter belongs downstream unless explicitly target-specific.

---

21. Resource requirements

Resource requirements are allowed when they are semantic requirements.

Examples:

requires qubits >= n

requires memory >= required_memory

requires nodes >= required_nodes

requires bandwidth >= required_bandwidth

These expressions do not establish a compiler maximum.

They establish program requirements.

The compiler/runtime must determine whether an available target satisfies them.

---

22. Requirements, constraints, preferences, and hints

The grammar MUST preserve the semantic distinction between:

requirement
constraint
capability
preference
hint
implementation decision

For example:

Requirement

requires capability("quantum.measurement")

Constraint

requires topology(...)

Preference

prefer accelerator("quantum")

Hint

hint locality(...)

Implementation decision

map logical resource -> physical resource

The first four can remain portable semantic intent.

The last belongs to target realization unless the source explicitly declares itself target-specific.

---

23. Fixed values that are legitimate

The prohibition does not mean Zamani cannot contain numbers.

The following are legitimate:

let n = 1024;
let iterations = 1000;
let threshold = 0.5;
let matrix = Matrix<1024, 1024>;

provided these are program semantics and not secretly interpreted as implementation limits.

A literal becomes suspicious when its meaning is:

"the compiler only supports up to this value"

rather than:

"the program requests this value"

---

24. Protocol-defined constants

Fixed constants may be required by:

- external protocols;
- file formats;
- ABI definitions;
- interoperability specifications;
- cryptographic formats;
- hardware standards;
- mathematical definitions;
- architectural standards.

These are allowed only when the value is genuinely owned by the external contract.

The implementation MUST document:

owner
source specification
semantic meaning
scope
whether the value is normative
whether it is a compatibility requirement

The validator MUST distinguish such constants from implementation capacity limits.

---

25. ABI and interoperability constants

Interoperability may legitimately require fixed values.

Examples include:

- externally defined calling conventions;
- protocol field widths;
- binary format magic values;
- standardized encodings;
- foreign ABI identifiers.

These values MUST remain within the interoperability boundary.

They MUST NOT be propagated into the general Zamani type system or grammar as universal machine limits.

---

26. Parser and validator implementation limits

Validation tooling itself may require bounded implementation resources.

For example, a validator implementation may use:

Vec<T>
HashMap<K, V>
BTreeMap<K, V>

or implementation-specific memory-management strategies.

These implementation details do not become Zamani semantics.

A validator may also have an operational safety policy concerning:

- recursion;
- memory consumption;
- diagnostic count;
- wall-clock execution;
- generated output size.

Such limits MUST be classified as tooling operational limits, not language limits.

They MUST NOT be presented as:

«Zamani programs cannot contain more than N constructs.»

Instead they represent:

«This validator invocation may terminate, reject, or report resource exhaustion under a configured operational policy.»

---

27. Safety limits

Safety limits require special treatment.

A safety limit may exist to protect:

- the compiler;
- the validator;
- a development machine;
- CI;
- a parser process;
- a runtime process.

A safety limit is acceptable only when all of the following are true:

1. It is not presented as language semantics.
2. It is not encoded into the grammar.
3. It is not encoded into the AST's semantic meaning.
4. It does not alter the definition of a valid Zamani program.
5. It is distinguishable from semantic diagnostics.
6. It can be configured or negotiated where appropriate.
7. It produces a tooling/resource diagnostic rather than a false syntax error.
8. It does not silently reduce the language accepted by production tooling.

---

28. No disguised parser limits

The validator MUST detect attempts to introduce scalability limits through grammar shape.

Examples:

register
    : qubit qubit qubit qubit
    ;

dimensions
    : dimension
    | dimension ',' dimension
    | dimension ',' dimension ',' dimension
    ;

when the intention is to restrict the language to a fixed number of elements.

Likewise:

qubitList
    : qubit
    | qubit ',' qubit
    | qubit ',' qubit ',' qubit
    ;

must not be used to establish a universal maximum.

Use repetition or generalized list structures where the semantics permit:

qubitList
    : qubit (',' qubit)*
    ;

The semantic layer may later determine resource feasibility.

---

29. No disguised enumerated universe

A finite grammar alternative can itself be hard-coding.

Suspicious examples:

device
    : cpu
    | gpu
    | fpga
    | qpu
    ;

This may be acceptable only when "cpu", "gpu", "fpga", and "qpu" are semantic categories rather than a claim that these are the only possible future computational substrates.

The validator MUST flag finite enumerations that claim exhaustiveness over extensible domains.

Prefer extensible identifiers/capabilities where the domain is inherently open.

---

30. Hardware vendor names

Vendor-specific names MUST NOT become universal hardware semantics.

Examples include:

specific GPU model
specific QPU model
specific FPGA family
specific CPU model
specific accelerator SKU

Vendor interoperability may explicitly reference such identifiers.

That reference must remain target/interoperability metadata.

It must not become the universal grammar's definition of the corresponding domain.

---

31. Fixed topology

The grammar MUST NOT assume:

linear topology
2D grid
fixed lattice size
fixed ring size
fixed mesh dimensions
fixed QPU coupling map
fixed cluster topology
fixed network topology

unless the topology itself is explicitly part of a source program's semantic requirement.

For example:

requires topology(...)

is valid.

A compiler saying:

all Zamani QPUs have topology X

is not valid language semantics.

---

32. Fixed identifiers

The following are suspicious:

CPU0
GPU0
QPU0
QUBIT0
NODE0
DEVICE0
MEMORY0
CORE0

They are not universally prohibited in all contexts.

They are prohibited when used to define target-independent semantics.

They may be valid in:

- target manifests;
- backend mappings;
- debugging metadata;
- explicit hardware-description source;
- external device configuration;
- physical mapping artifacts.

The ownership boundary MUST be explicit.

---

33. Fixed width

A width is not automatically hard-coding.

For example:

u32
i64

may be valid language types with explicitly defined semantics.

A fixed width becomes a hard-coding violation when the implementation assumes:

«all Zamani registers, vectors, addresses, or hardware resources have this width.»

Likewise:

wire [31:0]

is valid only when the 32-bit width is intentionally part of that particular hardware design.

It MUST NOT be used as the universal HDL model.

---

34. Fixed address spaces

Memory addresses and physical addresses MUST NOT be hard-coded into portable language semantics.

Target-specific address mappings belong to:

- hardware realization;
- linker configuration;
- deployment metadata;
- backend-specific lowering;
- target-specific HDL;
- device manifests.

Portable source should use semantic references.

---

35. Fixed scheduling assumptions

The grammar MUST NOT assume:

- a fixed number of execution slots;
- fixed core count;
- fixed quantum cycle duration;
- fixed hardware clock;
- fixed network latency;
- fixed accelerator latency;
- fixed number of scheduler queues.

Scheduling intent may be expressed.

Actual scheduling is downstream.

---

36. Fixed calibration assumptions

Quantum and hardware grammar MUST NOT embed:

- fixed calibration values;
- fixed device frequencies;
- fixed pulse durations;
- fixed gate fidelities;
- fixed error rates;
- fixed coherence times;
- fixed physical calibration constants.

Calibration belongs to device/runtime/HAL layers.

Source-level semantic constraints may refer to properties such as:

requires fidelity >= threshold

without embedding the current device's measured value.

---

37. QEC hard-coding

Quantum error correction MUST NOT be represented as a finite machine-specific language limit.

The grammar MUST NOT impose universal constants for:

code distance
syndrome rounds
ancilla count
physical qubit overhead
error rate
decoder capacity

unless a value is explicitly part of a program's requested error-correction strategy.

The grammar expresses intent.

QEC analysis determines realization.

---

38. Resilience states

The resilience model may define semantic states such as:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

These are semantic vocabulary, not machine-capacity limits.

The validator MUST distinguish:

finite semantic state set

from:

finite hardware-resource universe

A closed state machine is not automatically hard-coding.

---

39. Outcome enumeration

Similarly, semantic outcomes may legitimately be enumerated:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

because they describe a defined semantic protocol.

The validator must not incorrectly classify every finite enumeration as hard-coding.

The key question is:

«Does the enumeration define a semantic state space, or does it claim to enumerate all possible resources/targets?»

---

40. Generated code

Generated artifacts may contain constants that are not present in the source language.

Examples:

- target-specific instruction encodings;
- register numbers;
- physical resource IDs;
- ABI constants;
- device-specific limits;
- generated lookup tables.

These are allowed only if they are clearly generated downstream.

The hard-coding validator MUST distinguish:

source language hard-coding

from:

target realization

A generated artifact must not be fed back into the language specification as authoritative grammar.

---

41. Macro expansion

Macros MUST NOT be used to bypass the hard-coding policy.

A macro that expands:

MAX_QUBITS = 32

is still a hard-coding violation if the expansion becomes language semantics.

The audit must inspect:

1. macro declarations;
2. macro arguments;
3. token expansion;
4. generated syntax;
5. generated AST;
6. generated semantic metadata.

Hard-coding validation must operate on both source and expanded representations where expansion is available.

---

42. Metaprogramming

Compile-time generation MUST NOT provide an escape hatch around hard-coding policy.

A generator that creates:

32 quantum alternatives

instead of explicitly writing them is still encoding a fixed universe.

The validator must reason about the semantic result, not merely the source text.

---

43. Dialects

A dialect may intentionally define target-specific behavior.

That does not make the behavior universally portable.

Every dialect MUST declare:

dialect name
version
scope
target/domain
syntax extensions
semantic extensions
AST mapping
IR mapping
compatibility
portability classification

A dialect-specific hard limit MUST NOT leak into the core Zamani language.

---

44. Explicit target-specific source

Zamani may eventually support explicitly target-specific programs.

Such programs MUST identify their scope.

For example:

target-specific

or an equivalent target declaration may establish that subsequent declarations are not intended to be portable.

Even then, the target-specific constraint must not silently modify the core language definition.

The validator must classify:

portable
target-specific
backend-specific
implementation-specific

separately.

---

45. Portability levels

Every hard-coded value detected by production validation should be assigned a portability scope:

Universal
Portable
Domain-specific
Dialect-specific
Target-specific
Backend-specific
Implementation-specific
Test-only
Documentation-only

The validator MUST reject values whose scope is narrower than the owning artifact's declared portability scope.

Example:

grammar/Zamani.g4
scope = Universal

cannot contain:

target-specific maximum

as semantic syntax.

---

46. Ownership rule

Every fixed value MUST have an owner.

Valid owners include:

language semantics
mathematical specification
external protocol
interoperability standard
program source
explicit target contract
backend implementation
runtime
test suite
documentation
tooling

If a value has no identifiable owner, production validation MUST fail.

This rule prevents unexplained constants from becoming permanent architecture.

---

47. Provenance rule

Every non-trivial fixed value that survives the audit MUST have provenance.

The provenance record should identify:

value
location
category
owner
scope
semantic meaning
source specification
why it is fixed
why it is not a scalability limit
downstream consumer
tests

For implementation values, provenance should additionally identify the layer responsible for them.

---

48. Semantic constants

A semantic constant is allowed when changing it would change the language's meaning rather than merely accommodate a different machine.

Examples may include:

- mathematical constants;
- language-defined sentinel values;
- protocol-defined field markers;
- language version identifiers;
- standardized semantic states.

The validator should ask:

«If hardware became arbitrarily larger tomorrow, would this value still define the same language semantics?»

If yes, it may be semantic.

If no, it is likely implementation or target hard-coding.

---

49. Resource quantities

Resource quantities are allowed as source data.

Examples:

requires qubits >= 1000
requires memory >= 1GiB
requires nodes >= n

The values belong to the program.

They do not define the maximum that Zamani supports.

The distinction MUST remain visible in AST and semantic contracts.

---

50. Arbitrary resource scale

Resource representation must scale according to representable program values and available implementation resources.

The grammar MUST NOT restrict:

resource count
resource quantity
resource dimension
resource rank
resource node count

to a fixed language-defined machine capacity.

Where arbitrary-precision or symbolic quantities are required, the implementation should use appropriate safe representations rather than silently narrowing them to a target machine width.

---

51. Integer overflow and validation

A numeric literal's value must not be rejected merely because it exceeds the implementation's native integer type.

The lexical and semantic design must distinguish:

literal syntax

from:

backend representation

Where the language requires arbitrary magnitude, the implementation must preserve the value using a suitable safe representation.

A parser MUST NOT silently truncate an oversized literal.

A validator MUST NOT reinterpret an oversized literal as evidence that the language itself has reached its maximum.

---

52. Resource availability

POCO-REAF requires that source validity and target feasibility remain separate.

Therefore:

valid program

does not necessarily mean:

currently executable on this target

A program may be syntactically and semantically valid while a selected target reports:

resource unavailable
capability unavailable
constraint unsatisfied

This is not hard-coding.

It is resource negotiation.

---

53. Compiler behavior

The compiler MUST NOT convert a backend limitation into a language limitation.

Bad:

compiler supports at most N qubits
therefore Zamani supports at most N qubits

Correct:

Zamani expresses arbitrary valid quantum intent
        ↓
compiler analyzes requirements
        ↓
target capabilities are discovered
        ↓
compiler determines feasibility
        ↓
compiler lowers if feasible

If the backend cannot satisfy a program, the compiler must report a target/resource/capability diagnostic.

It must not redefine the language.

---

54. Diagnostic distinction

Hard-coding validation must distinguish at least:

language error
semantic error
resource requirement failure
capability failure
target incompatibility
implementation limitation
tooling resource exhaustion
hard-coding violation

A target that cannot execute a program is not evidence that the program is syntactically invalid.

---

55. Required diagnostic information

A hard-coding violation diagnostic should identify:

diagnostic code
source file
source span
detected value
classification
reason
owning layer
expected layer
affected portability scope
recommended semantic boundary

Example conceptual diagnostic:

ZMN-VAL-HARD-CODE-RESOURCE-LIMIT

A universal grammar rule encodes a fixed hardware-resource limit.

Detected:
    MAX_QUBITS = 32

Location:
    grammar/quantum/...

Classification:
    accidental language-level hard-coding

Required:
    express the requirement through resource/capability semantics

Expected ownership:
    resource analysis / target realization

Forbidden ownership:
    universal grammar

---

56. Diagnostic source spans

Hard-coding diagnostics MUST integrate with:

"grammar/validation/source-spans.md"

and:

"grammar/spec/source-spans.md"

A diagnostic must identify the smallest useful source region.

The validator must not require copying the entire source file into every diagnostic.

It must not store raw memory addresses as source-span semantics.

---

57. Token-level detection

The hard-coding validator may inspect:

- identifiers;
- numeric literals;
- string literals;
- ranges;
- enumerations;
- parser rules;
- grammar alternatives;
- annotations;
- attributes;
- comments where policy metadata is declared.

Token detection is necessary but insufficient.

A numeric literal by itself is not proof of hard-coding.

---

58. Grammar-structure detection

The validator MUST inspect grammar structure for:

- finite repetition used as a hidden maximum;
- fixed-length sequences;
- closed resource enumerations;
- target-specific alternatives;
- fixed physical identifiers;
- fixed hardware dimensions;
- fixed gate sets;
- target-specific literals;
- finite capability lists presented as exhaustive;
- hard-coded ranges;
- parser-level resource limits.

---

59. Semantic detection

Where possible, validation must inspect semantic metadata.

Examples:

field = max_qubits

and:

field = resource_requirement

must not receive the same classification.

The validator should use feature contracts and semantic annotations to distinguish intent.

---

60. AST detection

The AST contract must not encode a backend limit as a universal structural invariant.

Bad:

struct QuantumRegister {
    qubits: [Qubit; 32],
}

when this means Zamani supports exactly 32 qubits.

Correct architecture uses a dynamically represented or semantically parameterized collection.

The actual representation must remain implementation-safe and must not use "unsafe".

---

61. IR detection

The canonical IR is where semantic resource information becomes explicit.

The validator must verify that resource requirements are represented as:

requirements
capabilities
constraints
preferences
properties

rather than silently becoming:

fixed backend dimensions

For quantum, the canonical semantic path remains:

Zamani
  ↓
AST
  ↓
semantic quantum model
  ↓
quantum::ir

There must not be a second frontend quantum IR created merely to carry hardware-specific limits.

---

62. "quantum::ir" integration

The hard-coding validator must inspect the boundary into:

src/quantum/ir

and its resource/capability analysis.

The quantum IR may contain resource requirements and target-independent resource metadata.

It MUST NOT turn current hardware availability into universal source-language constants.

The IR must remain capable of representing programs whose resource requirements exceed any particular currently available machine.

---

63. Routing integration

Routing may determine physical realization.

It may therefore contain target-specific topology data.

Such data is not a language hard-coding violation if it remains within routing/target realization.

However:

routing constraint

must not be copied into:

grammar semantic rule

without explicit semantic justification.

---

64. Scheduling integration

Scheduling may use:

- device capacity;
- timing;
- queue state;
- topology;
- calibration;
- resource availability.

These are execution concerns.

The grammar must not encode them as universal language limits.

The scheduling layer may reject an execution plan because a target cannot satisfy it.

That is target feasibility, not grammar validation.

---

65. QEC integration

QEC may calculate:

- physical resource overhead;
- code distance;
- syndrome rounds;
- ancilla requirements;
- error budget;
- decoder requirements.

These values must remain downstream semantic/optimization data.

The grammar must express QEC intent, not the implementation's current maximum capacity.

---

66. ZQN integration

ZQN/fault/noise semantics may contain physical or operational quantities.

The hard-coding policy must ensure those quantities do not become universal grammar limits.

The ownership chain is:

source intent
    ↓
quantum::ir
    ↓
ZQN / resilience analysis
    ↓
target/device information

not:

target device
    ↓
grammar

---

67. HAL integration

The Hardware Abstraction Layer owns target realization.

The grammar must not know:

- device IDs;
- physical addresses;
- calibration records;
- physical topology;
- backend-specific resource counts.

The grammar may express the capability contract that the HAL must satisfy.

---

68. HDL integration

HDL is allowed to express concrete hardware.

This is not automatically portable source semantics.

For example:

register width = 32

can be valid inside a specific hardware module because the designer explicitly requested that hardware structure.

The validator must determine whether the declaration is:

explicit design intent

or:

universal language limit

Only the latter is forbidden.

---

69. Hardware intent integration

The "hardware/" grammar should describe:

- capabilities;
- resources;
- constraints;
- topology;
- timing;
- power;
- reliability;
- calibration intent;
- deployment.

It must not transform one target's resource profile into the language's global resource model.

---

70. Resource grammar integration

"resources/" is the preferred location for target-independent resource requirements.

A resource requirement should remain semantic:

requires X

rather than:

compiler has X

The hard-coding validator must verify that "resources/" constructs do not encode maximums.

---

71. Compile integration

"compile/" may contain target selection and compilation profiles.

It must distinguish:

requested target

from:

required target

and:

backend implementation

A compilation profile may intentionally be target-specific.

It must not silently redefine the portable source language.

---

72. Execution integration

"execution/" may contain runtime policies.

Runtime policies must not become parser-level resource limits.

For example:

retry count
checkpoint interval
scheduling policy
recovery policy

may be program semantics.

A runtime's own maximum retry count or queue size is an implementation property.

---

73. Distributed integration

Distributed source constructs must remain independent of cluster size.

The validator must flag:

MAX_NODES
MAX_REPLICAS
MAX_PARTITIONS
MAX_SERVICES

when used as universal language limits.

It must allow:

requires nodes >= n
replicas = desired_replication

when these are program semantics.

---

74. AI integration

AI constructs must not encode framework-specific hardware assumptions.

Forbidden universal assumptions include:

maximum tensor rank
maximum model size
maximum accelerator count
maximum training nodes
maximum batch size

when imposed by the language.

Program-specific model dimensions remain valid.

---

75. Data integration

Data schemas may legitimately contain fixed fields and dimensions.

The validator must not classify:

record has exactly 5 fields

as hardware hard-coding when that is the actual data schema.

It must distinguish:

semantic schema

from:

compiler storage limit

---

76. Security integration

Security algorithms and protocols may have fixed parameters defined by standards.

These are not automatically hard-coding.

However, the grammar must not claim:

only algorithm X is possible

when the security domain is intentionally extensible.

Capability and algorithm identifiers should remain extensible where appropriate.

---

77. Interoperability integration

Foreign interfaces may contain fixed:

- ABI widths;
- calling conventions;
- protocol fields;
- external type identifiers.

These belong to the interoperability contract.

They must not leak into the universal Zamani hardware model.

---

78. Compatibility integration

A historical implementation may have had a fixed limit.

Compatibility support may need to recognize it.

That does not make the old limit a current universal limit.

Compatibility documentation must distinguish:

historical limitation

from:

current language rule

---

79. Deprecated hard-coding

A deprecated hard-coded feature remains a validation concern.

The validator must not allow a deprecated universal hardware limit merely because it is old.

Instead it should be classified:

deprecated hard-coding

and routed through the compatibility/migration system.

---

80. Historical "Zamani-Grammar.md"

"grammar/Zamani-Grammar.md" may contain historical, proposed, experimental, or unimplemented material.

A hard-coded limit appearing there does not automatically define current Zamani.

However, if the feature is promoted into the authoritative language, it MUST pass this policy before promotion.

The promotion path remains:

historical/proposed
    ↓
semantic design
    ↓
AST contract
    ↓
canonical grammar
    ↓
semantic implementation
    ↓
IR contract
    ↓
tests
    ↓
stable

---

81. "grammar.md" integration

"grammar/grammar.md" is an implementation/conformance reference.

It must report hard-coding status where appropriate.

A construct marked:

IMPLEMENTED

must not silently violate this policy.

A construct marked:

PLANNED

does not receive an exemption.

---

82. "Zamani.g4" integration

"grammar/Zamani.g4" is the canonical ANTLR composition root.

The hard-coding audit must specifically inspect it for:

- hardware constants;
- fixed resource ranges;
- finite operation universes;
- fixed target IDs;
- fixed physical mappings;
- parser-level maximums;
- vendor-specific universal syntax;
- fixed quantum gate universes.

"Zamani.g4" must remain a composition root rather than becoming the place where every possible hardware capability is enumerated.

---

83. Lexer integration

The lexer must not create hidden hard-coding through token definitions.

Examples:

only allowing 32-bit numeric literals
only allowing N predefined devices
only recognizing fixed qubit names
only recognizing a fixed vendor vocabulary

are violations unless the token is a genuine semantic keyword or externally standardized literal.

---

84. Numeric literals

Numeric literal handling must not silently narrow values.

The validator must ensure consistency between:

lexical representation

and:

semantic numeric representation

If the language claims arbitrary magnitude, an implementation that truncates or rejects values because of a native integer width is a conformance failure.

---

85. Unicode and identifiers

Unicode identifier handling must not introduce fixed hardware vocabularies.

Identifiers should remain extensible subject to lexical rules.

The hard-coding validator must not permit:

QUBIT0
QUBIT1
...

to become the implicit quantum universe simply because the lexer has special cases for them.

---

86. Duplicate-token integration

Hard-coding may hide behind duplicate tokens.

For example:

GPU
Gpu
gpu

should not accidentally create multiple competing hardware semantics.

Token authority must remain centralized.

This policy therefore integrates with:

"grammar/validation/duplicate-tokens.md"

and:

"grammar/lexer/tokens.md".

---

87. Keyword integration

The validator must distinguish:

semantic keyword

from:

enumerated implementation object

A keyword such as:

requires
capability
resource

can be language-level syntax.

A keyword for every current hardware model is generally inappropriate for the universal core language.

---

88. Extensibility requirement

Any domain that is inherently open-ended must use an extensible representation.

Open-ended domains include:

- quantum operations;
- hardware capabilities;
- device types;
- accelerator kinds;
- future computing substrates;
- protocols;
- vendor extensions;
- dialects;
- resource kinds.

A finite grammar list may be used only when the semantic domain itself is intentionally closed.

---

89. Closed semantic domains

The validator must not reject every finite enumeration.

Examples of legitimately closed domains may include:

boolean

or a formally specified finite protocol state machine.

The correct test is:

«Is the finite set part of the definition of the semantic abstraction, or is it an accidental snapshot of current implementations?»

Only the latter is hard-coding.

---

90. Detection categories

The validator should classify findings using stable identifiers.

Recommended categories:

HC001  Universal hardware capacity
HC002  Universal resource maximum
HC003  Fixed topology
HC004  Fixed device identity
HC005  Fixed vendor universe
HC006  Fixed quantum operation universe
HC007  Fixed hardware width
HC008  Fixed memory capacity
HC009  Fixed concurrency capacity
HC010  Fixed distributed capacity
HC011  Fixed tensor capacity
HC012  Fixed network capacity
HC013  Fixed accelerator capacity
HC014  Fixed timeline capacity
HC015  Fixed QEC capacity
HC016  Hidden grammar cardinality limit
HC017  Hidden semantic cardinality limit
HC018  Backend limit promoted to language
HC019  Runtime limit promoted to language
HC020  Unproven fixed value

Additional repository-specific identifiers may be added without changing these meanings.

---

91. Severity

Recommended severity:

Category| Severity
Universal hardware maximum| Error
Universal resource maximum| Error
Fixed physical identity in portable grammar| Error
Fixed quantum gate universe| Error
Fixed topology in portable grammar| Error
Hidden grammar cardinality limit| Error
Backend limit promoted to language| Error
Runtime limit promoted to language| Error
Unproven fixed value| Error in normative files
Target-specific value in target-specific file| Allowed
Test-only fixed value| Allowed
Documentation example| Allowed
Protocol-defined value| Allowed with provenance
Semantic finite enumeration| Allowed

Production validation MUST fail on unresolved errors.

---

92. Machine-checkable policy

The validator should represent each finding conceptually as:

HardCodingFinding {
    rule_id
    classification
    severity
    file
    source_span
    symbol
    value
    owner
    portability_scope
    semantic_role
    justification
    expected_layer
}

The exact Rust type may differ according to the repository's existing validation architecture.

This policy must not require a particular implementation representation.

---

93. Safe Rust implementation

All validation tooling implementing this policy MUST use safe Rust.

Rust "unsafe" is prohibited.

The validator MUST NOT require:

unsafe { ... }

or:

unsafe fn

or:

unsafe trait

or:

unsafe impl

or raw pointer dereferencing.

Safe standard-library data structures are preferred unless the repository already has an established safe dependency for a particular requirement.

---

94. Rust version

The implementation baseline is:

Rust 1.97
Rust 1.97.1

The implementation MUST NOT depend on language features newer than the selected supported Rust baseline.

Where the repository declares Rust "1.97.1", that version is the preferred concrete validation target.

---

95. Determinism

Hard-coding validation MUST be deterministic.

For identical:

source tree
configuration
tool version
policy version

the validator must produce equivalent findings.

The validator must not depend on:

- hash-map iteration order;
- filesystem traversal order;
- machine CPU count;
- current hardware;
- current wall-clock time;
- random ordering;
- network availability.

Diagnostics should be sorted using a stable ordering such as:

file path
source offset
rule identifier
symbol

---

96. No hardware discovery in grammar validation

The hard-coding validator MUST NOT query:

- CPU topology;
- GPU inventory;
- QPU inventory;
- FPGA inventory;
- memory size;
- network topology;
- local device state;
- runtime calibration.

Grammar validation determines whether the source architecture is portable.

Target feasibility is a downstream concern.

This keeps validation deterministic and prevents the local machine from changing the language accepted by the validator.

---

97. No network dependency

Hard-coding validation MUST work without network access.

External standards may be represented by repository-local provenance metadata.

The validator must not need to contact a vendor service to determine whether:

MAX_QUBITS = 32

is allowed.

The classification must be determined from repository contracts.

---

98. Repository scanning

Production validation should inspect at minimum:

grammar/
src/
tests/

and relevant generated/configuration files.

The audit should be capable of scanning:

.g4
.rs
.md
.toml
.yaml
.yml
.json

where those files participate in language or validation semantics.

Generated artifacts may be separately classified.

---

99. Scope-aware scanning

The validator must know whether a file is:

normative
implementation
target-specific
backend-specific
test-only
documentation-only
generated
historical
experimental
deprecated

A literal found in:

tests/fixtures/gpu-32-device.zm

must not automatically receive the same classification as the same literal in:

grammar/Zamani.g4

Context is mandatory.

---

100. Comments are not semantics

A comment containing:

MAX_QUBITS = 32

does not by itself make the grammar invalid.

However, normative comments that document a universal restriction are still policy violations.

The validator may therefore distinguish:

example-only comment

from:

normative documentation

Documentation must not contradict the language specification.

---

101. Documentation audit

The validator should inspect normative documentation for statements such as:

Zamani supports at most...
Zamani has exactly...
Zamani allows only...
The compiler supports up to...
Quantum programs may contain no more than...

when those statements describe artificial implementation limits as language rules.

Documentation hard-coding is a conformance problem even when the grammar itself is correct.

---

102. Test audit

Tests may contain fixed values.

Examples:

32 qubits
8 CPUs
1024 nodes

are valid test fixtures when they demonstrate behavior.

But tests MUST NOT assert that the value is the universal maximum unless the test explicitly verifies a target/tooling limit.

A test named:

reject_more_than_32_qubits

is prohibited for portable quantum grammar conformance unless it is explicitly scoped to a target-specific backend.

---

103. Positive scalability tests

The test suite must prove that values larger than historical implementation examples remain syntactically expressible where the language semantics permit them.

Examples should include:

small resource requirement
larger resource requirement
symbolic resource requirement
parameterized resource requirement

The purpose is to prove that examples such as 32, 64, or 1024 do not accidentally become maxima.

---

104. Boundary tests

Boundary tests must test semantic boundaries rather than invent artificial language boundaries.

For example:

zero
one
small positive value
large representable value
symbolic value

must be distinguished from:

compiler maximum

A boundary test that establishes a maximum must identify its owner.

---

105. Negative tests

Negative tests must demonstrate rejection of genuine hard-coding.

Examples:

MAX_QUBITS = 32

inside a universal grammar contract.

MAX_CPUS = 8

inside a portable compiler semantic model.

device_count <= 4

as a universal language restriction.

quantumGate : H | X | Y | Z | CNOT

when documented as an exhaustive universal gate language.

---

106. False-positive prevention

The validator must not report ordinary semantic constants as hard-coding merely because they are numeric.

Examples that should normally be allowed:

let retries = 3;
let threshold = 0.95;
let matrix = Matrix<4, 4>;

The finding must depend on ownership and semantic role.

---

107. Value-flow analysis

Where practical, the validator should follow constant values through simple aliases.

For example:

MAX = 32
QUANTUM_LIMIT = MAX

must still be detected.

Likewise:

SUPPORTED = 64
if qubits > SUPPORTED ...

must be detected when the value defines a universal capacity.

The implementation must remain deterministic and safe.

---

108. Structural aliasing

Hard-coding can be disguised through names.

Examples:

SUPPORTED
CAPACITY
LIMIT
BOUND
SIZE
WIDTH
COUNT
DEVICE_CAPACITY
AVAILABLE

must not automatically be considered safe.

The validator should inspect how the value is used.

---

109. Mathematical hard-coding versus hardware hard-coding

The validator must distinguish:

semantic mathematical bound

from:

implementation bound

For example, a mathematically defined function may have a domain restriction.

That restriction is semantic.

A compiler's inability to represent the result is implementation-specific.

---

110. Memory representation

The validator implementation itself must not assume that a Zamani quantity fits in a particular machine integer merely because the host is 64-bit.

Where exact numeric values are required, use a representation appropriate to the language contract.

Where only symbolic comparison is needed, preserve symbolic values rather than prematurely narrowing them.

---

111. Resource quantity semantics

Resource values should conceptually support:

literal
symbol
expression
range
constraint
requirement
unknown
target-discovered value

A validator must not collapse these categories into a fixed integer solely for convenience.

---

112. Capability semantics

Capabilities should remain extensible.

The validator must not reject:

requires capability("future.compute")

merely because that capability is unknown to today's validator, provided the syntax and semantic contract permit extensible capability identifiers.

Unknown capability means:

capability not established

not:

language-invalid

unless the language explicitly defines a closed capability namespace.

---

113. Extensible operation names

Quantum and other extensible operation names must not be validated against a hard-coded list unless a specific dialect intentionally defines such a list.

For example:

apply H
apply custom_gate
apply vendor.operation
apply future.operation(parameter)

must share the generic operation syntax.

Semantic validation may determine whether an operation is known, supported, or negotiable.

---

114. Target negotiation

The preferred flow is:

program
    ↓
requirements
    ↓
capabilities
    ↓
target discovery
    ↓
feasibility
    ↓
lowering

not:

target
    ↓
grammar definition

This is a core POCO-REAF invariant.

---

115. "Infinity" interpretation

"Scale to infinity" means:

«The language imposes no artificial finite maximum on computation merely because current machines are finite.»

It does not mean:

«Every physical or virtual machine can execute every program.»

Actual execution remains bounded by:

- available resources;
- physical laws;
- target capabilities;
- runtime constraints;
- time;
- memory;
- energy;
- deployment policy.

Those constraints belong downstream.

---

116. Resource exhaustion

Resource exhaustion is not necessarily a language failure.

For example:

program requires N qubits
target has fewer suitable resources

should produce a resource/capability/target diagnostic.

It should not cause the grammar to redefine:

N > current capacity => invalid Zamani

---

117. Compilation once

POCO-REAF requires a stable semantic representation independent of current hardware.

Compilation artifacts should therefore preserve:

source semantics
requirements
capabilities
constraints
provenance

rather than embedding current target assumptions into the source-language contract.

A backend may specialize the compiled artifact.

That specialization must remain distinguishable from the source semantics.

---

118. Run everywhere

"Run everywhere" means that valid target-independent programs can be lowered to compatible targets where requirements can be satisfied.

A target that lacks a required capability must report incompatibility.

It must not imply that the source language itself was wrong.

---

119. Future hardware

A production hard-coding audit must consider hypothetical future targets.

A finding is suspicious if adding a new target would require modifying the language merely because the target has:

- a new device category;
- a new accelerator type;
- a new quantum operation;
- a new topology;
- a new resource;
- a new memory architecture;
- a new execution model.

Extensible semantic identifiers should absorb such additions where the domain is open.

---

120. Future operation compatibility

New operations should normally be introducible through semantic registration/capability mechanisms rather than grammar rewrites.

The hard-coding audit must flag any proposed feature whose only implementation strategy is:

add another grammar alternative

when the domain is intended to be extensible.

---

121. Future hardware compatibility

New hardware should normally be introducible through:

capabilities
resources
target profiles
dialects
backend registration
HAL support

rather than editing the universal grammar to add another finite hardware case.

---

122. Generated feature manifests

If the repository uses feature manifests, each hard-coding-sensitive feature should be able to declare:

hard_coding_policy
portability_scope
resource_model
capability_model
target_specificity

The hard-coding validator should consume this metadata where available.

---

123. Feature completion contract

A feature is not complete merely because its ".g4" file parses.

For hard-coding compliance, every feature must establish:

Purpose
Owns
Does Not Own
Syntax
AST contract
Semantic contract
IR contract
Resource contract
Capability contract
Portability scope
Target-specific scope
Diagnostics
Positive tests
Negative tests
Boundary tests
Scalability tests
Compatibility tests
Hard-coding audit
Completion criteria

This preserves the repository requirement that a file can be completed independently without later rediscovery of missing architectural responsibilities.

---

124. File ownership rule

Every grammar file must state:

Owns:

and:

Does Not Own:

Hard-coding validation uses those boundaries.

If a file claims to own universal syntax but contains target-specific capacity assumptions, validation fails.

---

125. Integration with "grammar/README.md"

The grammar README must identify:

grammar/validation/hard-coding.md

as the normative hard-coding policy.

It must direct audit execution to:

grammar/validation/hardcoding-audit.md

The two files must not duplicate their responsibilities.

---

126. Integration with "grammar/DESIGN.md"

"DESIGN.md" remains the higher-level architectural authority.

This file implements the hard-coding portion of that architecture.

If the design document states:

no artificial hardware limits

this file defines how that principle is mechanically enforced.

---

127. Integration with "grammar/specification/portability.md"

Portability defines the language-level POCO-REAF contract.

This file defines the validation mechanism that prevents implementation details from violating that contract.

The portability specification remains the authority for the language principle.

This file remains the authority for hard-coding classification.

---

128. Integration with "grammar/validation/scalability-rules.md"

The relationship is:

hardcoding.md
    ↓
"What is this fixed value?"
    ↓
classification
    ↓
scalability-rules.md
    ↓
"Does this violate scalable semantics?"

"hardcoding.md" detects and classifies.

"scalability-rules.md" evaluates scalability implications.

The documents must not create contradictory definitions.

---

129. Integration with "grammar/validation/hardcoding-audit.md"

The relationship is:

hard-coding.md
    ↓
normative policy
    ↓
hardcoding-audit.md
    ↓
repository audit procedure
    ↓
findings/evidence

The existing "hardcoding-audit.md" MUST NOT create an alternative definition of hard-coding.

Where it currently contains policy language, that language must be interpreted consistently with this document.

No existing file needs to be renamed.

---

130. Integration with "grammar/validation/grammar-validator.md"

The grammar validator should invoke or expose hard-coding validation as one validation phase.

Conceptually:

parse grammar
    ↓
validate syntax
    ↓
validate references
    ↓
validate ambiguity
    ↓
validate tokens
    ↓
validate precedence
    ↓
validate source spans
    ↓
validate semantic coverage
    ↓
validate hard-coding
    ↓
validate scalability

The hard-coding phase must remain deterministic.

---

131. Integration with duplicate-token validation

Duplicate tokens can create hidden fixed universes.

For example:

Device0
Device1
Device2

must not accidentally become the complete device model.

Hard-coding validation and duplicate-token validation should exchange findings where one finding can explain another.

They must not duplicate diagnostic ownership.

---

132. Integration with semantic coverage

A hard-coding finding must be traceable through:

grammar rule
→ AST representation
→ semantic representation
→ IR representation
→ backend consumer

If a fixed value has no legitimate downstream semantic owner, it is presumptively accidental.

---

133. Integration with source spans

Every hard-coding finding must carry a stable source span.

The validator should not require re-reading the complete source for every finding.

It should use the repository's source-span abstraction.

---

134. Integration with diagnostics

Diagnostics must use the repository's diagnostic architecture.

Recommended conceptual identifiers:

ZMN-VAL-HARD-CODE-RESOURCE
ZMN-VAL-HARD-CODE-HARDWARE
ZMN-VAL-HARD-CODE-QUANTUM
ZMN-VAL-HARD-CODE-TOPOLOGY
ZMN-VAL-HARD-CODE-DEVICE
ZMN-VAL-HARD-CODE-VENDOR
ZMN-VAL-HARD-CODE-GRAMMAR
ZMN-VAL-HARD-CODE-IMPLEMENTATION

The final identifier set must follow the repository's canonical diagnostics registry if one exists.

---

135. Integration with tests

Tests must cover all major domains:

classical
quantum
HDL
hybrid
hardware
resources
distributed
AI
data
networking
security
concurrency
memory
compile
execution
interoperability
dialects
macros
metaprogramming

Every domain must contain at least:

valid scalable example
invalid hard-coded example
boundary example
target-specific example
portable resource requirement example

where applicable.

---

136. Required quantum tests

At minimum:

generic quantum operation
custom quantum operation
parameterized operation
multi-target operation
dynamic resource requirement
symbolic qubit requirement
large qubit requirement
logical-to-physical mapping outside source semantics
capability requirement
topology requirement

The tests must prove that:

MAX_QUBITS
fixed physical IDs
closed gate universes

do not define the portable language.

---

137. Required classical tests

At minimum:

large integer
large collection
large tensor
large matrix
large parallel workload
symbolic resource count
dynamic memory requirement
capability-based accelerator selection

No test may establish a fixed compiler maximum as language semantics.

---

138. Required HDL tests

At minimum:

parameterized width
concrete width
parameterized memory
concrete memory
generic module
target-specific module
timing intent
resource intent

The validator must distinguish:

designer explicitly requests 32 bits

from:

Zamani HDL only supports 32 bits

---

139. Required distributed tests

At minimum:

one node
multiple nodes
symbolic node count
large node requirement
replication
partitioning
dynamic placement
topology requirement

No universal maximum node count may be inferred.

---

140. Required AI/data tests

At minimum:

small tensor
large tensor
symbolic tensor dimensions
large model requirement
distributed training requirement
accelerator capability
dataset size
streaming data

No framework-specific machine capacity may become a language limit.

---

141. Required concurrency tests

At minimum:

single task
multiple tasks
dynamic task count
parallel operation
data parallelism
task parallelism
pipeline
distributed concurrency

No fixed thread/core count may be encoded into the language.

---

142. Required networking tests

At minimum:

single endpoint
multiple endpoints
dynamic endpoint count
stream
distributed service
topology requirement
capability requirement

No fixed network size may become a universal language limit.

---

143. Required compatibility tests

Compatibility tests must verify that:

historical limits

do not become:

current language limits

Deprecated syntax may remain parseable for compatibility according to the version policy, but it must not silently impose old hardware limits on new programs.

---

144. Required determinism tests

Run the hard-coding validator repeatedly against the same tree.

Expected:

same findings
same rule IDs
same source locations
same classifications
same ordering

No dependence on:

filesystem order
HashMap iteration
machine hardware
network state
wall-clock time
random seed

is permitted.

---

145. Required fuzzing

Fuzzing should target:

- numeric literals;
- resource expressions;
- ranges;
- identifiers;
- operation names;
- grammar alternatives;
- generated syntax;
- macro expansion;
- nested structures;
- large source files.

Fuzzing must remain safe Rust.

It must not use "unsafe".

Fuzzing must verify:

no panic
no undefined behavior
no infinite recovery loop
no non-deterministic classification
no silent truncation
no false acceptance of hard-coded limits

---

146. Large-source scalability

The validator should process large source trees without repeatedly rescanning the same content unnecessarily.

It should avoid:

- quadratic scans where practical;
- duplicated source storage;
- unbounded diagnostic duplication;
- recursive algorithms whose depth is derived directly from attacker-controlled source without a safe strategy.

Operational resource exhaustion may be reported as tooling failure.

It must not become a language semantic limit.

---

147. Large-value scalability

The validator must not convert every resource quantity to a fixed-width machine integer merely to compare it.

It should preserve:

literal
symbolic expression
range
unit
constraint

until semantic evaluation requires concrete resolution.

---

148. Unit-aware resource values

If the language supports units such as:

bytes
qubits
seconds
joules
bandwidth
frequency

the validator must distinguish:

unit semantics

from:

machine capacity

For example:

64GiB

is a valid program quantity.

It is not a declaration that Zamani's maximum memory is 64GiB.

---

149. Range expressions

Ranges require special attention.

A range such as:

0..1024

is valid when it describes program data.

It becomes forbidden when used to define the universal set of physical resources:

physical_qubit in 0..32

as the language's complete physical-qubit universe.

The validator must inspect ownership and context.

---

150. Collection cardinality

The language may represent fixed-size collections.

For example:

array[1024]

is valid program semantics.

The grammar MUST NOT use a fixed collection length to limit all arrays to 1024 elements.

---

151. Recursion and nesting

A grammar must not introduce artificial nesting limits such as:

MAX_NESTING = 256

as language semantics.

A parser implementation may require operational protection against pathological input.

Such a protection is tooling policy and must be clearly separated.

---

152. Macro recursion

Macro systems may require expansion-cycle detection.

A maximum expansion depth can be a tooling safety policy if necessary.

It must not be represented as:

Zamani macros may never be nested beyond N

unless that is intentionally part of the language specification.

The distinction must be documented.

---

153. AST collection limits

AST implementations must not impose fixed semantic collection limits.

For example, a "Vec<T>" may contain as many elements as the implementation can safely allocate.

The language itself remains unbounded with respect to artificial fixed limits.

---

154. IR collection limits

The same principle applies to IR.

A backend may fail to allocate an IR graph of a certain size.

That is not a language maximum.

The validator must distinguish:

allocation failure

from:

semantic invalidity

---

155. Compiler cache limits

Compiler caches may have finite capacity.

These values are implementation policy.

They must not appear in:

- language grammar;
- semantic specification;
- portable AST contracts;
- universal IR semantics.

---

156. Parallel validation

The validator may use parallel processing internally if the repository's implementation does so safely.

Parallelism must not affect result ordering.

No fixed number of worker threads may become a language limit.

The implementation must remain safe Rust.

---

157. Memory safety

No hard-coding validator may rely on:

unsafe

to achieve scalability.

Safe ownership, borrowing, slices, iterators, collections, and explicit lifetime management should be used.

A scalability optimization that requires unsafe code is outside this policy.

---

158. Security

Hard-coding validation must treat source input as untrusted.

It must guard against:

- pathological nesting;
- enormous literals;
- excessive diagnostic generation;
- repeated equivalent findings;
- pathological macro expansion;
- malicious identifiers;
- malformed Unicode;
- denial-of-service input.

These protections must remain tooling protections.

They must not silently change the language's semantics.

---

159. No implicit host-machine semantics

The validator must not use:

host CPU count
host memory
host pointer width
host architecture
host GPU count
host OS

to decide whether a Zamani grammar is valid.

Validation must be host-independent.

---

160. Cross-platform requirement

The same source and policy must produce equivalent hard-coding results on:

Linux
Windows
macOS
other supported environments

subject to repository-defined path normalization.

Host architecture must not alter the language's hard-coding rules.

---

161. Reproducible validation

Validation results should be reproducible from:

repository revision
policy revision
validator version
configuration

The result must not depend on the current machine's resource inventory.

---

162. CI requirements

Production CI should run at least:

cargo fmt --check
cargo check
cargo test
cargo clippy
grammar validation
hard-coding audit
scalability audit
semantic coverage

using the repository's actual commands and workspace structure.

No command may require Rust "unsafe".

The exact CI command set remains owned by the repository's build configuration.

---

163. CI failure conditions

CI MUST fail when a new universal hard-coding violation is introduced.

Examples:

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

or semantically equivalent limits.

CI should also fail for:

- fixed physical identifiers in portable grammar;
- closed future-hardware universes;
- hidden grammar cardinality limits;
- undocumented implementation limits promoted to semantics;
- inconsistent portability metadata.

---

164. Baseline and migration

Existing violations must be handled explicitly.

The audit should produce:

baseline
finding
classification
owner
migration plan
status

A baseline must never be interpreted as permanent permission.

New violations must fail validation even if old findings remain temporarily tracked.

---

165. No silent exemptions

An exemption must not be created merely to make CI pass.

Every exemption must identify:

finding
owner
reason
scope
expiry/review condition
replacement or migration path

Permanent exemptions for universal hardware limits are prohibited.

---

166. Exemption classes

Permitted exemption classes include:

external protocol
mathematical definition
target-specific artifact
test fixture
documentation example
generated artifact
tooling safety limit
historical compatibility

The exemption itself must not become language semantics.

---

167. Exemption validation

The validator should verify that exemptions remain within their declared scope.

For example:

target-specific exemption

must not be accepted when the same value appears in:

grammar/Zamani.g4

without target-specific scope.

---

168. Hard-coded names versus hard-coded meaning

Name-based scanning is useful but insufficient.

For example:

MAX_QUBITS

is suspicious.

But:

maximum

is not necessarily a violation.

Conversely:

capacity

may hide a violation.

Therefore production validation must combine:

lexical matching
+
syntax analysis
+
context
+
ownership
+
semantic role
+
scope

where the repository architecture makes those layers available.

---

169. Pattern families

The validator should maintain semantic pattern families rather than only one literal blacklist.

Families include:

maximum-capacity
minimum-capacity
fixed-count
fixed-width
fixed-topology
fixed-identity
fixed-enumeration
fixed-device
fixed-vendor
fixed-memory
fixed-concurrency
fixed-network
fixed-quantum
fixed-accelerator
fixed-distributed
fixed-timeline
fixed-QEC
fixed-runtime

Each family may contain aliases.

---

170. Blacklist policy

A blacklist of known identifiers is useful as an early warning.

It is not sufficient as the complete validator.

The validator MUST NOT assume:

not named MAX_* => safe

Semantic equivalents must still be detected.

---

171. Allowlist policy

An allowlist may identify known legitimate constants.

It must not override semantic analysis blindly.

For example:

u32
i64

may be valid type names.

An allowlist entry does not make:

MAX_REGISTER_WIDTH = 32

valid.

---

172. Layer-aware policy

Each repository path should have an expected portability scope.

Conceptually:

grammar/
    universal language policy

grammar/specification/
    normative language policy

grammar/spec/
    normative feature contracts

grammar/quantum/
    quantum language syntax

grammar/hardware/
    abstract hardware intent

grammar/resources/
    abstract resource intent

src/quantum/ir/
    canonical quantum semantic IR

routing/
    target realization

scheduling/
    execution realization

HAL/
    physical/backend realization

The validator should compare detected hard-coding against this ownership map.

---

173. Target-specific directories

Target-specific files may contain hard-coded values.

That is allowed when the directory/file explicitly declares target scope.

Examples:

backend-specific resource count
device mapping
physical topology
register assignment

These values must not flow backward into the universal grammar.

---

174. Source-to-target direction

The allowed dependency direction is:

portable source
    ↓
semantic requirements
    ↓
target discovery
    ↓
target constraints
    ↓
lowering
    ↓
target-specific realization

The forbidden direction is:

target capacity
    ↓
universal grammar

This is one of the most important hard-coding invariants.

---

175. No reverse contamination

If a backend introduces:

MAX_QUBITS = 127

the source language MUST NOT gain:

qubit count <= 127

unless 127 is genuinely a semantic property of a declared target-specific dialect.

A backend implementation cannot redefine the core language.

---

176. Capability discovery

Actual target capacity belongs to discovery.

Conceptually:

target
  ↓
capabilities
  ↓
resources
  ↓
constraints

The compiler then compares those against:

program requirements

This allows one source program to scale across machines with different capacities.

---

177. Resource negotiation

When multiple targets exist, the compiler may select an appropriate target based on:

requirements
capabilities
constraints
preferences
cost
availability
policy

The grammar must not encode the result of that negotiation as a universal constant.

---

178. Optimization

Optimization may specialize a program.

Specialization values must remain distinguishable from source semantics.

For example:

source requires capability X

may become:

backend selects device Y

The latter is not source-language hard-coding.

---

179. Compilation artifacts

Compilation artifacts may contain:

target IDs
physical resource IDs
instruction widths
scheduling values
addresses
device-specific constants

They must be classified as target realization.

They must not be regenerated into the grammar specification.

---

180. Provenance preservation

Whenever a fixed value is introduced downstream, provenance should preserve:

origin
source requirement
target capability
lowering decision
backend reason

This makes it possible to audit whether target-specific information has leaked backward.

---

181. Security against semantic injection

A malicious or accidental backend must not inject a grammar rule by generating a source-level file that is later treated as authoritative.

Generated artifacts must be clearly marked.

Authority remains:

DESIGN/specification/Zamani.g4

according to the repository authority model.

---

182. Historical implementation constants

Historical source code may contain:

32
64
1024

because an earlier implementation used those values.

The audit must not automatically declare them language violations.

It must determine whether those values affect:

language acceptance
semantic meaning
IR validity
target feasibility

If they do, they require remediation or explicit target-specific classification.

---

183. Documentation examples

Documentation may use:

32 qubits
8 cores
1024 nodes

as examples.

Examples must clearly be examples.

They must not say:

maximum
only supported
language limit

unless the limit is genuinely part of a declared target-specific contract.

---

184. Example labeling

Recommended documentation labels:

Example
Illustrative value
Target-specific example
Test fixture
Reference configuration

Avoid language such as:

maximum supported by Zamani

unless that is explicitly scoped to a particular implementation target.

---

185. Production-readiness criterion

Hard-coding validation is production-ready only when:

- universal hardware limits are rejected;
- semantic constants remain allowed;
- resource requirements remain allowed;
- target-specific values remain scoped;
- quantum operation universes are extensible;
- HDL concrete designs remain possible;
- future hardware can be introduced without core grammar changes where the domain is extensible;
- diagnostics are deterministic;
- source spans are precise;
- validation is safe Rust;
- no "unsafe" is required;
- validation is independent of host hardware;
- large valid source values are not silently narrowed;
- tests cover positive/negative/boundary/scalability cases;
- compatibility behavior is explicit;
- generated artifacts do not become grammar authority.

---

186. Completion checklist for this file

This file is complete only when the following are satisfied:

Authority

- [ ] "DESIGN.md" relationship defined.
- [ ] "Zamani.g4" relationship defined.
- [ ] "grammar.md" relationship defined.
- [ ] "Zamani-Grammar.md" relationship defined.
- [ ] "hardcoding-audit.md" relationship defined.
- [ ] "scalability-rules.md" relationship defined.
- [ ] "grammar-validator.md" relationship defined.

Classification

- [ ] semantic constants defined;
- [ ] program data defined;
- [ ] resource requirements defined;
- [ ] constraints defined;
- [ ] capabilities defined;
- [ ] preferences defined;
- [ ] implementation limitations defined;
- [ ] target-specific values defined;
- [ ] safety limits defined;
- [ ] test-only values defined;
- [ ] documentation-only values defined;
- [ ] generated values defined;
- [ ] accidental hard-coding defined.

Scalability

- [ ] quantum limits prohibited;
- [ ] classical limits prohibited;
- [ ] GPU limits prohibited;
- [ ] FPGA limits prohibited;
- [ ] ASIC assumptions prohibited;
- [ ] distributed limits prohibited;
- [ ] networking limits prohibited;
- [ ] memory limits prohibited;
- [ ] tensor limits prohibited;
- [ ] timeline limits prohibited;
- [ ] QEC limits prohibited;
- [ ] accelerator limits prohibited.

Quantum

- [ ] fixed gate universe prohibited;
- [ ] physical qubit universe prohibited;
- [ ] QEC capacity limits prohibited;
- [ ] quantum resource requirements supported;
- [ ] quantum capability requirements supported;
- [ ] "quantum::ir" integration defined.

HDL

- [ ] parameterized hardware supported;
- [ ] concrete hardware designs supported;
- [ ] universal hardware limits prohibited;
- [ ] target-specific hardware distinguished.

Implementation

- [ ] Rust 1.97/1.97.1 defined;
- [ ] "unsafe" prohibited;
- [ ] deterministic validation required;
- [ ] host-independent validation required;
- [ ] network-independent validation required;
- [ ] large values preserved;
- [ ] source spans integrated.

Testing

- [ ] positive tests;
- [ ] negative tests;
- [ ] boundary tests;
- [ ] scalability tests;
- [ ] determinism tests;
- [ ] compatibility tests;
- [ ] fuzzing;
- [ ] repository-wide CI integration.

---

187. Required production test matrix

The final validation suite must cover:

Domain| Portable semantics| Hard-coding detection| Target-specific allowance| Scalability
Classical| Required| Required| Required| Required
Quantum| Required| Required| Required| Required
Hybrid| Required| Required| Required| Required
HDL| Required| Required| Required| Required
Hardware| Required| Required| Required| Required
Resources| Required| Required| Required| Required
Distributed| Required| Required| Required| Required
AI| Required| Required| Required| Required
Data| Required| Required| Required| Required
Networking| Required| Required| Required| Required
Security| Required| Required| Required| Required
Concurrency| Required| Required| Required| Required
Memory| Required| Required| Required| Required
Compile| Required| Required| Required| Required
Execution| Required| Required| Required| Required
Interoperability| Required| Required| Required| Required
Dialects| Required| Required| Required| Required
Macros| Required| Required| Required| Required
Metaprogramming| Required| Required| Required| Required

---

188. Canonical examples

Allowed

requires qubits >= n

requires memory >= required_memory

requires capability("gpu.compute")

requires capability("quantum.measurement")

requires capability("tensor.compute")

requires topology(required_topology)

let n = 1024;

Tensor<f64, [1024, 1024]>

apply custom_gate to q

apply vendor.operation(parameter) to q0, q1

register<u32> r;

when the width is explicit program/hardware intent.

---

189. Forbidden universal semantics

MAX_QUBITS = 32

MAX_CPUS = 8

MAX_GPUS = 4

MAX_FPGAS = 16

MAX_NODES = 1024

MAX_MEMORY = 64GB

MAX_THREADS = 1024

MAX_TENSOR_RANK = 8

MAX_REGISTER_WIDTH = 64

MAX_NETWORK_SIZE = 1024

MAX_DEVICE_COUNT = 16

and semantic equivalents such as:

qubits <= 32

node_count < 1025

register_width in 1..64

when these define universal Zamani limits.

---

190. Forbidden closed quantum universe

quantumGate
    : H
    | X
    | Y
    | Z
    | CNOT
    ;

when that rule claims to define all legal quantum operations.

Preferred:

quantumOperation
    : operationSpecifier quantumTargetList
    ;

with operation identity resolved semantically.

---

191. Forbidden hidden HDL limit

wire [31:0]

is not intrinsically forbidden as a concrete HDL declaration.

What is forbidden is a rule equivalent to:

all Zamani hardware wires are 32-bit

or:

Zamani HDL accepts no width above 32

The validator must therefore inspect semantic scope.

---

192. Forbidden backend promotion

This sequence is prohibited:

backend currently supports 32 qubits
        ↓
compiler assumes 32
        ↓
grammar rejects >32
        ↓
language specification says maximum 32

The correct sequence is:

source requirement
        ↓
semantic IR
        ↓
target capabilities
        ↓
feasibility
        ↓
backend realization

---

193. Required architecture

The final hard-coding architecture must therefore be:

                    Zamani source
                         │
                         ▼
                  Universal grammar
                         │
                         ▼
                     Frontend AST
                         │
                         ▼
                Semantic validation
                         │
              ┌──────────┴──────────┐
              │                     │
         Requirements          Capabilities
              │                     │
              └──────────┬──────────┘
                         ▼
                  Canonical semantic IR
                         │
              ┌──────────┼──────────┐
              │          │          │
        Classical IR  quantum::ir   HDL
              │          │          │
              └──────────┼──────────┘
                         ▼
                   Optimization
                         │
              ┌──────────┼──────────┐
              │          │          │
           Routing   Scheduling    QEC
              │          │          │
              └──────────┼──────────┘
                         ▼
                        ZQN
                         │
                         ▼
                        HAL
                         │
                         ▼
                 Target realization

Hard-coded target capacity may exist only at or below the target-realization boundary unless the program explicitly declares target-specific semantics.

---

194. Final invariant

The complete rule is:

«No implementation detail may become a universal Zamani language limitation merely because the current implementation has finite resources.»

Therefore:

tiny machine
    ↓
same program
    ↓
larger machine
    ↓
same program
    ↓
distributed system
    ↓
accelerator
    ↓
FPGA
    ↓
ASIC
    ↓
QPU
    ↓
future computational substrate

must remain architecturally possible without modifying the source language solely because the target became larger or different.

The actual target may reject execution when resources or capabilities are insufficient.

That is a target-feasibility result, not a grammar limitation.

---

195. Final production rule

A hard-coded value is acceptable only when all of the following are true:

1. It has an identifiable semantic owner.
2. Its portability scope is explicit.
3. It is not an artificial hardware/resource maximum.
4. It does not constrain future targets unnecessarily.
5. It does not create a hidden grammar cardinality limit.
6. It does not create a closed universe for an extensible domain.
7. It does not leak backend state into universal semantics.
8. Its source span and provenance are available.
9. Its tests establish the intended scope.
10. Its integration with AST/semantic/IR is defined.
11. Its compatibility behavior is defined.
12. Its implementation does not require unsafe Rust.

Otherwise:

PRODUCTION VALIDATION = FAIL

---

196. Definition of done

"grammar/validation/hard-coding.md" is considered fully integrated when the repository can demonstrate all of the following:

                 hard-coding.md
                       │
                       ▼
             classification policy
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
       grammar      semantics      tests
          │            │            │
          ▼            ▼            ▼
      validator      AST/IR     conformance
          │            │            │
          └────────────┼────────────┘
                       ▼
               scalability audit
                       │
                       ▼
                 CI enforcement

and the repository can prove:

No accidental hardware limit
No accidental resource limit
No accidental topology limit
No accidental device limit
No accidental quantum-operation universe
No accidental tensor limit
No accidental concurrency limit
No accidental distributed limit
No accidental memory limit
No accidental timeline limit
No accidental QEC limit
No backend-to-language contamination
No target-to-language contamination
No unsafe Rust
No host-machine-dependent validation
No nondeterministic validation
No silent numeric truncation
No competing hard-coding authority

The resulting contract is:

Zamani language semantics
        ≠
current hardware capacity
        ≠
compiler implementation capacity
        ≠
runtime capacity
        ≠
target availability

and:

Program Once
      ↓
Compile Once
      ↓
Preserve portable semantics
      ↓
Discover capabilities/resources
      ↓
Specialize and lower
      ↓
Run wherever requirements can be satisfied

This is the hard-coding foundation required for production-grade POCO-REAF.