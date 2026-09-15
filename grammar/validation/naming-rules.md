Zamani Naming Rules

Path: "grammar/validation/naming-rules.md"
Status: Normative
Language: Zamani
Grammar layer: Naming validation and language-policy specification
Grammar technology: ANTLR4-compatible Zamani grammar
Implementation baseline: Rust 1.97 / Rust 1.97.1
Safety requirement: Safe Rust only; "unsafe" is prohibited
Portability model: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability model: No arbitrary language-imposed finite resource limits

---

1. Purpose

This document defines the normative naming policy for Zamani.

It specifies how names are:

- formed;
- classified;
- compared;
- resolved;
- qualified;
- exposed;
- imported;
- exported;
- versioned;
- deprecated;
- generated;
- referenced across computing domains;
- preserved across compilation;
- represented across AST and IR boundaries.

This document is a validation and semantic-policy contract.

It does not replace:

- the lexer;
- parser grammar;
- AST;
- name-resolution implementation;
- type checker;
- semantic analyzer;
- canonical IR;
- compiler;
- runtime.

The central architectural distinction is:

lexer
  ↓
lexical identifier syntax
  ↓
grammar/core/names.g4
  ↓
structural name syntax
  ↓
grammar/validation/naming-rules.md
  ↓
naming-policy validation
  ↓
AST / symbol resolution
  ↓
semantic model
  ↓
canonical IR

Therefore:

«"grammar/core/names.g4" defines what constitutes a syntactically valid name.»

«"grammar/validation/naming-rules.md" defines what naming policies must be enforced after syntax is recognized.»

«Semantic analysis determines what a valid name refers to.»

---

2. Ownership

2.1 This file owns

This file owns the normative policy for:

- identifier naming conventions;
- declaration naming conventions;
- domain-neutral naming;
- qualified-name policy;
- namespace naming;
- module naming;
- package naming;
- type naming;
- function naming;
- variable naming;
- constant naming;
- resource naming;
- capability naming;
- requirement naming;
- constraint naming;
- effect naming;
- quantum naming;
- classical naming;
- HDL naming;
- hardware naming;
- distributed-system naming;
- generated-name policy;
- compiler-generated-name isolation;
- dialect naming;
- extension naming;
- versioned names;
- deprecation naming;
- reserved-name policy;
- collision policy;
- case policy;
- Unicode policy;
- normalization policy;
- confusable-name policy;
- source/AST/IR name preservation;
- compatibility requirements for names;
- deterministic naming requirements.

2.2 This file does not own

This file does not own:

- lexical identifier character definitions;
- lexer tokenization;
- keyword token definitions;
- Unicode normalization implementation;
- parser structure;
- source paths;
- filesystem paths;
- URLs;
- hardware addresses;
- physical addresses;
- network addresses;
- symbol-table implementation;
- type resolution;
- overload resolution;
- resource allocation;
- target selection;
- hardware discovery;
- calibration;
- scheduling;
- routing;
- optimization;
- QEC;
- ZQN;
- resilience;
- runtime execution;
- canonical quantum semantics;
- canonical classical IR;
- hardware IR;
- backend-specific identifiers.

These remain owned by their respective repository subsystems.

---

3. Normative Language

The following terms are normative:

- MUST — mandatory.
- MUST NOT — prohibited.
- SHOULD — strongly recommended unless a documented compatibility reason exists.
- SHOULD NOT — strongly discouraged.
- MAY — permitted.
- MAY NOT — not permitted in the specified context.
- IMPLEMENTATION LIMIT — a practical implementation/resource limitation that is not a language naming limit.

---

4. Architectural Boundary

Zamani naming is divided into four layers.

Layer 1: Lexical naming
        ↓
Layer 2: Syntactic naming
        ↓
Layer 3: Naming-policy validation
        ↓
Layer 4: Semantic resolution

4.1 Lexical naming

Owned by:

grammar/antlr/ZamaniLexer.g4
grammar/lexer/

The lexer determines whether a character sequence is an identifier token.

This document MUST NOT duplicate the lexer identifier rule.

---

4.2 Syntactic naming

Owned by:

grammar/core/names.g4

The current canonical name grammar provides:

simpleName
identifier
nameSegment
qualifiedName
nameList
qualifiedNameList
nameAlias
nameReference

and related reusable structures.

The canonical grammar uses:

::

for qualified names.

Examples:

quantum::ir
hardware::capability
math::linear

The name grammar intentionally does not determine semantic meaning.

---

4.3 Naming-policy validation

Owned by:

grammar/validation/naming-rules.md

This layer determines whether a syntactically valid name satisfies Zamani's naming policy.

---

4.4 Semantic resolution

Owned downstream by semantic analysis and symbol resolution.

It determines whether:

quantum::ir

means:

- a module;
- a namespace;
- a type;
- a declaration;
- an IR component;
- an imported symbol;
- another semantic entity.

The naming validator MUST NOT perform semantic resolution.

---

5. Canonical Identifier Rule

A Zamani identifier MUST originate from the canonical lexer.

No grammar fragment may define a competing identifier token.

Forbidden:

identifier
    : [a-zA-Z]+
    ;

inside parser grammars.

Forbidden:

QubitName
    : ...
    ;

as a substitute for the canonical identifier.

Forbidden:

DeviceName
    : ...
    ;

Forbidden:

GpuName
    : ...
    ;

Domain-specific semantic objects use ordinary Zamani names.

For example:

q
logical_state
backend
device
accelerator
node
matrix
tensor
pipeline

are ordinary names.

Their semantic interpretation belongs downstream.

---

6. Case Policy

Zamani identifiers are case-sensitive.

Therefore:

value
Value
VALUE

are distinct source names.

Likewise:

quantum::ir
Quantum::ir
quantum::IR

are distinct names unless semantic resolution explicitly establishes another relationship.

The compiler MUST NOT silently case-fold source identifiers.

This ensures:

- deterministic symbol identity;
- predictable cross-platform behavior;
- stable serialization;
- portable source semantics;
- compatibility across filesystems with different case behavior.

---

7. Canonical Naming Styles

Zamani uses context-sensitive naming conventions.

These conventions are semantic-policy rules, not lexer restrictions.

7.1 Variables

Variables SHOULD use "snake_case".

Examples:

state
measurement_result
logical_qubit
tensor_shape
execution_context

---

7.2 Functions

Functions SHOULD use "snake_case".

Examples:

compute_energy
measure_state
allocate_buffer
compile_program
schedule_operations

---

7.3 Methods

Methods SHOULD use "snake_case".

Examples:

read
write
measure
reset
execute
serialize
deserialize

---

7.4 Types

Named types SHOULD use "PascalCase".

Examples:

Qubit
QuantumState
ExecutionContext
ResourceRequirement
HardwareCapability
Tensor
Matrix

---

7.5 Traits and interfaces

Traits/interfaces SHOULD use "PascalCase".

Examples:

Backend
Scheduler
Decoder
CapabilityProvider
ResourceManager
QuantumProgram

---

7.6 Modules

Modules SHOULD use lowercase "snake_case".

Examples:

quantum
hardware
classical
distributed
error_correction
resource_management

---

7.7 Packages

Package names SHOULD use lowercase "snake_case".

Examples:

quantum_runtime
hardware_backend
scientific_computing

---

7.8 Constants

Constants SHOULD use "SCREAMING_SNAKE_CASE" when the language construct is explicitly a constant.

Examples:

PI
DEFAULT_PRECISION
MAX_RETRY_ATTEMPTS

However, a constant MUST NOT be used to hide an arbitrary machine limit.

This is invalid architectural practice:

MAX_QUBITS = 64

when "64" is merely an implementation limitation.

A genuine program requirement may be represented as data:

required_qubits = expression

or through a resource requirement.

---

7.9 Enum variants

Enum variants SHOULD use "PascalCase".

Examples:

Healthy
Degraded
Recovering
Unavailable

---

8. Naming Must Not Encode Hardware Limits

Names MUST NOT encode accidental physical limitations.

The following are prohibited as language architecture:

Qubit32
Qubit64
Core128
Gpu8
Node1024
Device16
TensorRank32

when the number represents an implementation-imposed maximum.

A name containing a number is not inherently illegal.

For example:

sha256
utf8
riscv64

may be legitimate identifiers when the number is part of the actual semantic identity of an external standard or technology.

The distinction is:

semantic identity

versus:

accidental resource limit

---

9. Quantum Naming

Quantum names MUST remain hardware-independent unless the program explicitly names a physical target.

The language MUST support names such as:

q
register
logical_qubit
physical_qubit
state
observable
measurement
circuit
decoder
syndrome

The naming system MUST NOT reserve special names for:

q0
q1
q2
...

as a finite universe.

Indexed quantum resources are collections whose size is determined by program semantics and available resources.

Therefore:

q[i]

may be meaningful when "i" is semantically defined.

The grammar MUST NOT require:

q[0]
q[1]

or impose an arbitrary upper bound.

"quantum::ir" remains a canonical downstream semantic boundary and is not a naming namespace invented by this file.

---

10. Physical and Logical Resource Names

Logical and physical resources must remain semantically distinguishable without requiring different lexical identifier classes.

For example:

logical_qubit
physical_qubit

may be names of variables or declarations.

The naming validator does not decide whether an object is:

- logical;
- physical;
- virtual;
- simulated;
- emulated;
- remote.

That determination belongs to semantic analysis and hardware/resource layers.

This prevents source syntax from coupling a program to one machine architecture.

---

11. Hardware Naming

Hardware names MUST be treated as ordinary semantic identifiers.

Examples:

cpu
gpu
fpga
asic
qpu
accelerator
memory
interconnect
device

These names MUST NOT automatically imply:

- device count;
- physical address;
- topology;
- vendor;
- model;
- capacity;
- clock rate;
- memory size.

Concrete hardware identities belong to target descriptions, hardware abstraction, deployment configuration, or runtime discovery.

---

12. HDL Naming

HDL constructs follow the same naming system.

Examples:

alu
control_unit
clock
reset
data_bus
pipeline
register_file
state_machine

Signals, ports, modules, processes, memories, and interfaces MUST use ordinary Zamani identifiers.

Names MUST NOT encode an implementation-specific width unless that width is part of the semantic design.

For example:

data_bus

is preferable to:

data_bus_64

when width is a configurable hardware parameter.

A concrete width may legitimately appear in a name only when it is part of the declared semantic identity.

---

13. Distributed Naming

Distributed resources MUST NOT require globally hard-coded node names.

Names such as:

node
worker
service
endpoint
cluster
partition

are semantic names.

Concrete runtime identity belongs to:

- deployment;
- resource discovery;
- scheduling;
- runtime;
- distributed coordination.

A source program MUST NOT require:

node_001
node_002
node_003

merely to express distributed computation.

---

14. AI, Data and Tensor Naming

AI/data constructs use ordinary Zamani names.

Examples:

model
dataset
tensor
batch
gradient
optimizer
inference
training
embedding

Names MUST NOT impose artificial dimensional limits.

Avoid architecture such as:

TensorRank32
Matrix4096
Batch1024

when these values are merely implementation constraints.

Semantic dimensions belong to:

- types;
- expressions;
- constraints;
- resources;
- runtime/backend capabilities.

---

15. Qualified Names

The canonical separator is:

::

Examples:

quantum::ir
quantum::measure
hardware::capability
classical::tensor
distributed::service

Qualified names have no fixed maximum number of segments.

Therefore:

a::b
a::b::c
a::b::c::d
...

are structurally valid subject to practical parser/resource availability.

The validator MUST NOT impose a finite namespace-depth limit.

---

16. Namespace Meaning

A qualified name is syntactic structure.

It does not automatically imply:

- filesystem hierarchy;
- package hierarchy;
- module hierarchy;
- hardware topology;
- network topology;
- object ownership.

Semantic resolution determines its meaning.

For example:

quantum::ir

must not be interpreted by the naming validator as a filesystem path or physical namespace.

---

17. Namespace Collision Rules

Within a semantic scope, two declarations MUST NOT produce indistinguishable canonical names when their namespace and declaration kind require uniqueness.

Collision checking belongs to semantic symbol resolution.

The naming validator may identify policy violations such as:

- forbidden reserved names;
- malformed generated names;
- prohibited spelling;
- invalid naming style where style is normative.

It MUST NOT independently construct the complete symbol table.

---

18. Unicode

Zamani MAY support Unicode identifiers where permitted by the canonical lexer.

Unicode identifier support MUST NOT be reimplemented in this document.

The lexer owns:

- accepted Unicode characters;
- lexical categories;
- Unicode identifier boundaries.

The semantic/name layer owns:

- canonical comparison policy;
- normalization policy;
- confusable detection.

---

19. Unicode Normalization

Source identifiers MUST have a deterministic canonical representation for semantic comparison.

The implementation MUST define whether identifiers are:

1. compared exactly as source code points, or
2. normalized before semantic comparison.

The chosen policy MUST be consistent across:

- lexer;
- parser;
- AST;
- symbol table;
- serialization;
- diagnostics;
- compiler caches;
- incremental compilation;
- generated metadata.

Normalization MUST NOT silently change source spelling.

The AST should preserve original source spelling for diagnostics and source fidelity while semantic identity uses the repository's canonical comparison representation.

---

20. Unicode Confusables

Implementations SHOULD detect potentially confusing identifiers such as visually similar characters.

For example:

alpha
аlpha

where characters may come from different Unicode scripts.

Confusable detection SHOULD produce a diagnostic or warning according to compiler policy.

Confusable detection MUST NOT arbitrarily reject legitimate multilingual identifiers solely because they are non-ASCII.

Security-sensitive contexts MAY require stricter identifier policies.

---

21. Mixed-Script Identifiers

Implementations SHOULD diagnose suspicious mixed-script identifiers.

For example:

latіn_value

where a visually similar character may originate from another script.

The compiler MUST distinguish:

naming validity

from:

security warning

so that policy can be configured without changing the grammar.

---

22. Reserved Words

Reserved words are owned by the lexer.

Naming validation MUST NOT duplicate the keyword list.

A spelling that lexes as a reserved keyword cannot be accepted as an ordinary identifier unless Zamani explicitly introduces an escaped-identifier mechanism.

Adding a keyword is therefore a compatibility-sensitive language change.

Before adding a keyword, the implementation MUST determine:

- existing identifier usage;
- parser impact;
- AST impact;
- documentation impact;
- tooling impact;
- compatibility impact;
- dialect impact.

---

23. Domain Keywords

A new domain must NOT automatically introduce a global keyword.

For example, adding a future accelerator domain MUST NOT require:

accelerator_keyword_1
accelerator_keyword_2
accelerator_keyword_3

when the same functionality can be expressed using:

- qualified names;
- declarations;
- attributes;
- capabilities;
- effects;
- resources;
- dialects.

This is essential for long-term extensibility.

---

24. Open-World Naming

Zamani is an open-world language.

Future technologies MUST be able to introduce semantic names without modifying the core naming model.

Therefore the core naming system MUST NOT contain predefined identifier categories for every possible future technology.

There is no requirement for:

QuantumName
GpuName
FpgaName
OpticalName
NeuromorphicName
BiologicalName
PhotonicName
FutureDeviceName

All are ordinary names until semantic analysis gives them meaning.

---

25. Dialect Naming

Dialect names MUST be qualified.

Recommended form:

domain::dialect
domain::dialect::feature

Examples:

quantum::openqasm
hdl::systemverilog
hardware::vendor_extension
ai::accelerator

Vendor-specific names MUST NOT pollute the global keyword namespace.

Vendor extensions MUST be isolated through dialect/module/namespace mechanisms.

---

26. Versioned Names

Version information MUST NOT normally be encoded by changing the base identifier.

Prefer:

module::feature

with version information represented through the language/module/dialect compatibility mechanism.

Avoid:

feature_v2
feature_v3
feature_v4

unless those names represent genuinely different semantic entities.

Versioning belongs to:

grammar/specification/language-version.md
grammar/compatibility/
grammar/dialects/versioning.g4

and corresponding compiler metadata.

---

27. Deprecation

Deprecated names MUST remain recognizable for the declared compatibility period.

Deprecation is a semantic/compiler policy, not a lexer change.

A deprecated name should produce an appropriate diagnostic containing:

- the deprecated spelling;
- replacement name;
- deprecation version;
- removal policy, if known.

Removing a deprecated name is a language-version change and MUST follow compatibility policy.

---

28. Generated Names

Compiler-generated names MUST be distinguishable from user-authored names.

Generated identifiers MUST:

- be deterministic;
- avoid collision with valid user names;
- be stable within the relevant compilation model;
- never change source semantics;
- never leak target-specific resource identities into portable source names.

A generated-name namespace SHOULD use an implementation-reserved prefix or equivalent internal representation.

Generated names MUST NOT become part of the user-facing language unless explicitly standardized.

---

29. Compiler Temporary Names

Temporary names such as:

_tmp
temp
tmp0
tmp1

MUST NOT be relied upon by user programs.

Compiler internals SHOULD represent temporaries using internal symbol IDs where possible rather than treating generated textual names as semantic identity.

This is particularly important for:

- optimization;
- SSA;
- lowering;
- scheduling;
- quantum decomposition;
- routing;
- hardware mapping;
- code generation.

---

30. Stable Semantic Identity

A source name MUST NOT be the sole long-term identity of a declaration across all compiler stages.

The compiler SHOULD assign stable internal symbol identities.

Conceptually:

source spelling
      ↓
resolved symbol
      ↓
stable internal identity
      ↓
IR identity

This prevents renaming of a temporary or qualified source path from accidentally changing semantic identity.

---

31. AST Contract

The AST MUST preserve sufficient information for:

- source diagnostics;
- original spelling;
- qualified segment order;
- source span;
- aliases;
- documentation;
- tooling;
- refactoring;
- deterministic serialization.

The AST SHOULD distinguish structurally:

SimpleName
QualifiedName
Alias

without resolving their meaning during parsing.

The AST MUST NOT replace names with hardware-specific IDs during parsing.

---

32. Semantic Contract

Semantic analysis MUST determine:

- declaration resolution;
- scope;
- visibility;
- imports;
- exports;
- namespace ownership;
- type identity;
- overload identity;
- capability identity;
- resource identity;
- effect identity.

Semantic analysis MUST report unresolved or ambiguous names deterministically.

---

33. IR Contract

Names may be retained in IR for:

- provenance;
- diagnostics;
- debugging;
- metadata;
- symbolic references.

However, IR identity MUST NOT depend exclusively on source names.

The quantum pipeline is especially important:

Zamani source
    ↓
AST
    ↓
semantic quantum lowering
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

The grammar and naming validator MUST NOT create a second quantum representation.

---

34. QEC and ZQN

Naming rules MUST NOT define:

- QEC algorithms;
- decoder names as semantic primitives;
- noise models;
- fault semantics;
- physical topology;
- calibration identities.

Names such as:

decoder
syndrome
logical_qubit
noise_model
fault
calibration

are ordinary names.

Their semantic interpretation belongs to QEC, ZQN, hardware, and related downstream systems.

---

35. Scheduling and Optimization

Names MUST remain independent of scheduling and optimization.

A name such as:

operation
gate
kernel
pipeline
task

does not determine:

- execution order;
- duration;
- physical placement;
- routing;
- optimization strategy.

Scheduling and optimization consume semantic representations after name resolution.

---

36. Hardware Identity

Concrete hardware identity MUST be represented through hardware/target abstractions.

A source identifier such as:

backend
device
qpu
gpu
fpga

does not automatically identify a physical device.

Physical identity belongs downstream.

This prevents names from violating POCO-REAF.

---

37. Resource Names

Resource names MUST remain resource-parametric.

Do not create a finite namespace such as:

core0
core1
...
core127

to represent the language's CPU universe.

Likewise:

qubit0
qubit1
...
qubit63

must not represent the language's quantum universe.

Resources are determined by:

- program semantics;
- resource requirements;
- target capabilities;
- runtime discovery;
- scheduling;
- deployment.

---

38. Name Length

The language MUST NOT impose an arbitrary small maximum identifier length.

No rule such as:

identifier <= 32 characters

may exist merely for implementation convenience.

An implementation MAY have resource limits imposed by:

- memory;
- parser implementation;
- storage;
- operating system;
- backend;
- user configuration.

Such limits MUST be reported as implementation/resource diagnostics rather than presented as universal language semantics.

---

39. Qualified Name Depth

There is no language-defined finite maximum qualified-name depth.

The following are structurally permitted:

a::b
a::b::c
a::b::c::d

and arbitrarily deeper structures, subject only to practical resource availability.

---

40. Declaration Count

Naming rules MUST NOT restrict the number of declarations.

A program may contain any number of:

- modules;
- functions;
- variables;
- types;
- resources;
- quantum objects;
- hardware objects;
- distributed objects.

The compiler may reject programs when available resources are exhausted, but that is not a naming-rule restriction.

---

41. Determinism

Naming validation MUST be deterministic.

Given identical:

- source;
- lexer version;
- language version;
- naming-policy version;
- semantic environment;

the same names MUST receive the same naming-policy results.

The validator MUST NOT depend on:

- wall-clock time;
- random values;
- machine-local paths;
- hardware identity;
- network state;
- runtime scheduling;
- nondeterministic iteration order.

---

42. Filesystem Independence

Source names MUST NOT be interpreted as filesystem paths.

These are different concepts:

module::name

versus:

module/name

versus:

./module/name.zm

"core/names.g4" owns name syntax.

"core/paths.g4" owns path syntax.

No naming rule may collapse the two.

---

43. URL Independence

URLs are not identifiers.

For example:

https://example.org

must not be parsed as a qualified Zamani name.

URL/URI syntax belongs to the relevant interoperability or literal grammar.

---

44. ABI and FFI Names

Foreign names MUST remain explicitly separated from ordinary Zamani names.

For example:

ffi
abi
extern
foreign

may introduce interoperability constructs.

External symbol names MUST NOT silently become Zamani semantic names.

The interoperability subsystem owns:

- ABI naming;
- calling conventions;
- external symbol mapping;
- language linkage;
- foreign declarations.

Zamani naming policy governs the Zamani-side declaration that refers to the external symbol.

---

45. Interoperability

External languages may have incompatible naming rules.

The interoperability layer MUST provide an explicit mapping:

Zamani name
     ↓
foreign binding
     ↓
external name

The external spelling MUST NOT change the core Zamani naming model.

This permits integration with C, C++, Python, OpenQASM, Verilog, SystemVerilog, and future systems without contaminating the core namespace.

---

46. Import Aliases

Import aliases MUST use canonical Zamani names.

Example:

import quantum::measurement as measurement;

An alias MUST:

- be syntactically a valid identifier;
- obey naming policy;
- remain local to its declared scope;
- not modify the imported entity's canonical identity.

---

47. Shadowing

Shadowing is a semantic-scope rule.

Naming validation MUST NOT reject every repeated spelling globally.

For example, legal lexical repetition may occur in separate scopes.

Semantic analysis determines whether:

outer::value
inner::value

are valid declarations.

The implementation MUST produce deterministic diagnostics for prohibited shadowing policies.

---

48. Reserved Prefixes

Implementation-reserved prefixes MAY be established for:

- compiler-generated names;
- internal symbols;
- metadata;
- expansion artifacts;
- macro hygiene;
- synthesized IR objects.

Such prefixes MUST be documented.

They MUST NOT prevent future user namespaces unnecessarily.

Reserved space must remain deliberately small.

---

49. Macro Hygiene

Macro expansion MUST NOT accidentally capture user names.

Macro systems MUST use hygienic symbol identity or equivalent mechanisms.

Generated identifiers MUST be distinguishable from user-authored identifiers.

The naming validator MUST NOT rely solely on textual prefixes to guarantee hygiene.

---

50. Metaprogramming

Generated declarations MUST pass the same semantic naming validation as ordinary declarations unless they belong exclusively to an internal compiler namespace.

Metaprogramming MUST NOT bypass:

- reserved-name rules;
- namespace rules;
- collision rules;
- visibility rules;
- compatibility policy.

---

51. Dialect Isolation

A dialect MAY define additional naming conventions.

It MUST NOT silently redefine the global Zamani naming model.

A dialect may define:

vendor::feature
domain::extension
domain::dialect::operation

but must preserve:

- canonical identifiers;
- canonical qualified-name syntax;
- source stability;
- deterministic resolution;
- compatibility boundaries.

---

52. Vendor Names

Vendor-specific identifiers MUST be isolated through namespaces or dialects.

Avoid globally reserving:

vendor_operation_a
vendor_operation_b
vendor_operation_c

Prefer:

vendor::operation_a

or an equivalent dialect namespace.

This prevents vendor growth from consuming the permanent core language namespace.

---

53. Future Computing Domains

Future domains MUST use the existing naming architecture.

No change to:

identifier
simpleName
qualifiedName

should be necessary merely because Zamani gains a new computational domain.

This applies to future:

- quantum technologies;
- optical computing;
- neuromorphic computing;
- biological computing;
- molecular computing;
- distributed architectures;
- accelerators;
- AI architectures;
- unknown future technologies.

---

54. POCO-REAF Naming Guarantee

Names MUST describe semantic program entities rather than accidental deployment properties.

Therefore the same source name should retain the same semantic meaning when the program moves between:

embedded system
CPU
multicore CPU
GPU
FPGA
ASIC
quantum processor
simulator
heterogeneous system
cluster
supercomputer
cloud
future architecture

Target-specific realization occurs downstream.

---

55. Compatibility Rules

A naming change is backward-compatible only if existing valid programs retain their previous semantic interpretation.

The following are potentially breaking:

- introducing a previously unused keyword;
- making an identifier invalid;
- changing case sensitivity;
- changing Unicode comparison;
- changing normalization;
- changing qualified-name separators;
- changing namespace resolution;
- changing alias semantics;
- changing generated-name collision behavior.

Such changes MUST go through:

grammar/specification/language-version.md
grammar/specification/compatibility.md
grammar/compatibility/

and associated migration tests.

---

56. Keyword Addition Policy

Before adding a new reserved keyword, the implementation MUST establish that:

1. the feature cannot be expressed compositionally;
2. a qualified name is insufficient;
3. an attribute is insufficient;
4. an effect/capability is insufficient;
5. a declaration is insufficient;
6. a dialect is insufficient;
7. backward compatibility has been assessed;
8. lexer/parser tests exist;
9. migration behavior is defined.

Keyword growth must be exceptional.

---

57. Naming Diagnostics

Naming diagnostics MUST identify:

- source location;
- offending name;
- applicable rule;
- reason for rejection;
- suggested correction where possible;
- compatibility implications where relevant.

Diagnostics MUST distinguish:

LEXICAL_ERROR

from:

NAMING_POLICY_ERROR

from:

NAME_RESOLUTION_ERROR

from:

TYPE_ERROR

from:

CAPABILITY_ERROR

from:

RESOURCE_ERROR

This separation is essential for correct tooling.

---

58. Scalability Requirements

Naming validation MUST NOT contain arbitrary limits on:

- identifier length;
- namespace depth;
- declaration count;
- module count;
- resource count;
- quantum-resource count;
- device count;
- node count;
- tensor dimensions;
- program size.

The validator may encounter implementation/resource exhaustion.

Such exhaustion MUST be treated as an implementation/resource condition.

The language itself remains unbounded in principle.

---

59. No-Hard-Coding Audit

The implementation MUST audit naming code for:

MAX_IDENTIFIER_LENGTH
MAX_NAMESPACE_DEPTH
MAX_MODULES
MAX_DEVICES
MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_NODES
MAX_GPUS
MAX_FPGAS
MAX_ACCELERATORS

Any such value must be classified.

Permitted:

implementation/resource limit

when explicitly configurable and documented.

Not permitted:

language semantic naming limit

unless it is an actual semantic requirement of the language.

---

60. Rust Implementation Requirements

All Rust implementation supporting naming validation MUST target:

Rust 1.97

and:

Rust 1.97.1

as the production baseline.

The implementation MUST use safe Rust.

Required:

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

where appropriate for Rust implementation crates/modules.

No naming validator may require:

unsafe

for:

- identifier validation;
- Unicode handling;
- normalization;
- name comparison;
- diagnostics;
- symbol identity;
- serialization;
- caching.

---

61. Memory Safety

Naming validation MUST NOT assume:

- fixed-size buffers;
- fixed-size name arrays;
- fixed-size namespace arrays;
- fixed-size symbol tables.

Collections must grow according to available resources.

Resource exhaustion must be handled through explicit errors rather than memory-unsafe behavior.

---

62. Deterministic Collections

Where naming validation produces ordered diagnostics or serialized results, deterministic ordering MUST be used.

Implementations SHOULD use deterministic data structures or explicitly sort results.

Hash iteration order MUST NOT determine language semantics.

---

63. Serialization

Serialized names MUST preserve:

- source spelling where required;
- qualified segment ordering;
- semantic identity where applicable;
- language version;
- namespace information.

Serialization MUST NOT depend on machine-local identifiers.

---

64. Caching

Compiler caches involving names MUST include all relevant identity inputs.

At minimum, cache identity must account for the applicable:

- language version;
- naming-policy version;
- dialect configuration;
- semantic environment;
- source identity.

A cache hit MUST NOT cause an older naming interpretation to be silently reused after a language-policy change.

---

65. Provenance

Name-derived IR and compiler artifacts SHOULD retain provenance sufficient to answer:

Which source declaration produced this entity?

Provenance must not alter semantic identity.

It exists for:

- diagnostics;
- debugging;
- reproducibility;
- verification;
- optimization tracing;
- runtime observability.

---

66. Security

Naming validation MUST defend against:

- Unicode confusables;
- normalization ambiguity;
- namespace collision;
- generated-name collision;
- identifier injection into external tooling;
- misleading diagnostics;
- path/name confusion;
- foreign-symbol confusion.

Names MUST NOT be interpreted as:

- filesystem paths;
- shell commands;
- SQL;
- URLs;
- device addresses;

unless an explicit interoperability construct says so.

---

67. Privacy

Naming systems MUST NOT require source names to contain:

- user identity;
- machine identity;
- hardware serial numbers;
- network addresses;
- credentials;
- secrets.

Secrets MUST NOT be embedded into identifiers.

---

68. Diagnostics and Tooling

Language servers, formatters, refactoring tools, documentation generators, compilers, and analyzers MUST consume the same canonical name model.

They MUST NOT implement independent identifier semantics.

Required consistency:

lexer
  ↓
parser
  ↓
AST
  ↓
naming validator
  ↓
symbol resolver
  ↓
IDE/tooling

---

69. Documentation Integration

The following files must remain consistent with this document:

grammar/README.md
grammar/Zamani.g4
grammar/Zamani-Grammar.md
grammar/grammar.md
grammar/specification/README.md
grammar/specification/syntax-model.md
grammar/specification/semantic-model.md
grammar/specification/compatibility.md
grammar/specification/extensibility.md
grammar/core/names.g4
grammar/core/qualified-names.g4
grammar/core/paths.g4
grammar/lexer/identifiers.g4
grammar/lexer/keywords.g4
grammar/dialects/namespaces.g4
grammar/dialects/versioning.g4

No document may claim a naming feature is implemented unless the implementation and tests actually support it.

---

70. Integration Contract

Lexer

Consumes/produces:

IDENTIFIER
keyword tokens
punctuation tokens

The naming validator does not redefine them.

Parser

Consumes:

simpleName
qualifiedName
nameAlias
nameReference

from the canonical name grammar.

AST

Represents:

SimpleName
QualifiedName
Alias

with source spans.

Semantic analysis

Performs:

scope resolution
symbol resolution
collision detection
visibility checking
type/name relationship
capability/name relationship

Classical IR

Consumes resolved semantic names where needed.

"quantum::ir"

Receives canonical quantum semantic objects after semantic lowering.

The grammar does not create quantum IR.

QEC

Consumes resolved quantum semantics.

The naming layer does not define QEC identities or algorithms.

ZQN

Consumes quantum fault/noise semantics.

The naming layer does not define noise semantics.

Optimization

Consumes canonical IR.

Names may remain as provenance/debug metadata.

Scheduling

Consumes operation/resource semantics.

Names do not determine scheduling.

Routing

Consumes logical/physical mapping information.

Names do not determine topology.

Hardware abstraction

Resolves target-specific hardware identity.

Names do not encode device limits.

Runtime

Consumes compiled semantic/target artifacts.

Runtime identity MUST NOT redefine source-name semantics.

---

71. Dependency Direction

The dependency direction is:

lexer
  ↓
core/names.g4
  ↓
domain grammar
  ↓
AST
  ↓
naming validation
  ↓
semantic resolution
  ↓
canonical IR
  ↓
optimization
  ↓
routing
  ↓
scheduling
  ↓
hardware
  ↓
runtime

This file MUST NOT introduce reverse dependencies such as:

naming-rules.md → quantum::ir
naming-rules.md → scheduler
naming-rules.md → hardware discovery
naming-rules.md → runtime

---

72. Required Tests

Naming validation MUST have dedicated tests under:

grammar/tests/

and where the repository has validation-specific fixtures:

grammar/tests/validation/

Tests MUST include:

Positive

value
value_name
QuantumState
quantum::ir
hardware::capability
classical::tensor
distributed::service

Qualified names

a::b
a::b::c
a::b::c::d

Aliases

quantum::measurement as measurement

Cross-domain

quantum::state
hardware::accelerator
classical::tensor
hdl::pipeline
distributed::node

Unicode

Valid Unicode identifiers according to the canonical lexer.

Case sensitivity

value
Value
VALUE

must remain distinguishable.

Negative

Tests must cover:

- invalid identifier tokens;
- reserved keywords used as names;
- malformed qualified names;
- malformed aliases;
- invalid separators;
- path/name confusion;
- URL/name confusion;
- generated-name collisions;
- prohibited reserved implementation names.

---

73. Scalability Tests

Tests MUST demonstrate that no naming rule introduces arbitrary limits.

Examples must include:

- very long valid identifiers;
- deeply qualified names;
- very large name lists;
- large declaration sets;
- large module graphs;
- large quantum-resource programs;
- large distributed-resource programs.

Tests should scale according to test-resource availability rather than a hard-coded semantic maximum.

---

74. Cross-Domain Tests

At minimum test combinations of:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

The same naming model must work across every combination.

---

75. Compatibility Tests

Every language-version change affecting names MUST include:

old valid source
new compiler
expected result

and, where applicable:

old compiler
new source
expected compatibility diagnostic

Keyword additions require collision tests against previously legal identifiers.

---

76. Determinism Tests

The same source must produce:

- identical identifier tokenization;
- identical qualified-name structure;
- identical naming diagnostics;
- identical symbol identity inputs;
- deterministic serialized representations.

Repeated runs must not alter results.

---

77. Round-Trip Tests

Where a source printer exists:

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

must preserve name semantics.

For qualified names:

a::b::c

must not become:

a.b.c

or another representation that changes semantics.

---

78. Hard-Coding Audit Tests

The validation CI MUST inspect naming implementation and grammar infrastructure for accidental patterns such as:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_DEVICES
MAX_NODES
MAX_IDENTIFIER
MAX_NAMESPACE

Each finding must be classified as:

1. semantic requirement;
2. target requirement;
3. resource constraint;
4. implementation limitation;
5. accidental hard-coding;
6. test-only limitation;
7. documentation-only limitation.

Accidental hard-coding MUST fail production validation.

---

79. Completion Criteria

This file is complete only when all of the following are true:

- [ ] "grammar/core/names.g4" remains the canonical structural name grammar.
- [ ] The lexer remains the canonical identifier owner.
- [ ] Naming policy is separated from parsing.
- [ ] Semantic resolution remains downstream.
- [ ] Qualified names use the canonical "::" structure.
- [ ] Names are case-sensitive.
- [ ] No arbitrary identifier-length limit is specified.
- [ ] No arbitrary namespace-depth limit is specified.
- [ ] No quantum-count naming limit exists.
- [ ] No CPU-count naming limit exists.
- [ ] No GPU-count naming limit exists.
- [ ] No FPGA-count naming limit exists.
- [ ] No node-count naming limit exists.
- [ ] No device-count naming limit exists.
- [ ] Hardware identity is separated from source naming.
- [ ] Quantum naming is hardware-independent.
- [ ] HDL naming is implementation-independent.
- [ ] Distributed naming is deployment-independent.
- [ ] AI/data naming is dimension-independent.
- [ ] Future domains can use the same naming model.
- [ ] Vendor extensions are namespace/dialect isolated.
- [ ] Generated names cannot collide with user semantics.
- [ ] Macro hygiene is preserved.
- [ ] Unicode policy is deterministic.
- [ ] Confusable-name handling is defined.
- [ ] Reserved-word ownership remains with the lexer.
- [ ] Keyword growth is compatibility-controlled.
- [ ] Versioning is separated from identifier spelling.
- [ ] Deprecation policy is defined.
- [ ] AST integration is defined.
- [ ] Semantic integration is defined.
- [ ] IR integration is defined.
- [ ] "quantum::ir" remains downstream and canonical.
- [ ] QEC remains downstream.
- [ ] ZQN remains downstream.
- [ ] optimization remains downstream.
- [ ] routing remains downstream.
- [ ] scheduling remains downstream.
- [ ] hardware realization remains downstream.
- [ ] runtime remains downstream.
- [ ] Rust implementation remains compatible with Rust 1.97/1.97.1.
- [ ] No "unsafe" is required.
- [ ] Deterministic behavior is specified.
- [ ] Security implications are covered.
- [ ] Positive tests exist.
- [ ] Negative tests exist.
- [ ] Boundary tests exist.
- [ ] Cross-domain tests exist.
- [ ] Scalability tests exist.
- [ ] Compatibility tests exist.
- [ ] Determinism tests exist.
- [ ] Round-trip tests exist where applicable.
- [ ] Hard-coding audits exist.
- [ ] Documentation references this policy consistently.

---

80. Final Naming Principle

Zamani naming MUST follow this principle:

«A name identifies a semantic entity; it must not accidentally become a description of the machine on which that entity happens to execute.»

Therefore:

name
  ↓
semantic identity
  ↓
portable meaning
  ↓
target-independent IR
  ↓
target realization

not:

name
  ↓
machine assumption
  ↓
fixed hardware
  ↓
fixed resource count

The naming system must therefore support:

one program
    ↓
one stable semantic naming model
    ↓
many scales
    ↓
many architectures
    ↓
many hardware configurations
    ↓
many execution environments
    ↓
future computing technologies

while preserving:

Program_Once
Compile_Once
Run_Everywhere
Run_Anywhere
Run_Forever

This is the naming-policy foundation required for:

Zamani — From Atom to Everywhere.