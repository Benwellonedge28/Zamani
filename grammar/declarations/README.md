Zamani Declaration Grammar

Path: "grammar/declarations/README.md"

Status: Production architecture specification

Scope: Zamani declaration syntax and declaration-grammar composition

Grammar technology: ANTLR4 parser grammars

Rust implementation baseline: Rust 1.97 / Rust 1.97.1

Safety requirement: Safe Rust only; no "unsafe"

Language objective: Universal computation from atom to everywhere

Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

The "grammar/declarations/" directory owns the syntactic declaration layer of the Zamani programming language.

Declarations introduce named source-level entities such as:

- constants;
- variables;
- types;
- aliases;
- structs;
- enums;
- unions;
- interfaces;
- implementations;
- traits;
- and future declaration families.

This directory establishes syntax, not machine-specific implementation.

The declaration grammar must therefore allow Zamani programs to describe computation independently of the machine on which the program will eventually execute.

The fundamental boundary is:

Zamani source
    |
    v
Lexer
    |
    v
Declaration parser
    |
    v
Frontend AST
    |
    v
Name/type/effect/capability/resource analysis
    |
    +-------------------+
    |                   |
    v                   v
Classical semantic   Quantum semantic
representation       representation
                        |
                        v
                    quantum::ir
                        |
                        v
              optimization / QEC / ZQN /
              routing / scheduling / HAL
                        |
                        v
                     runtime

The declaration grammar MUST NOT bypass this architecture.

---

2. Core architectural principle

A declaration describes what a program means, not how a particular machine happens to realize it.

Therefore:

source semantics
        !=
hardware realization

A declaration must not silently encode:

- a fixed CPU count;
- a fixed core count;
- a fixed thread count;
- a fixed GPU count;
- a fixed FPGA count;
- a fixed ASIC;
- a fixed QPU;
- a fixed qubit count;
- a fixed physical-qubit identifier;
- a fixed memory capacity;
- a fixed register count;
- a fixed vector width;
- a fixed network size;
- a fixed cluster size;
- a fixed hardware topology;
- a fixed device address;
- a fixed accelerator;
- a fixed deployment environment.

If a declaration genuinely requires a resource, that requirement belongs to the appropriate resource/capability/constraint model.

For example:

requires quantum

must not inherently mean:

use device X
use N qubits
use topology Y

The latter decisions belong downstream.

---

3. POCO-REAF

The declaration grammar participates in:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever»

A valid declaration should remain semantically meaningful when the program moves between:

- embedded hardware;
- CPUs;
- multicore CPUs;
- GPUs;
- FPGAs;
- ASICs;
- quantum processors;
- quantum simulators;
- heterogeneous accelerators;
- clusters;
- supercomputers;
- distributed systems;
- cloud systems;
- future execution architectures.

Declarations therefore describe portable program semantics.

Physical realization is a downstream concern.

---

4. Ownership

4.1 This directory owns

The directory owns the concrete grammar of declaration constructs.

Specifically:

grammar/declarations/

owns:

- declaration syntax;
- declaration-family syntax;
- declaration-specific modifiers where explicitly owned;
- declaration-specific generic attachment;
- declaration-specific members;
- declaration-specific fields;
- declaration-specific variants;
- declaration-specific inheritance/implementation syntax;
- declaration parse-tree structure.

---

4.2 This directory does not own

This directory does not own:

- lexer tokens;
- keyword spelling;
- Unicode lexical policy;
- identifiers;
- qualified-name semantics;
- complete expression syntax;
- complete type-system semantics;
- type checking;
- name resolution;
- ownership analysis;
- borrow checking;
- effect checking;
- capability checking;
- resource discovery;
- hardware discovery;
- calibration;
- scheduling;
- routing;
- optimization;
- QEC;
- ZQN;
- resilience;
- runtime execution;
- device selection;
- physical qubit allocation;
- physical memory allocation;
- ABI selection;
- backend selection;
- canonical quantum IR;
- classical IR;
- hardware IR.

In particular:

grammar/declarations/
        |
        X
        |
        quantum::ir

is forbidden.

The correct direction is:

declaration grammar
        |
        v
frontend AST
        |
        v
semantic analysis
        |
        v
canonical semantic representation
        |
        v
quantum::ir

---

5. Current directory

The declaration directory currently contains:

grammar/declarations/
├── aliases.g4
├── constants.g4
├── declarations.g4
├── enums.g4
├── implementations.g4
├── interfaces.g4
├── structs.g4
├── traits.g4
├── types.g4
├── unions.g4
└── variables.g4

A directory README is therefore required as the architectural contract for these files.

The current repository confirms these declaration modules exist.

---

6. Final declaration architecture

The intended architecture is:

grammar/declarations/
│
├── README.md
│
├── declarations.g4
│
├── constants.g4
├── variables.g4
│
├── types.g4
├── aliases.g4
├── structs.g4
├── enums.g4
├── unions.g4
│
├── interfaces.g4
├── traits.g4
└── implementations.g4

No additional file should be created merely for symmetry.

A new declaration file is justified only when:

1. the construct has independently meaningful syntax;
2. it has a stable ownership boundary;
3. its grammar would otherwise make another file unmaintainable;
4. it has a distinct integration contract;
5. it can be independently tested.

---

7. Single declaration dispatcher

"declarations.g4" is the only declaration dispatcher.

It owns:

declaration

and declaration-family composition.

It does not own the concrete implementation of each declaration.

Conceptually:

declaration
    |
    +-- valueDeclaration
    |     |
    |     +-- constantDeclaration
    |     +-- variableDeclaration
    |
    +-- typeDeclarationFamily
          |
          +-- typeDeclaration
          +-- typeAliasDeclaration
          +-- structDeclaration
          +-- enumDeclaration
          +-- unionDeclaration
          +-- interfaceDeclaration

Future declaration families are added by integrating another dedicated grammar owner rather than copying its grammar into "declarations.g4".

The existing "declarations.g4" already follows this intended composition architecture and explicitly states that it is the single composition owner.

---

8. Concrete file responsibilities

8.1 "declarations.g4"

Purpose

Authoritative declaration dispatcher.

Owns

- "declaration";
- declaration-family composition;
- declaration ordering at the declaration boundary;
- delegate grammar composition.

Does not own

- concrete declaration syntax;
- type expressions;
- identifiers;
- expressions;
- quantum semantics;
- hardware semantics.

Inputs

Canonical lexer vocabulary and declaration delegate grammars.

Outputs

Declaration parse-tree branches.

Integration

source-unit
    |
    v
declarations.g4
    |
    +--> constants.g4
    +--> variables.g4
    +--> types.g4
    +--> aliases.g4
    +--> structs.g4
    +--> enums.g4
    +--> unions.g4
    +--> interfaces.g4

Completion criteria

No concrete declaration is duplicated in this file.

No delegate imports this file.

No downstream IR is referenced.

---

9. "constants.g4"

Purpose

Own constant declaration syntax.

Owns

- constant declaration structure;
- optional declared type;
- initializer attachment;
- declaration-level constant syntax.

Does not own

- constant evaluation;
- compile-time execution;
- type checking;
- constant folding;
- memory placement;
- hardware representation.

Required semantic pipeline

constant syntax
    |
    v
AST
    |
    v
type checking
    |
    v
constant evaluation
    |
    v
compiler/IR

Scalability

No fixed number of constants.

No fixed initializer size.

No target-specific constant representation.

Completion

The file is complete when constant syntax is independently parseable and integrates with canonical expression/type rules without redefining either.

---

10. "variables.g4"

Purpose

Own variable declaration syntax.

Owns

- variable declaration;
- declaration mutability keyword;
- optional type;
- optional initializer.

Does not own

- allocation;
- ownership;
- borrowing;
- lifetime;
- memory placement;
- register allocation;
- physical memory selection.

Required boundary

variable declaration
        |
        v
AST
        |
        v
ownership/type/effect analysis
        |
        v
classical/hardware/quantum semantic lowering

A variable must not imply a specific physical storage location.

---

11. "types.g4"

Purpose

Own named type declarations.

Owns

- named type declaration;
- type declaration modifiers permitted by the language;
- generic attachment;
- type definition boundary.

Does not own

- complete type-expression syntax;
- type inference;
- type compatibility;
- layout;
- ABI;
- hardware representation.

Integration

It consumes canonical type-expression rules from the type subsystem.

It must not invent another type-expression language.

---

12. "aliases.g4"

Purpose

Own type-alias declaration syntax.

Owns

- alias name;
- alias generic parameters where supported;
- alias target attachment.

Does not own

- type equivalence;
- substitution;
- normalization;
- recursive-type validation;
- representation.

Example semantic shape

alias Identifier = TypeExpression;

The actual canonical syntax must follow the authoritative Zamani type specification.

---

13. "structs.g4"

Purpose

Own structure declaration syntax.

Owns

- struct name;
- generic parameters;
- field declarations;
- field attributes;
- field ordering;
- struct declaration boundary.

Does not own

- memory layout;
- alignment;
- padding;
- ABI;
- serialization;
- hardware placement.

Universal-computing rule

A struct may contain any semantically valid type:

classical type
quantum type
tensor
resource
hardware abstraction
distributed value
AI type
future registered type

The struct grammar must not import every domain grammar merely to enumerate possible field types.

Canonical "typeExpression" remains the extension point.

---

14. "enums.g4"

Purpose

Own enumeration declaration syntax.

Owns

- enum declaration;
- enum variants;
- variant attributes;
- explicit source-level discriminant syntax if supported.

Does not own

- ABI discriminant representation;
- integer width selection;
- memory layout;
- serialization;
- hardware representation.

Scalability

No fixed maximum number of variants.

The grammar must use repetition rather than enumerating a fixed number of variants.

---

15. "unions.g4"

Purpose

Own tagged/alternative union declaration syntax.

Owns

- "unionDeclaration";
- union generic parameters;
- union variants;
- unit variants;
- tuple payloads;
- named-field payloads;
- variant attributes.

Does not own

- recursive type validation;
- exhaustiveness;
- layout;
- discriminant representation;
- ABI;
- quantum allocation;
- hardware mapping.

The existing union design explicitly follows this separation and supports arbitrary variant/payload repetition rather than machine-sized limits.

Important migration requirement

The current union grammar documents an older "ZamaniTokens" vocabulary while the declaration dispatcher is migrating toward "ZamaniLexer". This must be resolved at integration time; "declarations.g4" must not duplicate union syntax as a workaround.

The final architecture must have exactly one canonical lexer vocabulary.

---

16. "interfaces.g4"

Purpose

Own interface declaration syntax.

Owns

- interface declaration;
- interface generic parameters;
- required members;
- associated declarations where supported;
- interface inheritance/extension syntax.

Does not own

- implementation selection;
- dynamic dispatch;
- ABI;
- vtable layout;
- backend implementation;
- hardware implementation.

Universal-computing principle

An interface expresses a semantic contract.

It must not require a particular machine.

For example, an interface representing an accelerator capability must not inherently mean:

GPU 0

or:

FPGA X

---

17. "traits.g4"

Purpose

Own trait declaration syntax.

Owns

- trait declaration;
- generic parameters;
- trait bounds where syntactically attached;
- required/provided members;
- trait inheritance syntax.

Does not own

- trait resolution;
- specialization decisions;
- monomorphization;
- code generation;
- ABI.

Trait semantics belong to semantic analysis and compilation.

---

18. "implementations.g4"

Purpose

Own implementation declarations.

Owns

- implementation declaration;
- implemented interface/trait;
- target type;
- implementation members;
- generic implementation syntax.

Does not own

- method resolution;
- trait solving;
- dispatch;
- code generation;
- optimization;
- hardware mapping.

Implementation declarations must remain target-independent unless the language explicitly defines a semantic target annotation.

---

19. Shared dependency rules

Declaration grammars may depend on lower-level syntax contracts:

lexer
core/names
core/paths
core/attributes
types
expressions

They must not depend directly on:

quantum::ir
QEC
ZQN
resilience
optimization
routing
scheduling
hardware discovery
calibration
runtime

The dependency direction is:

lexer
  |
  v
core
  |
  v
types / expressions
  |
  v
declarations
  |
  v
frontend AST
  |
  v
semantic analysis
  |
  +--> classical semantic representation
  +--> quantum semantic representation
  +--> HDL/hardware representation
  +--> distributed representation
  +--> accelerator representation
  |
  v
IR/lowering

---

20. AST contract

Grammar files do not define Rust AST structures.

The parser produces parse-tree contexts.

The frontend maps them to the repository's canonical AST.

For every declaration the AST must preserve:

- declaration kind;
- source span;
- source ordering;
- identifier;
- generic parameters;
- modifiers;
- attributes;
- members;
- child expressions;
- child types;
- documentation where required.

The grammar must not introduce parallel AST systems such as:

DeclarationAst
QuantumDeclarationAst
HardwareDeclarationAst
UnionAst
StructIr

inside the grammar directory.

The AST belongs to the frontend.

---

21. Quantum integration

Declarations may contain quantum types.

For example, a declaration may semantically represent:

value : Qubit

or:

type QuantumResult<T> = ...

The declaration grammar does not decide:

- number of physical qubits;
- physical qubit IDs;
- gate topology;
- gate set;
- pulse model;
- calibration;
- QEC code;
- noise model;
- backend;
- scheduling;
- routing.

The semantic pipeline is:

declaration
    |
    v
AST
    |
    v
quantum semantic analysis
    |
    v
quantum::ir

"quantum::ir" remains the canonical quantum semantic boundary.

---

22. Classical integration

Declaration syntax must support types and values used by:

- scalar computation;
- vectors;
- matrices;
- tensors;
- numerical computing;
- symbolic computing;
- parallel computing;
- systems programming;
- AI/ML;
- data processing.

The declaration grammar must not enumerate every possible future classical type.

Instead:

declaration
    |
    v
typeExpression

provides the extensibility boundary.

---

23. HDL and hardware integration

Declarations may introduce semantic constructs used by HDL and hardware domains.

However, declaration syntax must not automatically imply physical implementation.

For example:

type Signal<T> = ...

does not imply:

register width = 32

or:

FPGA = device-7

or:

physical address = ...

Hardware realization belongs to:

hardware abstraction
resource model
target description
compiler
lowering
scheduling
runtime

---

24. Resource and capability integration

Declarations may carry attributes or type relationships representing semantic requirements.

The concepts remain distinct:

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
portability
scalability

The grammar must not collapse these concepts into a single target-selection construct.

For example:

requires quantum

is a semantic requirement.

It is not equivalent to:

requires device IBM_X

unless a future language specification explicitly defines such target pinning as source-level semantics.

---

25. No machine-size hard-coding

Every declaration grammar file must pass the following audit.

Forbidden:

MAX_FIELDS
MAX_VARIANTS
MAX_PARAMETERS
MAX_GENERIC_PARAMETERS
MAX_TYPES
MAX_DECLARATIONS
MAX_QUbits
MAX_DEVICES

and equivalent constructs.

Also forbidden are grammar structures that silently cap capacity, such as:

field1 field2 field3

instead of:

field*

when arbitrary repetition is semantically valid.

All practical implementation limits must be outside language semantics.

They may be represented by:

- parser resource policy;
- compiler resource policy;
- process limits;
- memory limits;
- execution limits;
- deployment configuration.

Such limits must never alter the meaning of a valid Zamani source program.

---

26. Infinite-scale interpretation

"Infinity" means that the language introduces no artificial finite machine limit.

It does not mean that a physical machine has infinite memory or infinite execution capacity.

Therefore:

Zamani source

may describe arbitrarily large declarations subject only to:

- available input;
- parser resources;
- compiler resources;
- runtime resources;
- target capabilities;
- semantic feasibility.

A resource-constrained machine may reject or defer a program because it cannot realize it.

That is different from the grammar itself imposing a fixed machine size.

---

27. ANTLR requirements

All declaration grammars must:

- use ANTLR4-compatible syntax;
- use the canonical lexer vocabulary;
- avoid embedded Rust actions;
- avoid embedded filesystem operations;
- avoid network access;
- avoid process execution;
- avoid hardware discovery;
- avoid runtime execution;
- avoid mutable global parser state;
- avoid semantic predicates unless explicitly justified by language semantics;
- preserve deterministic parsing;
- remain composable.

The generated parser is an implementation artifact.

It is not the canonical semantic representation.

---

28. Rust requirements

The parser/frontend implementation integrating these grammars must support:

Rust 1.97
Rust 1.97.1

and must use safe Rust.

Forbidden:

unsafe
unsafe fn
unsafe {}

The grammar itself contains no Rust implementation code.

Rust safety requirements therefore apply to:

- parser adapters;
- AST builders;
- diagnostics;
- semantic integration;
- grammar test infrastructure;
- compiler integration.

No declaration feature may require unsafe Rust merely to parse or represent it.

---

29. Source-order preservation

The declaration parser must preserve source ordering.

For example:

type A = ...
type B = ...
const C = ...

must remain ordered as:

A
B
C

in the parse-tree/AST contract.

The declaration grammar must not reorder declarations.

Ordering-dependent semantic rules belong to semantic analysis.

---

30. Attributes and metadata

Attributes may decorate declarations where supported.

The declaration grammar recognizes their syntactic structure through the canonical attribute grammar.

It does not interpret:

@quantum
@hardware
@resource
@compile
@runtime
@security

or future attributes.

Attribute interpretation belongs to the appropriate registry/semantic subsystem.

This allows future computing domains to be added without rewriting the declaration dispatcher.

---

31. Diagnostics boundary

Grammar diagnostics must describe syntax errors.

Examples:

expected identifier
expected type
expected '='
expected declaration body
expected ';'
unexpected declaration token

Semantic diagnostics do not belong here.

Examples that must be downstream:

duplicate declaration
unknown type
unsatisfied capability
insufficient resource
invalid quantum operation
invalid hardware mapping
invalid implementation
invalid trait constraint

This distinction is required for reliable compiler diagnostics.

---

32. Error recovery

The parser integration must support useful error recovery without changing valid-program semantics.

A syntax error must not:

- execute user code;
- access hardware;
- access the filesystem;
- access the network;
- inspect devices;
- select a backend.

Error recovery must remain deterministic.

---

33. Testing contract

Every declaration file must have dedicated tests.

At minimum:

grammar/tests/declarations/

must eventually contain coverage for:

- constants;
- variables;
- types;
- aliases;
- structs;
- enums;
- unions;
- interfaces;
- traits;
- implementations;
- declaration composition.

Tests should be divided into:

positive/
negative/
boundary/
cross-domain/
compatibility/
scalability/
determinism/
roundtrip/

---

34. Positive tests

Positive tests must demonstrate valid declarations.

Examples include:

const value: Int = 1;

let value: Int = 1;

type Identifier = SomeType;

alias Value = SomeType;

struct Point {
    x: Int,
    y: Int,
}

enum State {
    Ready,
    Running,
    Finished,
}

union Result<T, E> {
    Ok(T),
    Err(E),
}

Exact syntax must follow the authoritative grammar rather than this illustrative section.

---

35. Negative tests

Negative tests must verify rejection of malformed declarations.

Examples:

missing identifier
missing type
missing initializer
missing delimiter
malformed generic list
malformed field
malformed variant
malformed implementation
malformed interface

Semantic-invalid programs should only be included where the test harness explicitly distinguishes parser acceptance from semantic rejection.

---

36. Boundary tests

Boundary tests must verify that no declaration-family limit is accidentally encoded.

Test:

- one declaration;
- many declarations;
- one field;
- many fields;
- one generic parameter;
- many generic parameters;
- one enum variant;
- many enum variants;
- nested declarations where legal;
- deeply nested generic types;
- large source units.

The test suite must not define a language-level "maximum" merely because the test fixture happens to use a particular number.

---

37. Scalability tests

Scalability tests must verify source-level independence from:

qubit count
CPU count
GPU count
FPGA count
node count
memory size
device count
network size
accelerator count

The declaration grammar must remain unchanged as those physical dimensions grow.

---

38. Cross-domain tests

The declaration system must eventually be tested with combinations including:

classical + quantum

classical + HDL

quantum + HDL

quantum + hardware

quantum + distributed

AI + quantum

AI + hardware

classical + quantum + distributed

classical + quantum + HDL + hardware

The purpose is to prove that declaration syntax remains a neutral semantic layer rather than becoming domain-specific.

---

39. Determinism tests

Given:

same source
same grammar version
same lexer version
same configuration

the parser must produce the same structural result.

No declaration grammar may depend on:

- current time;
- random state;
- filesystem state;
- network state;
- machine discovery;
- environment-dependent parser behavior.

---

40. Round-trip tests

Where the frontend provides a canonical formatter/printer:

source
  |
  v
lexer
  |
  v
parser
  |
  v
AST
  |
  v
printer
  |
  v
source
  |
  v
parser

must preserve declaration semantics.

Formatting differences are acceptable.

Semantic changes are not.

---

41. Repository integration

The declaration directory must integrate with the repository in this direction:

grammar/
    |
    +-- lexer/
    |
    +-- core/
    |
    +-- types/
    |
    +-- expressions/
    |
    +-- declarations/
    |
    +-- functions/
    |
    +-- modules/
    |
    +-- effects/
    |
    +-- classical/
    |
    +-- quantum/
    |
    +-- hybrid/
    |
    +-- hdl/
    |
    +-- hardware/
    |
    +-- distributed/
    |
    +-- ai/
    |
    +-- data/
    |
    +-- resources/
    |
    +-- compile/
    |
    +-- execution/
    |
    +-- interoperability/

The declaration layer should be reusable by all of these domains.

---

42. Root grammar integration

The repository currently contains declaration syntax in the root "grammar/Zamani.g4".

That architecture must be migrated toward:

Zamani root parser
       |
       v
source-unit
       |
       v
declarations
       |
       +--> declaration delegates

There must not be two independent declaration systems.

The root grammar must eventually stop independently redefining declaration constructs such as:

functionDecl
structDecl
enumDecl
traitDecl
implDecl
interfaceDecl
typeAlias
constDecl
...

when those constructs have dedicated authoritative grammar owners.

The old rules should be:

1. audited;
2. mapped to the new owners;
3. migrated;
4. tested;
5. removed or retained only as explicitly documented compatibility/reference artifacts.

---

43. "core/source-unit.g4" integration

The current "grammar/core/source-unit.g4" also contains declaration forwarding and concrete declaration syntax.

This creates a potential ownership conflict.

The final architecture must make:

source-unit.g4

responsible for:

- source-unit boundaries;
- source-item ordering;
- EOF;
- source-level metadata/documentation;
- integration points.

It must not become a second concrete declaration grammar.

The final relationship should be:

source-unit
      |
      v
declaration
      |
      v
grammar/declarations/declarations.g4
      |
      v
concrete declaration delegates

---

44. No circular grammar dependencies

Forbidden:

declarations.g4
    |
    v
types.g4
    |
    v
declarations.g4

Forbidden:

declarations
    |
    v
quantum grammar
    |
    v
declarations

Forbidden:

declarations
    |
    v
hardware
    |
    v
runtime
    |
    v
declarations

Declaration grammars must form an acyclic dependency graph.

---

45. Canonical type integration

Declaration grammars frequently need type expressions.

They must use the canonical type subsystem.

They must not create competing forms such as:

declarationType
fieldType
unionTypeExpression
interfaceTypeExpression

when these are semantically the same concept.

The intended boundary is:

grammar/types/
        |
        v
canonical type expression
        |
        +--> constants
        +--> variables
        +--> aliases
        +--> structs
        +--> enums
        +--> unions
        +--> interfaces
        +--> traits
        +--> implementations

---

46. Canonical expression integration

Initializers, default values, discriminants, attributes, and other declaration constructs must use the canonical expression grammar.

Declaration files must not create duplicate expression precedence systems.

This prevents one declaration family from interpreting:

a + b * c

differently from another.

---

47. Domain extensibility

A new computing domain must not require rewriting every existing declaration grammar.

For example, adding a future domain must ideally require:

new domain grammar
        |
        v
canonical type/declaration extension
        |
        v
declaration dispatcher integration

rather than:

modify every existing declaration grammar

This is essential for POCO-REAF and long-term language evolution.

---

48. Quantum, HDL and hardware neutrality

The declaration layer must remain neutral toward:

CPU
GPU
FPGA
ASIC
QPU
NPU
TPU
DSP
accelerator
cluster
cloud
edge

These are execution/target concepts unless explicitly elevated into Zamani source-level semantics.

A source declaration must not become obsolete merely because hardware architecture changes.

---

49. Future-proofing

The grammar must support future declaration categories through controlled extension.

Future examples may include:

resource declaration
capability declaration
dialect declaration
component declaration
actor declaration
service declaration
accelerator declaration
quantum declaration
hardware declaration
distributed declaration

Such features must be introduced through dedicated specifications and grammar owners.

Do not place speculative syntax in existing files merely to reserve names.

---

50. Versioning

Declaration syntax is versioned through the language-version/compatibility system.

Each declaration construct must have:

- introduction version;
- compatibility status;
- deprecation status where applicable;
- migration path where applicable.

Changing the grammar must not silently change the meaning of existing programs.

---

51. Compatibility

Backward compatibility must distinguish:

Source compatibility

Old source continues to parse.

Semantic compatibility

Old source retains the same meaning.

AST compatibility

AST representation remains compatible where promised.

IR compatibility

Downstream canonical representations remain compatible where promised.

Runtime compatibility

Existing semantic programs remain executable where the target provides required capabilities.

Grammar compatibility alone does not guarantee runtime compatibility.

---

52. Deprecation

A declaration feature must never disappear silently.

The lifecycle is:

introduced
    |
    v
stable
    |
    v
deprecated
    |
    v
migration-supported
    |
    v
removed

Removal requires:

- documented version;
- diagnostic;
- migration guidance;
- compatibility entry;
- test updates.

---

53. Security

The declaration grammar must be inert.

Parsing declarations must never:

- execute code;
- access the filesystem;
- access the network;
- inspect hardware;
- invoke shell commands;
- load arbitrary plugins;
- access secrets;
- select credentials;
- mutate external state.

This remains true even for declarations describing:

- hardware;
- networking;
- security;
- quantum systems;
- distributed execution.

The grammar describes syntax only.

---

54. Resource-exhaustion protection

The language must not impose semantic limits such as:

MAX_FIELDS = 64
MAX_VARIANTS = 256
MAX_GENERIC_PARAMETERS = 32

If parser resource protection is necessary, it belongs outside the grammar.

The implementation may use explicit resource policies for:

- maximum input bytes;
- parser memory;
- parser time;
- recursion/resource budgets;
- compiler memory;
- compiler time.

Those are implementation safety controls, not language semantics.

---

55. Implementation order

The declaration directory must be completed in dependency order.

Recommended sequence:

1. declarations/README.md

2. canonical lexer/token contract

3. core names/paths/attributes contracts

4. canonical type-expression contract

5. canonical expression contract

6. constants.g4

7. variables.g4

8. types.g4

9. aliases.g4

10. structs.g4

11. enums.g4

12. unions.g4

13. interfaces.g4

14. traits.g4

15. implementations.g4

16. declarations.g4

17. source-unit/root parser integration

18. AST integration

19. semantic integration

20. cross-domain tests

This order allows the concrete declaration files to be completed before the dispatcher is finalized.

---

56. Independent-file completion rule

A file is not complete merely because ANTLR accepts it.

Before declaring a file complete, verify:

syntax defined
ownership defined
non-ownership defined
dependencies defined
integration contracts defined
AST contract defined
semantic boundary defined
scalability audited
hard-coding audited
positive tests complete
negative tests complete
boundary tests complete
compatibility considered
documentation complete

No later file should require changing an already-completed file's fundamental ownership model.

If a later integration exposes a genuine contradiction, the earlier file was not actually complete and must be reopened deliberately rather than silently patched.

---

57. Completion contract for every declaration file

Every declaration grammar must satisfy:

Purpose

Clearly documented.

Ownership

Every grammar rule has one owner.

Non-ownership

No duplicated responsibility.

Inputs

Canonical lexer/core/type/expression contracts identified.

Outputs

Parse-tree contract identified.

AST

Canonical AST mapping identified.

Semantics

Semantic validation responsibilities identified.

IR

Downstream IR boundary identified.

Compiler

Compilation integration identified.

Runtime

Explicitly documented as downstream/non-dependent unless source semantics require otherwise.

Tooling

Formatter, syntax highlighting, diagnostics and language-server implications identified.

Tests

Positive, negative, boundary, determinism and scalability tests identified.

Compatibility

Version/deprecation requirements identified.

Security

No external side effects.

Scalability

No artificial machine limits.

Hard-coding

Audit completed.

---

58. Production readiness checklist

The declaration grammar is production-ready only when all of the following are true.

Architecture

- [ ] One declaration dispatcher exists.
- [ ] Each declaration family has one owner.
- [ ] No circular grammar dependencies exist.
- [ ] Root grammar does not duplicate concrete declaration syntax.
- [ ] "source-unit.g4" does not duplicate concrete declaration syntax.
- [ ] Canonical lexer vocabulary is used consistently.

Syntax

- [ ] Constants complete.
- [ ] Variables complete.
- [ ] Types complete.
- [ ] Aliases complete.
- [ ] Structs complete.
- [ ] Enums complete.
- [ ] Unions complete.
- [ ] Interfaces complete.
- [ ] Traits complete.
- [ ] Implementations complete.

Semantics boundary

- [ ] Grammar does not perform semantic analysis.
- [ ] Grammar does not construct IR.
- [ ] Grammar does not select hardware.
- [ ] Grammar does not allocate resources.
- [ ] Grammar does not perform routing.
- [ ] Grammar does not schedule.
- [ ] Grammar does not execute.

Quantum

- [ ] Quantum-containing types are accepted through canonical type rules.
- [ ] No fixed qubit count exists.
- [ ] No physical qubit IDs are embedded.
- [ ] No backend IDs are embedded.
- [ ] No topology assumptions exist.
- [ ] No QEC implementation exists in declaration grammar.
- [ ] No ZQN implementation exists in declaration grammar.
- [ ] "quantum::ir" remains downstream.

Hardware

- [ ] No CPU count is hard-coded.
- [ ] No GPU count is hard-coded.
- [ ] No FPGA count is hard-coded.
- [ ] No ASIC identity is hard-coded.
- [ ] No physical address is hard-coded.
- [ ] No machine topology is hard-coded.

Scalability

- [ ] No declaration-count limit.
- [ ] No field-count limit.
- [ ] No variant-count limit.
- [ ] No generic-parameter limit.
- [ ] No payload-arity limit.
- [ ] No type nesting limit encoded in grammar.
- [ ] No hardware-scale limit.
- [ ] No quantum-scale limit.

Safety

- [ ] No unsafe Rust.
- [ ] No parser actions executing Rust.
- [ ] No filesystem access.
- [ ] No network access.
- [ ] No process execution.
- [ ] No hardware discovery.
- [ ] No runtime execution.

Testing

- [ ] Positive tests.
- [ ] Negative tests.
- [ ] Boundary tests.
- [ ] Scalability tests.
- [ ] Cross-domain tests.
- [ ] Determinism tests.
- [ ] Round-trip tests.
- [ ] Compatibility tests.

---

59. Final integration graph

The final declaration architecture is:

                         Zamani source
                              |
                              v
                       canonical lexer
                              |
                              v
                         core grammar
                              |
              +---------------+---------------+
              |                               |
              v                               v
        type grammar                   expression grammar
              |                               |
              +---------------+---------------+
                              |
                              v
                    declaration grammar
                              |
             +----------------+----------------+
             |                |                |
             v                v                v
          values             types          behaviors
             |                |                |
       +-----+-----+    +-----+------+    +----+------+
       |           |    |     |      |    |           |
    constants   vars  aliases structs enums unions interfaces traits impls
                              |
                              v
                         frontend AST
                              |
                              v
                       semantic analysis
                              |
       +----------------------+-----------------------+
       |                      |                       |
       v                      v                       v
 classical semantic      quantum semantic       hardware/HDL
 representation          representation         representation
                              |
                              v
                         quantum::ir
                              |
                 +------------+------------+
                 |            |            |
                 v            v            v
                QEC          ZQN      optimization
                                             |
                                             v
                                  routing / scheduling
                                             |
                                             v
                                      hardware HAL
                                             |
                                             v
                                          runtime

The declaration grammar is therefore a front-end syntax boundary, not a compiler/runtime subsystem.

---

60. Final architectural rule

The declaration directory must preserve this invariant:

«A Zamani declaration describes a source-level semantic entity, not an accidental limitation of the machine currently available.»

Therefore:

one declaration
      |
      v
one semantic meaning
      |
      +----> tiny machine
      +----> CPU
      +----> multicore
      +----> GPU
      +----> FPGA
      +----> ASIC
      +----> QPU
      +----> simulator
      +----> accelerator
      +----> cluster
      +----> supercomputer
      +----> cloud
      +----> future machine

subject only to the target's ability to satisfy the program's actual semantic requirements.

The declaration grammar must never become the place where today's hardware limitations become tomorrow's permanent language limitations.

Zamani declaration architecture therefore follows:

Describe once.
Preserve meaning.
Compile once where the compilation model permits.
Lower according to capabilities.
Adapt realization to available resources.
Run everywhere.
Run anywhere.
Remain extensible forever.

That is the declaration-level foundation for:

Zamani — From Atom to Everywhere

and:

POCO-REAF — Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.