Zamani Determinism Specification

Path: "grammar/spec/determinism.md"
Language: Zamani
Specification role: Normative production specification for deterministic semantics, reproducible compilation, deterministic analysis, explicit nondeterminism, concurrency, distributed execution, quantum behavior, diagnostics, and target-independent execution
Status: Production / Normative
Specification version: 1.0
Grammar technology: ANTLR4
Implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Rust safety requirement: Production Zamani implementation MUST use safe Rust only; Rust "unsafe" MUST NOT be used
Primary portability objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Scalability objective: Deterministic semantics MUST remain valid from the smallest supported computation to arbitrarily large computations, subject only to program semantics, declared requirements, implementation representation limits, and resources actually available
Canonical grammar composition root: "grammar/Zamani.g4"
Canonical quantum semantic boundary: "quantum::ir"

---

0. Purpose

This document defines the normative determinism contract of the Zamani programming language.

It establishes:

- what deterministic behavior means;
- what nondeterministic behavior means;
- what randomness means;
- what quantum measurement means;
- how concurrency interacts with determinism;
- how distributed execution interacts with determinism;
- how evaluation order is determined;
- how unspecified implementation choices differ from nondeterminism;
- how compilers and optimizers MUST preserve deterministic semantics;
- how deterministic diagnostics are produced;
- how deterministic compilation and reproducibility are achieved;
- how deterministic behavior interacts with resources and capabilities;
- how deterministic semantics interact with classical, quantum, hybrid, HDL, AI, data, networking, security, and distributed computation;
- how deterministic behavior integrates with the AST, semantic model, IR, compiler, runtime, routing, scheduling, QEC, ZQN, HAL, and backends;
- how deterministic behavior scales without artificial machine-size limits;
- how conformance is tested.

This document is a semantic contract.

It is not:

- a parser implementation;
- a lexer implementation;
- a scheduler implementation;
- a distributed runtime implementation;
- a random-number generator implementation;
- a quantum simulator implementation;
- a quantum hardware specification;
- a hardware topology specification;
- a QEC implementation;
- a ZQN implementation;
- a HAL implementation;
- a vendor API;
- a backend implementation.

Those systems consume this contract.

---

1. Normative Terminology

The following terms are normative:

- MUST — mandatory.
- MUST NOT — prohibited.
- REQUIRED — mandatory.
- SHOULD — recommended unless a documented technical reason exists otherwise.
- SHOULD NOT — discouraged unless a documented technical reason exists.
- MAY — permitted.
- DETERMINISTIC — identical semantic inputs and relevant semantic environment produce the same observable behavior.
- NONDETERMINISTIC — multiple observable outcomes are explicitly permitted by the language semantics.
- RANDOM — behavior whose outcome is intentionally determined by a defined stochastic source.
- UNSPECIFIED — the implementation may choose among permitted behaviors without exposing that choice as program nondeterminism.
- IMPLEMENTATION-DEFINED — implementation choice that MUST be documented and stable within its declared implementation contract.
- RESOURCE-DEPENDENT — behavior whose feasibility depends on available resources.
- TARGET-DEPENDENT — behavior dependent on an explicitly selected target.
- OBSERVABLE — behavior that the Zamani semantic model permits a program or external observer to distinguish.
- SEMANTIC EQUIVALENCE — two executions satisfy the same source-level semantic contract.
- REPRODUCIBLE — repeated compilation/execution under the same declared conditions can reproduce the required artifact or observable behavior.
- STABLE ORDER — an ordering explicitly required by semantic rules.
- CANONICAL ORDER — deterministic ordering selected solely to eliminate implementation-dependent ordering where order itself has no semantic significance.

---

2. Scope

Determinism applies across the complete Zamani pipeline:

Zamani source
    │
    ▼
lexical analysis
    │
    ▼
parsing
    │
    ▼
frontend AST
    │
    ▼
structural validation
    │
    ▼
name/type/effect/resource/capability analysis
    │
    ▼
canonical semantic model
    │
    ▼
canonical IR
    │
    ├── classical IR
    ├── quantum::ir
    └── HDL/hardware/domain IR
    │
    ▼
verification
    │
    ▼
optimization
    │
    ├── routing
    ├── scheduling
    ├── resilience
    ├── QEC
    └── ZQN
    │
    ▼
HAL
    │
    ▼
target realization
    │
    ▼
runtime

Determinism MUST be preserved across every layer where the source program requires deterministic semantics.

A downstream implementation MUST NOT introduce observable nondeterminism merely because its internal implementation is parallel, distributed, heterogeneous, randomized, or hardware-dependent.

---

3. Authority and Repository Integration

The determinism contract integrates with the existing repository authority hierarchy.

The intended relationship is:

grammar/specification/language.md
              │
              ▼
grammar/spec/semantics.md
              │
              ▼
grammar/spec/determinism.md
              │
      ┌───────┼────────┐
      │       │        │
      ▼       ▼        ▼
 effects  concurrency  quantum
      │       │        │
      └───────┼────────┘
              ▼
       grammar/*.g4
              │
              ▼
       grammar/Zamani.g4
              │
              ▼
            lexer
              │
              ▼
            parser
              │
              ▼
      frontend domain-neutral AST
              │
              ▼
      semantic analysis
              │
              ▼
      canonical semantic model
              │
              ▼
       canonical domain IR
              │
              ▼
 optimization / lowering
              │
              ▼
 routing / scheduling / resilience
              │
              ▼
        ZQN / HAL
              │
              ▼
      target realization

This document MUST NOT create another semantic authority.

"grammar/spec/semantics.md" owns the general semantic model.

"grammar/spec/effects.md" owns the effect-system contract.

"grammar/spec/concurrency.md" owns concurrency semantics.

"grammar/spec/quantum.md" owns quantum-domain semantics.

"grammar/spec/portability.md" owns portability and POCO-REAF.

"grammar/spec/compatibility.md" owns compatibility and implementation coherence.

This document owns their determinism-related intersection.

---

4. Determinism Is a Semantic Property

Determinism is not a property of a particular CPU, compiler, scheduler, runtime, or operating system.

A program is deterministic when its semantic contract determines one observable result for a given relevant semantic environment.

Conceptually:

P = program semantics
I = semantic inputs
E = relevant semantic environment

Deterministic(P, I, E)
    =>
    exactly one permitted observable result

This does not require the implementation to use:

- one processor;
- one thread;
- one core;
- one machine;
- one node;
- one accelerator;
- one QPU;
- one execution order internally.

A deterministic program MAY be implemented using massive parallelism.

The implementation MUST preserve the program's observable semantics.

---

5. Determinism and POCO-REAF

Determinism MUST coexist with:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever»

POCO-REAF does not require every implementation to use identical execution strategies.

The same deterministic source program MAY be realized through:

single-threaded CPU
multithreaded CPU
GPU
FPGA
ASIC
distributed cluster
heterogeneous accelerator system
quantum simulator
QPU
hybrid system
future computational substrate

provided that all realizations preserve the deterministic semantic contract.

Therefore:

deterministic source
        ≠
identical implementation

Instead:

deterministic source
        ↓
canonical semantics
        ↓
different valid implementations
        ↓
same required observable result

---

6. Three Fundamental Execution Classes

Zamani MUST distinguish at least three classes.

6.1 Deterministic

The observable result is uniquely determined.

Example:

let x = 10;
let y = 20;
let z = x + y;

The observable value of "z" is deterministic.

---

6.2 Explicitly Nondeterministic

The language explicitly permits more than one valid outcome.

Examples may include:

- nondeterministic scheduling where scheduling is semantically observable;
- racing external events where the language explicitly exposes the race;
- distributed arrival order;
- explicitly nondeterministic APIs.

The nondeterminism MUST be represented in the semantic model.

It MUST NOT be accidentally introduced by an implementation.

---

6.3 Explicitly Random

Randomness is a defined source of stochastic behavior.

For example:

random()

may produce different values across executions.

Randomness MUST NOT be confused with:

unordered map iteration
thread scheduling
hash seed
compiler traversal order
network packet arrival
backend selection

Those are implementation-order issues unless explicitly modeled as semantic nondeterminism.

---

7. Determinism Is the Default for Pure Computation

Where the language does not explicitly introduce:

- randomness;
- external nondeterminism;
- concurrency races;
- quantum measurement;
- nondeterministic effects;

ordinary pure computation MUST be deterministic.

A compiler MUST NOT introduce observable nondeterminism simply because:

- optimization is enabled;
- compilation is parallel;
- execution is parallel;
- data is distributed;
- multiple equivalent hardware targets exist.

---

8. Relevant Semantic Environment

Determinism is defined relative to all information that is semantically relevant.

The relevant environment MAY include:

- explicit program inputs;
- explicit configuration;
- declared random seeds;
- explicitly observed external state;
- language version;
- imported semantic definitions;
- declared capabilities;
- explicitly observable resource state;
- externally provided data;
- quantum measurement outcomes;
- defined time values where time is an explicit input/effect.

It MUST NOT implicitly include accidental implementation state such as:

- hash-map iteration order;
- pointer addresses;
- allocation addresses;
- process IDs;
- thread IDs;
- CPU IDs;
- compiler worker IDs;
- backend traversal order;
- filesystem directory ordering;
- network arrival order unless explicitly semantic.

---

9. Hidden Sources of Nondeterminism Are Prohibited

The following MUST NOT affect the meaning of a deterministic program:

- hash randomization;
- hash-map iteration order;
- set iteration order;
- filesystem enumeration order;
- thread scheduling;
- compiler parallel worker order;
- backend discovery order;
- device enumeration order;
- memory addresses;
- allocator behavior;
- pointer identity unless explicitly semantic;
- machine-specific floating-point implementation differences where the type contract requires deterministic arithmetic;
- random temporary names;
- nondeterministic diagnostic ordering;
- unstable serialization ordering;
- unordered dependency traversal.

If any of these can affect semantics, the semantic contract MUST explicitly define the behavior.

---

10. Canonical Ordering

When ordering is not semantically meaningful but deterministic output is required, the implementation MUST establish a canonical ordering.

Canonical ordering MAY be based on:

- source position;
- declaration identity;
- stable semantic identifier;
- canonical qualified name;
- explicit dependency order;
- specification-defined ordering.

Canonical ordering MUST NOT depend on:

- memory addresses;
- hash iteration;
- thread completion order;
- machine topology;
- arbitrary backend enumeration.

Canonical ordering is an implementation mechanism for reproducibility.

It does not automatically make an inherently nondeterministic program deterministic.

---

11. Semantic Ordering

When order affects semantics, the language MUST define that order explicitly.

Examples include:

a(); 
b();

when "a" and "b" have observable effects.

The compiler MUST preserve required ordering.

For independent pure expressions, reordering MAY be permitted.

The distinction is:

semantic order
    ≠
incidental source traversal order

---

12. Evaluation Order

Evaluation order MUST be specified wherever it can affect observable behavior.

The implementation MUST NOT rely on unspecified evaluation order for deterministic programs.

For example, if:

f() + g()

permits both "f" and "g" to mutate the same observable state, the language MUST define whether:

f before g
g before f

is permitted.

If both are semantically permitted, the program is not deterministic with respect to that observable state.

If the language requires a deterministic result, semantic analysis MUST either:

- establish a required order;
- prove independence;
- reject the ambiguous construct;
- require an explicit ordering construct.

---

13. Pure Expressions

A pure expression MUST NOT depend on hidden mutable state.

Pure expressions MAY be:

- reordered;
- duplicated;
- eliminated;
- memoized;
- parallelized;
- vectorized;
- distributed;
- specialized.

Such transformations MUST preserve:

- value semantics;
- type semantics;
- effects;
- ownership;
- resource correctness;
- termination requirements;
- explicitly observable timing where applicable.

---

14. Effects and Determinism

Effects are a primary boundary between deterministic and potentially nondeterministic behavior.

An effect MUST declare or otherwise expose the semantic properties necessary to determine whether it is:

- deterministic;
- random;
- nondeterministic;
- externally dependent;
- order-sensitive;
- time-sensitive;
- concurrency-sensitive.

An effect implementation MUST NOT secretly introduce nondeterminism into an effect whose contract is deterministic.

---

15. Effect Ordering

Effect ordering MUST be preserved whenever ordering is semantically observable.

For example:

write A
write B

cannot be freely reordered if an external observer can distinguish the order.

An optimizer MAY reorder independent effects only when the effect system proves the transformation semantically safe.

---

16. Randomness

Randomness MUST be explicit.

A random operation MUST have a defined semantic source.

Conceptually:

random source
    ↓
random value

The source MAY be:

- explicitly seeded pseudorandom generation;
- cryptographically secure randomness;
- hardware randomness;
- quantum randomness;
- externally supplied entropy.

The source semantics MUST be distinguishable.

---

17. Seeded Randomness

Where a deterministic pseudorandom sequence is requested, the seed MUST be part of the semantic input.

Conceptually:

R = RandomStream(seed)

Then:

same seed
+
same semantic random algorithm/version
+
same requested sequence

MUST produce the same semantic sequence.

The implementation MUST NOT silently substitute another random algorithm if the source program requires reproducibility.

---

18. Randomness and Portability

A portable random computation MUST distinguish:

reproducible pseudorandomness

from:

true/environmental randomness

A program requesting reproducible pseudorandomness SHOULD remain reproducible across targets.

A program requesting environmental entropy is inherently dependent on the environment and MUST be classified accordingly.

---

19. Random Algorithm Identity

If reproducibility across compiler/runtime versions is required, the random algorithm identity MUST be part of the reproducibility contract.

A seed alone is insufficient if the implementation is allowed to change the algorithm.

Therefore a reproducible random stream conceptually consists of:

algorithm identity
algorithm version
seed
stream/substream identity
requested sequence

The implementation MAY use a completely different internal representation as long as the required semantic sequence is preserved.

---

20. Parallel Randomness

Parallel random generation MUST NOT depend on thread scheduling.

Invalid conceptual implementation:

worker 1 gets next random number
worker 2 gets next random number
worker 3 gets next random number

where worker assignment determines semantic results.

Instead, reproducible parallel randomness SHOULD use deterministic logical stream partitioning, such as:

master seed
    │
    ├── logical stream 0
    ├── logical stream 1
    ├── logical stream 2
    └── ...

Logical stream identity MUST NOT depend on physical worker identity.

---

21. Concurrency and Determinism

Concurrency MUST NOT automatically imply nondeterminism.

A concurrent computation may be deterministic when its dependencies and synchronization fully determine the result.

For example:

A ──► C
B ──► C

allows "A" and "B" to execute concurrently while "C" waits for both.

The physical execution order of "A" and "B" is irrelevant if they are semantically independent.

---

22. Deterministic Parallelism

Zamani SHOULD support deterministic parallelism as a first-class semantic concept.

A deterministic parallel region means:

parallel execution
+
deterministic observable result

The implementation MAY use:

- one worker;
- many workers;
- CPU;
- GPU;
- FPGA;
- distributed nodes;
- heterogeneous devices.

The result MUST remain semantically deterministic.

---

23. Reduction Operations

Parallel reductions are a major source of accidental nondeterminism.

For example:

sum(values)

may be mathematically associative but finite floating-point addition is generally not exactly associative.

Therefore the language MUST distinguish:

- mathematically associative reduction;
- representation-exact deterministic reduction;
- implementation-dependent reduction;
- explicitly nondeterministic reduction.

If exact reproducibility is required, the reduction contract MUST define its combination order or an equivalent deterministic algorithm.

---

24. Floating-Point Determinism

Floating-point operations MUST define the required semantic precision and rounding behavior where reproducibility requires it.

A compiler MUST NOT silently change:

- rounding mode;
- precision;
- contraction behavior;
- exceptional-value behavior;
- NaN handling;
- signed-zero semantics;

when those properties are observable under the selected numeric contract.

Optimization MAY change implementation strategy only when the semantic result remains valid.

---

25. Numerical Reproducibility Levels

Zamani MAY distinguish at least:

Level 0 — Semantic equivalence

Results satisfy the language's mathematical/semantic contract but may differ in representation details allowed by that contract.

Level 1 — Value reproducibility

Repeated executions produce identical observable values.

Level 2 — Bitwise reproducibility

Relevant serialized results are byte-for-byte identical.

Level 3 — Artifact reproducibility

Compilation produces identical reproducible artifacts under identical declared build inputs.

The selected level MUST be explicit where it matters.

---

26. Compiler Determinism

The compiler itself MUST behave deterministically for deterministic compilation inputs.

Given identical:

- source;
- imported source;
- compiler version;
- language version;
- dependency versions;
- feature configuration;
- target-independent compilation settings;
- reproducibility settings;

the compiler SHOULD produce the same canonical intermediate results.

Compilation parallelism MUST NOT change semantic results.

---

27. Compiler Worker Ordering

Compiler passes MUST NOT depend on the order in which parallel workers finish.

Invalid:

first worker to finish wins semantic decision

unless the language explicitly defines that race as semantic nondeterminism.

Compiler workers MUST instead communicate through deterministic data structures and stable conflict resolution.

---

28. Deterministic Name Resolution

Name resolution MUST be deterministic.

Given identical source and module graph:

name → declaration

MUST resolve identically.

If multiple declarations are equally applicable, the compiler MUST produce a deterministic ambiguity diagnostic.

It MUST NOT select one based on:

- filesystem order;
- hash order;
- dependency discovery order;
- network response order;
- thread timing.

---

29. Deterministic Type Inference

Type inference MUST be deterministic.

The result MUST NOT depend on:

- hash-map order;
- traversal order;
- backend availability;
- CPU count;
- compiler thread count;
- target enumeration.

If multiple valid inferred types remain and the language does not define a canonical choice, compilation MUST fail with a deterministic ambiguity diagnostic.

---

30. Deterministic Generic Specialization

Generic specialization MUST be deterministic.

The compiler MAY choose:

- monomorphization;
- specialization;
- polymorphic lowering;
- runtime dispatch;

according to the implementation contract.

But the semantic result MUST remain unchanged.

Specialization order MUST NOT alter program meaning.

---

31. Deterministic Optimization

Optimization is permitted to transform implementation.

It MUST NOT transform semantics.

For deterministic input:

unoptimized semantics
=
optimized semantics

under the language's observable equivalence relation.

An optimizer MUST NOT use random choices unless:

- the random choice is explicitly controlled;
- the choice cannot affect observable semantics;
- reproducible build requirements are satisfied.

---

32. Randomized Optimization

A compiler MAY use randomized algorithms internally.

If randomized optimization cannot affect generated semantics, it MAY remain an implementation detail.

If reproducible artifacts are required, the optimizer's randomness MUST be:

- disabled;
- deterministically seeded;
- or otherwise canonicalized.

Compiler randomness MUST NOT alter program meaning.

---

33. Deterministic Intermediate Representations

Canonical semantic representations and IRs MUST have deterministic representations where serialization, comparison, caching, or verification depends on ordering.

Canonical IR serialization SHOULD use:

- stable node identity;
- stable operand ordering;
- stable attribute ordering;
- stable declaration ordering;
- canonical numeric representation;
- canonical metadata ordering.

IRs MUST NOT depend on:

- memory addresses;
- hash iteration;
- nondeterministic graph traversal.

---

34. Canonical Graph Ordering

Graphs are frequently inherently unordered.

When graph order is semantically irrelevant but deterministic serialization is required, the implementation MUST canonicalize the graph.

Canonicalization MUST preserve graph semantics.

It MUST NOT accidentally turn an unordered semantic relation into an ordered semantic relation.

---

35. Deterministic Diagnostics

Diagnostics MUST be deterministic.

Given identical source and compilation context, diagnostics MUST be emitted in a stable order.

Diagnostic ordering SHOULD be based on:

1. source location;
2. semantic dependency;
3. diagnostic severity;
4. stable diagnostic identifier;
5. canonical secondary ordering.

The implementation MUST NOT order diagnostics according to:

- thread completion;
- hash-map iteration;
- backend discovery;
- random IDs.

---

36. Diagnostic Identity

Every production diagnostic SHOULD have a stable identifier.

Conceptually:

ZAM-<domain>-<category>-<number>

The exact identifier scheme is owned by the diagnostics specification.

The identifier MUST remain stable across non-breaking compiler releases unless a compatibility transition is explicitly documented.

---

37. Error Recovery

Parser error recovery MAY report multiple errors.

Recovery MUST NOT:

- manufacture semantic meaning;
- randomly select a parse;
- reorder errors nondeterministically;
- hide the first deterministic root cause;
- create a valid semantic AST from invalid source.

The production compiler MUST distinguish:

error-recovery AST

from:

validated semantic AST

---

38. Parsing Determinism

The parser MUST be deterministic.

Identical token streams MUST produce the same parse structure.

Ambiguous syntax MUST be resolved through:

- grammar disambiguation;
- precedence;
- associativity;
- explicit language rules;
- deterministic parser policy.

It MUST NOT be resolved according to:

- machine architecture;
- hash ordering;
- random selection;
- parser thread timing.

---

39. Lexical Determinism

The lexer MUST deterministically tokenize identical valid source bytes.

The lexical result MUST NOT depend on:

- locale;
- operating system;
- filesystem;
- CPU architecture;
- thread scheduling;
- environment variables,

unless those are explicitly part of the lexical contract.

UTF-8 source handling MUST remain deterministic.

---

40. Unicode Determinism

If Unicode normalization or classification is used, the exact semantic rules MUST be versioned.

The implementation MUST NOT allow host-library Unicode version changes to silently alter identifier identity or lexical meaning in a supposedly compatible language version.

---

41. Serialization Determinism

Any canonical Zamani serialization used for:

- AST;
- semantic model;
- IR;
- cache keys;
- build artifacts;
- manifests;
- provenance;
- reproducibility;

MUST define deterministic serialization rules.

Maps and sets MUST use canonical ordering when serialized.

Binary representations MUST define:

- byte order;
- integer representation;
- floating representation where applicable;
- string encoding;
- optional-field encoding;
- collection ordering.

---

42. Hashes and Cache Keys

Hashing MUST distinguish:

semantic identity

from:

implementation address

Cache keys for deterministic compilation SHOULD depend on canonical semantic inputs.

They MUST NOT depend on:

- temporary filesystem paths unless declared;
- memory addresses;
- process IDs;
- random temporary names;
- worker IDs;
- host-specific incidental ordering.

---

43. Build Reproducibility

A reproducible build MUST identify all semantic inputs.

At minimum, this MAY include:

- source content;
- language version;
- compiler version;
- compiler configuration;
- dependency versions;
- feature flags;
- dialect versions;
- target-independent semantic configuration;
- deterministic random seeds;
- relevant generated sources.

Undeclared environment state MUST NOT silently affect a reproducible build.

---

44. Environment Variables

Environment variables are external inputs.

If a program reads an environment variable, that behavior MUST be represented through the relevant effect/environment model.

A compiler MUST NOT read arbitrary environment variables and use them to silently change source semantics.

Build systems MAY provide explicitly declared environment inputs.

---

45. Filesystem Determinism

Filesystem traversal order MUST NOT be treated as semantic order unless explicitly specified.

Directory entries MUST be canonically ordered when deterministic behavior requires it.

A compiler MUST NOT generate different semantic programs because an operating system returns directory entries in a different order.

---

46. Network Determinism

Network arrival order is inherently external unless the program explicitly treats it as semantic input.

For deterministic network processing, the program MUST establish an ordering rule such as:

- sequence number;
- logical timestamp;
- causal order;
- explicit priority;
- deterministic merge rule.

The runtime MUST NOT pretend that arbitrary network arrival order is deterministic.

---

47. Distributed Determinism

Distributed execution MAY be deterministic.

A distributed program MUST explicitly define the semantic ordering or conflict-resolution model when message order can affect results.

Valid deterministic approaches include:

- causal ordering;
- logical clocks;
- deterministic reduction;
- transactional ordering;
- consensus-defined ordering;
- deterministic conflict resolution.

Physical network timing MUST NOT silently determine semantic results where deterministic semantics are required.

---

48. Distributed Failure

Failure introduces another semantic dimension.

A deterministic distributed program MUST distinguish:

same inputs + same declared failure model

from:

same inputs + arbitrary external failure

External failures MAY be nondeterministic.

If failures are observable, the program's effect model MUST represent them.

A runtime MUST NOT hide a failed distributed operation by silently changing semantics.

---

49. Concurrency Race Semantics

Data races MUST NOT be used as an implicit nondeterministic programming mechanism.

If the memory/type system prohibits a race, semantic analysis MUST reject it.

If a language construct intentionally permits race-like behavior, its semantics MUST explicitly define:

- permitted outcomes;
- synchronization;
- memory visibility;
- ordering;
- ownership;
- reproducibility classification.

---

50. Atomic Operations

Atomic operations MUST have defined ordering semantics.

The semantic model MUST distinguish:

- relaxed;
- acquire;
- release;
- acquire-release;
- sequentially consistent;

or equivalent language-level concepts if exposed.

The grammar records the requested semantic mode.

The compiler and target determine implementation.

---

51. Memory Model

Determinism depends on the memory model.

The memory model MUST define:

- visibility;
- ordering;
- ownership;
- aliasing;
- mutation;
- synchronization;
- atomicity;
- lifetime.

A compiler MUST NOT assume stronger or weaker ordering than the language contract permits.

---

52. Deterministic Data Parallelism

Data-parallel operations SHOULD expose deterministic semantics whenever mathematically possible.

For example:

map(f, data)

is deterministic if:

- "f" is deterministic;
- the input sequence is deterministic;
- output ordering is specified.

A data-parallel implementation MAY execute elements in arbitrary physical order if the semantic result is order-independent.

---

53. Ordered and Unordered Collections

Zamani MUST distinguish collections whose order is semantic from collections whose order is not.

For an ordered collection:

[a, b, c]

and:

[c, b, a]

are distinct.

For an unordered set, the semantic model MUST NOT accidentally treat iteration order as meaningful.

Tooling MAY impose canonical display order.

---

54. Maps and Dictionaries

Map key/value associations are semantic.

Iteration order MUST be explicitly classified as either:

- defined;
- insertion-preserving;
- canonical;
- unspecified;
- nondeterministic.

If deterministic iteration is required, the language/runtime MUST provide a deterministic mechanism.

A compiler MUST NOT accidentally expose hash-table ordering as semantic behavior.

---

55. Quantum Determinism

Quantum computation requires a special distinction.

Quantum evolution may be deterministic at the state-transform level while measurement outcomes are probabilistic.

Therefore Zamani MUST distinguish:

deterministic quantum evolution

from:

probabilistic measurement outcome

A quantum program is not automatically "nondeterministic" merely because it contains quantum operations.

---

56. Quantum State Evolution

A deterministic quantum operation applied to a defined state has a mathematically defined semantic transformation.

The compiler MAY implement it using:

- state vectors;
- tensor networks;
- stabilizer representations;
- symbolic representations;
- hardware-native operations;
- another valid representation.

The representation MUST NOT change the defined quantum semantics.

---

57. Quantum Measurement

Quantum measurement is an explicitly probabilistic semantic operation unless the measured state guarantees a deterministic outcome.

The measurement result MUST be represented as an explicit semantic event.

The runtime MUST NOT convert measurement into a deterministic constant merely because one backend happens to know a special case unless that result is semantically proven.

---

58. Quantum Randomness

Quantum measurement MAY be a source of genuine randomness.

This MUST be distinguished from:

compiler randomness
scheduler nondeterminism
device enumeration order

A quantum measurement outcome is semantic.

Compiler scheduling order is normally not.

---

59. Quantum Reproducibility

Quantum reproducibility requires a declared model.

Possible levels include:

Semantic reproducibility

The circuit/program is identical and satisfies the same quantum semantics.

Statistical reproducibility

Repeated executions produce statistically consistent distributions.

Simulator reproducibility

A simulator using a defined seed and deterministic numerical model produces repeatable outcomes.

Hardware-shot reproducibility

A physical QPU MAY NOT guarantee identical measurement samples because physical quantum measurement is inherently stochastic.

The language MUST NOT falsely promise bit-for-bit repetition of inherently stochastic quantum measurements.

---

60. Quantum Seed Control

Where a simulator or execution environment exposes seeded sampling, the seed MUST be represented as an explicit execution parameter.

A compiler MUST NOT silently treat a hardware quantum measurement as deterministic merely because a simulator can be seeded.

---

61. Quantum Optimization

Quantum optimization MUST preserve the defined probability distribution and observable semantics.

An optimizer MAY:

- simplify gates;
- cancel inverse operations;
- reorder commuting operations;
- decompose operations;
- choose equivalent representations.

It MUST NOT change:

- measurement semantics;
- required probabilities;
- observable expectation values;
- declared error/fidelity requirements;

unless the transformation is explicitly permitted by the semantic contract.

---

62. Quantum Routing and Scheduling

Routing and scheduling are implementation decisions.

They MAY vary between targets.

For example:

logical q0
logical q1

may be mapped differently on different devices.

This does not constitute source-level nondeterminism.

However, if the source explicitly requests a deterministic physical mapping, the request becomes a target-dependent semantic constraint and MUST be treated as such.

---

63. QEC and Determinism

QEC is downstream from source semantics.

QEC transformations MUST preserve the deterministic/probabilistic classification of the logical computation.

A QEC implementation MAY use:

- different codes;
- different decoders;
- different schedules;
- different physical mappings;

provided the resulting logical semantics satisfy the requested contract.

---

64. ZQN Integration

ZQN represents fault/noise semantics downstream.

Noise MAY be stochastic.

The language MUST distinguish:

source semantic determinism

from:

physical execution noise

A deterministic source program executed on noisy hardware does not thereby become semantically defined as arbitrary nondeterminism.

The runtime MUST expose noise according to the relevant execution/fault contract.

---

65. HDL Determinism

HDL introduces concurrency and hardware timing.

Determinism MUST distinguish:

- synchronous behavior;
- asynchronous behavior;
- event ordering;
- clock semantics;
- reset semantics;
- simulation scheduling;
- synthesis semantics.

The grammar MUST NOT assume that source ordering alone determines physical signal timing.

---

66. Hardware Simulation Determinism

A deterministic simulation MUST use a defined event ordering model.

If multiple events occur at the same logical time, the HDL semantic contract MUST define whether:

- ordering is semantically irrelevant;
- a deterministic priority applies;
- events are resolved together;
- the situation is invalid.

Simulation output MUST NOT depend on host thread scheduling.

---

67. AI and Machine Learning Determinism

AI/ML computation MAY contain:

- randomized initialization;
- stochastic optimization;
- sampling;
- nondeterministic parallel reductions;
- distributed training;
- external data.

Each source of nondeterminism MUST be explicit or documented by the relevant semantic contract.

A training operation SHOULD expose reproducibility configuration where reproducibility is required.

---

68. Data Pipeline Determinism

Data processing MUST distinguish:

ordered input

from:

unordered input

If a pipeline requires deterministic results, transformations such as:

- grouping;
- reduction;
- joins;
- distributed aggregation;
- sampling;
- shuffling;

MUST have defined deterministic semantics where requested.

---

69. Networking and Determinism

Networking is externally influenced.

A deterministic network program MUST define which aspects of network behavior are semantic.

For example:

request ID
sequence number
message ID
logical timestamp

may establish deterministic correlation.

Physical arrival time MUST NOT silently determine semantic ordering where ordering is required to be deterministic.

---

70. Security and Determinism

Security-sensitive randomness MUST NOT be replaced by deterministic pseudorandomness merely to improve reproducibility.

For example, cryptographic key generation may intentionally require secure entropy.

The language MUST allow the program to distinguish:

reproducible randomness

from:

security-required entropy

The compiler MUST NOT weaken a security contract for reproducibility.

---

71. Cryptographic Determinism

Some cryptographic operations are deterministic; others intentionally use randomness.

The semantic contract MUST preserve this distinction.

For example:

hash(message)

may be deterministic.

A randomized cryptographic protocol may not be.

The grammar MUST express semantic intent rather than hard-code a vendor implementation.

---

72. Resource Availability

Resource availability MUST NOT silently change deterministic semantics.

If a deterministic program requires:

resources >= R

and the target cannot satisfy that requirement, the implementation MUST:

- reject compilation;
- defer execution;
- fail explicitly at runtime;
- adapt through a semantically valid alternative.

It MUST NOT silently return a different result.

---

73. Resource-Dependent Parallelism

A deterministic program MAY scale from:

1 execution resource

to:

N execution resources

without changing its semantic result.

The compiler/runtime MAY change:

- partitioning;
- scheduling;
- vectorization;
- parallelism;
- distribution;
- memory placement;
- accelerator use.

The resulting observable behavior MUST remain valid.

---

74. No Artificial Determinism Limits

The language MUST NOT impose universal constants such as:

MAX_TASKS
MAX_THREADS
MAX_WORKERS
MAX_NODES
MAX_MESSAGES
MAX_RANDOM_STREAMS
MAX_QUANTUM_SHOTS
MAX_QUBITS
MAX_OPERATIONS
MAX_COLLECTION_ELEMENTS
MAX_DIAGNOSTICS
MAX_DEPENDENCIES
MAX_GRAPH_NODES

unless a particular quantity is intrinsically bounded by a semantic type or representation defined by the language.

Implementation limits MUST remain implementation/resource limits.

---

75. Tiny-to-Infinite Determinism

The same deterministic semantic model MUST scale from tiny computations to arbitrarily large computations.

"Infinity" means:

«no artificial finite language-level limit.»

It does not mean that physical execution is infinite.

The actual limit may be determined by:

- memory;
- storage;
- compiler capacity;
- runtime capacity;
- communication capacity;
- target resources;
- physical constraints;
- user policy.

These limits MUST NOT redefine deterministic semantics.

---

76. Determinism and Resource Adaptation

A compiler MAY adapt a deterministic computation to available resources.

Examples:

one worker
→
many workers

CPU
→
GPU

local
→
distributed

single-device quantum simulation
→
distributed simulation

The adaptation MUST preserve semantic determinism.

---

77. Deterministic Scheduling

Schedulers MUST distinguish:

schedule order

from:

semantic execution order

If two operations are independent, the scheduler MAY choose either order.

If the program requires:

A before B

the scheduler MUST preserve that dependency.

A scheduler MUST NOT use physical worker identity as a semantic tie-breaker.

---

78. Canonical Scheduler Tie-Breaking

When multiple schedules are semantically equivalent and deterministic reproducibility is required, the scheduler SHOULD use a canonical tie-breaker.

Possible keys include:

- stable operation ID;
- source span;
- dependency order;
- canonical semantic ID.

The tie-breaker MUST NOT alter the semantics of independent operations.

---

79. Deterministic Placement

Placement MAY be nondeterministic internally when all placements are semantically equivalent.

For reproducible builds or reproducible execution plans, placement SHOULD use canonical ordering.

Physical placement MUST remain downstream from portable source semantics.

---

80. Deterministic Routing

Routing may have multiple valid solutions.

A deterministic routing implementation SHOULD choose the same route given the same:

- logical program;
- target description;
- routing policy;
- compiler version;
- routing configuration.

If a route is not semantically relevant, different routes MAY still be considered semantically equivalent.

---

81. Optimization Equivalence

An optimization is valid if:

Observable(Original)
=
Observable(Optimized)

under the applicable semantic equivalence relation.

The optimizer MUST consider:

- values;
- effects;
- ownership;
- resource constraints;
- timing where semantic;
- concurrency;
- quantum semantics;
- hardware intent;
- explicit nondeterminism.

---

82. No Optimization-Induced Nondeterminism

The compiler MUST NOT turn deterministic code into nondeterministic code by:

- unordered parallelization;
- race introduction;
- unstable reductions;
- uncontrolled randomization;
- unspecified floating-point transformations;
- changing evaluation order across effects.

If an optimization requires weaker reproducibility, it MUST be explicitly permitted by the program or compilation mode.

---

83. Deterministic Compilation Modes

The toolchain SHOULD support explicit deterministic/reproducible compilation modes.

A deterministic compilation mode SHOULD:

- canonicalize ordering;
- stabilize identifiers;
- stabilize diagnostics;
- stabilize serialization;
- control compiler randomness;
- stabilize dependency traversal;
- stabilize generated metadata;
- record semantic configuration.

A non-deterministic optimization mode MAY exist where it cannot affect program semantics, but generated artifacts may differ.

---

84. Build Metadata

Generated artifacts MUST NOT contain nondeterministic metadata when reproducible artifacts are requested.

Examples requiring canonical handling:

- timestamps;
- random IDs;
- temporary paths;
- hostnames;
- process IDs;
- worker IDs;
- absolute paths;
- unordered metadata;
- machine-local usernames.

If such information is semantically necessary, it MUST be explicitly declared as build input.

---

85. Source Spans

Source spans MUST be deterministic.

The same source bytes MUST produce the same:

- line numbers;
- column information;
- byte offsets;
- token spans;
- AST spans.

Diagnostic source locations MUST NOT vary with parser execution order.

---

86. Deterministic AST

The frontend AST MUST preserve deterministic structure.

The AST MUST NOT depend on:

- hash-map traversal;
- memory addresses;
- parser worker order;
- target availability.

Collections whose order is semantically meaningful MUST preserve order.

Collections whose order is semantically irrelevant SHOULD have canonical ordering when serialized or compared.

---

87. Deterministic Semantic Model

The canonical semantic model MUST represent all information necessary to determine deterministic behavior.

It MUST preserve:

- operation identity;
- dependencies;
- effects;
- resource requirements;
- capability requirements;
- explicit ordering;
- explicit nondeterminism;
- random sources;
- quantum measurement;
- concurrency semantics;
- ownership;
- source provenance.

Semantic information MUST NOT be dropped merely because a backend does not currently use it.

---

88. Deterministic IR

The canonical IR MUST preserve deterministic semantic requirements.

For quantum:

source
  ↓
frontend AST
  ↓
semantic quantum model
  ↓
quantum::ir

There MUST NOT be a second quantum IR invented merely for deterministic scheduling or optimization.

For classical and HDL domains, the corresponding canonical IR boundaries MUST be respected.

---

89. IR Verification

IR verification MUST verify deterministic invariants where applicable.

Examples:

- dependency correctness;
- effect ordering;
- resource correctness;
- type correctness;
- quantum measurement semantics;
- ownership;
- synchronization;
- deterministic reduction contracts.

A verified IR MUST NOT contain an unresolved semantic ambiguity that later phases are expected to guess.

---

90. Runtime Determinism

The runtime MUST preserve source-level determinism.

The runtime MAY use:

- worker pools;
- asynchronous execution;
- task stealing;
- distributed execution;
- accelerators;
- batching;
- caching.

These are implementation strategies.

They MUST NOT alter deterministic observable results.

---

91. Runtime Scheduling and Reproducibility

A runtime SHOULD distinguish:

semantic determinism

from:

execution-plan reproducibility

Two valid executions may have different internal schedules while producing the same result.

If exact execution-plan reproducibility is requested, scheduling MUST be canonicalized accordingly.

---

92. Checkpoint and Recovery

Checkpoint/recovery MUST preserve deterministic semantics.

If a deterministic computation is interrupted and resumed, the result SHOULD be equivalent to uninterrupted execution.

Recovery MUST NOT accidentally:

- duplicate effects;
- lose effects;
- reorder effects;
- duplicate random values;
- change random streams;
- duplicate quantum operations.

---

93. Cancellation

Cancellation is an effect and MUST have defined semantics.

A cancelled computation MUST have a deterministic state transition where the language requires deterministic cancellation.

Cancellation races MUST be explicitly modeled if observable.

---

94. Time

Time is an external input.

The language MUST distinguish:

logical time

from:

physical wall-clock time

A deterministic computation depending on physical time is not deterministic unless time is explicitly provided as a semantic input.

For reproducible simulation, logical clocks SHOULD be preferred.

---

95. Timers

Timers MAY introduce external nondeterminism.

A deterministic simulation mode SHOULD use a logical time model.

A real-time program MAY intentionally depend on physical time, but this MUST be represented as an external/time effect.

---

96. Distributed Logical Time

Distributed systems SHOULD use logical time where semantic ordering must be deterministic.

Possible mechanisms include:

- sequence numbers;
- Lamport-style logical clocks;
- vector/causal clocks;
- deterministic event numbering;
- consensus ordering.

The exact implementation belongs to the distributed subsystem.

---

97. Event Systems

Event-driven programs MUST distinguish:

event identity

from:

arrival timing

If arrival order is semantically meaningful, it MUST be modeled.

If events are independent, their physical arrival order SHOULD NOT change the result of a deterministic computation.

---

98. External Services

External services are semantic inputs when their responses affect program behavior.

A deterministic test environment SHOULD replace external services with deterministic fixtures or recorded inputs.

Production execution MAY use live services, but such behavior MUST be classified as externally dependent.

---

99. Reproducibility Profiles

Zamani SHOULD define reproducibility profiles.

A profile MAY specify:

semantic
numeric
random
compiler
artifact
execution
distributed
quantum-simulation

Each profile MUST define exactly what is guaranteed.

The implementation MUST NOT advertise stronger reproducibility than the selected profile provides.

---

100. Deterministic Testing

Every production feature that claims deterministic semantics MUST have deterministic tests.

Tests MUST include:

- repeated execution;
- different compiler worker counts;
- different scheduler configurations;
- different resource scales;
- different valid target realizations where available;
- serialization round trips;
- optimization on/off;
- debug/release-equivalent semantic modes;
- deterministic random seeds where relevant.

---

101. Negative Determinism Tests

The test suite MUST verify that invalid deterministic assumptions are rejected.

Examples:

- hidden race;
- ambiguous ordering;
- unsupported reproducibility guarantee;
- unavailable deterministic reduction;
- conflicting effect order;
- impossible deterministic target requirement;
- invalid random-source declaration;
- unsupported exact reproducibility request.

---

102. Boundary Tests

Boundary tests MUST include:

- zero-element collections where valid;
- one-element computations;
- empty effect sets;
- one task;
- many tasks;
- one worker;
- many workers;
- one node;
- many nodes;
- one qubit;
- many qubits;
- one measurement;
- many measurements;
- empty maps;
- large maps;
- very large numeric values where semantically representable;
- deeply nested dependency graphs;
- large module graphs.

No boundary test may accidentally establish a universal maximum.

---

103. Scalability Tests

Scalability tests MUST verify that determinism is preserved as scale changes.

The test matrix SHOULD vary:

problem size
worker count
memory availability
device count
node count
parallelism
data partition count
quantum resource count
circuit size
graph size

The semantic result MUST remain equivalent whenever all required resources are available.

---

104. Cross-Target Determinism Tests

Where multiple backends implement the same semantic domain, tests SHOULD compare:

target A
target B
target C

against the canonical semantic result.

The comparison MUST use the appropriate semantic equivalence relation.

Bit-for-bit equality MUST NOT be demanded when the language only guarantees semantic equivalence.

---

105. Differential Testing

The compiler SHOULD support differential testing between:

- interpreter;
- reference runtime;
- optimized compiler;
- simulator;
- hardware backend;
- alternate backend.

All implementations MUST agree within the declared semantic equivalence contract.

---

106. Property-Based Determinism Testing

Property-based tests SHOULD verify invariants such as:

compile(P) repeatedly
    => equivalent result

and:

execute(P, resources=A)
    ≡
execute(P, resources=B)

when both resources satisfy the semantic requirements.

For random programs:

seed + algorithm + inputs

SHOULD determine reproducible results where the selected profile requires it.

---

107. Fuzzing

Parser and semantic fuzzing MUST preserve determinism.

The same fuzz input MUST produce:

- the same parse classification;
- the same diagnostic classification;
- the same accepted/rejected result;
- the same canonicalized semantic result,

subject to explicitly modeled randomness.

---

108. Deterministic Grammar Validation

Grammar validation MUST verify:

- no ambiguous production with unspecified resolution;
- deterministic precedence;
- deterministic associativity;
- stable tokenization;
- no duplicate semantic authority;
- stable source spans;
- no hidden target-dependent parsing;
- no resource-limit grammar rules.

---

109. Deterministic Feature Manifests

When feature manifests are used, each manifest SHOULD include:

determinism:
  semantic_class: deterministic
  nondeterminism_sources: []
  reproducibility_profile: semantic
  ordering_contract: canonical
  random_inputs: []
  external_inputs: []
  concurrency_contract: deterministic
  distributed_contract: deterministic
  quantum_contract: deterministic_or_probabilistic

The exact manifest schema belongs to the feature-manifest system.

A feature MUST NOT claim deterministic behavior without defining its sources of possible nondeterminism.

---

110. Feature Completion Contract

A determinism-sensitive feature is complete only when all of the following exist:

✓ lexical contract
✓ syntax contract
✓ AST contract
✓ semantic contract
✓ effect contract
✓ ordering contract
✓ nondeterminism classification
✓ randomness classification
✓ resource interaction
✓ capability interaction
✓ concurrency interaction
✓ distributed interaction
✓ quantum interaction where applicable
✓ canonical IR mapping
✓ compiler integration
✓ optimizer integration
✓ scheduler integration where applicable
✓ runtime integration
✓ diagnostics
✓ serialization behavior
✓ reproducibility behavior
✓ positive tests
✓ negative tests
✓ boundary tests
✓ scalability tests
✓ determinism tests
✓ compatibility tests
✓ hard-coding audit

A parser rule alone is never sufficient.

---

111. Determinism and Compatibility

Determinism is part of language compatibility.

A compatible release MUST NOT silently change a deterministic construct into:

- nondeterministic;
- implementation-defined;
- unspecified;

behavior.

Likewise, a construct explicitly classified as nondeterministic MUST NOT silently become deterministic if programs may depend on its nondeterministic semantics.

Changes to ordering, randomness, floating-point behavior, or concurrency semantics MUST follow the compatibility process in:

grammar/spec/compatibility.md

---

112. Versioning Determinism Semantics

Changes to:

- operator evaluation order;
- collection ordering;
- floating-point semantics;
- random algorithms;
- random seed interpretation;
- concurrency ordering;
- atomic ordering;
- quantum sampling semantics;
- distributed conflict resolution;

MUST be versioned when they can alter observable behavior.

A new compiler MUST NOT silently apply incompatible determinism rules to an older language version.

---

113. Implementation-Defined Behavior

Implementation-defined behavior MUST be documented.

Examples may include:

- default numeric precision;
- backend-specific optimization;
- scheduling strategy;
- target-specific timing;
- non-semantic metadata.

Implementation-defined behavior MUST NOT be confused with nondeterminism.

If the implementation choice can vary between executions and become observable, it MUST be classified appropriately.

---

114. Unspecified Behavior

Unspecified behavior is permitted only where the semantic contract explicitly permits multiple equivalent choices.

For example:

two independent pure computations

may execute in either order.

However, unspecified behavior MUST NOT be used as an excuse for:

- race conditions;
- lost effects;
- invalid ownership;
- undefined quantum behavior;
- arbitrary diagnostic ordering.

---

115. Undefined Behavior

Zamani SHOULD minimize undefined behavior.

When undefined behavior exists for unavoidable low-level reasons, safe portable Zamani constructs MUST NOT depend on it.

Production semantic analysis SHOULD reject or isolate constructs whose behavior cannot be determined safely.

The use of Rust "unsafe" MUST NOT be required to implement the deterministic language semantics.

---

116. Safe Rust Requirement

The reference implementation MUST use safe Rust.

Production source MUST NOT use:

unsafe

or require unsafe FFI assumptions for ordinary language semantics.

This includes deterministic:

- parsing;
- AST construction;
- semantic analysis;
- effect analysis;
- resource analysis;
- IR construction;
- IR verification;
- compiler transformations;
- diagnostic generation.

Any external unsafe implementation boundary MUST be isolated behind a separately specified integration mechanism and MUST NOT become a requirement of the language semantics.

---

117. Deterministic Resource Analysis

Resource analysis MUST produce deterministic results.

Given identical:

program
resource requirements
target capability description
compiler configuration

resource feasibility analysis MUST produce the same classification.

Possible results include:

SATISFIABLE
UNSATISFIABLE
UNKNOWN
REQUIRES_RUNTIME_NEGOTIATION
TARGET_DEPENDENT

The classification MUST be stable.

---

118. Unknown Resource State

An unknown resource state MUST NOT be silently interpreted as:

available

or:

unavailable

unless the relevant policy explicitly defines that interpretation.

The compiler/runtime MUST preserve uncertainty where necessary.

---

119. Deterministic Capability Resolution

Capability matching MUST be deterministic.

If multiple capabilities satisfy the same abstract requirement, selection MAY be implementation-defined.

For reproducible realization plans, selection SHOULD use canonical ordering.

Capability names MUST NOT depend on physical device enumeration order.

---

120. Deterministic Target Discovery

Target discovery MUST NOT alter source semantics.

If several valid targets are discovered:

target A
target B
target C

the program MUST retain the same semantic meaning.

Target selection is an implementation decision unless the source explicitly constrains it.

---

121. Deterministic Adaptation

A compiler MAY adapt deterministic source to available resources.

Examples:

serial execution
parallel execution
distributed execution
accelerated execution
quantum simulation
hardware quantum execution

All adaptations MUST preserve the source semantic contract.

---

122. Deterministic Failure

If a deterministic computation cannot be executed because resources are unavailable, the failure itself MUST follow a defined contract.

It MUST NOT silently produce a different successful result.

Examples of valid behavior:

resource unavailable

capability unavailable

compilation resource exhausted

runtime resource exhausted

The distinction between semantic failure and resource failure MUST remain observable where specified.

---

123. Retry Semantics

Retries can introduce nondeterminism.

A retry mechanism MUST define:

- retry count or policy;
- retry conditions;
- idempotency;
- effect duplication rules;
- randomization;
- backoff;
- cancellation;
- final failure behavior.

A retry MUST NOT duplicate a non-idempotent effect without explicit semantics.

---

124. Resilience Integration

Resilience may retry, recover, migrate, or replace an implementation.

The resilience layer MUST preserve deterministic semantics where the source requires them.

For example:

logical operation
    ↓
failed realization
    ↓
recovery
    ↓
alternate realization

is valid if the observable semantic result remains valid.

Resilience MUST NOT become a hidden source of semantic nondeterminism.

---

125. Deterministic Checkpoint Identity

Checkpoint identifiers MUST be stable where deterministic recovery is required.

They MUST NOT depend on:

- memory address;
- process ID;
- random temporary names;
- worker ID.

Checkpoint content MUST represent semantic state rather than accidental implementation state.

---

126. Deterministic Provenance

Provenance metadata MUST be reproducible.

It SHOULD identify:

- source version;
- language version;
- compiler version;
- dependency versions;
- feature versions;
- semantic configuration;
- target information where relevant;
- transformation chain.

Non-semantic host-specific metadata SHOULD be excluded from canonical provenance.

---

127. Deterministic Caching

Caching MUST NOT change semantics.

A cache hit and cache miss MUST produce equivalent observable behavior.

Cache keys MUST identify semantic inputs.

Cache implementation MAY vary.

A stale cache MUST NOT silently provide a result corresponding to incompatible semantics.

---

128. Deterministic Incremental Compilation

Incremental compilation MUST produce the same semantic result as clean compilation.

Given:

source state S

then:

clean compile(S)

and:

incremental compile(S)

MUST be semantically equivalent.

Dependency invalidation MUST be deterministic.

---

129. Deterministic Module Graphs

Module discovery MUST not depend on filesystem traversal order.

Dependency resolution MUST have deterministic conflict handling.

If multiple versions satisfy a requirement, the language/package system MUST define a canonical selection rule or reject the ambiguity.

---

130. Deterministic Macro Expansion

Macro expansion MUST be deterministic for deterministic input.

Macro systems MUST NOT depend on:

- hash ordering;
- process IDs;
- current time;
- random IDs;
- filesystem ordering;

unless those are explicitly exposed as macro inputs.

Macro hygiene MUST preserve deterministic identity.

---

131. Deterministic Metaprogramming

Compile-time reflection and code generation MUST be deterministic unless explicitly classified otherwise.

Reflection over collections, declarations, modules, types, or attributes MUST have a defined order.

The compiler MUST NOT expose arbitrary internal hash ordering through metaprogramming.

---

132. Deterministic Dialects

Dialect loading MUST be deterministic.

Dialect resolution MUST use:

- explicit namespace;
- version;
- compatibility rules;
- deterministic dependency resolution.

A dialect MUST NOT silently override another dialect's semantics based on load order.

---

133. Deterministic Interoperability

Foreign formats MUST have deterministic conversion rules.

For example:

OpenQASM
QIR
HDL
C
C++
Python
WASM

may be interoperability formats.

Conversion into Zamani MUST produce the same semantic representation for the same declared input.

External formats MUST NOT become a second Zamani semantic authority.

---

134. Deterministic Serialization of Quantum Programs

Quantum operations, operands, parameters, modifiers, measurements, and metadata MUST serialize deterministically where canonical serialization is requested.

The serialization MUST preserve:

- operation identity;
- operand identity;
- parameter expressions;
- control structure;
- measurement semantics;
- effects;
- source provenance where required.

---

135. Deterministic HDL Serialization

HDL semantic serialization MUST preserve:

- signal identity;
- module identity;
- port identity;
- dependency relationships;
- timing semantics;
- clock semantics;
- reset semantics.

Ordering used only for serialization MAY be canonicalized.

---

136. Deterministic Distributed Serialization

Distributed messages MUST have canonical serialization where deterministic hashing, signatures, replay, or consensus depends on serialized form.

Equivalent semantic messages MUST NOT produce multiple canonical byte representations.

---

137. Deterministic Security Metadata

Security-sensitive metadata MUST NOT be weakened for reproducibility.

For example:

- cryptographic randomness;
- secure nonces;
- key material;

MUST retain their required security semantics.

Reproducibility requirements MUST never silently reduce security guarantees.

---

138. Deterministic Replay

The runtime SHOULD support replay where the program's effects permit replay.

A replay record SHOULD identify:

program identity
language version
compiler/runtime version
inputs
semantic configuration
random seeds where applicable
external observations
resource/capability context where semantically relevant
nondeterministic events
quantum measurement records where simulation/replay semantics permit

Secrets MUST NOT be recorded unless explicitly authorized by the security contract.

Replay MUST preserve semantic meaning rather than requiring identical physical execution.

---

139. Replay and Quantum Execution

Physical quantum executions cannot generally be replayed by reproducing the same random measurement outcomes without changing the meaning of the original physical experiment.

Replay systems MUST distinguish:

re-execution

from:

recorded-outcome replay

A simulator MAY support exact seeded replay.

A physical QPU MAY support recorded measurement replay as a testing mechanism, but it MUST NOT be presented as identical physical re-execution.

---

140. Deterministic Observability

Tracing and profiling MUST NOT alter program semantics.

Instrumentation MAY change:

- execution time;
- resource usage;
- scheduling;

but MUST NOT change semantic results.

If timing itself is observable by the program, instrumentation becomes semantically relevant and MUST be treated as such.

---

141. Deterministic Logging

Logs intended for reproducibility MUST have stable:

- event identifiers;
- ordering;
- serialization;
- timestamps where logical time is used.

Physical timestamps MUST be classified as environmental data.

---

142. Deterministic Profiling

Profiling measurements are inherently environment-dependent.

They MUST NOT be treated as deterministic semantic outputs unless the language explicitly defines them as such.

Compiler optimizations MUST NOT depend on unstable profiling measurements when reproducible compilation is requested unless the profile itself is a declared input.

---

143. Deterministic Benchmarking

Benchmarks MUST distinguish:

functional correctness

from:

performance measurement

Performance is generally target/environment dependent.

A benchmark result MUST NOT be used as semantic input unless explicitly declared.

---

144. Deterministic Verification

Formal verification, static analysis, and type/effect checking MUST be deterministic.

Given identical semantic inputs, the verifier MUST produce the same:

- acceptance result;
- proof obligations;
- diagnostic classification.

Parallel verification is permitted, but result ordering MUST be canonicalized.

---

145. Proof and Certificate Determinism

Where verification produces proof/certificate artifacts, canonical serialization SHOULD be provided.

The certificate MUST represent the same semantic claim independent of internal solver traversal order.

Solver randomness MUST be controlled when reproducible certificates are required.

---

146. SAT/SMT and Constraint Solver Integration

Constraint solvers MAY use randomized algorithms internally.

If solver results determine language semantics, the compiler MUST ensure that:

- equivalent results produce equivalent semantic decisions;
- unsupported ambiguity is reported;
- solver randomness cannot silently change a valid program's meaning.

For reproducible builds, solver seeds and versions SHOULD be controlled.

---

147. Deterministic Resource Negotiation

Resource negotiation MAY occur at runtime.

A negotiation result MUST distinguish:

required
preferred
optional

resources.

If several realizations satisfy the same semantic requirements, their selection MAY differ unless reproducibility requires canonical selection.

---

148. Deterministic Preferences

Preferences MUST NOT change semantic correctness.

For example:

prefer accelerator("tensor")

may influence realization.

It MUST NOT change the mathematical meaning of the program.

If a preference changes semantics, it is not merely a preference and MUST be represented as a requirement or explicit semantic mode.

---

149. Deterministic Hints

Hints MUST be non-semantic unless explicitly declared otherwise.

A hint MAY improve:

- locality;
- scheduling;
- optimization;
- routing;
- memory placement.

A compiler MAY ignore a hint.

Ignoring a hint MUST NOT change program correctness.

---

150. Deterministic Target-Specific Constraints

Target-specific constraints MAY be explicit.

For example:

requires capability("quantum.mid_circuit_measurement")

is portable semantic intent.

A concrete physical mapping such as:

logical q0 -> physical qubit 17

is target-specific.

The latter MUST NOT silently become a universal language semantic.

---

151. Deterministic Physical Mapping

When explicit physical mapping is permitted, it MUST be deterministic within the declared target context.

The mapping MUST preserve:

- logical identity;
- operation semantics;
- connectivity constraints;
- timing constraints;
- resource requirements.

The mapping belongs downstream of the canonical quantum semantic boundary.

---

152. Deterministic Scheduling of Quantum Operations

Quantum scheduling MUST preserve:

- dependency order;
- measurement semantics;
- classical feed-forward;
- required barriers;
- timing constraints;
- resource conflicts.

The scheduler MAY choose different schedules across targets.

Schedule differences MUST NOT change logical quantum semantics.

---

153. Deterministic Classical Feed-Forward

Hybrid quantum/classical programs MUST preserve dependencies such as:

quantum measurement
        ↓
classical result
        ↓
classical decision
        ↓
quantum operation

A compiler MUST NOT reorder the dependent quantum operation before the measurement result is semantically available.

---

154. Deterministic Hybrid Execution

Hybrid computation MAY involve:

- CPU;
- GPU;
- QPU;
- FPGA;
- distributed systems.

The semantic dependency graph MUST remain deterministic where the program requires it.

Physical transport latency is an implementation concern unless timing is explicitly semantic.

---

155. Deterministic HDL Timing

If HDL timing is semantic, the timing model MUST be explicit.

If timing is only an implementation constraint, it MUST remain downstream.

The compiler MUST NOT infer semantic timing merely from a target's physical clock frequency.

---

156. Deterministic Accelerator Selection

Accelerator selection MUST be treated as a realization decision unless explicitly required.

If multiple accelerators satisfy a capability:

capability("tensor.compute")

their selection MUST NOT change deterministic program semantics.

---

157. Deterministic Distributed Reduction

Distributed reductions MUST define deterministic aggregation where required.

For floating-point values, deterministic reduction SHOULD define:

- partition order;
- combination order;
- precision;
- rounding;
- overflow behavior.

A tree reduction whose shape depends on worker count MUST NOT claim bitwise reproducibility unless its semantics guarantee equivalent results.

---

158. Worker-Count Independence

A central POCO-REAF requirement is:

same program
+
same semantic inputs
+
different valid worker count

MUST preserve semantic behavior.

For example:

1 worker
8 workers
64 workers
1024 workers

MAY produce different performance.

They MUST NOT produce different deterministic results merely because the worker count changed.

---

159. Node-Count Independence

Likewise:

1 node
2 nodes
N nodes

MAY use different implementations.

If all declared semantic requirements are satisfied, deterministic observable behavior MUST remain equivalent.

---

160. Device-Count Independence

The same rule applies to:

- GPUs;
- FPGAs;
- QPUs;
- accelerators;
- storage devices;
- network paths.

Physical resource count is not semantic identity.

---

161. Determinism and Infinite Scaling

Zamani's determinism model MUST NOT depend on finite machine-size assumptions.

The language MUST support:

tiny computation
→
large computation
→
massively parallel computation
→
distributed computation
→
heterogeneous computation

without redefining deterministic semantics.

"Infinity" means:

no artificial language-level ceiling

not:

infinite physical resources

---

162. Hard-Coding Audit

The determinism specification and its implementation MUST reject or flag artificial semantic limits such as:

MAX_THREADS
MAX_WORKERS
MAX_TASKS
MAX_NODES
MAX_QUBITS
MAX_SHOTS
MAX_OPERATIONS
MAX_RANDOM_STREAMS
MAX_GRAPH_SIZE
MAX_COLLECTION_SIZE

The same rule applies to disguised forms such as:

worker0
worker1
...
worker127

when those names are used to define a universal language ceiling.

Program-level constants remain valid.

---

163. Program Constants

This is valid:

let workers = 8;

if "8" is program data or a semantic request.

This is not a universal language rule:

compiler supports at most 8 workers

The distinction is:

program semantics

versus:

implementation ceiling

---

164. Deterministic Infinite Data Structures

Logical infinite streams MAY be supported if the language defines lazy/stream semantics.

The implementation MUST NOT attempt to materialize an infinite structure merely because it exists semantically.

Determinism is defined over the requested finite observation of the stream.

---

165. Lazy Evaluation

Lazy computation MUST preserve deterministic semantics.

A lazy value MAY be evaluated:

- once;
- multiple times;
- on demand;
- in parallel;

only when the selected strategy preserves observable behavior, ownership, effects, and resource semantics.

---

166. Memoization

Memoization MAY be used for pure computations.

A memoized computation MUST be semantically equivalent to recomputation.

Memoization MUST NOT suppress an observable effect.

---

167. Speculative Execution

Speculative execution MAY be used.

Speculation MUST NOT cause observable effects to occur incorrectly.

Effects that cannot be safely speculated MUST remain ordered or be protected by a semantic mechanism.

---

168. Transactional Semantics

Transactions MAY provide deterministic commit behavior.

The transaction model MUST define:

- isolation;
- conflict detection;
- ordering;
- rollback;
- retry;
- visibility.

Retries MUST preserve effect semantics.

---

169. Deterministic Conflict Resolution

If concurrent operations conflict, the language MUST define one of:

- deterministic resolution;
- explicit nondeterministic resolution;
- conflict failure;
- programmer-selected policy.

The runtime MUST NOT silently choose an arbitrary result where deterministic semantics are required.

---

170. Deterministic Ownership

Ownership transfer MUST have deterministic semantic consequences.

An object MUST NOT simultaneously have two exclusive owners merely because concurrent execution makes that state convenient.

The memory/type system remains authoritative for ownership.

---

171. Deterministic Garbage Collection / Resource Reclamation

If resource reclamation is observable, its semantics MUST be defined.

If reclamation is not observable, implementations MAY reclaim resources at different times.

Program semantics MUST NOT depend on incidental reclamation timing.

---

172. Deterministic Finalization

Finalization of resources with observable effects MUST have defined ordering.

Finalizers MUST NOT be used as an implicit concurrency mechanism.

If exact finalization order matters, the language MUST provide explicit lifetime/ordering semantics.

---

173. Deterministic External State

External state is an input.

A deterministic program that reads external state MUST either:

- treat the external state as an explicit input;
- operate in a controlled environment;
- declare that execution is externally dependent.

The compiler MUST NOT claim full reproducibility while silently depending on uncontrolled external state.

---

174. Deterministic Configuration

Configuration MUST be explicit when it affects semantics.

Configuration precedence MUST be deterministic.

For example:

source configuration
→
module configuration
→
build configuration
→
runtime configuration

must have a defined precedence model.

---

175. Configuration and Portability

Configuration MUST NOT silently transform portable source into target-specific semantics.

Target selection SHOULD remain downstream.

---

176. Deterministic Environment Capture

Reproducible execution SHOULD provide an environment manifest describing relevant semantic inputs.

It MAY include:

- language version;
- compiler/runtime version;
- dependencies;
- declared capabilities;
- declared resources;
- random seeds;
- external input identities;
- target identity where target-dependent semantics are explicit.

---

177. Deterministic Package Resolution

Package/dependency resolution MUST be deterministic.

Given the same package registry state and declared requirements, the resolver MUST select the same dependency graph.

If registry state can change, lockfiles or equivalent reproducibility mechanisms SHOULD be used.

---

178. Dependency Graph Determinism

Dependency graph traversal MUST not depend on:

- filesystem order;
- network response order;
- hash order.

Canonical graph ordering SHOULD be used for:

- diagnostics;
- lockfiles;
- manifests;
- generated documentation;
- cache keys.

---

179. Deterministic Documentation Generation

Generated grammar/reference documentation MUST use canonical ordering.

Documentation generation MUST NOT randomly reorder:

- declarations;
- grammar rules;
- features;
- diagnostics;
- capabilities.

---

180. Deterministic Tooling

IDE, formatter, linter, language server, and compiler tooling SHOULD use the same canonical semantic contracts.

Tooling MUST NOT invent alternate semantic interpretations merely because it operates incrementally.

---

181. Formatter Determinism

The formatter MUST produce the same formatted result for the same semantic/source input and formatting configuration.

Formatting MUST NOT alter semantics.

---

182. Linter Determinism

Lint diagnostics MUST be deterministic.

Lint rule execution MAY be parallel.

Diagnostic collection MUST be canonically ordered.

---

183. Language Server Determinism

Language-server responses for identical document state SHOULD be deterministic.

Incremental processing MAY optimize performance but MUST preserve semantic equivalence with a clean analysis.

---

184. Deterministic Version Control Integration

Generated artifacts intended for version control SHOULD avoid nondeterministic metadata.

Examples:

- timestamps;
- random IDs;
- machine-specific paths;
- nondeterministic ordering.

---

185. Deterministic Code Generation

Generated source MUST use canonical:

- declaration ordering;
- imports;
- identifiers;
- formatting;
- metadata;
- serialization.

Generated code MUST be semantically equivalent to the canonical source representation.

---

186. Deterministic Lowering

Lowering from semantic model to IR MUST preserve deterministic semantics.

For every source construct:

source
 ↓
AST
 ↓
semantic model
 ↓
IR

the lowering MUST preserve all deterministic constraints.

It MUST NOT:

- drop ordering;
- drop effects;
- drop random-source identity;
- drop synchronization;
- drop measurement semantics;
- drop resource requirements.

---

187. No Silent Semantic Loss

This specification inherits the repository's compatibility rule:

parsed information
    ↓
AST
    ↓
semantic model
    ↓
IR

must not silently lose deterministic information.

Examples of prohibited loss:

- dropped ordering modifier;
- dropped effect;
- dropped synchronization;
- dropped random seed;
- dropped reduction order;
- dropped quantum measurement semantics;
- dropped deterministic resource constraint.

---

188. Deterministic Canonicalization

Canonicalization MAY simplify representation.

It MUST NOT change semantic identity.

Examples:

effects { A, B }

and:

effects { B, A }

may canonicalize to the same set representation if effects are unordered.

But:

A(); B();

and:

B(); A();

MUST NOT canonicalize identically if the order is semantically observable.

---

189. Semantic Equivalence

Every optimization, canonicalization, parallelization, routing, scheduling, or lowering transformation SHOULD be describable as:

source semantics
    ≡
transformed semantics

under the applicable equivalence relation.

The equivalence relation MUST be stronger when the selected reproducibility profile requires stronger guarantees.

---

190. Determinism Levels for the Language

Zamani SHOULD recognize at least:

D0 — unspecified implementation choice
D1 — semantic determinism
D2 — reproducible values
D3 — bitwise reproducibility
D4 — reproducible build/execution artifact

A feature MUST declare which level it supports.

A D1 feature MUST NOT be advertised as D3.

---

191. Semantic Determinism Is the Default Goal

Unless a feature explicitly declares:

random
nondeterministic
external
quantum-probabilistic
implementation-defined

the language SHOULD treat ordinary semantic computation as deterministic.

---

192. Explicit Nondeterminism API

Any language feature intentionally introducing nondeterminism SHOULD be explicit.

Conceptually:

nondeterministic { ... }

or an equivalent effect/capability mechanism.

The exact syntax belongs to the appropriate grammar file.

The semantics MUST be defined here.

---

193. Nondeterminism Must Not Leak

A nondeterministic operation MUST NOT make unrelated surrounding computations nondeterministic.

The effect system SHOULD track the propagation boundary.

For example:

random value
    ↓
computation depending on random value

is nondeterministic.

Independent pure computation elsewhere need not become nondeterministic.

---

194. Deterministic Isolation

Programs SHOULD be able to isolate nondeterministic effects.

This allows:

deterministic core
+
explicit nondeterministic boundary

Such isolation improves:

- testing;
- reproducibility;
- formal verification;
- debugging;
- optimization;
- portability.

---

195. Deterministic Replay Boundary

Replay systems SHOULD capture every nondeterministic input crossing into deterministic computation.

This may include:

- random values;
- external events;
- network messages;
- environment reads;
- clock observations;
- quantum measurement results where replay semantics permit.

---

196. Deterministic Debugging

Debug mode MUST NOT silently change semantics.

Debug instrumentation MAY alter performance.

It MUST NOT introduce or remove observable behavior unless the language explicitly defines debugging effects.

---

197. Optimization-Level Independence

Changing optimization level MUST NOT change semantic results.

For example:

-O0
-O1
-O2
-O3

MAY produce different artifacts.

They MUST preserve the same semantic result for deterministic programs.

---

198. Backend Independence

Changing backend MUST NOT alter source semantics when both backends satisfy the same semantic contract.

For example:

CPU backend
GPU backend
FPGA backend
QPU backend

may produce different performance and internal representations.

They MUST preserve the applicable semantic contract.

---

199. Simulation Independence

A simulator MAY use approximations only when the selected semantic contract permits approximation.

If exact semantics are required, approximation MUST NOT be silently substituted.

The simulator MUST document its reproducibility level.

---

200. Deterministic Approximation

Approximation algorithms MUST expose their error/tolerance semantics where approximation affects observable correctness.

Repeated deterministic approximation with the same configuration SHOULD produce reproducible results.

---

201. Error Tolerance

Numerical equivalence MAY use a tolerance where the type/domain contract defines approximate equality.

The tolerance MUST itself be explicit and deterministic.

A backend MUST NOT silently change the tolerance based on hardware.

---

202. Deterministic Precision Adaptation

Precision adaptation MAY occur when explicitly permitted.

For example:

prefer lower precision

is different from:

requires exact precision

The implementation MUST NOT silently lower precision when the semantic contract requires exactness.

---

203. Deterministic Energy/Performance Policies

Performance and energy are normally implementation properties.

A program MAY express:

require latency <= budget
prefer energy <= budget

The realization may vary.

If a policy affects semantic correctness, it MUST be classified as a constraint rather than merely a hint.

---

204. Deterministic Power/Thermal Adaptation

Hardware may throttle execution.

If throttling is not semantically observable, it MUST NOT alter deterministic results.

If timing is semantically observable, timing belongs to the relevant execution contract.

---

205. Deterministic Fault Handling

Fault handling MUST distinguish:

recoverable implementation failure

from:

semantic program failure

A recovered implementation failure MUST preserve source semantics.

A semantic failure MUST be reported according to the defined failure contract.

---

206. Deterministic Resilience Decisions

Resilience may choose among multiple recovery strategies.

If those strategies are semantically equivalent, implementation choice is permitted.

If strategies produce different observable results, the choice MUST be explicitly specified.

---

207. Deterministic Quarantine

When hardware is degraded or quarantined, the resilience system MAY select another resource.

The source program MUST retain its semantic identity.

Physical resource identity MUST remain downstream.

---

208. Deterministic Calibration Integration

Calibration data is external target state.

Calibration MAY affect feasibility and performance.

It MUST NOT silently change source semantics.

If calibration changes whether an operation is executable, the implementation MUST produce a defined capability/resource result.

---

209. Deterministic HAL Integration

HAL reports target capabilities/state.

HAL decisions MUST NOT redefine language semantics.

Given the same HAL state and policy, capability analysis SHOULD be deterministic.

---

210. Deterministic Runtime Negotiation

Runtime negotiation MAY select different resources across executions if the program allows it.

For deterministic execution-plan reproducibility, negotiation SHOULD use canonical selection.

Semantic results MUST remain equivalent.

---

211. Security and Reproducibility Conflict

Security MUST take precedence over optional reproducibility where the two conflict.

For example:

secure random key

MUST NOT be replaced with:

fixed random seed

merely because deterministic replay is convenient.

The program MUST explicitly choose a weaker/reproducible mode if such a mode is semantically permitted.

---

212. Privacy

Replay, logging, provenance, and deterministic diagnostics MUST respect privacy/security contracts.

Sensitive data MUST NOT be recorded merely because deterministic replay would be easier.

---

213. No Secret-Dependent Diagnostic Ordering

Diagnostics MUST NOT expose secrets merely to establish deterministic ordering.

Stable identifiers or redacted metadata SHOULD be used.

---

214. Determinism and Macros

Macros MUST preserve deterministic semantics.

Macro expansion order MUST be defined.

If macros can inspect declarations, the declaration traversal order MUST be canonical.

---

215. Determinism and Metaprogramming

Reflection MUST expose canonical semantic order where order is not semantically meaningful.

Metaprograms MUST NOT depend on internal hash ordering.

---

216. Determinism and Dialects

A dialect MUST declare whether its constructs are:

- deterministic;
- random;
- nondeterministic;
- externally dependent;
- implementation-defined.

A dialect MUST NOT weaken the determinism guarantees of core Zamani without explicit language-version/feature semantics.

---

217. Determinism and Compatibility Adapters

Compatibility adapters MUST preserve deterministic behavior.

An adapter MAY translate old syntax to new semantics.

It MUST NOT:

- randomly select interpretations;
- silently reorder effects;
- discard deterministic constraints;
- discard random-source information.

---

218. Deterministic Migration

A migration tool MUST produce the same migrated source for the same input and migration configuration.

If a migration cannot determine a unique translation, it MUST report the ambiguity.

---

219. Deterministic Generated Grammar

If "grammar/grammar.md" or other grammar references are generated, generation MUST be deterministic.

The generated output MUST NOT depend on:

- filesystem order;
- hash iteration;
- parallel generation order.

---

220. Deterministic Specification Validation

Specification validators MUST produce stable results.

Cross-reference checks MUST use canonical ordering.

Feature completeness reports MUST be reproducible.

---

221. Deterministic Conformance Matrix

The compatibility/conformance system SHOULD track:

feature
    ↓
lexical status
    ↓
grammar status
    ↓
parser status
    ↓
AST status
    ↓
semantic status
    ↓
determinism status
    ↓
IR status
    ↓
compiler status
    ↓
runtime status
    ↓
tests

A feature MUST NOT be marked production-ready merely because syntax parses.

---

222. Required Feature Manifest Fields

Every determinism-sensitive feature manifest SHOULD identify:

determinism:
  semantic_class:
  reproducibility_level:
  ordering:
  randomness:
  external_inputs:
  concurrency:
  distributed:
  quantum:
  numeric:
  compiler:
  runtime:
  diagnostics:
  serialization:
  replay:

Missing fields MUST be treated as incomplete when determinism is relevant.

---

223. Required AST Contract

Every deterministic feature MUST define:

AST representation
source span
semantic identity
ordering information
effects
resource requirements
capability requirements
randomness metadata
nondeterminism metadata

Only information genuinely relevant to the feature needs to be represented.

---

224. Required Semantic Contract

Every deterministic feature MUST define:

What is deterministic?
What is nondeterministic?
What is random?
What is externally dependent?
What is implementation-defined?
What ordering is semantic?
What ordering is canonical only?
What resources affect feasibility?
What transformations preserve meaning?

---

225. Required IR Contract

The IR mapping MUST define:

semantic operation
    ↓
IR representation
    ↓
ordering/dependency representation
    ↓
effect representation
    ↓
randomness representation
    ↓
resource/capability representation

No deterministic information may be lost.

---

226. Required Runtime Contract

Runtime integration MUST define:

- scheduling;
- concurrency;
- resource failure;
- randomness;
- external input;
- replay;
- cancellation;
- recovery;
- observability;
- deterministic result requirements.

---

227. Required Compiler Contract

Compiler integration MUST define:

- deterministic parsing;
- deterministic name resolution;
- deterministic inference;
- deterministic semantic analysis;
- deterministic IR construction;
- deterministic optimization behavior;
- reproducibility modes;
- canonical serialization;
- deterministic diagnostics.

---

228. Required Tooling Contract

Tooling MUST define deterministic behavior for:

- formatter;
- linter;
- language server;
- documentation generator;
- feature validator;
- grammar validator;
- conformance runner;
- test harness.

---

229. Required Negative Tests

Negative tests MUST verify that the implementation rejects:

- ambiguous semantic ordering;
- hidden data races;
- invalid deterministic guarantees;
- unsupported exact reproducibility;
- illegal random-source substitution;
- nondeterministic effect use where deterministic behavior is required;
- impossible resource requirements;
- unsupported deterministic target constraints.

---

230. Required Scalability Tests

Scalability tests MUST verify:

same semantics
+
different resource scale
=
equivalent deterministic result

Examples:

1 worker
8 workers
64 workers

1 node
many nodes

1 accelerator
many accelerators

small quantum register
large quantum register

where each configuration satisfies the program's semantic requirements.

---

231. Required Determinism Test Matrix

At minimum:

Dimension| Required tests
Parser| repeated parse equivalence
Lexer| repeated tokenization equivalence
AST| canonical structure
Name resolution| stable binding
Type inference| stable inference
Effects| stable effect set
Resources| stable feasibility
Compiler| repeated compilation
Optimization| optimization equivalence
Runtime| repeated deterministic execution
Parallelism| varying worker counts
Distribution| varying node counts
Randomness| fixed-seed replay
Quantum| simulator reproducibility where supported
HDL| deterministic simulation
Diagnostics| stable ordering
Serialization| canonical output
Cache| hit/miss equivalence
Incremental compilation| clean/incremental equivalence
Compatibility| version-preserving behavior

---

232. Acceptance Criteria

"grammar/spec/determinism.md" is complete only when:

- deterministic semantics are explicitly defined;
- nondeterminism is explicitly defined;
- randomness is explicitly defined;
- unspecified behavior is distinguished from nondeterminism;
- implementation-defined behavior is distinguished from nondeterminism;
- evaluation order is specified;
- concurrency integration is defined;
- distributed integration is defined;
- quantum integration is defined;
- measurement semantics are distinguished from compiler randomness;
- floating-point reproducibility is addressed;
- deterministic compilation is addressed;
- deterministic diagnostics are addressed;
- reproducible builds are addressed;
- canonical serialization is addressed;
- replay is addressed;
- resource scaling is addressed;
- POCO-REAF is preserved;
- artificial hard-coded limits are prohibited;
- Rust 1.97/1.97.1 compatibility is established;
- Rust "unsafe" is prohibited;
- AST integration is defined;
- semantic-model integration is defined;
- canonical IR integration is defined;
- "quantum::ir" remains the canonical quantum boundary;
- compiler integration is defined;
- runtime integration is defined;
- routing/scheduling/resilience integration is defined;
- ZQN/HAL integration is defined;
- interoperability is defined;
- dialect integration is defined;
- compatibility/versioning is defined;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- determinism tests exist;
- cross-target tests exist where applicable.

---

233. Integration With Existing Repository Files

The following relationships are normative.

"grammar/spec/semantics.md"

Owns general semantic meaning.

This file extends it with deterministic/nondeterministic classification.

It MUST NOT duplicate the complete semantic model.

---

"grammar/spec/effects.md"

Owns effect semantics.

This file defines how effects influence determinism.

Effect implementations MUST NOT secretly introduce nondeterminism.

---

"grammar/spec/concurrency.md"

Owns concurrency semantics.

This file defines how concurrency is classified as deterministic, nondeterministic, or externally dependent.

Concurrency remains logical; scheduling remains downstream.

---

"grammar/spec/quantum.md"

Owns quantum semantics.

This file defines the determinism distinction between:

quantum evolution

and:

measurement randomness

The canonical quantum semantic boundary remains:

quantum::ir

---

"grammar/spec/portability.md"

Owns POCO-REAF and target independence.

This file ensures determinism survives target adaptation.

---

"grammar/spec/compatibility.md"

Owns compatibility.

Determinism changes MUST follow its versioning rules.

---

"grammar/spec/diagnostics.md"

Owns the general diagnostics contract.

This file requires diagnostics to be deterministic.

---

"grammar/spec/lexical.md"

Owns lexical semantics.

Lexical processing MUST be deterministic.

---

"grammar/spec/type-system.md"

Owns types.

Type inference and type checking MUST be deterministic.

---

"grammar/Zamani.g4"

Remains the canonical ANTLR composition root.

It MUST NOT contain implementation-specific determinism logic.

---

"grammar/grammar.md"

Remains implementation-conformance documentation.

It MUST distinguish specified from implemented deterministic behavior.

---

"grammar/Zamani-Grammar.md"

Remains historical/extended design material.

It MUST NOT introduce determinism semantics without promotion through the specification process.

---

"grammar/validation/"

Must validate:

- deterministic grammar;
- deterministic diagnostics;
- hard-coding;
- ordering;
- canonicalization;
- feature completeness.

---

"grammar/tests/"

Must contain deterministic, nondeterministic, random, boundary, and scalability tests.

---

234. Final Architectural Rule

The fundamental Zamani determinism rule is:

«A deterministic Zamani program MUST have one semantic meaning regardless of how many valid computational resources are used to realize it.»

Therefore:

one machine
many machines
one CPU
many CPUs
one GPU
many GPUs
one FPGA
many FPGAs
one QPU
many QPUs
one node
many nodes
serial execution
parallel execution
local execution
distributed execution
simulation
physical execution
current hardware
future hardware

MAY change the implementation.

They MUST NOT change deterministic program meaning.

---

235. Final Determinism Model

The complete model is:

                    Zamani Source
                         │
                         ▼
                    Lexical Layer
                         │
                         ▼
                       Parser
                         │
                         ▼
                  Domain-Neutral AST
                         │
                         ▼
              Structural Validation
                         │
                         ▼
       ┌─────────────────────────────────┐
       │ Semantic Analysis                │
       │                                 │
       │ types                           │
       │ effects                         │
       │ ownership                       │
       │ resources                       │
       │ capabilities                    │
       │ ordering                        │
       │ determinism                     │
       │ randomness                      │
       │ nondeterminism                  │
       │ portability                     │
       └─────────────────────────────────┘
                         │
                         ▼
              Canonical Semantic Model
                         │
             ┌───────────┼───────────┐
             │           │           │
             ▼           ▼           ▼
        Classical    quantum::ir    HDL
             │           │           │
             └───────────┼───────────┘
                         │
                         ▼
                    Verification
                         │
                         ▼
                    Optimization
                         │
              ┌──────────┼──────────┐
              │          │          │
              ▼          ▼          ▼
           Routing   Scheduling  Resilience
              │          │          │
              └──────────┼──────────┘
                         │
                         ▼
                        ZQN
                         │
                         ▼
                        HAL
                         │
                         ▼
                Target Realization
                         │
          ┌──────────────┼──────────────┐
          │              │              │
         CPU            GPU            FPGA
          │              │              │
          ├──────────────┼──────────────┤
          │              │              │
         QPU        Distributed      Future
          │              │          Substrate
          └──────────────┼──────────────┘
                         │
                         ▼
                      Runtime

The semantic invariant is:

                  deterministic source
                         │
                         ▼
                 canonical semantics
                         │
          ┌──────────────┼──────────────┐
          │              │              │
       target A       target B       target C
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                 same semantics

The implementation may differ.

The semantic result MUST NOT.

---

236. Production Completion Statement

A production Zamani implementation conforms to this determinism specification only when:

Source
  ↓
Lexer
  ↓
Parser
  ↓
AST
  ↓
Semantic Analysis
  ↓
Determinism Analysis
  ↓
Canonical Semantic Model
  ↓
Canonical IR
  ↓
Verification
  ↓
Optimization
  ↓
Routing / Scheduling / Resilience
  ↓
ZQN / HAL
  ↓
Target
  ↓
Runtime

forms a traceable, tested, deterministic semantic chain.

The implementation MUST:

1. preserve deterministic semantics;
2. explicitly represent nondeterminism;
3. explicitly represent randomness;
4. explicitly represent external inputs;
5. prevent hidden nondeterminism;
6. canonicalize implementation-only ordering;
7. preserve semantic ordering;
8. preserve deterministic effects;
9. preserve deterministic concurrency;
10. preserve deterministic distributed computation;
11. preserve quantum semantics while acknowledging measurement probability;
12. preserve HDL timing semantics;
13. preserve numerical reproducibility guarantees;
14. preserve resource/capability semantics;
15. preserve deterministic diagnostics;
16. preserve deterministic serialization;
17. preserve deterministic compilation;
18. preserve deterministic replay where supported;
19. avoid artificial resource ceilings;
20. remain compatible with Rust 1.97 / 1.97.1;
21. use no Rust "unsafe";
22. maintain "quantum::ir" as the canonical quantum semantic boundary;
23. keep target realization downstream from portable source semantics;
24. allow scaling from tiny computations to arbitrarily large computations subject only to actual resources and declared semantic requirements;
25. support the POCO-REAF architecture.

The ultimate invariant is:

«Zamani source defines the computation's meaning. Determinism defines which aspects of that meaning must remain invariant. The compiler, runtime, scheduler, router, resilience system, QEC system, ZQN, HAL, and hardware are free to change the realization only within those semantic boundaries.»

That is the determinism foundation required for a language intended to scale from atom to everywhere while preserving:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.