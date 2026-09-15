Zamani Compatibility Rules

Path: "grammar/validation/compatibility-rules.md"
Status: Normative production validation contract
Language: Zamani
Compiler baseline: Rust 1.97 / Rust 1.97.1
Safety: "unsafe" Rust is prohibited
Primary portability objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

This document defines the production compatibility-validation rules for the Zamani language grammar and every representation derived from it.

This file does not define an independent Zamani language.

It validates conformance between the authoritative language specification and:

- lexical implementations;
- ANTLR grammar;
- Rust lexer;
- Rust parser;
- AST;
- name resolution;
- type checking;
- effect checking;
- capability checking;
- resource validation;
- canonical semantic representations;
- "quantum::ir";
- optimization;
- routing;
- scheduling;
- resilience;
- QEC;
- ZQN;
- hardware abstraction;
- compilation;
- runtime;
- interoperability;
- serialization;
- diagnostics;
- tooling;
- version migration.

The fundamental invariant is:

One Zamani language
        |
        +--> specification
        |
        +--> lexer
        |
        +--> parser
        |
        +--> AST
        |
        +--> semantic analysis
        |
        +--> canonical IR
        |
        +--> target-independent compilation
        |
        +--> target realization
        |
        +--> execution

No implementation layer may silently create a second interpretation of Zamani.

---

2. Authority and Ownership

Compatibility validation MUST distinguish language authority from implementation artifacts.

The following ownership model applies.

Artifact| Authority
"grammar/specification/*"| normative language architecture and contracts
"grammar/spec/lexical.md"| normative lexical contract
"grammar/spec/syntax.md"| normative syntax contract
"grammar/spec/semantics.md"| normative semantic contract
"grammar/spec/compatibility.md"| normative compatibility model
"grammar/specification/language-version.md"| language-version authority
"grammar/Zamani.g4"| canonical ANTLR syntax representation after reconciliation
"src/lexer.rs"| executable Rust lexical implementation
"src/parser.rs"| executable Rust parser implementation
"src/ast/*"| frontend structural representation
semantic-analysis implementation| executable semantic validation
canonical IR| executable semantic representation
"quantum::ir"| canonical quantum semantic boundary
"grammar/grammar.md"| implementation/conformance reference
"grammar/Zamani-Grammar.md"| broad language/design documentation unless formally promoted
generated parser artifacts| derived artifacts; never authority
examples| conformance fixtures; never authority

No generated file may become an independent source of truth.

No README may introduce syntax that is absent from the normative specification.

No domain README may independently redefine a language construct.

---

3. Core Rule: One Language

Zamani MUST remain one language.

The following are not independent languages:

- classical Zamani;
- quantum Zamani;
- hybrid Zamani;
- HDL Zamani;
- distributed Zamani;
- AI Zamani;
- accelerator Zamani;
- embedded Zamani;
- mathematical Zamani.

They are semantic domains of the same language.

Domain grammars MAY partition implementation responsibilities, but they MUST compose into one language contract.

A construct such as:

quantum

does not create a second language.

Likewise:

hardware

does not create a separate HDL language unless an explicitly versioned external dialect is being parsed.

---

4. No Competing Grammar Authorities

The repository MUST NOT contain competing definitions of the same syntax.

The following relationship is mandatory:

Normative specification
        |
        +--> Zamani.g4
        |
        +--> Rust lexer/parser
        |
        +--> AST
        |
        +--> tests

"grammar/Zamani.g4" MUST NOT silently disagree with:

- "grammar/spec/syntax.md";
- "grammar/spec/lexical.md";
- "src/lexer.rs";
- "src/parser.rs";
- AST structures.

If disagreement is discovered, compatibility validation MUST classify it before implementation proceeds.

Allowed classifications:

SPECIFICATION_BUG
ANTLR_BUG
RUST_FRONTEND_BUG
AST_BUG
SEMANTIC_BUG
TEST_BUG
DOCUMENTATION_DRIFT
UNIMPLEMENTED_FEATURE
DEPRECATED_FEATURE
RESERVED_FEATURE

No disagreement may be resolved merely by making one implementation accept more syntax.

---

5. Validation Does Not Invent Semantics

This file MUST NOT define new language semantics.

It validates whether existing semantic contracts are consistently represented.

For example, this file MAY state:

«A quantum operation accepted by the parser must not disappear before the canonical quantum semantic boundary.»

It MUST NOT define a new meaning for that operation.

Semantic ownership remains with the language semantic specification and canonical semantic/IR layers.

---

6. Compatibility Dimensions

Compatibility MUST be evaluated independently across the following dimensions:

1. source compatibility;
2. encoding compatibility;
3. lexical compatibility;
4. token compatibility;
5. grammar compatibility;
6. parser compatibility;
7. AST compatibility;
8. source-location compatibility;
9. name-resolution compatibility;
10. module compatibility;
11. type compatibility;
12. generic compatibility;
13. ownership compatibility;
14. effect compatibility;
15. capability compatibility;
16. resource compatibility;
17. quantum semantic compatibility;
18. classical semantic compatibility;
19. HDL semantic compatibility;
20. hardware-target compatibility;
21. dialect compatibility;
22. macro compatibility;
23. metaprogramming compatibility;
24. canonical IR compatibility;
25. serialization compatibility;
26. compiled-artifact compatibility;
27. ABI compatibility;
28. runtime compatibility;
29. deployment compatibility;
30. interoperability compatibility;
31. diagnostic compatibility;
32. determinism compatibility;
33. reproducibility compatibility;
34. migration compatibility.

A failure in one dimension MUST NOT be reported as another.

For example:

Program requires 128 qubits.
Target provides 64.

is not a grammar incompatibility.

It is a target/resource compatibility failure.

Likewise:

Program uses syntax introduced in Zamani 2.0.
Compiler is compiling as Zamani 1.x.

is a language-version compatibility failure, not a hardware failure.

---

7. Compatibility Versus Availability

Compatibility and availability MUST remain separate.

A program may be:

syntactically valid
semantically valid
resource-valid in principle
target-compatible in principle
currently unavailable

For example:

requires capability(quantum);

may be completely valid even when no quantum backend is currently available.

The compiler MUST NOT rewrite the source merely because a selected target is unavailable.

The result MUST instead identify the failed boundary:

Language: compatible
Syntax: valid
Semantics: valid
Capability: required
Target availability: unavailable

---

8. Compatibility Versus Optimization

Optimization MUST NOT be used to justify semantic changes.

The following are compatible:

source
  ↓
canonical semantic meaning
  ↓
optimization
  ↓
equivalent implementation

The following is prohibited:

source
  ↓
optimization
  ↓
different computation

Unless the language explicitly defines the operation as approximate, nondeterministic, probabilistic, or otherwise semantically qualified.

Optimization MUST preserve all semantics that the source program promises to preserve.

This includes, where applicable:

- classical values;
- quantum behavior;
- measurement semantics;
- effects;
- resource requirements;
- ordering;
- synchronization;
- observable timing constraints;
- numerical guarantees;
- precision requirements;
- security properties;
- ownership;
- externally observable behavior.

---

9. POCO-REAF

Compatibility MUST support:

Program Once
      ↓
Stable Semantic Meaning
      ↓
Compile Once
      ↓
Portable Compilation Representation
      ↓
Target Adaptation
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

POCO-REAF does not mean that one historical binary must execute unchanged on every future machine.

Instead:

source semantics
+
versioned representation
+
migration rules
+
target-independent meaning

MUST provide a durable path for future implementations.

Therefore:

semantic stability != binary immortality

and:

source portability != identical machine representation

A future machine MAY require:

- new lowering;
- new optimization;
- new scheduling;
- new routing;
- new resource allocation;
- new ABI;
- new runtime;
- new hardware realization.

It MUST NOT require rewriting the program's semantic intent merely because the machine changed.

---

10. Version Authority

There MUST be one authoritative language-version model.

The authority is:

grammar/specification/language-version.md

The following MUST consume that model rather than inventing their own:

grammar/compatibility/versions.md
grammar/validation/compatibility-rules.md
grammar/dialects/versioning.g4
grammar/Zamani.g4
src/lexer.rs
src/parser.rs
AST
semantic analysis
compiler
tooling

No individual grammar file may introduce a new language version.

No backend may define language-version semantics.

No hardware backend may determine source-language compatibility.

---

11. Version Identity

Language versions MUST be machine-readable.

A version MUST be capable of identifying:

- language major version;
- language minor version;
- compatibility-relevant revision;
- active dialect where applicable;
- feature profile where applicable.

Version comparison MUST NOT depend exclusively on arbitrary strings.

Version information MUST be parsed structurally.

---

12. Major-Version Compatibility

A major-version transition MAY introduce breaking changes.

Breaking changes include:

- changing stable syntax meaning;
- removing stable syntax;
- changing operator precedence;
- changing type meaning;
- changing ownership rules;
- changing effect semantics;
- changing quantum measurement semantics;
- changing resource semantics;
- changing module resolution;
- changing macro hygiene;
- changing canonical semantic meaning.

A major release MUST provide:

- migration documentation;
- compatibility classification;
- affected-feature inventory;
- test migration;
- diagnostic migration;
- AST/IR migration where required.

---

13. Minor-Version Compatibility

Minor releases SHOULD preserve existing stable source semantics.

Additive features SHOULD use:

- contextual keywords;
- explicit namespaces;
- attributes;
- compositional syntax;
- version gates where needed.

A new feature MUST NOT accidentally reinterpret an existing valid program.

---

14. Patch-Version Compatibility

Patch releases MUST NOT intentionally change the semantics of valid stable programs.

Patch releases MAY improve:

- diagnostics;
- parser performance;
- error recovery;
- memory behavior;
- implementation correctness;
- optimization;
- internal representation;
- tooling;
- documentation.

Patch releases MUST NOT silently alter:

- precedence;
- associativity;
- type meaning;
- ownership;
- effects;
- resource semantics;
- quantum semantics;
- HDL semantics;
- hardware semantics.

---

15. Source Compatibility

A source program is source-compatible with a language version when:

1. source encoding is valid;
2. lexing succeeds;
3. parsing succeeds;
4. the syntax is valid for that version;
5. referenced features are available;
6. semantic rules are satisfied;
7. required dialects are available;
8. required language capabilities are available.

Source compatibility MUST be evaluated independently from target hardware availability.

---

16. Backward Compatibility

When a release claims backward compatibility:

Program P
Language version V

MUST retain its intended semantics under the compatible compiler implementation.

Validation MUST include:

- existing examples;
- positive fixtures;
- negative fixtures;
- AST comparisons where stable;
- semantic equivalence;
- canonical IR comparison where appropriate;
- round-trip tests;
- diagnostic compatibility where diagnostics are contractual.

A parser MUST NOT continue accepting old syntax while silently changing its meaning.

---

17. Forward Compatibility

Future evolution MUST have explicit extension mechanisms.

Supported mechanisms MAY include:

- language versions;
- dialect namespaces;
- feature namespaces;
- attributes;
- reserved keywords;
- reserved punctuation;
- extension declarations;
- versioned metadata;
- structured unknown-extension handling.

Unknown syntax MUST NOT silently become known syntax.

Unknown syntax MUST NOT be converted into a no-op.

Unknown syntax MUST NOT be discarded.

Unknown syntax MUST NOT be guessed.

---

18. Unknown Syntax

Production compilation MUST reject unsupported syntax deterministically.

The compiler MUST NOT transform:

unknown construct

into:

comment

or:

no-op

or:

best guess

unless the language specification explicitly defines that behavior.

Unsupported syntax MUST result in a structured diagnostic containing, where available:

- source location;
- construct identity;
- language version;
- feature status;
- reason;
- migration/replacement guidance.

---

19. Feature Lifecycle

Every language feature MUST have one lifecycle status:

PROPOSED
SPECIFIED
IMPLEMENTED
STABLE
EXPERIMENTAL
DEPRECATED
REMOVED
RESERVED

These statuses MUST NOT be conflated.

PROPOSED

Design exists but is not normative.

SPECIFIED

Normative language contract exists but implementation is incomplete.

IMPLEMENTED

Reference implementation accepts and represents the feature but stability is not yet guaranteed.

STABLE

Feature is production-supported.

EXPERIMENTAL

Feature is explicitly versioned or feature-gated.

DEPRECATED

Feature remains supported but has a documented replacement.

REMOVED

Feature is no longer accepted.

RESERVED

Spelling or syntax is reserved but not implemented.

---

20. Deprecation

Deprecation MUST specify:

- feature name;
- introduction version;
- deprecation version;
- replacement;
- migration guidance;
- expected removal version or policy;
- compatibility impact.

Deprecation MUST NOT silently change semantics.

A deprecated construct remains semantically stable until removal unless the specification explicitly states otherwise.

---

21. Removal

A stable construct MAY be removed only when:

1. the language-version policy permits removal;
2. affected consumers have been identified;
3. migration guidance exists;
4. tests have been migrated;
5. parser artifacts are regenerated;
6. documentation is updated;
7. compatibility fixtures are updated;
8. dialect dependencies have been checked;
9. AST and IR migration is complete.

Silent removal is prohibited.

---

22. Reserved Syntax

Reserved syntax is not implemented syntax.

Reserved words MUST NOT be interpreted as implemented constructs.

Reserved syntax exists to preserve future evolution space.

Reservation MUST NOT create machine-size limits.

The following is valid reservation:

future quantum operation namespace

The following is not acceptable scalability architecture:

q0
q1
...
q63

as the complete reserved quantum space.

---

23. Keyword Compatibility

A new keyword can break existing programs that previously used that word as an identifier.

Therefore new keywords SHOULD preferably be:

- contextual;
- namespace-qualified;
- version-gated;
- explicitly escaped;
- introduced through compositional syntax.

The keyword registry MUST be canonical.

No lexer, parser, domain grammar, or backend may independently introduce a keyword.

---

24. Identifier Compatibility

Identifier compatibility includes:

- character set;
- Unicode policy;
- normalization;
- case sensitivity;
- escape rules;
- reserved-word interaction;
- namespace qualification.

Identifiers MUST NOT implicitly identify physical hardware.

For example:

gpu7
qpu3
core15
device42

are ordinary identifiers unless an explicit hardware/resource construct gives them physical meaning.

The language MUST NOT infer:

gpu7 == physical GPU 7

without an explicit semantic boundary.

---

25. Numeric Literal Compatibility

Numeric literal syntax MUST remain independent of host machine width.

The lexer MUST NOT truncate literals to host types.

For example:

999999999999999999999999999999999999999999

MUST be representable lexically if valid under the language's numeric grammar.

Type checking later determines whether it fits a requested semantic type.

Overflow MUST produce a diagnostic.

It MUST NOT silently wrap.

The existence of types such as:

u8
u16
u32
u64
u128
usize

does not impose a language-wide ceiling.

They are semantic types, not language capacity limits.

---

26. Source Encoding

The canonical source encoding is UTF-8.

Invalid UTF-8 MUST produce a deterministic diagnostic.

Host encoding MUST NOT change program meaning.

Source spans MUST be stable with respect to the canonical source representation.

---

27. Lexer Compatibility

The Rust lexer and ANTLR lexer MUST agree on:

- token boundaries;
- identifiers;
- keywords;
- literals;
- punctuation;
- operators;
- comments;
- source spans;
- invalid-input behavior.

The lexer MUST be deterministic.

The lexer MUST NOT depend on:

- hardware;
- runtime state;
- target discovery;
- calibration;
- random values;
- network state;
- filesystem contents.

Domain names that do not require lexical distinction SHOULD remain identifiers.

In particular, quantum gate names MUST NOT be an artificially closed lexer keyword list.

---

28. Parser Compatibility

The parser MUST:

- produce canonical AST structures;
- preserve semantic information;
- preserve source locations;
- reject unsupported constructs;
- recover deterministically where recovery is supported;
- make progress after recovery;
- avoid infinite loops;
- avoid silent semantic loss;
- use safe Rust;
- avoid fixed machine-size assumptions.

Parser behavior MUST be independent of target hardware.

The parser MUST NOT ask:

How many qubits does this QPU have?

to decide whether syntax is valid.

---

29. No Parser-Imposed Machine Ceiling

The grammar and parser MUST NOT define production ceilings such as:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_DEVICES
MAX_NODES
MAX_MEMORY
MAX_TENSORS
MAX_TENSOR_RANK
MAX_CIRCUIT_DEPTH

as language compatibility limits.

Implementation safety limits MAY exist where required to prevent denial-of-service or resource exhaustion.

Such limits MUST be:

- explicit;
- configurable where appropriate;
- classified as implementation limits;
- separate from language semantics;
- reported as resource/execution failures;
- not encoded into the grammar as language meaning.

---

30. Deep Source and Scalability

Zamani MUST scale from tiny programs to programs limited only by available resources and representational constraints.

Grammar validation MUST test:

- tiny source units;
- large source units;
- deeply nested expressions;
- deeply nested blocks;
- large module graphs;
- large generic structures;
- large quantum programs;
- large hardware descriptions;
- large distributed descriptions.

Where recursive parsing could exhaust the host call stack, the implementation SHOULD use explicit worklists/stacks or another safe strategy.

No "unsafe" Rust may be introduced to achieve scalability.

---

31. AST Compatibility

Every successfully parsed semantic construct MUST have a corresponding AST representation.

This invariant is mandatory:

accepted syntax
      ↓
AST

No parser rule may accept syntax that is impossible to represent.

AST structures MUST preserve, where semantically relevant:

- source span;
- identifier;
- modifiers;
- generic parameters;
- type expressions;
- effects;
- resource requirements;
- capabilities;
- quantum intent;
- hardware intent;
- timing intent;
- security intent;
- domain metadata.

Parser-specific temporary structures MUST NOT become semantic meaning.

---

32. No Phantom Syntax

The following is prohibited:

parser accepts
        ↓
AST drops information
        ↓
semantic layer guesses

Likewise:

parser accepts
        ↓
AST stores partial construct
        ↓
IR generator ignores missing portion

A construct is not production-ready merely because the parser accepts it.

It is production-ready only when the full required semantic pipeline exists.

---

33. No Silent Semantic Loss

The following are prohibited:

- dropped quantum modifiers;
- dropped gate parameters;
- dropped measurement semantics;
- dropped effects;
- dropped resource requirements;
- dropped capabilities;
- dropped timing constraints;
- dropped security properties;
- dropped ownership information;
- dropped source locations needed by tooling;
- unknown operations emitted as comments;
- unsupported operations emitted as no-ops;
- hardware requirements silently ignored.

Every semantically relevant property MUST either:

1. survive to the layer that owns it; or
2. cause a structured unsupported-feature diagnostic.

---

34. Semantic Compatibility

Semantic compatibility means:

same source
+
same language version
+
compatible implementation
=
same intended program meaning

Compiler optimization, target selection, routing, scheduling, or deployment MUST NOT redefine the source semantics.

Semantic changes MUST be language-versioned.

---

35. Type Compatibility

Type compatibility MUST preserve:

- type identity;
- generic parameters;
- constraints;
- variance where defined;
- ownership;
- lifetime semantics where defined;
- effect annotations;
- resource annotations;
- quantum type identity;
- hardware/resource type identity.

Surface syntax MAY evolve while lowering to one canonical type model.

The repository MUST NOT create multiple incompatible representations of the same semantic type merely because they originated in different domains.

---

36. Effect Compatibility

Effects MUST remain semantically explicit.

An effect declaration MUST survive:

source
 ↓
AST
 ↓
semantic analysis
 ↓
canonical representation

An effect MUST NOT be parsed and then silently discarded.

Hardware, quantum, network, distributed, security, and I/O effects MUST remain distinguishable where their semantic guarantees differ.

---

37. Capability Compatibility

Capabilities describe what an execution environment can provide.

A capability is not a machine identity.

For example:

capability(quantum)
capability(gpu)
capability(fpga)
capability(distributed)

does not mean:

device = X

Capability checking MUST occur after syntax parsing.

A missing capability MUST NOT make otherwise valid syntax invalid.

---

38. Resource Compatibility

Resource requirements MUST be represented separately from:

- syntax;
- language version;
- hardware identity;
- capability identity.

Examples include:

requires qubits >= n
requires memory >= amount
requires accelerator(kind)
requires bandwidth >= amount
requires latency <= amount
requires energy <= amount

These are resource semantics.

They are not parser limits.

---

39. No Compatibility Ceilings

The following MUST NOT occur in grammar compatibility logic:

MAX_QUBITS = 64
MAX_CORES = 256
MAX_THREADS = 1024
MAX_DEVICES = 32
MAX_NODES = 1024

If an implementation has a finite capacity, it belongs to:

- resource management;
- compilation configuration;
- target description;
- runtime;
- deployment;
- safety limits.

It does not belong to language compatibility.

---

40. Quantum Compatibility

Quantum syntax MUST remain independent of physical QPU capacity.

The grammar MUST NOT assume:

- fixed qubit count;
- fixed register width;
- fixed topology;
- fixed coupling map;
- fixed native gate set;
- fixed timing grid;
- fixed calibration;
- fixed device;
- fixed number of measurement channels.

A program may semantically require:

n qubits

where "n" is part of the program.

That does not impose:

language maximum = n

---

41. Quantum Semantic Boundary

The canonical quantum semantic boundary is:

quantum::ir

The required flow is:

Zamani quantum syntax
        ↓
frontend AST
        ↓
semantic validation
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

Grammar MUST NOT create a competing quantum IR.

Frontend-specific quantum gate structures MUST be adapters or syntax representations only.

---

42. Quantum Gate Compatibility

The language MUST NOT make a fixed gate vocabulary the definition of quantum computing.

A backend MAY have a native gate set:

{H, X, Y, Z, CX, RZ, ...}

while Zamani expresses higher-level operations.

The compiler may perform:

portable operation
        ↓
decomposition
        ↓
native operation

provided semantic equivalence is verified.

If an operation cannot be realized:

unsupported target capability

MUST be reported.

The compiler MUST NOT silently replace it with a different computation.

---

43. Quantum Measurement Compatibility

Measurement is semantically meaningful.

A parser or frontend MUST NOT automatically insert measurements merely because a backend needs them.

The following is prohibited:

source circuit
    ↓
compiler silently adds measure-all

unless the language semantics explicitly define that behavior.

Measurement boundaries MUST remain visible to the semantic pipeline.

---

44. QEC Compatibility

Grammar-level QEC constructs MAY express:

- logical-qubit intent;
- protection requirements;
- correction intent;
- error tolerance;
- code-family intent;
- syndrome-related intent;
- fault-tolerance requirements.

The grammar MUST NOT own:

- decoder algorithms;
- syndrome decoding;
- physical error-correction execution;
- QEC resource accounting;
- QEC scheduling;
- QPU transport.

Those remain subsystem responsibilities.

The compatibility direction is:

Zamani syntax
      ↓
semantic intent
      ↓
QEC subsystem

not:

grammar
      ↓
QEC implementation
      ↓
grammar

---

45. ZQN Compatibility

ZQN owns quantum noise/fault semantics.

Grammar MAY express noise-related intent or requirements where those are part of the language contract.

Grammar MUST NOT become the owner of:

- physical noise models;
- calibration measurements;
- runtime fault observations;
- fault classification algorithms;
- backend telemetry.

Those belong to ZQN and runtime layers.

---

46. Scheduling Compatibility

Scheduling is not grammar semantics unless timing itself is explicitly part of the program's semantic contract.

The grammar MAY express:

- timing requirements;
- ordering requirements;
- synchronization intent;
- temporal constraints;
- latency bounds.

The scheduler determines executable ordering using target capabilities and constraints.

The grammar MUST NOT encode:

cycle 0
cycle 1
cycle 2
...

as a universal machine model.

Likewise it MUST NOT assume a fixed clock period.

---

47. Routing Compatibility

Routing maps logical computation to physical resources.

Routing MUST NOT change source semantics.

A topology mismatch is a routing/target compatibility problem.

It is not a grammar problem.

The grammar MUST NOT hard-code:

- coupling maps;
- physical qubit coordinates;
- device topology;
- fixed communication links.

---

48. Resilience Compatibility

Resilience is an orchestration/decision layer.

Grammar MUST NOT own resilience algorithms.

It MAY express semantic resilience requirements such as:

- fault tolerance required;
- retry permitted;
- recovery constraints;
- result acceptance requirements;
- reliability requirements.

Resilience determines how to invoke:

- QEC;
- mitigation;
- rerouting;
- rescheduling;
- recompilation;
- backend switching;
- recovery.

A source program MUST retain its semantics across resilience decisions.

---

49. Hardware Compatibility

Hardware constructs MUST distinguish:

hardware intent

from:

hardware instance

The grammar MAY describe:

- ports;
- interfaces;
- clocks;
- timing requirements;
- hardware modules;
- resource requirements;
- accelerators;
- target capabilities;
- hardware constraints.

It MUST NOT silently bind portable programs to:

- device IDs;
- physical addresses;
- vendor-specific topology;
- fixed register counts;
- fixed resource counts.

Explicit hardware-bound programs MAY exist, but their binding MUST be explicit and versioned.

---

50. HDL Compatibility

HDL syntax MUST distinguish:

hardware behavior

from:

physical implementation

A hardware description MAY specify:

- combinational behavior;
- sequential behavior;
- state machines;
- ports;
- signals;
- clocks;
- timing constraints;
- pipelines;
- memories.

Target-specific synthesis information belongs to hardware compilation.

A Zamani HDL semantic construct MUST NOT accidentally become tied to one FPGA, ASIC, or vendor.

---

51. Classical Compatibility

Classical semantics MUST remain independent from host architecture.

The language MUST NOT assume:

- fixed CPU count;
- fixed register count;
- fixed SIMD width;
- fixed cache size;
- fixed memory size.

Optimization MAY exploit these properties after target discovery.

Source semantics remain unchanged.

---

52. Distributed Compatibility

Distributed constructs MUST distinguish:

logical distributed computation

from:

current physical node topology

Programs may express:

- placement requirements;
- replication requirements;
- consistency;
- communication;
- fault tolerance;
- service relationships.

They MUST NOT require source rewriting solely because:

2 nodes

becomes:

2,000 nodes

or vice versa, unless node count is explicitly semantic.

---

53. AI/Data Compatibility

Tensor, model, dataset, and AI constructs MUST remain independent of accelerator identity.

The language MAY describe:

- tensor semantics;
- model semantics;
- differentiation;
- training;
- inference;
- dataflow;
- accelerator requirements.

The compiler determines whether realization occurs on:

- CPU;
- GPU;
- NPU;
- FPGA;
- quantum accelerator;
- distributed hardware;
- future accelerator.

---

54. Interoperability Compatibility

Interoperability boundaries MUST be explicit.

Examples include:

- C;
- C++;
- Python;
- OpenQASM;
- Verilog;
- SystemVerilog;
- external ABIs;
- foreign runtimes.

Imported languages MUST NOT silently redefine Zamani semantics.

Foreign constructs MUST be represented as explicit interoperability boundaries.

---

55. Dialect Compatibility

Dialects MUST be:

- explicitly named;
- versioned;
- namespaced;
- capability-declared;
- semantically bounded.

A dialect MUST NOT redefine core Zamani semantics without a language-version transition.

Vendor extensions MUST NOT silently become portable language constructs.

---

56. Macro Compatibility

Macros MUST preserve semantic boundaries.

Macro expansion MUST be:

- deterministic;
- version-aware;
- hygienic where required;
- source-located;
- bounded by explicit resource policy;
- independent of hardware discovery.

A macro MUST NOT inspect target hardware during ordinary parsing to decide what syntax means.

Compile-time target information MAY be consumed only through an explicit compilation/resource interface.

---

57. Metaprogramming Compatibility

Compile-time execution MUST remain distinguishable from runtime execution.

Compile-time evaluation MUST NOT silently make source semantics dependent on:

- current machine;
- current time;
- network availability;
- local filesystem;
- hardware topology;
- random environment state.

If environment-dependent compilation is intentionally supported, the dependency MUST be explicit, reproducible, and represented in compilation provenance.

---

58. Determinism

Identical:

source
+
language version
+
declared compilation inputs

MUST produce deterministic:

- tokenization;
- parsing;
- AST structure;
- diagnostics ordering;
- compatibility classification.

No compatibility result may depend on unordered map iteration.

No compatibility result may depend on hardware discovery.

No compatibility result may depend on network ordering.

---

59. Reproducibility

A reproducible compilation MUST identify all compatibility-relevant inputs.

These may include:

- language version;
- dialect versions;
- source files;
- dependency versions;
- macro versions;
- target-independent compilation configuration;
- explicit target information;
- resource policy;
- compiler version.

Undeclared environment state MUST NOT silently affect language semantics.

---

60. Diagnostics Compatibility

Diagnostics MUST be:

- deterministic;
- structured;
- source-located;
- machine-readable;
- human-readable;
- categorized;
- stable enough for tooling.

A compatibility diagnostic SHOULD identify:

code
category
severity
source span
feature
language version
observed representation
expected representation
owning subsystem
suggested remediation

Diagnostics MUST distinguish:

syntax error
semantic error
unsupported feature
version incompatibility
resource failure
capability failure
target incompatibility
backend failure
runtime failure

---

61. No Comment-Based Error Semantics

A failed construct MUST NOT be emitted as a comment to make compilation appear successful.

Forbidden:

// unsupported quantum gate

followed by successful execution.

Required:

structured compilation diagnostic

or a formally defined deferred representation that preserves the construct and explicitly prevents incorrect execution.

---

62. Serialization Compatibility

Serialized AST, IR, package, cache, and artifact representations MUST be versioned.

A serialized representation MUST identify:

- artifact kind;
- schema version;
- language version where relevant;
- producer version where required;
- compatibility contract;
- required migration path.

Deserialization MUST validate versions before interpreting data.

Unknown future fields MAY be preserved where the format explicitly supports forward-compatible extensions.

Unknown required fields MUST cause a deterministic error.

---

63. Compiled Artifact Compatibility

Compiled artifacts MUST NOT be assumed universally portable merely because source is portable.

The compatibility model MUST distinguish:

source artifact
semantic artifact
target-independent compilation artifact
target-specific artifact
binary artifact
runtime state

A target-specific binary MAY require:

- matching ABI;
- matching runtime;
- matching architecture;
- matching device capabilities.

That does not invalidate POCO-REAF.

---

64. Canonical IR Compatibility

Canonical IR is the semantic bridge between frontend and target systems.

IR compatibility MUST preserve all semantic information required for correct execution.

The grammar MUST NOT depend on target-specific IR implementation details.

The direction is:

grammar
   ↓
AST
   ↓
semantic analysis
   ↓
canonical IR
   ↓
target realization

Never:

grammar
   ↔
hardware-specific IR

---

65. Quantum IR Compatibility

"quantum::ir" remains the canonical quantum semantic boundary.

All quantum frontends MUST converge on that boundary.

OpenQASM or another quantum frontend MUST NOT create a second permanent gate/qubit semantic model.

Adapters MAY exist.

Duplicate semantic ownership MUST NOT.

---

66. Runtime Compatibility

Runtime behavior MUST NOT redefine source syntax.

Runtime state MAY determine whether a valid program can execute.

It MUST NOT determine whether the source was syntactically valid.

For example:

runtime has 8 qubits

MUST NOT cause the parser to reject:

program requesting 16 qubits

The program should parse and semantically validate, after which resource admission may reject execution.

---

67. Target Compatibility

Target selection MUST occur after language interpretation.

Conceptually:

source
 ↓
language semantics
 ↓
requirements/capabilities
 ↓
target matching
 ↓
realization

Not:

target
 ↓
change meaning of source

---

68. Resource Availability

Resource failures MUST be represented separately from compatibility.

Examples:

INSUFFICIENT_QUBITS
INSUFFICIENT_MEMORY
INSUFFICIENT_ACCELERATORS
INSUFFICIENT_BANDWIDTH
UNSUPPORTED_GATE
UNSUPPORTED_TIMING
UNSUPPORTED_CAPABILITY
UNAVAILABLE_BACKEND

These MUST NOT be encoded as grammar errors.

---

69. Target-Specific Extensions

Target-specific syntax MAY exist when explicitly required.

It MUST be:

- namespaced;
- versioned;
- clearly target-bound;
- separated from portable semantics;
- documented as non-portable when appropriate.

Example:

target "vendor.example" {
    ...
}

must not cause the entire language to become vendor-specific.

---

70. Hardware Discovery

Hardware discovery MUST NOT be performed by:

- lexer;
- parser;
- AST construction;
- grammar validation.

Discovery belongs to:

- hardware abstraction;
- compilation context;
- runtime;
- resource manager.

The result of discovery may be consumed after semantic interpretation.

---

71. Calibration

Calibration MUST NOT alter source-language meaning.

Calibration may affect:

- gate implementation;
- pulse realization;
- scheduling;
- routing;
- mitigation;
- resilience;
- backend selection.

It MUST NOT change what a source-level quantum operation means.

---

72. Security Compatibility

Compatibility validation MUST prevent semantic weakening through version migration.

Security-relevant properties MUST NOT disappear during:

source
→ AST
→ semantic analysis
→ IR
→ target lowering

A compiler MUST reject transformations that violate mandatory security semantics.

---

73. Ownership of Domain Features

Each concept MUST have one semantic owner.

Examples:

Concept| Owner
syntax| grammar/frontend
lexical tokens| lexer
AST structure| AST
type meaning| semantic/type system
effects| effect system
quantum meaning| "quantum::ir"
QEC algorithms| QEC
fault/noise semantics| ZQN
routing| routing
ordering/timing| scheduling
recovery decisions| resilience
hardware capabilities| HAL
resource accounting| resource subsystem
optimization| optimization
runtime execution| runtime
ABI| interoperability/runtime
target realization| backend

A compatibility test MUST fail architectural ownership violations.

---

74. Circular Dependency Prohibition

The following architectural cycle is prohibited:

grammar
 ↓
IR
 ↓
grammar

Also prohibited:

grammar
 ↓
runtime
 ↓
grammar

and:

quantum grammar
 ↓
hardware grammar
 ↓
quantum grammar

The dependency direction MUST remain acyclic.

---

75. Repository Integration Contract

The compatibility validator MUST check the following integration boundaries:

grammar/specification/
grammar/spec/
grammar/Zamani.g4
grammar/lexer/
grammar/core/
grammar/types/
grammar/expressions/
grammar/statements/
grammar/declarations/
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
grammar/distributed/
grammar/ai/
grammar/data/
grammar/networking/
grammar/security/
grammar/resources/
grammar/compile/
grammar/execution/
grammar/interoperability/
grammar/dialects/
grammar/macros/
grammar/metaprogramming/

Where one domain grammar consumes another, the dependency MUST be explicit.

---

76. Integration With "grammar/grammar.md"

"grammar/grammar.md" is an implementation/conformance reference.

It MUST NOT become a competing normative language.

Compatibility validation MUST compare it against:

grammar/specification/*
grammar/spec/*
Zamani.g4
src/lexer.rs
src/parser.rs
src/ast/*

Any construct present in "grammar/grammar.md" but not implemented MUST be classified explicitly.

---

77. Integration With "grammar/Zamani-Grammar.md"

"grammar/Zamani-Grammar.md" may contain broader or historical design material.

It MUST NOT cause a feature to become production syntax merely because it is documented there.

Every feature promoted from this document MUST pass:

proposal
 ↓
specification
 ↓
AST design
 ↓
semantic definition
 ↓
canonical representation
 ↓
implementation
 ↓
tests
 ↓
compatibility classification
 ↓
stable release

---

78. Integration With "grammar/Zamani.g4"

"Zamani.g4" is the canonical ANTLR representation after reconciliation.

It MUST:

- match the canonical lexical model;
- match syntax specification;
- preserve semantic distinctions required by AST;
- avoid target-specific limits;
- avoid duplicate semantic constructs;
- remain deterministic;
- be testable independently;
- be checked against the Rust frontend.

ANTLR generation MUST be reproducible.

Generated files MUST NOT be hand-modified as authoritative implementations.

---

79. Integration With Rust Lexer

The Rust lexer MUST implement the canonical lexical contract.

Compatibility tests MUST compare representative token streams between:

Rust lexer
ANTLR lexer

Differences require classification.

No silent divergence is allowed.

---

80. Integration With Rust Parser

The Rust parser MUST accept precisely the syntax supported by the active language version and implementation profile.

Where ANTLR and Rust parser differ, the difference MUST be:

- intentional;
- documented;
- versioned if semantic;
- tested.

---

81. Integration With AST

Every accepted construct MUST map to a canonical AST representation.

AST adapters MAY exist during migration.

Adapters MUST be lossless for supported stable constructs.

A lossy adapter is not production-compatible.

---

82. Integration With Semantic Analysis

Semantic analysis owns:

- type validity;
- name resolution;
- effect validity;
- capability validity;
- resource semantics;
- ownership;
- domain semantics.

The parser MUST NOT perform target-dependent semantic decisions.

---

83. Integration With Optimization

Optimization MUST consume canonical semantic representations.

Grammar compatibility MUST NOT depend on a particular optimizer.

A new optimizer MUST preserve source semantics.

---

84. Integration With Scheduling

Scheduling MUST consume semantic/IR information.

Grammar compatibility MUST remain valid when:

- scheduler implementation changes;
- scheduling policy changes;
- machine clock changes;
- hardware timing changes.

---

85. Integration With Routing

Routing MUST consume canonical representations and hardware topology.

Grammar compatibility MUST remain independent of routing algorithms.

---

86. Integration With QEC and ZQN

QEC and ZQN MUST remain downstream semantic consumers.

Grammar validation MUST verify that quantum syntax does not accidentally become an implementation of:

- decoder logic;
- fault injection;
- physical noise;
- calibration;
- QPU transport.

---

87. Integration With Resilience

Resilience MUST be able to adapt execution without changing source semantics.

A resilience action such as:

retry
restart
resume
rollback
remap
reroute
reschedule
recompile
reoptimize
change_qec
mitigate
switch_backend
quarantine
abort

MUST NOT silently change the meaning of the original program.

---

88. Integration With Hardware Abstraction

Hardware abstraction owns:

- capabilities;
- topology;
- device state;
- calibration;
- physical resources;
- backend identity.

Grammar compatibility MUST NOT depend on a specific hardware implementation.

---

89. Integration With Runtime

Runtime consumes compiled representations.

Runtime failures MUST NOT be reported as parser errors.

Runtime state MUST NOT redefine syntax.

---

90. Integration With Interoperability

Foreign-language integration MUST preserve explicit boundaries.

The compatibility validator MUST verify:

- ABI declarations;
- calling conventions;
- type mappings;
- ownership transfer;
- lifetime contracts;
- error propagation;
- quantum/classical boundaries;
- hardware interface boundaries.

---

91. Integration With Tooling

IDE, formatter, linter, language server, documentation generator, and code navigation tools MUST consume the same syntax/AST contract.

No tooling-only grammar may silently diverge from production syntax.

---

92. Test Requirements

Every grammar feature MUST have:

1. positive tests;
2. negative tests;
3. boundary tests;
4. version tests;
5. compatibility tests;
6. determinism tests where applicable;
7. round-trip tests where applicable.

---

93. Cross-Domain Tests

The compatibility suite MUST include combinations of:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

These tests verify that domain grammars compose rather than accidentally create competing languages.

---

94. Scalability Tests

The compatibility suite MUST test source programs whose sizes are varied by generated fixtures.

Tests MUST NOT encode production ceilings such as:

64 qubits
1024 threads
256 nodes

as language limits.

Instead, tests SHOULD generate sizes according to available test resources.

A test may intentionally impose a local test budget, but that budget MUST be identified as a test resource limit rather than a language limit.

---

95. Quantum Scalability Tests

The suite MUST verify that syntax remains valid for different logical resource sizes.

For example:

1 qubit
2 qubits
8 qubits
64 qubits
1024 qubits
larger generated logical programs

must not require grammar changes merely because the number changes.

The exact maximum tested is determined by available test resources.

---

96. Hardware Scalability Tests

The same source-level semantics MUST be testable against abstract target descriptions containing different:

- core counts;
- memory capacities;
- accelerator counts;
- topology sizes;
- clock characteristics;
- communication resources.

The grammar MUST remain unchanged.

---

97. Determinism Tests

Repeated compilation of identical:

source
language version
configuration

MUST produce identical:

- tokenization;
- parse structure;
- AST;
- compatibility classification;
- diagnostics ordering.

---

98. Round-Trip Tests

Where a canonical printer/serializer exists:

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

MUST preserve semantics.

Whitespace or formatting differences are acceptable where syntax permits them.

Semantic differences are not.

---

99. Migration Tests

Every compatibility-affecting language change MUST provide migration tests.

Migration testing MUST cover:

old source
 ↓
migration
 ↓
new source
 ↓
new parser
 ↓
new semantic model

The resulting program MUST preserve intended semantics unless the change is explicitly breaking.

---

100. Compatibility Matrix

Each release SHOULD maintain a machine-readable feature matrix containing:

feature
introduced_version
current_status
lexer
parser
AST
semantic
IR
verifier
backend
tests
deprecated
replacement

The matrix MUST NOT claim "STABLE" unless all required implementation boundaries exist.

---

101. No Phantom Features

A feature MUST NOT be advertised as production-supported if:

- parser support exists but AST does not;
- AST exists but semantics do not;
- semantics exist but IR does not;
- IR exists but verification does not;
- verification exists but execution is silently incorrect;
- tests do not cover the stable contract.

A feature with incomplete implementation MUST be classified accordingly.

---

102. No Phantom Compatibility

The existence of syntax compatibility does not imply target compatibility.

For example:

syntax valid

does not imply:

QPU available

Similarly:

AST valid

does not imply:

hardware executable

Each compatibility boundary MUST be evaluated independently.

---

103. Error Classification

Compatibility errors MUST have stable categories.

At minimum:

LANGUAGE_VERSION_ERROR
LEXICAL_COMPATIBILITY_ERROR
SYNTAX_COMPATIBILITY_ERROR
AST_COMPATIBILITY_ERROR
SEMANTIC_COMPATIBILITY_ERROR
TYPE_COMPATIBILITY_ERROR
EFFECT_COMPATIBILITY_ERROR
CAPABILITY_COMPATIBILITY_ERROR
RESOURCE_COMPATIBILITY_ERROR
DIALECT_COMPATIBILITY_ERROR
IR_COMPATIBILITY_ERROR
ABI_COMPATIBILITY_ERROR
RUNTIME_COMPATIBILITY_ERROR
TARGET_COMPATIBILITY_ERROR
INTEROPERABILITY_COMPATIBILITY_ERROR
SERIALIZATION_COMPATIBILITY_ERROR
MIGRATION_ERROR

Provider-specific hardware error codes MUST NOT become language compatibility codes.

---

104. Safe Rust Requirement

All grammar compatibility infrastructure MUST compile under:

Rust 1.97
Rust 1.97.1

and MUST use safe Rust only.

The following are prohibited:

unsafe { ... }

unsafe fn

unsafe trait

unsafe implementations or hidden unsafe dependencies introduced solely for grammar scalability.

Crates used by the grammar infrastructure MUST be compatible with the project's supported Rust baseline.

---

105. Panic-Free Validation

Production compatibility validation MUST NOT panic on malformed user source.

Invalid input MUST result in structured diagnostics.

Internal invariant violations MAY use controlled failure mechanisms only where they represent programmer errors, but user-controlled source MUST NOT be able to trigger undefined behavior or uncontrolled process failure.

---

106. Resource-Safe Validation

Validation MUST scale according to available resources.

Where an implementation limit is unavoidable, it MUST be:

- explicit;
- documented;
- configurable where appropriate;
- independent from language semantics;
- reported as a resource/implementation limit.

A resource guard MUST never be presented as:

Zamani language maximum

unless it genuinely is a language semantic restriction.

---

107. No Hidden Host Limits

The compatibility architecture MUST NOT accidentally derive language limits from:

- "usize";
- host pointer width;
- host memory size;
- host CPU count;
- host stack size;
- host thread count.

Host representation may constrain one compiler invocation.

It MUST NOT redefine Zamani's semantic capacity.

---

108. Configuration Compatibility

Compilation configuration MUST be explicit.

Configuration MAY influence:

- optimization;
- target selection;
- resource budgets;
- diagnostics;
- feature gates;
- deployment.

Configuration MUST NOT silently alter language semantics.

---

109. Environment Compatibility

Environment variables MUST NOT silently alter source meaning.

If environment-dependent behavior is supported, it MUST be represented as an explicit compilation or runtime input.

The dependency MUST be visible to reproducibility/provenance systems.

---

110. Provenance

Compatibility-relevant compilation MUST be traceable to:

- source identity;
- source version;
- language version;
- compiler version;
- grammar version;
- dialect versions;
- dependency versions;
- target description;
- resource context where relevant;
- migration information.

Provenance MUST distinguish semantic inputs from target realization inputs.

---

111. Future-Proofing

Compatibility rules MUST preserve extension space for future computing paradigms.

The language may eventually support computing substrates that do not currently exist.

Therefore compatibility validation MUST avoid assumptions that:

- quantum is the final non-classical paradigm;
- CPUs are the final classical architecture;
- GPUs are the final accelerator model;
- FPGA/ASIC are the final hardware description targets;
- distributed computing has a fixed topology;
- memory is always address-based;
- computation is always discrete.

Future domains MUST be able to attach to the semantic model without changing existing source semantics unnecessarily.

---

112. Atom-to-Everywhere Requirement

The compatibility model MUST permit:

tiny computation
        ↓
single machine
        ↓
large machine
        ↓
heterogeneous machine
        ↓
distributed system
        ↓
quantum/classical system
        ↓
future computational substrate

without changing the semantic identity of the program merely because scale changed.

Scale is a resource/realization concern unless explicitly made semantic by the program.

---

113. Explicit Semantic Resource Requests

The language MAY express:

requires qubits >= n
requires memory >= m
requires capability(quantum)
requires capability(gpu)
requires capability(fpga)
requires latency <= t
requires reliability >= r

These are valid semantic requirements.

They MUST remain separate from:

use device "specific-device"

A requirement is not an implementation decision.

---

114. Requirement Versus Preference

Compatibility MUST distinguish:

requirement
constraint
capability
preference
hint

A preference MUST NOT become a hard requirement.

A hint MUST NOT become a semantic guarantee.

A capability MUST NOT become a physical identity.

A resource requirement MUST NOT become a parser ceiling.

---

115. Target Adaptation

Target adaptation MAY change:

- gate decomposition;
- instruction selection;
- scheduling;
- routing;
- memory placement;
- accelerator mapping;
- distributed placement;
- communication strategy;
- pulse realization.

It MUST preserve source semantics.

---

116. Backend Addition Compatibility

Adding a new backend MUST NOT require modifying portable grammar semantics merely to recognize the backend.

A new target should normally require:

target description
+
capability model
+
lowering
+
backend implementation

rather than new universal syntax.

---

117. Future Hardware Compatibility

A future hardware target MUST be able to consume existing semantic representations if it can realize their requirements.

If new semantics are genuinely necessary, they MUST enter through the language evolution process.

The target MUST NOT retroactively redefine existing syntax.

---

118. Documentation Compatibility

Documentation MUST identify whether a feature is:

normative
implemented
experimental
planned
historical
reserved

Documentation MUST NOT present planned functionality as implemented.

Examples MUST correspond to the language version they claim to demonstrate.

---

119. Example Compatibility

Every production example SHOULD declare or otherwise inherit a language version.

Examples used for compatibility testing MUST be immutable fixtures or versioned.

Changing an example that serves as a compatibility fixture MUST be treated as a compatibility change.

---

120. Generated Artifact Compatibility

Generated ANTLR/parser artifacts MUST be reproducible from their source grammar.

Generated output MUST NOT be manually patched to implement language features.

The build process MUST detect stale generated artifacts where practical.

---

121. Grammar Import Compatibility

If ANTLR grammar fragments are imported, their ownership MUST be explicit.

Imported grammars MUST NOT accidentally override lexical priority or introduce conflicting token definitions.

Every imported grammar MUST be checked for:

- duplicate token names;
- duplicate parser rules;
- precedence conflicts;
- keyword collisions;
- hidden semantic divergence.

---

122. Rule Naming Compatibility

Grammar rule names SHOULD remain stable once exposed to tooling.

Renaming a rule MAY affect:

- generated parser APIs;
- diagnostics;
- tooling;
- tests;
- integrations.

Therefore rule renames MUST be treated as tooling compatibility changes even if source syntax remains unchanged.

---

123. Token Naming Compatibility

Token names are tooling/API contracts.

A token rename MUST NOT silently alter semantic identity.

Aliases MAY be retained during migration where the parser/tooling architecture permits.

---

124. Operator Compatibility

Operator precedence and associativity are language semantics.

Changing them MUST be versioned.

The compatibility suite MUST include expressions that distinguish adjacent precedence levels.

For example:

a + b * c
a << b + c
a == b && c == d
a | b & c

must have stable interpretation across compatible releases.

---

125. Delimiter Compatibility

Changing delimiter meaning is a syntax compatibility change.

Examples:

()
[]
{}
<>
::

must not be repurposed without versioning and migration analysis.

---

126. Whitespace Compatibility

Whitespace MUST NOT alter semantics unless the language explicitly defines significant whitespace.

If significant whitespace is introduced, it MUST be versioned and documented.

---

127. Comments and Documentation

Comments MUST NOT alter semantic interpretation.

Documentation comments MAY be preserved in AST metadata when tooling requires them.

They MUST NOT become executable semantics accidentally.

---

128. Module Compatibility

Module resolution MUST remain version-aware and deterministic.

Changing:

- import resolution;
- package resolution;
- namespace resolution;
- visibility;

is a compatibility change.

Module identity MUST NOT depend on physical machine topology.

---

129. Package Compatibility

Package metadata MUST be versioned independently from language syntax.

A package version MUST NOT be treated as a language version.

Dependencies MUST be resolved through explicit package/dependency semantics.

---

130. ABI Compatibility

ABI compatibility is separate from source compatibility.

The compiler MUST distinguish:

source-compatible
semantic-compatible
IR-compatible
ABI-compatible
binary-compatible
runtime-compatible

A program may be source-compatible but require recompilation for a new ABI.

That is expected and does not violate POCO-REAF.

---

131. Runtime State Compatibility

Runtime state MUST NOT be confused with source semantics.

For resumable systems, checkpoint/recovery compatibility belongs to the runtime/resilience/checkpoint contracts.

The grammar MUST NOT assume arbitrary quantum state can always be serialized.

Where state cannot be preserved, the runtime must identify a valid semantic boundary such as:

- classical execution state;
- compiled program;
- logical checkpoint;
- measurement boundary;
- QEC-defined recoverable state;
- provider-supported state.

---

132. Security and Trust

Compatibility migration MUST NOT weaken:

- authentication;
- authorization;
- capability restrictions;
- cryptographic requirements;
- isolation;
- provenance;
- integrity verification.

A compatibility adapter that removes a security property is invalid.

---

133. Deterministic Feature Detection

Feature detection MUST be explicit.

The compiler MAY determine:

feature supported?
capability available?
target supported?

but MUST NOT reinterpret syntax differently merely because a feature happens to be available.

---

134. Capability Negotiation

Negotiation belongs to compilation/runtime layers.

The source may declare requirements.

The target may advertise capabilities.

The compiler resolves:

requirements ∩ capabilities

without modifying source semantics.

---

135. Graceful Failure

When compatibility cannot be established, the system MUST fail at the earliest correct boundary.

Examples:

invalid token
    → lexer

invalid syntax
    → parser

invalid type
    → type checker

missing capability
    → capability validation

insufficient resources
    → resource validation

unsupported IR feature
    → IR/backend boundary

unsupported target
    → target compiler

runtime failure
    → runtime/resilience

Failures MUST NOT be pushed into unrelated layers.

---

136. Compatibility Audit

Before a language release, the following audit MUST be performed:

[ ] specification reconciled
[ ] Zamani.g4 reconciled
[ ] Rust lexer reconciled
[ ] Rust parser reconciled
[ ] AST reconciled
[ ] semantic analysis reconciled
[ ] canonical IR reconciled
[ ] quantum::ir reconciled
[ ] domain grammars reconciled
[ ] dialects reconciled
[ ] compatibility matrix updated
[ ] migration rules updated
[ ] deprecated features checked
[ ] reserved features checked
[ ] positive tests pass
[ ] negative tests pass
[ ] boundary tests pass
[ ] scalability tests pass
[ ] deterministic tests pass
[ ] round-trip tests pass
[ ] cross-domain tests pass
[ ] hard-coding audit passes
[ ] unsafe-code audit passes
[ ] generated artifacts reproducible
[ ] documentation reconciled

---

137. Hard-Coding Audit

Every grammar-related release MUST search for accidental fixed limits involving:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_DEVICES
MAX_NODES
MAX_MEMORY
MAX_REGISTER
MAX_TENSOR
MAX_CIRCUIT
MAX_GATE
MAX_PORT
MAX_CHANNEL
MAX_ACCELERATOR
MAX_TOPOLOGY

Every discovered constant MUST be classified as:

LANGUAGE_SEMANTIC_LIMIT
IMPLEMENTATION_LIMIT
RESOURCE_LIMIT
TARGET_LIMIT
SECURITY_LIMIT
TEST_LIMIT
DOCUMENTATION_LIMIT
ACCIDENTAL_HARD_CODING

"ACCIDENTAL_HARD_CODING" MUST be removed.

---

138. Safe-Rust Audit

Compatibility infrastructure MUST be checked for:

unsafe blocks
unsafe functions
unsafe traits
unsafe implementations
unsafe FFI assumptions
unchecked target-dependent casts

The grammar infrastructure MUST remain safe Rust.

FFI boundaries elsewhere in the compiler MUST not cause grammar code to become unsafe.

---

139. No Host-Dependent Semantics

The following MUST NOT alter language meaning:

- host architecture;
- pointer width;
- CPU count;
- memory capacity;
- available GPUs;
- available QPUs;
- filesystem layout;
- operating system;
- network topology;
- current clock;
- environment variable ordering.

If such information is explicitly requested by the program, it becomes an explicit capability/resource/runtime dependency.

---

140. Compatibility and Optimization Profiles

Optimization profiles MAY change implementation quality.

They MUST NOT change semantic meaning.

For example:

-O0
-O2
-O3

or future optimization policies may produce different machine representations while preserving the same semantics.

---

141. Approximate Computing

Approximate constructs MUST explicitly state their semantic contract.

An approximation MUST NOT be introduced merely because a target is resource-constrained.

If approximation is permitted, the language MUST define:

- acceptable error;
- quality bounds;
- determinism;
- probabilistic semantics where applicable;
- verification expectations.

---

142. Floating-Point Compatibility

Floating-point semantics MUST distinguish:

- language mathematical intent;
- numerical representation;
- target floating-point implementation.

A target MUST NOT silently change required numerical guarantees.

Where implementation-dependent floating behavior is allowed, the contract MUST state it.

---

143. Quantum Numerical Compatibility

Quantum parameters such as:

- angles;
- amplitudes;
- probabilities;
- observables;

MUST be represented semantically rather than prematurely bound to one target numeric representation.

Precision reduction MUST be explicit and verified where semantic guarantees require it.

---

144. HDL Timing Compatibility

Timing constraints MUST be semantic only when the source program explicitly requires them.

Otherwise:

clock frequency
cycle duration
pulse width

remain target realization properties.

A target with a different timing model may require different implementation without changing source semantics.

---

145. Distributed Timing Compatibility

Network latency and topology MUST NOT silently redefine source semantics.

If timing is semantically constrained:

latency <= T

then it is a requirement.

Otherwise network timing is a realization concern.

---

146. Future-Domain Rule

A new computing domain MUST NOT require modifying unrelated grammar semantics merely to integrate.

A new domain should define:

syntax
AST mapping
semantic contract
capabilities
resources
canonical representation
lowering boundary
tests
compatibility classification

and integrate through existing architecture.

---

147. File Completion Contract

"grammar/validation/compatibility-rules.md" is complete only when:

- all compatibility dimensions are defined;
- ownership boundaries are explicit;
- version authority is explicit;
- source/target compatibility are separated;
- quantum compatibility is integrated with "quantum::ir";
- QEC/ZQN/resilience boundaries are explicit;
- hardware/HDL boundaries are explicit;
- scalability rules exist;
- hard-coding rules exist;
- Rust 1.97/1.97.1 compatibility is stated;
- "unsafe" is prohibited;
- diagnostics rules exist;
- migration rules exist;
- testing requirements exist;
- repository integration rules exist;
- no second language authority is created.

---

148. Completion Criteria for the Repository

This file MUST NOT be marked complete merely because the document exists.

The repository-level compatibility work is complete only when:

Specification
     ↓
ANTLR
     ↓
Rust lexer
     ↓
Rust parser
     ↓
AST
     ↓
Semantic analysis
     ↓
Canonical IR
     ↓
quantum::ir
     ↓
Optimization
     ↓
Routing
     ↓
Scheduling
     ↓
Resilience / QEC / ZQN
     ↓
Hardware abstraction
     ↓
Compilation
     ↓
Runtime

has no unexplained semantic divergence.

---

149. Final Compatibility Invariant

The following invariant is absolute:

A Zamani program describes computation and intent.

The machine describes available means of realization.

Compatibility preserves the computation while allowing the realization to change.

Therefore:

ONE PROGRAM
     ↓
ONE SEMANTIC MEANING
     ↓
MANY COMPILATION STRATEGIES
     ↓
MANY RESOURCE CONFIGURATIONS
     ↓
MANY MACHINE SIZES
     ↓
MANY HARDWARE ARCHITECTURES
     ↓
MANY EXECUTION ENVIRONMENTS
     ↓
FUTURE COMPUTATIONAL SUBSTRATES

The language MUST scale from the smallest meaningful computation to arbitrarily large computations bounded by actual available resources and explicit implementation safety constraints.

It MUST NOT impose artificial machine ceilings through grammar design.

The fundamental Zamani compatibility objective is therefore:

Program_Once
Compile_Once
Run_Everywhere
Anywhere
Forever

or:

POCO-REAF

while preserving:

- semantic stability;
- hardware independence;
- quantum/classical interoperability;
- HDL capability;
- deterministic parsing;
- safe Rust;
- version compatibility;
- migration capability;
- canonical IR ownership;
- explicit resource semantics;
- explicit capability semantics;
- explicit target adaptation;
- repository-wide consistency;
- long-term extensibility;
- production correctness.