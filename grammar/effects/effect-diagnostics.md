Zamani Effect Diagnostics

Path: "grammar/effects/effect-diagnostics.md"
Status: Production-ready normative effect-diagnostic integration contract
Language: Zamani
Grammar subsystem: "grammar/effects/"
Implementation baseline: Rust 1.97 / Rust 1.97.1, Edition 2021
Safety: "unsafe" prohibited
Portability model: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Scalability model: no artificial language-level machine, hardware, topology, resource, effect, or diagnostic cardinality limits
Canonical quantum semantic boundary: "quantum::ir"

---

1. Purpose

This file defines the effect-specific diagnostic contract for the Zamani language.

It does not create a second diagnostic framework.

The authoritative general diagnostic architecture remains:

grammar/spec/diagnostics.md

This file specializes that architecture for:

- effect declarations;
- effect references;
- effect sets;
- effect operations;
- effect invocation;
- effect handlers;
- handler patterns;
- handler guards;
- resumptions;
- effect composition;
- effect polymorphism;
- effect inference;
- effect-row compatibility;
- effect implementation contracts;
- effect/capability separation;
- effect/resource separation;
- effect/requirement separation;
- effect/constraint separation;
- effect/target separation;
- cross-domain effects;
- effect lowering;
- effect-related compatibility;
- effect-related diagnostics emitted by later compiler stages.

The central rule is:

«Effect diagnostics report effect-language and effect-analysis facts; they do not redefine effect semantics, capabilities, resources, target selection, runtime behavior, or domain-specific IR semantics.»

---

2. Authority hierarchy

Effect diagnostics participate in the existing repository authority model.

The authority chain is:

grammar/DESIGN.md
        │
        ▼
grammar/spec/diagnostics.md
        │
        ▼
grammar/spec/effects.md
        │
        ▼
grammar/effects/effect-diagnostics.md
        │
        ├── grammar/effects/effects.g4
        ├── grammar/effects/effect-declarations.g4
        ├── grammar/effects/effect-sets.g4
        ├── grammar/effects/effect-handling.g4
        ├── grammar/effects/custom-effects.g4
        ├── grammar/effects/effect-composition.g4
        └── grammar/effects/effect-polymorphism.g4
        │
        ▼
frontend AST
        │
        ▼
semantic effect analysis
        │
        ├── name resolution
        ├── type analysis
        ├── effect analysis
        ├── capability analysis
        ├── resource analysis
        ├── ownership analysis
        └── domain analysis
        │
        ▼
canonical semantic representation
        │
        ├── classical representation
        ├── quantum::ir
        ├── HDL/hardware representation
        └── other domain IR
        │
        ▼
optimization / lowering
        │
        ├── routing
        ├── scheduling
        ├── resilience
        ├── QEC
        ├── ZQN
        └── HAL
        │
        ▼
target / runtime diagnostics

No lower layer may silently redefine this file's effect-diagnostic meanings.

"grammar/grammar.md" documents implementation conformance.

"grammar/Zamani-Grammar.md" may contain historical or proposed diagnostic material.

Neither may override the normative contract established by:

grammar/spec/diagnostics.md
grammar/spec/effects.md
grammar/effects/effect-diagnostics.md

---

3. Ownership

3.1 This file owns

This file owns:

- effect diagnostic categories;
- effect diagnostic code namespace;
- effect-specific diagnostic classification;
- effect-specific diagnostic payload requirements;
- effect-specific diagnostic source-location rules;
- effect-specific diagnostic relationships;
- effect-specific diagnostic severity guidance;
- effect-specific diagnostic recovery classification;
- effect-specific diagnostic deterministic ordering;
- effect-specific diagnostic suppression behavior;
- effect-specific diagnostic test requirements;
- effect-specific diagnostic integration contracts;
- effect-specific distinction between semantic errors and target/resource failures.

3.2 This file does not own

This file does not own:

- lexical token definitions;
- parser grammar;
- general source-span semantics;
- general diagnostic rendering;
- general diagnostic serialization;
- type-system rules;
- ownership rules;
- capability semantics;
- resource allocation;
- hardware discovery;
- target selection;
- scheduling;
- routing;
- QEC;
- ZQN;
- HAL;
- runtime dispatch;
- quantum gate semantics;
- "quantum::ir";
- vendor APIs;
- backend APIs.

Those remain owned by their respective repository components.

---

4. Required existing integrations

The effect-diagnostic implementation MUST integrate with the existing repository rather than introducing parallel infrastructure.

4.1 General diagnostics

Use:

grammar/spec/diagnostics.md

for:

- diagnostic structure;
- severity;
- diagnostic identity;
- diagnostic phases;
- source locations;
- primary/secondary labels;
- notes;
- help;
- suggestions;
- fix-its;
- deterministic ordering;
- diagnostic budgets;
- serialization;
- tooling integration.

Effect diagnostics MUST use that model.

---

4.2 Source spans

Use the existing source-span contract:

grammar/spec/source-spans.md

Effect diagnostics MUST preserve:

source identity
+
byte start
+
byte end

and MUST NOT invent an effect-specific span representation.

The primary diagnostic span should identify the smallest useful source construct responsible for the problem.

---

4.3 Effect grammar

The syntax authorities remain:

grammar/effects/effects.g4
grammar/effects/effect-declarations.g4
grammar/effects/effect-sets.g4
grammar/effects/effect-handling.g4
grammar/effects/custom-effects.g4
grammar/effects/effect-composition.g4
grammar/effects/effect-polymorphism.g4

This file does not duplicate their productions.

---

4.4 Frontend AST

The existing native frontend effect representation is source-level and domain-neutral.

The effect AST represents structures such as:

Effect
├── source Node
├── effect identity
├── generic parameter references
├── parameter references
└── optional return-type reference

Effect diagnostics MUST reference AST source information and semantic identities without modifying the AST into a backend-specific representation.

The effect AST MUST NOT acquire:

- physical device IDs;
- QPU IDs;
- physical qubit IDs;
- CPU IDs;
- GPU IDs;
- FPGA IDs;
- backend handles;
- runtime handles;
- routing decisions;
- scheduling decisions.

---

5. Effect diagnostic architecture

The effect diagnostic pipeline is:

source
  │
  ▼
lexer
  │
  ▼
parser
  │
  ▼
effect AST
  │
  ▼
structural validation
  │
  ▼
name resolution
  │
  ▼
type analysis
  │
  ▼
effect analysis
  │
  ├── effect identity
  ├── effect sets
  ├── effect operations
  ├── effect polymorphism
  ├── effect composition
  ├── handlers
  └── resumptions
  │
  ├───────────────┬────────────────┐
  ▼               ▼                ▼
capability      resource        ownership
analysis        analysis        analysis
  │               │                │
  └───────────────┼────────────────┘
                  ▼
          semantic representation
                  │
        ┌─────────┼──────────┐
        ▼         ▼          ▼
   classical  quantum::ir   HDL/domain IR
        │         │          │
        └─────────┼──────────┘
                  ▼
       downstream compilation
                  │
        routing/scheduling/QEC/
        ZQN/HAL/target/runtime

A diagnostic MUST be emitted by the earliest subsystem that can establish the condition correctly.

For example:

unknown effect name

belongs to effect/name analysis.

It MUST NOT be deferred to hardware lowering merely because the effect might eventually map to hardware.

---

6. Fundamental semantic distinctions

Effect diagnostics MUST preserve these distinctions.

Effect
    ≠
Capability
    ≠
Resource
    ≠
Requirement
    ≠
Constraint
    ≠
Preference
    ≠
Hint
    ≠
Target
    ≠
Implementation decision

6.1 Effect

An effect describes computational interaction or externally observable computational behavior.

Example:

effect storage::Read;

6.2 Capability

A capability describes something an execution environment can provide.

Example:

requires capability("quantum.measurement")

6.3 Resource

A resource describes computational capacity or an abstract resource.

Example:

requires qubits >= n

6.4 Requirement

A requirement is something a valid realization must satisfy.

6.5 Constraint

A constraint restricts the set of valid realizations.

6.6 Preference

A preference selects among otherwise valid realizations.

6.7 Hint

A hint suggests an implementation direction without becoming semantic necessity.

6.8 Target

A target is a downstream realization environment.

Effect diagnostics MUST NOT report a target limitation as an effect-language error when the source program itself is valid.

---

7. POCO-REAF rule

Effects MUST remain portable.

For example:

effect quantum::Measurement;

does not mean:

use physical QPU 3
use physical qubit 17
use topology X
use backend Y

Likewise:

effect accelerator::Compute;

does not mean:

use GPU 0
use 8 cores
use accelerator 2

An effect diagnostic MUST NOT encode such assumptions.

---

8. Open-world effect model

Effects are open-world.

The grammar and diagnostic system MUST NOT maintain a closed list of all legal effects.

Valid syntactic identities may include:

IO
storage::Read
network::Send
quantum::Measurement
quantum::Reset
qec::Correction
zqn::Observation
distributed::Consensus
accelerator::Compute
photonic::Interaction
neuromorphic::Spike
future::domain::Operation
vendor::extension::Operation

A new effect namespace MUST NOT require modification of this diagnostic file merely because the namespace is new.

Semantic registration may determine whether the effect is:

- known;
- unknown;
- imported;
- experimental;
- deprecated;
- unavailable;
- target-dependent.

---

9. Diagnostic identity

Effect diagnostics use the repository-wide diagnostic code model:

ZMN-EFFECT-<CONDITION>

Codes are stable machine-readable identifiers.

Human-readable diagnostic wording MAY evolve without changing the code's fundamental meaning.

The code MUST NOT encode:

- hardware model;
- vendor;
- CPU count;
- GPU count;
- QPU count;
- qubit count;
- memory size;
- topology size.

---

10. Canonical effect diagnostic categories

The following categories are reserved for effect diagnostics.

ZMN-EFFECT-UNKNOWN
ZMN-EFFECT-DUPLICATE
ZMN-EFFECT-INVALID-DECLARATION
ZMN-EFFECT-INVALID-NAME
ZMN-EFFECT-INVALID-REFERENCE
ZMN-EFFECT-UNDECLARED
ZMN-EFFECT-IMPORT-MISSING
ZMN-EFFECT-AMBIGUOUS
ZMN-EFFECT-OPERATION-UNKNOWN
ZMN-EFFECT-OPERATION-DUPLICATE
ZMN-EFFECT-OPERATION-SIGNATURE-MISMATCH
ZMN-EFFECT-ARGUMENT-COUNT
ZMN-EFFECT-ARGUMENT-TYPE
ZMN-EFFECT-RETURN-TYPE
ZMN-EFFECT-SET-MISMATCH
ZMN-EFFECT-SET-DUPLICATE
ZMN-EFFECT-SET-UNSATISFIED
ZMN-EFFECT-REQUIRED-MISSING
ZMN-EFFECT-UNEXPECTED
ZMN-EFFECT-POLYMORPHISM-MISMATCH
ZMN-EFFECT-POLYMORPHISM-UNRESOLVED
ZMN-EFFECT-ROW-MISMATCH
ZMN-EFFECT-COMPOSITION-INVALID
ZMN-EFFECT-CYCLE
ZMN-EFFECT-HANDLER-INVALID
ZMN-EFFECT-HANDLER-MISSING
ZMN-EFFECT-HANDLER-DUPLICATE
ZMN-EFFECT-HANDLER-UNREACHABLE
ZMN-EFFECT-HANDLER-SIGNATURE
ZMN-EFFECT-HANDLER-GUARD
ZMN-EFFECT-RESUMPTION-INVALID
ZMN-EFFECT-RESUMPTION-UNAVAILABLE
ZMN-EFFECT-NONRESUMABLE
ZMN-EFFECT-HANDLED
ZMN-EFFECT-UNHANDLED
ZMN-EFFECT-RECURSIVE
ZMN-EFFECT-DECLARATION-CONFLICT
ZMN-EFFECT-VISIBILITY
ZMN-EFFECT-VERSION
ZMN-EFFECT-DEPRECATED
ZMN-EFFECT-EXPERIMENTAL
ZMN-EFFECT-UNSUPPORTED-PROFILE
ZMN-EFFECT-LOWERING-UNAVAILABLE
ZMN-EFFECT-TARGET-UNSATISFIED
ZMN-EFFECT-CAPABILITY-MISSING
ZMN-EFFECT-RESOURCE-UNSATISFIED
ZMN-EFFECT-SECURITY-POLICY
ZMN-EFFECT-PORTABILITY

These are diagnostic categories, not necessarily all mandatory initially implemented codes.

A code becomes stable only after its semantics and structured payload are defined.

---

11. Diagnostic phases

Effect diagnostics SHOULD identify their phase through the general diagnostic system.

Typical phases are:

AST
NAME
TYPE
EFFECT
OWNERSHIP
RESOURCE
CAPABILITY
DOMAIN
SEMANTIC
IR_GENERATION
IR_VERIFICATION
OPTIMIZATION
ROUTING
SCHEDULING
QEC
ZQN
HAL
TARGET
RUNTIME
COMPATIBILITY

Effect-specific diagnostics must not falsely claim to originate in the "EFFECT" phase when the actual condition is target-specific.

For example:

ZMN-EFFECT-UNDECLARED
phase = EFFECT

whereas:

ZMN-EFFECT-TARGET-UNSATISFIED
phase = TARGET

may be appropriate when the effect is valid but the selected realization cannot provide it.

---

12. Declaration diagnostics

Effect declaration diagnostics concern the source declaration itself.

Example:

effect;

may produce:

ZMN-EFFECT-INVALID-DECLARATION

with the primary span anchored at the incomplete declaration.

Declaration diagnostics MUST distinguish:

syntactically malformed

from:

syntactically valid but semantically invalid

Parser syntax errors belong to the parser diagnostic domain.

Effect semantic errors belong here.

---

13. Duplicate effect declarations

If the same semantic effect identity is declared more than once in an incompatible scope:

effect storage::Read;
effect storage::Read;

the semantic analyzer MAY emit:

ZMN-EFFECT-DUPLICATE

The diagnostic SHOULD identify:

1. the current declaration as primary;
2. the earlier declaration as a secondary labeled span.

Example:

error[ZMN-EFFECT-DUPLICATE]: effect `storage::Read` is already declared

  --> module.zm:12:1
   |
12 | effect storage::Read;
   | ^^^^^^^^^^^^^^^^^^^^^ duplicate declaration

note: previous declaration is here

  --> module.zm:4:1
   |
 4 | effect storage::Read;
   | ^^^^^^^^^^^^^^^^^^^^^ previous declaration

---

14. Duplicate declarations versus overloads

A diagnostic MUST NOT incorrectly classify legitimate overloads as duplicates.

If the language permits overloaded effect operations, identity must include the semantic signature required by "grammar/spec/effects.md".

Therefore:

effect Storage {
    fn read(Key) -> Value;
    fn read(Path) -> Value;
}

must not automatically produce a duplicate diagnostic.

If overloads are not semantically legal, the semantic rule—not this diagnostics file—determines that fact.

---

15. Unknown effect

If an effect reference cannot be resolved:

with effects { storage::Missing }

the semantic analyzer may emit:

ZMN-EFFECT-UNKNOWN

or:

ZMN-EFFECT-UNDECLARED

according to the canonical effect-resolution taxonomy.

The diagnostic SHOULD contain structured data:

effect_identity
namespace
reference_span
resolution_scope
language_version

It MUST NOT embed a backend assumption.

---

16. Unknown effect operation

For:

perform storage::missing(key);

the diagnostic may be:

ZMN-EFFECT-OPERATION-UNKNOWN

The primary span SHOULD identify the operation identity:

storage::missing
^^^^^^^^^^^^^^^

rather than unnecessarily highlighting the entire invocation.

---

17. Ambiguous effect resolution

When multiple effect identities are visible and a reference cannot be uniquely resolved:

ZMN-EFFECT-AMBIGUOUS

The diagnostic SHOULD contain:

requested_identity
candidate_identities
candidate_locations
resolution_scope

Each candidate declaration SHOULD be represented as a secondary span.

---

18. Effect operation signature mismatch

For an operation such as:

perform storage::read(a, b);

when the resolved declaration expects a different signature:

ZMN-EFFECT-OPERATION-SIGNATURE-MISMATCH

The diagnostic SHOULD separate:

operation identity
actual arguments
expected signature

from ordinary type diagnostics.

If an argument's type is wrong, the canonical type diagnostic system remains responsible for the detailed type mismatch.

Effect diagnostics should explain the effect-specific context.

---

19. Argument count

When an effect operation expects a different number of arguments:

ZMN-EFFECT-ARGUMENT-COUNT

The structured payload SHOULD include:

actual_count
expected_count

provided those quantities can be represented without imposing artificial source-language limits.

The compiler MUST NOT define a universal maximum number of effect arguments.

---

20. Effect-set diagnostics

Effect sets may contain repeated identities:

with effects {
    IO,
    IO
}

If duplicate effect-set members have no semantic meaning, the compiler MAY emit:

ZMN-EFFECT-SET-DUPLICATE

A canonical semantic normalization may alternatively deduplicate them.

The chosen behavior MUST be defined by "grammar/spec/effects.md".

Diagnostics MUST not independently invent effect-set algebra.

---

21. Effect-set mismatch

If a function or computation requires:

{ IO, Storage }

but its declared effect context provides only:

{ IO }

the diagnostic may be:

ZMN-EFFECT-SET-MISMATCH

The payload SHOULD identify:

required_effects
available_effects
missing_effects
source_context

The diagnostic SHOULD identify the smallest source span representing the missing requirement.

---

22. Missing required effect

For an operation requiring effect "Storage::Read" in a context that does not permit it:

ZMN-EFFECT-REQUIRED-MISSING

This is distinct from:

ZMN-EFFECT-CAPABILITY-MISSING

because:

effect requirement

and:

execution capability

are different semantic concepts.

---

23. Unhandled effects

If the language requires an effect to be handled before a computation boundary and it is not:

ZMN-EFFECT-UNHANDLED

The diagnostic SHOULD identify:

1. the operation causing the effect;
2. the nearest relevant computation boundary;
3. the missing handler/context;
4. an optional help suggestion.

Example:

error[ZMN-EFFECT-UNHANDLED]: effect `Storage::Read` is not handled here

help: handle the effect or propagate it through the enclosing effect context

The diagnostic MUST NOT automatically assume a specific runtime mechanism.

---

24. Unexpected effects

If a declaration explicitly prohibits an effect:

with effects {}

and an operation introduces an effect not permitted by the semantic context:

ZMN-EFFECT-UNEXPECTED

The diagnostic SHOULD show:

actual effect
allowed effect set
source operation

---

25. Effect polymorphism

Effect-polymorphic functions may contain abstract effect variables.

For example:

fn map<F>(value: T) -> U
    with effects F

If effect inference cannot resolve the required relation:

ZMN-EFFECT-POLYMORPHISM-UNRESOLVED

If two effect constraints are incompatible:

ZMN-EFFECT-POLYMORPHISM-MISMATCH

Diagnostics MUST report semantic constraints rather than exposing internal inference implementation details.

---

26. Effect rows

If the language uses effect rows or an equivalent effect-set algebra, row incompatibilities may use:

ZMN-EFFECT-ROW-MISMATCH

The diagnostic SHOULD provide:

required row
provided row
missing effects
forbidden effects
originating expression

The effect-row implementation remains owned by semantic analysis.

This file specifies how its failures are communicated, not how row inference is implemented.

---

27. Effect composition

For incompatible composition:

ZMN-EFFECT-COMPOSITION-INVALID

Possible causes include:

- incompatible effect signatures;
- invalid composition relation;
- illegal handler boundary;
- incompatible effect polymorphism;
- invalid semantic effect transformation.

The diagnostic MUST identify which semantic composition rule failed.

It MUST NOT merely say:

effect composition failed

without structured context.

---

28. Effect cycles

Recursive effect dependencies MAY be valid depending on the language semantics.

Therefore a cycle MUST NOT automatically be reported as an error.

Only cycles prohibited by the normative semantic model may produce:

ZMN-EFFECT-CYCLE

When reported, the diagnostic SHOULD provide the cycle path:

A
 ↓
B
 ↓
C
 ↓
A

with source spans for the relevant declarations.

---

29. Handler diagnostics

The handler grammar is owned by:

grammar/effects/effect-handling.g4

Effect handler semantic diagnostics include:

ZMN-EFFECT-HANDLER-INVALID
ZMN-EFFECT-HANDLER-MISSING
ZMN-EFFECT-HANDLER-DUPLICATE
ZMN-EFFECT-HANDLER-UNREACHABLE
ZMN-EFFECT-HANDLER-SIGNATURE
ZMN-EFFECT-HANDLER-GUARD

Parser errors remain parser errors.

---

30. Duplicate handler arms

If two handler arms are semantically indistinguishable:

handle computation {
    case Storage::read(x) => a
    case Storage::read(y) => b
}

the semantic analyzer may emit:

ZMN-EFFECT-HANDLER-DUPLICATE

The diagnostic SHOULD identify both arms.

---

31. Unreachable handler arms

An arm that can never be selected MAY produce:

ZMN-EFFECT-HANDLER-UNREACHABLE

This SHOULD normally be a warning unless the language semantics make the condition invalid.

The compiler MUST NOT classify an arm as unreachable merely because a particular hardware target currently lacks an implementation.

---

32. Handler signature diagnostics

If a handler arm does not match the operation it claims to handle:

ZMN-EFFECT-HANDLER-SIGNATURE

The diagnostic SHOULD expose:

effect operation
expected parameters
actual handler pattern
expected result/resumption shape

---

33. Handler guard diagnostics

If a handler guard is semantically invalid:

ZMN-EFFECT-HANDLER-GUARD

The diagnostic SHOULD delegate type-specific details to the canonical type/expression diagnostic subsystem.

---

34. Resumption diagnostics

If a handler attempts to resume an operation that is not resumable:

ZMN-EFFECT-RESUMPTION-UNAVAILABLE

If the effect itself is defined as non-resumable:

ZMN-EFFECT-NONRESUMABLE

If a resumption is structurally invalid:

ZMN-EFFECT-RESUMPTION-INVALID

These diagnostics MUST be semantic.

They MUST NOT expose implementation-specific continuation objects.

---

35. Effect handler versus capability authorization

Handling an effect does not automatically authorize it.

For example:

handle quantum::Measurement(q) {
    ...
}

does not prove that the execution environment has:

capability("quantum.measurement")

If capability analysis rejects the program, the appropriate diagnostic belongs to the capability domain, such as:

ZMN-EFFECT-CAPABILITY-MISSING

or the repository's canonical capability diagnostic code.

The effect subsystem MUST preserve this separation.

---

36. Effect versus resource availability

A valid effect may require resources unavailable on a selected target.

Example:

effect quantum::LargeComputation;

The existence of the effect is not invalid merely because a particular target lacks sufficient resources.

The downstream diagnostic should distinguish:

ZMN-EFFECT-RESOURCE-UNSATISFIED

from:

ZMN-EFFECT-UNKNOWN

---

37. Resource diagnostic payload

Where an effect cannot be realized because of resources, the diagnostic SHOULD carry structured data such as:

effect_identity
resource_kind
requested
available
unit
requirement
target_context
scope

The values must remain symbolic or arbitrary-precision where the semantic model requires it.

The diagnostic representation MUST NOT introduce a universal maximum such as:

MAX_QUBITS
MAX_THREADS
MAX_MEMORY
MAX_EFFECT_RESOURCES

---

38. Target-dependent diagnostics

A target may lack an implementation of a valid effect.

This may produce:

ZMN-EFFECT-TARGET-UNSATISFIED

or the canonical target diagnostic defined by "grammar/spec/diagnostics.md".

Such a diagnostic MUST state that:

the source-level effect is valid

but:

the selected realization cannot satisfy it

when that distinction is known.

This is essential to POCO-REAF.

---

39. Quantum effects

Quantum effects MUST remain open-world.

Examples include:

quantum::Measurement
quantum::Reset
quantum::DynamicControl
quantum::Readout
quantum::Noise
quantum::LogicalOperation

The diagnostic subsystem MUST NOT require a closed enumeration of quantum effects.

It MUST NOT contain diagnostic logic equivalent to:

if effect == H
if effect == X
if effect == CNOT

Quantum operation identity belongs to the quantum semantic layer.

---

40. Canonical quantum integration

Effect-related quantum semantics ultimately cross the established boundary:

effect AST
    │
    ▼
semantic effect analysis
    │
    ▼
canonical semantic representation
    │
    ▼
quantum::ir

The canonical "quantum::ir" remains the semantic quantum boundary.

This file MUST NOT introduce:

EffectQuantumIR
QuantumEffectIR
EffectQIR
EffectCircuitIR

or any second quantum IR.

---

41. Quantum diagnostic classification

The following distinction is mandatory.

Invalid source-level effect

ZMN-EFFECT-UNKNOWN

Invalid quantum semantic use

A quantum semantic diagnostic belongs to the quantum diagnostic domain.

Missing quantum capability

ZMN-EFFECT-CAPABILITY-MISSING

or the canonical capability code.

Insufficient quantum resources

ZMN-EFFECT-RESOURCE-UNSATISFIED

or the canonical resource code.

Routing failure

Routing subsystem owns the diagnostic.

Scheduling failure

Scheduling subsystem owns the diagnostic.

QEC failure

QEC subsystem owns the diagnostic.

ZQN failure

ZQN subsystem owns the diagnostic.

HAL failure

HAL owns the diagnostic.

Runtime failure

Runtime owns the diagnostic.

Effect diagnostics MUST NOT absorb these downstream responsibilities.

---

42. Classical, HDL, AI, distributed, and future effects

The same diagnostic model applies to:

classical::...
quantum::...
hdl::...
hardware::...
distributed::...
ai::...
data::...
networking::...
security::...
accelerator::...
photonic::...
neuromorphic::...
future::...

No domain-specific diagnostic grammar is required merely because a new effect namespace is introduced.

This preserves the open-world architecture.

---

43. Cross-domain diagnostics

If one effect crosses domains:

classical::control
        ↓
quantum::measurement
        ↓
classical::decision

the effect diagnostic should identify the semantic effect boundary.

If the actual failure belongs to another domain, the diagnostic must be emitted by that domain.

For example:

effect capability missing

belongs to capability analysis.

A malformed quantum operation belongs to quantum semantic validation.

An invalid HDL connection belongs to HDL semantic validation.

---

44. Portability diagnostics

A valid effect may be supported only under certain profiles.

A portability warning may use:

ZMN-EFFECT-PORTABILITY

It should distinguish:

language-invalid

from:

valid but not portable to the selected environment

The diagnostic SHOULD identify:

effect
profile
unsupported assumption
portable alternative

when available.

---

45. Compatibility and deprecation

Deprecated effects may produce:

ZMN-EFFECT-DEPRECATED

Experimental effects may produce:

ZMN-EFFECT-EXPERIMENTAL

Version-specific behavior may produce:

ZMN-EFFECT-VERSION

These diagnostics MUST integrate with:

grammar/compatibility/versions.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md
grammar/spec/compatibility.md

A compatibility warning MUST NOT silently change effect semantics.

---

46. Effect visibility

If an effect exists but is inaccessible because of module visibility:

ZMN-EFFECT-VISIBILITY

The diagnostic SHOULD distinguish:

unknown

from:

known but inaccessible

This prevents misleading suggestions to declare an effect that already exists.

---

47. Effect import diagnostics

If an effect is referenced through an unavailable import:

ZMN-EFFECT-IMPORT-MISSING

The diagnostic SHOULD identify:

requested namespace
import context
source reference

Module-resolution diagnostics remain authoritative for module graph failures.

---

48. Effect security diagnostics

An effect may be semantically valid but prohibited by a security policy.

Example:

security::secret_access

A policy rejection may produce:

ZMN-EFFECT-SECURITY-POLICY

The diagnostic MUST NOT expose secrets, credentials, keys, tokens, or private hardware information.

---

49. Sensitive diagnostic data

Effect diagnostics MUST NOT leak:

- credentials;
- authentication tokens;
- private keys;
- secret effect arguments;
- private source contents unrelated to the diagnostic;
- backend credentials;
- device authentication information;
- network secrets.

Structured diagnostic payloads must be safe to serialize.

---

50. Determinism

For identical:

source
language version
compiler version
compiler configuration
semantic environment
diagnostic policy

effect diagnostics MUST be deterministic.

Diagnostic output MUST NOT depend on:

- hash-map iteration order;
- filesystem traversal order;
- network timing;
- hardware discovery order;
- number of CPUs;
- number of GPUs;
- number of QPUs;
- thread scheduling;
- randomness;
- wall-clock time.

---

51. Diagnostic ordering

Effect diagnostics SHOULD be ordered using the general repository diagnostic ordering contract.

Where multiple effect diagnostics share a source position, ordering MUST use a stable secondary key such as:

phase
diagnostic code
semantic identity
source order

Unordered collection iteration MUST NEVER determine output ordering.

---

52. Cascading diagnostics

The effect analyzer SHOULD suppress secondary diagnostics that are direct consequences of an already-established missing effect declaration.

For example:

unknown effect A
operation B requires A
handler C expects A

should not necessarily generate three independent root-cause errors if B and C cannot be meaningfully analyzed without A.

The analyzer SHOULD report:

primary:
ZMN-EFFECT-UNKNOWN

and attach dependent information as notes where useful.

The goal is:

one root cause
+
useful context

rather than diagnostic floods.

---

53. Error recovery

Effect diagnostics must support continued analysis where safe.

If an effect declaration is malformed, the parser may construct an error-tolerant structure for tooling.

However:

«An error-recovery placeholder MUST NOT be treated as a valid semantic effect.»

The compiler must distinguish:

valid Effect

from:

recovered ErrorEffect

or equivalent internal state.

The exact recovery representation belongs to parser/AST implementation.

---

54. No panic-based user diagnostics

Ordinary invalid source input MUST NOT cause the compiler to panic merely because effect analysis encountered an invalid user construct.

Effect diagnostic generation should return structured diagnostics through the repository's diagnostic APIs.

Compiler-internal invariant violations may use the repository's established internal-error mechanism, but user-controlled source must not be able to turn ordinary effect errors into unchecked process termination.

---

55. Rust safety requirements

The implementation baseline is:

Rust 1.97
Rust 1.97.1
Edition 2021

The effect diagnostic implementation MUST use safe Rust.

"unsafe" is prohibited.

The implementation SHOULD enforce this through:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

where appropriate at the relevant crate/module boundary.

Effect diagnostics MUST NOT require:

- raw pointers;
- unchecked memory access;
- manual allocation;
- FFI;
- unsafe string slicing;
- unsafe concurrency primitives.

Safe standard-library and existing repository diagnostic abstractions are sufficient.

---

56. No hard-coded scalability limits

This file MUST NOT define semantic constants such as:

MAX_EFFECTS
MAX_EFFECT_OPERATIONS
MAX_EFFECT_PARAMETERS
MAX_EFFECT_HANDLERS
MAX_EFFECT_NESTING
MAX_EFFECT_SET_SIZE
MAX_EFFECT_GENERIC_PARAMETERS
MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY

The architecture must scale from tiny programs to arbitrarily large programs subject to actual compiler, execution, and resource availability.

This means:

unbounded language model
+
finite concrete execution resources

rather than:

language model limited to today's hardware

---

57. Diagnostic resource budgets

The compiler MAY impose operational diagnostic budgets.

Examples:

maximum diagnostics retained
maximum diagnostic output bytes
maximum related spans
maximum diagnostic rendering memory

These are compiler policies, not language semantics.

If a budget is reached, the compiler should emit the repository's canonical diagnostic-budget condition.

It MUST NOT misclassify the source as invalid.

---

58. Large effect sets

Effect sets may become very large.

The implementation MUST avoid algorithms whose complexity unnecessarily becomes quadratic in effect-set size.

Where semantic normalization is required, the implementation SHOULD use an appropriate canonical representation while preserving deterministic source-order information for diagnostics.

The semantic representation may use:

ordered source references
+
canonical semantic identity index

provided that no duplicate authority is introduced.

---

59. Large handler collections

Handler lists may contain many arms.

The implementation SHOULD avoid repeatedly scanning all previous arms when determining duplicate or shadowed patterns where a suitable semantic index can be maintained.

Any index MUST be an implementation detail.

It MUST NOT become part of the source language.

---

60. Symbolically large quantities

Effect diagnostics may describe symbolic quantities.

For example:

requires resources { qubits >= n }

where:

n

is symbolic.

The diagnostic must not attempt to manufacture a concrete hardware quantity merely to make the message simpler.

It may instead report:

required: qubits >= n
available: target-dependent

when concrete target information is unavailable.

---

61. "Infinity" interpretation

POCO-REAF's scalability requirement does not require a compiler to allocate infinite memory or execute an infinite artifact.

It means:

«The effect language and its diagnostic contract introduce no artificial finite machine-size ceiling.»

Therefore:

tiny machine
large machine
distributed system
heterogeneous system
future architecture

are all valid realization classes.

Concrete compilation remains limited by actual:

memory
storage
time
address space
target capacity
compiler policy
runtime capacity

---

62. Diagnostics must not change semantics

Producing a warning, hint, or information message MUST NOT alter the effect semantics.

Likewise, diagnostic suppression MUST NOT alter compilation semantics.

For example:

-Wno-effect-portability

may suppress a warning.

It must not transform an invalid effect into a valid one.

---

63. Warning policy

Warnings may be promoted or suppressed through explicit compiler policy.

A diagnostic's fundamental identity MUST remain stable.

For example:

ZMN-EFFECT-DEPRECATED

may be configured as:

warning
error
suppressed

but the semantic condition remains the same.

---

64. Machine-readable representation

Every effect diagnostic SHOULD expose structured fields where the compiler has the information.

Conceptually:

{
    code,
    severity,
    phase,
    message,
    primary_span,
    related_spans,
    effect_identity,
    operation_identity,
    expected,
    actual,
    missing,
    conflicting,
    capability,
    resource,
    target,
    notes,
    help,
    suggestions
}

Not every field is required for every diagnostic.

Unknown or unavailable fields MUST be omitted rather than populated with fabricated values.

---

65. Diagnostic payload stability

Tooling must consume:

diagnostic code
severity
phase
source span
structured fields

rather than parsing the English message.

The textual message may change for clarity.

The semantic meaning of a stable diagnostic code may not silently change.

---

66. IDE/LSP integration

Effect diagnostics MUST be suitable for IDE and LSP consumers.

They should support:

- source range;
- severity;
- stable code;
- message;
- related locations;
- notes;
- suggestions;
- fix-its;
- semantic context.

LSP-specific coordinate conversions belong at the tooling boundary.

The effect subsystem must not introduce an LSP-specific span model.

---

67. CLI integration

CLI rendering may display:

error[ZMN-EFFECT-UNHANDLED]: ...
  --> source.zm:...

The rendering layer is not the effect subsystem's authority.

Terminal formatting, colors, Unicode decorations, JSON output, and machine-readable output belong to the general diagnostics implementation.

---

68. Library integration

Compiler libraries must be able to receive structured effect diagnostics without requiring terminal output.

Effect analysis MUST NOT directly write to:

stdout
stderr
files
network sockets

Diagnostics are data.

Rendering is downstream.

---

69. Compiler integration

The compiler should receive effect diagnostics as structured diagnostic values.

Conceptually:

parse
  ↓
AST
  ↓
effect analysis
  ↓
DiagnosticBag
  ↓
compiler orchestration

Effect analysis must not independently terminate the compilation process merely because it finds an effect error.

The compiler decides whether compilation can continue.

---

70. Runtime integration

Runtime failures are not effect-language diagnostics unless the runtime is explicitly reporting a source-correlated effect failure.

For example:

effect Storage::Read

may be semantically valid but fail at runtime because the external service is unavailable.

That should normally be classified as:

RUNTIME

rather than:

EFFECT

The runtime may preserve the original effect identity as structured provenance.

---

71. Capability integration

When semantic analysis maps an effect to capabilities:

effect quantum::Measurement
        │
        ▼
capability("quantum.measurement")

the capability subsystem owns capability semantics.

If the capability is missing:

ZMN-EFFECT-CAPABILITY-MISSING

may provide effect context, while the canonical capability diagnostic remains authoritative for capability semantics.

The two systems must not duplicate capability definitions.

---

72. Resource integration

When an effect requires resources:

effect quantum::Measurement
        │
        ▼
resource requirements

resource analysis owns the resource semantics.

The effect diagnostic may carry:

effect_identity
resource_requirement

but must not become the resource registry.

---

73. Hardware integration

Effect diagnostics MUST NOT select hardware.

The correct pipeline is:

effect intent
   ↓
semantic analysis
   ↓
capabilities/resources/requirements
   ↓
target selection
   ↓
hardware realization

Hardware failure belongs downstream.

---

74. QEC and ZQN integration

Effect diagnostics MUST NOT implement QEC or ZQN.

If an effect's realization eventually requires:

error correction
noise analysis
resilience
fault classification

the appropriate downstream subsystem owns the semantic decision.

The effect diagnostic may preserve provenance:

originating_effect = quantum::...

without becoming a QEC/ZQN diagnostic registry.

---

75. Routing and scheduling integration

An effect may influence later routing or scheduling.

For example:

quantum::Measurement
distributed::Send
accelerator::Compute

may affect downstream realization.

Effect diagnostics must not report:

routing impossible

or:

schedule impossible

as effect-language errors unless the responsible subsystem explicitly returns the diagnostic through an effect-associated context.

---

76. Lowering diagnostics

If a valid effect cannot be lowered into the selected canonical representation:

ZMN-EFFECT-LOWERING-UNAVAILABLE

may be used.

The diagnostic MUST include:

effect identity
source span
semantic context
target/domain context

when available.

It must not imply that the source syntax itself is invalid unless that is actually true.

---

77. Effect implementation versus declaration

A declaration:

effect Storage;

does not provide an implementation.

Therefore absence of an implementation should not automatically be:

ZMN-EFFECT-UNKNOWN

The semantic pipeline should distinguish:

declared
known
implemented
available
authorized
realizable

These are separate states.

---

78. Diagnostic state model

Where needed, effect analysis may distinguish:

Declared
Resolved
TypeChecked
EffectChecked
CapabilityChecked
ResourceChecked
Lowerable
TargetRealizable
RuntimeAvailable

A diagnostic should identify the first state transition that failed.

This prevents misleading messages such as:

unknown effect

when the effect is known but merely unavailable on a target.

---

79. Compatibility with historical syntax

When historical effect syntax is accepted through compatibility facilities, diagnostics must distinguish:

accepted and current
accepted but deprecated
accepted under compatibility mode
proposed only
invalid

Historical syntax must not silently become a new canonical effect grammar.

---

80. Diagnostic suggestions

Suggestions should be generated only when semantically justified.

Examples:

help: declare the effect in this module

or:

help: propagate `Storage::Read` through the enclosing effect context

or:

help: provide a handler for `Storage::Read`

A suggestion must not tell a developer to select a specific physical device merely because the current implementation happens to require one.

---

81. Fix-it requirements

Effect fix-its may include:

- inserting a missing effect declaration;
- correcting an unambiguous effect name;
- adding an import;
- adding a handler where syntax and semantics make the transformation safe;
- removing an exact duplicate effect-set entry where the language defines duplicates as redundant.

Fix-its MUST NOT:

- silently add hardware selection;
- silently add physical qubit IDs;
- silently select a backend;
- silently add target-specific resource constraints;
- alter semantic effect requirements merely to make compilation succeed.

---

82. Security boundary

Effect diagnostics are part of compiler input processing and must treat source as untrusted.

They must not:

- execute effect operations;
- invoke handlers;
- access external resources;
- inspect hardware;
- contact networks;
- access secrets;
- execute user code;
- invoke vendor APIs.

Parsing and semantic diagnosis are pure compiler activities.

---

83. Deterministic semantic context

Effect diagnostic generation may use semantic information such as:

resolved effect
scope
generic substitution
effect row
capabilities
resource requirements

but those inputs must themselves be deterministic under identical compilation inputs.

Effect diagnostics must not depend on live runtime state unless the compiler is explicitly performing a target/runtime realization phase.

---

84. Testing contract

Every implemented effect diagnostic MUST have tests in:

grammar/tests/effects/

and, where applicable:

grammar/tests/negative/
grammar/tests/boundary/
grammar/tests/scalability/
grammar/tests/determinism/
grammar/tests/compatibility/

Tests must cover:

- positive cases;
- negative cases;
- boundary cases;
- diagnostic code;
- severity;
- primary span;
- secondary spans;
- message invariants;
- structured payload;
- deterministic ordering;
- recovery behavior;
- compatibility behavior.

---

85. Required positive tests

Positive tests must cover at least:

effect IO;
effect storage::Read;
effect quantum::Measurement;
effect future::domain::Operation;
effect vendor::extension::Operation;

and:

effect sets
effect handlers
effect generic effects
effect polymorphic effects
effect composed effects
effect cross-domain effects

The tests must demonstrate that open-world effect names remain syntactically and semantically extensible.

---

86. Required negative tests

Negative tests must cover:

unknown effect
unknown operation
ambiguous effect
duplicate declaration
invalid effect signature
argument mismatch
effect-set mismatch
missing required effect
unhandled effect
invalid handler
duplicate handler
unreachable handler
invalid resumption
non-resumable resumption
invalid effect composition
invalid effect polymorphism
visibility failure
compatibility failure

---

87. Required boundary tests

Boundary tests must include:

empty effect set
single-effect set
large effect set
nested handlers
large handler list
generic effect with no parameters
generic effect with many parameters
nested qualified names
long effect names
Unicode identifiers where supported
empty handler body
single handler arm
trailing comma
multi-file effect references
generated-source effect references

No test may establish a universal finite maximum merely because the test fixture happens to use one.

---

88. Required scalability tests

Scalability tests must verify that effect processing remains correct as the number of:

effects
operations
handlers
effect-set entries
generic parameters
modules
source files
cross-domain references

increases.

Tests should verify semantic correctness and deterministic diagnostics rather than define an artificial maximum.

---

89. Required determinism tests

Run the same source multiple times with identical compiler configuration.

Verify:

same diagnostic code
same severity
same primary span
same secondary spans
same structured payload
same ordering

The result must not change because of:

hash-map ordering
thread scheduling
machine resource count
backend enumeration
filesystem ordering

---

90. Required target-independence tests

The same effect source should be analyzed independently from target selection wherever possible.

For example:

effect quantum::Measurement;

must not produce a different language-semantic result merely because one invocation knows about:

CPU
GPU
FPGA
QPU
simulator
future target

Target-dependent diagnostics may appear only after target context becomes part of the relevant compilation phase.

---

91. Required POCO-REAF tests

At least the following conceptual realizations must be tested:

tiny classical target
multicore target
GPU-capable target
FPGA-capable target
quantum simulator
quantum hardware
heterogeneous target
distributed target
future/unknown target

The effect syntax must remain unchanged.

Only downstream capability/resource/target results may differ.

---

92. Hard-coding audit

The effect diagnostic subsystem MUST be audited for forbidden assumptions.

The following patterns are prohibited as universal language semantics:

MAX_EFFECTS
MAX_EFFECT_OPERATIONS
MAX_EFFECT_HANDLERS
MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY

Also prohibited are diagnostics that assume:

physical qubit numbering
specific CPU IDs
specific GPU IDs
fixed accelerator counts
fixed network topology
fixed memory sizes
fixed register widths
fixed device counts

A compiler resource budget may exist, but it must be explicitly classified as an implementation policy.

---

93. Diagnostic ownership matrix

Condition| Owner| Effect diagnostic role
malformed "effect" syntax| parser| none beyond context
unknown effect name| effect/name analysis| "ZMN-EFFECT-UNKNOWN"
inaccessible effect| module/name analysis| effect-context diagnostic
duplicate declaration| effect semantic analysis| "ZMN-EFFECT-DUPLICATE"
operation signature mismatch| effect/type analysis| effect diagnostic + type context
missing effect| effect analysis| "ZMN-EFFECT-REQUIRED-MISSING"
unhandled effect| effect analysis| "ZMN-EFFECT-UNHANDLED"
handler mismatch| effect analysis| handler diagnostic
missing capability| capability analysis| effect-context diagnostic
insufficient resources| resource analysis| effect-context diagnostic
invalid quantum semantics| quantum semantic analysis| quantum diagnostic
routing failure| routing| routing diagnostic
scheduling failure| scheduling| scheduling diagnostic
QEC failure| QEC| QEC diagnostic
ZQN failure| ZQN| ZQN diagnostic
HAL failure| HAL| HAL diagnostic
target unavailable| target layer| target diagnostic
runtime effect failure| runtime| runtime diagnostic

This table is an integration contract, not a second semantic specification.

---

94. File-to-file integration contract

This file is complete independently when the following contracts are already defined.

"grammar/effects/effects.g4"

Provides effect composition entry points.

This file supplies diagnostic semantics for those constructs.

"grammar/effects/effect-declarations.g4"

Owns effect declaration syntax.

This file defines how semantic declaration failures are diagnosed.

"grammar/effects/effect-sets.g4"

Owns effect-set syntax.

This file defines set-related diagnostic categories.

"grammar/effects/effect-handling.g4"

Owns handler syntax.

This file defines handler-related diagnostic categories.

"grammar/effects/custom-effects.g4"

Provides extensible/custom effect syntax.

This file ensures unknown/custom identities are not rejected merely for being new names.

"grammar/effects/effect-composition.g4"

Owns composition syntax.

This file defines composition diagnostic integration.

"grammar/effects/effect-polymorphism.g4"

Owns polymorphic syntax.

This file defines inference/compatibility diagnostic integration.

"grammar/spec/effects.md"

Owns normative effect semantics.

This file reports violations of those semantics.

"grammar/spec/diagnostics.md"

Owns the general diagnostic architecture.

This file specializes it for effects.

"grammar/spec/source-spans.md"

Owns source-span semantics.

This file consumes that model.

"src/frontend/ast/node/effects/effect.rs"

Owns source-level effect AST representation.

This file must never require backend-specific AST fields.

"src/frontend/ast/node/mod.rs"

Owns AST module composition.

This file must not create a parallel diagnostic AST.

"src/compiler/diagnostics.rs"

Owns compiler diagnostic implementation/infrastructure.

Effect diagnostics must flow through it.

"src/error_reporting.rs"

Provides existing compiler-error/reporting integration.

Effect errors must remain compatible with its established error model where that API remains active.

"src/quantum/ir/"

Owns the canonical quantum semantic boundary.

This file must not define another quantum IR.

---

95. Completion criteria

"grammar/effects/effect-diagnostics.md" is complete when all of the following are true:

1. Effect diagnostic ownership is explicit.
2. General diagnostic ownership remains in "grammar/spec/diagnostics.md".
3. Source-span ownership remains in "grammar/spec/source-spans.md".
4. Effect syntax remains owned by the modular effect grammars.
5. Effect semantics remain owned by "grammar/spec/effects.md".
6. Effect AST representation remains domain-neutral.
7. Capability semantics remain outside this file.
8. Resource semantics remain outside this file.
9. Hardware selection remains outside this file.
10. Quantum semantics remain outside this file.
11. "quantum::ir" remains the canonical quantum semantic boundary.
12. QEC remains outside this file.
13. ZQN remains outside this file.
14. Routing remains outside this file.
15. Scheduling remains outside this file.
16. HAL remains outside this file.
17. Runtime behavior remains outside this file.
18. Effect identities remain open-world.
19. No closed effect enumeration exists.
20. No machine-size limits are introduced.
21. No physical-resource IDs are introduced.
22. Diagnostic codes are machine-readable.
23. Diagnostic ordering is deterministic.
24. Source spans are deterministic.
25. Multi-file diagnostics are supported.
26. Structured payloads are supported.
27. Fix-its cannot silently change semantic effect requirements.
28. Resource failures are distinguished from source invalidity.
29. Capability failures are distinguished from effect invalidity.
30. Target failures are distinguished from language invalidity.
31. Runtime failures are distinguished from compile-time effect errors.
32. Quantum effect failures remain downstream-compatible with "quantum::ir".
33. Positive tests exist.
34. Negative tests exist.
35. Boundary tests exist.
36. Scalability tests exist.
37. Determinism tests exist.
38. Compatibility tests exist.
39. Rust 1.97/1.97.1 compatibility is maintained.
40. "unsafe" is prohibited.
41. The file does not require subsequent redesign merely because another effect domain is added.
42. Adding a new effect namespace does not require modifying this file.
43. Adding a new backend does not require modifying this file.
44. Adding a new quantum technology does not require modifying this file.
45. Adding a new accelerator technology does not require modifying this file.
46. Adding a new distributed topology does not require modifying this file.

---

96. Final invariant

The effect diagnostic subsystem must preserve the following architectural rule:

                    EFFECT DIAGNOSTICS
                           │
                           ▼
              report semantic effect facts
                           │
          ┌────────────────┼────────────────┐
          ▼                ▼                ▼
      capabilities      resources        semantics
          │                │                │
          └────────────────┼────────────────┘
                           ▼
                  canonical semantic model
                           │
              ┌────────────┼────────────┐
              ▼            ▼            ▼
          classical     quantum::ir     HDL
              │            │            │
              └────────────┼────────────┘
                           ▼
                  optimization/lowering
                           │
              ┌────────────┼────────────┐
              ▼            ▼            ▼
           routing     scheduling      QEC/ZQN
              │            │            │
              └────────────┼────────────┘
                           ▼
                          HAL
                           │
                           ▼
                    target realization
                           │
                           ▼
                        runtime

The effect diagnostic layer must never reverse this dependency direction.

The language therefore remains capable of expressing:

what computation does
what effects it performs
what effects it permits
what effects it requires
how effects compose
how effects are handled

without prematurely deciding:

which CPU
which GPU
which FPGA
which ASIC
which QPU
which physical qubit
which memory bank
which network node
which topology
which scheduler
which router
which QEC implementation
which calibration
which backend

That separation is mandatory for Zamani's:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

architecture.

The production invariant is therefore:

«Effect diagnostics describe failures in the portable effect model and its semantic contracts. They never turn today's hardware, backend, topology, resource capacity, or implementation strategy into a language-level limitation.»