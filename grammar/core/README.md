Zamani Core Grammar

Status

Production architecture

This directory defines the foundational, domain-neutral syntax contracts used to assemble the Zamani programming language.

The core grammar is part of the authoritative Zamani language grammar and exists to provide stable syntax for concepts that are shared across classical, quantum, hybrid, HDL, hardware, distributed, AI, networking, security, and future computational domains.

The core layer is deliberately not a computational backend.

It does not execute programs, discover hardware, select devices, allocate resources, optimize circuits, schedule operations, perform QEC, model quantum noise, or construct runtime state.

---

1. Purpose

"grammar/core/" owns the language-wide syntactic foundations required by all higher-level Zamani grammar domains.

These foundations include:

- compilation units;
- source units;
- names;
- paths;
- qualified names;
- attributes;
- annotations;
- metadata;
- language/version declarations;
- capabilities;
- requirements;
- constraints;
- hints;
- pragmas.

The core layer provides the common vocabulary through which the rest of the grammar can express computation without duplicating fundamental syntax.

The central architectural principle is:

«Core grammar defines portable source syntax. It does not define the physical machine on which that syntax eventually executes.»

---

2. POCO-REAF Principle

Zamani is designed around:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

or:

POCO-REAF

The core grammar therefore MUST NOT encode assumptions about the machine available when a program is written.

A core grammar construct must remain valid independently of:

- CPU count;
- CPU architecture;
- core count;
- thread count;
- GPU count;
- FPGA count;
- ASIC count;
- accelerator count;
- memory capacity;
- register capacity;
- vector width;
- network size;
- cluster size;
- node count;
- quantum processor count;
- qubit count;
- physical topology;
- device identifier;
- vendor;
- calibration state;
- runtime location;
- deployment environment.

The source language describes:

- computation;
- intent;
- semantics;
- requirements;
- constraints;
- capabilities;
- preferences;
- hints;
- effects;
- portability requirements.

The compiler and runtime determine how those semantics can be realized on available resources.

---

3. Architectural Boundary

The complete Zamani direction is:

Zamani source
    |
    v
canonical lexer
    |
    v
core/domain parser grammar
    |
    v
AST
    |
    v
name/module resolution
    |
    v
type analysis
    |
    v
effect analysis
    |
    v
capability analysis
    |
    v
resource/requirement analysis
    |
    v
semantic representation
    |
    +--------------------+
    |                    |
    v                    v
classical IR        quantum::ir
    |                    |
    +----------+---------+
               |
               v
       optimization
               |
               v
        routing/scheduling
               |
               v
       QEC / ZQN / resilience
               |
               v
       target lowering
               |
               v
       hardware/runtime

The grammar is upstream of semantic IR.

Therefore:

grammar -> AST -> semantic analysis -> IR

is valid.

The following are architectural violations:

grammar -> quantum::ir -> grammar
grammar -> runtime -> grammar
grammar -> hardware discovery -> grammar
grammar -> scheduler -> grammar
grammar -> QEC -> grammar

---

4. Core Directory Ownership

The core directory owns only language-wide syntax.

It does not own computational-domain semantics.

Core owns

- source roots;
- source-unit composition;
- canonical names;
- canonical paths;
- qualified names;
- source attributes;
- source annotations;
- metadata syntax;
- language-version syntax;
- capability-expression syntax;
- requirement-expression syntax;
- constraint syntax;
- hint syntax;
- pragma syntax.

Core does not own

- classical IR;
- quantum IR;
- HDL IR;
- hardware IR;
- runtime state;
- physical qubit allocation;
- physical CPU allocation;
- routing;
- scheduling;
- optimization;
- QEC algorithms;
- ZQN semantics;
- resilience decisions;
- calibration;
- backend discovery;
- device selection;
- resource allocation;
- code generation;
- executable generation;
- machine topology;
- machine limits.

---

5. Files

The core directory currently contains the following architectural units:

core/
├── README.md
├── compilation-unit.g4
├── source-unit.g4
├── names.g4
├── paths.g4
├── qualified-names.g4
├── attributes.g4
├── annotations.g4
├── metadata.g4
├── versioning.g4
├── capabilities.g4
├── requirements.g4
├── constraints.g4
├── hints.g4
└── pragmas.g4

Every file has a single primary owner.

---

6. "compilation-unit.g4"

Purpose

Defines the canonical root of a complete Zamani source artifact.

Owns

- compilation-unit root;
- source-item composition;
- top-level source ordering;
- end-of-file boundary;
- composition of domain-independent source structures.

Does not own

- lexer definitions;
- domain-specific syntax;
- AST definitions;
- semantic validation;
- IR;
- target selection;
- runtime behavior.

Integration

Consumes the canonical lexer vocabulary.

Composes:

- source-unit grammar;
- core declarations;
- module grammar;
- type grammar;
- statement grammar;
- function grammar;
- classical grammar;
- quantum grammar;
- HDL grammar;
- hardware grammar;
- hybrid grammar;
- distributed grammar;
- AI/data grammar;
- networking grammar;
- security grammar;
- compilation grammar;
- execution grammar.

The current compilation-unit architecture already establishes this as the composition boundary and explicitly avoids fixed machine capacities.

Completion criterion

A complete Zamani source file can be parsed through exactly one authoritative root without requiring a domain-specific root.

There must not be separate roots such as:

QuantumCompilationUnit
GpuCompilationUnit
CpuCompilationUnit
FpgaCompilationUnit
HardwareCompilationUnit

A Zamani program can contain multiple computational domains.

---

7. "source-unit.g4"

Purpose

Defines source-level structural units beneath the compilation-unit boundary.

Owns

- source-unit structure;
- source-level ordering;
- source item grouping;
- source-level declarations/directives where applicable.

Does not own

- semantic resolution;
- module loading;
- filesystem access;
- dependency fetching;
- package management.

Integration

Feeds source structure into the AST.

A source unit must remain independent of physical deployment.

---

8. "names.g4"

Purpose

Defines canonical lexical/parser-level name structures.

Owns

- identifiers;
- identifier composition;
- name syntax;
- reserved-name boundaries where syntactically required.

Does not own

- symbol resolution;
- scope;
- declaration lookup;
- type resolution;
- runtime names;
- hardware identifiers.

A name is syntax.

Its meaning belongs to semantic analysis.

---

9. "paths.g4"

Purpose

Defines language-level path syntax.

Owns

- source/module paths;
- path components;
- path composition;
- path syntax normalization boundaries.

Does not own

- filesystem access;
- operating-system paths;
- network access;
- dependency downloading;
- module loading.

A syntactically valid path must not imply permission to access a filesystem or network resource.

---

10. "qualified-names.g4"

Purpose

Defines canonical qualified-name syntax.

Examples include conceptual forms such as:

namespace::name
namespace::subnamespace::name
domain::feature::operation

Owns

- qualification;
- component ordering;
- qualified-name syntax;
- arbitrary supported qualification depth.

Does not own

- symbol lookup;
- namespace existence;
- module loading;
- vendor registration;
- capability verification.

Scalability

There must be no arbitrary semantic depth such as:

namespace::subnamespace::name

being the maximum supported form.

The grammar should use repetition so the structure scales with parser resources rather than an artificial language limit.

---

11. "attributes.g4"

Purpose

Defines source attributes attached to declarations or other syntactic entities.

Owns

- attribute syntax;
- attribute names;
- attribute arguments;
- attribute attachment structure.

Does not own

- attribute semantics;
- compiler policy;
- optimization policy;
- runtime behavior.

An attribute is parsed first and interpreted later.

---

12. "annotations.g4"

Purpose

Defines structured source annotations.

Annotations may communicate portable source intent to downstream semantic stages.

Owns

- annotation syntax;
- annotation names;
- annotation arguments;
- annotation attachment.

Does not own

- semantic interpretation;
- backend behavior;
- hardware discovery;
- optimization;
- scheduling.

Annotations must never silently become executable commands.

---

13. "metadata.g4"

Purpose

Defines source metadata structures.

Metadata may describe:

- provenance;
- documentation;
- language information;
- compilation intent;
- compatibility information;
- reproducibility information;
- source-level descriptors.

Does not own

- provenance storage implementation;
- cryptographic signing;
- artifact storage;
- runtime telemetry.

Metadata is source structure, not runtime state.

---

14. "versioning.g4"

Purpose

Defines source-language version and compatibility syntax.

Owns

- language-version declarations;
- syntax-version references;
- compatibility declarations;
- version ranges where the language specification permits them.

Does not own

- migration execution;
- compiler version detection;
- package version resolution;
- runtime version negotiation.

POCO-REAF requirement

Versioning must preserve the meaning of an existing program across future implementations.

Version declarations must not embed temporary machine characteristics.

---

15. "capabilities.g4"

Purpose

Defines syntax for declaring or referring to capabilities.

Capabilities describe what an execution environment or semantic domain can support.

Examples conceptually include:

quantum
dynamic_control
parallel_execution
tensor_acceleration
hardware_description
distributed_execution

Critical distinction

A capability is not a resource allocation.

For example:

requires quantum

does not mean:

use device X

and:

requires dynamic_control

does not mean:

use backend Y

Does not own

- capability discovery;
- hardware probing;
- provider APIs;
- backend selection;
- resource allocation.

Those belong downstream.

---

16. "requirements.g4"

Purpose

Defines source-level requirements.

Requirements express properties that must be satisfied for a program or operation to be valid.

Examples include conceptual requirements such as:

requires quantum
requires distributed_execution
requires dynamic_measurement
requires hardware_description

Owns

Requirement syntax.

Does not own

Requirement satisfaction.

A requirement becomes a semantic obligation that the compiler/runtime must evaluate against available capabilities.

---

17. "constraints.g4"

Purpose

Defines source-level constraints.

A constraint restricts permissible implementation choices without necessarily specifying a particular implementation.

For example, a program may express a constraint involving:

- latency;
- precision;
- reliability;
- memory class;
- communication properties;
- timing;
- energy;
- execution ordering.

Does not own

- scheduling;
- resource allocation;
- target selection;
- hardware discovery.

A constraint becomes an input to downstream planning.

---

18. "hints.g4"

Purpose

Defines non-binding implementation hints.

A hint communicates a preferred implementation strategy without changing the program's semantic meaning.

For example:

prefer parallel execution
prefer low latency
prefer energy efficiency
prefer quantum acceleration

A hint MUST NOT become an unconditional requirement.

Critical rule

If a target cannot honor a hint, the program remains semantically valid unless the source explicitly expressed a requirement or constraint.

This distinction is essential for POCO-REAF.

---

19. "pragmas.g4"

Purpose

Defines the syntax of source pragmas.

The existing pragma grammar intentionally treats pragmas as source-level data/directives rather than executable commands. It also deliberately avoids fixed maximums for pragmas, arguments, qubits, devices, memory, cores, threads, or other resources.

Owns

- pragma keyword;
- pragma name;
- qualified pragma name;
- pragma payload;
- positional arguments;
- named arguments;
- structured values;
- arrays;
- objects;
- tuples;
- tagged values;
- pragma termination;
- pragma grouping.

Does not own

- pragma interpretation;
- compiler execution;
- process spawning;
- filesystem access;
- network access;
- backend invocation;
- hardware discovery;
- optimization;
- scheduling.

A parsed pragma is data until a downstream semantic component explicitly gives it meaning.

---

20. Pragma Security Boundary

A pragma such as:

pragma tool::execute("command");

must remain syntactically representable data if the language permits that namespace.

Parsing it MUST NOT execute anything.

The parser MUST NOT:

- spawn processes;
- open files;
- access environment variables;
- contact networks;
- access credentials;
- discover hardware;
- invoke compilers;
- invoke backends;
- modify the filesystem.

Interpretation must occur under explicit semantic/compiler policy.

---

21. Open-World Extensibility

Core syntax must avoid enumerating every future computational concept.

For example, the language should be able to represent a future qualified construct such as:

future::photonic
future::neuromorphic
future::molecular
future::optical
future::biological
future::unknown_domain

without modifying the fundamental name grammar merely because a new namespace was introduced.

This is essential for:

Run Forever

in POCO-REAF.

Future domain semantics may require new grammar files, but the foundational name system must not artificially prevent them.

---

22. No Machine-Specific Core Syntax

The following must never become core grammar constants:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_NODES
MAX_DEVICES
MAX_MEMORY
MAX_REGISTERS
MAX_TENSOR_RANK

Nor should core grammar encode:

q[0]
q[1]
cpu0
gpu0
device0
node0

as mandatory physical assumptions.

A source program may explicitly name a resource when the semantics genuinely require it, but the core grammar must not impose such identities or capacities.

---

23. Resource Semantics Boundary

The following concepts must remain distinct:

requirement
capability
constraint
preference
hint
resource
target
placement

They must not collapse into a single generic directive.

Requirement

Must be satisfied.

Capability

Describes what an environment can provide.

Constraint

Limits acceptable implementations.

Preference

Expresses desired behavior without necessarily being mandatory.

Hint

Provides implementation guidance without changing semantic validity.

Resource

Represents something consumed, reserved, or required.

Target

Describes an intended compilation/execution domain.

Placement

Determines where something is realized.

The core grammar only supplies syntax.

Semantic analysis owns the distinctions.

---

24. Quantum Integration

Core grammar may provide generic syntax for quantum-related:

- requirements;
- capabilities;
- constraints;
- hints;
- metadata;
- pragmas.

It must not create quantum semantic objects.

For example:

pragma quantum::dynamic_control;
requires quantum;

may be parsed.

But core grammar does not create:

QubitId
PhysicalQubitId
GateKind
QuantumCircuit
quantum::ir

Quantum syntax belongs under:

grammar/quantum/

and quantum semantic lowering ultimately targets the repository's canonical:

quantum::ir

Core grammar therefore remains upstream of the quantum IR and never duplicates it.

---

25. Classical Integration

Core constructs may be consumed by:

grammar/classical/

and the classical semantic/IR layers.

Core does not define:

- arithmetic semantics;
- memory layout;
- CPU execution;
- vectorization;
- tensor operations;
- accelerator lowering.

Those belong to their respective domains.

---

26. HDL and Hardware Integration

Core syntax may express:

requires hardware_description
requires timing
requires accelerator

or equivalent language-approved forms.

But core does not define:

- ports;
- wires;
- clocks;
- registers;
- physical pins;
- FPGA resources;
- ASIC cells;
- placement;
- routing.

Those belong to:

grammar/hdl/
grammar/hardware/

and their downstream semantic systems.

---

27. Distributed Integration

Core requirements/capabilities/constraints may describe distributed intent.

Core does not own:

- node discovery;
- network topology;
- service placement;
- replication;
- distributed scheduling;
- consensus;
- transport protocols.

Those belong to:

grammar/distributed/
grammar/networking/

and downstream systems.

---

28. AST Contract

Core grammar must produce parse structures that can be represented by the repository's canonical AST.

The grammar must not define a second AST.

The AST layer owns:

- source spans;
- syntax node representation;
- normalized structural representation;
- source-preservation requirements.

Core grammar provides parse structure only.

If a core grammar feature cannot be represented by the existing AST, the AST contract must be extended deliberately rather than embedding semantic information into the grammar.

---

29. Semantic Contract

After parsing:

source
  ↓
parse tree
  ↓
AST
  ↓
semantic analysis

Semantic analysis determines:

- whether names resolve;
- whether a pragma is recognized;
- whether a capability exists;
- whether a requirement is satisfiable;
- whether a constraint is contradictory;
- whether a hint is applicable;
- whether a version is compatible;
- whether an annotation is valid in context.

The grammar MUST NOT perform those decisions.

---

30. IR Contract

Core grammar has no direct ownership of IR.

The correct direction is:

core grammar
    ↓
AST
    ↓
semantic analysis
    ↓
canonical semantic representation
    ↓
domain IR

For quantum:

grammar/quantum/
    ↓
AST
    ↓
semantic lowering
    ↓
quantum::ir

The core grammar must never introduce an alternative quantum IR.

---

31. Compiler Integration

Compiler stages consume semantic information originating from core syntax.

Potential consumers include:

- type checker;
- capability checker;
- requirement checker;
- effect checker;
- resource analysis;
- target selection;
- optimization;
- scheduling;
- routing;
- lowering;
- code generation.

Core grammar itself must remain compiler-independent.

---

32. Runtime Integration

There is no direct runtime dependency on "grammar/core".

The runtime should receive compiled semantic artifacts rather than reparsing core grammar.

This prevents:

runtime -> grammar

from becoming a mandatory architectural dependency.

A runtime may retain provenance or metadata derived from source, but that is a compiler/artifact contract rather than a grammar/runtime coupling.

---

33. Hardware Integration

There is no direct hardware dependency.

Hardware capabilities may satisfy semantic requirements originating from core grammar.

The direction is:

source requirement
    ↓
semantic requirement
    ↓
capability evaluation
    ↓
hardware/runtime capability

Never:

hardware
    ↓
grammar modification

---

34. Scheduling Integration

Core grammar can express scheduling-related intent through the appropriate resource/constraint/hint abstractions.

It must not implement scheduling.

Scheduling belongs downstream.

For example:

prefer low latency

can become scheduling input.

It must not cause "core/" to know about:

- scheduler algorithms;
- DAG scheduling;
- ASAP;
- ALAP;
- RCPSP;
- physical timing grids.

---

35. Optimization Integration

Core syntax may express optimization preferences.

It must not implement optimization.

Optimization consumes semantic information after parsing and analysis.

---

36. QEC / ZQN / Resilience Integration

Core grammar does not own:

- QEC algorithms;
- fault models;
- noise models;
- mitigation algorithms;
- resilience policy;
- recovery;
- retry;
- checkpoint execution.

Core syntax may express portable intent or requirements that downstream systems interpret.

For example, a quantum resilience-related declaration may eventually become semantic input to the resilience subsystem, but "grammar/core/" remains unaware of recovery implementation.

---

37. Determinism

Core grammar must be deterministic.

It must contain no:

- semantic predicates dependent on runtime state;
- random behavior;
- filesystem access;
- network calls;
- environment inspection;
- hardware discovery;
- runtime callbacks;
- embedded unsafe code.

The same token stream must produce the same parse structure.

---

38. Rust Compatibility

The grammar infrastructure must remain compatible with:

Rust 1.97
Rust 1.97.1

The implementation must use safe Rust only.

There must be:

no unsafe

in the grammar implementation or grammar support infrastructure.

ANTLR-generated or parser-integration code must be reviewed so that the project's Rust safety policy remains intact.

---

39. Scalability

Grammar scalability means that syntax does not impose arbitrary semantic limits.

The grammar must not impose fixed limits on:

- number of source items;
- declarations;
- modules;
- imports;
- attributes;
- annotations;
- pragma count;
- argument count;
- nesting represented by grammar repetition;
- qualified-name components;
- resource declarations;
- capability declarations;
- requirements;
- constraints.

Use grammar repetition and recursive structures where appropriate.

Actual limits caused by:

- available memory;
- parser implementation;
- stack;
- execution time;
- operating-system limits;

are implementation-resource constraints rather than language semantics.

---

40. "Infinity" Interpretation

"Infinity" means:

«No artificial language-level upper bound is introduced where the underlying semantics do not require one.»

It does not mean that a finite computer has infinite memory or processing capacity.

The correct model is:

language capacity
    ≤
implementation capacity
    ≤
available physical resources

The grammar must not reduce the first term unnecessarily.

---

41. Hard-Coding Audit

Every core grammar change must be checked for:

fixed counts
fixed capacities
fixed resource IDs
fixed hardware IDs
fixed topology
fixed device assumptions
fixed qubit counts
fixed processor counts
fixed deployment assumptions

Every detected limit must be classified as one of:

1. language semantic requirement;
2. target requirement;
3. resource constraint;
4. implementation limitation;
5. accidental hard-coding;
6. test-only limitation;
7. documentation-only limitation.

Accidental hard-coding must be removed.

---

42. Error Handling

Core grammar should allow the parser infrastructure to provide precise source diagnostics.

Diagnostics should preserve:

- source location;
- offending token;
- expected syntax;
- relevant grammar construct;
- recovery location where recovery is supported.

Semantic errors must remain distinct from syntax errors.

For example:

syntax error

must not be conflated with:

capability unavailable

or:

requirement unsatisfied

or:

target cannot satisfy constraint

---

43. Error Recovery

Parser recovery must not change language semantics.

Recovery exists to improve diagnostics.

A recovered parse tree must not accidentally be treated as a valid semantic program.

Downstream semantic validation must know whether the parse completed without syntax errors.

---

44. Source Preservation

Where tooling requires it, the core grammar must preserve enough structure for:

- formatter;
- syntax highlighter;
- IDE tooling;
- language server;
- diagnostics;
- source maps;
- refactoring;
- round-trip serialization.

The AST should retain source spans independently of the grammar.

---

45. Compatibility

Core grammar changes are language compatibility changes.

Before changing a rule:

1. identify current syntax;
2. identify consumers;
3. identify examples;
4. identify tests;
5. identify documentation;
6. determine whether the change is additive;
7. determine whether it changes parse ambiguity;
8. determine whether it changes AST shape;
9. determine whether it changes semantic meaning;
10. define migration behavior if required.

No existing valid language feature should be silently removed.

---

46. Reserved Space

The core grammar must reserve only identifiers that genuinely require language-level reservation.

Future computational domains should preferentially use qualified namespaces rather than forcing all future terminology into the global keyword set.

This allows Zamani to grow without repeatedly destabilizing the lexer.

---

47. Testing Contract

Every core grammar file requires:

Positive tests

Valid examples for every rule.

Negative tests

Invalid syntax.

Boundary tests

Large valid structures constrained only by available test resources.

Determinism tests

Repeated parsing produces equivalent syntax structures.

Round-trip tests

Where supported:

source
→ lexer
→ parser
→ AST
→ formatter/serializer
→ parser

must preserve intended semantics.

Cross-domain tests

At minimum:

core + classical
core + quantum
core + hybrid
core + HDL
core + hardware
core + distributed
core + AI
core + networking
core + security

and combinations such as:

classical + quantum + distributed
classical + quantum + HDL + hardware
AI + quantum + accelerator

---

48. Core-Specific Test Matrix

File| Required tests
"compilation-unit.g4"| complete programs, mixed domains, EOF, empty source
"source-unit.g4"| source ordering and grouping
"names.g4"| valid/invalid identifiers and reserved names
"paths.g4"| relative/qualified/path forms
"qualified-names.g4"| arbitrary qualification depth
"attributes.g4"| declaration attachment and arguments
"annotations.g4"| annotation structure and nesting
"metadata.g4"| metadata forms and preservation
"versioning.g4"| versions, ranges, compatibility forms
"capabilities.g4"| capability declarations/references
"requirements.g4"| requirements and combinations
"constraints.g4"| constraint structures
"hints.g4"| advisory forms
"pragmas.g4"| positional/named/structured pragma values

---

49. Dependency Order

The core files should be implemented in dependency-first order.

1. names.g4
       ↓
2. qualified-names.g4
       ↓
3. paths.g4
       ↓
4. versioning.g4
       ↓
5. attributes.g4
       ↓
6. annotations.g4
       ↓
7. metadata.g4
       ↓
8. capabilities.g4
       ↓
9. requirements.g4
       ↓
10. constraints.g4
       ↓
11. hints.g4
       ↓
12. pragmas.g4
       ↓
13. source-unit.g4
       ↓
14. compilation-unit.g4

The exact ANTLR composition mechanism may require some files to be implemented together during grammar assembly, but their semantic ownership remains separate.

---

50. Integration With Other Grammar Domains

Core provides foundational rules consumed by:

types/
expressions/
statements/
declarations/
functions/
modules/
effects/
memory/
concurrency/
classical/
quantum/
hybrid/
hdl/
hardware/
distributed/
ai/
data/
networking/
security/
resources/
compile/
execution/
interoperability/
dialects/
macros/
metaprogramming/

These domains must consume core rules rather than redefine:

identifier
qualifiedName
path
attribute
annotation
metadata
capability
requirement
constraint
hint
pragma

Duplicate definitions are prohibited unless there is a documented semantic distinction.

---

51. Single-Authority Rule

There must be one canonical owner for each fundamental syntactic concept.

For example:

identifier
    -> core/names.g4

qualifiedName
    -> core/qualified-names.g4

pragma
    -> core/pragmas.g4

requirement
    -> core/requirements.g4

Other grammar domains reference these constructs.

They do not silently redefine them.

---

52. No Grammar-to-IR Leakage

A core grammar rule must never contain implementation concepts such as:

QubitId
PhysicalQubitId
ScheduleId
OperationId
DeviceId
BackendId
CalibrationId
ResourceManager

Those are semantic/runtime identifiers.

Grammar-level names remain source-language names.

---

53. No Grammar-to-Hardware Leakage

Core grammar must never query or assume:

CPU architecture
GPU architecture
FPGA family
ASIC technology
QPU topology
memory size
network topology
device availability
calibration

Such information enters compilation through a target/capability/resource context.

---

54. No Grammar-to-Runtime Leakage

The parser must not:

execute
schedule
allocate
route
retry
recover
measure hardware
discover hardware
invoke hardware

Parsing is a pure source transformation.

---

55. Security Requirements

Core grammar infrastructure must be safe against:

- parser-triggered execution;
- path traversal through parsing;
- command execution;
- network access;
- credential access;
- environment-dependent semantics;
- resource exhaustion caused by unnecessary fixed expansion;
- ambiguous directive interpretation.

Input should be treated as untrusted source text.

---

56. Tooling Integration

The core grammar is expected to support:

- compiler frontends;
- parser diagnostics;
- syntax highlighting;
- formatter;
- language server;
- IDE tooling;
- source analyzers;
- documentation generators;
- grammar validation;
- compatibility tooling;
- AST inspection;
- source-to-source transformation.

Tooling must consume the canonical grammar rather than maintain incompatible copies.

---

57. Documentation Integration

The documentation hierarchy should be:

grammar/
├── README.md
├── Zamani-Grammar.md
├── grammar.md
│
└── core/
    └── README.md

"grammar/core/README.md" documents this subsystem.

It must not become a competing language specification.

The normative syntax authority must remain explicitly identified by the grammar-authority specification.

---

58. Generated Artifacts

Generated parser/lexer files must not be manually maintained as independent grammar authorities.

The dependency must remain:

canonical grammar
      ↓
grammar generation
      ↓
generated parser
      ↓
Rust integration

Generated output is derived.

It is not the source of truth.

---

59. ANTLR Boundary

The core grammar is parser-level syntax.

Lexer ownership remains with the canonical Zamani lexer.

Therefore core parser grammars should use the repository's canonical token vocabulary instead of independently redefining lexical tokens.

This prevents divergent definitions of:

IDENTIFIER
STRING_LITERAL
INTEGER_LITERAL
BOOLEAN_LITERAL

and similar tokens.

---

60. Rust Boundary

The ".g4" files are language grammar specifications.

Rust owns:

- parser integration;
- AST implementation;
- diagnostics;
- semantic analysis;
- compiler integration;
- tooling;
- runtime integration.

The ".g4" files must not embed Rust actions to perform semantic work.

---

61. Completion Criteria

"grammar/core/" is complete only when:

- every core rule has one owner;
- no core concept is duplicated;
- all dependencies are documented;
- all downstream consumers are identified;
- AST mappings are defined;
- semantic ownership is defined;
- IR boundaries are defined;
- compiler boundaries are defined;
- runtime non-dependency is documented;
- hardware non-dependency is documented;
- quantum::ir remains downstream;
- no fixed machine capacities exist;
- no unsafe implementation is required;
- deterministic parsing is demonstrated;
- positive tests pass;
- negative tests pass;
- boundary tests pass;
- cross-domain tests pass;
- compatibility tests pass;
- round-trip tests pass where applicable;
- hard-coding audit passes;
- generated parser artifacts are reproducible;
- documentation agrees with the grammar;
- grammar authority is unambiguous.

---

62. Definition of Done for an Individual Core File

A core file is not "done" merely because ANTLR accepts it.

It is done only when all of the following are true:

[ ] Purpose documented
[ ] Ownership documented
[ ] Non-ownership documented
[ ] Dependencies fixed
[ ] Upstream contracts fixed
[ ] Downstream contracts fixed
[ ] AST representation defined
[ ] Semantic interpretation boundary defined
[ ] IR boundary defined
[ ] Compiler integration defined
[ ] Runtime dependency explicitly classified
[ ] Hardware dependency explicitly classified
[ ] Quantum dependency explicitly classified
[ ] Tooling integration defined
[ ] Compatibility behavior defined
[ ] Scalability reviewed
[ ] Hard-coding audit passed
[ ] Positive tests implemented
[ ] Negative tests implemented
[ ] Boundary tests implemented
[ ] Determinism tests implemented
[ ] Cross-domain tests implemented where applicable
[ ] Documentation synchronized
[ ] Generated parser integration verified

Once those conditions are satisfied, later implementation of another domain should not require redesigning the completed core contract merely because that domain happens to be quantum, GPU, FPGA, distributed, or a future computational technology.

---

63. Architectural Invariants

The following invariants are mandatory.

Invariant 1 — Syntax is not semantics

parse != validate

Invariant 2 — Semantics are not hardware

semantic requirement != physical allocation

Invariant 3 — Capability is not selection

capability != device

Invariant 4 — Requirement is not preference

requirement != hint

Invariant 5 — Grammar is not IR

grammar != quantum::ir
grammar != classical IR
grammar != HDL IR

Invariant 6 — Grammar is not runtime

grammar != runtime

Invariant 7 — Future domains must not require rewriting the foundation

New domains should extend the language through existing core abstractions wherever possible.

Invariant 8 — No accidental machine ceiling

The grammar must never silently impose a machine-size ceiling.

Invariant 9 — One source meaning

A program has one semantic meaning independent of its eventual hardware realization.

Invariant 10 — POCO-REAF

one program
    ↓
one semantic meaning
    ↓
many compilation targets
    ↓
many hardware configurations
    ↓
many execution environments
    ↓
future platforms

---

64. Final Core Architecture

The production architecture is therefore:

                         Zamani Source
                              |
                              v
                    +-------------------+
                    | Canonical Lexer   |
                    +-------------------+
                              |
                              v
                    +-------------------+
                    |    grammar/core   |
                    |                   |
                    | names             |
                    | paths             |
                    | qualified names   |
                    | attributes        |
                    | annotations       |
                    | metadata          |
                    | versioning        |
                    | capabilities      |
                    | requirements      |
                    | constraints       |
                    | hints             |
                    | pragmas           |
                    +-------------------+
                              |
                              v
                    +-------------------+
                    | Domain Grammars   |
                    +-------------------+
                      /       |       \
                     /        |        \
              classical    quantum     HDL
                  |           |          |
                  |           |          |
                  v           v          v
                AST / Semantic Analysis
                         |
                         v
                Canonical Semantic Model
                    /              \
                   /                \
                  v                  v
          Classical IR          quantum::ir
                  \                  /
                   \                /
                    +--------------+
                           |
                           v
                    Optimization
                           |
                           v
                  Routing / Scheduling
                           |
                 +---------+---------+
                 |                   |
                 v                   v
                QEC                 ZQN
                 |                   |
                 +---------+---------+
                           |
                           v
                      Resilience
                           |
                           v
                    Target Lowering
                           |
                           v
                    Hardware / Runtime

The core grammar is therefore a stable syntactic foundation, not a machine-specific execution language.

Its job is to make the rest of Zamani extensible without sacrificing semantic stability.

The governing rule is:

«Zamani source describes what computation means and what properties its realization must satisfy. The compiler, scheduler, router, hardware abstraction, runtime, and deployment layers determine how that computation is realized with whatever resources are actually available.»

That separation is what allows the same core language to scale from an extremely small system to arbitrarily larger systems supported by the implementation and available resources, without putting an artificial machine ceiling into the language itself.