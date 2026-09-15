Below is the complete normative content for grammar/validation/compatibility-rules.md. It is designed to stand on its own while explicitly integrating with the other grammar validation/specification files and with Zamani’s compiler, IR, quantum, hardware, runtime, and interoperability boundaries.

# Zamani Compatibility Rules

**Path:** `grammar/validation/compatibility-rules.md`

**Status:** Normative production specification  
**Scope:** Zamani grammar, language syntax, AST, semantic analysis, types, effects, resources, dialects, compilation, canonical IR, runtime, interoperability, and target integration  
**Implementation baseline:** Rust 1.97 / Rust 1.97.1  
**Safety requirement:** Safe Rust only; `unsafe` MUST NOT be required by the grammar, parser, AST, validation, or compatibility infrastructure.  
**Primary portability objective:** Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

## 1. Purpose

This document defines the compatibility rules for the Zamani language and its grammar infrastructure.

Compatibility MUST be treated as a first-class architectural property.

Zamani MUST permit a program to remain semantically meaningful while the implementation target changes across:

- tiny embedded systems;
- CPUs;
- multicore CPUs;
- GPUs;
- FPGAs;
- ASICs;
- quantum processors;
- quantum simulators;
- heterogeneous accelerators;
- clusters;
- distributed systems;
- cloud environments;
- edge environments;
- future computing architectures.

The central compatibility principle is:

> A Zamani program expresses computation and intent.  
> A target expresses how that computation can be realized.

Changing a target MUST NOT require changing source semantics merely because the target has a different:

- number of cores;
- number of qubits;
- amount of memory;
- accelerator count;
- topology;
- instruction set;
- physical device;
- timing model;
- network size;
- hardware vendor;
- deployment topology.

Target-specific adaptation belongs to compilation, resource management, routing, scheduling, hardware abstraction, runtime, deployment, or explicitly target-bound source constructs.

---

# 2. Normative language

The following terms are normative:

- **MUST** — mandatory.
- **MUST NOT** — prohibited.
- **SHOULD** — recommended unless a documented reason exists.
- **SHOULD NOT** — normally prohibited unless justified.
- **MAY** — permitted.
- **REQUIRED** — equivalent to MUST.
- **OPTIONAL** — explicitly non-mandatory.

A compatibility rule applies to all grammar components unless a more specific domain specification explicitly defines a stricter rule.

A domain specification MUST NOT weaken a core compatibility invariant.

---

# 3. Compatibility objectives

Zamani compatibility has several independent dimensions.

They MUST NOT be collapsed into one generic notion of "compatibility."

The following dimensions are distinct:

1. source compatibility;
2. lexical compatibility;
3. grammar compatibility;
4. parser compatibility;
5. AST compatibility;
6. name-resolution compatibility;
7. type compatibility;
8. semantic compatibility;
9. effect compatibility;
10. capability compatibility;
11. resource compatibility;
12. dialect compatibility;
13. macro compatibility;
14. metaprogramming compatibility;
15. canonical IR compatibility;
16. compiler compatibility;
17. compiled-artifact compatibility;
18. ABI compatibility;
19. runtime compatibility;
20. execution-environment compatibility;
21. target compatibility;
22. hardware compatibility;
23. interoperability compatibility;
24. serialization compatibility;
25. diagnostic compatibility;
26. reproducibility compatibility.

A failure in one dimension MUST NOT be reported as a failure in another dimension.

For example:

> "This program requires more qubits than the selected machine provides"

is a **resource/target compatibility failure**, not a grammar incompatibility.

Likewise:

> "This syntax is not valid in the selected language version"

is a **language/grammar compatibility failure**, not a hardware failure.

---

# 4. Core compatibility invariant

The following invariant is mandatory:

```text
Source
  ↓
Lexing
  ↓
Parsing
  ↓
AST
  ↓
Semantic Analysis
  ↓
Canonical Semantic Representation
  ↓
Compilation
  ↓
Target Adaptation
  ↓
Execution

Compatibility MUST preserve the intended semantics across these boundaries.

The grammar MUST NOT depend on:

runtime state;

hardware discovery;

calibration state;

device availability;

scheduler state;

router state;

optimization state;

QEC execution state;

ZQN runtime observations;

network availability;

filesystem state;

environment-specific random values.



---

5. Compatibility versus availability

Compatibility and availability are different.

A program may be compatible with the Zamani language while being impossible to execute on a particular target.

For example:

requires quantum;

may be valid Zamani syntax and semantics even when the selected execution environment has no quantum backend.

The correct result is:

Language compatibility: PASS
Semantic validity: PASS
Target compatibility: UNKNOWN/PASS/FAIL
Resource availability: FAIL

The compiler MUST NOT rewrite the source merely because the target lacks resources.


---

6. Compatibility versus portability

Portability means that semantics can be preserved across targets.

Compatibility means that two representations, versions, implementations, or environments can correctly interact under a defined contract.

They are related but not identical.

A language feature MAY be:

source-compatible but not target-portable;

target-portable but not ABI-compatible;

grammar-compatible but semantically incompatible;

semantically compatible but unsupported by a particular runtime;

compatible at the IR level but incompatible at the binary level.


Each case MUST be represented explicitly.


---

7. POCO-REAF compatibility model

Zamani's compatibility architecture MUST support:

Program Once
      ↓
Stable Source Semantics
      ↓
Compile Once
      ↓
Portable Semantic / Compilation Artifact
      ↓
Target Adaptation
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

"Run Forever" MUST NOT mean that an old binary is guaranteed to execute unchanged on every future machine.

It means that Zamani's semantic and artifact model MUST provide a versioned path for future implementations to interpret, migrate, lower, or recompile older programs.

Therefore:

semantic stability
≠
binary immortality

and:

source compatibility
≠
unchanged hardware representation


---

8. Version authority

There MUST be one authoritative language-version model.

Version information MUST NOT be independently invented by:

individual grammar files;

domain grammar files;

parser implementations;

AST implementations;

compiler backends;

examples;

documentation.


The authoritative version contract belongs to:

grammar/specification/language-version.md

and is referenced by:

grammar/compatibility/versions.md
grammar/validation/compatibility-rules.md
grammar/dialects/versioning.g4

These files MUST agree.

No file may silently introduce a new language version.


---

9. Version identifiers

Language versions MUST be structured and machine-readable.

A version MUST identify at least:

language major version;

language minor version;

compatibility-relevant revision where required;

dialect identity where applicable.


The implementation MUST NOT use arbitrary strings as the sole compatibility mechanism.

For example, compatibility logic MUST NOT depend on:

if version == "whatever"

without a formally defined version model.


---

10. Major-version compatibility

A major language-version change MAY introduce breaking changes.

Breaking changes include:

changing the meaning of valid syntax;

removing required syntax;

changing operator precedence;

changing type meaning;

changing ownership semantics;

changing effect meaning;

changing quantum measurement semantics;

changing hardware semantics;

changing resource semantics;

changing module resolution;

changing macro hygiene;

changing canonical semantic meaning.


A major-version transition MUST provide migration documentation.

Where practical, an automated migration mechanism SHOULD exist.


---

11. Minor-version compatibility

A minor version SHOULD preserve existing valid source semantics.

Adding:

new keywords;

new operators;

new syntax;

new types;

new domain constructs;

new attributes;

new dialect capabilities


MUST be designed so that existing valid programs do not silently change meaning.

New syntax MUST NOT introduce accidental reinterpretation of existing programs.


---

12. Patch-version compatibility

Patch-level changes MUST NOT intentionally change the meaning of valid source programs.

Patch changes MAY correct:

diagnostics;

implementation bugs;

parser performance;

error recovery;

internal representations;

tooling behavior;

non-semantic documentation.


A patch release MUST NOT silently change:

operator precedence;

type semantics;

resource semantics;

ownership;

effect semantics;

quantum semantics;

hardware semantics.



---

13. Source compatibility

A source program is source-compatible with a language version when:

1. it lexes successfully;


2. it parses successfully;


3. its grammar interpretation is supported;


4. its required semantic rules are supported;


5. its required language features are available;


6. its required dialects are available.



Source compatibility MUST be determined independently from target availability.


---

14. Backward compatibility

When a new language version is backward compatible, an existing valid program MUST retain its intended meaning.

Backward compatibility MUST be tested using:

existing source examples;

existing test fixtures;

canonical AST snapshots where appropriate;

semantic equivalence tests;

canonical IR comparisons where stable;

diagnostics where compatibility requires them.


A parser MUST NOT accept an old construct while silently assigning it a different semantic meaning.


---

15. Forward compatibility

Zamani MUST be designed so that future language features can be introduced without unnecessarily invalidating existing tooling.

Forward-compatible mechanisms MAY include:

explicit dialects;

versioned syntax;

namespaced extensions;

attributes;

reserved keywords;

reserved syntax;

extension namespaces;

structured metadata.


Unknown extensions MUST NOT be silently interpreted as known semantics.


---

16. Unknown syntax

Unknown syntax MUST produce a deterministic diagnostic unless an explicit extension mechanism permits it.

The implementation MUST NOT:

silently discard unknown constructs;

convert unknown constructs into comments;

invent semantics;

guess the intended construct;

query hardware to decide what unknown syntax means.


An exporter or parser MUST report unsupported syntax as an error.


---

17. Deprecation

A feature MUST NOT be removed immediately after being marked deprecated unless the language version policy explicitly permits immediate removal.

Deprecation SHOULD provide:

feature identity;

introduction version;

deprecation version;

removal policy;

replacement;

migration guidance;

compatibility impact.


Deprecation MUST NOT change semantics before removal unless explicitly documented.


---

18. Removal

A grammar construct MAY be removed only when:

1. its removal is permitted by the language-version policy;


2. consumers have been identified;


3. migration is documented;


4. tests have been updated;


5. generated parser artifacts are regenerated;


6. documentation is updated;


7. compatibility fixtures are updated;


8. no supported dialect still depends on the construct.



Silent removal is prohibited.


---

19. Reserved syntax

Reserved syntax exists to preserve future evolution space.

Reserved syntax MUST NOT be treated as implemented language syntax.

A reserved keyword MUST produce an appropriate diagnostic when used in a position where it cannot legally appear.

The reservation mechanism MUST NOT introduce resource limits.

For example, reserving future quantum or hardware terminology is acceptable.

Reserving only:

qubit0 ... qubit63

as a mechanism for future scalability is not acceptable.


---

20. Keywords and identifiers

Keyword compatibility MUST be handled carefully.

Adding a new keyword can break an existing program that used the word as an identifier.

Therefore new keywords SHOULD be introduced using one of:

contextual keyword rules;

explicit namespace qualification;

versioned syntax;

migration support;

carefully managed reserved-word policy.


The grammar MUST NOT accidentally turn an existing identifier into a keyword without compatibility analysis.


---

21. Identifier compatibility

Identifier syntax MUST remain stable unless a language version explicitly changes it.

Identifier compatibility includes:

Unicode policy;

normalization policy;

case sensitivity;

escape rules;

reserved-word interaction;

namespace qualification.


Identifiers MUST NOT encode machine-specific assumptions.

Examples of prohibited semantic assumptions include:

cpu0
gpu7
qpu3
core15
device42

being implicitly interpreted as physical resources.

A source identifier is not a physical resource identifier unless an explicit hardware-bound semantic construct says so.


---

22. Numeric literal compatibility

Numeric literal syntax MUST remain independent from target machine widths.

The grammar MUST NOT define language compatibility using:

u8
u16
u32
u64

as arbitrary universal limits.

Where such types exist, they are language-level semantic types.

They are not claims that the entire language is limited to those widths.

Large integers, arbitrary precision, symbolic values, or implementation-defined numeric domains MUST be represented by explicit type semantics where supported.


---

23. Resource compatibility

Resource requirements MUST be separate from language version compatibility.

For example:

requires qubits >= n;

is a resource requirement.

It is not a grammar version.

Similarly:

requires memory >= amount;
requires accelerator(kind);
requires capability(quantum);

must not become hidden language limits.


---

24. No compatibility ceilings for scalable resources

Compatibility specifications MUST NOT introduce arbitrary ceilings such as:

MAX_QUBITS = 64
MAX_CORES = 256
MAX_THREADS = 1024
MAX_DEVICES = 32
MAX_NODES = 1024

Such values MUST NOT define language compatibility.

If an implementation has an operational limit, it MUST be represented as an implementation or target limitation.


---

25. Quantum compatibility

Quantum language compatibility MUST remain independent of physical QPU capacity.

The grammar MUST NOT assume:

a fixed number of qubits;

a fixed register width;

a fixed topology;

a fixed gate set;

a fixed coupling map;

a fixed device;

a fixed calibration;

a fixed timing grid.


For example, this is prohibited as a universal grammar assumption:

q[0]
q[1]

as though every program has exactly two physical qubits.

A program MAY explicitly use two qubits.

That is a semantic property of that particular program, not a language-wide machine limit.


---

26. Quantum semantic compatibility

Quantum syntax MUST lower into the repository's canonical quantum semantic representation.

quantum::ir remains the canonical quantum semantic boundary.

The grammar MUST NOT create an independent competing quantum IR.

The dependency direction is:

Zamani quantum syntax
        ↓
AST
        ↓
semantic analysis
        ↓
quantum::ir
        ↓
optimization
        ↓
routing
        ↓
scheduling
        ↓
ZQN / hardware / runtime

The grammar MUST NOT depend on the physical implementation details of quantum::ir.


---

27. Quantum backend compatibility

A valid quantum program MUST remain semantically valid even when:

a backend changes;

a QPU has fewer available physical qubits;

a simulator is selected;

topology changes;

native gates differ;

calibration changes;

error characteristics change.


If a target cannot execute the program, compilation MUST produce a target/resource failure rather than rewriting source semantics.


---

28. Measurement compatibility

Measurement MUST remain explicit in source semantics.

A parser or compatibility layer MUST NOT automatically insert measurements merely because a backend requires them.

For example, a frontend MUST NOT silently transform:

quantum computation

into:

measure every qubit

unless the language semantics explicitly define that behavior.

Measurement is semantically meaningful and therefore belongs to the program's explicit semantic model.


---

29. QEC compatibility

Quantum error correction syntax MUST remain compatible with the QEC subsystem without making the grammar the owner of QEC algorithms.

Grammar constructs MAY express:

logical-qubit intent;

protection requirements;

QEC policy;

error tolerance requirements;

code-family selection where explicitly semantic;

correction intent.


The grammar MUST NOT embed:

implementation-specific decoder algorithms;

fixed hardware code distances;

fixed physical-qubit layouts;

provider-specific QEC implementations.


QEC remains responsible for detection and correction.


---

30. ZQN compatibility

ZQN owns quantum noise/fault semantics.

Grammar compatibility MUST NOT duplicate ZQN's noise model.

Source-level noise declarations, where supported, describe semantic assumptions, requirements, models, or experimental intent.

They MUST NOT automatically mean:

> this is the actual noise present on the selected device.



Actual noise belongs to ZQN, calibration, hardware, or runtime observation.


---

31. Classical compatibility

Classical language compatibility MUST remain independent of:

CPU count;

register count;

SIMD width;

cache size;

memory size;

thread count;

GPU count.


Changing from:

single core

to:

many-core

MUST NOT require source semantic changes unless the program explicitly requests a target-specific execution property.


---

32. Parallelism compatibility

Parallel constructs MUST describe parallel semantics or execution intent.

They MUST NOT encode an implicit fixed machine size.

For example:

parallel for item in data

MUST NOT mean:

run on exactly 8 threads

unless 8 is explicitly part of the program's semantics or resource constraint.


---

33. Concurrency compatibility

Concurrency semantics MUST be stable across runtimes.

Differences in:

worker count;

scheduler implementation;

CPU topology;

operating system;

accelerator availability


MUST NOT silently change the semantics of a correctly synchronized program.

Where scheduling order is intentionally nondeterministic, the language semantics MUST define the allowed outcomes.


---

34. Memory compatibility

Memory syntax MUST distinguish:

semantic memory behavior;

ownership;

borrowing;

allocation;

lifetime;

placement;

physical memory availability.


A source program MUST NOT assume that:

memory = fixed physical address

unless it is explicitly using a hardware-bound interface.

Physical addresses belong to target/hardware-specific layers.


---

35. HDL compatibility

HDL syntax MUST distinguish hardware semantics from implementation-specific realization.

Compatible hardware semantics MAY describe:

modules;

interfaces;

signals;

registers;

state machines;

timing behavior;

pipelines;

memory behavior;

parameterized widths.


Implementation-specific choices MAY include:

FPGA family;

ASIC technology;

physical cells;

placement;

routing;

clock tree;

device package.


Those choices MUST NOT silently become universal language semantics.


---

36. Parameterized hardware compatibility

Hardware dimensions SHOULD be parameterized where they represent scalable structure.

Examples include:

bus width;

memory depth;

vector width;

pipeline stages;

module instances;

channel counts.


A construct such as:

parameter WIDTH;

is fundamentally different from:

WIDTH = 32

The latter may be a valid program-specific choice.

It MUST NOT be interpreted as a universal Zamani limitation.


---

37. Hardware target compatibility

A hardware target MAY impose constraints.

For example:

requires capability(fpga);
requires resource(memory >= M);
requires timing(latency <= L);

The compiler MAY reject an incompatible target.

The compiler MUST NOT alter the language definition to accommodate that target.


---

38. Distributed compatibility

Distributed syntax MUST NOT assume a fixed number of nodes.

Programs SHOULD describe:

roles;

services;

communication semantics;

replication requirements;

consistency requirements;

placement constraints;

fault-tolerance policies.


The runtime and deployment system determine actual node allocation.

Changing:

1 node

to:

1000 nodes

MUST NOT require changing source semantics when the program's semantics are scale-independent.


---

39. Network compatibility

Network constructs MUST distinguish:

protocol semantics;

endpoint identity;

transport requirements;

security requirements;

topology;

deployment.


A source-level protocol requirement MUST NOT implicitly bind the program to a fixed physical network.


---

40. AI/data compatibility

AI and data constructs MUST support symbolic or parameterized dimensions where appropriate.

Compatibility MUST NOT assume fixed:

tensor dimensions;

dataset sizes;

batch sizes;

accelerator counts;

model sizes;

memory capacities.


A particular model MAY have fixed semantic dimensions.

That is different from imposing a language-wide dimension limit.


---

41. Generic compatibility

Generic constructs MUST preserve semantics across valid instantiations.

Adding a new valid type or resource implementation MUST NOT require changing the grammar's generic model.

Generic syntax MUST NOT enumerate finite supported types such as:

generic_type ::= cpu | gpu | fpga | qpu

when the intent is extensible computation.

Capability-based or type-based extension mechanisms SHOULD be preferred.


---

42. Type compatibility

Type changes MUST be evaluated semantically.

A type is compatible only when its:

representation;

operations;

conversions;

ownership;

lifetime;

effects;

invariants


remain compatible under the relevant language contract.

A compiler MUST NOT silently reinterpret a type merely because the target architecture has a different representation.


---

43. Effect compatibility

Effects are semantic contracts.

Adding a new backend MUST NOT silently remove or weaken effects.

Examples include:

I/O;

quantum operations;

hardware access;

networking;

distributed execution;

security-sensitive operations.


An implementation MAY realize an effect differently on different targets, but it MUST preserve the effect contract.


---

44. Capability compatibility

Capabilities represent what an environment can provide.

A capability MUST NOT automatically become a semantic requirement.

For example:

capability quantum

describes an environment capability.

It does not mean that every Zamani program requires a quantum processor.

Likewise:

requires quantum

is a program requirement.

The two concepts MUST remain distinct.


---

45. Requirement compatibility

Requirements express conditions necessary for correct execution.

Requirements MUST be:

explicit;

inspectable;

deterministic;

target-independent unless explicitly target-bound.


A requirement MAY be evaluated against:

compile-time target information;

deployment configuration;

runtime capability information.


The grammar itself MUST NOT perform that evaluation.


---

46. Constraint compatibility

Constraints restrict valid implementations.

Examples:

latency <= L
memory >= M
energy <= E
reliability >= R

Constraints MUST NOT be confused with semantic values.

If a constraint cannot be satisfied, the implementation MUST report a constraint failure rather than alter the program.


---

47. Preference compatibility

Preferences are optimization guidance.

A preference MAY influence:

backend selection;

scheduling;

routing;

optimization;

placement.


A preference MUST NOT be treated as a correctness requirement unless explicitly declared as such.

Changing a preference MUST NOT change the semantic meaning of the program.


---

48. Hint compatibility

Hints are non-binding implementation guidance.

Hints MUST NOT become hidden semantic dependencies.

A backend MAY ignore a hint.

A program MUST remain semantically valid if a hint cannot be honored.


---

49. Target binding

Target binding MUST be explicit.

Portable source MUST NOT become target-specific merely because it was compiled for a particular machine.

A target-bound program SHOULD make its binding explicit through:

target declarations;

target profiles;

hardware interfaces;

deployment configuration;

dialects;

resource constraints.



---

50. AST compatibility

The AST MUST preserve all source information required for semantic interpretation and diagnostics.

AST changes MUST distinguish:

additive fields;

representation changes;

semantic changes;

removed nodes;

renamed nodes;

changed source-span behavior.


If ASTs are serialized or exposed publicly, their schema MUST be versioned.

Internal AST structures MAY evolve without public compatibility guarantees when explicitly classified as internal.


---

51. Source spans

Source spans MUST remain compatible with diagnostics and tooling.

Span representations MUST support source sizes allowed by the implementation without artificially restricting the language.

Small fixed-width indexes MUST NOT be selected merely because existing examples are small.

Arithmetic involving source offsets MUST use checked operations where required.


---

52. Parser compatibility

Parser behavior MUST be deterministic.

Given:

same source
+
same language version
+
same dialect set
+
same grammar configuration

the parser MUST produce the same structural interpretation.

Parser behavior MUST NOT depend on:

machine topology;

runtime state;

hardware;

current time;

network state;

random values.



---

53. Entry-point compatibility

The authoritative parser entry rule MUST consume the complete source input.

The equivalent of:

compilationUnit EOF

MUST be enforced at the top-level parsing boundary.

Trailing unexpected input MUST NOT be silently ignored.

This prevents a source file from being considered valid when only a prefix was successfully parsed.


---

54. Lexer compatibility

Lexer behavior MUST be deterministic.

Keyword and identifier precedence MUST be explicitly defined.

Lexer compatibility MUST account for:

longest applicable token;

explicit lexer ordering;

keyword versus identifier behavior;

Unicode;

escape sequences;

comments;

literals;

malformed tokens.


Adding a token MUST undergo compatibility analysis for collisions with existing token rules.


---

55. Grammar ambiguity compatibility

Grammar changes MUST be evaluated against:

grammar/validation/ambiguity-rules.md.

A grammar change is not compatible merely because the parser accepts both versions.

The interpretation MUST remain stable.

Changes MUST NOT introduce:

ambiguous parse trees;

target-dependent interpretation;

runtime-dependent interpretation;

accidental keyword collisions;

precedence changes;

associativity changes.



---

56. Ambiguity resolution

Ambiguity MUST be resolved through language-defined syntax and precedence.

It MUST NOT be resolved using:

hardware capabilities;

runtime observations;

calibration;

network responses;

filesystem contents;

backend availability.


If two constructs are semantically indistinguishable, the language specification MUST explicitly define whether they are:

equivalent;

canonicalized;

rejected;

dialect-specific.



---

57. Dialect compatibility

Dialects MUST be explicit and namespaced.

A dialect MUST identify:

name;

version;

compatibility contract;

feature set;

ownership;

extension namespace.


A dialect MUST NOT silently redefine core Zamani syntax.

Dialect selection MUST NOT depend on hardware discovery.


---

58. Dialect isolation

A dialect MUST NOT modify the meaning of unrelated core constructs without an explicit compatibility contract.

For example, a quantum dialect MUST NOT redefine:

ordinary arithmetic;

generic types;

core control flow


without an explicitly versioned language rule.


---

59. Vendor extensions

Vendor extensions MAY exist.

They MUST be:

explicitly namespaced;

versioned;

identifiable;

non-conflicting with core syntax;

isolated from portable semantics.


Vendor extensions MUST NOT be required for ordinary portable Zamani programs unless the program explicitly opts into them.


---

60. Experimental features

Experimental features MUST be explicitly marked.

They MUST NOT silently become permanent syntax.

Experimental syntax SHOULD provide:

feature identity;

version;

stability status;

compatibility policy;

migration path.



---

61. Macro compatibility

Macros MUST preserve hygiene and deterministic expansion.

Macro compatibility MUST account for:

expansion order;

name hygiene;

source spans;

diagnostics;

versioning;

recursion;

resource budgets.


A macro implementation MUST NOT rely on machine-specific resource counts.


---

62. Macro expansion limits

Macro systems MAY enforce implementation resource budgets.

For example:

expansion depth;

expansion count;

memory budget;

CPU budget.


These are operational safety controls.

They MUST NOT become language semantic limits.

They MUST be:

configurable where appropriate;

documented;

reported explicitly;

deterministic.


A failure caused by an expansion budget MUST NOT be reported as invalid Zamani syntax.


---

63. Metaprogramming compatibility

Compile-time execution MUST be versioned and deterministic.

Metaprogramming MUST NOT silently depend on:

current hardware;

filesystem contents;

network contents;

current time;

environment variables;

random state


unless such dependencies are explicitly modeled capabilities/effects.


---

64. Compile-time environment compatibility

Compile-time execution SHOULD receive an explicit compilation context.

That context MAY contain:

language version;

dialects;

target description;

resource capabilities;

feature configuration.


The grammar MUST NOT directly inspect the environment.


---

65. Compile-time target adaptation

Target-specific compile-time specialization MAY exist.

However, target-dependent specialization MUST NOT change the semantics of portable source unexpectedly.

For example:

if target supports capability X
    use implementation A
else
    use implementation B

is an implementation-selection mechanism.

It MUST NOT silently change what the program means.


---

66. Canonical IR compatibility

Canonical IR compatibility MUST be separately versioned.

The grammar MUST NOT assume that AST structure and IR structure are identical.

The canonical IR MUST represent semantic meaning after semantic analysis.

For quantum computation:

grammar
    ↓
AST
    ↓
semantic analysis
    ↓
quantum::ir

quantum::ir remains the canonical semantic boundary.

The grammar MUST NOT duplicate quantum IR types.


---

67. IR migration

When canonical IR changes:

1. identify the semantic change;


2. classify compatibility impact;


3. version the IR where required;


4. provide migration or lowering;


5. update consumers;


6. update compatibility tests.



An IR representation change MUST NOT automatically require a source-language change.


---

68. Optimization compatibility

Optimization MUST preserve semantics.

The optimization subsystem MUST NOT define grammar compatibility.

Optimization may change:

gate decomposition;

instruction selection;

expression representation;

scheduling choices;

routing;

implementation strategy.


It MUST NOT change the program's observable semantic meaning.


---

69. Scheduling compatibility

Scheduling MUST remain downstream of semantic representation.

Changing:

durations;

resource availability;

timing constraints;

scheduling policy


MUST NOT require changes to portable source semantics unless timing is itself part of the explicit program meaning.

The grammar MUST NOT hard-code scheduler assumptions.


---

70. Routing compatibility

Routing MUST adapt logical operations to physical resources.

A source program MUST NOT need to encode physical qubit topology unless it intentionally uses a hardware-bound construct.

The grammar MUST NOT require:

nearest-neighbor topology

or any particular coupling map for general quantum programs.


---

71. ZQN compatibility

ZQN may change:

noise models;

fault classifications;

channel models;

correlated-fault representations;

leakage models;

erasure models.


Such changes MUST NOT silently change the core grammar.

Source syntax MAY refer to abstract fault/noise semantics through defined contracts.

Actual device observations remain outside grammar ownership.


---

72. QEC compatibility

Changes to QEC algorithms MUST NOT require grammar changes unless the language itself introduces a new semantic construct.

For example, changing a decoder implementation is an implementation change.

It is not a language compatibility change.


---

73. Hardware abstraction compatibility

The hardware abstraction layer owns target capabilities and state.

Grammar compatibility MUST NOT duplicate hardware discovery.

Hardware capability schemas MAY evolve independently from the core grammar, subject to explicit interface/version compatibility.


---

74. Runtime compatibility

Runtime compatibility includes the ability to execute a valid compiled representation.

A runtime MAY reject an artifact because:

required capability is absent;

required resource is unavailable;

target is incompatible;

ABI is unsupported;

deployment constraints cannot be satisfied.


Such failures MUST NOT be represented as parser errors.


---

75. ABI compatibility

ABI compatibility is distinct from source compatibility.

ABI changes MAY include:

calling convention;

data representation;

alignment;

symbol naming;

binary layout.


An ABI change MUST NOT silently alter language semantics.

The interoperability subsystem owns ABI contracts.


---

76. FFI compatibility

Foreign-function interfaces MUST explicitly declare their external contract.

FFI declarations SHOULD identify:

calling convention;

ABI;

symbol;

parameter types;

return types;

ownership;

safety requirements;

effects;

target restrictions.


FFI compatibility MUST NOT be used to introduce hidden machine assumptions into the core language.


---

77. Interoperability compatibility

Interoperability adapters MUST be versioned independently where their external standards require it.

Examples include:

C;

C++;

Python;

OpenQASM;

Verilog;

other HDLs;

foreign runtimes.


An interoperability format MUST NOT redefine Zamani's canonical semantics.


---

78. OpenQASM compatibility

OpenQASM integration MUST remain an interoperability concern.

The OpenQASM frontend MUST translate into Zamani's canonical semantic model.

It MUST NOT create a parallel quantum semantic authority.

OpenQASM version differences MUST be handled by the interoperability layer.

Unsupported constructs MUST generate explicit diagnostics.


---

79. HDL interoperability

Verilog and other HDL interoperability MUST preserve hardware semantics.

Conversion MUST NOT silently discard:

timing semantics;

widths;

signedness;

clock behavior;

reset behavior;

state-machine behavior.


If exact semantic preservation is impossible, conversion MUST fail or explicitly mark the resulting representation as lossy according to the interoperability contract.


---

80. Serialization compatibility

Serialized AST, IR, metadata, compilation artifacts, and execution manifests MUST carry sufficient version information.

A serialized object MUST NOT be interpreted using an incompatible schema without:

migration;

explicit compatibility support;

or rejection.


Serialization formats SHOULD provide:

format identifier;

version;

schema version;

producer information where useful;

feature/dialect metadata.



---

81. Artifact compatibility

Compiled artifacts MUST identify the semantic and compilation contracts required to interpret them.

Artifacts SHOULD identify:

Zamani language version;

dialect versions;

IR version;

compiler compatibility range;

target requirements;

resource requirements;

ABI requirements;

runtime requirements;

provenance.


An artifact MUST NOT claim universal execution merely because it was successfully compiled once.


---

82. POCO-REAF artifact model

The preferred architecture is:

Source Program
      ↓
Semantic Model
      ↓
Portable Compilation Artifact
      ↓
Target Adaptation
      ↓
Target Artifact

The portable artifact SHOULD retain enough semantic information to permit future implementations to:

validate it;

migrate it;

lower it;

recompile it;

retarget it.


A target-specific binary MUST NOT be confused with the permanent semantic representation.


---

83. Future-version artifact handling

Future implementations SHOULD be able to recognize older artifacts.

If they cannot execute them directly, they SHOULD be able to report:

recognized format
unsupported version
migration available/unavailable

rather than treating the artifact as malformed.


---

84. Semantic compatibility

Two versions are semantically compatible when all specified observable behavior remains equivalent for the relevant program class.

Observable behavior includes, where applicable:

returned values;

mutations;

I/O;

measurement outcomes;

allowed nondeterministic outcomes;

timing guarantees when semantically specified;

resource guarantees;

security guarantees;

hardware behavior explicitly included in semantics.


Implementation-level representation differences do not constitute semantic incompatibility by themselves.


---

85. Semantic equivalence

Compatibility testing SHOULD compare semantic results rather than textual representation whenever possible.

For example:

source A

and:

source B

may be textually different while being semantically equivalent.

Likewise:

AST A

and:

AST B

may differ structurally after an internal refactor while lowering to equivalent canonical semantics.


---

86. Canonicalization

Where multiple syntactic representations are intentionally equivalent, semantic analysis MAY canonicalize them.

Canonicalization MUST be:

deterministic;

documented;

versioned where observable;

independent of hardware.


Canonicalization MUST NOT be used to silently erase meaningful source semantics.


---

87. Deterministic compatibility resolution

Compatibility resolution MUST be deterministic.

Given identical:

source;

language version;

dialect set;

compilation context;

dependency versions;

compatibility policy;


the same compatibility result MUST be produced.

Resolution MUST NOT depend on:

map iteration order;

network timing;

current time;

random selection;

machine discovery order.



---

88. Dependency compatibility

Modules and packages MUST declare their language/dialect/API compatibility requirements where needed.

Dependency resolution MUST distinguish:

language version;

library/API version;

ABI version;

dialect version;

compiler version;

runtime version.


These MUST NOT be collapsed into a single version number.


---

89. Module compatibility

A module's public interface MUST remain compatible according to its declared stability policy.

Changes MAY be:

additive;

source-breaking;

semantic-breaking;

ABI-breaking;

implementation-only.


The module system MUST identify the category.


---

90. Public versus private compatibility

Public language contracts require stronger compatibility guarantees than internal implementation details.

The following MAY change without public compatibility guarantees when classified as internal:

internal parser structs;

private helper functions;

internal caches;

internal optimization representations.


The following require explicit compatibility policy when exposed:

source syntax;

public AST schema;

serialized artifacts;

public IR schema;

ABI;

FFI;

public dialect contracts.



---

91. Documentation compatibility

The following files MUST remain mutually consistent:

grammar/README.md
grammar/Zamani.g4
grammar/Zamani-Grammar.md
grammar/grammar.md

grammar/specification/language-version.md
grammar/specification/compatibility.md
grammar/compatibility/versions.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md
grammar/compatibility/reserved.md
grammar/compatibility/compatibility-matrix.md

grammar/validation/grammar-validation.md
grammar/validation/semantic-boundaries.md
grammar/validation/ambiguity-rules.md
grammar/validation/scalability-rules.md
grammar/validation/compatibility-rules.md
grammar/validation/naming-rules.md
grammar/validation/hardcoding-audit.md

No explanatory document may claim syntax that the authoritative grammar does not implement.


---

92. Grammar authority integration

grammar/specification/grammar-authority.md MUST define which representation is authoritative.

This document governs compatibility behavior after that authority is established.

If conflicting grammar representations are discovered, the conflict MUST be resolved explicitly.

No implementation may guess which grammar is authoritative.


---

93. Integration with scalability rules

This document MUST be read together with:

grammar/validation/scalability-rules.md

Compatibility MUST NOT introduce artificial scalability ceilings.

A compatibility change is invalid if it introduces hidden limits such as:

maximum 64 qubits
maximum 1024 threads
maximum 256 nodes
maximum 32 devices

unless the value is explicitly a target/implementation limit and is not part of language semantics.


---

94. Integration with semantic boundaries

This document MUST be read together with:

grammar/validation/semantic-boundaries.md

The following boundary MUST remain intact:

Syntax
  ↓
AST
  ↓
Semantic Analysis
  ↓
Canonical IR
  ↓
Compiler
  ↓
Target
  ↓
Runtime

Compatibility logic MUST NOT bypass this boundary.


---

95. Integration with ambiguity rules

This document MUST be read together with:

grammar/validation/ambiguity-rules.md

A compatibility-preserving grammar evolution MUST preserve deterministic interpretation.

A syntactic addition is not compatible if it causes existing source to acquire a different parse.


---

96. Integration with grammar validation

grammar/validation/grammar-validation.md owns structural grammar validation.

It MUST validate:

lexer/parser generation;

grammar consistency;

rule reachability;

entry-point completeness;

generated parser validity;

invalid-input behavior.


This document adds compatibility requirements to those checks.


---

97. Integration with hardcoding audit

grammar/validation/hardcoding-audit.md MUST detect compatibility-breaking machine assumptions.

At minimum, audit patterns SHOULD include:

MAX_
MIN_
LIMIT
CAPACITY
QUBIT
QUBITS
CORE
CORES
THREAD
THREADS
GPU
FPGA
ASIC
DEVICE
TOPOLOGY
ADDRESS
NODE
NODES
q[0]
q[1]

Every match MUST be classified rather than blindly deleted.

Valid classifications include:

1. semantic constant;


2. lexical constant;


3. implementation safety limit;


4. target limit;


5. test fixture;


6. documentation example;


7. accidental hard-coding.




---

98. No hard-coded compatibility matrix

Compatibility MUST NOT be implemented through enormous manually enumerated tables such as:

CPU_1
CPU_2
CPU_3
...

or:

QPU_1
QPU_2
...

Target compatibility SHOULD be expressed through:

capabilities;

requirements;

constraints;

version ranges;

interfaces;

feature sets;

target descriptions.



---

99. Compatibility matrix ownership

The file:

grammar/compatibility/compatibility-matrix.md

MAY document supported compatibility relationships.

It MUST NOT become a hard-coded enumeration of all possible machines.

It SHOULD describe compatibility dimensions such as:

Language version
Dialect version
IR version
Compiler version
Runtime version
ABI version
Target capability


---

100. Migration ownership

The file:

grammar/compatibility/migrations.md

owns source migration guidance.

Migration tooling SHOULD operate on:

old syntax
    ↓
structured representation
    ↓
migration
    ↓
new syntax

rather than performing uncontrolled textual substitutions.


---

101. Migration safety

A migration MUST NOT silently change program semantics.

If semantic preservation cannot be guaranteed, the migration MUST:

reject the migration;

emit an explicit warning;

or require developer confirmation according to the language policy.



---

102. Compatibility diagnostics

Diagnostics MUST distinguish at least:

LEXICAL_INCOMPATIBILITY
SYNTAX_INCOMPATIBILITY
VERSION_INCOMPATIBILITY
DIALECT_INCOMPATIBILITY
SEMANTIC_INCOMPATIBILITY
TYPE_INCOMPATIBILITY
EFFECT_INCOMPATIBILITY
CAPABILITY_INCOMPATIBILITY
RESOURCE_INCOMPATIBILITY
TARGET_INCOMPATIBILITY
IR_INCOMPATIBILITY
ABI_INCOMPATIBILITY
RUNTIME_INCOMPATIBILITY
INTEROPERABILITY_INCOMPATIBILITY
MIGRATION_REQUIRED
UNSUPPORTED_FEATURE

Exact error-code ownership belongs to the relevant diagnostic/error subsystem.

The grammar MUST NOT invent provider-specific hardware error codes.


---

103. Diagnostic stability

Diagnostics SHOULD remain stable enough for:

IDE tooling;

CI systems;

tests;

migration tools;

language servers.


Human-readable wording MAY evolve.

Machine-readable error categories/codes SHOULD remain stable across compatible versions.


---

104. Error provenance

Compatibility errors MUST identify the relevant layer.

For example:

source syntax → grammar
unsupported feature → language/dialect
insufficient qubits → resource/target
unsupported gate decomposition → compiler/backend
missing decoder → QEC capability
calibration mismatch → hardware/runtime

This prevents downstream failures from being misreported as grammar failures.


---

105. Compatibility and diagnostics truncation

Diagnostic output limits MUST NOT change semantic compatibility.

An implementation MAY limit the number of emitted diagnostics to protect resources.

For example:

diagnostic_budget = configured value

is an operational safety policy.

It MUST NOT mean:

the source contains only N errors

semantically.


---

106. Parser resource budgets

The parser MAY enforce configurable operational limits for:

CPU;

memory;

nesting;

token count;

diagnostic count.


Such limits MUST be implementation policies.

They MUST NOT become language semantic ceilings.

An implementation failure due to a resource budget MUST be distinguishable from invalid syntax.


---

107. Compatibility and denial-of-service protection

Compatibility infrastructure MUST be resistant to malicious or pathological input.

The implementation SHOULD use:

iterative processing where practical;

checked arithmetic;

bounded diagnostics;

configurable expansion budgets;

deterministic resource accounting;

fallible allocation where appropriate;

safe Rust.


No unsafe code is permitted merely to improve parser performance.


---

108. No parser-time hardware dependency

The parser MUST NOT:

discover a QPU;

query a GPU;

inspect CPU count;

read FPGA topology;

query calibration;

query runtime capabilities;

inspect network topology.


Those operations belong downstream.

This rule is both a compatibility and architecture invariant.


---

109. No filesystem/network compatibility dependency

Core grammar parsing MUST NOT require arbitrary:

filesystem access;

network access;

process execution.


If an explicit language feature requires external information, that dependency MUST be modeled as an effect/capability and handled outside pure grammar parsing.


---

110. Rust compatibility requirements

The implementation baseline is:

Rust 1.97 / Rust 1.97.1
Rust edition 2021

The grammar infrastructure MUST compile under the repository's supported Rust version.

The implementation MUST NOT require a newer Rust language or standard-library feature without an explicit repository-wide version change.


---

111. Safe Rust requirement

The compatibility implementation MUST use safe Rust.

Prohibited:

unsafe

unless the entire repository policy is explicitly changed and separately approved.

The grammar subsystem MUST NOT depend on unsafe for:

parser operation;

AST construction;

compatibility checking;

version resolution;

diagnostics;

serialization;

migration.



---

112. Checked arithmetic

Compatibility implementations MUST use checked arithmetic where values may originate from:

source files;

token counts;

source offsets;

collection lengths;

expansion counts;

resource expressions.


Integer overflow MUST NOT silently corrupt compatibility decisions.


---

113. No small-index assumptions

Compatibility infrastructure MUST NOT assume that:

files are small;

token streams are small;

ASTs are shallow;

module graphs are small;

identifiers are short;

generic nesting is shallow.


The implementation MAY have operational limits, but those limits MUST be explicit implementation policy.


---

114. Infinite scalability interpretation

"Scale to infinity" MUST be interpreted correctly.

Zamani MUST NOT claim that physical machines provide infinite resources.

Instead:

> Zamani MUST NOT impose arbitrary finite semantic ceilings where the underlying computation can conceptually scale.



Thus:

atom
↓
small machine
↓
large machine
↓
cluster
↓
supercomputer
↓
future architecture

is supported to the extent that:

the semantics remain valid;

the compiler can represent them;

the target provides the required resources;

the runtime can execute them.



---

115. No artificial finite language ceiling

The grammar MUST NOT contain arbitrary resource ceilings.

The following are prohibited as universal language rules:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_TENSOR_DIMENSION
MAX_DEVICES

This does not prohibit legitimate semantic constants.

For example:

u64

may be a language type.

That does not mean Zamani has only 64-bit values globally.


---

116. Explicit machine-specific semantics

A program MAY intentionally depend on a physical machine.

Such dependency MUST be explicit.

Examples include:

memory-mapped hardware;

physical qubit selection;

FPGA pins;

exact clock behavior;

device registers;

ABI-specific interfaces.


These constructs MUST be isolated from portable core semantics through explicit hardware/target boundaries.


---

117. Compatibility of hardware-bound programs

A hardware-bound program is compatible only with targets satisfying its declared requirements.

This is not a failure of POCO-REAF.

POCO-REAF means portable semantics where portability is semantically possible.

Intentional physical coupling is allowed when explicitly expressed.


---

118. Resource negotiation

Where a program can operate over a range of resources, the source SHOULD express that range.

For example:

requires capability(quantum);
requires qubits >= required_qubits;

rather than:

use device("specific-device");

when the latter is not semantically necessary.

The resource manager, compiler, scheduler, routing layer, hardware abstraction layer, and runtime may then negotiate the realization.


---

119. Compatibility under resource scaling

If a program's semantics do not depend on a fixed resource quantity, changing available resources MUST NOT change source compatibility.

Examples:

1 core → many cores
1 GPU → many GPUs
small memory → large memory
small cluster → large cluster
small QPU → larger QPU

The compiler MAY choose different implementations.

The source semantic contract remains unchanged.


---

120. Compatibility under target substitution

Replacing:

target A

with:

target B

MUST preserve semantics whenever both satisfy the program's declared requirements and constraints.

Differences in:

native instructions;

topology;

timing;

calibration;

resource counts


belong to target adaptation.


---

121. Compatibility under optimization

Any valid optimization MUST preserve the semantic contract.

An optimization that changes observable semantics MUST NOT be presented as a compatibility-preserving transformation.

Optimization correctness belongs to the optimization subsystem.


---

122. Compatibility under scheduling

Scheduling transformations MUST preserve the semantic ordering required by the program.

Where operations are explicitly unordered, different valid schedules MAY be selected.

Scheduling differences MUST NOT be confused with source incompatibility.


---

123. Compatibility under routing

Routing may introduce:

swaps;

movement;

remapping;

decomposition.


These are implementation transformations.

The logical program MUST remain semantically equivalent.


---

124. Compatibility under resilience

Resilience mechanisms MAY:

retry;

restart;

recover;

rollback;

remap;

reroute;

reschedule;

recompile;

reoptimize;

change QEC;

mitigate;

switch backend;

quarantine resources.


These actions MUST preserve semantic correctness according to the resilience contract.

Resilience MUST NOT redefine grammar compatibility.


---

125. Checkpoint compatibility

Checkpoint compatibility MUST distinguish:

classical execution state;

compiled program state;

logical quantum state;

measurement boundary;

QEC-supported state;

provider-supported reconstructible state.


The grammar MUST NOT imply that arbitrary unknown quantum states can always be serialized and restored.


---

126. Compatibility of runtime observations

Runtime observations such as:

fidelity;

error rate;

latency;

queue time;

hardware health;

calibration;

resource availability


MUST NOT alter the meaning of already parsed source.

They may influence execution strategy.


---

127. Security compatibility

Security semantics MUST remain stable.

Compatibility changes MUST NOT silently weaken:

permissions;

capability checks;

isolation;

cryptographic guarantees;

identity semantics;

privacy constraints.


A backend MAY have stronger security capabilities, but it MUST NOT silently weaken explicit source requirements.


---

128. Cryptographic compatibility

Cryptographic constructs MUST identify their semantic algorithm or protocol requirements.

Implementation/provider-specific algorithms MUST be explicitly namespaced.

A change in hardware acceleration MUST NOT change cryptographic semantics.


---

129. Reproducibility

Where deterministic compilation is promised, compatibility MUST include:

stable version resolution;

deterministic grammar interpretation;

deterministic AST construction;

deterministic semantic analysis;

deterministic canonicalization.


The compiler MUST NOT use uncontrolled:

randomization;

timestamps;

machine enumeration order;

network order;

hash-map iteration


for semantic decisions.


---

130. Build reproducibility

Build metadata SHOULD record:

language version;

grammar version;

dialect versions;

compiler version;

dependency versions;

target context;

feature configuration.


Reproducible builds SHOULD use deterministic inputs.


---

131. Generated grammar artifacts

Generated parser/lexer artifacts MUST be treated as derived outputs.

The authoritative source is the grammar specification.

Generated artifacts MUST identify or be reproducibly associated with:

grammar version;

generator version;

language version.


Generated artifacts MUST NOT become an independent source of syntax truth.


---

132. ANTLR compatibility

If ANTLR is used for Zamani grammar generation, grammar evolution MUST account for:

lexer precedence;

parser rule precedence;

labeled alternatives;

left recursion behavior;

generated parser compatibility;

complete input consumption;

generated-code version compatibility.


ANTLR generator upgrades MUST be tested as compatibility-impacting toolchain changes.


---

133. Generated parser versioning

A generated parser MUST NOT be committed or distributed as though it were authoritative syntax.

The repository SHOULD define whether generated artifacts are:

committed;

reproducibly generated;

CI-generated;

release-generated.


That policy belongs to grammar tooling documentation.


---

134. Grammar splitting compatibility

If grammar components are split into multiple files, each rule MUST have one authoritative owner.

The same production MUST NOT be independently defined in multiple files.

Splitting MUST NOT create:

A → B
B → A

or equivalent circular grammar ownership merely to separate domains.


---

135. Cross-domain compatibility

Zamani domains MUST compose without requiring mutually incompatible syntax models.

The following combinations MUST be considered:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

A domain extension MUST NOT make another domain's valid syntax ambiguous without an explicit versioned resolution.


---

136. Quantum + classical compatibility

Quantum-classical interaction MUST preserve explicit boundaries.

Classical values may control quantum operations.

Quantum measurements may produce classical values.

The grammar MUST express the relationship without assuming:

fixed qubit counts;

fixed classical register sizes;

fixed backend topology.



---

137. Quantum + hardware compatibility

Logical quantum semantics MUST be separable from physical hardware realization.

For example:

logical qubit

is not automatically:

physical qubit #7

Physical identity MUST require an explicit hardware-bound context.


---

138. Quantum + distributed compatibility

Distributed quantum programs MUST not assume a fixed number of QPUs or nodes.

Distributed quantum semantics MAY express:

remote operations;

entanglement resources;

communication requirements;

fault tolerance;

placement constraints.


Actual physical distribution belongs downstream.


---

139. Classical + HDL compatibility

Classical control software MAY operate hardware modules.

The interface between them MUST be explicit.

The grammar MUST distinguish:

software semantic
hardware semantic
hardware binding

so changing implementation hardware does not automatically invalidate software semantics.


---

140. AI + accelerator compatibility

AI programs MAY request accelerator capabilities.

They MUST NOT assume that:

accelerator capability

means a specific:

GPU;

TPU;

FPGA;

ASIC;

vendor device.


Backend selection belongs to compilation and resource management.


---

141. Future-domain compatibility

The grammar architecture MUST leave extension space for future domains.

New computing models SHOULD be introduced through:

domain namespaces;

dialects;

capability interfaces;

explicit semantic ownership.


Future extensions MUST NOT require changing unrelated core grammar rules unless necessary.


---

142. Reserved extension space

Reserved namespaces SHOULD be available for:

core
quantum
classical
hdl
hardware
distributed
ai
data
networking
security
future
vendor
experimental

Exact namespace syntax belongs to the core grammar.

The compatibility policy MUST prevent extension collisions.


---

143. Extension collision rules

Two extensions MUST NOT define the same semantic identifier without an explicit resolution mechanism.

Collision resolution MUST be:

deterministic;

explicit;

version-aware;

namespace-aware.


Implicit "last loaded extension wins" behavior is prohibited.


---

144. Feature negotiation

Feature negotiation MAY occur between:

compiler and target;

compiler and runtime;

runtime and backend;

module and dependency;

dialect and compiler.


It MUST NOT occur inside lexical analysis.

Feature negotiation MUST be explicit and deterministic.


---

145. Capability negotiation

Capability negotiation MUST be distinct from feature negotiation.

For example:

language supports quantum

is different from:

target provides quantum capability

The former is a language property.

The latter is an environment property.


---

146. Runtime feature negotiation

Runtime negotiation MAY select an implementation compatible with the program.

It MUST NOT silently change required semantics.

If no compatible implementation exists, the runtime MUST fail explicitly.


---

147. Graceful degradation

A program MAY declare optional capabilities or fallback implementations.

For example:

preferred accelerator
fallback classical

When the accelerator is unavailable, the compiler/runtime MAY select the fallback.

However, if the accelerator is semantically required, fallback is prohibited.


---

148. Optional versus required features

The compatibility model MUST distinguish:

requires
prefers
allows
supports
forbids

A missing preferred feature MUST NOT be treated as a semantic incompatibility.

A missing required feature MUST.


---

149. Compatibility and feature flags

Feature flags MUST be explicit.

Feature flags MUST NOT silently change:

operator meaning;

type meaning;

quantum measurement semantics;

resource semantics;

ownership semantics.


Feature flags that alter language semantics MUST be versioned.


---

150. Conditional compilation

Conditional compilation MUST be treated as implementation selection.

Conditions MAY inspect explicit compilation context.

They MUST NOT make source semantics dependent on arbitrary runtime hardware discovery.

The resulting branches MUST remain individually semantically valid.


---

151. Compatibility of conditional compilation

If conditional compilation produces different implementations, the language MUST define whether those implementations are:

semantically equivalent;

target-specialized;

intentionally different.


Silent semantic divergence is prohibited.


---

152. API evolution

Public grammar APIs SHOULD evolve additively where possible.

Breaking API changes MUST be versioned.

API compatibility MUST distinguish:

source syntax;

AST API;

parser API;

semantic API;

compiler API;

runtime API.



---

153. Test compatibility

Existing compatibility tests MUST remain meaningful across implementation refactors.

Tests SHOULD prefer semantic assertions over fragile internal representation assertions.

Snapshots MAY be used for:

diagnostics;

AST;

canonical IR


when those representations are intentionally stable.


---

154. Compatibility test categories

The grammar compatibility suite MUST include:

source compatibility
lexer compatibility
parser compatibility
AST compatibility
semantic compatibility
type compatibility
effect compatibility
capability compatibility
resource compatibility
dialect compatibility
migration compatibility
IR compatibility
serialization compatibility
interoperability compatibility
diagnostic compatibility
determinism compatibility
scalability compatibility


---

155. Positive compatibility tests

Tests MUST include valid programs from previous supported versions.

Examples MUST cover:

core syntax;

modules;

generics;

classical computation;

quantum computation;

hybrid computation;

HDL;

hardware;

distributed;

AI/data;

networking;

security;

interoperability.



---

156. Negative compatibility tests

Tests MUST verify that removed or unsupported constructs are rejected deterministically.

Examples include:

removed syntax;

unsupported dialect;

incompatible version;

unknown extension;

invalid migration;

unsupported ABI;

impossible target requirement.



---

157. Boundary compatibility tests

Boundary tests MUST include:

smallest valid program;

single element;

large collections;

deeply nested valid constructs;

large generic structures;

many modules;

many quantum operations;

large logical-qubit sets;

large hardware parameter sets;

large distributed configurations;

large tensor descriptions.


The tests MUST not accidentally establish those values as language limits.


---

158. Scalability compatibility tests

Compatibility testing MUST verify that no arbitrary source ceiling is introduced.

Tests SHOULD generate parameterized cases rather than enumerating a fixed maximum.

For example:

N = 1
N = small
N = large
N = implementation-budget boundary

The test framework MUST distinguish:

language semantic boundary

from:

test execution budget


---

159. Cross-version semantic tests

For a compatible language evolution:

old source
    ↓
old semantic representation

new parser
    ↓
new semantic representation

MUST be semantically equivalent.

Where direct structural equivalence is impossible, the test MUST compare observable semantic behavior or canonicalized semantic forms.


---

160. Migration tests

Every supported migration MUST have tests for:

representative old syntax;

edge cases;

invalid old syntax;

ambiguous cases;

semantic preservation;

diagnostics;

round-trip output.



---

161. Round-trip compatibility

Where serialization or pretty-printing is supported:

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

MUST preserve intended semantics.

For serialized artifacts:

object
 ↓
serialize
 ↓
deserialize

MUST preserve the versioned contract.


---

162. Determinism tests

The same input MUST produce the same:

tokens;

parse structure;

AST;

compatibility result;

diagnostics;

migration result.


Repeated runs MUST NOT depend on:

machine enumeration order;

random values;

timestamps;

network response order.



---

163. Fuzz compatibility tests

Fuzzing SHOULD cover:

lexer;

parser;

compatibility resolver;

migration;

serialization;

dialect loading.


The invariant is:

> malformed input may be rejected, but must not cause undefined behavior, memory unsafety, uncontrolled resource consumption, or nondeterministic semantic selection.




---

164. Grammar evolution procedure

Before changing an authoritative grammar rule:

1. identify the rule owner;


2. identify all consumers;


3. identify affected AST nodes;


4. identify semantic consumers;


5. identify dialects;


6. identify interoperability formats;


7. identify compatibility impact;


8. update version policy;


9. update migration policy;


10. update tests;


11. regenerate artifacts;


12. run cross-domain validation.



No grammar rule should be changed in isolation.


---

165. File ownership model

Every grammar file MUST have one primary owner.

Ownership means responsibility for:

syntax;

contract;

compatibility;

tests;

documentation;

integration.


Ownership does not imply that every downstream subsystem depends directly on the file.


---

166. Non-ownership is mandatory

Every domain grammar file MUST document what it does NOT own.

Examples:

quantum/gates.g4 does not own:

physical routing;

scheduling;

calibration;

QEC decoding;

noise;

runtime execution.


hardware/topology.g4 does not own:

generic quantum semantics;

routing algorithms;

scheduling algorithms.


resources/requirements.g4 does not own:

actual hardware discovery.


This prevents compatibility cycles.


---

167. Compatibility dependency graph

The intended dependency direction is:

specification
    ↓
lexer
    ↓
core
    ↓
types
    ↓
expressions
    ↓
statements
    ↓
declarations
    ↓
functions
    ↓
modules
    ↓
effects
    ↓
memory
    ↓
concurrency
    ↓
domain grammars
    ↓
resources
    ↓
compile
    ↓
execution
    ↓
interoperability
    ↓
dialects
    ↓
macros
    ↓
metaprogramming
    ↓
validation/tests

Validation documentation governs these layers but MUST NOT introduce reverse runtime dependencies.


---

168. Forbidden dependency cycles

The following are prohibited:

grammar → IR → grammar
grammar → runtime → grammar
grammar → hardware → grammar
grammar → scheduler → grammar
grammar → router → grammar
grammar → ZQN → grammar
grammar → QEC → grammar

The correct direction is:

grammar
  ↓
AST
  ↓
semantic analysis
  ↓
IR
  ↓
downstream systems


---

169. Quantum integration graph

The quantum compatibility boundary is:

grammar/quantum/
        ↓
frontend / parser AST
        ↓
semantic analysis
        ↓
src/quantum/ir
        ↓
src/quantum/optimization
        ↓
src/quantum/routing
        ↓
src/quantum/scheduling
        ↓
src/quantum/zqn
        ↓
src/quantum/hardware
        ↓
runtime

QEC and resilience integrate through their defined semantic/runtime boundaries.

The grammar MUST NOT bypass this architecture.


---

170. QEC integration

Grammar-level QEC intent flows toward QEC semantics.

The grammar does not implement:

decoders;

stabilizer measurement algorithms;

correction algorithms;

syndrome processing.


Compatibility of QEC implementation changes is downstream from grammar compatibility.


---

171. Scheduling integration

Scheduling receives semantic operations and target/resource context.

The grammar does not determine:

final schedule;

exact start times;

physical resource allocation;

scheduler algorithm.


Source timing constraints, where semantic, are inputs to scheduling rather than scheduler output.


---

172. Hardware integration

Hardware grammar describes explicit hardware semantics.

Hardware discovery belongs to the hardware abstraction layer.

The grammar MUST NOT inspect actual devices.


---

173. Resource integration

grammar/resources/ defines source-level resource language.

The resource subsystem consumes those declarations and evaluates them against capabilities.

Compatibility MUST preserve the distinction:

requirement
constraint
preference
hint
capability
target
resource


---

174. Compile integration

grammar/compile/ expresses compilation intent.

It MUST NOT embed backend implementation details that belong to compiler infrastructure.

Compiler changes SHOULD NOT require grammar changes unless they introduce or remove a source-visible semantic feature.


---

175. Execution integration

grammar/execution/ expresses execution/deployment intent where source-visible.

Actual runtime behavior belongs to the runtime.

Runtime compatibility failures MUST remain distinct from grammar failures.


---

176. Interoperability integration

grammar/interoperability/ owns syntax for interacting with external formats and ABIs.

Adapters MUST preserve Zamani semantic ownership.

External representations MUST NOT become alternative semantic authorities.


---

177. Dialect integration

Dialect definitions MUST identify compatibility with:

core language version;

dialect version;

required capabilities;

syntax extensions;

semantic extensions.


A dialect MUST declare whether it is:

stable;

experimental;

vendor-specific;

deprecated.



---

178. Example compatibility

Examples MUST demonstrate portable semantics.

Examples MUST NOT accidentally teach:

Zamani supports only 32 qubits

by always using exactly 32 qubits.

Examples that use a concrete resource count MUST label it as an example value.

Examples SHOULD demonstrate symbolic or parameterized scaling.


---

179. Documentation examples

Documentation MUST distinguish:

example value

from:

language limit

For example:

allocate 8 qubits

means one example program.

It MUST NOT imply:

maximum = 8


---

180. Compatibility and comments

Comments MUST NOT be used as a compatibility mechanism.

The parser MUST NOT transform unsupported semantics into comments.

Invalid or unsupported source MUST produce structured diagnostics.


---

181. Compatibility and formatting

Whitespace and formatting changes SHOULD NOT affect semantic compatibility except where the language explicitly defines layout-sensitive syntax.

Formatting tools MUST preserve semantics.


---

182. Compatibility and Unicode

Unicode policy MUST be explicitly versioned.

Changing normalization or identifier handling can be source-breaking and MUST undergo compatibility analysis.


---

183. Compatibility and localization

Human-language diagnostic translations MUST NOT alter machine-readable compatibility codes.

Localization changes MUST NOT alter parser or semantic behavior.


---

184. Compatibility and comments/docstrings

Changes to comments or documentation strings SHOULD NOT change semantics unless the language explicitly assigns semantic meaning to them.

If documentation strings are semantic metadata, they MUST have their own compatibility contract.


---

185. Compatibility and annotations

Annotations MUST identify whether they are:

semantic;

compiler-directed;

optimization hints;

resource declarations;

documentation metadata;

target-specific.


A target-specific annotation MUST NOT silently become a universal semantic requirement.


---

186. Compatibility and pragmas

Pragmas MUST be explicitly versioned where they affect semantics.

Undocumented pragmas are prohibited.

A pragma MUST NOT silently override core language compatibility rules.


---

187. Compatibility and metadata

Metadata MUST be distinguishable from executable semantics.

Metadata may include:

provenance;

source information;

compiler information;

documentation;

optimization hints.


Metadata changes SHOULD NOT change program semantics unless explicitly specified.


---

188. Compatibility and provenance

Compiled artifacts SHOULD retain provenance sufficient to establish:

source version;

dialects;

semantic model;

compiler version;

relevant transformations.


Provenance MUST NOT contain secrets unless explicitly protected by the relevant security policy.


---

189. Compatibility and secrets

Compatibility mechanisms MUST NOT require embedding:

credentials;

private keys;

access tokens;

passwords;

provider secrets


into source or grammar artifacts.


---

190. Provider neutrality

The core grammar MUST remain provider-neutral.

Provider-specific features belong under explicit:

dialect
hardware
interoperability
target

boundaries.

Provider names MUST NOT become universal semantic concepts without a language-level justification.


---

191. Device identity

Physical device identifiers MUST NOT be embedded into portable core syntax as implicit requirements.

Explicit hardware binding MAY use a device identity.

Such a program is then intentionally target-bound.


---

192. Topology compatibility

Topology belongs to hardware/routing/deployment contexts.

The grammar MUST NOT require a particular:

graph;

lattice;

bus;

interconnect;

coupling map;

network topology.


A topology requirement MUST be explicit if semantically necessary.


---

193. Timing compatibility

Timing is compatible when timing behavior is explicitly semantic.

Physical timing implementation belongs to:

hardware;

scheduling;

runtime;

target context.


A source duration MUST NOT automatically become a fixed hardware clock period.


---

194. Calibration compatibility

Calibration data MUST remain outside core grammar semantics.

Changing calibration MUST NOT make previously valid source syntax invalid.

Calibration may affect target compatibility or compilation.


---

195. Noise compatibility

Changing actual noise measurements MUST NOT change source grammar meaning.

Noise assumptions declared by a program are semantic requirements/models.

Observed noise belongs to ZQN/hardware/runtime.


---

196. Resource availability compatibility

A target may have:

insufficient resources

while the source remains fully compatible.

The correct behavior is explicit resource failure or alternative target selection.


---

197. Graceful target failure

When no target satisfies a valid program, the compiler SHOULD report:

program valid
requirements identified
available targets evaluated
no compatible target found

rather than:

invalid Zamani program


---

198. Future hardware compatibility

Future hardware SHOULD be integrated through:

capability descriptions;

target descriptions;

dialects;

compiler backends;

runtime adapters.


A new hardware architecture SHOULD NOT require changing existing portable source semantics.


---

199. Future language compatibility

Future language constructs SHOULD be introduced through explicit versioned grammar changes.

The core language SHOULD remain extensible without reserving every possible future keyword.


---

200. Compatibility invariants checklist

Every compatibility change MUST answer:

Does existing valid syntax still parse?

Does it parse the same way?

Does it mean the same thing?

Does the AST preserve required information?

Does semantic analysis preserve meaning?

Does canonical IR preserve meaning?

Are quantum semantics preserved?

Are hardware-independent semantics preserved?

Are resource semantics preserved?

Are dialect boundaries preserved?

Are migrations documented?

Are deprecated features handled?

Are diagnostics classified correctly?

Are generated artifacts regenerated?

Are cross-domain tests passing?

Are scalability guarantees preserved?

Are hard-coded limits absent?

Is deterministic behavior preserved?

Is Rust 1.97/1.97.1 compatibility preserved?

Is unsafe absent?



---

201. Per-file compatibility contract

Every grammar file MUST have the following documented contract before implementation:

File:
Purpose:
Owns:
Does Not Own:
Inputs:
Outputs:
Dependencies:
Upstream Contracts:
Downstream Consumers:
Public Grammar Contract:
AST Contract:
Semantic Contract:
IR Integration:
Compiler Integration:
Runtime Integration:
Tooling Integration:
Cross-Domain Integration:
Compatibility Contract:
Tests:
Negative Tests:
Boundary Tests:
Scalability Tests:
Determinism Tests:
Migration Requirements:
Hard-Coding Audit:
Completion Criteria:

A file is not complete until all fields have been resolved.


---

202. File independence

Completing one grammar file MUST NOT require later architectural changes to its fundamental ownership model.

Before implementation, its downstream integration MUST be known.

If later work discovers a genuine architectural contradiction, the repository MUST treat it as an explicit architecture change rather than silently reopening completed files.


---

203. Compatibility ownership by file

The following ownership model applies:

File	Primary compatibility responsibility

specification/language-version.md	Language version authority
specification/compatibility.md	High-level compatibility model
compatibility/versions.md	Supported versions
compatibility/migrations.md	Migration policy
compatibility/deprecated.md	Deprecation policy
compatibility/reserved.md	Reserved syntax
compatibility/compatibility-matrix.md	Compatibility relationships
validation/grammar-validation.md	Grammar correctness
validation/semantic-boundaries.md	Ownership boundaries
validation/ambiguity-rules.md	Deterministic interpretation
validation/scalability-rules.md	Scale invariants
validation/compatibility-rules.md	Compatibility invariants
validation/hardcoding-audit.md	Hard-code classification
validation/naming-rules.md	Naming rules


No two files may silently compete for the same normative responsibility.


---

204. Required compatibility matrix dimensions

The compatibility matrix SHOULD represent at least:

Language Version
Grammar Version
Dialect Version
AST Version
Semantic Model Version
IR Version
Compiler Version
Artifact Version
ABI Version
Runtime Version
Target Capability
Resource Requirements

It MUST NOT enumerate every possible hardware instance.


---

205. Compatibility status vocabulary

Compatibility evaluation SHOULD use structured statuses such as:

Compatible
ConditionallyCompatible
MigrationRequired
Unsupported
TargetIncompatible
ResourceUnavailable
CapabilityMissing
VersionConflict
DialectConflict
SemanticConflict
ABIConflict
RuntimeConflict
Unknown

The exact public API may use equivalent typed representations.


---

206. Unknown compatibility

Unknown MUST NOT be treated as compatible by default when correctness depends on the missing information.

Likewise, unknown MUST NOT automatically mean incompatible.

The system MUST distinguish:

known compatible
known incompatible
not yet evaluated

This is especially important for:

capabilities;

resources;

future dialects;

target support.



---

207. Compatibility confidence

Where compatibility information is uncertain, the system SHOULD represent confidence or provenance where appropriate.

An uncertain target capability MUST NOT be silently treated as guaranteed.

This aligns with the repository's broader resilience principle that uncertain state must not be acted upon as certain state.


---

208. Compatibility and resilience

Resilience MAY react to:

target failure;

resource loss;

backend changes;

runtime failures.


However, resilience MUST NOT modify language compatibility semantics.

The relationship is:

compatibility
    ↓
valid candidate implementations
    ↓
resilience
    ↓
recovery/adaptation


---

209. Compatibility and benchmarking

Benchmarking MUST NOT define language compatibility.

Benchmark results MAY inform:

preferences;

target selection;

optimization;

scheduling.


A benchmark result MUST NOT silently become a language semantic requirement.


---

210. Compatibility and performance

Performance constraints MUST be explicitly classified as:

semantic requirement;

constraint;

preference;

hint.


A performance regression is not automatically a language compatibility failure.


---

211. Compatibility and energy

Energy constraints MUST be explicit.

Changing target energy characteristics MUST NOT change the language semantics.

The compiler MAY select another implementation when a declared energy constraint permits it.


---

212. Compatibility and reliability

Reliability requirements MUST remain explicit.

For example:

requires reliability >= R

is different from:

preferred reliability >= R

The distinction MUST survive parsing and semantic analysis.


---

213. Compatibility and security constraints

Security constraints MUST be treated as correctness/security requirements where declared.

A target lacking a required security capability MUST fail target compatibility.

The compiler MUST NOT silently weaken the requirement.


---

214. Compatibility and portability annotations

Portability annotations SHOULD be explicit.

Possible categories include:

portable
target-bound
dialect-bound
vendor-bound
experimental

The exact syntax is owned by the core annotation system.


---

215. Compatibility and target profiles

Target profiles SHOULD be external to portable source where possible.

A target profile MAY describe:

capabilities;

resources;

topology;

instruction sets;

timing;

ABI;

deployment constraints.


The source program consumes the profile through compilation context rather than embedding the entire target description into grammar semantics.


---

216. Compatibility and compilation context

The compilation context is the boundary where target information becomes available.

It MAY contain:

LanguageVersion
DialectSet
TargetDescription
Capabilities
Resources
Constraints
Preferences
ABI
RuntimeRequirements

The grammar itself remains target-independent.


---

217. Compatibility and execution context

Execution context may contain dynamic information:

available resources
backend health
queue state
calibration
noise
latency
runtime capabilities

These MUST NOT alter already-defined source semantics.


---

218. Compatibility and deployment

Deployment compatibility MUST distinguish:

program compatibility;

target compatibility;

placement compatibility;

network compatibility;

security compatibility;

runtime compatibility.


Deployment failures MUST be reported at the appropriate layer.


---

219. Compatibility and scaling down

A portable program SHOULD scale down when its semantic requirements can be satisfied by a smaller target.

If the program explicitly requires more resources than the smaller target provides, execution may fail.

That is a resource constraint, not a grammar incompatibility.


---

220. Compatibility and scaling up

A portable program SHOULD scale up without source rewriting where semantics permit.

Increasing:

qubits;

cores;

threads;

nodes;

memory;

accelerators


MUST NOT require source changes merely because more resources are available.


---

221. Compatibility and elastic execution

Programs MAY be designed for elastic resource environments.

Resource expressions SHOULD support:

minimum;

maximum;

preferred;

unbounded;

symbolic;

negotiated.


"Unbounded" means no source-imposed upper requirement, not physically infinite resources.


---

222. Compatibility and streaming

Streaming constructs SHOULD remain compatible across different buffer sizes and execution environments.

A runtime MAY select different buffering strategies.

The source semantics remain stable.


---

223. Compatibility and lazy computation

Lazy computation MUST define evaluation semantics independently of target resources.

The runtime MAY choose:

eager;

lazy;

fused;

distributed


implementations where semantics permit.


---

224. Compatibility and data partitioning

Partitioning MAY vary by target.

A source-level distributed algorithm MUST NOT require a fixed partition count unless partition count is semantically meaningful.


---

225. Compatibility and tensor distribution

Tensor partitioning MAY change with accelerator count.

The source semantic tensor MUST remain stable.

Physical sharding belongs to compilation/runtime.


---

226. Compatibility and accelerator substitution

A supported accelerator abstraction MUST allow implementation substitution.

For example:

accelerator capability

may map to:

CPU
GPU
FPGA
ASIC
quantum accelerator
future accelerator

where the semantics permit.


---

227. Compatibility and simulator substitution

A quantum program MAY execute on:

hardware;

simulator;

emulator;

hybrid backend.


The execution model MUST identify whether results are exact, approximate, sampled, or hardware-observed.

Changing simulator/backend MUST NOT alter core quantum semantics.


---

228. Compatibility and probabilistic semantics

Where quantum or probabilistic behavior is semantically nondeterministic, compatibility MUST define the allowed probability distribution or observable behavior.

Different valid executions MAY produce different samples.

That is not necessarily incompatibility.


---

229. Compatibility and numerical semantics

Floating-point implementation differences MAY exist across targets.

The language MUST define where numerical equivalence is exact versus approximate.

If exact numerical behavior is required, the type or semantic contract MUST express it.


---

230. Compatibility and undefined behavior

The language MUST clearly define any undefined or implementation-dependent behavior.

Such behavior MUST NOT be used to hide compatibility failures.

Where portability is a core goal, implementation-defined behavior SHOULD be minimized.


---

231. Compatibility and implementation-defined behavior

Implementation-defined behavior MUST be:

documented;

discoverable;

deterministic for a given implementation;

isolated from portable semantics where possible.



---

232. Compatibility and unspecified behavior

Unspecified behavior MUST have an explicitly defined allowed set.

A compiler may choose any allowed behavior.

Different choices are compatible if they remain within the language specification.


---

233. Compatibility and optimization freedom

The language SHOULD grant implementations freedom to optimize while preserving observable semantics.

This is essential for POCO-REAF.

A source program SHOULD NOT prescribe:

exact instructions;

exact gate decomposition;

exact scheduling;

exact physical placement


unless intentionally target-bound.


---

234. Compatibility and semantic preservation

The strongest invariant is:

same source intent
+
valid requirements
+
compatible target
=
same program semantics

Implementation details MAY differ.


---

235. Compatibility review rule

Every proposed language change MUST answer:

What source programs can change?
What ASTs can change?
What semantic meanings can change?
What dialects can change?
What artifacts can change?
What runtimes can change?
What targets can change?
What migrations are required?

No change is production-ready until these questions are answered.


---

236. Compatibility change classification

Every change MUST be classified as one or more of:

NoCompatibilityImpact
LexicalCompatible
ParserCompatible
ASTCompatible
SemanticCompatible
Additive
Deprecated
MigrationRequired
SourceBreaking
SemanticBreaking
IRBreaking
ABIChanging
RuntimeChanging
TargetOnly
DialectOnly
ToolingOnly

The classification MUST be recorded in the relevant change documentation.


---

237. Compatibility gates

CI MUST reject changes that:

break supported source without version justification;

introduce accidental parser ambiguity;

introduce undocumented keywords;

alter semantic meaning without classification;

add hidden machine limits;

break canonical IR boundaries;

introduce circular dependencies;

require unsafe;

violate Rust 1.97/1.97.1 support;

remove required migrations;

make compatibility resolution nondeterministic.



---

238. Release compatibility gate

Before a grammar release:

1. grammar validation passes;


2. ambiguity validation passes;


3. scalability validation passes;


4. compatibility suite passes;


5. migration tests pass;


6. cross-domain tests pass;


7. determinism tests pass;


8. generated artifacts are reproducible;


9. hard-coding audit passes;


10. documentation is synchronized.




---

239. Compatibility audit checklist

Before marking this file complete, verify:

Language

[ ] language version ownership is defined;

[ ] major/minor/patch rules are defined;

[ ] backward compatibility is defined;

[ ] forward compatibility is defined;

[ ] deprecation is defined;

[ ] migration is defined.


Grammar

[ ] lexer compatibility is defined;

[ ] parser compatibility is defined;

[ ] entry-point completeness is defined;

[ ] ambiguity interaction is defined;

[ ] keyword compatibility is defined;

[ ] identifier compatibility is defined.


Semantics

[ ] semantic compatibility is defined;

[ ] AST compatibility is defined;

[ ] type compatibility is defined;

[ ] effect compatibility is defined;

[ ] capability compatibility is defined;

[ ] resource compatibility is defined.


Quantum

[ ] no fixed qubit ceiling exists;

[ ] logical/physical distinction is preserved;

[ ] quantum::ir remains canonical;

[ ] QEC ownership is preserved;

[ ] ZQN ownership is preserved;

[ ] routing/scheduling ownership is preserved.


Hardware

[ ] no fixed machine size exists;

[ ] no fixed topology exists;

[ ] target binding is explicit;

[ ] hardware discovery remains downstream.


Scalability

[ ] no arbitrary resource ceilings exist;

[ ] implementation limits are distinguished from language limits;

[ ] operational parser budgets are distinct from semantics;

[ ] "infinity" is correctly qualified.


Security

[ ] no unsafe;

[ ] no secret embedding;

[ ] no arbitrary parser filesystem access;

[ ] no arbitrary parser network access;

[ ] metaprogramming budgets exist where necessary.


Tooling

[ ] generated grammar policy is defined;

[ ] deterministic generation is required;

[ ] diagnostics are versioned appropriately.


Repository

[ ] specification files are synchronized;

[ ] compatibility files are synchronized;

[ ] validation files are synchronized;

[ ] domain grammars have explicit ownership;

[ ] compiler/runtime integration is defined;

[ ] interoperability integration is defined.



---

240. Completion criteria

grammar/validation/compatibility-rules.md is complete only when:

1. all compatibility dimensions are explicitly classified;


2. language version authority is identified;


3. grammar authority is identified;


4. source compatibility is defined;


5. semantic compatibility is defined;


6. AST compatibility is defined;


7. IR compatibility is defined;


8. ABI compatibility is separated;


9. runtime compatibility is separated;


10. target compatibility is separated;


11. resource compatibility is separated;


12. dialect compatibility is defined;


13. migration policy is defined;


14. deprecation policy is defined;


15. reserved syntax policy is defined;


16. deterministic parsing is required;


17. complete input consumption is required;


18. ambiguity interactions are defined;


19. quantum compatibility is defined;


20. quantum::ir remains the canonical quantum boundary;


21. QEC is not duplicated;


22. ZQN is not duplicated;


23. routing is not duplicated;


24. scheduling is not duplicated;


25. hardware discovery is not duplicated;


26. resilience is not duplicated;


27. scalability rules are preserved;


28. no arbitrary machine limits are introduced;


29. target-specific behavior is explicitly separated;


30. cross-domain compatibility is addressed;


31. interoperability compatibility is addressed;


32. macro/metaprogramming compatibility is addressed;


33. security compatibility is addressed;


34. Rust 1.97/1.97.1 compatibility is defined;


35. unsafe is prohibited;


36. deterministic and safe resource handling is required;


37. compatibility diagnostics are layered;


38. tests cover positive, negative, boundary, scalability, determinism, migration, and cross-domain behavior;


39. CI/release compatibility gates are defined;


40. every relevant grammar file has an integration contract.




---

241. Final architectural invariant

The Zamani compatibility architecture MUST preserve the following:

ZAMANI SOURCE
                         │
                         ▼
                    LANGUAGE SYNTAX
                         │
                         ▼
                        AST
                         │
                         ▼
                 SEMANTIC ANALYSIS
                         │
                         ▼
              CANONICAL SEMANTIC MODEL
                         │
              ┌──────────┴──────────┐
              ▼                     ▼
       Classical semantics    quantum::ir
              │                     │
              └──────────┬──────────┘
                         ▼
                    COMPILATION
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
       Routing       Scheduling      Optimization
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                 Target / Hardware
                         │
                         ▼
                      Runtime
                         │
                         ▼
                    Execution

Compatibility MUST preserve meaning across this architecture.


---

242. Final POCO-REAF principle

The ultimate compatibility invariant is:

ONE PROGRAM
    ↓
ONE DEFINED SEMANTIC MEANING
    ↓
MANY COMPILATION STRATEGIES
    ↓
MANY TARGETS
    ↓
MANY ARCHITECTURES
    ↓
MANY HARDWARE CONFIGURATIONS
    ↓
MANY SCALES
    ↓
MANY EXECUTION ENVIRONMENTS
    ↓
FUTURE IMPLEMENTATIONS

Therefore:

> Zamani compatibility is semantic compatibility first, representation compatibility second, and hardware compatibility only where explicitly required.



A change in machine size MUST NOT be a language-version change.

A change in device MUST NOT be a grammar change.

A change in topology MUST NOT be a source-semantic change.

A change in calibration MUST NOT change grammar meaning.

A change in scheduler MUST NOT redefine syntax.

A change in QEC implementation MUST NOT redefine the language.

A change in ZQN noise modeling MUST NOT redefine the grammar.

A change in optimization MUST preserve semantics.

A change in runtime MUST remain behind the execution boundary.

A future machine MUST be able to consume existing Zamani semantics through a new compilation/target path without requiring the language to encode that machine's temporary physical properties.


---

243. Final statement

Zamani MUST therefore follow:

Program Once
      ↓
Stable Semantics
      ↓
Compile Once
      ↓
Portable Representation
      ↓
Adapt to Available Capabilities
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Remain Semantically Evolvable Forever

subject only to:

available resources
+
declared requirements
+
declared constraints
+
supported semantics
+
target capabilities
+
implementation feasibility

and never to an arbitrary language-imposed machine ceiling.

Zamani: From Atom to Everywhere.

POCO-REAF: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.

### Integration position

This file should be treated as the **normative compatibility validation layer**, not as another grammar source. Its strongest dependencies are:

```text
specification/language-version.md
specification/compatibility.md
specification/scalability-model.md
        ↓
compatibility/*
        ↓
validation/semantic-boundaries.md
validation/ambiguity-rules.md
validation/scalability-rules.md
validation/hardcoding-audit.md
        ↓
all grammar domains
        ↓
AST / semantic analysis
        ↓
canonical IR
        ↓
compiler / runtime / target systems

In particular, it deliberately prevents compatibility rules from becoming a back door for hard-coded qubit/core/device/node limits, while preserving the existing quantum::ir ownership boundary.