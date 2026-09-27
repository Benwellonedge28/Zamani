Zamani Compilation Grammar

Production Architecture, Ownership, Integration, Scalability, and POCO-REAF Contract

Path: "grammar/compile/README.md"
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Grammar technology: ANTLR-compatible grammar
Rust implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Rust safety policy: production Rust MUST NOT use "unsafe"
Primary portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)
Scalability objective: from the smallest useful computation to arbitrarily large computations, bounded by program semantics, representational constraints, explicitly declared policies, and resources actually available to the compiler, runtime, and target.

---

1. Status

This document is the normative architecture and integration contract for:

grammar/compile/

It defines what the compilation grammar owns, what every compilation grammar file owns, how those files integrate with the rest of "grammar/", how compilation syntax maps into the frontend, and how compilation intent reaches the canonical compiler/IR pipeline.

This document does not claim that every compilation construct currently has a complete implementation.

A compilation feature is production-complete only when its entire required pipeline exists:

Specification
    ↓
Lexical contract
    ↓
Grammar
    ↓
Lexer
    ↓
Parser
    ↓
Frontend AST
    ↓
Structural validation
    ↓
Name/module resolution
    ↓
Type analysis
    ↓
Effect analysis
    ↓
Resource analysis
    ↓
Capability analysis
    ↓
Portability analysis
    ↓
Compilation semantic model
    ↓
Canonical IR / semantic representation
    ↓
Optimization / lowering
    ↓
Target realization
    ↓
Backend / runtime
    ↓
Tests

A parser accepting syntax is therefore not sufficient to classify that feature as production-ready.

---

2. Mission

The compilation grammar exists to let a Zamani program express compilation-related intent without turning source code into a description of one particular machine.

Compilation syntax may describe:

- compilation intent;
- compilation policies;
- compilation profiles;
- target requirements;
- target capabilities;
- resource requirements;
- optimization objectives;
- optimization constraints;
- optimization preferences;
- specialization intent;
- feature selection;
- conditional compilation;
- code-generation intent;
- lowering intent;
- artifact intent;
- reproducibility requirements;
- deterministic-build requirements;
- caching intent;
- provenance intent;
- cross-compilation intent;
- deployment intent at the compilation boundary;
- compile-time evaluation;
- compile-time generated regions;
- portability requirements.

It MUST NOT become responsible for actually performing those operations.

The fundamental boundary is:

Zamani source
     ↓
express compilation intent
     ↓
frontend semantic analysis
     ↓
canonical semantic representation
     ↓
canonical IR
     ↓
compiler implementation
     ↓
optimization
     ↓
lowering
     ↓
target/capability/resource resolution
     ↓
routing / scheduling where applicable
     ↓
QEC / ZQN where applicable
     ↓
HAL / backend
     ↓
runtime / deployment

---

3. Fundamental Compilation Principle

Zamani source describes:

WHAT

Compilation infrastructure determines:

HOW
WHERE
WHEN
WITH WHICH AVAILABLE RESOURCES
ON WHICH COMPATIBLE TARGET

Therefore:

source intent
    ≠
physical realization

and:

compilation policy
    ≠
compiler implementation

and:

target requirement
    ≠
hard-coded target selection

This distinction is mandatory for POCO-REAF.

---

4. POCO-REAF

4.1 Definition

POCO-REAF means:

«A Zamani program should be expressible in terms of stable semantics and portable compilation intent rather than being rewritten merely because the available hardware, execution environment, accelerator, QPU, compiler backend, or machine scale changes.»

The same source program may therefore be compiled for:

tiny embedded system
single CPU
multicore CPU
GPU
FPGA
ASIC
accelerator
QPU
quantum simulator
HPC system
cluster
distributed system
cloud system
edge system
heterogeneous system
future computational target

provided the target can satisfy the program's semantic requirements.

POCO-REAF does not mean:

«every program must execute on every target regardless of resource availability.»

For example:

requires qubits >= n

may be semantically valid while a particular target lacks sufficient resources.

The compiler must then perform one of the explicitly permitted actions:

select another compatible target
use an available realization
distribute the computation
simulate where permitted
lower/decompose where semantically valid
wait for resources where the execution model permits it
or report an explicit incompatibility

It MUST NOT silently change the meaning of the program.

---

5. Scalability Contract

The compilation grammar MUST scale without artificial language-level ceilings.

Conceptually it must support:

one operation
one value
one function
one target
one artifact
one resource
one qubit

through:

arbitrarily many operations
arbitrarily large programs
arbitrarily large datasets
arbitrarily large tensors
arbitrarily large classical computations
arbitrarily large quantum computations
arbitrarily large hybrid computations
arbitrarily large hardware descriptions
arbitrarily large distributed computations
arbitrarily large compilation plans

subject only to:

- language semantics;
- representation constraints;
- declared program constraints;
- compiler implementation resources;
- runtime resources;
- target capabilities;
- target resource availability;
- operating-environment constraints.

---

6. Hard-Coding Prohibition

The compilation grammar MUST NOT establish artificial universal machine limits.

In particular, compilation syntax MUST NOT encode language ceilings such as:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_QPUS
MAX_NODES
MAX_DEVICES
MAX_ACCELERATORS
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_NETWORK_SIZE
MAX_CHANNELS
MAX_TIMELINES
MAX_ARTIFACTS
MAX_COMPILATION_STAGES
MAX_TARGETS
MAX_DEPENDENCIES

The list is illustrative, not exhaustive.

The prohibition also applies to disguised finite grammar alternatives.

For example, the following architectural model is prohibited:

target0
target1
target2
...
target15

if those alternatives represent a universal language limit.

Likewise:

physical_qubit_0
physical_qubit_1
...
physical_qubit_127

MUST NOT constitute the universal source-language quantum model.

---

7. Constants Are Not Automatically Hard-Coding

Ordinary program constants remain legal.

For example:

let n = 1024;

is program data.

Likewise:

Tensor<1024, 1024>

may be a valid program-level type/value.

The prohibition concerns compiler-defined artificial ceilings such as:

the language supports no more than 1024 qubits

or:

the compiler grammar accepts no tensor dimension greater than 1024

unless such a restriction is explicitly and intentionally part of a particular semantic type/profile/target contract.

---

8. Requirement, Capability, Constraint, Preference, Hint, Realization

Compilation syntax MUST preserve the distinction between these concepts.

8.1 Requirement

A property is mandatory.

Conceptually:

requires qubits >= n
requires memory >= required_memory

---

8.2 Capability

A capability is mandatory.

Conceptually:

requires capability("quantum.measurement")
requires capability("tensor.compute")
requires capability("gpu.compute")

---

8.3 Constraint

A property must satisfy a specified condition.

Conceptually:

requires latency <= budget

---

8.4 Preference

A characteristic is preferred but not mandatory.

Conceptually:

prefer accelerator("quantum")

---

8.5 Hint

A hint assists compilation without changing program meaning.

Conceptually:

hint locality
hint parallel
hint vectorize

---

8.6 Realization

A downstream compiler chooses an actual implementation.

Conceptually:

logical resource
    ↓
physical resource

or:

portable operation
    ↓
target-specific implementation

Physical realization MUST NOT leak backward and become a hidden source-language requirement.

---

9. Compilation Grammar Does Not Own Hardware

The compilation grammar MUST NOT directly own:

- physical CPU identities;
- physical GPU identities;
- physical FPGA identities;
- physical QPU identities;
- physical qubit identities;
- physical memory addresses;
- hardware discovery;
- hardware calibration;
- device enumeration;
- backend implementation;
- provider APIs;
- routing algorithms;
- scheduling algorithms;
- QEC algorithms;
- ZQN implementation;
- HAL implementation.

The source may express requirements that downstream hardware/resource infrastructure evaluates.

For example:

requires capability("quantum.mid_circuit_measurement")

is compilation/resource intent.

Selecting:

QPU provider X
device Y
physical qubit 37

is downstream realization.

---

10. Existing Repository Authority Model

The compilation grammar is part of one language.

It MUST integrate with, and never compete against:

grammar/DESIGN.md
grammar/README.md
grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/specification/
grammar/spec/
src/lexer.rs
src/parser.rs
src/ast/
src/quantum/ir/
ZAMANI_IR_SPEC.md

The compilation directory MUST NOT create a second language.

---

11. Authority Hierarchy

The repository-wide authority relationship is:

grammar/DESIGN.md
        ↓
language/specification contracts
        ↓
grammar/spec contracts
        ↓
canonical Zamani.g4
        ↓
Rust lexer/parser conformance
        ↓
frontend AST
        ↓
semantic analysis
        ↓
canonical IR
        ↓
compiler/backend/runtime

The roles are distinct.

"grammar/DESIGN.md"

Owns architecture.

"grammar/specification/"

Owns normative language meaning.

"grammar/spec/"

Owns formal feature contracts and conformance specifications.

"grammar/Zamani.g4"

Owns canonical ANTLR syntax representation.

"grammar/compile/*.g4"

Owns modular compilation syntax represented through the canonical grammar composition mechanism.

"grammar/grammar.md"

Documents implementation conformance.

"grammar/Zamani-Grammar.md"

Contains historical, extended, experimental, or proposed design material.

"src/lexer.rs"

Owns actual Rust lexical implementation.

"src/parser.rs"

Owns actual Rust parser implementation.

"src/ast/"

Owns the frontend AST.

"src/quantum/ir/"

Owns the canonical quantum IR boundary.

"ZAMANI_IR_SPEC.md"

Defines the repository's existing root IR specification and must be reconciled with the broader canonical IR architecture rather than duplicated by compilation grammar.

---

12. Current "grammar/compile/" File Set

The existing compilation directory contains these major files:

grammar/compile/
├── README.md
├── compile.g4
├── compilation.g4
├── compile-time.g4
├── conditional-compilation.g4
├── cross-compilation.g4
├── profiles.g4
├── target.g4
├── target-selection.g4
├── feature-selection.g4
├── optimization.g4
├── specialization.g4
├── code-generation.g4
├── lowering.g4
├── artifacts.g4
├── caching.g4
├── deterministic-builds.g4
├── reproducibility.g4
├── provenance.g4
└── deployment.g4

These files MUST NOT evolve into mutually competing composition roots.

The most important architectural correction is therefore:

ONE canonical compilation composition boundary
+
independent feature grammars
+
one canonical Zamani root

---

13. "grammar/compile/README.md"

This file owns the architecture of the compilation grammar subsystem.

It MUST define:

- purpose;
- scope;
- authority;
- ownership;
- non-ownership;
- file boundaries;
- dependency direction;
- AST integration;
- semantic integration;
- IR integration;
- compiler integration;
- target integration;
- resource/capability integration;
- POCO-REAF;
- scalability;
- hard-coding policy;
- diagnostics;
- compatibility;
- production criteria;
- testing requirements.

It MUST NOT define individual grammar productions in enough detail to become a second grammar.

---

14. "grammar/compile/compile.g4"

Ownership

"compile.g4" is the canonical compilation composition boundary.

It owns:

- the compilation grammar entry point;
- composition of compilation subgrammars;
- public compilation dispatch;
- integration of compilation constructs;
- stable delegation points.

It does NOT own the internal syntax of every compilation feature.

Conceptually:

Compile
 ├── profiles
 ├── compile-time
 ├── conditional compilation
 ├── target
 ├── target selection
 ├── feature selection
 ├── optimization
 ├── specialization
 ├── code generation
 ├── lowering
 ├── cross compilation
 ├── artifacts
 ├── caching
 ├── deterministic builds
 ├── reproducibility
 ├── provenance
 └── deployment integration

Critical invariant

There MUST be one canonical compilation composition root.

---

15. "grammar/compile/compilation.g4"

The current repository contains "compilation.g4", and this file overlaps conceptually with "compile.g4".

It MUST NOT become a second independent public compilation language.

Its final role must be one of:

1. a subordinate compilation-plan grammar;
2. a compatibility grammar;
3. a specialized compilation-orchestration grammar;
4. or, after dependency/reference audit, be retired.

The repository MUST NOT maintain:

compile.g4 = language A
compilation.g4 = language B

Both must describe the same language.

If "compile.g4" remains the composition root, "compilation.g4" must delegate to or specialize a clearly separate concept.

If "compilation.g4" is chosen as the composition root instead, "compile.g4" must be reduced accordingly.

The decision MUST be recorded in the repository's authority documentation and MUST NOT create two parser roots.

---

16. "grammar/compile/compile-time.g4"

Ownership

Owns source syntax whose semantics explicitly occur during compilation.

Examples:

- compile-time expressions;
- compile-time assertions;
- compile-time conditions;
- compile-time generated regions;
- compile-time selection;
- compile-time specialization;
- compile-time evaluation.

It does NOT own general expression syntax.

Expressions MUST integrate with:

grammar/expressions/

Types MUST integrate with:

grammar/types/

Functions MUST integrate with:

grammar/functions/

Semantic rule

The grammar identifies compile-time intent.

The compiler determines whether and how the operation can actually be evaluated.

---

17. "grammar/compile/conditional-compilation.g4"

Ownership

Owns conditional compilation syntax.

It MUST support conditions based on semantic compilation information such as:

- feature availability;
- declared configuration;
- target capabilities;
- compilation profile;
- language version;
- explicitly supplied compilation facts.

It MUST NOT directly query hardware.

Bad architecture:

if cpu_count > 16

when the parser itself treats physical hardware as source semantics.

Preferred architecture:

if capability("parallel.compute")

or a semantic configuration predicate.

The actual condition evaluation belongs downstream.

---

18. "grammar/compile/profiles.g4"

Ownership

Owns named compilation profiles.

A profile may describe:

- optimization policy;
- diagnostics policy;
- reproducibility policy;
- determinism policy;
- specialization policy;
- code-generation policy;
- artifact policy;
- portability policy.

A profile MUST NOT hard-code physical hardware as its universal meaning.

For example, a profile named:

release

may have semantic compilation behavior.

A profile MUST NOT silently mean:

run on 8 CPUs

unless that is explicitly represented as a target-specific profile outside the universal language contract.

Integration

Profiles are consumed by:

semantic analysis
compile planner
optimization planner
artifact generation
reproducibility
diagnostics

---

19. "grammar/compile/target.g4"

Ownership

Owns source-level target intent.

It may express:

- target class;
- target capabilities;
- target requirements;
- target constraints;
- target preferences;
- target-independent target predicates;
- target compatibility intent.

It MUST NOT become a physical-device language.

It MUST NOT require source syntax such as:

device = gpu0
qpu = device_17
physical_qubit = 42
cpu_core = 7
memory_address = ...

as the universal model.

Target realization belongs downstream.

---

20. "grammar/compile/target-selection.g4"

Ownership

Owns target-selection criteria.

It may express:

requires capability(...)
requires resource(...)
prefer capability(...)
requires topology(...)

where these are semantic requirements/criteria.

It MUST distinguish:

requirement
constraint
preference
hint

The grammar MUST NOT embed an algorithm for ranking physical machines.

The target-selection implementation belongs downstream.

---

21. "grammar/compile/feature-selection.g4"

Ownership

Owns source-level selection of language/compiler features.

Examples include:

feature("quantum")
feature("gpu")
feature("hdl")
feature("distributed")

where the feature identifier is semantic data.

Feature selection MUST NOT become:

enable_every_vendor_backend_name_as_keyword

Feature names must remain extensible.

Unknown features must be handled through semantic capability/feature resolution rather than requiring a grammar edit for every future feature.

---

22. "grammar/compile/optimization.g4"

Ownership

Owns optimization intent.

It may describe:

- objectives;
- constraints;
- preferences;
- optimization profile;
- allowed transformations;
- prohibited transformations;
- approximation policy;
- reproducibility policy;
- deterministic policy;
- pass-selection intent;
- pass-exclusion intent;
- optimization budget.

It does NOT implement optimization.

The existing repository already has quantum optimization infrastructure, including optimization profiles, pipelines, target models, planners, and constraints.

The grammar must feed that infrastructure rather than duplicate it.

Important distinction

optimization intent
        ↓
optimization planner
        ↓
optimization passes

not:

grammar
        ↓
execute optimizer

---

23. "grammar/compile/specialization.g4"

Ownership

Owns specialization intent.

Examples:

- specialization conditions;
- generic specialization;
- constant specialization;
- capability specialization;
- profile specialization;
- target-aware specialization.

Specialization MUST preserve semantic equivalence unless the source explicitly permits a weaker semantic contract.

The grammar MUST NOT force a particular CPU/GPU/QPU implementation.

---

24. "grammar/compile/code-generation.g4"

Ownership

Owns code-generation intent.

It may specify:

- output representation class;
- code-generation policy;
- ABI intent;
- calling-convention intent;
- debug/profiling information intent;
- symbol visibility intent;
- artifact requirements.

It does NOT generate machine code.

The implementation belongs to compiler backends.

It MUST NOT hard-code:

x86-only
CUDA-only
QPU-X-only
FPGA-Y-only

into universal grammar semantics.

---

25. "grammar/compile/lowering.g4"

Ownership

Owns source-level lowering intent.

It may describe:

- lowering policy;
- permitted decomposition;
- abstraction-level requirements;
- representation preferences;
- semantic-preservation requirements;
- lowering constraints.

It MUST NOT define the internal implementation of:

- classical lowering;
- quantum lowering;
- HDL synthesis;
- machine-code generation.

Those belong downstream.

---

26. "grammar/compile/cross-compilation.g4"

Ownership

Owns explicit cross-compilation intent.

It may express:

- source environment;
- target environment;
- compatibility constraints;
- target capability requirements;
- portable artifact requirements.

It MUST NOT hard-code a finite list of targets into the language.

Target identifiers must be extensible semantic data.

Cross compilation means:

same program semantics
        ↓
different compatible target realization

not:

rewrite source for every architecture

---

27. "grammar/compile/artifacts.g4"

Ownership

Owns artifact intent.

It may describe:

- requested artifact class;
- artifact identity;
- artifact relationship;
- artifact dependency;
- artifact metadata;
- artifact format intent;
- artifact scope;
- artifact lifetime;
- artifact provenance;
- artifact reproducibility requirements.

It MUST NOT implement:

- artifact storage;
- artifact hashing;
- artifact signing;
- artifact caching;
- artifact deployment;
- artifact execution.

Those belong downstream.

Artifact lists MUST use scalable repetition rather than fixed alternatives.

---

28. "grammar/compile/caching.g4"

Ownership

Owns caching intent.

It may describe:

- cache eligibility;
- cache policy;
- cache invalidation semantics;
- cache identity requirements;
- reuse requirements;
- cache preferences.

It MUST NOT assume:

one local cache
fixed cache size
fixed artifact count
fixed storage capacity

Cache implementation belongs to compiler/build infrastructure.

---

29. "grammar/compile/deterministic-builds.g4"

Ownership

Owns source-level deterministic-build requirements.

It may express that a build must be:

- deterministic;
- reproducible;
- order-independent where semantics permit;
- free from uncontrolled environmental inputs.

It MUST NOT pretend that every backend is automatically deterministic.

Determinism is a semantic/build contract that downstream tooling must satisfy and verify.

---

30. "grammar/compile/reproducibility.g4"

Ownership

Owns reproducibility intent.

It may describe:

- reproducible artifact requirements;
- input identity;
- dependency identity;
- compiler identity;
- profile identity;
- configuration identity;
- provenance requirements.

It integrates with:

compile/provenance.g4
compile/artifacts.g4
compile/deterministic-builds.g4
compile/caching.g4

without duplicating their ownership.

---

31. "grammar/compile/provenance.g4"

Ownership

Owns compilation provenance.

Compilation provenance answers:

«What inputs, compiler state, configuration, profile, and transformations produced this compilation result?»

It MUST remain distinct from:

grammar/data/provenance.g4
grammar/security/provenance.g4

Data provenance and security provenance may be linked downstream, but they are different semantic domains.

Compilation provenance MUST remain target-neutral.

---

32. "grammar/compile/deployment.g4"

The existing repository already recognizes that deployment has a broader execution-domain owner.

Therefore:

grammar/compile/deployment.g4

is a compilation-layer integration adapter, not a second deployment language.

Canonical deployment semantics belong to the appropriate execution/deployment grammar.

This file may express:

deployment requested as part of compilation

but MUST delegate deployment semantics to its canonical owner.

It MUST NOT duplicate:

- deployment lifecycle;
- deployment environment;
- deployment placement;
- deployment rollout;
- deployment recovery;
- deployment execution.

---

33. "grammar/compile/compilation.g4" vs "compile.g4"

This overlap is one of the most important existing issues.

The final architecture MUST establish one of these forms:

Preferred model

Zamani.g4
    ↓
Compile
    ↓
compile.g4
    ↓
dedicated compilation grammars

with:

compilation.g4

being a subordinate compilation-plan grammar.

OR, if repository dependency analysis proves the reverse is more appropriate:

Zamani.g4
    ↓
Compilation
    ↓
dedicated compilation grammars

with "compile.g4" becoming an adapter.

What is forbidden is:

Zamani.g4
 ├── compile.g4
 └── compilation.g4

where both define overlapping independent meanings.

The choice MUST be documented once and then treated as stable.

---

34. ANTLR Composition Contract

Compilation grammars are modular source files.

They MUST be integrated using the repository's actual ANTLR generation strategy.

The grammar subsystem MUST NOT assume that simply placing ".g4" files in the same directory automatically composes them.

The build must explicitly establish:

grammar file
    ↓
ANTLR generation
    ↓
generated parser
    ↓
Rust parser/frontend

No embedded Rust actions are permitted as a substitute for semantic analysis.

No compilation grammar may execute compiler behavior from inside ANTLR actions.

No compilation grammar may perform:

- filesystem access;
- network access;
- hardware discovery;
- compiler execution;
- target probing;
- optimization;
- code generation;
- runtime execution.

---

35. Canonical Zamani Integration

The public language entry point remains:

grammar/Zamani.g4

The compilation subsystem must therefore integrate as:

Zamani.g4
     ↓
compile composition
     ↓
compilation constructs

The compilation directory MUST NOT become a standalone language.

---

36. Lexer Integration

Compilation syntax must use the canonical lexical system.

The compilation grammar MUST NOT invent an independent lexer.

Integration is:

grammar/lexer/
        ↓
src/lexer.rs
        ↓
tokens
        ↓
grammar/compile/*.g4

Compilation identifiers, paths, literals, operators, strings, attributes, and expressions must use the canonical lexical contracts.

This is particularly important because the current Rust lexer already contains a broad keyword/token inventory.

Compilation grammar changes MUST therefore be checked against:

src/lexer.rs
grammar/lexer/tokens.md
grammar/lexer/keywords.md
grammar/lexer/operators.md

before being considered complete.

---

37. Existing Lexer Compatibility

The current lexer contains a large number of keywords and several areas requiring eventual canonicalization.

Compilation grammar work MUST NOT introduce duplicate lexical concepts merely because a compilation feature wants a convenient token.

Examples of issues that belong to the repository-wide lexical audit include duplicate semantic concepts around:

Question / QuestionMark
Ampersand / BitAnd

and discrepancies between documented literals and actual lexer emission.

The compilation subsystem must consume the canonical token model rather than creating local lexical exceptions.

---

38. Parser Integration

The current Rust parser is a recursive-descent / Pratt parser and already handles a broad language surface, including:

- declarations;
- expressions;
- modules;
- control flow;
- quantum constructs;
- effects;
- async constructs;
- nano/Sankofa-related constructs;
- other extensions.

Therefore compilation grammar additions MUST have an explicit parser integration contract.

For every compilation construct:

ANTLR rule
    ↕
Rust parser construct
    ↓
AST node

must be predetermined.

The grammar MUST NOT become a feature graveyard in which ANTLR accepts syntax that "src/parser.rs" cannot represent.

---

39. AST Integration

Compilation syntax MUST map into the existing domain-neutral frontend AST architecture.

The AST MUST NOT become a hardware-specific AST.

The AST representation for compilation intent should conceptually preserve:

CompilationIntent
    ├── profile
    ├── requirements
    ├── capabilities
    ├── constraints
    ├── preferences
    ├── hints
    ├── specialization
    ├── optimization
    ├── target intent
    ├── artifact intent
    ├── reproducibility
    ├── provenance
    └── deployment intent

The exact Rust types belong to "src/ast/" and semantic/compiler implementation.

The grammar README establishes the contract; it does not define those Rust structures.

---

40. AST Independence Requirement

The AST MUST NOT contain target-specific fields such as:

gpu0
qpu7
physical_qubit42
cpu_core3
ram64gb

as universal fields.

Instead, it should preserve semantic information such as:

resource requirement
capability requirement
target predicate
resource preference
mapping intent

The actual target realization occurs later.

---

41. Semantic Integration

After parsing, compilation constructs enter semantic analysis.

The semantic pipeline MUST distinguish:

syntax
    ↓
meaning
    ↓
feasibility
    ↓
realization

For example:

requires qubits >= n

is syntactically parsed first.

Semantic analysis then determines:

- whether "n" is valid;
- what resource kind is being requested;
- whether the requirement is statically knowable;
- whether it is symbolic;
- what capability/resource model it belongs to.

Target feasibility is evaluated later against actual target information.

---

42. Resource Integration

Compilation grammar integrates with:

grammar/resources/

The compilation subsystem MUST NOT duplicate the canonical resource model.

The relationship is:

compile syntax
     ↓
resource intent
     ↓
resource semantic model
     ↓
resource analysis

Resource analysis determines feasibility.

Compilation grammar only describes source intent.

---

43. Capability Integration

Compilation syntax integrates with:

grammar/resources/
grammar/hardware/

and corresponding semantic/compiler capability systems.

A capability should be represented as extensible semantic data:

capability("tensor.compute")
capability("gpu.compute")
capability("quantum.measurement")

rather than requiring one parser keyword for every future capability.

This is essential for long-term extensibility.

---

44. Hardware Integration

Compilation syntax may consume hardware intent from:

grammar/hardware/

but must not own hardware semantics.

The direction is:

compile
   ↓
hardware/resource requirement
   ↓
hardware capability model
   ↓
target discovery
   ↓
target realization

Never:

compile grammar
   ↓
physical hardware database

---

45. Classical Integration

Classical compilation intent must integrate with:

grammar/classical/

and the classical semantic/compiler pipeline.

The compilation grammar MUST remain independent of:

- CPU instruction sets;
- fixed register widths;
- fixed vector widths;
- specific vendor intrinsics.

Those belong to downstream target lowering.

---

46. Quantum Integration

Quantum compilation intent integrates with:

grammar/quantum/
src/quantum/
src/quantum/ir/

The canonical downstream boundary is:

quantum source semantics
        ↓
quantum::ir

There MUST NOT be a second compilation-specific quantum IR.

The compilation subsystem may express:

requires capability("quantum.measurement")
requires capability("quantum.mid_circuit_measurement")
requires capability("quantum.error_correction")
prefer quantum

but must not implement:

- quantum routing;
- gate decomposition;
- physical qubit placement;
- QEC;
- calibration;
- pulse execution;
- hardware scheduling.

---

47. Quantum Gate Extensibility

Compilation syntax MUST NOT assume a fixed universal quantum gate enumeration.

The repository's quantum architecture should support generic semantic operations.

Conceptually:

operation
    ↓
name
namespace
operands
parameters
results
attributes
modifiers
effects
capabilities
source location

This permits:

apply H
apply X
apply custom_gate
apply vendor.operation
apply operation(parameter)

without requiring a grammar change for every future operation.

---

48. Hybrid Integration

Compilation intent must support programs combining:

classical
quantum
HDL/hardware
AI
distributed

without creating separate compilation languages.

The semantic model remains:

one Zamani program
        ↓
one compilation intent model
        ↓
multiple canonical domain representations

where appropriate.

---

49. HDL Integration

Compilation syntax may request hardware artifacts or synthesis intent.

It MUST integrate with:

grammar/hdl/
grammar/hardware/

It must not turn compilation grammar into a second HDL.

For example:

generate hardware

is compilation intent.

The actual:

signals
ports
clocking
timing
synthesis
verification

belong to HDL/hardware grammars.

---

50. AI/Data Integration

Compilation syntax may express optimization or deployment intent for AI/data workloads.

It MUST NOT encode framework-specific compiler syntax such as a fixed framework's API.

The source-level language should remain semantic.

Framework lowering belongs downstream.

---

51. Distributed Integration

Compilation syntax may describe:

distributed artifact
parallel target
communication capability
replication requirement
deployment intent

but it must not impose:

MAX_NODES

or a fixed cluster topology.

Distributed realization belongs downstream.

---

52. Networking Integration

Compilation may express network capability or deployment requirements.

It must not hard-code:

- fixed endpoint counts;
- fixed topology;
- fixed bandwidth;
- fixed address space;
- vendor network implementations.

Networking semantics belong to:

grammar/networking/

and target/runtime infrastructure.

---

53. Security Integration

Compilation may express:

reproducibility
provenance
artifact integrity
trust requirements
verification requirements

but cryptographic implementation belongs to security/compiler infrastructure.

The grammar must not become an algorithm registry.

---

54. Interoperability Integration

Compilation artifacts may target interoperable representations such as:

QIR
OpenQASM
LLVM-related representations
WebAssembly
HDL representations
foreign ABI representations

but these are interoperability targets, not the canonical Zamani semantic model.

The direction is:

Zamani semantics
      ↓
canonical representation
      ↓
interoperability lowering
      ↓
external representation

not:

external representation
      ↓
becomes Zamani's semantic authority

---

55. Canonical IR Boundary

The compilation grammar MUST NOT define another IR.

The repository already has a root-level:

ZAMANI_IR_SPEC.md

and a canonical quantum IR under:

src/quantum/ir/

The compilation grammar must feed semantic/IR construction downstream.

The rule is:

Grammar
    ↓
AST
    ↓
Semantics
    ↓
Canonical IR

not:

Grammar
    ↓
Compilation IR
    ↓
Quantum IR

unless a representation is explicitly defined as a compiler-internal lowering representation rather than a competing semantic authority.

---

56. Quantum IR Rule

The canonical quantum boundary remains:

quantum::ir

Compilation grammar MUST NOT create:

compile::quantum_ir

or another parallel quantum IR.

Quantum compilation intent is metadata/policy/requirements around the canonical semantic program.

---

57. Optimization Integration

The existing repository contains quantum optimization infrastructure including:

src/quantum/optimization/

and associated profiles, targets, constraints, pipelines, and planners.

The compilation grammar must integrate through semantic configuration.

Conceptually:

optimization syntax
        ↓
optimization intent
        ↓
optimization configuration
        ↓
optimization planner
        ↓
optimization pipeline
        ↓
canonical IR

The grammar MUST NOT know how an optimization pass is implemented.

---

58. Routing Integration

Routing is downstream.

Compilation syntax may express:

requires topology(...)
prefer locality
requires connectivity(...)

but routing decides the physical realization.

Therefore:

compile
   ↓
topology requirement
   ↓
routing

not:

compile
   ↓
physical routing algorithm

---

59. Scheduling Integration

Compilation grammar may express scheduling constraints or intent.

It must not implement scheduling.

The downstream pipeline decides:

operation order
resource occupancy
timing
parallelism

subject to semantic correctness.

No universal maximum number of scheduling slots may be encoded.

---

60. Resilience, QEC, and ZQN Integration

Compilation grammar may express:

requires fault_tolerance
requires error_correction
requires reliability(...)
requires noise_budget(...)

but must not implement:

QEC
ZQN
noise modeling
calibration
fault recovery

Those remain downstream responsibilities.

The desired architecture is:

Zamani source
      ↓
semantic compilation intent
      ↓
quantum::ir
      ↓
optimization
      ↓
routing
      ↓
scheduling
      ↓
resilience / QEC
      ↓
ZQN
      ↓
HAL
      ↓
target

---

61. Target Realization

Target realization occurs only after semantic validation.

The compiler may use:

target capabilities
resource availability
topology
memory
latency
power
thermal characteristics
reliability
available accelerators

to construct a valid realization.

These facts MUST NOT retroactively redefine the source program.

---

62. Compile Once vs Execute Everywhere

The phrase:

Compile Once

must be understood as a portability architecture rather than a promise that one binary can physically execute unchanged on every architecture.

The durable object should be a stable semantic compilation artifact or portable representation from which compatible target realizations can be produced.

Therefore the architecture should preserve:

source semantics
        ↓
portable compilation representation
        ↓
target-specific realization

This prevents source rewriting merely because hardware changes.

---

63. Profiles Must Not Destroy Portability

A compilation profile may select policy.

For example:

profile release
profile debug
profile reproducible
profile deterministic
profile performance

But profiles MUST NOT silently alter the meaning of the source computation.

A profile may change:

optimization strategy
artifact format
diagnostic level
debug information
code-generation policy

but semantic changes require explicit language constructs.

---

64. Specialization Must Preserve Semantics

Specialization may use known information:

constant values
capabilities
types
compile-time facts
target properties

but the resulting specialization must preserve the source semantic contract unless the program explicitly permits approximation or target-specific behavior.

---

65. Conditional Compilation Semantics

Conditional compilation MUST distinguish:

source semantics

from:

selected source region

The compiler must evaluate conditions using explicitly defined semantic configuration.

Hardware probing belongs to target discovery.

The grammar itself must never probe hardware.

---

66. Compile-Time Evaluation

Compile-time computation must be bounded by compiler resource policies without becoming a language-level semantic ceiling.

For example:

compile-time computation

may require substantial memory/time.

The compiler may impose operational resource policies.

Those policies are not equivalent to:

language cannot represent larger compile-time computations

The distinction must remain explicit.

---

67. Resource Availability

The compilation system must distinguish:

semantic requirement

from:

currently available resource

For example:

requires memory >= required_memory

is source semantics.

Actual memory availability is an environment fact.

This is critical for:

tiny → large → distributed → future

scalability.

---

68. Resource Negotiation

Where the runtime/compiler architecture supports negotiation, compilation intent may be represented as:

requirements
constraints
preferences
hints

The compiler can then negotiate a valid realization.

A failed negotiation must produce an explicit diagnostic.

It MUST NOT silently weaken requirements.

---

69. Artifact Scalability

Artifact syntax must permit arbitrary collections.

Use semantic repetition:

artifact*
artifact+
dependency*
output*

rather than fixed grammar alternatives.

No artificial:

MAX_ARTIFACTS
MAX_OUTPUTS
MAX_DEPENDENCIES

may be encoded into the language.

---

70. Dependency Scalability

Compilation dependency syntax must support arbitrary dependency graphs.

The grammar must not impose a fixed dependency count or depth.

Compiler implementation may impose operational limits for safety/resource management, but these must remain implementation policies rather than language semantics.

---

71. Compilation Pipeline Scalability

Compilation plans may contain multiple stages.

The language MUST NOT assume:

stage1
stage2
stage3
...
stage8

as a universal maximum.

Instead:

stage*

or an equivalent extensible semantic representation must be used.

---

72. Determinism

Where deterministic compilation is requested:

deterministic

must describe a semantic/build requirement.

The implementation must control:

- iteration order;
- unordered collection effects;
- random seeds where relevant;
- dependency ordering;
- generated identifiers;
- artifact metadata;
- compiler configuration;
- target-independent transformations.

A grammar declaration alone cannot guarantee determinism.

---

73. Reproducibility

A reproducible build requires stable identification of relevant inputs.

The compilation system should be able to account for:

source
dependencies
compiler
language version
grammar version
configuration
profile
semantic inputs
toolchain
target realization where relevant

Provenance belongs downstream, but the source language may request it.

---

74. Caching

Compilation caching must use semantic identity rather than merely file timestamps.

A cache key may ultimately incorporate:

source identity
dependency identity
compiler identity
language version
grammar version
profile
semantic configuration
relevant target properties

The grammar only expresses cache intent.

Cache implementation remains outside grammar.

---

75. Code Generation

Code-generation intent may request:

executable
library
object
intermediate representation
foreign representation
hardware artifact
quantum artifact
verification artifact

but the actual backend decides how to construct it.

No backend must become a mandatory language dependency.

---

76. Lowering

Lowering may transform:

high-level semantic operation
        ↓
lower-level operation

provided semantic invariants are preserved.

The compilation grammar must not assume one fixed lowering chain.

Future backends may introduce new lowerings without requiring the source language to be redesigned.

---

77. Cross-Compilation

Cross compilation must preserve:

program semantics

while changing:

target realization

The source may specify target requirements.

The compiler resolves compatible targets.

Target names remain extensible semantic values.

---

78. Deployment Boundary

Compilation deployment intent ends at:

deployment plan / deployment request

Actual deployment execution belongs downstream.

The grammar MUST NOT:

- open sockets;
- authenticate;
- contact cloud services;
- provision machines;
- launch containers;
- allocate physical devices;
- execute jobs.

---

79. Diagnostics

Every compilation grammar feature must define diagnostic behavior.

Diagnostics should identify:

- source span;
- offending construct;
- expected construct;
- semantic category;
- requirement/capability failure;
- compatibility failure;
- resource failure;
- unsupported feature;
- conflicting constraints;
- invalid profile;
- invalid target intent;
- invalid artifact request.

Diagnostics must not expose implementation details as if they were language semantics.

---

80. Source Span Requirement

Every compilation construct that reaches the AST MUST preserve source location information.

At minimum:

file identity
start position
end position

must remain available for diagnostics and tooling.

This integrates with the existing source-map/span infrastructure.

---

81. Error Recovery

Compilation grammar must be designed so parser recovery does not produce misleading semantic constructs.

Invalid constructs should be recoverable where practical, but semantic analysis must distinguish:

recovered syntax

from:

valid compilation intent

IDE/LSP support must therefore be able to operate on incomplete source without treating recovery nodes as production semantics.

---

82. Security

Compilation grammar must not provide an implicit escape hatch into arbitrary compiler execution.

Grammar files must contain no host-language execution actions.

The compiler implementation must use safe Rust only.

Production implementation MUST NOT use:

unsafe

or equivalent unsafe escape mechanisms.

---

83. Rust Baseline

The compilation subsystem must remain compatible with:

Rust 1.97
Rust 1.97.1
Rust 2021

where those are the repository's supported baselines.

The grammar itself must remain independent of Rust implementation details.

Rust-specific implementation belongs in:

src/

not in grammar actions.

---

84. Existing Toolchain Files

The repository currently contains:

Cargo.toml
rust-toolchain.toml

These files must eventually contain syntactically valid Rust/toolchain configuration.

In particular, version expressions such as:

"1.97" or "1.97.1"

are not valid TOML values.

The compilation grammar README does not modify those files, but production completion of the repository requires the toolchain declarations to be normalized separately.

The intended baseline remains:

Rust 1.97.1
Rust 2021

with safe Rust only.

---

85. No Unsafe Requirement

Nothing in the compilation grammar requires Rust "unsafe".

The compiler architecture MUST therefore prefer:

- ownership;
- borrowing;
- "Arc";
- "Rc" where appropriate;
- standard collections;
- checked arithmetic where required;
- explicit error handling;
- immutable data where practical;
- safe abstractions.

No grammar design may require unsafe Rust.

---

86. Memory Scalability

Compilation grammar MUST NOT establish fixed compiler memory capacities.

The compiler may encounter:

small source
large source
huge source
distributed source graph

subject to actual available resources.

Operational compiler safeguards are permitted, but they must be documented as implementation/resource policies rather than language restrictions.

---

87. Parallel Compilation

Compilation infrastructure may parallelize:

module compilation
semantic analysis
optimization
code generation
artifact generation

where semantics permit.

The grammar may express parallel compilation intent.

It must not assume:

8 threads
16 threads
64 threads

as universal capacities.

---

88. Incremental Compilation

The architecture should support incremental compilation without changing source semantics.

Compilation artifacts should be independently invalidatable based on semantic dependencies.

This integrates naturally with:

caching
provenance
artifacts
reproducibility
dependency analysis

---

89. Remote Compilation

Compilation intent must remain compatible with remote compilation.

The source program should not need to know whether compilation occurs:

locally
remotely
distributed
in the cloud
on an accelerator
on a future compilation service

provided the semantic contract is preserved.

---

90. Heterogeneous Compilation

A single program may contain:

classical computation
quantum computation
AI computation
data computation
HDL/hardware intent
distributed computation

The compilation model must therefore support multiple downstream representations without requiring multiple source languages.

Conceptually:

                    Zamani
                       |
              semantic compilation
                       |
       +---------------+---------------+
       |               |               |
   classical       quantum::ir      HDL/hardware
       |               |               |
       +---------------+---------------+
                       |
                 optimization
                       |
                target realization

---

91. Compile-Time vs Runtime

The grammar MUST distinguish:

compile-time intent

from:

runtime intent

A compile-time construct cannot silently execute at runtime merely because the compiler could not evaluate it.

Likewise, a runtime construct cannot silently become compile-time behavior unless the language specification explicitly permits such transformation.

---

92. Semantic Preservation

Any optimization, specialization, lowering, or target realization initiated by compilation syntax must preserve the program's semantic contract.

This includes:

- classical behavior;
- quantum behavior;
- observable behavior;
- resource semantics where contractual;
- effect semantics;
- ownership semantics;
- concurrency semantics;
- security semantics.

---

93. Approximation

If the compilation system permits approximation, approximation MUST be explicit.

For example, a source-level approximation policy may state acceptable semantic error.

The compiler must not silently introduce approximation because a target is insufficient.

---

94. Target Failure

If a target cannot satisfy a requirement:

requires capability("x")

the compiler must produce an explicit result such as:

target incompatible

with sufficient diagnostic information.

It MUST NOT silently:

remove the feature
weaken the requirement
substitute a different computation

unless the source explicitly authorizes such fallback behavior.

---

95. Fallback

A portable program may explicitly provide fallback paths.

Conceptually:

if capability("quantum.compute")
    ...
else
    ...

This is a source-level semantic choice.

The compiler may also perform valid automatic lowering where the specification explicitly permits it.

Automatic fallback must never silently alter required semantics.

---

96. Future-Proof Extensibility

New target types MUST NOT require redesigning the compilation grammar.

The following should be semantic categories, not exhaustive grammar enumerations:

target
capability
resource
artifact
profile
feature
operation
backend
accelerator
device

Future systems can therefore introduce:

new accelerator
new QPU technology
new processor architecture
new hardware paradigm
new distributed model
new computational substrate

without changing the fundamental compilation language.

---

97. Vendor Neutrality

Vendor identifiers may exist as external semantic data where interoperability requires them.

They must not become universal grammar keywords.

The source language should express:

capability("...")

rather than requiring a new keyword whenever a vendor creates a new device.

---

98. Backend Independence

A backend may support:

CPU
GPU
FPGA
ASIC
QPU
simulator
WASM
distributed runtime

without requiring a new compilation grammar.

Backend-specific syntax belongs in explicitly scoped interoperability/dialect mechanisms.

---

99. Dialect Integration

Compilation syntax may interact with:

grammar/dialects/

but dialects must declare:

- identity;
- version;
- syntax extensions;
- semantic extensions;
- AST mapping;
- IR mapping;
- compatibility;
- feature gates.

A dialect MUST NOT silently modify the meaning of core compilation syntax.

---

100. Macros Integration

Compilation macros integrate with:

grammar/macros/

Macros may generate compilation constructs.

However:

macro expansion
    ↓
normal parsing/semantic validation

must still occur.

Macros MUST NOT bypass compilation safety or semantic validation.

---

101. Metaprogramming Integration

Compilation metaprogramming integrates with:

grammar/metaprogramming/

Compile-time generated code must eventually enter the ordinary semantic pipeline.

The compiler must not treat generated code as inherently trusted.

---

102. Feature Lifecycle

Every compilation feature must have a status:

PROPOSED
EXPERIMENTAL
SPECIFIED
IMPLEMENTED
PARTIALLY_IMPLEMENTED
STABLE
DEPRECATED
REMOVED

A feature MUST NOT be marked "STABLE" merely because a ".g4" file exists.

---

103. Feature Completion Contract

A compilation feature is complete only when all applicable items below are satisfied:

[ ] specification exists
[ ] lexical contract exists
[ ] grammar exists
[ ] lexer compatibility verified
[ ] parser compatibility verified
[ ] AST contract exists
[ ] semantic contract exists
[ ] resource/capability behavior defined
[ ] portability behavior defined
[ ] canonical IR mapping defined
[ ] compiler consumer identified
[ ] backend/runtime consumer identified where applicable
[ ] diagnostics defined
[ ] positive tests exist
[ ] negative tests exist
[ ] boundary tests exist
[ ] scalability tests exist
[ ] compatibility tests exist
[ ] determinism tests exist where applicable
[ ] reproducibility tests exist where applicable
[ ] hard-coding audit passed
[ ] no duplicate authority exists

---

104. Independent-File Completion Principle

Each compilation grammar file must be independently completable.

Before a file is considered complete, it MUST already specify:

Purpose
Ownership
Non-ownership
Inputs
Outputs
Dependencies
Dependency direction
Syntax contract
AST contract
Semantic contract
IR contract
Compiler integration
Runtime integration where applicable
Cross-domain integration
Diagnostics
Testing
Compatibility
Scalability
Hard-coding policy
Completion criteria

This ensures that completing another file later does not require redesigning the completed file.

If another file changes, the change must instead be handled through an explicit versioned contract or compatibility update.

---

105. Dependency Direction

The dependency direction must remain acyclic.

Preferred architecture:

Zamani.g4
    ↓
Compile
    ↓
Compilation feature grammar
    ↓
frontend AST
    ↓
semantic analysis
    ↓
canonical semantic representation
    ↓
canonical IR
    ↓
compiler infrastructure

A feature grammar MUST NOT depend upward on its composition root.

For example:

compile.g4
    ↓
optimization.g4

is acceptable.

But:

optimization.g4
    ↓
compile.g4

must not be required merely to define optimization syntax.

---

106. Cross-Domain Dependency Rule

Compilation grammar may reference shared foundational concepts.

It MUST NOT duplicate them.

For example:

expression

belongs to:

grammar/expressions/

not separately to every compilation file.

Likewise:

type
identifier
qualified name
attribute
resource
capability

must have canonical owners.

---

107. No Grammar-Level Compiler Logic

Grammar files must remain declarative.

They MUST NOT contain logic that:

- discovers targets;
- allocates resources;
- invokes optimizers;
- invokes QEC;
- invokes ZQN;
- executes code;
- accesses hardware;
- modifies compiler state;
- writes artifacts;
- signs artifacts.

Those are implementation responsibilities.

---

108. Canonical Expression Integration

Compilation conditions and objectives should reuse the ordinary expression grammar.

Do not create:

compileExpression
targetExpression
optimizationExpression

as independent incompatible expression languages unless there is a demonstrable semantic need.

Prefer:

canonical expression
+
context-specific semantic interpretation

This preserves one expression language.

---

109. Canonical Type Integration

Compilation resource quantities, profile parameters, constraints, and configuration values must reuse canonical type semantics.

Do not invent separate numeric/type systems merely for compilation.

For example:

resource quantity

should be semantically typed rather than represented by arbitrary parser-level special cases.

---

110. Resource Quantities

Resource quantities must support:

literal quantity
symbolic quantity
computed quantity
parameterized quantity
unknown-at-compile-time quantity

For example:

requires qubits >= n

must be representable even when "n" cannot be statically evaluated.

---

111. Representation Limits

The language must distinguish:

semantic unboundedness

from:

finite machine representation

No physical machine is literally infinite.

POCO-REAF means the language does not impose artificial finite ceilings where semantics do not require them.

---

112. Resource Exhaustion

Compiler resource exhaustion is an implementation event.

It must not be represented as:

program is semantically invalid

unless the language specification explicitly defines a semantic resource constraint.

This distinction is essential for large-scale compilation.

---

113. Compiler Budgets

The compiler may have configurable budgets for:

time
memory
parallelism
cache
I/O
optimization effort
simulation effort

Those are compiler operational policies.

They must not become language-level hardware limits.

---

114. Security and Resource Budgets

Operational limits may also be necessary for denial-of-service protection.

Such limits must be:

- explicit;
- configurable where appropriate;
- documented;
- independent from language semantics;
- distinguishable from source-level requirements.

---

115. Deterministic Parsing

Compilation grammar must preserve deterministic parsing.

Ambiguities must be resolved through:

- grammar restructuring;
- precedence;
- explicit delimiters;
- canonical lexical rules.

Semantic target information must not be required merely to parse source code.

---

116. Parser Must Not Need Hardware Discovery

This is mandatory.

The parser must be capable of parsing:

requires capability("future.capability")

without knowing whether the capability currently exists.

The compiler may reject the requirement later during semantic/target validation.

---

117. Unknown Targets

A target name must be representable as data.

An unknown target should not necessarily be a parser error.

The semantic system decides whether the target identifier is:

known
supported
compatible
available

This permits future targets.

---

118. Unknown Capabilities

The same principle applies to capability identifiers.

The grammar must support extensible capability names.

Semantic analysis determines whether a capability is:

known
provided
required
unsupported
deprecated

---

119. Unknown Features

Feature selection should likewise remain extensible.

A future feature should not require changing the grammar merely because its name is new.

The compiler may report:

feature unavailable

without declaring the entire grammar obsolete.

---

120. Artifact Identity

Artifact identity must be semantic and stable.

Where cryptographic identity is required, the actual hash/signature implementation belongs downstream.

The grammar should express:

requires provenance
requires integrity
requires reproducibility

rather than embedding a cryptographic implementation.

---

121. Compiler Provenance

Compilation provenance should be capable of describing:

source identity
dependency identity
compiler identity
grammar/specification version
language version
configuration
profile
transformation lineage
artifact lineage

without requiring fixed-length provenance chains.

---

122. Compatibility

Every compilation feature must be versioned.

Compatibility must account for:

language version
grammar version
AST version
semantic model version
IR version
compiler version
backend capability version
dialect version

The compilation grammar must not silently introduce breaking syntax.

---

123. Deprecation

Deprecated compilation constructs must remain parseable only for the period defined by the compatibility policy.

The deprecation path is:

STABLE
   ↓
DEPRECATED
   ↓
compatibility period
   ↓
REMOVED

Migration guidance must be documented outside the grammar production itself.

---

124. Grammar-to-AST Traceability

Every compilation rule must have a predetermined AST mapping.

The traceability chain is:

grammar/compile/X.g4
       ↓
rule
       ↓
AST node
       ↓
semantic model
       ↓
canonical IR
       ↓
compiler consumer

A grammar production without an AST mapping is incomplete.

---

125. AST-to-IR Traceability

Every AST node introduced for compilation intent must specify whether it:

maps directly to canonical IR

or:

becomes compiler configuration/metadata

or:

is consumed only during semantic analysis

This prevents dead syntax.

---

126. Compilation Metadata vs Program Semantics

Not every compilation construct becomes an IR instruction.

For example:

optimization profile

may become compiler configuration.

Likewise:

reproducibility requirement

may become build metadata.

Therefore the contract must allow:

AST
 ├── semantic program representation
 ├── compilation configuration
 └── compilation metadata

while preserving one coherent semantic model.

---

127. No Dead-End Grammar

A compilation grammar feature is invalid architecturally if it has no identified consumer.

Every feature must answer:

Who consumes this?

Possible consumers include:

semantic analyzer
resource analyzer
capability analyzer
compile planner
optimization planner
specialization engine
lowering engine
artifact generator
reproducibility system
cache
cross-compiler
deployment planner
backend
runtime

If no consumer exists, the feature must remain proposed/experimental rather than being called production-ready.

---

128. Integration Matrix

Every compilation feature should be traceable across:

Layer| Required relationship
Specification| Defines meaning
Lexer| Defines/uses tokens
Grammar| Defines syntax
Parser| Constructs AST
AST| Represents intent
Semantics| Validates meaning
Resources| Resolves requirements
Capabilities| Resolves capabilities
Target| Evaluates compatibility
IR| Preserves required semantics
Optimization| Applies permitted transformations
Lowering| Produces lower representation
Backend| Produces target realization
Runtime| Executes/deploys where applicable
Tests| Proves conformance

---

129. Compilation Test Matrix

The compilation subsystem MUST contain tests for:

lexical
syntax
semantic
negative
boundary
scalability
compatibility
determinism
reproducibility
diagnostics

where applicable.

---

130. Positive Compilation Tests

Examples should cover:

compile declaration
compile profile
target requirement
capability requirement
optimization intent
specialization
conditional compilation
feature selection
artifact request
cross compilation
reproducibility
deterministic build
caching
provenance
deployment integration

---

131. Negative Tests

Must cover:

conflicting requirements
invalid profile
unknown invalid syntax
malformed target intent
invalid resource quantity
invalid capability expression
invalid optimization policy
invalid specialization
invalid artifact relationship
invalid reproducibility declaration

---

132. Boundary Tests

Boundary tests must include:

empty compilation plan
single clause
many clauses
deeply nested semantic expressions
symbolic resource values
large numeric values
unknown capabilities
unknown targets
future feature names
multiple artifacts
multiple dependencies
large compilation graphs

No artificial maximum may be inferred from a test.

---

133. Scalability Tests

The tests must explicitly verify that the grammar does not impose fixed ceilings.

Examples should progressively cover:

1
small
medium
large
very large
symbolically unbounded

for:

resources
artifacts
dependencies
targets
features
optimization objectives
compilation stages

The exact maximum is determined by the test environment rather than becoming a language constant.

---

134. Quantum Scalability Tests

Compilation tests must include:

1 qubit requirement
2 qubit requirement
symbolic qubit requirement
large symbolic requirement
capability-based quantum compilation
quantum optimization intent
quantum artifact generation
quantum target selection
quantum simulation fallback where explicitly allowed

The tests must never establish a universal maximum qubit count.

---

135. Classical Scalability Tests

Tests must cover:

single-core realization
multicore realization
vectorized realization
GPU realization
distributed realization

without making any one of those a language requirement.

---

136. HDL Scalability Tests

Tests must cover:

small module
parameterized module
large generated module
variable widths
parameterized memory
multiple interfaces
pipeline intent
timing intent
verification intent

without hard-coding a universal width or device capacity.

---

137. Distributed Scalability Tests

Tests must cover:

one node
multiple nodes
symbolic node requirement
capability-based distributed placement
replication
partitioning
communication requirements

without defining a maximum node count.

---

138. Diagnostic Tests

Diagnostics must verify that the compiler distinguishes:

parse failure
semantic failure
unsupported capability
resource insufficiency
target incompatibility
compiler resource exhaustion
backend failure
runtime failure

These are different failure classes.

---

139. Determinism Tests

Where deterministic compilation is required, repeated builds of the same semantic input must produce equivalent outputs according to the defined reproducibility contract.

Tests must detect:

- unstable ordering;
- random identifiers;
- uncontrolled metadata;
- unstable artifact ordering;
- nondeterministic optimization choices.

---

140. Reproducibility Tests

Tests must verify that changing irrelevant environment details does not alter reproducible artifacts.

Relevant environment changes must be explicitly defined.

---

141. Hard-Coding Audit

The compilation subsystem MUST be audited for forbidden universal limits.

Search patterns should include at minimum:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_QPUS
MAX_NODES
MAX_DEVICES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_NETWORK_SIZE

The audit must also detect disguised finite enumeration patterns.

---

142. False Positive Rule for Hard-Coding Audit

The audit must distinguish:

program constant

from:

universal compiler limit

For example:

let n = 1024;

is not automatically a violation.

A test fixture may also intentionally contain:

MAX_QUBITS

when testing that the forbidden pattern detector works.

Therefore the audit needs context rather than a naive text search alone.

---

143. Infinite / Unbounded Terminology

The documentation may use:

unbounded
arbitrarily large
scale with resources

but must not imply that finite machines provide literal infinity.

The intended guarantee is:

no artificial finite language ceiling

subject to actual semantics and resources.

---

144. Compilation Resource Model

Compilation must treat resources as data/constraints rather than fixed constants.

A compilation plan may express:

requires memory >= required_memory
requires capability("parallel.compile")
requires capability("quantum.compile")
requires capability("tensor.compute")

The compiler resolves these against actual resources.

---

145. Target Profiles

Target profiles may contain target-specific information outside the portable program.

This is permitted when clearly separated from the source-language semantic contract.

For example:

portable source
        +
external target profile
        ↓
target realization

The external profile may contain physical information.

That information must not become a universal language restriction.

---

146. Physical Mapping

Physical mapping is downstream.

Examples include:

logical qubit → physical qubit
logical buffer → memory region
logical task → processor
logical kernel → accelerator
logical module → FPGA resources

The compilation grammar may express constraints/preferences concerning mappings.

It does not perform mappings.

---

147. Compiler Architecture

The compilation subsystem must integrate with the compiler approximately as:

                 Zamani source
                       |
                       v
                    Lexer
                       |
                       v
                    Parser
                       |
                       v
                  Frontend AST
                       |
              +--------+--------+
              |                 |
              v                 v
       program semantics   compile intent
              |                 |
              +--------+--------+
                       |
                       v
              semantic analysis
                       |
       +---------------+----------------+
       |               |                |
       v               v                v
   resources      capabilities       portability
       |               |                |
       +---------------+----------------+
                       |
                       v
             canonical representation
                       |
          +------------+------------+
          |            |            |
          v            v            v
      classical    quantum::ir    HDL/hardware
          |            |            |
          +------------+------------+
                       |
                       v
                  optimization
                       |
                       v
                    lowering
                       |
                       v
              target realization
                       |
                       v
                    backend
                       |
                       v
                    runtime

---

148. Compilation Grammar Must Remain Above IR

The grammar must never depend on the internal layout of a backend.

For example, it must not need to know:

LLVM register representation
GPU instruction format
FPGA netlist encoding
QPU native instruction encoding

Those are downstream representations.

---

149. Interoperability Boundary

If a target requires:

LLVM
MLIR
QIR
OpenQASM
WASM
HDL
vendor IR

the compiler performs the translation after semantic analysis.

The compilation grammar expresses intent, not external IR syntax.

---

150. Compiler Once / Target Many

The architecture should support:

one source program
        ↓
one semantic compilation model
        ↓
multiple target realizations

without requiring:

program_cpu.zm
program_gpu.zm
program_qpu.zm
program_fpga.zm

for the same algorithm merely because hardware differs.

---

151. Target-Specific Source Is Still Possible

There may be legitimate target-specific programs.

The architecture should permit explicit target-specific constructs through:

dialects
interoperability
target profiles
foreign interfaces

but those must be clearly identified as target-specific.

They must not contaminate the portable core language.

---

152. Compilation Intent vs ABI

Calling conventions and ABI constraints may be expressed where necessary.

However, ABI implementation belongs downstream.

The grammar must not embed a fixed ABI as universal semantics.

---

153. Compilation Intent vs Operating System

The same principle applies to operating systems.

The grammar should express:

requires capability(...)

rather than making:

Linux
Windows
macOS
OS-X

part of the universal compilation semantics unless explicitly scoped as target configuration.

---

154. Cloud and Edge

Compilation intent must remain neutral regarding:

local
edge
cloud
cluster
remote
embedded

A target/deployment profile can determine the realization.

The source program remains semantic.

---

155. Nano and Future Domains

Compilation grammar must not require special compiler architecture for every future domain.

A new domain should be able to provide:

semantic domain
capabilities
resources
operations
IR mapping
backend

while using the same compilation infrastructure.

This is necessary for the broader:

atom → everywhere

objective.

---

156. Sankofa Integration

Sankofa-related concepts may participate in compilation where they have actual language/compiler semantics.

However, compilation grammar must not turn:

remember
recall
wisdom
learning
history

into compiler behavior merely because those concepts exist in "Zamani-Grammar.md".

Their semantics must first be established through the normal feature lifecycle.

---

157. MTS Integration

Multi-timeline concepts may affect compilation if explicitly specified.

The compilation subsystem must not assume a fixed number of timelines.

Any timeline count is semantic data or resource information.

---

158. Compilation of Quantum + Classical Programs

The compiler must support:

classical computation
      ↓
quantum computation
      ↓
measurement
      ↓
classical control
      ↓
quantum computation

without requiring multiple compilation languages.

Compilation intent applies to the complete program.

---

159. Compilation of Hardware + Software Programs

The compiler must support:

software algorithm
      +
hardware intent
      +
resource requirements
      +
timing requirements
      +
verification requirements

as one program where the language semantics permit it.

The downstream compiler determines the actual software/hardware partition.

---

160. Compilation of AI + Classical + Accelerator Programs

The same architecture permits:

AI model
      ↓
tensor computation
      ↓
classical preprocessing
      ↓
accelerator realization

without requiring the source program to be tied to one framework.

---

161. Compilation of Distributed Programs

The same architecture permits:

logical computation
      ↓
parallel/distributed intent
      ↓
resource/capability analysis
      ↓
placement
      ↓
communication
      ↓
execution

without a fixed number of nodes.

---

162. Compilation Profiles and Hardware Profiles

These must remain distinct.

Compilation profile

Defines compiler policy.

Hardware/target profile

Defines target facts/capabilities.

The source program may select or request a compilation profile.

The target environment supplies hardware information.

Neither should silently become the other.

---

163. Compiler Configuration vs Source Semantics

Compiler configuration may include:

optimization effort
diagnostic verbosity
cache location
artifact storage
parallel build policy

These are not necessarily source semantics.

The distinction must be documented so changing a build-system setting does not unexpectedly change program meaning.

---

164. Build-System Integration

The compilation grammar should integrate with the repository's build/package infrastructure without taking ownership of it.

Potential downstream consumers include:

Zamani.toml
Danga/build system
zmc
compiler driver
artifact manager

The grammar describes source intent.

The build system determines invocation/environment.

---

165. Generated Documentation

"grammar/grammar.md" remains the implementation-conformance document.

Compilation grammar documentation should therefore feed a traceability process:

specification
   ↓
grammar
   ↓
implementation
   ↓
grammar.md

The compilation README must not become another generated grammar reference.

---

166. Existing "grammar/Zamani-Grammar.md"

This file may contain broad compilation concepts.

Those concepts are not automatically production syntax.

A feature must be promoted through:

Zamani-Grammar.md
      ↓
proposal
      ↓
semantic specification
      ↓
AST contract
      ↓
grammar
      ↓
implementation
      ↓
IR mapping
      ↓
tests
      ↓
stable

---

167. Existing "grammar/specification/"

Compilation semantics must eventually be represented in the authoritative specification.

Relevant areas include:

language
syntax
semantics
portability
POCO-REAF

The compile README defines how those specifications connect to compilation grammar.

It does not supersede them.

---

168. Existing "grammar/spec/"

Compilation feature contracts should be represented there where detailed machine-checkable/conformance specifications are appropriate.

Suggested future contracts include:

spec/compilation.md
spec/compile-profiles.md
spec/compile-targets.md
spec/optimization.md
spec/specialization.md
spec/artifacts.md
spec/reproducibility.md
spec/deterministic-builds.md

Only create missing files when they are actually needed; do not create parallel specifications merely for symmetry.

---

169. Feature Manifests

For large features, the repository may use a machine-readable feature contract.

A feature contract should identify:

id
name
status
version
grammar
tokens
AST
semantics
resources
capabilities
IR mapping
compiler consumer
runtime consumer
tests
compatibility

This is especially useful for preventing cross-file rework.

---

170. Independent Completion Record

For each compilation grammar file, completion should be recorded using:

Purpose:
Owns:
Does not own:
Inputs:
Outputs:
Dependencies:
Dependency direction:
Lexical contract:
Grammar contract:
AST contract:
Semantic contract:
Resource contract:
Capability contract:
IR contract:
Compiler integration:
Runtime integration:
Diagnostics:
Security:
Scalability:
Positive tests:
Negative tests:
Boundary tests:
Compatibility tests:
Determinism tests:
Hard-coding audit:
Completion criteria:

This record may live in the file's header or associated specification.

---

171. No Hidden Cross-File Assumptions

A file is not complete if it says:

"the other file will define this later"

Instead it must state the contract it consumes.

For example:

target.g4

must explicitly say:

Consumes:
canonical identifier
canonical qualified-name
canonical expression
canonical resource/capability semantics

rather than depending on undocumented future behavior.

---

172. Versioned Contracts

When another subsystem changes, compatibility should be handled by versioned contracts.

For example:

compile grammar vX
AST contract vY
IR contract vZ

rather than repeatedly rewriting completed grammar files.

---

173. Repository-Wide Validation

Compilation grammar validation must eventually verify:

grammar
    ↕
lexer
    ↕
parser
    ↕
AST
    ↕
semantics
    ↕
resources/capabilities
    ↕
IR
    ↕
compiler
    ↕
backend

A compilation grammar file should not be marked production-ready if it cannot be traced through this chain.

---

174. ANTLR Validation

The grammar pipeline must check:

syntax validity
duplicate rules
unreachable rules
ambiguity
left recursion
token conflicts
import correctness
rule references
generation success

where applicable.

---

175. Rust Parser Validation

The Rust parser must be tested against the same semantic examples.

Where ANTLR and Rust parser implementations both exist, they must agree on accepted/rejected language constructs according to the canonical specification.

They are two implementations/representations of one language, not two languages.

---

176. AST Validation

Tests must verify that equivalent source forms produce equivalent semantic AST structures where the language specifies equivalence.

Source spans must remain correct.

---

177. Semantic Validation

Tests must verify:

requirements
capabilities
constraints
preferences
hints
profiles
targets
specialization
optimization
artifacts

are classified correctly.

---

178. IR Validation

Compilation metadata/configuration must reach the appropriate compiler boundary without introducing an unauthorized second IR.

For semantic program constructs, canonical IR must preserve the required meaning.

---

179. Backend Validation

Backend tests must verify that target realization satisfies source requirements.

Examples:

capability required
resource required
target constraint
optimization requirement
artifact requirement

---

180. Runtime Validation

Runtime validation applies where compilation intent reaches runtime/deployment behavior.

The grammar itself remains independent of runtime implementation.

---

181. Production Readiness Gate

"grammar/compile/" may be called production-ready only when:

[ ] one compilation authority exists
[ ] compile.g4 / compilation.g4 overlap is resolved
[ ] all grammar modules have ownership
[ ] all grammar modules have AST contracts
[ ] all grammar modules have semantic contracts
[ ] all grammar modules have IR/compiler integration contracts
[ ] canonical lexer integration is verified
[ ] Rust parser integration is verified
[ ] domain-neutral AST integration is verified
[ ] resource/capability separation is verified
[ ] target realization remains downstream
[ ] quantum::ir remains canonical
[ ] no second quantum IR exists
[ ] no universal hardware ceilings exist
[ ] no vendor lock-in exists in core syntax
[ ] diagnostics are defined
[ ] compatibility is defined
[ ] positive tests exist
[ ] negative tests exist
[ ] boundary tests exist
[ ] scalability tests exist
[ ] determinism tests exist where applicable
[ ] reproducibility tests exist where applicable
[ ] hard-coding audit passes
[ ] safe Rust requirement is satisfied
[ ] ANTLR generation succeeds
[ ] Rust parser conformance succeeds
[ ] semantic conformance succeeds
[ ] IR conformance succeeds

---

182. Definition of "Scalable"

For this subsystem, scalable means:

adding more resources
does not require grammar redesign;

adding more targets
does not require grammar redesign;

adding more devices
does not require grammar redesign;

adding more capabilities
does not require grammar redesign;

adding more quantum operations
does not require grammar redesign;

adding more CPUs/GPUs/FPGAs/QPUs
does not require grammar redesign;

adding more nodes
does not require grammar redesign;

adding larger tensors
does not require grammar redesign;

adding future hardware
does not require grammar redesign.

The semantic model expands through data and capability descriptions rather than finite grammar enumeration.

---

183. Definition of "Everywhere"

"Everywhere" means:

where a compatible realization exists

not:

every physically possible machine without regard to semantics

A program requiring a capability unavailable on a target must be diagnosed or explicitly lowered/fallbacked according to the program's contract.

---

184. Definition of "Forever"

"Forever" is a language-architecture objective, not a promise that every future target will preserve every implementation detail.

The durable contract is:

stable source semantics
+
versioned language specification
+
compatibility rules
+
portable semantic representation
+
explicit lowering boundaries

This allows future implementations to realize old programs without requiring source rewriting solely because the machine changed.

---

185. Canonical Final Architecture

The intended compilation path is:

                         Zamani Source
                              |
                              v
                     grammar/Zamani.g4
                              |
                              v
                           Lexer
                              |
                              v
                           Parser
                              |
                              v
                       Frontend AST
                              |
                 +------------+------------+
                 |                         |
                 v                         v
          Program semantics         Compilation intent
                 |                         |
                 +------------+------------+
                              |
                              v
                    Semantic analysis
                              |
       +----------------------+----------------------+
       |                      |                      |
       v                      v                      v
    Types                 Resources             Capabilities
       |                      |                      |
       +----------------------+----------------------+
                              |
                              v
                       Portability analysis
                              |
                              v
                  Canonical semantic model
                              |
            +-----------------+------------------+
            |                 |                  |
            v                 v                  v
        Classical         quantum::ir       HDL/Hardware
            |                 |                  |
            +-----------------+------------------+
                              |
                              v
                         Optimization
                              |
                              v
                           Lowering
                              |
                     +--------+--------+
                     |                 |
                     v                 v
                  Routing          Scheduling
                     |                 |
                     +--------+--------+
                              |
                              v
                     Resilience / QEC
                              |
                              v
                             ZQN
                              |
                              v
                             HAL
                              |
                              v
                     Target realization
                              |
              +---------------+---------------+
              |               |               |
              v               v               v
             CPU             GPU             FPGA
              |               |               |
              +---------------+---------------+
                              |
                       +------+------+
                       |             |
                       v             v
                      QPU        Future targets
                              |
                              v
                           Runtime

---

186. What the Compilation Grammar Must Never Become

The compilation grammar must never become:

a hardware database
a device registry
a compiler implementation
an optimizer implementation
a scheduler
a router
a QEC engine
a ZQN engine
a HAL
a runtime
a vendor API
a second IR
a second language
a fixed machine model
a finite resource universe

---

187. What It Must Become

It must become:

the stable source-language expression of compilation intent

with:

one authority
one semantic model
one AST contract
one canonical IR boundary
one target-resolution boundary
one resource/capability model
one compatibility model
one conformance model

and extensibility through:

data
capabilities
resources
profiles
dialects
interoperability
versioned contracts

rather than through endless hard-coded grammar alternatives.

---

188. Final Ownership Table

File| Primary responsibility| Must not own
"README.md"| subsystem architecture| individual grammar authority
"compile.g4"| compilation composition| feature internals
"compilation.g4"| subordinate compilation-plan semantics/composition if retained| second public language
"compile-time.g4"| compile-time syntax| general expressions
"conditional-compilation.g4"| conditional compilation| hardware probing
"profiles.g4"| compilation profiles| physical device selection
"target.g4"| target intent| physical realization
"target-selection.g4"| selection criteria| selection algorithm
"feature-selection.g4"| feature intent| fixed feature universe
"optimization.g4"| optimization intent| optimizer implementation
"specialization.g4"| specialization intent| backend implementation
"code-generation.g4"| code-generation intent| machine-code generation
"lowering.g4"| lowering intent| lowering implementation
"cross-compilation.g4"| cross-target intent| target backend
"artifacts.g4"| artifact intent| artifact storage
"caching.g4"| cache intent| cache implementation
"deterministic-builds.g4"| deterministic-build requirements| guarantee implementation
"reproducibility.g4"| reproducibility intent| provenance implementation
"provenance.g4"| compilation provenance| data/security provenance
"deployment.g4"| compilation/deployment integration| deployment execution

---

189. Final Integration Contract

Every compilation construct MUST be traceable as:

source syntax
    ↓
canonical lexer
    ↓
canonical parser
    ↓
frontend AST
    ↓
semantic compilation model
    ↓
resource/capability/portability analysis
    ↓
canonical semantic representation
    ↓
canonical IR
    ↓
optimization/lowering
    ↓
target realization
    ↓
backend/runtime

No step may be silently skipped for a feature claiming production status.

---

190. Final POCO-REAF Contract

The compilation subsystem satisfies the POCO-REAF architecture when:

Program Once
     ↓
stable Zamani semantics
     ↓
portable compilation intent
     ↓
canonical semantic representation
     ↓
canonical IR
     ↓
target-independent optimization/lowering
     ↓
capability/resource-aware realization
     ↓
CPU / GPU / FPGA / ASIC / QPU /
embedded / HPC / distributed / cloud /
future compatible target

without requiring source-level rewriting merely because:

machine size changes
resource quantity changes
processor count changes
GPU count changes
FPGA capacity changes
QPU capacity changes
memory capacity changes
network size changes
cluster size changes
accelerator availability changes
target vendor changes
hardware topology changes
future hardware appears

The program remains a description of its computation and its contractual requirements.

The compiler and runtime remain responsible for finding a valid realization.

---

191. Final Production Rule

The single governing rule for "grammar/compile/" is:

«Compilation syntax expresses portable compilation intent; semantic analysis determines meaning; resource and capability analysis determines feasibility; compiler infrastructure determines transformation; target infrastructure determines realization; runtime infrastructure determines execution.»

Therefore:

DO NOT hard-code today's machine into tomorrow's language.

DO NOT make every new device a grammar keyword.

DO NOT make every new quantum operation a grammar alternative.

DO NOT create a second quantum IR.

DO NOT create a second compilation language.

DO NOT let target realization leak into portable source semantics.

DO NOT confuse compiler resource limits with language limits.

DO NOT call syntax production-ready without AST, semantic, IR,
compiler, and test integration.

Instead:

DEFINE THE SEMANTICS
        ↓
DEFINE THE CONTRACT
        ↓
DEFINE THE SYNTAX
        ↓
DEFINE THE AST
        ↓
DEFINE THE SEMANTIC MODEL
        ↓
DEFINE THE IR MAPPING
        ↓
DEFINE THE COMPILER CONSUMER
        ↓
DEFINE THE TARGET BOUNDARY
        ↓
DEFINE THE TESTS
        ↓
AUDIT FOR HARD-CODING
        ↓
MARK STABLE

That is the production architecture required for:

Zamani
    →
Quantum
    →
Classical
    →
HDL
    →
Hybrid
    →
AI
    →
Distributed
    →
Accelerated
    →
Embedded
    →
HPC
    →
Cloud
    →
Future computing

while preserving:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

and allowing the implementation to scale from the smallest supported computation to arbitrarily large computations according to actual program semantics, capabilities, and available resources.