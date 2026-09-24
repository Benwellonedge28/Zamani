Zamani Quantum Interoperability Specification

Path: "grammar/quantum/interoperability.md"
Domain: Quantum computing
Status: Normative
Specification role: Quantum interoperability, import/export, external-format boundaries, semantic preservation, canonical IR integration, compatibility, scalability, security, determinism, and conformance contract
Language: Zamani
Grammar technology: ANTLR4
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust edition: Rust 2021
Safety: "#![deny(unsafe_code)]"; no "unsafe" Rust
Execution model: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the production interoperability contract between the Zamani quantum language and external quantum representations.

It specifies how Zamani interacts with:

- OpenQASM;
- QIR;
- Quil;
- other standardized quantum representations;
- vendor-neutral quantum representations;
- vendor-specific quantum formats;
- quantum simulators;
- quantum circuit interchange formats;
- quantum hardware descriptions;
- pulse-level representations;
- future quantum formats;
- dialect-defined quantum representations.

The central architectural rule is:

«External quantum formats are interoperability boundaries, not competing definitions of Zamani quantum semantics.»

The canonical direction is:

External quantum format
        │
        ▼
format-specific frontend
        │
        ▼
format-specific validation
        │
        ▼
canonical semantic mapping
        │
        ▼
canonical quantum::ir
        │
        ▼
Zamani optimization / routing / scheduling / QEC / resilience / ZQN
        │
        ▼
HAL / target realization

For export:

Zamani source
        │
        ▼
frontend AST
        │
        ▼
semantic analysis
        │
        ▼
canonical quantum::ir
        │
        ▼
representability analysis
        │
        ▼
format-specific exporter
        │
        ▼
external quantum format

No external format may replace the canonical Zamani semantic model.

---

2. Normative Authority

Quantum interoperability is governed by the repository-wide authority hierarchy.

The authority order is:

1. "grammar/DESIGN.md"
2. versioned Zamani language specification
3. "grammar/spec/quantum.md"
4. "grammar/quantum/interoperability.md"
5. canonical grammar and lexical contracts
6. domain-neutral frontend AST
7. semantic analysis
8. canonical "quantum::ir"
9. compiler/lowering contracts
10. runtime/HAL contracts
11. external format implementations

The following documents are not independent authorities:

- "grammar/Zamani-Grammar.md";
- generated/reference "grammar/grammar.md";
- external OpenQASM specifications;
- QIR specifications;
- vendor SDK specifications;
- vendor-specific circuit formats;
- simulator APIs.

External standards define the meaning of their own formats.

They do not redefine the meaning of Zamani.

---

3. Repository Integration

This contract is specifically designed around the existing Zamani repository.

Relevant boundaries include:

grammar/DESIGN.md
grammar/Zamani.g4
grammar/spec/quantum.md
grammar/spec/compatibility.md
grammar/quantum/README.md
grammar/quantum/quantum.g4
grammar/quantum/operations.g4
grammar/quantum/types.g4
grammar/quantum/registers.g4
grammar/quantum/states.g4
grammar/quantum/measurement.g4
grammar/quantum/channels.g4
grammar/quantum/noise.g4
grammar/quantum/error-correction.g4
grammar/quantum/resource-requirements.g4
grammar/quantum/quantum-capabilities.g4
grammar/quantum/dynamic-control.g4
grammar/quantum/classical-feedforward.g4
grammar/quantum/circuits.g4
grammar/quantum/kernels.g4
grammar/quantum/adjoints.g4
grammar/quantum/controls.g4
grammar/quantum/controlled-operations.g4
grammar/quantum/interoperability.md

The implementation side currently includes the OpenQASM frontend boundary:

src/quantum/frontend/formats/openqasm/

with the public facade:

src/quantum/frontend/formats/openqasm/mod.rs

The current OpenQASM architecture already establishes the correct boundary:

OpenQASM
    ↓
OpenQASM lexer
    ↓
OpenQASM parser
    ↓
OpenQASM AST
    ↓
OpenQASM validation
    ↓
controlled lowering
    ↓
canonical quantum::ir

This interoperability contract formalizes that architecture for OpenQASM and future formats.

---

4. One Canonical Quantum Semantic Boundary

There MUST be exactly one canonical quantum semantic/IR boundary:

quantum::ir

Interoperability implementations MUST NOT introduce another canonical representation such as:

OpenQasmIR
QasmIR
QIRIR
QuilIR
VendorQuantumIR
QuantumInterchangeIR
QuantumFormatIR
QuantumCircuitIR

as a competing semantic representation.

A format-specific AST is permitted.

A format-specific parser representation is permitted.

A format-specific validation representation is permitted.

A temporary format-specific lowering structure is permitted when required internally.

A second canonical quantum IR is not permitted.

The boundary MUST remain:

external format
    ↓
format representation
    ↓
validation
    ↓
canonical quantum::ir

and:

canonical quantum::ir
    ↓
representability validation
    ↓
external format

---

5. Interoperability Is Not Semantic Authority

External formats can represent concepts that Zamani does not currently expose directly.

External formats can also omit concepts that Zamani represents.

Therefore interoperability MUST NOT assume:

external syntax == Zamani syntax

or:

external AST == Zamani AST

or:

external IR == quantum::ir

Instead:

External semantics
       ↓
semantic interpretation
       ↓
Zamani semantic compatibility analysis
       ↓
quantum::ir

If the external representation contains semantics that cannot be represented faithfully by "quantum::ir", the importer MUST reject the conversion unless an explicitly specified lossless or approved lowering exists.

---

6. Three Conversion Outcomes

Every import or export operation MUST have exactly one of these semantic outcomes:

SUPPORTED
    ↓
validated and converted

UNSUPPORTED
    ↓
structured diagnostic
    ↓
conversion rejected

INVALID
    ↓
structured syntax/semantic diagnostic
    ↓
conversion rejected

There MUST NOT be an implicit fourth outcome:

parsed
    ↓
unsupported information silently discarded

Silent semantic loss is prohibited.

---

7. Lossless Interoperability

An interoperability conversion is lossless when all semantically relevant information required by the applicable contract is preserved.

For import:

External P
    ↓
Zamani
    ↓
quantum::ir

must preserve every required semantic property that the external format and Zamani contract define as representable.

For export:

quantum::ir
    ↓
External P

must preserve every semantic property required by the export contract.

If a property cannot be represented:

unsupported

MUST be reported explicitly.

The exporter MUST NOT silently replace it with an approximate operation unless the approximation is explicitly authorized by the applicable semantic contract.

---

8. Semantic Equivalence

Interoperability MUST reason about semantic equivalence rather than textual equivalence.

These may be textually different:

apply H(q);

and an equivalent representation using another valid external syntax.

They may nevertheless represent equivalent quantum computation.

Conversely, two visually similar operations may have different:

- parameter conventions;
- endian conventions;
- control ordering;
- measurement semantics;
- qubit ordering;
- classical-bit ordering;
- global phase semantics;
- reset semantics;
- channel semantics;
- timing semantics.

Therefore interoperability MUST NOT use textual similarity as the definition of equivalence.

---

9. Canonical Semantic Mapping

The canonical mapping is:

format source
    ↓
format AST
    ↓
format semantic model
    ↓
Zamani semantic model
    ↓
quantum::ir

The mapping MUST preserve, where representable:

- operation identity;
- namespace;
- operation parameters;
- operation operands;
- operation results;
- controls;
- adjoints;
- measurement;
- reset;
- classical dependencies;
- classical feed-forward;
- state preparation;
- observables;
- channels;
- noise intent;
- timing intent;
- resource requirements;
- capability requirements;
- attributes;
- source provenance.

---

10. Format-Specific ASTs

A format-specific AST MAY exist.

For example:

OpenQASM source
    ↓
OpenQASM AST

is valid.

However:

OpenQASM AST
    ↓
OpenQASM IR
    ↓
Zamani Quantum IR

must not become a second long-lived semantic hierarchy unless the intermediate representation is strictly an implementation detail of the importer.

The public semantic boundary remains:

quantum::ir

---

11. OpenQASM Integration

OpenQASM is an external interoperability format.

The current implementation boundary is:

src/quantum/frontend/formats/openqasm/

The public facade is:

src/quantum/frontend/formats/openqasm/mod.rs

The public boundary currently exposes:

OpenQasmImporter
OpenQasmExporter
ValidationConfig
OPENQASM_FORMAT_ID
OPENQASM_MEDIA_TYPE
OPENQASM_3_0
OPENQASM_3_1
STANDARD_LIBRARY_INCLUDE

The facade MUST remain a public API boundary rather than becoming an implementation container.

---

12. OpenQASM Module Ownership

The OpenQASM implementation is divided conceptually as:

openqasm/
├── mod.rs
├── ast.rs
├── lexer.rs
├── parser.rs
├── validation.rs
├── stdgates.rs
├── importer.rs
└── exporter.rs

Ownership:

mod.rs
    public facade

ast.rs
    OpenQASM source representation

lexer.rs
    OpenQASM lexical analysis

parser.rs
    OpenQASM parsing

validation.rs
    OpenQASM semantic validation

stdgates.rs
    OpenQASM standard-library definitions

importer.rs
    OpenQASM → quantum::ir

exporter.rs
    quantum::ir → OpenQASM

No module may become a second canonical Zamani semantic authority.

---

13. OpenQASM Versioning

The currently supported production versions are:

OpenQASM 3.0
OpenQASM 3.1

The implementation MUST distinguish versions explicitly.

It MUST NOT assume:

major == 3

means every future OpenQASM 3.x version is automatically supported.

For example:

OpenQASM 3.2

must not be silently accepted until its compatibility contract is implemented.

Version selection MUST be explicit.

---

14. OpenQASM Standard Library

The standard include:

include "stdgates.inc";

is an external-format construct.

Its interpretation belongs to the OpenQASM frontend.

It MUST NOT cause the Zamani quantum grammar to become a closed list of standard gates.

For example, the existence of:

stdgates.inc

does not justify:

quantumGate
    : H
    | X
    | Y
    | Z
    | ...

as Zamani's fundamental operation model.

---

15. OpenQASM Gate Mapping

OpenQASM standard operations may be mapped into the canonical quantum operation model.

Conceptually:

OpenQASM operation
        ↓
operation identity
        ↓
parameters
        ↓
operands
        ↓
modifiers
        ↓
canonical quantum operation
        ↓
quantum::ir

The operation name remains data.

Standard-library membership is semantic metadata.

It is not a reason to hard-code every gate into "grammar/quantum/operations.g4".

---

16. QIR Integration

QIR is an interoperability/lowering representation, not a replacement for the Zamani quantum semantic model.

The architecture is:

Zamani
   ↓
semantic analysis
   ↓
quantum::ir
   ↓
QIR lowering
   ↓
QIR

or for supported import:

QIR
   ↓
QIR validation
   ↓
semantic reconstruction
   ↓
quantum::ir

QIR-specific details MUST remain inside the QIR interoperability implementation.

QIR concepts MUST NOT be copied wholesale into the Zamani grammar merely because they exist in QIR.

---

17. QIR Is Not Zamani::quantum::ir

The following distinction is mandatory:

quantum::ir
    =
Zamani's canonical quantum semantic/IR boundary

while:

QIR
    =
external/interoperability representation

The compiler MAY lower:

quantum::ir → QIR

but MUST NOT redefine:

quantum::ir == QIR

This preserves Zamani's target-independent semantic model.

---

18. Quil Integration

Quil is another external representation.

The architecture is:

Quil
   ↓
Quil lexer/parser
   ↓
Quil validation
   ↓
semantic mapping
   ↓
quantum::ir

and:

quantum::ir
   ↓
representability validation
   ↓
Quil exporter
   ↓
Quil

Quil-specific syntax, instruction encoding, and semantics MUST remain within the Quil interoperability implementation.

Quil syntax MUST NOT be added to the core Zamani quantum grammar merely for convenience.

---

19. Vendor-Specific Formats

Vendor-specific formats MAY be supported.

Examples include vendor-specific:

- gate sets;
- instruction formats;
- calibration formats;
- pulse formats;
- scheduling formats;
- device descriptions;
- execution metadata.

However:

vendor format
    ≠
Zamani language

A vendor format MUST be isolated behind an interoperability boundary.

Vendor-specific semantics MUST NOT become universal Zamani syntax.

---

20. Vendor Namespace Model

Vendor operations may be represented through qualified semantic identities such as:

vendor::operation

or:

provider::operation

where the normal Zamani namespace system permits this.

The grammar must not need to know every vendor in existence.

Adding a vendor MUST NOT require modifying the fundamental quantum operation grammar merely to recognize its operation names.

---

21. Future Quantum Formats

A future quantum format MUST be addable without modifying the existing semantics of unrelated formats.

The desired architecture is:

frontend/
└── formats/
    ├── openqasm/
    ├── qir/
    ├── quil/
    ├── future_format/
    └── ...

Each format owns:

- format parser;
- format AST;
- format validation;
- importer;
- exporter;
- format tests;
- format compatibility;
- format diagnostics.

The canonical quantum semantic boundary remains unchanged.

---

22. Generic Format Contract

Every production quantum interoperability format SHOULD satisfy the following conceptual contract:

FormatImporter
    import(input, configuration)
        →
        validated canonical quantum representation

FormatExporter
    export(quantum::ir, configuration)
        →
        validated external representation

The exact Rust traits/types are owned by the existing generic frontend implementation.

This document does not redefine them.

---

23. Import Contract

Every importer MUST:

1. accept only the declared input format;
2. identify the format version;
3. validate syntax;
4. validate format semantics;
5. validate required declarations;
6. preserve source provenance;
7. validate representability in Zamani;
8. lower to canonical "quantum::ir";
9. preserve relevant metadata;
10. report unsupported constructs;
11. avoid silent semantic loss;
12. avoid hardware execution;
13. avoid network access unless explicitly delegated to a higher-level resolver;
14. avoid filesystem access unless explicitly delegated to a higher-level resolver;
15. avoid process execution;
16. remain deterministic for identical input and configuration.

---

24. Export Contract

Every exporter MUST:

1. accept canonical quantum semantics;
2. validate representability;
3. identify the requested target format/version;
4. preserve operation meaning;
5. preserve supported parameters;
6. preserve supported controls;
7. preserve supported measurements;
8. preserve supported classical dependencies;
9. preserve required metadata;
10. reject unsupported semantics explicitly;
11. avoid silently dropping operations;
12. avoid silently changing semantics;
13. avoid mutating canonical "quantum::ir";
14. emit deterministic output for identical input/configuration.

---

25. Import/Export Must Be Side-Effect Free

Parsing and conversion MUST NOT execute a quantum program.

Importing:

OpenQASM
QIR
Quil
vendor format

must not:

- submit a QPU job;
- allocate a QPU;
- contact a provider;
- start a simulator;
- access calibration services;
- modify hardware;
- launch a process;
- execute arbitrary external code.

Interoperability is a compilation/data boundary.

Execution belongs downstream.

---

26. Include Resolution

External formats may contain constructs such as:

include "stdgates.inc";

or equivalent import/include mechanisms.

The interoperability frontend MUST treat include declarations as source-level data.

An include MUST NOT automatically grant arbitrary:

- filesystem access;
- network access;
- process execution;
- dynamic code loading.

Resolution must occur through an explicit resolver/policy owned by the appropriate higher-level frontend layer.

---

27. Security Boundary

External quantum input is untrusted input.

The interoperability layer MUST treat it accordingly.

It MUST NOT execute:

- embedded code;
- calibration programs;
- arbitrary host-language expressions;
- external commands;
- dynamic libraries;
- network instructions;
- vendor scripts.

A format may contain constructs that appear executable.

They remain data until an explicitly authorized downstream execution subsystem interprets them.

---

28. No Unsafe Rust

All interoperability implementation code MUST be compatible with:

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

The implementation must use safe Rust.

No interoperability requirement may justify "unsafe".

This includes:

- parsers;
- validators;
- importers;
- exporters;
- format registries;
- serialization;
- source mapping;
- diagnostics;
- conversion tables.

---

29. Memory Scalability

Interoperability MUST NOT establish fixed universal memory limits.

It MUST NOT define:

MAX_QASM_SIZE
MAX_QIR_SIZE
MAX_QUIL_SIZE
MAX_OPERATION_COUNT
MAX_QUBIT_COUNT
MAX_REGISTER_COUNT
MAX_PARAMETER_COUNT
MAX_INCLUDE_COUNT

as language-level constants.

Actual resource bounds MAY exist in implementation configuration.

Such bounds are runtime/compiler resource policies and MUST NOT redefine language semantics.

---

30. Unbounded Semantic Scale

The interoperability architecture must remain capable of handling programs whose size grows according to available resources.

This includes:

- tiny circuits;
- large circuits;
- symbolic circuits;
- parameterized circuits;
- dynamically sized registers;
- distributed quantum programs;
- very deep circuits;
- very wide circuits;
- hybrid quantum/classical workloads;
- future computational substrates.

The principle is:

language scale
    ≠
implementation resource capacity

The grammar must not establish artificial ceilings.

---

31. Qubit Ordering

Interoperability implementations MUST explicitly account for qubit ordering.

Different formats may use different conventions for:

- declaration ordering;
- register indexing;
- bit significance;
- tensor ordering;
- measurement ordering;
- physical identifiers.

An importer MUST normalize ordering into the canonical Zamani representation.

An exporter MUST convert ordering explicitly to the target format's convention.

No implicit assumption may be made that all external formats share the same ordering.

---

32. Classical Bit Ordering

The same requirement applies to classical bits.

The importer/exporter MUST explicitly handle:

- register ordering;
- bit numbering;
- measurement destination ordering;
- endian conventions;
- packed values;
- classical expressions.

A conversion that changes classical interpretation MUST be rejected unless the transformation is explicitly proven equivalent.

---

33. Measurement Semantics

Measurement is not merely an operation name.

Interoperability MUST preserve, where representable:

- measured quantum operands;
- measurement basis;
- measurement result;
- classical destination;
- measurement timing;
- mid-circuit status;
- conditional dependencies;
- ordering;
- repeated-shot semantics where applicable.

The exporter MUST NOT invent measurement destinations.

The importer MUST NOT discard them.

---

34. Mid-Circuit Measurement

External formats may support mid-circuit measurement.

When supported by Zamani semantics:

measurement
    ↓
classical result
    ↓
classical computation
    ↓
quantum control

must remain semantically connected.

An importer MUST NOT reduce:

measure
if result ...
apply ...

to an unrelated sequence that loses the dependency.

---

35. Classical Feed-Forward

Classical feed-forward must remain explicit.

The semantic dependency:

quantum operation
      ↓
measurement
      ↓
classical value
      ↓
classical condition
      ↓
quantum operation

must survive interoperability conversion when representable.

The relevant Zamani grammar boundaries include:

grammar/quantum/classical-feedforward.g4
grammar/quantum/dynamic-control.g4

and the universal expression/control-flow grammar.

The interoperability layer consumes their resulting semantics; it does not redefine them.

---

36. Dynamic Circuits

Dynamic circuit behavior may include:

- measurement-dependent control;
- runtime conditions;
- reset;
- dynamic allocation;
- dynamic operation selection;
- classical feed-forward.

Interoperability MUST distinguish:

static circuit structure

from:

runtime-dependent circuit behavior

An exporter MUST reject a dynamic program when the target format cannot represent its required dynamic semantics.

It MUST NOT silently flatten the program into an inequivalent static circuit.

---

37. Parameters

External operation parameters must map into Zamani's canonical parameter semantics.

Parameters may be:

- constants;
- symbolic values;
- expressions;
- runtime values;
- compile-time values;
- generic values;
- dependent values.

The importer MUST preserve symbolic parameters when Zamani can represent them.

It MUST NOT prematurely evaluate a symbolic parameter merely because a host-language representation happens to use a numeric type.

---

38. Floating-Point and Numeric Semantics

Interoperability must explicitly distinguish:

exact symbolic value

from:

finite numeric approximation

An exporter MUST NOT silently convert exact symbolic semantics into an approximation unless the export contract permits that conversion.

If conversion changes required semantics, the export must fail explicitly.

---

39. Global Phase

Global phase MUST be handled according to the semantic equivalence rules of the source and target formats.

An importer MUST determine whether the source format treats global phase as observable.

An exporter MUST not discard global phase when the target contract makes it semantically relevant.

If a target cannot represent required phase information, export MUST report an explicit incompatibility.

---

40. Custom Operations

Interoperability MUST support open-world operation identity.

A source operation may be:

standard
custom
library-defined
dialect-defined
vendor-defined
future-defined

A format importer MUST preserve an operation identity when it cannot map the operation to a known standard operation but the canonical IR can represent the operation.

If the canonical IR cannot represent the required semantics, the importer MUST return a structured unsupported-feature diagnostic.

It must not silently replace the operation with another operation.

---

41. Operation Decomposition

Importers MUST NOT unnecessarily decompose operations.

For example:

custom_operation

should remain a semantic operation if "quantum::ir" can represent it.

Decomposition belongs to:

- optimization;
- synthesis;
- target lowering;
- routing;
- backend compilation.

Interoperability should preserve source semantics first.

---

42. Operation Names Are Not Hardware Instructions

An external operation such as:

vendor::operation

does not automatically mean:

physical hardware instruction

The distinction is:

operation identity
        ↓
semantic interpretation
        ↓
target capability analysis
        ↓
lowering
        ↓
hardware realization

This is essential for POCO-REAF.

---

43. Controls and Adjoints

Interoperability MUST preserve operation modifiers where the canonical representation supports them.

Examples include:

controlled
adjoint
inverse
power
repeat
conditional

The importer MUST NOT unnecessarily lower:

controlled(operation)

into an implementation-specific gate sequence merely because a target backend may eventually require one.

Such lowering belongs downstream.

---

44. Resource Requirements

External formats may contain resource assumptions.

These must be translated into the universal Zamani resource/capability model.

For example:

requires qubits >= n;
requires capability("quantum.measurement");
requires capability("quantum.mid_circuit_measurement");

are semantic requirements.

They are not device selections.

Interoperability MUST NOT convert them into:

use QPU 0
use physical qubit 17
use device X

unless explicit target-specific semantics require that transformation downstream.

---

45. Capability Requirements

Capabilities must remain separate from resources.

For example:

requires capability("quantum.dynamic_control");

means the target environment must provide the capability.

It does not mean:

use vendor X

or:

use hardware Y

Capability resolution belongs to semantic/resource/target analysis.

---

46. Physical Qubit Mapping

Interoperability MUST preserve the distinction between:

logical qubit

and:

physical qubit

If an external format contains physical mapping information, the importer must classify it correctly.

It MUST NOT automatically turn physical IDs into source-level logical qubit identities.

The canonical distinction remains:

logical identity
        ↓
mapping
        ↓
physical identity

Mapping belongs downstream unless the source explicitly expresses physical intent.

---

47. Topology

External topology information MUST remain target metadata or explicit target intent.

It MUST NOT become a universal language-level topology assumption.

For example:

coupling map

is not automatically a Zamani language requirement.

It may become:

requires topology(...)

when the source semantics explicitly require a topology property.

Routing remains a downstream concern.

---

48. Scheduling and Timing

Interoperability MUST preserve timing information where the canonical semantic model supports it.

Timing MAY include:

- operation duration;
- ordering constraints;
- synchronization;
- barriers;
- temporal dependencies;
- pulse timing;
- latency requirements.

Timing information must not be confused with a universal machine clock.

Scheduling remains downstream.

---

49. Pulse-Level Interoperability

Pulse-level formats may describe:

- pulses;
- waveforms;
- channels;
- timing;
- amplitudes;
- frequencies;
- calibration;
- control electronics.

Such data MUST remain outside the general high-level quantum operation grammar unless explicitly represented as abstract pulse intent.

The interoperability layer must distinguish:

pulse intent

from:

device calibration

and:

physical control implementation

---

50. Calibration Data

Calibration information is target-specific.

Interoperability MUST NOT promote calibration values into universal Zamani language semantics.

Calibration may be imported/exported as:

- target metadata;
- deployment metadata;
- backend-specific configuration;
- explicit pulse-level information.

It must not alter the meaning of ordinary Zamani quantum operations.

---

51. Noise and Channels

External representations may describe:

- noise channels;
- decoherence;
- errors;
- stochastic behavior;
- density-matrix operations;
- error models.

These must be mapped into the canonical semantic model only when representable.

The interoperability layer does not implement:

- noise simulation;
- QEC;
- decoding;
- fault diagnosis;
- calibration.

Those belong to the downstream quantum toolchain.

---

52. QEC Interoperability

External representations may contain fault-tolerance or QEC information.

The importer MUST preserve supported QEC intent.

It MUST NOT implement QEC itself.

The architecture remains:

external QEC information
        ↓
semantic QEC intent
        ↓
quantum::ir / semantic metadata
        ↓
QEC subsystem

The QEC subsystem owns:

- code selection;
- syndrome extraction;
- decoding;
- correction;
- logical-to-physical overhead;
- fault-tolerance realization.

---

53. ZQN Integration

Noise/fault semantics that are represented through ZQN remain downstream semantic/runtime concerns.

Interoperability MAY preserve ZQN-relevant metadata.

It MUST NOT create another ZQN implementation.

The boundary remains:

interoperability
        ↓
canonical semantic information
        ↓
ZQN

---

54. Routing

Interoperability MUST NOT perform target routing as part of basic import.

The preferred pipeline is:

external format
        ↓
canonical quantum::ir
        ↓
routing
        ↓
target realization

This allows the same imported computation to be retargeted to different hardware.

---

55. Scheduling

Scheduling MUST remain outside the interoperability boundary unless an external format explicitly contains scheduling metadata that must be preserved.

Even then:

external schedule
        ↓
validated scheduling metadata

must not be confused with:

Zamani scheduler implementation

---

56. Hardware Abstraction

External hardware formats may contain:

- device properties;
- topology;
- capabilities;
- memory;
- timing;
- connectivity;
- execution modes.

The importer/exporter MUST keep those properties separate from the portable program.

The architecture is:

portable quantum program
        ↓
requirements/capabilities
        ↓
hardware discovery
        ↓
target matching
        ↓
mapping/routing
        ↓
scheduling
        ↓
execution

---

57. POCO-REAF

Interoperability must preserve the Zamani principle:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

An external format conversion must not unnecessarily specialize the program to:

- one QPU;
- one vendor;
- one topology;
- one qubit count;
- one simulator;
- one compiler;
- one instruction set.

The canonical semantic representation must retain as much target-independent information as possible.

---

58. No Hard-Coded Hardware Limits

Interoperability code MUST NOT introduce universal constants such as:

MAX_QUBITS
MAX_PHYSICAL_QUBITS
MAX_LOGICAL_QUBITS
MAX_QPUS
MAX_DEVICES
MAX_OPERATIONS
MAX_CIRCUIT_DEPTH
MAX_CIRCUIT_WIDTH
MAX_REGISTER_WIDTH
MAX_PARAMETERS
MAX_MEASUREMENTS
MAX_SHOTS
MAX_MEMORY
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

as language semantics.

Implementation resource limits may exist as explicit configuration.

They MUST NOT be represented as Zamani grammar limitations.

---

59. Program Numbers Versus Implementation Limits

A source program may legitimately contain:

1000

or:

n

or:

problem_size

as program data.

For example:

requires qubits >= n;

is valid semantic intent.

This does not establish:

MAX_QUBITS = n

or any fixed compiler ceiling.

The distinction must be preserved throughout import/export.

---

60. Arbitrary Scale

The interoperability architecture must support:

1 qubit
2 qubits
n qubits
symbolically sized systems
large systems
distributed systems
future systems

without requiring grammar changes.

Scaling is determined by:

source semantics
+
compiler capabilities
+
available resources
+
target capabilities

not by a fixed grammar constant.

---

61. Determinism

For identical:

input
format version
configuration

the importer MUST produce deterministic semantic output.

For identical:

quantum::ir
format version
configuration

the exporter MUST produce deterministic output.

Determinism must not depend on:

- hash-map iteration order;
- thread scheduling;
- nondeterministic global state;
- wall-clock time;
- random values;
- target discovery;
- network ordering.

If nondeterminism is semantically required, it must be represented explicitly rather than accidentally introduced by the interoperability implementation.

---

62. Source Provenance

Imported constructs MUST retain source provenance when the frontend architecture supports source spans.

At minimum, diagnostics should be capable of identifying:

- source file;
- format;
- version;
- source range;
- construct;
- conversion stage;
- semantic reason for failure.

When a semantic construct originates from an external format, that provenance should remain available through the frontend conversion boundary.

---

63. Diagnostics

Interoperability errors MUST be structured.

The implementation MUST NOT require users or downstream tools to parse human-readable strings.

Diagnostics should distinguish categories such as:

InvalidSyntax
UnsupportedVersion
UnsupportedFeature
UnsupportedOperation
UnsupportedParameter
UnsupportedControl
UnsupportedMeasurement
UnsupportedDynamicBehavior
UnsupportedType
UnsupportedResource
UnsupportedCapability
SemanticMismatch
RepresentationMismatch
LossyConversion
InvalidMapping
InvalidOrdering
InvalidClassicalDependency
InvalidInclude
SecurityViolation
ResourceLimit
ConfigurationError

The exact Rust error taxonomy belongs to the repository's canonical frontend error system.

---

64. Unsupported Features

An unsupported feature must produce an explicit diagnostic.

For example:

external operation
    ↓
not representable by quantum::ir
    ↓
UnsupportedFeature

The implementation MUST NOT:

drop it
replace it silently
comment it out
replace it with identity
replace it with an approximate operation
ignore it

---

65. Lossy Conversion

Lossy conversion is prohibited by default.

If a specific interoperability contract intentionally permits loss, it must be explicit.

The caller must be able to distinguish:

lossless conversion

from:

lossy conversion

Lossy conversion MUST NOT be silently treated as exact semantic conversion.

---

66. Round-Trip Property

Where a format supports a lossless representation of a canonical quantum program:

quantum::ir
    ↓
export
    ↓
external format
    ↓
import
    ↓
quantum::ir

must preserve canonical semantic equivalence.

Likewise, where the external format is fully representable:

external
    ↓
import
    ↓
quantum::ir
    ↓
export
    ↓
external

must preserve the format's specified semantics.

Byte-for-byte equality is not required unless explicitly specified by the format contract.

---

67. Canonicalization

Exporters MAY canonicalize output.

Canonicalization may include:

- deterministic declaration ordering where semantically irrelevant;
- normalized formatting;
- normalized numeric formatting;
- normalized namespace representation;
- stable metadata ordering.

Canonicalization MUST NOT alter semantics.

---

68. Metadata

Metadata must be classified.

Metadata may be:

semantic
diagnostic
provenance
optimization
target
deployment
vendor-specific
format-specific

Semantic metadata that affects program meaning must not be silently discarded.

Purely informational metadata may be discarded when the target format cannot represent it, provided doing so does not violate the source contract.

---

69. Vendor Metadata

Vendor metadata must be namespaced or otherwise isolated.

For example:

vendor::property

must not become a universal Zamani property simply because one importer recognizes it.

Vendor metadata must not create hidden dependencies on a particular backend.

---

70. Dialect Integration

Quantum dialects may extend interoperability.

The dialect architecture must remain:

Zamani
   ↓
dialect declaration
   ↓
explicit syntax extension
   ↓
semantic extension
   ↓
AST mapping
   ↓
IR mapping

A dialect MUST declare:

- identity;
- version;
- namespace;
- owner;
- syntax extension;
- semantic extension;
- AST mapping;
- IR mapping;
- compatibility;
- feature status.

A dialect must not silently modify standard quantum semantics.

---

71. External Format Registration

Adding a format must be an additive operation.

For example:

OpenQASM
QIR
Quil
FutureFormat

must each implement the generic interoperability contract.

Adding "FutureFormat" must not require modifying:

OpenQASM parser
OpenQASM AST
OpenQASM importer
OpenQASM exporter

merely to register the new format.

Registration belongs to the generic frontend format layer.

---

72. Interoperability and Grammar Separation

"grammar/quantum/interoperability.md" is a specification.

It does not define parser productions.

External format grammar belongs to the corresponding format implementation.

For example:

OpenQASM grammar

belongs under:

src/quantum/frontend/formats/openqasm/

or its explicitly designated grammar source.

It MUST NOT be copied into:

grammar/quantum/

as if OpenQASM syntax were Zamani syntax.

---

73. Zamani Grammar Separation

The Zamani quantum grammar remains responsible for Zamani source syntax:

grammar/quantum/*.g4

Interoperability files describe the boundaries between that language and external representations.

Therefore:

Zamani syntax
    ≠
OpenQASM syntax
    ≠
QIR syntax
    ≠
Quil syntax

All may map to:

quantum::ir

where semantically representable.

---

74. Integration with "operations.g4"

"grammar/quantum/operations.g4" owns generic source-level operation invocation.

Interoperability MUST map external operations into the same semantic operation model.

It must not require "operations.g4" to enumerate every operation found in external standards.

The desired relationship is:

Zamani operation syntax ──────┐
                              │
OpenQASM operation ───────────┤
                              │
QIR operation ────────────────┤
                              ▼
                    canonical operation semantics
                              │
                              ▼
                         quantum::ir

---

75. Integration with Quantum Types

External format types/registers must map into the canonical Zamani quantum type model.

Relevant ownership remains with:

grammar/quantum/types.g4
grammar/quantum/registers.g4
grammar/types/

Interoperability must not create a second quantum type system.

External types are interpreted and mapped.

---

76. Integration with Measurement

Measurement semantics must integrate with:

grammar/quantum/measurement.g4
grammar/quantum/classical-feedforward.g4
grammar/quantum/dynamic-control.g4

The interoperability layer must preserve measurement dependencies.

It must not create a separate measurement semantic model.

---

77. Integration with Resources

Resource requirements must integrate with:

grammar/resources/
grammar/quantum/resource-requirements.g4
grammar/quantum/quantum-capabilities.g4

External resource information must become canonical resource/capability semantics where supported.

It must not create:

OpenQASMResourceModel
VendorResourceModel
QIRResourceModel

as competing universal resource authorities.

---

78. Integration with Hardware

Hardware realization remains owned by:

grammar/hardware/

and downstream hardware/compiler subsystems.

Interoperability may preserve hardware metadata.

It must not decide:

- QPU selection;
- physical qubit allocation;
- topology routing;
- calibration;
- scheduling;
- target deployment.

---

79. Integration with Routing

The conversion pipeline is:

external format
    ↓
canonical quantum::ir
    ↓
routing

not:

external format
    ↓
vendor routing
    ↓
Zamani quantum

unless the external format itself explicitly represents already-routed physical intent.

Even then, the distinction between:

source intent

and:

target realization

must remain explicit.

---

80. Integration with Scheduling

The conversion pipeline is:

external format
    ↓
canonical quantum::ir
    ↓
scheduling

where possible.

External schedules may be preserved as metadata when required, but must not become universal scheduler semantics.

---

81. Integration with QEC

The conversion pipeline is:

external QEC intent
    ↓
canonical semantic information
    ↓
QEC subsystem

Interoperability does not perform error correction.

---

82. Integration with ZQN

The conversion pipeline is:

external noise/fault information
    ↓
canonical semantic information
    ↓
ZQN

Interoperability does not perform noise analysis or fault modelling itself beyond validating the source format's semantics.

---

83. Integration with HAL

HAL is the target-facing boundary.

Interoperability MUST NOT directly call HAL merely to parse or convert a file.

The intended pipeline is:

external format
    ↓
frontend
    ↓
semantic analysis
    ↓
quantum::ir
    ↓
compiler
    ↓
HAL

This keeps conversion independent of hardware availability.

---

84. Simulator Independence

Import/export must not assume a particular simulator.

A circuit imported from an external format must remain usable for:

CPU simulation
GPU simulation
distributed simulation
tensor-network simulation
state-vector simulation
future simulation model

where the canonical semantics permit it.

---

85. QPU Independence

Likewise, imported programs must remain target-independent until target realization.

An importer must not implicitly select:

QPU 0
QPU 1
vendor X
device Y

unless the external program explicitly contains target-specific semantics and the caller requests preservation of those semantics.

---

86. Execution Independence

Interoperability conversion must never imply execution.

These are separate operations:

import
compile
deploy
execute

A successful import means only:

the external representation was successfully interpreted
and converted according to the contract

It does not mean:

the quantum program was executed

---

87. Interoperability With Hybrid Programs

External quantum representations may interact with classical computation.

The canonical architecture is:

classical computation
        ↓
quantum operation
        ↓
measurement
        ↓
classical result
        ↓
classical computation
        ↓
quantum operation

Interoperability must preserve this boundary when the external representation supports it.

Classical semantics remain owned by the universal Zamani language and classical semantic layers.

---

88. Host/Device Separation

External formats may distinguish host and device computation.

This distinction must map to semantic execution intent rather than being hard-coded as a particular:

CPU
GPU
QPU

The compiler determines actual realization.

---

89. Distributed Quantum Interoperability

Future interoperability formats may represent distributed quantum systems.

They may include:

- multiple quantum processors;
- quantum communication;
- distributed state;
- entanglement links;
- remote operations;
- distributed classical control.

The interoperability layer must preserve semantic intent where representable.

It must not impose a fixed number of:

QPU
nodes
links
devices
processes

---

90. Networking

Network addresses and communication details are target/environment information.

They must not become universal quantum grammar constraints.

If an external format contains networking information, it must be classified as:

semantic
deployment
target
vendor
metadata

before mapping.

---

91. Security

Interoperability must maintain security boundaries across formats.

An external format must not be allowed to smuggle:

- executable host code;
- arbitrary commands;
- filesystem traversal;
- network requests;
- credentials;
- secret material;
- dynamic libraries.

Cryptographic metadata may be preserved where the canonical security model supports it.

---

92. Serialization

Serialization must be deterministic where the external format permits deterministic representation.

Serialization MUST NOT:

- reorder semantic operations;
- remove required metadata;
- change numeric meaning;
- change qubit identity;
- change classical dependencies.

---

93. Configuration

Format-specific configuration must remain separate from program semantics.

For example:

OpenQASM version
export formatting
validation policy
include policy
dialect policy
compatibility policy

may be configuration.

They must not silently modify the semantic meaning of Zamani source.

---

94. Resource Limits in Implementations

An implementation MAY impose operational limits to prevent denial-of-service or resource exhaustion.

For example, an importer may have a configurable:

input byte budget
AST node budget
recursion budget
diagnostic budget
conversion budget
memory budget

These are implementation safety/resource controls.

They MUST NOT be represented as:

language maximum
quantum maximum
grammar maximum

and MUST be explicit in the relevant implementation configuration.

---

95. No Hidden Limits

Hidden limits are prohibited.

A limit must be:

1. documented;
2. configurable where appropriate;
3. represented by a structured error;
4. distinguishable from semantic invalidity;
5. absent from the language specification;
6. absent from the canonical grammar as a universal ceiling.

---

96. Error Classification

The interoperability implementation must distinguish:

invalid source

from:

valid source but unsupported feature

and:

valid source and supported feature but insufficient implementation resources

These are different conditions.

For example:

invalid OpenQASM

is not equivalent to:

valid OpenQASM requiring a capability unavailable on the current target

---

97. Compatibility

Interoperability compatibility must be version-aware.

The relevant repository compatibility surfaces include:

grammar/compatibility/
grammar/spec/compatibility.md

A format version change must be classified as:

compatible
conditionally compatible
breaking
unsupported

according to the applicable format specification and Zamani conversion contract.

---

98. Zamani Compatibility

A new interoperability feature must not silently alter existing Zamani semantics.

For example, adding support for a new OpenQASM operation must not change the meaning of an existing Zamani operation with the same textual name.

Format namespaces and semantic registries must remain distinct.

---

99. Backward Compatibility

Existing supported external formats should remain importable/exportable according to their declared compatibility contract.

If support is removed:

1. it must be documented;
2. the affected version must be identified;
3. diagnostics must be structured;
4. migration guidance must exist;
5. compatibility tests must cover the change.

---

100. Forward Compatibility

Unknown future format versions must not be silently treated as known versions.

For example:

OpenQASM 3.2

must not automatically be interpreted as:

OpenQASM 3.1

unless the format contract explicitly guarantees compatibility.

---

101. Feature Compatibility Matrix

Every interoperability feature should be traceable across:

External format
      ↓
Format parser
      ↓
Format AST
      ↓
Format validation
      ↓
Zamani semantic model
      ↓
quantum::ir
      ↓
Optimizer
      ↓
Router
      ↓
Scheduler
      ↓
QEC
      ↓
ZQN
      ↓
HAL
      ↓
Target

A feature is production-complete only when all applicable stages are identified.

---

102. Feature-Level Completion Contract

Every new interoperability feature must document:

Feature identity
Format
Format version
Source syntax
Format AST
Format semantic meaning
Zamani semantic mapping
Canonical quantum::ir mapping
Import behavior
Export behavior
Losslessness
Unsupported behavior
Diagnostics
Source spans
Resource implications
Capability implications
Security implications
Determinism
Scalability
Compatibility
Positive tests
Negative tests
Boundary tests
Round-trip tests
Hard-coding audit
Completion criteria

This is what allows the feature to be completed independently without reopening it merely because another file later changes.

---

103. Independent-First Integration Rule

A format implementation should be independently completable in this order:

1. Format contract
2. Version contract
3. Lexical contract
4. AST contract
5. Semantic contract
6. Quantum::IR mapping
7. Import contract
8. Export contract
9. Error contract
10. Security contract
11. Determinism contract
12. Scalability contract
13. Compatibility contract
14. Tests
15. Public frontend registration

The format implementation must specify all downstream integration points before implementation is declared complete.

---

104. "interoperability.md" Ownership

This file OWNS:

- quantum interoperability architecture;
- external-format boundaries;
- import/export semantic rules;
- canonical mapping requirements;
- lossless conversion policy;
- external-format compatibility;
- format version policy;
- interoperability security;
- interoperability determinism;
- interoperability scalability;
- cross-format integration rules.

This file DOES NOT OWN:

- Zamani grammar productions;
- lexer implementation;
- frontend AST implementation;
- canonical quantum IR implementation;
- QEC implementation;
- ZQN implementation;
- routing implementation;
- scheduling implementation;
- HAL implementation;
- QPU drivers;
- vendor SDKs;
- simulator implementation.

---

105. Integration With "grammar/quantum/README.md"

"grammar/quantum/README.md" owns the overall quantum grammar architecture.

This document specializes that architecture for interoperability.

Therefore:

README.md
    ↓
overall quantum grammar architecture

interoperability.md
    ↓
external-format boundary

Neither should redefine the other's ownership.

---

106. Integration With "grammar/spec/quantum.md"

"grammar/spec/quantum.md" remains the normative quantum-language specification.

This document adds the external interoperability contract.

The relationship is:

spec/quantum.md
    ↓
what Zamani quantum means

interoperability.md
    ↓
how external representations map to/from that meaning

If an external standard conflicts with Zamani semantics, the Zamani specification remains authoritative for Zamani.

---

107. Integration With "grammar/Zamani.g4"

"grammar/Zamani.g4" remains the canonical Zamani composition root.

It must not import OpenQASM, QIR, Quil, or vendor grammars as if those formats were Zamani syntax.

Interoperability happens at the format frontend boundary.

---

108. Integration With "grammar/quantum/quantum.g4"

"quantum.g4" owns quantum-domain composition.

It does not own external format syntax.

The interoperability contract consumes the semantic result of the Zamani quantum grammar and maps external formats into that same semantic boundary.

---

109. Integration With "grammar/quantum/operations.g4"

"operations.g4" must remain open-ended.

Interoperability must map external operation names into its generic semantic operation model.

No external standard should cause a closed gate enumeration to be introduced.

---

110. Integration With "grammar/quantum/resource-requirements.g4"

External resource declarations must map into the universal resource semantics.

The existing distinction remains:

requirement
constraint
preference
hint
capability

Interoperability must preserve those distinctions.

---

111. Integration With "grammar/quantum/quantum-capabilities.g4"

External capability requirements must map to the canonical capability model.

For example:

quantum.mid_circuit_measurement
quantum.dynamic_control
quantum.measurement

remain capability identities.

They do not select a vendor or device.

---

112. Integration With "src/quantum/frontend/formats/openqasm/mod.rs"

The current OpenQASM facade already establishes the desired public boundary.

This document therefore requires:

mod.rs
    ↓
public facade

while keeping:

ast.rs
lexer.rs
parser.rs
validation.rs
stdgates.rs
importer.rs
exporter.rs

implementation-local.

The public facade must not expose unnecessary parser internals.

---

113. Integration With "OpenQasmImporter"

The importer must implement:

external OpenQASM
    ↓
validated OpenQASM representation
    ↓
canonical quantum::ir

It must not:

OpenQASM
    ↓
execute

or:

OpenQASM
    ↓
hardware mapping

or:

OpenQASM
    ↓
QPU submission

---

114. Integration With "OpenQasmExporter"

The exporter must implement:

quantum::ir
    ↓
representability validation
    ↓
OpenQASM

It must not modify canonical IR.

Unsupported operations must generate structured diagnostics.

---

115. Integration With Other External Formats

Future format modules should follow:

src/quantum/frontend/formats/<format>/

where that is the established frontend convention.

Each module should independently contain:

mod.rs
ast.rs
lexer.rs
parser.rs
validation.rs
importer.rs
exporter.rs

only where actually required.

Files should not be created merely to satisfy a template.

---

116. No Duplicate Frontend Architectures

The repository must not end up with:

src/quantum/frontend/openqasm.rs

and:

src/quantum/frontend/formats/openqasm/

both acting as independent OpenQASM implementations.

The current repository already has the newer structured:

src/quantum/frontend/formats/openqasm/

boundary.

If an older implementation exists elsewhere, it must be classified as:

canonical
compatibility
migration
obsolete

and only one production implementation may own OpenQASM semantics.

---

117. External Formats Must Not Become Dialects Accidentally

OpenQASM, QIR, Quil, and vendor formats are interoperability formats.

They are not automatically Zamani dialects.

A dialect extends Zamani semantics.

An interoperability format represents another language or representation.

These are different concepts.

---

118. Format-to-Dialect Conversion

A format may optionally be converted into a Zamani dialect representation if explicitly specified.

The pipeline would be:

external format
    ↓
format semantics
    ↓
Zamani dialect
    ↓
canonical semantics
    ↓
quantum::ir

The dialect must not become a hidden substitute for "quantum::ir".

---

119. Semantic Preservation Priority

When converting between representations, preserve in this order:

1. program meaning;
2. quantum state evolution;
3. measurement semantics;
4. classical dependencies;
5. operation parameters;
6. resource/capability intent;
7. timing semantics where required;
8. metadata;
9. formatting.

Formatting is the least important property.

Semantic preservation is the primary property.

---

120. Interoperability and Optimization

Importers MUST preserve source semantics before optimization.

Optimization occurs after canonicalization:

external
    ↓
quantum::ir
    ↓
optimization

Exporters may serialize an optimized canonical representation.

They must not silently optimize away semantics that the target format requires.

---

121. Interoperability and Decomposition

Decomposition is a compiler concern.

The importer should preserve:

operation A

rather than automatically producing:

operation B
operation C
operation D

unless decomposition is required to make the operation representable in the canonical IR.

Target decomposition belongs downstream.

---

122. Interoperability and Routing

Routing must remain target-specific.

A portable imported program should remain usable on multiple topologies.

Therefore:

external → quantum::ir

must precede:

routing

whenever possible.

---

123. Interoperability and Scheduling

Scheduling must remain target/runtime specific.

An external representation may contain schedule information, but that information must be explicitly classified and preserved only when semantically relevant.

---

124. Interoperability and Resilience

Resilience policies must remain separate from basic format conversion.

An interoperability implementation may preserve:

reliability requirement
fault-tolerance requirement
noise metadata
error model

but does not itself perform resilience analysis.

The downstream resilience subsystem owns that work.

---

125. Interoperability and Execution Outcomes

Interoperability does not own execution outcomes.

The following remain downstream:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

Interoperability can report:

SUPPORTED
UNSUPPORTED
INVALID

but it must not conflate those with runtime resilience outcomes.

---

126. Interoperability and Quantum State

State representations must be preserved according to semantic capability.

An importer may receive:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

or equivalent external representations.

It must map them into the canonical state semantics.

It must not allocate a fixed-size state vector merely to parse the source.

---

127. Symbolic Quantum Systems

Interoperability must support symbolic systems where the external format permits them.

For example:

n
θ
φ
problem_size

must not be forced into a fixed machine representation prematurely.

Semantic evaluation belongs to the appropriate compiler stage.

---

128. Large Values

Numeric values in external formats must not be artificially narrowed solely because the current target machine uses a particular integer width.

If the external format supports a larger semantic domain than a local representation, the implementation must:

- preserve the value;
- use an appropriate representation;
- or reject it explicitly as unsupported.

It must not silently truncate.

---

129. Integer Overflow

Import/export implementations must detect integer overflow when converting between representations.

For example:

external large integer
    ↓
host representation

must not silently wrap.

Overflow must become a structured conversion/resource diagnostic.

---

130. Floating-Point Conversion

Floating-point conversion must specify:

- precision;
- rounding;
- representability;
- NaN handling;
- infinity handling;
- signed zero where relevant.

A conversion must not silently change numerical semantics beyond the declared contract.

---

131. Identifier Mapping

External identifiers must map into Zamani identifiers without collisions.

If escaping or renaming is required, the mapping must be deterministic.

For example:

external_name
    ↓
canonical_name

must be reversible where round-trip fidelity requires it.

---

132. Namespace Mapping

External namespaces must not collide with Zamani namespaces.

The mapping must be:

external namespace
    ↓
explicit semantic namespace

rather than a silent global rename.

---

133. Symbol Tables

Format-specific symbol tables belong to format implementations.

They must not become global compiler state.

Importers must use request-scoped state.

This supports:

- determinism;
- thread safety;
- parallel compilation;
- independent conversions.

---

134. Thread Safety

Interoperability implementations should avoid mutable global state.

The public importer/exporter objects should be configuration-driven and request-local.

No global:

current QPU
current circuit
current parser
current include cache
current symbol table

may be required for correctness.

---

135. Concurrency

Independent format conversions should be capable of running concurrently when the underlying frontend contracts permit it.

One import operation must not mutate another import operation's:

- parser state;
- symbol table;
- diagnostics;
- configuration;
- semantic state.

---

136. Reproducibility

Given the same:

input
format version
configuration
compiler/frontend version

conversion should be reproducible.

If an implementation uses optimization or canonicalization, its deterministic behavior must be documented.

---

137. Provenance

The interoperability layer must preserve enough provenance to answer:

Where did this semantic operation originate?
Which external format produced it?
Which version?
Which source location?
Which conversion rule?

This is particularly important for diagnostics and debugging.

---

138. Security of Metadata

Metadata must not be trusted merely because it came from an external format.

Metadata that claims:

trusted
verified
calibrated
safe
authenticated

must not automatically be treated as authoritative security state.

Security authority belongs to the security/trust subsystem.

---

139. Authentication

External files may carry signatures or provenance.

Verification must be handled by the appropriate security/provenance subsystem.

The interoperability parser should preserve signature metadata where required but must not invent trust.

---

140. Resource Negotiation

If an external representation declares resource requirements, those requirements must map to the canonical resource model.

The compiler later determines:

satisfiable
unsatisfiable
requires fallback
requires different target

The importer does not choose the target.

---

141. Target Fallback

If a program can execute on several target categories:

QPU
simulator
accelerator
future target

interoperability should preserve target-independent semantics so that downstream target selection can choose an appropriate realization.

---

142. Future-Proof Operation Registry

Standard operations may be maintained by a semantic operation registry.

The registry must be extensible.

Adding:

new standard operation

must not necessarily require modifying:

grammar/quantum/operations.g4

or the Zamani lexer.

Operation identity is data.

Operation semantics are registered/validated downstream.

---

143. Future-Proof Format Registry

The same principle applies to formats.

The format registry should be extensible without making:

Zamani.g4

a registry of external languages.

---

144. Testing Architecture

Interoperability tests must exist at multiple levels.

At minimum:

lexical
syntax
semantic
import
export
round-trip
compatibility
diagnostics
security
determinism
scalability
boundary

---

145. OpenQASM Tests

OpenQASM tests must cover:

OpenQASM 3.0
OpenQASM 3.1
standard gates
custom operations
parameterized operations
qualified operations
registers
qubits
measurement
reset
classical conditions
mid-circuit measurement
feed-forward
dynamic control
unsupported constructs
invalid syntax
invalid semantics
round-trip conversion
deterministic output
large programs

---

146. QIR Tests

QIR tests must cover:

supported operations
unsupported operations
parameter mapping
qubit mapping
measurement
classical dependencies
metadata
version compatibility
import
export
round-trip semantics

---

147. Quil Tests

Quil tests must cover:

operation mapping
parameter mapping
measurement
classical control
labels
control flow
unsupported constructs
version handling
round-trip behavior

where supported by the implementation.

---

148. Negative Tests

Negative tests must include:

invalid syntax
unknown version
unsupported version
unsupported operation
unsupported parameter
unsupported control
invalid qubit index
invalid classical destination
invalid dependency
invalid include
invalid metadata
semantic mismatch
lossy conversion
integer overflow
invalid numeric value

---

149. Boundary Tests

Boundary tests must cover:

zero-size where semantically valid
one qubit
many qubits
symbolic qubit count
large numeric values
large operation lists
large namespace depth
large parameter expressions
large classical expressions
deep dynamic control
large metadata
large source files

Actual implementation resource limits must be tested separately from language semantics.

---

150. Scalability Tests

Scalability tests must establish that no artificial quantum ceiling has accidentally entered the interoperability implementation.

The tests should use parameterized/generated cases rather than establishing a fixed "maximum supported qubits" as language semantics.

For example:

n = 1
n = 2
n = several values
n = implementation stress values

The purpose is to test scaling behavior, not define a language maximum.

---

151. Hard-Coding Audit

Every interoperability implementation must be audited for:

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
MAX_QPUS
MAX_OPERATIONS

and equivalent hidden constants.

The audit must distinguish:

program constants

from:

implementation limits

from:

forbidden language ceilings

---

152. Forbidden Hardware Assumptions

The interoperability layer must not assume:

32-bit registers
64 GB RAM
24 GB VRAM
8 CPU cores
1 GPU
32 qubits
64 qubits
127 qubits
1000 qubits
fixed QPU topology
fixed coupling map
fixed pulse width
fixed clock
fixed number of devices

as universal Zamani assumptions.

These may exist only as properties of a specific target/environment.

---

153. No Fixed Gate Catalogue

The interoperability system must not turn an external standard's gate catalogue into Zamani's universal gate grammar.

This includes standard sets from:

- OpenQASM;
- QIR;
- Quil;
- vendors;
- simulators.

They remain operation definitions/registries.

The fundamental Zamani operation model remains open-ended.

---

154. Compatibility With "quantum::ir"

Every interoperability feature must answer:

What exact canonical quantum::ir representation does this become?

If the answer is:

another IR

the feature is not complete.

If no faithful "quantum::ir" representation exists, the feature is:

unsupported

until the canonical IR itself is extended through the proper repository-wide process.

---

155. No Interoperability-Driven IR Duplication

An external format must never cause:

new format
    ↓
new quantum IR

The correct sequence is:

new format
    ↓
semantic mapping analysis
    ↓
existing quantum::ir

Only if the semantic requirement genuinely exceeds the capabilities of the canonical IR should the canonical IR itself be evaluated for extension.

That extension belongs outside this interoperability file.

---

156. Canonical IR Extension Rule

If an external format exposes a semantic feature that "quantum::ir" cannot currently represent:

1. document the missing semantic capability;
2. determine whether the feature belongs in Zamani semantics;
3. update the canonical semantic/IR contract if approved;
4. update the relevant AST/semantic mappings;
5. update interoperability;
6. add conformance tests.

Do not solve the problem by creating a second IR inside the format frontend.

---

157. Import/Export Matrix

The production system should maintain a matrix conceptually equivalent to:

Capability| OpenQASM| QIR| Quil| Future formats
Basic operations| version-dependent| version-dependent| version-dependent| declared
Parameters| declared| declared| declared| declared
Controls| declared| declared| declared| declared
Adjoints| declared| declared| declared| declared
Measurement| declared| declared| declared| declared
Mid-circuit measurement| declared| declared| declared| declared
Classical feed-forward| declared| declared| declared| declared
Dynamic control| declared| declared| declared| declared
Custom operations| policy-dependent| policy-dependent| policy-dependent| declared
Resource metadata| policy-dependent| policy-dependent| policy-dependent| declared
Hardware metadata| policy-dependent| policy-dependent| policy-dependent| declared
Pulse data| policy-dependent| policy-dependent| policy-dependent| declared

The matrix is an implementation/conformance artifact.

It must not become another language specification.

---

158. Format Capability Declaration

Each format implementation must declare what it supports.

Conceptually:

format
version
import
export
operations
parameters
measurement
dynamic_control
classical_feedforward
channels
noise
timing
physical_mapping
metadata
resource_requirements
capabilities

Unsupported capabilities must be explicit.

---

159. Format Version Capability

A format implementation must not claim support for a version merely because it can parse some syntax from it.

Version support requires:

syntax
+
semantics
+
conversion
+
diagnostics
+
tests

---

160. Production Readiness Criteria

An interoperability format is production-ready only when:

[ ] format specification identified
[ ] supported versions explicitly declared
[ ] lexer complete
[ ] parser complete
[ ] AST complete
[ ] semantic validation complete
[ ] source spans preserved
[ ] operation mapping complete
[ ] parameter mapping complete
[ ] qubit mapping complete
[ ] classical mapping complete
[ ] measurement mapping complete
[ ] dynamic-control mapping complete
[ ] resource mapping complete
[ ] capability mapping complete
[ ] canonical quantum::ir mapping complete
[ ] import complete
[ ] export complete
[ ] unsupported semantics rejected
[ ] silent semantic loss impossible
[ ] deterministic behavior verified
[ ] security boundary verified
[ ] no unsafe Rust
[ ] no hidden hardware limits
[ ] scalability tests pass
[ ] compatibility tests pass
[ ] round-trip tests pass where applicable
[ ] diagnostics tested
[ ] public API documented
[ ] integration with generic frontend verified

---

161. File-Level Completion Contract

This file itself is complete when it establishes all of:

Purpose
Ownership
Non-ownership
Authority
Format boundary
Import contract
Export contract
Canonical IR boundary
Semantic preservation
Lossless conversion
Unsupported behavior
Versioning
Compatibility
Security
Determinism
Scalability
Resource handling
Capability handling
Qubit ordering
Classical ordering
Measurement
Dynamic control
Feed-forward
Custom operations
Vendor extensions
Dialect interaction
QEC interaction
ZQN interaction
Routing interaction
Scheduling interaction
HAL interaction
Testing
Completion criteria

No later grammar file should need to modify this document merely because that grammar file was implemented.

---

162. Integration Checklist for Existing Files

The interoperability contract is integrated with the existing repository as follows:

Existing file/boundary| Responsibility| Interoperability relationship
"grammar/DESIGN.md"| repository architecture| highest architectural authority
"grammar/spec/quantum.md"| quantum language semantics| source semantic authority
"grammar/Zamani.g4"| canonical Zamani composition| does not import external format syntax
"grammar/quantum/quantum.g4"| quantum grammar composition| owns Zamani quantum syntax only
"grammar/quantum/operations.g4"| generic quantum operation syntax| receives semantic operation mappings
"grammar/quantum/types.g4"| quantum types| canonical source type mapping
"grammar/quantum/registers.g4"| registers| external register mapping
"grammar/quantum/measurement.g4"| measurement| external measurement mapping
"grammar/quantum/dynamic-control.g4"| dynamic quantum control| external dynamic-control mapping
"grammar/quantum/classical-feedforward.g4"| classical feed-forward| external dependency mapping
"grammar/quantum/resource-requirements.g4"| quantum resource intent| external resource mapping
"grammar/quantum/quantum-capabilities.g4"| capabilities| external capability mapping
"grammar/quantum/error-correction.g4"| QEC intent| external QEC metadata mapping
"grammar/quantum/noise.g4"| noise intent| external noise metadata mapping
"grammar/quantum/README.md"| quantum grammar architecture| parent architectural contract
"grammar/spec/compatibility.md"| compatibility| format compatibility participates here
"grammar/compatibility/"| migrations/versioning| external format migration
"grammar/validation/"| repository validation| interoperability conformance
"grammar/tests/"| repository tests| interoperability test integration
"src/quantum/frontend/"| quantum frontend| generic frontend boundary
"src/quantum/frontend/formats/openqasm/mod.rs"| OpenQASM facade| current concrete interoperability implementation
"quantum::ir"| canonical quantum IR| sole canonical semantic/IR destination

---

163. Required Integration Direction

The required direction is:

Zamani grammar
      │
      ▼
frontend AST
      │
      ▼
semantic analysis
      │
      ▼
quantum::ir
      │
      ├───────────────┐
      │               │
      ▼               ▼
 OpenQASM            QIR
      │               │
      └───────┬───────┘
              ▼
       external formats

For import:

OpenQASM ──┐
QIR ───────┤
Quil ──────┤
Vendor ────┤
Future ────┘
      │
      ▼
format frontend
      │
      ▼
canonical semantic mapping
      │
      ▼
quantum::ir

---

164. What This File Must Never Become

This file must never become:

another Zamani grammar

or:

OpenQASM specification

or:

QIR specification

or:

vendor hardware manual

or:

quantum compiler implementation

or:

QEC specification

or:

routing specification

or:

scheduler specification

It is the contract connecting those systems.

---

165. Final Architectural Invariant

The entire interoperability architecture reduces to:

             ZAMANI
                │
                ▼
       canonical semantics
                │
                ▼
          quantum::ir
                │
      ┌─────────┼──────────┐
      │         │          │
      ▼         ▼          ▼
   OpenQASM    QIR       Quil
      │         │          │
      └─────────┼──────────┘
                │
          other formats

and in the reverse direction:

external format
       │
       ▼
format-specific frontend
       │
       ▼
validation
       │
       ▼
semantic mapping
       │
       ▼
quantum::ir
       │
       ▼
Zamani compiler

There is exactly one canonical quantum semantic boundary.

---

166. Final POCO-REAF Invariant

The interoperability layer must preserve:

PROGRAM
   ↓
ONCE
   ↓
CANONICAL SEMANTICS
   ↓
QUANTUM::IR
   ↓
TARGET-INDEPENDENT COMPILATION
   ↓
TARGET REALIZATION

The same semantic program may therefore be represented through:

Zamani
OpenQASM
QIR
Quil
vendor formats
future formats

without making any one external format the definition of Zamani.

The compiler may later determine:

CPU
GPU
FPGA
ASIC
QPU
simulator
distributed system
hybrid system
future computational substrate

according to:

requirements
capabilities
resources
constraints
preferences
hints
target availability

and not according to arbitrary grammar-level hardware assumptions.

---

167. Final Production Rule

The definitive interoperability rule is:

«External representations may describe quantum computation, but only Zamani's canonical semantic model and "quantum::ir" define how that computation participates in the Zamani compiler.»

Therefore:

External format
      ↓
parse
      ↓
validate
      ↓
preserve
      ↓
map
      ↓
quantum::ir
      ↓
optimize
      ↓
route
      ↓
schedule
      ↓
QEC / resilience / ZQN
      ↓
HAL
      ↓
target

No external format may bypass that semantic boundary.

No interoperability implementation may introduce a second quantum IR.

No interoperability implementation may impose artificial hardware ceilings.

No interoperability implementation may silently discard semantics.

No interoperability implementation may execute untrusted input.

No interoperability implementation may require "unsafe" Rust.

The resulting architecture remains compatible with:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

while allowing quantum interoperability to grow from the smallest supported computation to arbitrarily large computations constrained only by the resources and capabilities actually available at compilation and execution time.

---

168. Definition of Done

"grammar/quantum/interoperability.md" is considered integrated when the repository can demonstrate:

✓ one interoperability architecture
✓ one canonical quantum semantic boundary
✓ one canonical quantum::ir
✓ OpenQASM isolated as an external format
✓ QIR isolated as an external format
✓ Quil isolated as an external format
✓ vendor formats isolated
✓ future formats independently addable
✓ no external format becomes Zamani grammar
✓ no fixed gate enumeration is required
✓ no fixed qubit ceiling exists
✓ no fixed device ceiling exists
✓ no fixed resource ceiling exists
✓ logical/physical resources remain distinct
✓ requirements/capabilities remain distinct
✓ measurement semantics are preserved
✓ classical feed-forward is preserved
✓ dynamic-control semantics are preserved
✓ operation parameters are preserved
✓ custom operations remain open-ended
✓ unsupported semantics produce structured diagnostics
✓ silent semantic loss is prohibited
✓ import/export is deterministic
✓ source provenance is preserved
✓ security boundaries are enforced
✓ no unsafe Rust is required
✓ version compatibility is explicit
✓ round-trip behavior is tested
✓ scalability is tested
✓ interoperability is independently completable
✓ downstream integration is predetermined

Canonical principle:

Interoperability is a bridge.

It is not a second language.
It is not a second AST authority.
It is not a second quantum IR.
It is not a hardware selector.
It is not a runtime.
It is not a QEC engine.

It is the controlled, validated, deterministic,
semantics-preserving bridge between external
quantum representations and Zamani's canonical
quantum semantic model.