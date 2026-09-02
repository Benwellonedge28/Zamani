Worked for 9s

Below is the production contract I recommend for src/quantum/zqn/SECURITY.md. I have aligned it with the ZQN architecture already established, including the existing noise/*, fault/*, probability/channel work, routing, QEC, benchmarking, calibration, runtime and canonical IR boundaries. The repository search confirms, for example, that ZQN already treats noise application, temporal/spatial/crosstalk/non-Markovian/conditional noise as separate responsibilities, while the existing QEC noise implementation has its own deterministic physical-fault machinery that should ultimately consume ZQN rather than duplicate it.

src/quantum/zqn/SECURITY.md

Zamani Quantum Noise (ZQN) — Security Architecture and Security Policy

Path: "src/quantum/zqn/SECURITY.md"
Subsystem: "crate::quantum::zqn"
Language: Rust 2021
Supported toolchain: Rust 1.97 / Rust 1.97.1
Safety requirement: "unsafe" code is forbidden
Status: Normative production-security contract
Scope: All ZQN source, serialization, simulation, characterization, calibration, target and integration boundaries

---

1. Purpose

ZQN — Zamani Quantum Noise — is the security boundary for the representation, validation, propagation, simulation, characterization and integration of physical and abstract quantum noise.

ZQN must safely handle:

- tiny quantum systems;
- large quantum systems;
- distributed quantum systems;
- arbitrarily large streamed workloads;
- user-defined noise models;
- externally supplied noise models;
- calibration data;
- characterization data;
- stochastic sampling;
- correlated noise;
- temporal noise;
- spatial noise;
- crosstalk;
- leakage;
- erasure;
- loss;
- non-Markovian noise;
- continuous-time models;
- channel representations;
- approximate models;
- exact models;
- simulation;
- hardware observations;
- fault-generation interfaces.

Security must remain correct regardless of:

- number of resources;
- number of operations;
- number of shots;
- topology;
- backend;
- quantum technology;
- representation;
- parallelism;
- distribution across machines;
- model size;
- calibration size;
- execution duration.

The security architecture MUST therefore avoid artificial semantic limits while still providing explicit resource-governance mechanisms.

---

2. Security objectives

ZQN MUST provide the following security properties.

2.1 Memory safety

All ZQN code MUST be memory safe without "unsafe".

No ZQN module may contain:

unsafe { ... }

or any equivalent unsafe construct.

ZQN MUST NOT depend on unsafe internal implementation techniques merely to improve performance.

Performance optimizations must remain within safe Rust.

The ZQN module boundary SHOULD enforce:

#![forbid(unsafe_code)]

where the surrounding crate/module structure permits the lint to be applied.

---

2.2 No undefined behavior

ZQN MUST never rely on:

- undefined behavior;
- invalid pointer assumptions;
- unchecked indexing;
- unchecked integer arithmetic where overflow can affect security;
- invalid enum discriminants;
- invalid UTF-8;
- unvalidated external memory layouts;
- unsafe FFI;
- undefined floating-point assumptions.

---

2.3 Deterministic security behavior

Security checks MUST NOT depend on:

- thread scheduling;
- hash-map iteration order;
- process ID;
- machine address;
- wall-clock time;
- operating-system randomness;
- CPU core;
- number of worker threads.

A malicious input must produce equivalent validation behavior regardless of execution ordering.

This integrates directly with "DETERMINISM.md".

Security decisions MUST be deterministic whenever the operation itself is deterministic.

---

3. Security boundary

The ZQN security boundary is:

                    UNTRUSTED
                       │
                       ▼
             ┌───────────────────┐
             │ external input    │
             │ model             │
             │ calibration       │
             │ observations      │
             │ serialized data   │
             │ configuration     │
             └─────────┬─────────┘
                       │
                       ▼
             ┌───────────────────┐
             │ ZQN validation    │
             │ + limits         │
             │ + canonicalization│
             └─────────┬─────────┘
                       │
                 VALIDATED DATA
                       │
                       ▼
             ┌───────────────────┐
             │ ZQN semantic core │
             └─────────┬─────────┘
                       │
            ┌──────────┼──────────┐
            ▼          ▼          ▼
         simulation  routing     QEC
            │          │          │
            └──────────┼──────────┘
                       ▼
                    runtime
                       │
                       ▼
                 hardware/QPU

The fundamental rule is:

«No externally supplied ZQN object may become trusted merely because it successfully deserialized.»

Deserialization establishes structural validity only.

Semantic validation MUST happen before execution.

---

4. Trust model

ZQN MUST distinguish at least these trust classes.

4.1 Trusted internal semantic objects

Objects constructed entirely from already validated ZQN APIs.

Examples:

- validated probability;
- validated channel;
- validated noise model;
- validated calibration snapshot.

These may be reused without repeating expensive validation when their immutability guarantees are intact.

---

4.2 Untrusted serialized objects

Examples:

- JSON;
- YAML;
- binary interchange formats;
- files;
- network payloads;
- cached model files;
- imported calibration data.

All must be treated as untrusted.

---

4.3 Untrusted user-defined models

A user may supply:

- parameters;
- distributions;
- correlations;
- functions through supported declarative mechanisms;
- model configuration;
- calibration values.

ZQN MUST validate them before execution.

---

4.4 Hardware observations

Hardware-produced data MUST be treated as externally sourced measurements.

Hardware output is not automatically trustworthy merely because it originated from a supported backend.

The hardware integration layer MUST validate:

- ranges;
- dimensions;
- resource identities;
- timestamps;
- calibration references;
- measurement counts;
- probability constraints;
- metadata;
- schema versions.

---

4.5 Derived data

Derived values such as:

- estimated error rates;
- characterization results;
- fitted noise models;
- uncertainty bounds;

MUST retain provenance linking them to their source observations and assumptions.

---

5. Security invariants

The following invariants are mandatory.

S-001 — No unsafe code

ZQN contains no "unsafe" Rust.

S-002 — No unchecked untrusted allocation

External dimensions MUST NOT directly determine unrestricted allocation.

S-003 — No unchecked arithmetic

Security-sensitive arithmetic MUST use checked or otherwise validated operations.

S-004 — No silent numerical coercion

ZQN MUST NOT silently convert:

NaN → 0
∞ → maximum
negative probability → absolute value
invalid dimension → zero
overflow → wrapped value

S-005 — No hidden randomness

ZQN MUST NOT use hidden/global randomness.

S-006 — No hidden mutable global state

No global mutable noise model, calibration state, cache or RNG may influence semantics.

S-007 — No implicit trust after deserialization

Deserialized values require semantic validation.

S-008 — No silent approximation

A security-sensitive or scientifically significant approximation MUST be explicit.

S-009 — No vendor-specific trust assumptions

Vendor identity MUST NOT bypass validation.

S-010 — No machine-size assumptions

Security controls MUST scale with configured resource policies rather than fixed machine-size assumptions.

S-011 — No denial-of-service through unbounded eager work

Potentially expensive operations MUST support resource limits and/or cancellation.

S-012 — Deterministic validation ordering

When multiple validation failures exist, their ordering MUST be canonical.

S-013 — Canonical identities

Security-relevant identities MUST use canonical identifiers.

S-014 — Canonical serialization

Security hashes MUST use canonical serialization rather than Rust implementation details.

S-015 — Retry safety

Retries MUST NOT silently produce a different deterministic stochastic realization.

---

6. No artificial scalability limits

The security system MUST NOT define limits such as:

const MAX_QUBITS: usize = 1024;
const MAX_CORRELATED_QUBITS: usize = 64;

as semantic restrictions.

That would make security depend on an arbitrary machine size.

Instead, limits belong to:

ZqnLimits
ExecutionContext
ResourcePolicy
TargetCapabilities
RuntimePolicy

A limit means:

«This particular execution environment is unwilling or unable to perform the requested operation.»

It MUST NOT mean:

«ZQN considers larger quantum systems invalid.»

---

7. Resource exhaustion protection

The largest security threat to a universal noise framework is often denial of service rather than memory corruption.

A malicious model can request:

10^12 resources
10^15 correlations
10^20 distribution entries
10^30 tensor elements

even when those values are mathematically valid.

Therefore ZQN MUST distinguish:

semantic validity
        from
resource feasibility

An object may be mathematically valid but infeasible under the current execution policy.

The correct result is:

ResourceLimitExceeded

not:

- uncontrolled allocation;
- process termination;
- stack overflow;
- infinite loop;
- silent truncation.

---

8. Resource limits

"core/limits.rs" owns configurable execution limits.

Limits SHOULD cover, where applicable:

- operations;
- resources;
- faults;
- distribution entries;
- correlation terms;
- tensor elements;
- matrix elements;
- channel dimension;
- memory;
- CPU work;
- simulation steps;
- shots;
- trajectory count;
- characterization samples;
- serialization size;
- deserialization size;
- recursion depth;
- nesting depth;
- output size;
- execution time;
- pending work;
- queue size.

Limits MUST be represented as policy rather than semantic constants.

For example:

None

means no ZQN-imposed policy limit.

The runtime or target may still impose physical/resource limits.

---

9. Limits must never change successful semantics

Changing a resource limit MUST NOT alter the result of a computation that successfully completes under both limits.

For example:

limit = 1 GiB
limit = 4 GiB

must not produce different deterministic noise samples merely because a different amount of memory was available.

A lower limit may cause:

ResourceLimitExceeded

but it MUST NOT silently change:

noise distribution
fault realization
probability
channel
calibration
random seed

unless an explicitly selected approximation/resource policy makes the algorithm choice part of the execution context.

---

10. Allocation safety

ZQN MUST never directly trust an externally supplied dimension.

Before allocation:

1. validate the dimension;
2. validate mathematical consistency;
3. validate configured limits;
4. calculate allocation size using checked arithmetic;
5. reject overflow;
6. reject impossible allocations;
7. allocate only after all checks pass.

The sequence MUST be:

input
 ↓
validate
 ↓
checked dimension arithmetic
 ↓
resource-policy check
 ↓
allocation

never:

input
 ↓
allocation
 ↓
validation

---

11. Integer overflow

Security-sensitive arithmetic MUST use checked operations.

Examples:

checked_add
checked_sub
checked_mul
checked_pow
checked_shl
checked_div

where applicable.

This is especially important for:

- matrix dimensions;
- tensor dimensions;
- qubit/resource counts;
- shot counts;
- fault counts;
- serialization sizes;
- memory calculations;
- correlation cardinality;
- Cartesian products.

Overflow MUST produce a structured ZQN error.

It MUST never wrap silently.

---

12. Integer type policy

Semantic counters SHOULD use sufficiently wide explicit integer types rather than platform-dependent "usize".

Where practical, semantic indices such as:

- shot;
- event;
- logical operation occurrence;
- correlation occurrence;

SHOULD use "u128".

"usize" MAY be used at concrete materialization boundaries where Rust collections require it.

Conversions MUST be checked.

This preserves the architecture's:

«no artificial machine-size ceiling»

principle while acknowledging the finite representational limits of a concrete execution environment.

---

13. Streaming architecture

Large ZQN operations MUST support streaming where materialization is unnecessary.

Examples:

FaultBatch
Distribution
Characterization observations
Monte Carlo shots
Correlation events
Noise events
Benchmark samples

must be able to operate through:

- iterators;
- bounded batches;
- lazy generators;
- streaming interfaces.

The system MUST NOT require:

Vec<EveryPossibleFault>

for a mathematically enormous fault space.

This is both a scalability and denial-of-service requirement.

---

14. Lazy evaluation security

Lazy generators MUST be guaranteed to terminate under bounded requests.

A generator MUST support:

- explicit requested work;
- cancellation;
- resource limits;
- deterministic progression.

A malformed generator MUST NOT be able to create an uncontrolled infinite execution.

---

15. Recursive model protection

Nested noise specifications can create pathological structures:

A contains B
B contains C
C contains A

or extremely deep nesting.

ZQN MUST detect:

- cycles where cycles are invalid;
- excessive recursion;
- recursive expansion;
- exponential expansion.

Recursive semantic models that are legitimately supported MUST use explicit bounded evaluation mechanisms.

---

16. Graph and correlation attacks

Spatial and correlated noise models may contain large graphs.

Protection MUST cover:

- number of vertices;
- number of edges;
- correlation terms;
- tensor dimensions;
- adjacency expansion;
- transitive expansion;
- connected-component processing.

Do not assume that a graph is small because a quantum processor is small.

Distributed and future quantum systems may have very large resource graphs.

---

17. Cartesian-product explosion

Operations such as:

A ⊗ B

or correlation expansion can cause exponential growth.

ZQN MUST estimate resource requirements before materialization whenever possible.

For example:

dimension(A) × dimension(B)

must be checked before allocating the resulting structure.

If the result exceeds policy:

ResourceLimitExceeded

must be returned.

Alternative streaming or sparse representations SHOULD be selected when explicitly permitted by the representation policy.

---

18. Sparse representation security

Sparse structures MUST protect against:

- duplicate keys;
- invalid indices;
- unsorted keys;
- pathological density;
- integer overflow;
- enormous index gaps;
- memory amplification.

Sparse input MUST NOT cause dense allocation merely because an attacker supplied a huge index.

---

19. Dense matrix/tensor security

For:

- Kraus operators;
- Choi matrices;
- process matrices;
- superoperators;
- tensor representations;

ZQN MUST validate dimensions before allocation.

A dimension:

d × d

must never be converted into an allocation without checking:

d * d

and the resulting byte size.

The same applies recursively to tensors.

---

20. Probability security

Probability values MUST satisfy their mathematical contract.

Reject:

NaN
+∞
-∞
negative values
values outside the permitted domain

unless a specific representation explicitly defines another mathematical domain.

Never silently repair invalid probabilities.

For example, this is forbidden:

p = p.max(0.0).min(1.0)

as an implicit validation mechanism.

Invalid input must be rejected.

---

21. Distribution security

Distributions MUST validate:

- support;
- weights;
- normalization;
- finite values;
- duplicate outcomes;
- canonical outcome ordering;
- parameter validity.

A distribution containing billions of entries MUST be subject to resource policy before materialization.

Sampling MUST not traverse attacker-controlled unbounded structures indefinitely.

---

22. Floating-point security

Floating-point inputs can contain:

NaN
+∞
-∞
-0.0
subnormal values

ZQN MUST define canonical handling.

For probability and physical parameter APIs:

- NaN MUST be rejected;
- infinities MUST be rejected unless explicitly meaningful;
- signed zero MUST be canonicalized where semantic identity requires it;
- tolerances MUST be explicit.

Floating-point comparisons MUST NOT accidentally become security decisions without an explicit tolerance policy.

---

23. Numerical instability

A mathematically valid input may still cause catastrophic numerical instability.

ZQN SHOULD detect:

- overflow risk;
- underflow risk;
- loss of normalization;
- ill-conditioned transformations;
- invalid square roots;
- invalid logarithms;
- unstable matrix operations;
- catastrophic cancellation where detectable.

The result should be an explicit:

NumericalFailure

or a declared approximate/bounded result.

Do not return a plausible-looking invalid value.

---

24. Approximation security

Approximation MUST be explicit.

Valid modes include:

Exact
Approximate
Bounded
Statistical
Unsupported

An approximation MUST record:

- requested semantics;
- realized semantics;
- approximation method;
- tolerance;
- error bound where available;
- confidence where statistical;
- assumptions;
- algorithm/version identity.

Never silently replace:

non-Markovian noise

with:

Markovian noise

or:

correlated noise

with:

independent noise

merely because the target cannot represent the original model.

---

25. Channel security

Every quantum channel representation MUST validate its mathematical requirements before use.

Depending on representation, this may include:

- dimension consistency;
- complete positivity;
- trace preservation;
- parameter bounds;
- Hermiticity constraints;
- normalization;
- finite numerical values.

Validation MUST be representation-aware.

Do not assume every channel is a Pauli channel.

---

26. Kraus representation security

"channel/kraus.rs" MUST protect against:

- invalid operator dimensions;
- inconsistent subsystem dimensions;
- excessive operator counts;
- excessive matrix sizes;
- invalid complex values;
- invalid normalization;
- allocation multiplication overflow.

Kraus completeness checks MUST use explicit numerical tolerances where approximate arithmetic is used.

---

27. Choi representation security

"channel/choi.rs" MUST protect against:

- invalid dimensions;
- enormous Choi matrices;
- non-finite entries;
- invalid positivity;
- invalid trace-preservation conditions;
- numerical instability.

A Choi representation MUST NOT be expanded into a larger representation unless the target/resource policy allows it.

---

28. Process-matrix security

General process representations may have high-dimensional structures.

Before evaluation:

requested representation
        ↓
dimension analysis
        ↓
resource estimate
        ↓
policy validation
        ↓
execution

must occur.

---

29. Correlated noise security

Correlated noise is especially dangerous for resource amplification.

A model specifying:

all resources correlated

could produce an enormous interaction structure.

ZQN MUST support symbolic/lazy correlation representations.

It MUST NOT eagerly enumerate all correlated combinations unless permitted by resource policy.

---

30. Crosstalk security

"noise/crosstalk.rs" MUST validate:

- participating resources;
- correlation domains;
- interaction scope;
- parameter ranges;
- target capability;
- evaluation complexity.

Crosstalk models MUST NOT be allowed to modify routing or scheduling state directly.

The ownership remains:

ZQN → describes noise
routing → chooses placement
scheduling → chooses time

This prevents privilege escalation across subsystem boundaries.

---

31. Temporal-noise security

Time-dependent noise MUST NOT implicitly read wall-clock time to determine semantic behavior.

Instead, temporal behavior MUST receive an explicit execution time/context.

For example:

logical execution time
+
calibration snapshot
+
drift model

determines the noise state.

Wall-clock timestamps MAY be recorded for provenance but MUST NOT silently become semantic inputs.

This integrates with:

noise/temporal.rs
noise/drift.rs
calibration/drift.rs

---

32. Non-Markovian security

Non-Markovian models may maintain history.

Security requirements:

- history size must be governed;
- memory usage must be bounded by policy;
- state transitions must be validated;
- recursive history expansion must be prevented;
- checkpoints must be explicit;
- cancellation must release resources;
- no global history state.

"noise/non_markovian.rs" MUST NOT use process-global mutable state to store environmental history.

---

33. Conditional-noise security

Conditional noise may depend on:

- measurement results;
- classical conditions;
- execution state;
- calibration;
- time;
- target context.

The condition evaluator MUST be:

- explicitly supplied;
- validated;
- bounded;
- deterministic when deterministic mode is selected.

A condition MUST NOT gain access to:

- filesystem;
- network;
- environment variables;
- arbitrary process execution;
- secret runtime state.

ZQN is not a general-purpose scripting environment.

---

34. Calibration security

Calibration data is security-sensitive scientific input.

"calibration/*" MUST validate:

- schema;
- identity;
- target;
- resource;
- units;
- ranges;
- uncertainty;
- timestamps;
- validity interval;
- provenance;
- version.

Malformed calibration MUST fail closed.

---

35. Calibration authenticity

Where calibration data comes from an external system, the integration layer SHOULD support authenticity verification.

Possible mechanisms include:

- signed artifacts;
- authenticated transport;
- trusted content hashes;
- provenance signatures.

ZQN itself SHOULD remain cryptography-provider-neutral.

The security layer MUST not invent an insecure home-grown signature algorithm.

---

36. Calibration replay protection

A calibration snapshot MUST have an immutable identity.

The identity SHOULD incorporate:

schema version
target identity
resource identity
calibration content
validity interval
provenance

A mutable calibration file MUST NOT silently change the meaning of an existing "CalibrationId".

If content changes, its identity changes.

---

37. Calibration cache poisoning

Calibration caches MUST NOT use:

device name

alone as a key.

A safe cache identity includes the relevant:

target identity
resource identity
calibration identity
schema version
parameter identity
validity context

A poisoned or incompatible cache entry MUST be rejected.

---

38. Provenance security

"core/provenance.rs" MUST record enough information to determine where a model came from.

At minimum, where applicable:

- source type;
- source identity;
- content digest;
- model version;
- calibration identity;
- experiment identity;
- schema version;
- software version;
- creation metadata.

Provenance MUST NOT be used as executable input.

---

39. No arbitrary code execution

ZQN serialization formats MUST NOT support arbitrary executable code.

Never deserialize an object in a way that causes:

shell execution
process execution
dynamic library loading
arbitrary script evaluation

A noise specification must remain data.

---

40. Expression safety

If future ZQN specifications support expressions, expressions MUST be evaluated in a restricted environment.

They MUST NOT automatically access:

- filesystem;
- environment variables;
- network;
- subprocesses;
- arbitrary native libraries;
- memory addresses;
- system clocks.

Any external capability must be explicitly provided by the runtime and governed by policy.

---

41. Deserialization security

"io/deserialization.rs" MUST treat every byte as hostile.

It must protect against:

- oversized input;
- malformed UTF-8;
- duplicate fields where ambiguous;
- invalid enum values;
- invalid numeric values;
- recursive structures;
- excessive nesting;
- allocation bombs;
- integer overflow;
- unknown incompatible schema versions.

Deserialization MUST have resource limits.

---

42. Serialization security

"io/serialization.rs" MUST ensure:

- bounded output;
- canonical encoding where required;
- explicit schema version;
- deterministic field ordering where canonical output is required;
- no secret leakage;
- no internal memory addresses;
- no debug representations.

---

43. Canonical serialization

"io/canonical.rs" owns canonical representation used for:

- identity;
- hashing;
- reproducibility;
- cache keys;
- provenance;
- signatures where required.

Never use:

std::hash::Hash

as a persistent protocol identity.

Never hash:

Debug
Display
memory address
pointer value
HashMap iteration order

Canonical encoding MUST be language-independent.

This is essential because Zamani is intended to be more than a Rust-only implementation.

---

44. Hash security

Whenever content hashes are used, the hash algorithm and protocol version MUST be explicit.

A hash identity should conceptually be:

Hash(
    protocol_version
    ||
    schema_version
    ||
    canonical_object
)

Changing the canonicalization algorithm MUST result in a protocol/version change.

---

45. Identity security

"core/ids.rs" owns ZQN identities.

Security-sensitive IDs MUST be:

- stable;
- canonical;
- serializable;
- collision-resistant where cryptographic identity is required;
- independent of allocation order.

Human-readable labels MUST NOT be treated as globally unique identities.

---

46. Qubit identity

ZQN MUST NOT create a competing physical qubit identity.

When a ZQN object refers to a qubit, it MUST use the canonical identity from:

crate::quantum::ir::qubit

including:

QubitId
PhysicalQubitId

where applicable.

A ZQN security check MUST NOT reinterpret:

QubitId

as an arbitrary array position merely because doing so is convenient.

Do not assume:

q0
q1
q2
...

are contiguous.

Do not assume a physical resource can be safely identified by:

usize

alone.

---

47. Resource identity security

Resource identities may represent:

- logical qubits;
- physical qubits;
- qudits;
- modes;
- bosonic modes;
- logical resources;
- communication links;
- composite resources.

Security-sensitive operations MUST use canonical resource identities.

Resource ordering for hashing, serialization or deterministic processing MUST be canonical.

---

48. Deterministic randomness security

ZQN's deterministic randomness MUST follow "DETERMINISM.md".

There MUST be:

- no global RNG;
- no hidden RNG;
- no thread-local semantic RNG;
- no wall-clock seed;
- no process-ID seed;
- no memory-address seed.

Randomness must be explicitly derived from a deterministic context.

---

49. Deterministic randomness is not cryptographic randomness

A deterministic ZQN seed is intended for:

- simulation;
- reproducibility;
- characterization;
- testing;
- benchmarking;
- deterministic fault generation.

It MUST NOT be used as a cryptographic secret generator.

If security-sensitive cryptographic randomness is ever required, it must use an explicitly approved cryptographic entropy source outside the deterministic simulation protocol.

---

50. Randomness-address isolation

Random events MUST be independently addressable.

Conceptually:

master seed
    +
domain
    +
program identity
    +
model identity
    +
calibration identity
    +
target identity
    +
operation identity
    +
resource identity
    +
shot
    +
event
    +
substream

determines the random event.

This prevents:

event A
event B
event C

from sharing a mutable global RNG stream.

---

51. Parallelism security

The result of deterministic ZQN execution MUST NOT depend on:

1 worker
8 workers
64 workers

when all semantic inputs are identical.

A malicious or accidental scheduling difference must not alter deterministic stochastic values.

This is achieved by addressable random events rather than shared RNG consumption.

---

52. Retry security

A retried event MUST reuse the same randomness address.

This guarantees:

attempt 1 → result X
attempt 2 → result X

for deterministic execution.

Retries MUST NOT consume another global RNG value.

---

53. Cancellation security

Canceling one operation MUST NOT shift the random sequence of unrelated operations.

This is another reason global sequential RNG streams are forbidden.

Addressable random events ensure:

cancel A

does not alter:

B's random value
C's random value
D's random value

---

54. Checkpoint security

Checkpoints MUST preserve enough deterministic context to reproduce execution.

At minimum:

- determinism protocol version;
- seed policy;
- seed identity;
- program identity;
- model identity;
- calibration identity;
- target identity;
- numerical profile;
- current logical execution state;
- completed event/shot identity.

A checkpoint MUST NOT rely only on mutable PRNG state if random events are addressable.

---

55. Speculative execution

Speculative execution MUST NOT consume shared mutable randomness.

Speculative branches must derive random values from branch/event identities.

If a speculative branch is discarded, it MUST NOT affect unrelated future randomness.

---

56. Distributed execution

Distributed execution MUST not derive semantic randomness from:

- machine hostname;
- operating-system PID;
- process launch time;
- network packet order;
- thread identity.

Stable logical node/resource identities may be used when they are explicitly part of the target execution context.

This is essential for distributed quantum computing.

---

57. Network security

ZQN itself must remain backend-independent.

Network communication belongs to the relevant hardware/runtime integration.

Nevertheless, ZQN MUST assume network-provided:

- calibration;
- characterization;
- model;
- execution metadata;

may be malicious.

Authenticated transport and authorization are responsibilities of the surrounding runtime/hardware boundary.

ZQN remains responsible for validating received semantic data.

---

58. Network response determinism

Network responses MUST NOT silently affect deterministic semantics unless the response itself is an explicit input.

For reproducible execution:

network data
↓
snapshot
↓
canonicalize
↓
hash
↓
explicit execution input

must be used.

"Whatever the backend returns right now" is not a deterministic execution contract.

---

59. Environment isolation

ZQN semantics MUST NOT implicitly depend on:

- environment variables;
- current directory;
- locale;
- timezone;
- hostname;
- process ID;
- CPU core;
- system memory size;
- current time;
- filesystem ordering.

If such data is semantically required, it MUST be explicitly passed through an execution context and recorded in provenance.

---

60. Filesystem security

ZQN semantic code MUST NOT directly open arbitrary filesystem paths.

Filesystem access belongs to:

- runtime;
- import layer;
- hardware adapter;
- application boundary.

Imported data must pass through the ZQN validation boundary.

If a file is imported:

filesystem
 ↓
bounded read
 ↓
deserialization
 ↓
validation
 ↓
canonical object

must be used.

---

61. Path traversal

Where ZQN-adjacent import facilities accept paths, they MUST protect against:

../
absolute paths
symlink escape
device paths
unexpected filesystem namespaces

ZQN itself should preferably consume bytes/content rather than filesystem paths.

---

62. Resource amplification

Every operation must consider amplification.

Examples:

small input
    ↓
large tensor

small graph
    ↓
large closure

small distribution
    ↓
large Cartesian product

small calibration file
    ↓
large interpolated model

Before amplification, ZQN SHOULD calculate or conservatively estimate the output resource requirement.

---

63. Algorithmic complexity attacks

ZQN must defend against inputs that cause pathological:

- O(n²);
- O(n³);
- exponential;
- factorial;
- recursive;
- graph traversal;

behavior.

This does not mean banning expensive algorithms.

It means expensive algorithms must be:

- explicit;
- bounded;
- cancellable;
- resource-accounted;
- preferably estimable before execution.

---

64. Denial-of-service categories

ZQN MUST explicitly test for:

Memory exhaustion

Huge:

- matrices;
- tensors;
- distributions;
- fault batches;
- observations.

CPU exhaustion

Pathological:

- correlations;
- numerical convergence;
- characterization;
- iterative estimation.

Stack exhaustion

Deep:

- recursive models;
- nested representations;
- malformed serialized structures.

Output amplification

Tiny input producing enormous output.

Nontermination

Generators or convergence procedures that never finish.

Numerical exhaustion

Repeated operations causing:

- overflow;
- underflow;
- precision collapse;
- convergence failure.

---

65. Cancellation

All potentially expensive operations SHOULD support cancellation through the existing runtime/context boundary.

Cancellation must be cooperative and deterministic.

A canceled operation must return an explicit error such as:

Cancellation

and release resources.

Cancellation MUST NOT leave:

- global mutable state;
- half-written cache entries;
- corrupted provenance;
- invalid calibration state.

---

66. Thread safety

ZQN semantic model types SHOULD be:

Send + Sync

where semantically possible.

Immutable models are preferred.

Mutable state must be explicitly owned by an execution context.

Never use:

GLOBAL_NOISE_MODEL
GLOBAL_CALIBRATION
GLOBAL_RNG
GLOBAL_CACHE

as semantic state.

---

67. Cache security

Caches MUST be treated as untrusted persistence boundaries.

A cache hit MUST be validated against:

- schema version;
- ZQN protocol version;
- object identity;
- model identity;
- target identity;
- calibration identity;
- configuration identity;
- numerical profile.

Cache poisoning MUST NOT silently change semantics.

---

68. Cache atomicity

A partially written result MUST never appear as a valid cache entry.

Use:

temporary result
 ↓
complete + validate
 ↓
canonicalize
 ↓
commit atomically

where the surrounding persistence system supports atomic replacement.

---

69. Cache confidentiality

ZQN cache entries may contain:

- calibration information;
- hardware characterization;
- proprietary noise models;
- experiment metadata.

The persistence layer SHOULD support access control and encryption where required.

ZQN itself should not assume that all model data is public.

---

70. Error-message security

Errors MUST NOT leak:

- credentials;
- authentication tokens;
- private calibration contents;
- filesystem secrets;
- environment secrets;
- private network information.

Error messages SHOULD contain:

- error class;
- relevant semantic identity;
- safe diagnostic information.

Do not dump complete attacker-controlled payloads into error messages.

---

71. Error determinism

When several validation errors are present, their order MUST be canonical.

For example:

resource identity
dimension
parameter
normalization

or another explicitly defined canonical order.

Do not report errors according to:

HashMap iteration order
thread completion order
parallel task completion

---

72. Logging security

ZQN logging MUST avoid logging:

- raw credentials;
- authentication material;
- secret seeds where classified as sensitive;
- private calibration payloads;
- arbitrary user data.

Logs SHOULD include:

- object identity;
- model identity;
- execution identity;
- schema version;
- validation result;
- security event;
- resource policy outcome.

---

73. Seed handling

Deterministic simulation seeds are normally reproducibility metadata rather than secrets.

Nevertheless, the API MUST allow callers to classify seed material appropriately.

A seed MUST NOT be written to logs merely because debug logging is enabled.

A seed should be represented as an explicit object rather than a random "u64" scattered throughout APIs.

---

74. Cryptographic boundaries

ZQN is not a cryptographic subsystem.

It MUST NOT implement:

- home-grown encryption;
- home-grown authentication;
- home-grown key exchange;
- home-grown signatures.

Where cryptographic protection is needed, ZQN should integrate with the repository's approved cryptographic/security layer.

---

75. Quantum-security distinction

ZQN models quantum noise.

It does not automatically provide:

post-quantum cryptographic security

and a quantum noise model MUST NOT be advertised as a security guarantee against quantum attackers.

Quantum-system reliability and post-quantum cryptography are separate security domains.

---

76. Hardware security

"integration/hardware.rs" MUST be an adapter boundary.

ZQN MUST NOT:

- store QPU credentials;
- authenticate directly against arbitrary vendors;
- execute arbitrary backend commands;
- trust vendor metadata without validation.

The hardware subsystem is responsible for authorization and transport.

ZQN consumes validated abstract:

TargetCapabilities
CalibrationSnapshot
NoiseObservation

objects.

---

77. Hardware result integrity

Hardware observations SHOULD include:

- target identity;
- execution identity;
- calibration identity;
- measurement context;
- schema version;
- observation provenance.

Where supported, the hardware integration SHOULD provide integrity/authenticity information.

---

78. Physical nondeterminism

A physical QPU cannot generally guarantee bit-for-bit deterministic measurement outcomes.

Therefore:

hardware execution

must not falsely advertise:

BitwiseDeterministic

unless the target explicitly guarantees that property.

ZQN can guarantee reproducibility of:

- model construction;
- request generation;
- deterministic preprocessing;
- deterministic simulation;
- deterministic analysis;

while physical outcomes generally remain statistical.

---

79. Routing security

"integration/routing.rs" must not allow a ZQN model to directly mutate router state.

The interface should be:

routing request
      ↓
ZQN noise/cost query
      ↓
validated result
      ↓
router decision

not:

noise model
      ↓
arbitrary routing mutation

The existing repository's noise-aware routing architecture is therefore a consumer boundary, not a security authority.

---

80. Scheduling security

"integration/scheduling.rs" must treat ZQN as a source of:

- duration-dependent noise;
- fidelity information;
- crosstalk information;
- calibration validity;
- error estimates.

ZQN MUST NOT directly control the scheduler.

This prevents cross-subsystem privilege escalation.

---

81. QEC security

The existing QEC noise subsystem already provides physical-noise/fault functionality and deterministic generation.

The long-term architecture is:

                 ZQN
                  │
        canonical physical noise
                  │
                  ▼
          QEC adapter
                  │
       ┌──────────┼──────────┐
       ▼          ▼          ▼
     faults    syndrome    decoder

QEC MUST NOT duplicate ZQN's:

- probability model;
- universal noise model;
- random-source architecture;
- calibration identity;
- correlation semantics.

During migration, an adapter may preserve legacy behavior.

---

82. QEC fault amplification

A physical noise model may generate enormous fault spaces.

QEC integration MUST therefore support:

- lazy faults;
- streaming;
- bounded batches;
- cancellation;
- deterministic addresses.

A malicious noise model MUST NOT force QEC to materialize an unbounded fault collection.

---

83. Benchmarking security

Benchmarking input may contain:

- random circuit definitions;
- model configurations;
- shot counts;
- observations;
- hardware data.

"integration/benchmarking.rs" MUST validate all of them.

Benchmark seed/configuration identities MUST be explicit.

The repository's benchmarking subsystem already treats seed and generator version as reproducibility information; ZQN integration must preserve that principle.

---

84. Characterization security

Characterization algorithms must protect against:

- malicious observation counts;
- impossible probabilities;
- pathological convergence requests;
- excessive shot counts;
- oversized experiment sets;
- invalid uncertainty parameters.

Every estimator MUST have a resource policy.

---

85. Statistical security

Statistical estimates MUST distinguish:

measurement
estimate
uncertainty
confidence
assumption

Do not allow malformed observations to become apparently authoritative model parameters.

---

86. Uncertainty security

Uncertainty values must be validated.

Reject:

- negative uncertainty where mathematically invalid;
- NaN;
- infinite values;
- inconsistent intervals;
- impossible confidence levels.

Intervals MUST satisfy:

lower <= upper

where applicable.

---

87. Characterization poisoning

A malicious or corrupted experiment can poison a noise model.

Therefore characterization pipelines SHOULD retain:

raw observation identity
experiment identity
estimator identity
estimator version
assumptions
confidence
uncertainty
calibration identity

A derived model must remain traceable to its evidence.

---

88. Model provenance

"noise/model.rs" models MUST expose stable identity/provenance.

A model's identity SHOULD depend on canonical semantic content, not:

- object address;
- allocation order;
- debug output;
- insertion order.

The repository already has a dedicated canonical noise-model contract in "noise/model.rs"; this security policy applies to that boundary.

---

89. Noise application security

"noise/application.rs" is the boundary where a validated model meets an execution scope.

It MUST validate:

- operation identity;
- resource identity;
- target compatibility;
- calibration identity;
- timing;
- representation;
- resource policy.

The existing file is explicitly positioned as the application/attachment boundary between a ZQN noise model and execution scope, which is the correct security boundary.

---

90. Temporal and drift security

Temporal and drift models must use explicit time inputs.

The repository already separates temporal and drift semantics into dedicated ZQN modules.

Security requirements:

- no implicit wall-clock semantics;
- bounded evaluation;
- validated time ranges;
- checked interpolation;
- explicit extrapolation policy;
- no infinite historical state.

---

91. Spatial security

Spatial noise models must treat topology as data.

They MUST NOT assume:

- fixed connectivity;
- fixed number of resources;
- fixed nearest-neighbor structure.

The existing spatial subsystem is already defined as the owner of spatial noise semantics.

Security validation must therefore operate on arbitrary resource graphs.

---

92. Crosstalk security

Crosstalk models must not assume a fixed two-resource interaction.

They may involve arbitrary resource sets.

The existing crosstalk subsystem is explicitly intended to provide provider-independent crosstalk semantics.

Resource expansion must therefore be bounded and streaming-capable.

---

93. Non-Markovian security

The existing "noise/non_markovian.rs" defines noise with memory.

Security requirements include:

- bounded retained state;
- explicit state ownership;
- checkpointability;
- cancellation;
- cycle detection;
- no process-global memory;
- deterministic replay.

---

94. Conditional execution security

The existing conditional subsystem is the boundary for conditional noise semantics.

Conditions MUST remain data/semantic expressions.

They MUST NOT become arbitrary executable host-language code.

---

95. Fault security

"fault/fault.rs" MUST validate every fault.

"fault/location.rs" MUST validate the target resource.

"fault/correlated.rs" MUST validate correlation domains.

"fault/leakage.rs" MUST validate leakage parameters.

"fault/erasure.rs" MUST validate erasure semantics.

"fault/loss.rs" MUST validate loss parameters.

"fault/batch.rs" MUST enforce streaming/resource policies.

The existing leakage implementation already recognizes deterministic construction from an explicit sampling context and seed policy; that contract must be preserved under the universal ZQN security model.

---

96. Serialization compatibility attacks

An attacker may provide an old or intentionally malformed schema.

"io/compatibility.rs" MUST:

- recognize supported versions;
- reject unsupported versions;
- perform explicit migrations;
- never guess semantic meaning;
- never silently reinterpret fields.

A migration MUST be deterministic.

---

97. Version confusion

Security-sensitive identity must include the relevant protocol version.

Do not allow:

ZQN v1 object

to be silently interpreted as:

ZQN v2 semantics

when semantics differ.

Protocol versions and serialization versions are separate concepts.

---

98. Downgrade protection

Where a target supports multiple representations, an attacker must not force an unsafe downgrade.

For example:

exact correlated channel
        ↓
unsupported

must not automatically become:

independent Pauli channel

unless an explicit approximation policy permits it.

---

99. Capability security

"target/capabilities.rs" describes what a target can support.

Capabilities MUST be treated as declarations requiring validation.

A target claiming support for:

non-Markovian noise

does not automatically make the implementation trustworthy.

Capability checks and semantic validation remain separate.

---

100. Capability confusion

Never use:

vendor name

as a security capability.

Use explicit capability values.

For example:

supports_correlated_noise
supports_time_dependent_noise
supports_leakage
supports_dynamic_noise

This keeps the security boundary technology-neutral.

---

101. Approximation downgrade protection

A target lacking a requested feature MUST produce one of:

Unsupported

or:

Explicit Approximation

according to the user's policy.

No silent security/scientific downgrade is allowed.

---

102. Memory subsystem integration

"integration/memory.rs" must pass validated ZQN channel/fault requests into the quantum memory subsystem.

ZQN MUST NOT directly manipulate internal memory representation.

This protects memory invariants.

The existing memory architecture's separation between unitary, channel and measurement semantics makes this a natural boundary.

---

103. Runtime integration

"integration/runtime.rs" must supply:

- execution identity;
- resource policy;
- cancellation;
- determinism context;
- target context;
- calibration context.

ZQN MUST NOT obtain those values implicitly from global runtime state.

---

104. Runtime privilege separation

Runtime capabilities such as:

filesystem
network
process execution
hardware control

must not automatically become available to ZQN.

ZQN should receive only the specific data/capabilities required for its computation.

---

105. Secret separation

ZQN APIs MUST NOT accept broad credential objects.

Do not pass:

CloudCredentials
QpuCredentials
AdminToken

through ZQN semantic APIs.

Use opaque validated target handles where needed.

---

106. Security of plugins/extensions

Future ZQN extension mechanisms MUST treat third-party implementations as untrusted unless explicitly trusted.

An extension must not receive more privileges than its declared interface requires.

The extension mechanism MUST NOT require "unsafe".

---

107. Dependency security

ZQN dependencies MUST be minimized.

Every dependency must have:

- clear purpose;
- maintained upstream;
- compatible license;
- known security posture;
- reproducible version;
- audit path.

Avoid dependencies merely for convenience when equivalent safe standard-library functionality exists.

---

108. Rust toolchain

Production ZQN MUST support:

Rust 1.97
Rust 1.97.1
Rust 2021

No unstable language features may be required.

Compiler/toolchain identity should be included in strict reproducibility provenance where applicable.

---

109. Dependency reproducibility

Production builds SHOULD use a locked dependency graph.

Security/reproducibility metadata SHOULD include:

Cargo.lock
compiler version
target architecture
enabled features
dependency versions

where strict build reproducibility is required.

---

110. Unsafe-code audit

CI MUST fail if ZQN introduces unsafe code.

The security gate SHOULD include a repository-level check for unsafe syntax within the ZQN subtree.

The preferred enforcement is the Rust lint:

#![forbid(unsafe_code)]

rather than relying solely on text searches.

---

111. FFI restrictions

ZQN core MUST NOT use FFI.

If a future hardware or numerical backend requires FFI, it belongs behind an explicit adapter boundary outside the semantic ZQN core.

That adapter must not contaminate:

probability
channel
noise
fault
determinism

semantics.

---

112. SIMD/GPU security

GPU/SIMD execution may have different numerical behavior.

Strict determinism MUST reject unsupported nondeterministic kernels.

A GPU implementation MUST NOT silently weaken:

- numerical guarantees;
- security checks;
- bounds checks;
- resource accounting.

---

113. Numerical backend selection

Numerical backend selection must be explicit when it can affect semantics.

For example:

CPU deterministic backend
GPU approximate backend

must have distinguishable execution identities.

---

114. Parallel reduction security

Floating-point reductions must not depend on arbitrary worker completion order.

Use:

- canonical reduction order;
- deterministic tree reduction;
- exact/compensated accumulation where required.

Do not use nondeterministic atomic accumulation in strict deterministic mode.

---

115. Convergence security

Iterative algorithms must have:

- convergence criterion;
- iteration/work limit;
- cancellation;
- numerical failure detection.

Never permit an externally supplied model to force an unbounded convergence loop.

---

116. Sampling security

Every sampler must validate:

- distribution;
- sample count;
- seed/context;
- output limits;
- algorithm identity.

A request for an astronomically large shot count must fail under policy rather than allocate or compute indefinitely.

Streaming sampling SHOULD be supported.

---

117. Monte Carlo security

"simulation/monte_carlo.rs" MUST support:

- bounded shot count;
- deterministic shot identities;
- streaming results;
- cancellation;
- deterministic aggregation;
- checkpoint/restart.

Adding workers must not change deterministic samples.

---

118. Trajectory security

"simulation/trajectory.rs" MUST bound:

- trajectory count;
- event count;
- state history;
- retained memory.

Each trajectory must have an explicit deterministic identity when deterministic mode is selected.

---

119. Fault-stream security

Fault generators MUST support:

lazy generation
bounded batches
deterministic addressing
cancellation
resource accounting

A fault generator MUST NOT require materializing all possible faults.

---

120. Resource exhaustion errors

Resource exhaustion must use structured errors.

At minimum:

ResourceLimitExceeded

should identify, where safe:

- resource category;
- requested amount;
- permitted amount;
- operation identity.

Do not disclose sensitive internal details.

---

121. Security error classification

Security-relevant failures should be distinguishable from:

InvalidProbability
InvalidChannel
InvalidNoiseModel

and from:

ResourceLimitExceeded
Cancellation
CompatibilityFailure

This allows callers to determine whether to:

- reject input;
- retry;
- reduce workload;
- choose another representation;
- report a security event.

---

122. No panic on untrusted input

ZQN public APIs processing untrusted input MUST return "Result".

They MUST NOT rely on:

unwrap()
expect()
panic!()

for attacker-controlled conditions.

Panics may remain appropriate for proven internal invariants only where they cannot be reached by untrusted data, but production ZQN should strongly prefer structured errors.

---

123. Indexing policy

Avoid unchecked:

collection[index]

when "index" may be externally influenced.

Prefer checked access.

Out-of-range access must become a structured validation error.

---

124. Iterator security

Iterators over untrusted or derived data must have clear termination semantics.

A lazy iterator that can theoretically be infinite MUST require explicit bounded consumption.

---

125. Serialization bomb protection

Deserializers MUST enforce limits before constructing:

- nested objects;
- matrices;
- vectors;
- maps;
- tensors;
- distributions;
- correlations.

Input size alone is insufficient because compressed/structured data may expand dramatically.

---

126. Compression bombs

If compressed input is ever supported, the decompressed-size limit MUST be enforced.

Never decompress an attacker-controlled payload without a bounded expansion policy.

---

127. Unicode/string security

Identifiers and labels MUST be validated as required by the canonical schema.

Security identity MUST NOT depend on ambiguous presentation.

Where strings participate in canonical identity:

- encoding must be explicit;
- normalization policy must be explicit;
- locale must not affect identity.

---

128. Locale independence

Security-sensitive parsing MUST NOT depend on system locale.

Numbers must use canonical representations.

For example:

1.25

must not change meaning because a machine uses a locale where commas represent decimal separators.

---

129. Time representation

Security and deterministic semantics should use explicit machine-readable time.

Prefer:

integer ticks
canonical duration
explicit timestamp

over locale-dependent strings or implicit wall-clock state.

---

130. Clock manipulation

Wall-clock time MUST NOT determine:

- random seeds;
- model identity;
- calibration identity;
- deterministic simulation results.

Clock data can be provenance.

It must not silently become semantic randomness.

---

131. Drift attack protection

An attacker must not be able to claim that a stale calibration is current by merely changing a timestamp field.

Calibration validity must be tied to:

- immutable content identity;
- target identity;
- provenance;
- validity policy.

---

132. Model substitution attacks

A model cache or serialized object must not be substituted merely because it has the same human-readable name.

Use canonical model identity.

---

133. TOCTOU protection

Avoid:

check file
↓
later read file

for security-critical content.

Prefer:

read
↓
canonicalize
↓
validate
↓
hash
↓
use

so the validated object is the object actually executed.

---

134. TOCTOU in calibration

Do not validate one calibration snapshot and then retrieve a potentially different live snapshot for execution.

The validated immutable snapshot must be passed through the execution context.

---

135. TOCTOU in target capabilities

Target capabilities used for validation must correspond to the target realization used for execution.

A target identity should bind:

capabilities
+
configuration
+
relevant calibration

where required.

---

136. Security of target lowering

"target/lowering.rs" MUST preserve semantics.

A lowering implementation MUST NOT:

- remove noise silently;
- drop faults;
- change probability;
- change resource identity;
- change calibration context;

without an explicit compatibility/approximation result.

---

137. Security of representation conversion

Conversions:

Kraus ↔ Choi
Choi ↔ superoperator
Pauli ↔ stochastic

must validate both source and destination.

A conversion must never bypass mathematical validation merely because the source was previously trusted.

---

138. Differential-validation security

When two representations are compared, numerical tolerance must be explicit.

Do not declare equality based on:

approximation looks close

without a documented tolerance.

---

139. Provenance tampering

Provenance must be immutable once associated with a validated object.

If provenance changes materially:

object identity

should be recomputed.

---

140. Security of canonical identity

Canonical identity must include all security/scientific properties that change semantics.

If:

parameter X

changes the noise behavior, it MUST influence model identity.

Do not hash only:

model_name

---

141. Cache key completeness

A ZQN result cache key should conceptually include:

ZQN protocol version
schema version
program identity
noise model identity
configuration identity
target identity
calibration identity
determinism policy
seed identity
numerical profile
approximation policy

Additional semantic inputs MUST be included when relevant.

---

142. Benchmark cache security

Benchmark results must include:

- benchmark identity;
- generator identity/version;
- seed;
- model identity;
- target identity;
- calibration identity;
- execution configuration.

This prevents stale benchmark results from being presented as current hardware characterization.

---

143. QEC cache security

QEC-derived results must include:

- code identity;
- decoder identity;
- physical noise model identity;
- calibration identity;
- seed policy;
- target identity.

A logical error rate from one noise model must never be reused for another merely because the labels match.

---

144. Security of migration from existing QEC noise

The current QEC noise implementation must not be abruptly deleted.

Migration should be:

existing QEC noise
       │
       ▼
compatibility adapter
       │
       ▼
ZQN semantic model
       │
       ▼
QEC

Legacy deterministic behavior may be preserved through an explicit compatibility profile.

New ZQN behavior must not silently claim byte-for-byte compatibility unless the old algorithm and derivation are exactly preserved.

---

145. Security of routing migration

The existing noise-aware routing implementation should gradually consume ZQN.

The adapter must ensure:

same noise model
+
same calibration
+
same target

produces the same routing-cost semantics regardless of which caller requests them.

---

146. Security of benchmarking migration

Existing benchmark seeds/generator versions must be mapped explicitly into the ZQN determinism context.

Do not reinterpret an old benchmark seed as a new ZQN seed algorithm without recording the compatibility profile.

---

147. Security of runtime migration

The runtime must become the explicit provider of:

resource policy
determinism context
cancellation
target context
calibration snapshot

ZQN must not reconstruct these from ambient state.

---

148. Security test architecture

ZQN security tests belong under:

src/quantum/zqn/tests/
├── unit/
├── property/
├── differential/
├── determinism/
├── scaling/
├── compatibility/
├── integration/
└── fixtures/

Security-specific tests SHOULD additionally be grouped where useful under:

tests/security/

If a separate directory is not desired, security tests may live under:

tests/property/
tests/scaling/
tests/compatibility/

with security-specific naming.

---

149. Required security tests

At minimum:

Memory

- zero-sized structures;
- maximum permitted structures;
- over-limit structures;
- allocation overflow;
- tensor explosion;
- matrix explosion.

Numeric

- NaN;
- infinity;
- negative probability;
- overflow;
- underflow;
- invalid square root;
- invalid logarithm.

Serialization

- malformed data;
- truncated data;
- huge dimensions;
- deep nesting;
- invalid enum;
- unknown schema;
- duplicate fields.

Determinism

- same seed;
- different thread count;
- different batch size;
- retry;
- checkpoint;
- cancellation;
- distributed partitioning.

Graphs

- huge graph;
- cyclic graph;
- duplicate edge;
- self-loop where invalid;
- dense correlation.

Calibration

- stale calibration;
- invalid interval;
- invalid units;
- NaN parameter;
- substituted calibration.

---

150. Property-based security testing

Property tests MUST verify:

invalid input never produces a valid semantic object

and:

valid bounded input never panics

Useful properties:

deserialize(malformed) -> Err
validate(NaN) -> Err
validate(negative_probability) -> Err
oversized_allocation -> Err
canonicalize(x) == canonicalize(x)

---

151. Fuzzing

ZQN SHOULD have fuzz targets for:

probability parsing
distribution parsing
channel parsing
Kraus parsing
Choi parsing
noise-model parsing
fault parsing
correlation parsing
calibration parsing
canonical serialization
deserialization
target capability input

The fuzzing invariant is:

«No untrusted input may cause undefined behavior, unsafe execution, uncontrolled allocation or uncontrolled nontermination.»

---

152. Fuzzing resource limits

Fuzzers must run with bounded:

- memory;
- CPU;
- input size;
- recursion;
- output.

A fuzzer must not itself become an availability risk.

---

153. Differential security testing

Equivalent channel representations should be compared.

For valid equivalent representations:

Kraus
Choi
Superoperator
Pauli transfer

must produce equivalent observables within declared tolerances.

Security failures include:

- representation-specific validation bypass;
- overflow in one representation;
- missing dimension checks;
- inconsistent resource limits.

---

154. Determinism security tests

Required tests include:

same input × N
same seed × N
1 worker vs many workers
batch size A vs B
streaming vs materialized
checkpoint vs uninterrupted
retry vs first attempt
different map insertion order
different traversal order
distributed partitioning

The semantic result must remain identical under the declared determinism guarantee.

---

155. Concurrency security tests

Run:

1
2
4
8
16
...

workers subject to the test environment.

No result may depend on worker count.

No data race is permitted.

No mutable global state may appear.

---

156. Scaling security tests

Scaling tests must use generated sizes.

Do not encode:

MAX_QUBITS

as the architecture's test boundary.

Instead:

N = generated resource count

and select N according to CI resource budget.

The implementation must not contain semantic branches based on particular N values.

---

157. Security tests for "atom to everywhere"

The same semantic noise specification must be tested against:

tiny resource set
medium resource set
large generated resource set
distributed resource set
different resource topology
different supported quantum modality

Security behavior must remain structurally identical.

Only resource feasibility may differ.

---

158. Security of future quantum modalities

ZQN must not assume:

qubit
gate
two-body interaction

as universal security primitives.

Security validation must work for:

- qubits;
- qudits;
- bosonic modes;
- continuous-variable systems;
- analog systems;
- annealing systems;
- fermionic systems;
- photonic systems;
- distributed quantum systems;
- measurement-based systems;
- logical resources;
- future resource types.

The canonical IR remains the semantic source for resource identity.

---

159. Security of "quantum::ir"

ZQN must treat canonical IR as an upstream trust boundary.

"integration/ir.rs" MUST validate that:

- referenced operation exists;
- resource identity is valid;
- operation/resource relationship is valid;
- semantic identity is stable.

ZQN must not mutate canonical IR merely to make noise processing easier.

---

160. Frontend security

ZQN must not trust frontend-generated data simply because it came from the Zamani compiler.

The security pipeline is:

frontend
 ↓
canonical IR
 ↓
ZQN boundary validation
 ↓
noise semantics

This prevents a frontend bug from becoming a direct physical-execution vulnerability.

---

161. No frontend-specific assumptions

ZQN must not depend on:

- OpenQASM AST;
- Zamani source AST;
- parser internals;
- source token positions;

for semantic security.

Source spans may be retained as diagnostic provenance but must not become physical resource identity.

---

162. OpenQASM integration

If OpenQASM is used:

OpenQASM AST
 ↓
frontend lowering
 ↓
canonical quantum IR
 ↓
ZQN

ZQN must never parse OpenQASM directly.

This maintains the repository's separation between frontend syntax and canonical IR.

---

163. Security of source locations

Source locations can be attacker-controlled.

Do not use:

file path
line number
column

as cryptographic identity.

They are diagnostics/provenance only.

---

164. Auditability

Security-relevant ZQN decisions should be auditable.

Important events include:

- rejected model;
- resource-limit rejection;
- unsupported representation;
- approximation;
- calibration rejection;
- capability mismatch;
- determinism violation;
- invalid serialization;
- compatibility downgrade attempt.

Audit records should contain safe identifiers rather than sensitive payloads.

---

165. Security event IDs

Security events SHOULD use stable event categories.

For example:

ZQN-SEC-RESOURCE
ZQN-SEC-NUMERIC
ZQN-SEC-SERIALIZATION
ZQN-SEC-DETERMINISM
ZQN-SEC-CAPABILITY
ZQN-SEC-CALIBRATION
ZQN-SEC-INTEGRITY

These IDs are diagnostic identifiers, not security credentials.

---

166. Security observability

Production deployments should expose metrics such as:

- rejected inputs;
- resource-limit failures;
- invalid numerical inputs;
- deserialization failures;
- approximation requests;
- capability mismatches;
- calibration failures;
- deterministic replay failures;
- cancellation events.

Metrics MUST NOT leak sensitive model contents.

---

167. Rate limiting

Rate limiting is primarily a runtime/application responsibility.

ZQN should expose enough information for the runtime to enforce:

- maximum requests;
- maximum model processing rate;
- maximum characterization work;
- maximum simulation work.

ZQN itself should not depend on a global rate limiter.

---

168. Multi-tenant security

If multiple users/tenants use one ZQN runtime:

- contexts MUST be isolated;
- caches MUST be tenant-aware where required;
- calibration identities MUST not cross tenant boundaries;
- private models MUST not leak through errors;
- global mutable state MUST NOT exist.

---

169. Cross-tenant cache isolation

A cache key must include tenant/security scope when the same semantic identity can exist under different authorization contexts.

Private calibration/model data must not be returned to another tenant merely because their semantic request matches.

---

170. Memory clearing

ZQN should avoid holding secrets at all.

If sensitive material is ever introduced, memory lifecycle requirements must be handled by the appropriate secure-memory subsystem.

ZQN MUST NOT invent a false claim of secure memory erasure using ordinary Rust memory operations.

---

171. Security of provenance timestamps

Timestamps are provenance data.

They MUST NOT be trusted as proof of authenticity.

An attacker can potentially modify:

created_at
updated_at
valid_from

unless the surrounding system authenticates the record.

---

172. Security of model signatures

If signed models are introduced:

signature
+
canonical model bytes
+
protocol version

must be verified before trusting authenticity.

Signature verification belongs to an approved cryptographic boundary.

---

173. No signature bypass

An object MUST NOT become trusted because:

signature field exists

The signature must be:

- cryptographically verified;
- bound to canonical content;
- bound to the intended protocol;
- validated against the appropriate trust policy.

---

174. Security of external observations

External noise observations may be:

- incomplete;
- contradictory;
- malicious;
- statistically unreliable.

ZQN MUST preserve the distinction between:

observed
estimated
validated
trusted

An observation is not automatically a validated physical law.

---

175. Contradictory calibration

If two calibration records conflict, ZQN MUST NOT silently choose one based on:

- insertion order;
- file order;
- network arrival order.

Selection must use an explicit policy involving:

- target;
- validity;
- provenance;
- version;
- timestamp;
- priority if explicitly configured.

---

176. Security of interpolation

Calibration interpolation MUST validate:

- domain;
- extrapolation policy;
- monotonicity where required;
- finite output;
- resource count.

Out-of-domain values MUST produce an explicit result rather than silent extrapolation unless policy permits it.

---

177. Security of drift models

Drift models can amplify small malicious inputs over long execution periods.

They MUST have:

- explicit time domain;
- bounded evaluation;
- finite output checks;
- overflow protection.

---

178. Security of non-Markovian history

History-dependent models must never grow indefinitely without policy.

Use:

bounded history
compressed history
streaming state
checkpointed state

as appropriate.

---

179. Security of distributed correlations

Distributed correlations may require large shared state.

The implementation must not assume all resources are locally materializable.

Use:

- partitioned representations;
- lazy evaluation;
- explicit communication boundaries;
- bounded state.

Network communication remains outside ZQN semantic ownership.

---

180. Security of distributed replay

For deterministic distributed simulation:

global execution identity
+
logical node identity
+
resource identity
+
operation identity
+
shot/event

must determine random events.

Transport packet timing MUST NOT determine random outcomes.

---

181. Security of speculative distributed execution

Speculative work must use independent randomness addresses.

Canceled speculative work must not change committed execution.

---

182. Security of checkpoints

Checkpoint files are untrusted inputs when restored.

They MUST be:

- versioned;
- validated;
- bounded;
- integrity checked where required;
- compatibility checked.

A corrupted checkpoint must fail closed.

---

183. Checkpoint rollback attacks

A runtime should prevent an attacker from repeatedly replaying an old checkpoint to bypass execution policy where such statefulness matters.

This is primarily a runtime/application concern, but ZQN checkpoint identity must make replay detectable.

---

184. Security of compatibility profiles

Every compatibility profile must specify:

- old protocol;
- new protocol;
- semantic differences;
- exact compatibility guarantees;
- known incompatibilities.

Do not call two versions "compatible" merely because both deserialize.

---

185. Security of legacy QEC compatibility

If a legacy QEC deterministic sampler is retained:

LegacyQecV1

or an equivalent explicit profile should identify its algorithm.

This allows:

old reproducibility

to remain distinguishable from:

new ZQN reproducibility

without corrupting either contract.

---

186. Security of API evolution

Public security-sensitive types must evolve conservatively.

Breaking changes include:

- changing canonical serialization;
- changing identity derivation;
- changing deterministic sampling;
- changing validation semantics;
- changing approximation semantics.

Such changes require protocol/version documentation.

---

187. Security review gates

A ZQN change affecting any of the following MUST receive security review:

- serialization;
- canonicalization;
- hashing;
- deterministic sampling;
- allocation;
- numerical validation;
- calibration;
- target lowering;
- hardware integration;
- external input;
- capability checking;
- resource limits.

---

188. Required CI security gates

Production CI SHOULD run:

cargo fmt --check
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings

plus:

- unsafe-code enforcement;
- dependency auditing;
- fuzzing where configured;
- property tests;
- determinism tests;
- serialization compatibility tests;
- security regression tests.

---

189. Dependency auditing

Dependency vulnerabilities MUST be reviewed before production releases.

Security-sensitive dependencies should be pinned through the repository's normal dependency-management policy.

Do not automatically upgrade cryptographic/numerical dependencies without compatibility testing.

---

190. Reproducible security builds

Where production deployment requires reproducibility, record:

Rust version
Cargo.lock
target
features
dependency versions
ZQN protocol version

This enables security investigations to reconstruct the exact implementation environment.

---

191. Supply-chain security

Production ZQN builds SHOULD use:

- verified dependency sources;
- locked versions;
- dependency auditing;
- reproducible build procedures;
- controlled CI runners.

Avoid unreviewed build scripts.

---

192. Build-script restrictions

ZQN should avoid build scripts unless necessary.

A build script must not become a hidden mechanism for:

- downloading executable code;
- executing arbitrary network content;
- changing semantic source behavior.

---

193. Security of generated code

If any ZQN code is generated:

- generated sources must be reviewed;
- generation must be deterministic;
- generated code must contain no unsafe code;
- generation inputs must be versioned.

---

194. Security of documentation-driven schemas

Documentation MUST NOT be the only source of validation rules.

Every normative security rule in this file that affects execution must eventually have:

implementation
+
test

This document defines the contract; code enforces it.

---

195. Security ownership by file

"core/error.rs"

Owns structured security-related errors.

Must include:

- resource exhaustion;
- numerical failure;
- invalid input;
- serialization failure;
- determinism violation;
- compatibility failure.

Does not own policy decisions.

---

"core/ids.rs"

Owns canonical ZQN identities.

Must integrate with:

quantum::ir::qubit::{QubitId, PhysicalQubitId}

where appropriate.

Does not own qubit semantics.

---

"core/metadata.rs"

Owns safe descriptive metadata.

Must not turn arbitrary metadata into executable behavior.

---

"core/version.rs"

Owns protocol/schema versions.

Security-sensitive compatibility depends on it.

---

"core/context.rs"

Owns explicit execution/security context.

It must carry relevant:

- limits;
- capabilities;
- calibration;
- determinism;
- cancellation;
- provenance.

It must not access globals.

---

"core/capabilities.rs"

Owns ZQN semantic capability declarations.

Capabilities must be explicit and validated.

---

"core/limits.rs"

Owns resource policies.

No fixed semantic machine-size maximums.

---

"core/provenance.rs"

Owns provenance and audit identity.

No sensitive secrets.

---

196. Probability security ownership

"probability/probability.rs"

Owns validated probability primitives.

Reject invalid numerical values.

---

"probability/distribution.rs"

Owns safe distribution abstraction.

Protect against enormous support.

---

"probability/categorical.rs"

Owns bounded deterministic categorical sampling.

Outcome ordering must be canonical.

---

"probability/continuous.rs"

Owns continuous distributions.

Parameter validation and convergence limits are mandatory.

---

"probability/bounds.rs"

Owns mathematical bounds.

No silent widening/narrowing.

---

"probability/statistics.rs"

Owns statistical calculations.

Must detect invalid sample counts and numerical failure.

---

197. Channel security ownership

"channel/channel.rs"

Owns universal channel contract.

Must require validation before application.

---

"channel/representation.rs"

Owns representation selection.

Must not bypass validation.

---

"channel/kraus.rs"

Owns safe Kraus representation.

Dimension and allocation checks mandatory.

---

"channel/choi.rs"

Owns safe Choi representation.

Complete positivity and dimension validation required.

---

"channel/process_matrix.rs"

Owns general process representations.

Must enforce resource limits.

---

"channel/pauli.rs"

Owns Pauli specialization.

Must not become the universal noise model.

---

"channel/stochastic.rs"

Owns stochastic channels.

Must use explicit deterministic sampling.

---

"channel/lindblad.rs"

Owns continuous-time semantics.

Numerical integration belongs to execution/simulation policy.

---

"channel/thermal.rs"

Owns thermal channel semantics.

Reject invalid temperature/rate parameters.

---

"channel/amplitude.rs"

Owns amplitude-related channels.

No invalid probability coercion.

---

"channel/phase.rs"

Owns phase-related channels.

No invalid parameter coercion.

---

"channel/depolarizing.rs"

Owns depolarizing specialization.

The current implementation must remain dimension-derived rather than fixed-size. Existing search results show it is already structured around qubit-based construction, which must not become a semantic restriction.

---

"channel/generalized.rs"

Owns generalized channels.

Must not assume qubits.

---

"channel/composition.rs"

Owns safe composition.

Must estimate dimension/resource growth before materialization.

---

198. Fault security ownership

"fault/fault.rs"

Universal fault validation.

---

"fault/location.rs"

Canonical resource location validation.

---

"fault/classification.rs"

Safe classification.

---

"fault/correlated.rs"

Arbitrary correlation validation with resource limits.

---

"fault/leakage.rs"

Leakage validation and deterministic construction.

---

"fault/erasure.rs"

Erasure validation.

---

"fault/loss.rs"

Loss validation.

---

"fault/batch.rs"

Streaming and bounded fault generation.

---

199. Noise security ownership

"noise/model.rs"

Canonical model validation contract.

---

"noise/specification.rs"

Safe declarative specification.

No arbitrary execution.

---

"noise/application.rs"

Security boundary between model and execution scope.

---

"noise/composition.rs"

Safe composition.

---

"noise/correlation.rs"

Safe correlation definitions.

The existing module is explicitly responsible for declarative correlation semantics.

---

"noise/temporal.rs"

Explicit-time evaluation.

---

"noise/spatial.rs"

Arbitrary topology with bounded expansion.

---

"noise/crosstalk.rs"

Safe multi-resource interaction.

---

"noise/drift.rs"

Bounded drift evaluation.

---

"noise/non_markovian.rs"

Bounded history.

---

"noise/conditional.rs"

Restricted condition evaluation.

---

200. Operation security ownership

"operations/operation.rs"

Validated universal operation reference.

---

"operations/gate.rs"

Gate-level noise attachment.

---

"operations/preparation.rs"

Preparation noise.

The existing preparation architecture already requires explicit deterministic seed/execution context; this contract is retained.

---

"operations/reset.rs"

Reset noise.

---

"operations/measurement.rs"

Readout and measurement noise.

---

"operations/idle.rs"

Time-dependent idle noise.

---

"operations/pulse.rs"

Pulse-level noise.

---

"operations/transport.rs"

Transport/link noise.

---

201. Calibration security ownership

Each calibration file must validate its own domain.

No calibration file may assume another module has already validated it.

---

202. Characterization security ownership

Each characterization component must:

- validate input;
- enforce work limits;
- retain provenance;
- expose uncertainty;
- support cancellation;
- remain deterministic under deterministic policy.

---

203. Simulation security ownership

"simulation/engine.rs"

Owns execution orchestration.

---

"simulation/sampler.rs"

Owns safe sampler interface.

---

"simulation/trajectory.rs"

Owns bounded trajectory execution.

---

"simulation/channel_engine.rs"

Owns channel application safety.

---

"simulation/monte_carlo.rs"

Owns bounded shot execution.

---

"simulation/deterministic.rs"

Owns deterministic execution mode.

---

"simulation/reproducibility.rs"

Owns deterministic random addressing/replay.

This file must integrate directly with "DETERMINISM.md".

---

204. Propagation security

"propagation/error_budget.rs"

Must not permit negative or invalid budgets.

---

"propagation/uncertainty.rs"

Must validate uncertainty.

---

"propagation/fidelity.rs"

Must validate metric domains.

---

"propagation/bounds.rs"

Must preserve mathematical bounds.

---

"propagation/sensitivity.rs"

Must bound expensive analyses.

---

"propagation/accumulation.rs"

Must protect against overflow and excessive expansion.

---

205. Target security

"target/requirements.rs"

Defines what the program requests.

---

"target/capabilities.rs"

Defines what the target supports.

---

"target/compatibility.rs"

Performs explicit compatibility validation.

---

"target/lowering.rs"

Performs semantics-preserving lowering.

---

"target/validation.rs"

Fails closed on unsupported semantics.

---

206. Integration security

"integration/ir.rs"

Canonical IR boundary.

No source AST dependency.

---

"integration/routing.rs"

Read-only ZQN cost/profile interface.

---

"integration/scheduling.rs"

Read-only noise/timing interface.

---

"integration/qec.rs"

ZQN → QEC adapter.

---

"integration/hardware.rs"

Hardware → validated ZQN observations/capabilities.

---

"integration/memory.rs"

Validated channel/fault application.

---

"integration/benchmarking.rs"

Benchmark reproducibility/provenance.

---

"integration/runtime.rs"

Execution/security context provider.

---

207. IO security ownership

"io/schema.rs"

Versioned schema.

---

"io/serialization.rs"

Bounded serialization.

---

"io/deserialization.rs"

Hostile-input boundary.

---

"io/canonical.rs"

Canonical identity bytes.

---

"io/compatibility.rs"

Explicit migrations and downgrade protection.

---

208. Public API security

The stable ZQN API SHOULD expose only validated types.

Prefer:

ValidatedProbability
ValidatedDistribution
ValidatedChannel
ValidatedNoiseModel
ValidatedCalibration
ValidatedFault

or equivalent invariant-preserving constructors.

Do not make invalid internal states publicly constructible merely for convenience.

---

209. Constructor security

Constructors receiving untrusted values MUST validate them.

Prefer:

new(...)

returning:

Result<T, ZqnError>

when validation can fail.

---

210. Mutation policy

Prefer immutable objects.

If mutation is necessary:

- validate before mutation;
- maintain invariants after mutation;
- prevent partially valid state;
- do not expose raw mutable internals.

---

211. Builder security

Builders must not permit execution before validation completes.

For example:

builder
 ↓
validate
 ↓
build
 ↓
immutable validated model

is preferred.

---

212. Security of cloning

Cloning a validated immutable object must preserve its security invariants.

A clone must not acquire a new semantic identity merely because it is a separate Rust object.

---

213. Security of equality

Semantic equality MUST be distinct from:

pointer equality

and should be based on validated semantic content.

---

214. Security of ordering

If objects are ordered for:

- canonical serialization;
- deterministic processing;
- hashing;

the ordering must be explicit and stable.

Never rely on arbitrary hash ordering.

---

215. HashMap policy

"HashMap" MAY be used internally for performance.

However:

- never serialize it directly as canonical output;
- never derive semantic identity from iteration order;
- never derive deterministic random streams from iteration order.

Canonical ordering must be applied first.

---

216. BTreeMap policy

Ordered collections SHOULD be preferred where ordering itself is semantically relevant.

The choice of collection remains an implementation detail provided the semantic ordering is stable.

---

217. Security of external identifiers

External provider identifiers must be treated as opaque data.

Do not embed provider-specific parsing assumptions in ZQN core.

---

218. Security of vendor adapters

Vendor adapters must perform:

external data
 ↓
adapter validation
 ↓
abstract target/calibration/observation
 ↓
ZQN validation

No vendor adapter may bypass ZQN validation.

---

219. Security of backend-specific noise

Hardware-native noise may contain provider-specific fields.

These must be represented through extensible metadata/typed extension boundaries rather than arbitrary executable payloads.

---

220. Security of future extensibility

Future modalities and representations must be able to plug into:

NoiseLocation
NoiseModel
QuantumChannel
Distribution
Calibration
TargetCapabilities

without weakening existing validation.

New types must inherit the same:

- resource limits;
- numerical validation;
- serialization;
- determinism;
- provenance;
- error contracts.

---

221. Security of "write once, scale everywhere"

The security interpretation of the Zamani scalability requirement is:

«A program must not become less secure merely because it is executed on a larger target.»

Therefore:

1 resource
10 resources
10^3 resources
10^6 resources
distributed resources

must use the same security architecture.

Only the resource policy may differ.

---

222. Infinity clarification

"Scale from atom to infinity" means:

«ZQN has no arbitrary semantic maximum imposed by its architecture.»

It does not mean:

«Rust, memory, CPUs, storage or quantum hardware are physically infinite.»

Concrete executions are bounded by:

- available memory;
- compute;
- hardware;
- storage;
- runtime policy;
- numerical representation;
- network capacity.

When those resources are insufficient, ZQN must fail safely or use an explicitly requested approximation.

---

223. No hard-coded technology assumptions

Security code MUST NOT contain logic such as:

if qubit_count == 127
if topology == ...
if vendor == ...
if backend == ...

unless the condition represents an explicit target capability supplied by the target layer.

---

224. Security invariant across technologies

The following must all pass through the same security boundary:

superconducting
ion-trap
neutral-atom
photonic
spin
bosonic
continuous-variable
annealing
analog
distributed
future technologies

ZQN's security model is semantic rather than vendor-specific.

---

225. Security of measurement noise

Measurement models MUST validate:

- assignment matrices;
- probability values;
- correlated outcomes;
- output dimensions;
- measurement-resource identity.

No measurement model may write arbitrary classical output.

---

226. Security of readout correlations

Correlated readout can create large joint distributions.

Use:

- sparse representation;
- factorization;
- streaming;
- bounded materialization.

Never blindly construct the full joint distribution.

---

227. Security of leakage

Leakage models may introduce states outside the computational subspace.

Validation must ensure that:

- leakage states are valid;
- dimensions remain consistent;
- simulation policy supports them;
- unsupported targets reject them.

Never silently collapse leakage back into ordinary qubit errors.

---

228. Security of erasure

Erasure events must retain explicit semantics.

Do not silently reinterpret erasure as:

bit flip

or:

measurement error

unless an explicit compatibility policy states this.

---

229. Security of loss

Transport/loss models must preserve the distinction between:

lost resource

and:

ordinary quantum error

because downstream systems may have different security and recovery behavior.

---

230. Security of transport

"operations/transport.rs" must validate:

- source resource;
- destination resource;
- link identity;
- duration;
- loss model;
- calibration;
- target capability.

---

231. Security of analog models

Analog models may contain continuous functions and parameters.

They require:

- finite-value checks;
- bounded evaluation;
- explicit numerical policy;
- cancellation;
- no arbitrary host execution.

---

232. Security of Hamiltonian models

Hamiltonian parameters must be validated for:

- dimensions;
- finite coefficients;
- representation size;
- numerical stability;
- execution cost.

---

233. Security of bosonic/CV models

Bosonic and continuous-variable representations may require large/infinite-dimensional mathematical spaces.

ZQN MUST use explicit truncation/approximation semantics.

It must never pretend that a finite truncation is exact unless mathematically justified.

---

234. Security of tensor networks

Tensor-network input can cause catastrophic bond-dimension growth.

Before contraction:

estimate
 ↓
limit
 ↓
execute

must be used.

---

235. Security of symbolic models

Symbolic expressions can suffer from expression explosion.

Protect against:

- repeated substitution;
- recursive expansion;
- huge intermediate expressions;
- simplification loops.

Use bounded symbolic evaluation.

---

236. Security of exact arithmetic

Exact arithmetic may cause enormous numerator/denominator growth.

It still requires resource accounting.

"Exact" does not mean "unbounded resource consumption is acceptable."

---

237. Security of approximate arithmetic

Approximate arithmetic must carry:

- precision;
- tolerance;
- numerical profile;
- error semantics.

Do not hide reduced precision.

---

238. Security of precision selection

An attacker must not force a system into an unexpectedly expensive arbitrary-precision calculation without resource policy.

Precision is a resource.

---

239. Security of adaptive algorithms

Adaptive algorithms must use explicit execution context.

If an adaptive choice depends on:

measurement result

that is semantic.

If it depends on:

worker availability

that is not semantic and must not silently affect deterministic results.

---

240. Security of scheduler-dependent noise

If scheduling changes physical noise because it changes:

idle duration

then schedule identity must be part of the physical execution context.

Security must never pretend two different physical schedules are the same execution.

---

241. Security of routing-dependent noise

If routing changes:

physical resource

then target/resource identity changes.

ZQN must retain that identity in the execution/provenance context.

---

242. Security of semantic equivalence

An optimization may transform the IR while preserving ideal semantics.

It may nevertheless change the physical noise realization.

Therefore ZQN must distinguish:

ideal semantic equivalence

from:

physical noise equivalence

This prevents unsafe assumptions during optimization.

---

243. Optimization integration

Optimization passes must not bypass ZQN's security checks.

After a transformation affecting:

- operation identity;
- resource mapping;
- duration;
- channel;
- measurement;
- pulse;
- calibration;

the resulting execution context must be revalidated.

---

244. Security of operation identity

Operation IDs must remain stable enough for deterministic replay.

If an optimization changes operation identity, it must establish an explicit mapping from old semantic operation identity to new identity where replay compatibility is required.

---

245. Security of source-to-hardware provenance

The system should be able to trace:

Zamani source
 ↓
canonical IR
 ↓
optimized IR
 ↓
routing
 ↓
schedule
 ↓
ZQN model
 ↓
target
 ↓
execution

without requiring ZQN to own every upstream layer.

---

246. Security manifest

Production execution SHOULD generate a security/reproducibility manifest containing:

ZQN protocol version
schema version
program identity
noise model identity
configuration identity
target identity
calibration identity
determinism profile
seed policy
numerical profile
approximation policy
resource policy identity
compiler version
dependency/build identity

---

247. Manifest integrity

The manifest itself should have a canonical serialization.

Its identity should be derived from canonical content.

If integrity protection is required, the surrounding security layer should authenticate/sign it.

---

248. Security profile levels

ZQN SHOULD support explicit deployment profiles such as:

Development

- extensive diagnostics;
- fuzzing;
- verbose validation.

Production

- strict validation;
- bounded resources;
- safe errors;
- audit logging.

High assurance

- strict determinism;
- pinned numerical backend;
- verified dependencies;
- signed provenance;
- strict schema compatibility;
- hardened resource limits.

The exact profile must be explicit rather than inferred from environment.

---

249. Fail-closed policy

When ZQN cannot determine whether an operation is safe, semantically valid or compatible, it must fail closed.

Examples:

unknown schema
unknown capability
invalid calibration
invalid dimension
invalid probability
unsupported approximation
unverified integrity

must not silently continue.

---

250. Fail-safe resource policy

When resource limits are exceeded:

return ResourceLimitExceeded

rather than:

- truncate;
- skip;
- sample less;
- silently approximate;
- reduce precision.

Any such alternative requires explicit policy.

---

251. No silent truncation

Never silently turn:

1,000,000 requested shots

into:

10,000 shots

because of resource pressure.

The result would otherwise be scientifically/security misleading.

---

252. No silent sampling reduction

Likewise:

requested correlated model

must not silently become:

independent approximation

unless explicitly configured.

---

253. Security of defaults

Secure defaults should be:

- validation enabled;
- finite values required;
- resource limits available;
- deterministic mode available;
- no network access;
- no filesystem access;
- no arbitrary execution;
- no global RNG;
- no unsafe code;
- no silent approximation.

---

254. Production release checklist

Before a ZQN release:

Code safety

- [ ] No unsafe code.
- [ ] No unsafe FFI in core ZQN.
- [ ] No unchecked attacker-controlled indexing.
- [ ] No uncontrolled allocation.

Numerical safety

- [ ] NaN rejected.
- [ ] Infinite values handled explicitly.
- [ ] Overflow checked.
- [ ] Invalid probabilities rejected.
- [ ] Approximation explicit.

Input security

- [ ] Deserialization bounded.
- [ ] Recursive structures bounded.
- [ ] Schema versions validated.
- [ ] Calibration validated.
- [ ] External observations validated.

Determinism

- [ ] No hidden RNG.
- [ ] Randomness is addressable.
- [ ] Parallelism independent.
- [ ] Retry independent.
- [ ] Checkpoint reproducibility tested.

Scalability

- [ ] No semantic maximum qubit count.
- [ ] No fixed correlation size.
- [ ] Streaming supported.
- [ ] Large allocations bounded by policy.
- [ ] Resource limits configurable.

Integration

- [ ] Canonical "quantum::ir" identities used.
- [ ] QEC adapter validated.
- [ ] Routing integration validated.
- [ ] Scheduling integration validated.
- [ ] Hardware integration validated.
- [ ] Benchmark integration validated.
- [ ] Runtime integration validated.

Serialization

- [ ] Canonical serialization.
- [ ] Versioned schema.
- [ ] Compatibility tests.
- [ ] Cache identity tests.

Testing

- [ ] Unit tests.
- [ ] Property tests.
- [ ] Fuzz tests.
- [ ] Differential tests.
- [ ] Determinism tests.
- [ ] Scaling tests.
- [ ] Security regression tests.

---

255. Required security regression suite

Every previously discovered security defect MUST become a permanent regression test.

The test must fail before the fix and pass afterward.

Never rely solely on a changelog entry.

---

256. Security issue severity

ZQN issues SHOULD be classified as:

Critical

Examples:

- arbitrary code execution;
- credential exposure;
- unsafe memory corruption;
- security-boundary bypass;
- arbitrary host access.

High

Examples:

- uncontrolled resource exhaustion;
- calibration/model substitution;
- cache poisoning affecting execution;
- deterministic execution corruption;
- target capability bypass.

Medium

Examples:

- information disclosure;
- incomplete validation;
- incorrect provenance;
- unsafe but bounded numerical behavior.

Low

Examples:

- diagnostic leakage;
- documentation security omissions;
- minor hardening gaps.

---

257. Vulnerability response

Security vulnerabilities should be reported privately through the repository's established security reporting mechanism.

Do not publish exploitable details in a normal public issue before remediation where responsible disclosure is appropriate.

The repository-level security policy is authoritative for contact/reporting details.

This ZQN document defines the technical security requirements, not the repository's external disclosure address.

---

258. Security advisory requirements

A ZQN security advisory should identify:

- affected versions;
- affected files;
- affected API;
- attack prerequisites;
- impact;
- mitigation;
- fixed version;
- regression test;
- compatibility consequences.

---

259. Security review ownership

Changes to the following files should automatically trigger review:

core/error.rs
core/ids.rs
core/version.rs
core/context.rs
core/limits.rs
core/provenance.rs

probability/*
channel/*
fault/*
noise/*

calibration/*
simulation/*
target/*
integration/*
io/*

when the change affects security semantics.

---

260. Documentation contract for every source file

Every ZQN source file MUST document:

Ownership
Non-ownership
Public API
Invariants
Dependencies
Consumers
Integration
Error contract
Resource contract
Determinism contract
Serialization contract
Thread-safety contract
Scaling contract
Security contract
Tests

This ensures that a file can be completed against a fixed contract without being reopened merely because another module was implemented later.

---

261. Mandatory source-file security header

ZQN source files SHOULD begin with documentation following this pattern:

//! # Security
//!
//! This module processes validated/untrusted ...
//!
//! # Ownership
//!
//! This module owns ...
//!
//! # Does not own
//!
//! This module does not own ...
//!
//! # Invariants
//!
//! ...
//!
//! # Resource safety
//!
//! ...
//!
//! # Determinism
//!
//! ...
//!
//! # Integration
//!
//! ...
//!
//! # Errors
//!
//! ...
//!
//! # Serialization
//!
//! ...
//!
//! # Testing
//!
//! ...

---

262. Integration contract with "DETERMINISM.md"

"SECURITY.md" and "DETERMINISM.md" are complementary.

"DETERMINISM.md" owns:

- deterministic semantics;
- seed derivation;
- random-address derivation;
- replay;
- parallel determinism;
- canonical execution identity.

"SECURITY.md" owns:

- abuse resistance;
- resource exhaustion;
- hostile input;
- integrity;
- privilege separation;
- validation;
- secure failure.

Neither document may redefine the other's protocol.

---

263. Integration contract with "SCALABILITY.md"

"SCALABILITY.md" owns:

- system-size scalability;
- representation scalability;
- streaming;
- distributed scaling;
- no artificial semantic maximums.

"SECURITY.md" owns the corresponding security requirement:

«Resource exhaustion controls must not become artificial semantic limits.»

---

264. Integration contract with "SEMANTICS.md"

"SEMANTICS.md" owns the mathematical meaning of ZQN.

"SECURITY.md" requires:

«Security validation must preserve those semantics and must never silently change them.»

---

265. Integration contract with "ARCHITECTURE.md"

"ARCHITECTURE.md" owns module boundaries.

"SECURITY.md" enforces:

no privilege escalation across boundaries
no circular security dependencies
no hidden global state

---

266. Integration contract with "COMPATIBILITY.md"

"COMPATIBILITY.md" owns version compatibility.

"SECURITY.md" requires:

- no unsafe downgrade;
- explicit migration;
- canonical version identity;
- rejection of ambiguous versions.

---

267. Integration with the existing ZQN implementation

The repository already contains ZQN modules under:

src/quantum/zqn/

including:

noise/model.rs
noise/specification.rs
noise/application.rs
noise/composition.rs
noise/correlation.rs
noise/temporal.rs
noise/spatial.rs
noise/crosstalk.rs
noise/drift.rs
noise/non_markovian.rs
noise/conditional.rs
fault/leakage.rs
probability/*
channel/*
operations/*
integration/*

The security policy applies to all of them.

The existing implementation must be brought into conformance rather than creating a parallel security subsystem. The repository search confirms these modules already have explicit ownership boundaries, which is the correct foundation.

---

268. Security migration strategy

Implement security in this order:

Stage 1 — Foundation

Complete:

core/error.rs
core/version.rs
core/ids.rs
core/limits.rs
core/context.rs
core/provenance.rs

Security outcome:

validated errors
stable identities
explicit limits
explicit context
provenance

---

Stage 2 — Canonical input security

Complete:

io/schema.rs
io/canonical.rs
io/serialization.rs
io/deserialization.rs
io/compatibility.rs

Security outcome:

bounded hostile-input boundary
canonical identity
versioned data

---

Stage 3 — Numerical security

Complete:

probability/*
channel/*

Security outcome:

no NaN
no invalid probability
no overflow
no allocation bombs

---

Stage 4 — Noise/fault security

Complete:

fault/*
noise/*
operations/*

Security outcome:

safe physical noise semantics

---

Stage 5 — Calibration security

Complete:

calibration/*
characterization/*

Security outcome:

validated physical evidence
immutable calibration identity
provenance

---

Stage 6 — Deterministic simulation security

Complete:

simulation/reproducibility.rs
simulation/sampler.rs
simulation/deterministic.rs
simulation/monte_carlo.rs
simulation/trajectory.rs

Security outcome:

parallel-safe
retry-safe
checkpoint-safe
resource-bounded

---

Stage 7 — Target security

Complete:

target/*

Security outcome:

explicit capability validation
no silent downgrade

---

Stage 8 — Integration security

Complete:

integration/*

Security outcome:

privilege-separated
validated
cross-subsystem execution

---

Stage 9 — Security testing

Complete:

tests/property
tests/determinism
tests/scaling
tests/compatibility
tests/integration

plus fuzzing.

---

269. Definition of security-complete

ZQN is security-complete only when all of the following are true:

                 ┌───────────────────────┐
                 │ no unsafe code        │
                 └───────────┬───────────┘
                             │
                 ┌───────────▼───────────┐
                 │ hostile input safe    │
                 └───────────┬───────────┘
                             │
                 ┌───────────▼───────────┐
                 │ resource bounded      │
                 └───────────┬───────────┘
                             │
                 ┌───────────▼───────────┐
                 │ numerical validation  │
                 └───────────┬───────────┘
                             │
                 ┌───────────▼───────────┐
                 │ deterministic         │
                 └───────────┬───────────┘
                             │
                 ┌───────────▼───────────┐
                 │ canonical identity    │
                 └───────────┬───────────┘
                             │
                 ┌───────────▼───────────┐
                 │ provenance            │
                 └───────────┬───────────┘
                             │
                 ┌───────────▼───────────┐
                 │ capability validated  │
                 └───────────┬───────────┘
                             │
                 ┌───────────▼───────────┐
                 │ integration tested    │
                 └───────────┬───────────┘
                             │
                 ┌───────────▼───────────┐
                 │ fuzz/property tested  │
                 └───────────────────────┘

---

270. Final security principles

ZQN MUST permanently follow these rules:

1. No "unsafe".
2. No hidden global state.
3. No hidden RNG.
4. No arbitrary code execution from data.
5. No silent approximation.
6. No silent truncation.
7. No silent numerical repair.
8. No unchecked allocation.
9. No unchecked arithmetic.
10. No arbitrary semantic maximum qubit count.
11. No fixed correlation-size assumption.
12. No vendor-specific trust inside ZQN.
13. No competing "QubitId".
14. Use "quantum::ir::qubit::{QubitId, PhysicalQubitId}" where appropriate.
15. All external data is untrusted until validated.
16. All expensive work is resource-governed.
17. All potentially unbounded work is cancellable/streamable where applicable.
18. All security-sensitive identities are canonical.
19. All deterministic randomness is explicitly addressed.
20. Parallelism must not change deterministic results.
21. Retries must not change deterministic results.
22. Checkpoints must support deterministic replay.
23. Calibration must be immutable and identifiable.
24. Provenance must survive model derivation.
25. Hardware nondeterminism must never be misrepresented as deterministic.
26. Capability mismatches must fail closed.
27. Resource limits are policies, not semantic limits.
28. The same security architecture must work from tiny systems to arbitrarily large systems permitted by available resources.
29. Security controls must not destroy the write-once/scale-everywhere property.
30. Every security rule must eventually have an implementation and regression test.

---

271. Final architecture

The resulting security boundary is:

                         Zamani Program
                               │
                               ▼
                       Canonical Quantum IR
                               │
                               ▼
                     ┌────────────────────┐
                     │        ZQN         │
                     │                    │
                     │ validation         │
                     │ probability        │
                     │ channels           │
                     │ faults             │
                     │ noise              │
                     │ calibration        │
                     │ characterization   │
                     │ determinism        │
                     │ provenance         │
                     │ resource policy    │
                     └─────────┬──────────┘
                               │
             ┌─────────────────┼──────────────────┐
             │                 │                  │
             ▼                 ▼                  ▼
          Routing          Scheduling             QEC
             │                 │                  │
             └─────────────────┼──────────────────┘
                               ▼
                        Target/Hardware
                               │
                               ▼
                         Runtime/Simulator
                               │
                               ▼
                          Observations
                               │
                 ┌─────────────┼─────────────┐
                 ▼             ▼             ▼
           Characterization Benchmarking   Analysis
                 │
                 ▼
            Calibration
                 │
                 └──────────────► ZQN

The critical security property is:

«Untrusted physical/noise data may enter ZQN, but it can never bypass validation, resource governance, deterministic identity, provenance, capability checking or explicit approximation policy before influencing execution.»

And the critical scalability property is:

«Security must scale with the workload; it must never impose an arbitrary semantic ceiling on the quantum system.»

A 2-resource computation and a trillion-resource conceptual computation therefore use the same security model. The difference is only the concrete resource policy and the resources available to execute it.

---

272. Production acceptance statement

"src/quantum/zqn/SECURITY.md" is considered satisfied only when the implementation underneath it demonstrates:

Rust 1.97 / 1.97.1
Rust 2021
unsafe forbidden
canonical IR integration
canonical QubitId/PhysicalQubitId usage
validated external input
checked arithmetic
bounded allocation
streaming
cancellation
deterministic randomness
parallel determinism
checkpoint determinism
canonical serialization
versioned compatibility
calibration integrity
provenance
capability validation
explicit approximation
QEC integration
routing integration
scheduling integration
hardware integration
benchmark integration
memory integration
runtime integration
property testing
fuzz testing
differential testing
security regression testing

Only after those conditions are met should ZQN be described as production-ready from a security perspective.Repository integration note

The important correction compared with treating SECURITY.md as a generic vulnerability-reporting document is that this version is a technical security contract for the entire ZQN subsystem. That is necessary because the repository already has substantial ZQN implementation boundaries rather than merely an empty placeholder: the current search shows dedicated model, application, composition, temporal, spatial, crosstalk, correlation, non-Markovian and conditional modules, alongside probability/channel/fault infrastructure.

It also deliberately preserves the existing repository architecture rather than creating competing concepts: quantum::ir remains authoritative for quantum-resource identity, ZQN owns noise/security semantics, QEC consumes ZQN through an adapter, and routing/scheduling consume ZQN information rather than becoming owners of the noise model.

For external security-policy practice, mature Rust cryptographic projects commonly separate a technical security policy from responsible-disclosure handling; for example, tfhe-rs uses a dedicated security policy focused on private vulnerability reporting. 