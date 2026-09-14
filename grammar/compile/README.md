Zamani Compilation Grammar

Production Architecture and Integration Contract

Path: "grammar/compile/README.md"

Scope: Source-level compilation intent and compilation-control grammar

Language: Zamani

Grammar technology: ANTLR

Compiler implementation: Rust 1.97 / Rust 1.97.1, Rust 2021

Safety: "unsafe" Rust is prohibited

Portability model: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

Status: Production architecture contract

---

1. Purpose

The "grammar/compile/" subsystem defines the portion of Zamani syntax that expresses compilation intent.

It is responsible for describing what the programmer is asking the compiler to do, without embedding assumptions about the physical machine that eventually executes the program.

The compilation grammar is therefore a language-level boundary between:

Zamani source
    |
    v
lexer
    |
    v
parser
    |
    v
frontend AST
    |
    v
semantic analysis
    |
    +--> compile-time evaluation
    +--> specialization
    +--> feature selection
    +--> requirement validation
    +--> target resolution
    +--> optimization intent
    +--> lowering intent
    |
    v
canonical semantic representations
    |
    +--> classical IR
    +--> quantum::ir
    +--> hardware/HDL semantic representations
    |
    v
optimization
    |
    v
routing
    |
    v
scheduling
    |
    v
hardware abstraction
    |
    v
runtime

The grammar does not perform these downstream operations.

---

2. Fundamental Principle

Zamani source code describes:

- computation;
- semantics;
- intent;
- requirements;
- constraints;
- capabilities;
- preferences;
- compilation policy;
- optimization intent;
- portability requirements;
- interoperability requirements.

It must not accidentally describe:

- a fixed machine size;
- a fixed number of processors;
- a fixed number of qubits;
- a fixed number of GPUs;
- a fixed number of FPGA resources;
- a fixed memory capacity;
- a fixed physical topology;
- a fixed device identifier;
- a fixed hardware address;
- a particular vendor implementation;
- a temporary runtime configuration.

The central rule is:

«Compilation syntax describes what the program means and what compilation is permitted or required to accomplish; target and runtime layers determine how that meaning is realized.»

---

3. POCO-REAF

The compilation grammar is explicitly designed around:

Program Once
        |
        v
Compile Once
        |
        v
Semantic / portable compilation representation
        |
        +-------------------+
        |                   |
        v                   v
classical             quantum / hybrid
        |                   |
        +---------+---------+
                  |
                  v
       target capability resolution
                  |
                  v
        routing / scheduling
                  |
                  v
          hardware / runtime

The same source semantics must be capable of being compiled for environments ranging from very small systems to arbitrarily large systems, subject to the actual capabilities and resources available at compilation/execution time.

Compilation grammar therefore must not encode artificial resource ceilings.

---

4. Ownership

4.1 "grammar/compile/" owns

The subsystem owns source syntax for:

- compilation-time control;
- conditional compilation;
- compile-time selection;
- compile-time requirements;
- compile-time assertions;
- specialization requests;
- compilation options;
- compilation configuration selection;
- feature selection;
- compilation-time generated regions;
- compilation-time iteration where supported;
- target-independent compilation intent;
- optimization intent;
- code-generation intent;
- lowering intent;
- compilation policy declarations;
- compilation-related annotations;
- compilation-stage composition.

The exact production for each concern belongs to its dedicated grammar file.

---

5. Non-Ownership

"grammar/compile/" does not own:

- ordinary expressions;
- expression precedence;
- ordinary statements;
- ordinary functions;
- function types;
- type definitions;
- memory semantics;
- concurrency semantics;
- quantum IR;
- classical IR;
- HDL IR;
- quantum gates;
- qubit identity;
- physical qubit allocation;
- QEC algorithms;
- ZQN fault models;
- routing algorithms;
- scheduling algorithms;
- hardware discovery;
- calibration;
- optimization algorithms;
- compiler pass implementations;
- runtime dispatch;
- runtime execution;
- device discovery;
- provider APIs;
- backend-specific APIs;
- physical topology;
- resource discovery.

Those responsibilities remain with the corresponding repository subsystems.

---

6. File Ownership

The compile subsystem is divided according to the following ownership model.

"compile.g4"

Owns the composition boundary for compilation-related grammar.

It must provide the authoritative entry points that combine the dedicated compile grammar modules without duplicating their productions.

It should answer:

«"Which compilation constructs can occur here?"»

It must not reimplement their internal syntax.

---

"compile-time.g4"

Owns source-level compilation-time control.

Examples include:

- compile-time conditional selection;
- compile-time requirements;
- compile-time assertions;
- compile-time specialization;
- compile-time feature selection;
- compile-time configuration selection;
- compile-time generated regions;
- compile-time control flow.

It delegates expression semantics to:

grammar/expressions/

and compile-time function semantics to:

grammar/functions/compile-time-functions.g4

---

"target.g4"

Owns source-level target intent.

It describes semantic target requirements and target-selection intent.

It must not become a device-description language.

It must not hard-code:

device = ibm_x
qubits = 127
cores = 64
gpu = 0
address = ...

unless such information is explicitly represented as an externally supplied target configuration or semantic constraint rather than an accidental grammar limitation.

Target-specific details belong downstream.

---

"optimization.g4"

Owns optimization intent.

It may express:

- objectives;
- preferences;
- requirements;
- constraints;
- optimization profiles;
- optimization pipelines;
- pass-selection intent;
- pass exclusion;
- optimization budgets;
- verification intent;
- reproducibility intent;
- approximation intent;
- stochastic policy.

It does not implement optimization.

The repository already establishes this separation: optimization grammar is intended to produce source-level optimization intent which is later consumed by the optimization planner and canonical IR pipeline.

---

"conditional-compilation.g4"

Owns dedicated conditional-compilation syntax where that syntax is sufficiently distinct to warrant a separate production boundary.

Conditions are semantic expressions.

The grammar must not attempt to determine whether a condition is true.

---

"feature-selection.g4"

Owns compilation-time selection of language/compiler/semantic capabilities.

It must distinguish:

feature requirement

from:

physical hardware selection

For example:

requires quantum

must not inherently mean:

use device X
use exactly N qubits

---

"code-generation.g4"

Owns source-level code-generation intent.

It must describe requests such as:

- generation;
- emission;
- representation selection;
- output intent;
- generated artifacts.

It does not implement code generators.

---

"lowering.g4"

Owns source-level lowering intent where such explicit language syntax is supported.

Lowering itself belongs to compiler infrastructure.

The grammar must never become a second IR.

---

7. Canonical Dependency Direction

The compile grammar follows:

lexer
  |
  v
core
  |
  +--> types
  |
  +--> expressions
  |
  +--> declarations
  |
  +--> functions
  |
  v
compile
  |
  +--> resources
  +--> target
  +--> optimization
  +--> lowering
  +--> code generation
  |
  v
frontend AST
  |
  v
semantic analysis
  |
  v
canonical IR

It must never form:

compile -> IR -> grammar

or:

compile -> runtime -> grammar

or:

compile -> hardware implementation -> grammar

---

8. Relationship With "Zamani.g4"

"grammar/Zamani.g4" is the assembled language grammar.

The compile subsystem must not silently become a competing root grammar.

The canonical composition must ultimately resemble:

Zamani.g4
    |
    +--> lexer vocabulary
    +--> core grammar
    +--> types
    +--> expressions
    +--> statements
    +--> declarations
    +--> functions
    +--> modules
    +--> effects
    +--> memory
    +--> concurrency
    +--> classical
    +--> quantum
    +--> hybrid
    +--> HDL
    +--> hardware
    +--> distributed
    +--> AI/data
    +--> networking
    +--> security
    +--> resources
    +--> compile
    +--> execution
    +--> interoperability
    +--> dialects

The existing root grammar currently contains many language domains directly in one grammar, including functions, control flow, mathematical constructs, quantum constructs, HDL constructs and other declarations. This must be treated as existing language capability to preserve and migrate deliberately rather than silently discard.

---

9. Relationship With "grammar/expressions"

Compilation grammar consumes expressions.

It must not redefine:

- arithmetic;
- logical expressions;
- comparisons;
- calls;
- indexing;
- member access;
- literals;
- compile-time expressions.

In particular:

grammar/expressions/compile-time.g4

owns expression-level compile-time semantics.

"grammar/compile/compile-time.g4" owns statement/control-level compilation-time syntax.

This separation prevents duplicate grammar rules and ambiguity.

---

10. Relationship With "grammar/resources"

Compilation requirements must use the universal resource model.

The compile grammar must distinguish:

requirement
constraint
capability
preference
hint
resource
target
placement
performance
latency
energy
reliability
scalability
portability

These are not interchangeable.

For example:

requires quantum capability

is not equivalent to:

requires device X

and:

prefers low latency

is not equivalent to:

requires latency <= X

Semantic analysis must preserve these distinctions.

---

11. Relationship With Hardware

The compilation grammar may refer to hardware capabilities and requirements, but it must not discover or describe physical hardware itself.

The architecture is:

source compilation intent
          |
          v
semantic requirements
          |
          v
resource/capability model
          |
          v
hardware HAL
          |
          v
available capabilities
          |
          v
target resolution

The hardware subsystem owns:

- devices;
- hardware capabilities;
- topology;
- physical resources;
- calibration;
- target-specific implementation.

The grammar owns only the syntax through which a programmer may express relevant semantic intent.

---

12. Quantum Integration

Compilation syntax involving quantum programs must preserve the canonical quantum path:

Zamani source
     |
     v
quantum grammar
     |
     v
frontend AST
     |
     v
quantum semantic analysis
     |
     v
quantum::ir
     |
     v
optimization
     |
     v
routing
     |
     v
scheduling
     |
     v
hardware HAL
     |
     v
runtime

The compile grammar must never define:

- a second quantum IR;
- a second gate representation;
- a second qubit identity;
- physical qubit allocation;
- routing;
- scheduling;
- QEC algorithms.

The repository explicitly treats "quantum::ir" as the canonical quantum semantic boundary, and the scheduling/optimization layers consume that representation.

---

13. Quantum Compilation Portability

The grammar must permit source intent such as:

optimize depth

or:

require quantum capability

without requiring:

use 127 qubits
use device X
use topology Y
use exactly N gates

Resource realization belongs downstream.

This preserves:

logical quantum program
        |
        v
canonical quantum IR
        |
        +--> simulator
        +--> small QPU
        +--> large QPU
        +--> future QPU

without rewriting source semantics.

---

14. Scheduling Integration

Compilation grammar must not schedule operations.

Scheduling consumes canonical semantic representations after compilation/optimization/routing.

The repository's scheduling architecture already separates frontend input from canonical "quantum::ir", followed by scheduling and optimization stages.

Therefore:

compile grammar
      |
      v
compiler semantic representation
      |
      v
quantum::ir / classical IR / hardware-independent IR
      |
      v
scheduler

The grammar must not contain:

schedule gate 1 at cycle 4

unless such syntax is explicitly defined as a semantic timing requirement rather than a physical scheduler command.

---

15. Optimization Integration

The compile grammar describes optimization intent.

It does not contain:

- optimization algorithms;
- rewrite engines;
- cost-model implementations;
- pass implementations;
- optimizer state;
- optimizer caches;
- hardware calibration;
- routing.

The downstream path is:

optimization syntax
       |
       v
optimization intent AST
       |
       v
semantic validation
       |
       v
optimization planner
       |
       +--> analysis
       +--> pass selection
       +--> rewrite
       +--> verification
       +--> provenance
       |
       v
optimized canonical IR

This matches the existing optimization subsystem's architecture.

---

16. Compilation-Time Safety

Compilation-time constructs must not imply arbitrary host execution.

The grammar must not provide implicit access to:

- filesystem;
- network;
- environment secrets;
- processes;
- devices;
- arbitrary host memory;
- arbitrary host execution.

The compiler/security layer determines what compile-time operations are permitted.

The existing compile-time grammar already establishes that compile-time syntax is a source-level control boundary and that filesystem/network access is not granted by the grammar itself.

---

17. Rust Contract

The generated compiler/parser integration must support:

Rust 1.97
Rust 1.97.1
Rust 2021

The implementation must use safe Rust.

The project must contain no:

unsafe

for grammar integration.

The grammar itself must contain no:

- embedded Rust actions;
- semantic predicates requiring unsafe behavior;
- host callbacks;
- filesystem operations;
- network operations;
- runtime execution;
- mutable global compiler state.

The parser is a syntax component, not an execution engine.

---

18. Scalability Contract

The grammar must not encode artificial limits.

The following must never appear as grammar-level ceilings:

MAX_TARGETS
MAX_FEATURES
MAX_SPECIALIZATIONS
MAX_CONFIGURATIONS
MAX_DEVICES
MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_OPERATIONS
MAX_PASSES
MAX_PIPELINE_STAGES

Repeated syntax must use grammar repetition:

item*
item+
(item)*

rather than finite enumerations.

Actual resource limits belong to:

- compiler policy;
- resource manager;
- target capabilities;
- scheduler;
- runtime;
- hardware;
- deployment configuration.

---

19. "Infinity" Interpretation

"Scale to infinity" means:

«The language grammar imposes no artificial finite architectural ceiling.»

It does not mean that physical hardware can execute an unbounded program.

The actual execution boundary remains:

available resources
+
compiler capacity
+
runtime capacity
+
target capabilities
+
semantic constraints
+
execution policy

Therefore a valid grammar construct must remain syntactically representable regardless of whether the eventual execution environment is:

tiny embedded device
CPU
multicore
GPU
FPGA
ASIC
QPU
simulator
accelerator
cluster
HPC system
cloud
distributed system
future architecture

---

20. Compilation Requirements Versus Machine Requirements

Compilation syntax must preserve the distinction between:

Semantic requirement

"What must be true for this program to be valid?"

Capability

"What can the target provide?"

Constraint

"What must not be violated?"

Preference

"What implementation is preferred?"

Hint

"What implementation is suggested but not guaranteed?"

Target

"What class of execution environment is intended?"

Device

"Which concrete physical resource is being selected?"

These must not collapse into one concept.

---

21. Conditional Compilation

Conditional compilation must be semantic.

For example:

compile if capability("quantum") {
    ...
}

may express a semantic branch.

It must not force the parser to know whether the current machine has that capability.

The compiler determines that.

Likewise, the grammar must not encode:

if CPU_COUNT == 64

as a special grammar construct.

Such information belongs in resource/capability evaluation.

---

22. Target Selection

Target syntax must remain abstract.

Valid concepts include:

target capability
target architecture class
target execution model
target ABI
target dialect
target resource requirement
target compatibility requirement

Target syntax must not turn the source language into a hardware inventory database.

Concrete target information is supplied through:

- target descriptions;
- compiler configuration;
- hardware HAL;
- runtime discovery;
- deployment configuration;
- capability negotiation.

---

23. Optimization Selection

Optimization syntax may identify:

objective
profile
pass
pipeline
policy
budget
verification requirement
reproducibility requirement

Identifiers are symbolic.

The grammar must not assume that a symbolic pass name corresponds to a Rust type.

For example:

optimization pass "x"

means:

«request a pass identified by the language/compiler contract as "x".»

The registry/planner determines whether that pass exists and what implementation performs it.

---

24. Code Generation

Code-generation syntax must describe intent.

It must not directly generate:

- machine instructions;
- assembly;
- LLVM internals;
- FPGA bitstreams;
- quantum hardware commands;
- provider-specific runtime calls.

Those are compiler/backend responsibilities.

The grammar should remain stable even when code-generation technologies change.

---

25. Lowering

Lowering is a compiler transformation.

The grammar may express explicit source-level lowering intent if the language requires it, but the grammar must never encode the actual lowered representation.

The boundary is:

source syntax
    |
    v
AST
    |
    v
semantic model
    |
    v
canonical IR
    |
    v
lowering

not:

source syntax
    |
    v
grammar-generated IR

---

26. AST Contract

Every compile grammar construct must lower into an AST representation containing semantic information sufficient for later validation.

The AST should preserve, where applicable:

- source span;
- construct kind;
- identifier;
- expression;
- ordered child constructs;
- modifiers;
- requirements;
- constraints;
- preferences;
- hints;
- explicit target intent;
- explicit optimization intent;
- source-level provenance;
- version information.

The grammar must not depend on concrete Rust compiler implementation types.

---

27. Semantic Contract

Semantic analysis must determine:

- whether the construct is legal;
- whether expressions have valid types;
- whether referenced capabilities exist;
- whether requirements are satisfiable;
- whether constraints are contradictory;
- whether a target request is compatible;
- whether an optimization request is meaningful;
- whether compile-time evaluation is permitted;
- whether specialization is valid;
- whether lowering is legal.

Syntax alone must not attempt these decisions.

---

28. Error Handling

Grammar errors must be deterministic and source-located.

Diagnostics should provide:

- source location;
- offending construct;
- expected syntax;
- relevant context;
- stable diagnostic identity where the compiler defines one;
- actionable explanation.

Semantic failures must remain distinct from syntax failures.

For example:

syntax error

must not be confused with:

valid syntax but unsatisfied target capability

or:

valid syntax but impossible resource requirement

---

29. Compatibility

The compile grammar must be version-aware.

Language evolution must distinguish:

introduced
stable
deprecated
reserved
removed

A future compiler must be able to encounter an older source program without silently changing its meaning.

POCO-REAF therefore requires preservation of semantic meaning rather than preservation of every implementation detail.

---

30. Dialect Integration

New compilation technologies must be extensible through dialect mechanisms rather than forcing permanent changes to the core language whenever possible.

Dialect extensions may introduce:

- new compilation policies;
- new target classes;
- new accelerator concepts;
- new optimization objectives;
- new lowering requests;
- new code-generation targets.

However, extensions must remain namespaced and versioned.

Vendor-specific syntax must not contaminate universal Zamani semantics.

---

31. Quantum/Classical/HDL Integration

The compile grammar must be domain-neutral.

A single source program may contain:

classical
+
quantum
+
HDL
+
hardware requirements
+
distributed execution
+
AI

The compilation grammar must therefore operate above individual computational domains.

For example:

source
 |
 +--> classical semantics
 |
 +--> quantum semantics
 |
 +--> HDL semantics
 |
 +--> resource requirements
 |
 +--> optimization intent
 |
 +--> execution intent
 |
 v
unified semantic compilation model

Domain-specific semantic layers retain ownership of their respective representations.

---

32. No Duplicate IR

The compile grammar must never define:

CompileIR
QuantumCompileIR
HardwareCompileIR
OptimizationIR
ScheduleIR

as replacements for repository-owned representations.

The grammar produces syntax.

Semantic analysis produces semantic models.

Existing IR systems remain authoritative.

For quantum:

quantum::ir

remains canonical.

---

33. No Circular Dependencies

Forbidden dependency patterns include:

grammar -> IR -> grammar
grammar -> runtime -> grammar
grammar -> scheduler -> grammar
grammar -> hardware implementation -> grammar
quantum grammar -> hardware implementation -> quantum grammar

Permitted direction:

grammar
   |
   v
AST
   |
   v
semantic analysis
   |
   v
IR
   |
   v
optimization
   |
   v
routing
   |
   v
scheduling
   |
   v
hardware
   |
   v
runtime

---

34. Integration With QEC

Compilation syntax may express an error-correction requirement or policy where the language requires such a construct.

It must not implement QEC.

QEC remains responsible for:

- codes;
- syndrome processing;
- correction algorithms;
- logical-error handling;
- QEC execution semantics.

Compilation grammar may express intent such as:

require fault tolerant execution

but the QEC subsystem determines how that requirement is fulfilled.

---

35. Integration With ZQN

The compilation grammar must not define noise models.

ZQN remains responsible for:

- fault models;
- noise classification;
- correlated faults;
- leakage;
- loss;
- erasure;
- fault semantics.

Compilation syntax may express relevant high-level requirements or optimization intent, but ZQN remains authoritative for fault/noise semantics.

---

36. Integration With Resilience

Compilation grammar must not become the resilience orchestrator.

Resilience decides when to:

- retry;
- recover;
- reroute;
- reschedule;
- recompile;
- reoptimize;
- switch backend;
- quarantine;
- mitigate;
- abort.

The grammar may express programmer-level resilience requirements where necessary, but operational recovery remains downstream.

---

37. Integration With Scheduling

Compilation syntax may express semantic scheduling requirements such as:

latency preference
timing requirement
ordering constraint
resource requirement

It must not implement:

- ASAP scheduling;
- ALAP scheduling;
- RCPSP;
- list scheduling;
- critical-path scheduling;
- resource allocation;
- physical timing resolution.

Those belong to scheduling.

---

38. Integration With Hardware HAL

The grammar may request capabilities.

The HAL determines actual capabilities.

The grammar must never directly depend on:

PhysicalQubitId
DeviceId
BackendId
CalibrationId

or equivalent concrete runtime types.

The AST should carry language-level symbolic references where necessary.

Semantic/compiler adapters resolve them later.

---

39. Integration With Runtime

The runtime must not need to parse source grammar.

Runtime consumes compiled representations and execution plans.

Therefore:

grammar
    |
    v
frontend
    |
    v
compiler
    |
    v
compiled representation
    |
    v
runtime

not:

runtime -> grammar

---

40. Testing Contract

Every compile grammar file requires tests.

At minimum:

Positive

- valid compilation declarations;
- valid compile-time controls;
- valid target intent;
- valid optimization intent;
- valid lowering intent;
- valid code-generation intent;
- valid feature selection;
- valid requirements;
- valid combinations.

Negative

- malformed compile directives;
- malformed expressions;
- duplicate/conflicting syntax where prohibited;
- invalid nesting;
- invalid modifiers;
- invalid target syntax;
- invalid optimization syntax.

Boundary

- empty collections where legal;
- very large collections;
- deeply nested compilation constructs;
- large optimization pipelines;
- large requirement sets;
- large target descriptions;
- large generated-source regions.

Cross-domain

At minimum test:

classical + compile
quantum + compile
HDL + compile
quantum + classical + compile
quantum + hardware + compile
HDL + hardware + compile
AI + quantum + compile
distributed + compile
classical + quantum + HDL + hardware + compile

---

41. Scalability Tests

Tests must explicitly prove that grammar does not impose limits on:

qubits
cores
threads
GPUs
FPGAs
nodes
devices
memory
operations
pipeline stages
optimization objectives
compilation requirements
features
targets

The tests must use generated inputs where appropriate rather than merely testing a handful of manually chosen sizes.

A test that uses a finite value is not itself a language maximum.

---

42. Determinism

Given the same source and grammar version:

source
    |
    v
lexer
    |
    v
parser

must produce deterministic syntactic results.

No grammar construct may depend on:

- current time;
- random state;
- hardware discovery;
- filesystem ordering;
- network state;
- provider availability.

Compiler-level optimization randomness must be controlled downstream through explicit reproducibility mechanisms.

---

43. Security

Compilation grammar must not provide an implicit security bypass.

It must not grant source programs direct access to:

- host filesystem;
- host network;
- credentials;
- secrets;
- devices;
- processes;
- compiler internals.

All external resources must cross explicit compiler/security boundaries.

---

44. Provenance

Compilation-related AST nodes should preserve enough provenance for downstream systems to answer:

Where did this requirement come from?
Which source construct requested it?
Which compilation decision consumed it?
Which transformation resulted from it?
Which final artifact represents it?

This is especially important for:

- optimization;
- quantum compilation;
- QEC;
- resilience;
- hardware adaptation;
- reproducibility;
- diagnostics.

---

45. Reproducibility

The language must distinguish:

semantic reproducibility

from:

implementation reproducibility

A program must preserve its meaning across target changes.

Where deterministic compilation is requested, downstream compiler components must be able to use:

- explicit policies;
- stable ordering;
- deterministic pass selection;
- explicit seeds where randomness is meaningful;
- stable serialization;
- provenance.

The grammar must not silently introduce randomness.

---

46. Resource Resolution

Compilation requirements flow into the universal resource system:

source requirement
       |
       v
compile AST
       |
       v
semantic requirement
       |
       v
resource/capability analysis
       |
       v
available target capabilities
       |
       v
target resolution

If a requirement cannot be satisfied, the compiler must report a semantic/resource diagnostic.

The grammar itself must not reject the program merely because the current machine is small.

For example, a program requiring a large quantum resource may be syntactically and semantically valid while being unschedulable on a particular current backend.

---

47. Portable Compilation

Compilation should produce a representation that is as target-independent as the semantics permit.

The preferred conceptual path is:

Zamani source
      |
      v
frontend AST
      |
      v
semantic model
      |
      v
canonical IR
      |
      +--> target-independent optimization
      |
      v
target resolution
      |
      v
target-specific lowering

Target-specific details must be introduced as late as possible.

---

48. Implementation Order

The compile subsystem must be implemented only after its upstream contracts are stable.

Recommended order:

1. specification/
2. lexer/
3. core/
4. types/
5. expressions/
6. statements/
7. declarations/
8. functions/
9. modules/
10. effects/
11. memory/
12. concurrency/
13. classical/
14. quantum/
15. hybrid/
16. HDL/
17. hardware/
18. distributed/
19. AI/data/
20. networking/
21. security/
22. resources/
23. compile/
24. execution/
25. interoperability/
26. dialects/
27. macros/metaprogramming/
28. validation
29. integration tests

Within "compile/":

compile.g4
    |
    +--> compile-time.g4
    +--> target.g4
    +--> optimization.g4
    +--> conditional-compilation.g4
    +--> feature-selection.g4
    +--> code-generation.g4
    +--> lowering.g4

Each module must be completed against its integration contract before downstream semantic/compiler implementation begins.

---

49. Independent File Completion Contract

A compile grammar file is not complete merely because ANTLR accepts it.

Before marking a file complete, verify:

Syntax

- all rules are unambiguous;
- token ownership is established;
- no duplicated lexical rules exist;
- no unreachable intended production exists.

Ownership

- every construct has one owner;
- no responsibility leaks into another subsystem;
- no duplicated IR representation exists.

Integration

- canonical parser integration point is documented;
- AST mapping is documented;
- semantic consumer is identified;
- downstream compiler consumer is identified.

Scalability

- no machine-size maximum;
- no fixed topology;
- no fixed device;
- no fixed qubit count;
- no fixed processor count;
- no fixed accelerator count.

Safety

- no embedded executable code;
- no unsafe;
- no filesystem/network behavior;
- no device access.

Compatibility

- version behavior is defined;
- future extension space is preserved;
- deprecation path is documented.

Tests

- positive tests;
- negative tests;
- boundary tests;
- deterministic tests;
- scalability tests;
- cross-domain tests.

Only after all of these pass is the file considered complete.

---

50. Hard-Coding Audit

Every compile grammar file must be searched for:

MAX_
32
64
128
256
512
1024
fixed qubit counts
fixed core counts
fixed device counts
fixed topology sizes
fixed addresses
fixed vendor IDs
fixed accelerator IDs
fixed machine names

A numeric literal is not automatically forbidden.

It must be classified as:

1. language semantic value;
2. literal syntax;
3. example;
4. test fixture;
5. resource expression;
6. implementation limit;
7. target property;
8. accidental hard-coding.

Only category 8 is an architectural defect.

---

51. Important Distinction About Numeric Values

The rule is not:

«"Zamani must contain no numbers."»

The rule is:

«"Zamani grammar must not turn arbitrary physical machine properties into permanent language limits."»

For example:

array[1024]

may be a legitimate program-level semantic request.

It must not imply:

MAX_ARRAY_SIZE = 1024

Likewise:

require qubits >= N

may be meaningful.

It must not become:

MAX_QUBITS = N

---

52. Grammar Versus Semantic Validation

Grammar answers:

«Is this syntactically a valid Zamani compilation construct?»

Semantic analysis answers:

«Does this construct make sense?»

Resource analysis answers:

«Can the requested requirement be satisfied?»

Target resolution answers:

«Which available execution environment can satisfy it?»

Compilation answers:

«How can the semantics be lowered?»

Runtime answers:

«How can the compiled computation be executed?»

These boundaries must remain separate.

---

53. Production Readiness Checklist

"grammar/compile/" is production-ready only when all of the following are true:

- [ ] Every compilation construct has one owner.
- [ ] "compile.g4" is the composition boundary.
- [ ] "compile-time.g4" owns compilation-time control.
- [ ] "target.g4" owns target intent.
- [ ] "optimization.g4" owns optimization intent.
- [ ] Code generation syntax is separated from implementation.
- [ ] Lowering syntax is separated from lowering implementation.
- [ ] Expressions are owned by "expressions/".
- [ ] Types are owned by "types/".
- [ ] Resources are owned by "resources/".
- [ ] Hardware is owned by "hardware/".
- [ ] Quantum semantics remain owned by quantum frontend/semantic layers.
- [ ] "quantum::ir" remains canonical.
- [ ] Scheduling is not implemented by grammar.
- [ ] Routing is not implemented by grammar.
- [ ] QEC is not implemented by grammar.
- [ ] ZQN is not implemented by grammar.
- [ ] Resilience is not implemented by grammar.
- [ ] Runtime is not implemented by grammar.
- [ ] No circular grammar/IR dependency exists.
- [ ] No machine-size maximum exists.
- [ ] No fixed qubit ceiling exists.
- [ ] No fixed processor ceiling exists.
- [ ] No fixed accelerator ceiling exists.
- [ ] No fixed topology exists.
- [ ] No device is implicitly selected.
- [ ] No hardware discovery occurs during parsing.
- [ ] No filesystem access occurs during parsing.
- [ ] No network access occurs during parsing.
- [ ] No embedded Rust actions exist.
- [ ] Rust integration is safe.
- [ ] Rust 1.97/1.97.1 compatibility is tested.
- [ ] Positive tests exist.
- [ ] Negative tests exist.
- [ ] Boundary tests exist.
- [ ] Cross-domain tests exist.
- [ ] Determinism tests exist.
- [ ] Scalability tests exist.
- [ ] Compatibility tests exist.
- [ ] Hard-coding audit passes.
- [ ] Documentation agrees with grammar.
- [ ] Grammar agrees with AST contracts.
- [ ] AST contracts agree with semantic analysis.
- [ ] Semantic contracts agree with compiler/IR boundaries.

---

54. Final Architecture

The completed compilation grammar must implement this architectural principle:

                 Zamani Source
                       |
                       v
                    Grammar
                       |
                       v
                       AST
                       |
                       v
              Semantic Analysis
                       |
          +------------+------------+
          |            |            |
          v            v            v
      Resources     Quantum      Classical
      /Targets        IR           IR
          |            |            |
          +------------+------------+
                       |
                       v
                 Canonical IR
                       |
          +------------+------------+
          |            |            |
          v            v            v
     Optimization   Routing     Lowering
          |            |            |
          +------------+------------+
                       |
                       v
                   Scheduling
                       |
                       v
                  Hardware HAL
                       |
                       v
                    Runtime
                       |
          +------------+-------------+
          |            |             |
         CPU          GPU           QPU
          |            |             |
         FPGA         ASIC       Future target

The grammar remains above these implementation layers.

It describes the program.

It does not become the machine.

---

55. Final POCO-REAF Contract

The ultimate contract of "grammar/compile/" is:

ONE SOURCE PROGRAM
        |
        v
ONE STABLE SEMANTIC MEANING
        |
        v
ONE PORTABLE COMPILATION MODEL
        |
        +-------------------------------+
        |               |               |
        v               v               v
      small           large           future
      machine         machine         machine
        |               |               |
        v               v               v
     classical       quantum        heterogeneous
        |               |               |
        +---------------+---------------+
                        |
                        v
                SAME PROGRAM SEMANTICS

Therefore:

«Zamani compilation syntax must describe compilation intent without permanently encoding the limitations of the machine available today.»

The grammar must remain capable of expressing computation from the smallest supported system to arbitrarily large heterogeneous systems, with actual limits determined by resources, capabilities, policies, compiler capacity, and execution environments rather than accidental grammar constants.

This is the required foundation for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

and ultimately:

Zamani — From Atom to Everywhere.