Worked for 41s

I inspected the actual repository files relevant to this directory rather than treating the previous architecture as hypothetical. In particular, the current grammar/declarations/ contains 16 files, including attributes.g4, capabilities.g4, classes.g4, domains.g4, records.g4, and resources.g4, which the existing README does not fully account for.

I also found a concrete production blocker in the current declarations.g4: its final rule is self-recursive as domainDeclaration : domainDeclaration ;, so the current dispatcher cannot correctly delegate to domains.g4.  There are also lexer-vocabulary inconsistencies: the canonical direction is ZamaniLexer, while some declaration/type/resource grammars still reference ZamaniTokens or legacy token names.

Below is the replacement grammar/declarations/README.md I would use. It preserves existing filenames, incorporates the newer declaration files, defines their ownership and integration contracts in advance, and makes the lexer normalization requirements explicit.

# Zamani Declaration Grammar

Path: `grammar/declarations/README.md`

Status: **Production architecture contract**

Language: **Zamani Universal Computing Language**

Grammar technology: **ANTLR4 parser grammars**

Compiler baseline:

- Rust 1.97
- Rust 1.97.1
- Rust 2021

Safety:

- Safe Rust only
- No `unsafe`
- No embedded Rust actions in grammar
- No filesystem access from grammar
- No network access from grammar
- No process execution from grammar
- No hardware discovery from grammar
- No runtime execution from grammar

Primary objectives:

- Universal computation from atom to everywhere
- Classical computation
- Quantum computation
- Hybrid computation
- HDL and hardware/software co-design
- Embedded systems
- Distributed systems
- Parallel/HPC
- AI/ML
- Data computation
- Networking
- Security/cryptography
- Accelerators
- Future computational domains
- Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

# 1. Purpose

`grammar/declarations/` owns the **source-level declaration syntax** of Zamani.

A declaration introduces or describes a named source-level entity.

Examples include:

- constants;
- variables;
- named types;
- aliases;
- structs;
- records;
- enums;
- unions;
- classes;
- interfaces;
- traits;
- implementations;
- resources;
- capabilities;
- domains;
- future declaration families explicitly admitted by the language specification.

The declaration layer is a **syntax boundary**.

It is not:

- a semantic analyzer;
- a type checker;
- a resource allocator;
- a hardware selector;
- a quantum compiler;
- a QEC implementation;
- a ZQN implementation;
- a routing engine;
- a scheduler;
- a runtime;
- an IR.

The required architecture is:

```text
Zamani source
      |
      v
canonical lexer
      |
      v
parser
      |
      v
declaration grammar
      |
      v
domain-neutral frontend AST
      |
      v
structural validation
      |
      v
semantic analysis
      |
      +------------------+------------------+
      |                  |                  |
      v                  v                  v
 classical          quantum             HDL/hardware
 semantics          semantics           semantics
                       |
                       v
                   quantum::ir
                       |
                       v
          optimization / lowering
                       |
          +------------+-------------+
          |            |             |
          v            v             v
        QEC           ZQN        optimization
                                      |
                                      v
                              routing / scheduling
                                      |
                                      v
                                      HAL
                                      |
                                      v
                                    runtime

The declaration grammar MUST NOT bypass this pipeline.


---

2. Fundamental invariant

A Zamani declaration describes source-level meaning.

It does not describe an accidental property of the machine currently available.

Therefore:

source semantics != physical realization

A declaration must not silently encode:

CPU count;

core count;

thread count;

GPU count;

FPGA count;

ASIC identity;

QPU identity;

qubit count;

physical qubit identifiers;

memory capacity;

register count;

register width;

vector width;

tensor dimension limits;

node count;

cluster size;

network topology;

device address;

accelerator identity;

deployment environment.


A program may express a genuine semantic requirement.

For example:

requires quantum
requires capability("quantum.measurement")
requires memory(required_memory)
requires resource(compute)

Those statements describe requirements or capabilities.

They must not automatically become:

use device 7
use qpu 2
use physical qubit 31
use GPU 0
use CPU core 3

Physical realization belongs downstream.


---

3. POCO-REAF

The declaration layer participates in:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

A declaration should retain the same source-level meaning when compiled or lowered toward:

tiny embedded systems;

microcontrollers;

CPUs;

multicore CPUs;

GPUs;

FPGAs;

ASICs;

QPUs;

quantum simulators;

NPUs;

TPUs;

DSPs;

accelerators;

clusters;

supercomputers;

distributed systems;

edge systems;

cloud systems;

future computational substrates.


The grammar therefore defines portable source semantics.

The target determines how those semantics are realized.


---

4. Meaning of "infinity"

"Infinity" does not mean that a physical machine has infinite resources.

It means:

> The language and grammar must not impose an artificial finite machine limit where the underlying semantic model does not require one.



For example:

struct Huge {
    ...
}

must not have a grammar-level:

MAX_FIELDS = 64

Likewise:

type Tensor<T, Shape> = ...

must not be restricted by a grammar-level:

MAX_TENSOR_RANK = 8

Similarly, the declaration layer must not define:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_DEVICES
MAX_GENERIC_PARAMETERS
MAX_FIELDS
MAX_VARIANTS
MAX_IMPLEMENTATIONS

Practical limits may exist in:

parser resource policies;

compiler resource policies;

process limits;

memory availability;

runtime policies;

target capabilities;

deployment constraints.


Those are implementation constraints, not language semantics.


---

5. Actual repository state

The current repository contains the following declaration files:

grammar/declarations/
├── README.md
├── aliases.g4
├── attributes.g4
├── capabilities.g4
├── classes.g4
├── constants.g4
├── declarations.g4
├── domains.g4
├── enums.g4
├── implementations.g4
├── interfaces.g4
├── records.g4
├── resources.g4
├── structs.g4
├── traits.g4
├── types.g4
├── unions.g4
└── variables.g4

The existing repository inventory confirms these files are already present.

Therefore:

Do not create duplicate replacements for these files.

Existing names should be retained.


---

6. Declaration ownership map

File	Owns

declarations.g4	declaration dispatch only
constants.g4	constant declaration syntax
variables.g4	variable declaration syntax
types.g4	named type declaration syntax
aliases.g4	alias declaration syntax
structs.g4	struct declaration syntax
records.g4	record declaration syntax
enums.g4	enum declaration syntax
unions.g4	union declaration syntax
classes.g4	class declaration syntax
interfaces.g4	interface declaration syntax
traits.g4	trait declaration syntax
implementations.g4	implementation declaration syntax
resources.g4	resource declaration syntax
capabilities.g4	declaration-layer capability integration
domains.g4	domain declaration syntax
attributes.g4	declaration-related attribute syntax/integration


No file may silently become the owner of another file's syntax.


---

7. Single dispatcher rule

declarations.g4 is the single declaration dispatcher.

It owns:

declaration

and only the composition needed to select a concrete declaration family.

It must not implement:

constantDeclaration
variableDeclaration
typeDeclaration
typeAliasDeclaration
structDeclaration
recordDeclaration
enumDeclaration
unionDeclaration
classDeclaration
interfaceDeclaration
traitDeclaration
implementationDeclaration
resourceDeclaration
capabilityDeclaration
domainDeclaration

Those rules belong to their dedicated owners.

The dispatcher must therefore remain small and stable.


---

8. Critical correction to the current dispatcher

The current declarations.g4 contains:

domainDeclaration
    : domainDeclaration
    ;

This is invalid architecture because the dispatcher recursively references its own rule instead of delegating to the actual domains.g4 owner.

The final dispatcher must instead import the domain grammar and expose its actual concrete rule.

The correct conceptual relationship is:

declarations.g4
       |
       v
ZamaniDomains
       |
       v
domainDeclaration

not:

declarations.g4
       |
       X
       |
domainDeclaration
       |
       +--> domainDeclaration

This correction is mandatory.


---

9. Canonical declaration dispatcher contract

The intended dispatcher shape is:

parser grammar ZamaniDeclarations;

options {
    tokenVocab = ZamaniLexer;
}

import
    Constants,
    Variables,
    ZamaniDeclarationTypesParser,
    ZamaniDeclarationAliases,
    ZamaniDeclarationStructs,
    ZamaniDeclarationEnums,
    ZamaniDeclarationUnions,
    ZamaniDeclarationRecords,
    ZamaniClasses,
    Interfaces,
    Traits,
    ZamaniImplementations,
    Resources,
    CapabilityDeclarations,
    ZamaniDomains
;

declaration
    : valueDeclaration
    | typeDeclarationFamily
    | aggregateDeclaration
    | objectContractDeclaration
    | resourceCapabilityDeclaration
    | domainDeclaration
    ;

valueDeclaration
    : constantDeclaration
    | variableDeclaration
    ;

typeDeclarationFamily
    : typeDeclaration
    | typeAliasDeclaration
    ;

aggregateDeclaration
    : structDeclaration
    | recordDeclaration
    | enumDeclaration
    | unionDeclaration
    ;

objectContractDeclaration
    : classDeclaration
    | interfaceDeclaration
    | traitDeclaration
    | implementationDeclaration
    ;

resourceCapabilityDeclaration
    : resourceDeclaration
    | declarationCapability
    ;

The exact imported grammar names must remain synchronized with the actual parser grammar declarations in each delegate.

For example, the repository currently identifies:

aliases.g4          -> ZamaniDeclarationAliases
types.g4            -> ZamaniDeclarationTypesParser
records.g4          -> ZamaniDeclarationRecords
classes.g4          -> ZamaniClasses
implementations.g4  -> ZamaniImplementations
capabilities.g4     -> CapabilityDeclarations
domains.g4          -> ZamaniDomains

The repository confirms these grammar names in the corresponding files.


---

10. No artificial declaration grouping

The grouping rules in declarations.g4 exist only to make the dispatcher understandable.

They have no semantic meaning.

For example:

aggregateDeclaration
    : structDeclaration
    | recordDeclaration
    | enumDeclaration
    | unionDeclaration
    ;

does not imply that structs, records, enums, and unions share an IR.

It merely groups syntactically related declaration families.

The frontend AST and semantic layer remain authoritative for meaning.


---

11. Constants

constants.g4 owns:

constant declaration syntax;

constant identifier;

optional declared type;

initializer attachment;

declaration-level attributes/modifiers explicitly assigned to constants.


It does not own:

constant evaluation;

constant folding;

compile-time execution;

type checking;

memory placement;

register allocation;

ABI;

hardware realization.


Pipeline:

constantDeclaration
        |
        v
AST
        |
        v
type analysis
        |
        v
constant evaluation
        |
        v
IR/compiler

Scalability:

unlimited source-level constants;

no fixed initializer size;

no target-specific representation.



---

12. Variables

variables.g4 owns:

variable declaration syntax;

identifier;

mutability;

optional type;

optional initializer;

declaration-local attributes/modifiers.


It does not own:

allocation;

ownership;

borrowing;

lifetime;

stack/register decisions;

physical memory;

accelerator memory;

quantum memory.


Pipeline:

variableDeclaration
        |
        v
AST
        |
        v
ownership/type/effect analysis
        |
        v
semantic representation
        |
        v
compiler/lowering

A variable must never imply a physical storage location.


---

13. Named types

types.g4 owns type declarations, not the entire type system.

The complete type-expression authority remains under:

grammar/types/

The existing types.g4 explicitly states that it does not own primitive/composite type syntax and that the complete type-expression system belongs under grammar/types/.

Therefore:

typeDeclaration
        |
        v
canonical typeExpression

must be the final integration boundary.

types.g4 must not become another complete implementation of typeExpression.


---

14. Type aliases

aliases.g4 owns alias declaration syntax.

The alias grammar must reuse the canonical type-expression grammar.

It must not invent:

aliasType
fieldType
unionType
interfaceType
quantumTypeAlias
hardwareTypeAlias

when those represent the same underlying type language.

The canonical structure is conceptually:

alias declaration
       |
       +--> name
       +--> generic parameters
       +--> target type expression

Semantic analysis handles:

alias resolution;

substitution;

normalization;

cycles;

compatibility;

recursive definitions.



---

15. Structs

structs.g4 owns:

struct name;

generic parameters;

fields;

field attributes;

field ordering;

declaration body.


It does not own:

memory layout;

padding;

alignment;

ABI;

serialization;

physical placement;

register allocation;

hardware mapping.


A struct field may eventually contain:

classical types;

quantum types;

tensors;

resource abstractions;

hardware abstractions;

distributed values;

AI/data types;

future types.


The struct grammar must not import every domain merely to enumerate possible field types.

The canonical typeExpression extension point handles this.


---

16. Records

records.g4 is an independent declaration owner.

It owns record-specific syntax.

It must not be treated as an alias for structs unless the language specification explicitly says they are syntactic aliases.

The record contract must define in advance:

name;

generics;

fields;

attributes;

modifiers;

ordering;

optional record-specific semantics;

AST mapping;

semantic validation;

IR/lowering consumer;

formatter support;

diagnostics;

tests.


No physical representation belongs in the grammar.


---

17. Enums

enums.g4 owns:

enum declaration;

variant declaration;

variant attributes;

optional source-level discriminant expressions if specified.


It does not own:

ABI discriminant width;

integer representation;

layout;

serialization;

backend representation.


There is no grammar-level maximum number of variants.

Use repetition:

enumBody
    : LBRACE enumVariant* RBRACE
    ;

where semantically appropriate.

Never encode:

variant1
variant2
variant3
...
variant64

as a universal limit.


---

18. Unions

unions.g4 owns:

union declaration;

generic parameters;

variants;

unit variants;

tuple payloads;

named-field payloads;

attributes.


It does not own:

exhaustiveness;

recursive-type validity;

discriminant layout;

ABI;

quantum allocation;

hardware mapping.


The current union implementation is one of the files that must be normalized to the canonical lexer vocabulary rather than maintaining legacy token names.

Do not duplicate union syntax inside declarations.g4.


---

19. Classes

classes.g4 is the concrete owner of class syntax.

The repository's class grammar already treats class declarations as source-level, domain-neutral constructs and explicitly separates them from physical hardware, QEC, ZQN, routing, scheduling, and runtime concerns.

It owns:

class declaration;

inheritance syntax;

implemented contracts;

permitted types;

class body;

fields;

methods;

constructors;

properties;

associated types;

associated constants;

nested types.


It does not own:

object layout;

ABI;

dynamic dispatch implementation;

memory placement;

hardware selection;

QPU selection;

physical qubits;

QEC;

ZQN.



---

20. Interfaces

interfaces.g4 owns interface syntax.

It expresses source-level contracts.

It does not determine:

vtable layout;

dispatch implementation;

ABI;

backend;

hardware;

accelerator;

runtime implementation.


An interface must remain portable.


---

21. Traits

traits.g4 owns trait syntax.

It does not own:

trait solving;

specialization;

monomorphization;

implementation selection;

code generation;

ABI.


Trait semantics belong downstream.


---

22. Implementations

implementations.g4 owns implementation declarations.

It owns:

target type;

implemented trait/interface;

generic implementation syntax;

implementation members.


It does not own:

trait solving;

method resolution;

dispatch;

specialization;

optimization;

backend selection.


The implementation grammar must remain target-independent.


---

23. Resources

resources.g4 owns resource declaration syntax.

The existing resource grammar already models concepts such as:

resources;

resource groups;

contracts;

profiles;

requirements;

constraints;

preferences;

hints;

capabilities;

targets;

quantities;

capacities;

availability;

performance;

latency;

throughput;

bandwidth;

energy;

power;

reliability;

resilience;

cost;

portability;

scalability;

reservation;

acquisition;

release;

derived resource values.


The resource grammar must remain target-independent.

It must not select physical resources.

For example:

resource quantum_compute

does not mean:

use QPU 0


---

24. Capabilities

capabilities.g4 is a declaration-layer adapter.

The repository explicitly defines the canonical capability syntax in:

grammar/core/capabilities.g4

and grammar/declarations/capabilities.g4 exists to integrate that capability grammar into declaration dispatch.

Therefore:

core/capabilities.g4
        |
        v
declarations/capabilities.g4
        |
        v
declarations.g4

is the correct direction.

Do not duplicate capability syntax in declarations.g4.


---

25. Domains

domains.g4 owns source-level domain declarations.

The repository defines ZamaniDomains as the grammar owner.

A domain is a source-level semantic/namespace boundary.

Possible domains include:

classical;

quantum;

hybrid;

HDL;

hardware;

distributed;

AI;

data;

networking;

security;

scientific;

embedded;

accelerator;

future domains.


These are not separate programming languages.

A domain must not create:

another lexer;

another parser;

another AST;

another type system;

another quantum IR.



---

26. Attributes

attributes.g4 owns declaration-related attribute syntax where applicable.

It does not own the semantic interpretation of every attribute.

For example:

@quantum
@hardware
@resource
@compile
@runtime
@security

may be syntactically recognized.

Their meaning belongs to the appropriate semantic registry.

This allows future domains to add attributes without forcing the dispatcher to understand every domain.


---

27. Canonical lexer authority

There must be exactly one canonical parser-visible lexer vocabulary.

The target is:

grammar/lexer/*.g4
        |
        v
grammar/lexer/tokens.g4
        |
        v
grammar/antlr/ZamaniLexer.g4
        |
        v
parser grammars

The lexer architecture already identifies grammar/lexer/tokens.g4 as the lexical composition boundary and ZamaniLexer as the parser-facing lexer.

Therefore declaration grammars MUST use:

options {
    tokenVocab = ZamaniLexer;
}


---

28. Forbidden legacy lexer vocabularies

Declaration grammars must not introduce or depend on competing vocabularies such as:

ZamaniTokens
K_*
IDENT
SEMI
legacy token aliases

Some existing repository grammars still use ZamaniTokens, including resource-related and type-related grammar surfaces.

Those are migration blockers.

Do not work around them by adding aliases inside declarations.g4.

Instead:

legacy token
      |
      v
canonical lexer mapping
      |
      v
ZamaniLexer


---

29. Declaration lexer token inventory

The following lexical vocabulary is required by the current declaration layer and must be resolved against the canonical grammar/lexer/ components.

29.1 Binding/declaration keywords

CONST
LET
VAR
MUT

TYPE
ALIAS

STRUCT
RECORD
ENUM
UNION

CLASS
INTERFACE
TRAIT
IMPL

FN

FN is required by declarations that contain function/method syntax, but function syntax remains owned by the function grammar.


---

30. Visibility and declaration modifiers

The canonical lexer currently defines:

PUBLIC
PUB
PRIVATE
PROTECTED
INTERNAL

STATIC
OVERRIDE
VIRTUAL
ABSTRACT
FINAL
SEALED
PARTIAL

EXTENDS
IMPLEMENTS

WHERE

These spellings are already present in grammar/lexer/keywords.g4.

Do not introduce duplicate forms such as:

K_PUBLIC
K_PUB
K_PRIVATE


---

31. Resource/capability declaration vocabulary

The declaration/resource architecture requires canonical lexical ownership for:

RESOURCE
RESOURCES

CAPABILITY
TARGET

REQUIRES
CONSTRAINT
PREFER
HINT

QUANTITY
CAPACITY
AVAILABILITY

PORTABILITY
SCALABILITY

PERFORMANCE
LATENCY
THROUGHPUT
BANDWIDTH

ENERGY
POWER

RELIABILITY
RESILIENCE

COST

RESERVE
ACQUIRE
RELEASE

DERIVE

GROUP

Additional resource vocabulary already present in grammar/lexer/keywords.g4 must remain in that lexer authority rather than being recreated in declaration grammars.


---

32. Quantum declaration vocabulary

The declaration layer may reference quantum types and semantic declarations.

The canonical lexer already defines language-level quantum vocabulary including:

QUANTUM
CIRCUIT
QUBIT

APPLY
MEASURE
RESET
BARRIER
CONTROL
ADJOINT
INVERSE
OBSERVE

ENTANGLE
NOISE
FIDELITY
SURFACE
CODE
LOGICAL
PARITY

These are lexical categories only.

The declaration grammar must not reserve every quantum gate.

The following must remain extensible identifiers unless the language specification explicitly promotes a spelling to a keyword:

H
X
Y
Z
CNOT
CX
U
RX
RY
RZ
custom_gate
vendor.operation
logical_operation

The existing keyword contract explicitly follows this extensible approach.


---

33. Core punctuation required by declarations

The canonical lexer must provide one stable token identity for every structural punctuation concept used by declaration grammars.

At minimum the declaration layer requires concepts corresponding to:

{
}
(
)
[
]
<
>
:
;
,
.
=
@
?

and, where used:

->
=>
::
+
*
&
|

The exact canonical token names must come from grammar/lexer/operators.g4 and grammar/lexer/punctuation.g4.

Do not maintain both:

LPAREN
LEFT_PAREN

for the same lexical spelling.

Likewise do not maintain both:

RPAREN
RIGHT_PAREN

for the same lexical spelling.

The current repository already exhibits this type of vocabulary drift: different modular grammars use different names for the same punctuation concepts. That must be normalized at the lexer layer, not patched individually in the dispatcher.


---

34. Canonical identifier token

The declaration system must use the canonical:

IDENTIFIER

token.

It must not introduce:

IDENT
NAME
TYPE_NAME
CLASS_NAME
FIELD_NAME
RESOURCE_NAME
QUANTUM_NAME

as competing lexical identities when they represent ordinary source identifiers.

Context-sensitive meaning belongs to parsing and semantic analysis.


---

35. Literal tokens

Declaration initializers, discriminants, defaults, attributes, and type-level values may consume canonical literal tokens.

The lexer architecture already separates literal families.

The declaration layer must therefore consume the canonical forms for:

INTEGER_LITERAL
DECIMAL_LITERAL
STRING_LITERAL
CHARACTER_LITERAL
BOOLEAN_LITERAL

QUANTUM_LITERAL
HARDWARE_LITERAL
DURATION_LITERAL
SIZE_LITERAL

if those tokens remain part of the canonical lexer.

Literal implementation must not introduce machine-size assumptions.


---

36. Boolean/null vocabulary

Where declaration expressions accept these values, canonical lexical ownership is:

TRUE
FALSE
NIL
NULL

The declaration grammar must not redefine them.


---

37. Token normalization work required before declaration completion

The declaration directory is not independently complete until all imported grammars converge on ZamaniLexer.

Required normalization:

ZamaniTokens
       |
       X
       |
       v
ZamaniLexer

and:

K_RESOURCE
K_CAPABILITY
K_UNION
K_LBRACE
K_RBRACE
K_COLON
K_SEMICOLON
...

must be eliminated from production declaration grammars unless they are genuine canonical tokens defined by ZamaniLexer.

The correct approach is:

grammar/lexer/
      |
      +--> keywords.g4
      +--> operators.g4
      +--> punctuation.g4
      +--> identifiers.g4
      +--> literals.g4
      +--> comments.g4
      |
      v
tokens.g4
      |
      v
ZamaniLexer.g4
      |
      v
all parser grammars


---

38. Type-system integration

Declaration grammars depend on:

grammar/types/

for canonical type expressions.

They must not duplicate:

primitive types;

arrays;

slices;

tuples;

maps;

references;

function types;

generic applications;

quantum types;

resource types;

hardware types;

tensor types;

dependent types;

linear types;

affine types.


The existing type declaration grammar itself currently contains forwarding/duplicate type-expression rules. Those rules must ultimately converge on the authoritative grammar/types/ implementation rather than becoming a second type system.


---

39. Expression integration

Declaration initializers and other declaration-level expressions must use:

grammar/expressions/

as the canonical expression authority.

Do not create:

constantExpression
fieldExpression
enumExpression
resourceExpression
classExpression

as independent precedence systems when they are ordinary Zamani expressions.

Specialized expression forms are allowed only when they have genuinely distinct language semantics.


---

40. Generic integration

Generic syntax must have one authoritative owner.

Declaration grammars may attach generic parameters to declarations.

They must not duplicate the complete generic type-expression system.

The architecture is:

generic syntax
      |
      +--> declaration generics
      |
      +--> type applications
      |
      +--> function generics
      |
      +--> trait/interface constraints

Semantic analysis handles:

bounds;

substitution;

inference;

coherence;

specialization;

satisfiability.


No generic-parameter count limit belongs in grammar.


---

41. Source-order preservation

The parser must preserve source order.

For:

type A = ...
type B = ...
const C = ...

the frontend must receive:

A
B
C

in that order.

The declaration grammar must not:

sort;

deduplicate;

normalize;

reorder.


Semantic analysis may later build symbol tables or indexes.


---

42. AST contract

The declaration grammar produces parse-tree contexts.

It does not create Rust AST nodes.

The frontend must map every declaration into the existing domain-neutral AST.

Every declaration AST representation must preserve, where applicable:

declaration kind;

source span;

source ordering;

identifier;

qualified name;

attributes;

visibility;

modifiers;

generic parameters;

generic bounds;

members;

variants;

fields;

child types;

child expressions;

documentation;

source-origin metadata.


No declaration grammar may create:

QuantumDeclarationIR
HardwareDeclarationIR
ClassIR
StructIR
QuantumClassAst
HardwareAst

inside the grammar layer.


---

43. Semantic contract

Parsing establishes syntax only.

Semantic analysis determines:

duplicate declarations;

name resolution;

scope;

type validity;

generic validity;

bounds;

trait satisfaction;

interface satisfaction;

implementation coherence;

ownership;

effects;

capabilities;

resource requirements;

resource feasibility;

portability;

domain validity;

target compatibility.


The parser must not perform these operations.


---

44. Quantum integration

A declaration may contain or introduce quantum-related types.

For example:

type QState<T> = ...

or:

struct QuantumResult {
    ...
}

The declaration grammar does not determine:

physical qubit count;

physical qubit identifiers;

QPU topology;

gate set;

calibration;

pulse schedule;

QEC code;

noise model;

backend;

routing;

scheduling.


The correct direction is:

declaration
    |
    v
frontend AST
    |
    v
semantic quantum representation
    |
    v
quantum::ir

quantum::ir remains the canonical quantum semantic boundary.


---

45. Classical integration

Declarations must support the complete canonical type system needed for:

scalar computation;

integer computation;

floating-point computation;

vectors;

matrices;

tensors;

symbolic computation;

scientific computation;

numerical computation;

parallel computation;

systems programming;

AI/ML;

data processing.


The declaration grammar must not enumerate every possible classical data type.

The canonical type system is the extension point.


---

46. HDL integration

HDL declarations may eventually introduce:

modules;

interfaces;

signals;

registers;

memories;

state machines;

pipelines;

protocols;

hardware resources;

timing intent.


The declaration layer must remain source-level.

For example:

type Signal<T> = ...

does not imply:

32-bit register
FPGA 3
memory bank 2
physical address X

Hardware realization belongs downstream.


---

47. Hardware/software co-design

A declaration may participate in a hardware/software co-design program.

It may express:

resource intent;

capability intent;

timing requirements;

performance requirements;

memory requirements;

accelerator contracts;

communication contracts;

reliability requirements.


It must not directly select:

GPU 0
FPGA 1
QPU 2
CPU core 7

unless a separate, explicitly specified target-pinning feature gives such syntax defined source-level semantics.


---

48. Distributed integration

Declarations must remain independent of the actual number of:

nodes;

processes;

services;

replicas;

workers;

accelerators;

network links.


A declaration may describe a distributed abstraction.

It must not encode an artificial maximum number of nodes.


---

49. AI/ML integration

Declarations must support AI/ML types and abstractions through the canonical type and semantic systems.

Possible consumers include:

models;

tensors;

datasets;

agents;

training artifacts;

inference artifacts;

probabilistic values;

differentiable computations.


The declaration grammar must not become a parser for a particular framework.

Do not hard-code:

PyTorch
TensorFlow
JAX
CUDA
ROCm
vendor model names

into the declaration syntax merely because an implementation currently consumes them.


---

50. Resource/capability separation

These concepts must remain distinct:

resource
capability
requirement
constraint
preference
hint
target
placement
performance
latency
energy
reliability
portability
scalability

Definitions:

Resource

Something a computation can consume or use.

Capability

Something an execution environment can provide.

Requirement

Something the program requires.

Constraint

A condition that must hold.

Preference

A desirable implementation property.

Hint

Advisory implementation information.

Target

An execution context/profile.

Placement

A downstream realization decision.

The grammar must not collapse these into one concept.


---

51. Hardware independence

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
future substrate

These are target/runtime concepts unless explicitly represented as source-level semantic abstractions.

The grammar must not become obsolete when hardware architecture changes.


---

52. Domain extensibility

A new domain must not require rewriting all declaration grammars.

For example, adding a future:

optical
neuromorphic
biological
photonic
nano
post-quantum
future-computational-domain

should use:

new domain grammar
       |
       v
canonical type/declaration interfaces
       |
       v
semantic model
       |
       v
IR/lowering

rather than modifying every existing declaration grammar.


---

53. Declaration dispatcher extensibility

declarations.g4 is deliberately a controlled exception.

A new top-level declaration family requires a dispatcher integration.

However, the dispatcher must only add:

one delegate grammar
+
one dispatcher alternative
+
its integration contract
+
its tests

It must not copy the concrete grammar.

Therefore the cost of adding a declaration family remains bounded.


---

54. Dependency direction

Allowed:

lexer
  |
  v
core
  |
  +--> names
  +--> paths
  +--> attributes
  +--> visibility
  +--> modifiers
  |
  v
types / expressions / generics
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
  v
canonical semantic representations

Forbidden:

declarations -> quantum::ir
declarations -> QEC
declarations -> ZQN
declarations -> routing
declarations -> scheduling
declarations -> HAL
declarations -> runtime
declarations -> physical hardware


---

55. No circular dependencies

Forbidden:

declarations
      |
      v
types
      |
      v
declarations

Forbidden:

declarations
      |
      v
quantum
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

Grammar dependencies must remain acyclic.


---

56. Root grammar integration

The canonical root:

grammar/Zamani.g4

must compose the declaration dispatcher.

It must not independently maintain a second implementation of:

constant declaration
variable declaration
type declaration
alias declaration
struct declaration
record declaration
enum declaration
union declaration
class declaration
interface declaration
trait declaration
implementation declaration
resource declaration
capability declaration
domain declaration

The migration process is:

existing root declaration syntax
          |
          v
audit
          |
          v
map to dedicated owner
          |
          v
conformance tests
          |
          v
remove duplicate root implementation

Do not rename Zamani.g4.


---

57. grammar/core/source-unit.g4

source-unit.g4 owns:

source-unit boundaries;

source-item ordering;

source-level metadata;

EOF;

top-level composition.


It must not become another concrete declaration grammar.

Final relationship:

source-unit
      |
      v
declaration
      |
      v
declaration-family delegate

There must be only one concrete declaration owner for each declaration family.


---

58. grammar/antlr/ integration

Do not rename existing files merely to make the architecture look cleaner.

First establish whether files under:

grammar/antlr/

are:

canonical;

compatibility artifacts;

generated artifacts;

obsolete;

still consumed.


The existing repository documentation already identifies the possibility of competing grammar surfaces under grammar/antlr/.

The final production architecture must have one canonical parser/lexer composition path.

If an old file is retained, its status must be explicitly documented.

It must not silently compete with:

grammar/Zamani.g4


---

59. Rust integration

The declaration grammar itself is language/runtime neutral.

Rust 1.97 / 1.97.1 requirements apply to the implementation surrounding the grammar.

The Rust implementation must:

use safe Rust;

contain no unsafe;

preserve source spans;

preserve deterministic parsing;

distinguish syntax errors from semantic errors;

avoid target-specific assumptions;

avoid machine-size constants;

avoid parser-time hardware discovery.


Generated parser artifacts are implementation details.

They are not the language authority.


---

60. Security contract

Parsing declarations must never:

execute user code;

access filesystem state;

access network state;

inspect hardware;

invoke shell commands;

load arbitrary plugins;

access credentials;

access secrets;

mutate external state;

select a backend;

allocate physical resources.


Even declarations describing:

quantum systems;

networking;

security;

distributed systems;

hardware;

deployment


remain inert parser input.


---

61. Error recovery

Parser error recovery must be deterministic.

Syntax diagnostics may include:

expected declaration
expected identifier
expected type
expected '='
expected ':'
expected ';'
expected declaration body
unexpected declaration token
malformed generic parameter list
malformed declaration member

Semantic diagnostics do not belong to the grammar.

Examples:

duplicate declaration
unknown type
unsatisfied capability
insufficient resource
invalid trait implementation
invalid hardware requirement
invalid quantum type

These belong downstream.


---

62. Source spans

Every declaration parse branch must preserve sufficient source information for:

diagnostics;

IDE tooling;

formatting;

source maps;

provenance;

refactoring;

semantic diagnostics.


The frontend must be able to identify:

declaration span
name span
attribute spans
generic parameter spans
type spans
member spans
initializer spans
variant spans

where supported by the AST architecture.


---

63. Source ordering

The grammar must preserve:

declaration ordering;

generic parameter ordering;

field ordering;

variant ordering;

member ordering;

implementation member ordering.


It must not normalize or sort source.


---

64. Determinism

For:

same source
same lexer version
same grammar version
same configuration

the parser must produce the same structural result.

Declaration parsing must not depend on:

current time;

randomness;

filesystem state;

network state;

hardware discovery;

environment-dependent target selection.



---

65. Testing directory

Declaration tests belong under:

grammar/tests/declarations/

Recommended structure:

grammar/tests/declarations/
├── positive/
├── negative/
├── boundary/
├── scalability/
├── cross-domain/
├── determinism/
├── compatibility/
└── roundtrip/


---

66. Positive tests

Every declaration family needs positive tests.

Required families:

constants
variables
types
aliases
structs
records
enums
unions
classes
interfaces
traits
implementations
resources
capabilities
domains

Tests must cover:

minimal declaration;

attributes;

visibility;

modifiers;

generics;

nested types;

type expressions;

initializers;

members;

variants;

inheritance/contracts where applicable.



---

67. Negative tests

Each declaration family needs malformed-input tests.

Examples:

missing declaration name
missing type
missing '='
missing ':'
missing ';'
unclosed body
unclosed generic list
malformed generic parameter
malformed field
malformed variant
malformed implementation
malformed resource
malformed capability
malformed domain

Parser rejection must be distinguished from semantic rejection.


---

68. Boundary tests

Boundary tests must cover:

one declaration
many declarations

one field
many fields

one variant
many variants

one generic parameter
many generic parameters

one implementation
many implementations

nested types
deep generic expressions
large source units

No test fixture should become an accidental language limit.


---

69. Scalability tests

The declaration layer must be tested independently of:

qubit count
CPU count
core count
thread count
GPU count
FPGA count
QPU count
node count
device count
memory capacity
tensor rank
vector width
network size
accelerator count

The same declaration grammar must remain valid as target resources scale.


---

70. Cross-domain tests

Required combinations include:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL
classical + quantum + HDL + hardware
AI + data + distributed
networking + security
embedded + hardware

The goal is to prove that declarations remain domain-neutral.


---

71. Round-trip tests

Where a canonical formatter/printer exists:

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
formatter
  |
  v
source'
  |
  v
parser

must preserve declaration semantics.

Formatting differences are acceptable.

Semantic changes are not.


---

72. Compatibility

Declaration compatibility has several layers.

Source compatibility

Old source continues to parse.

Semantic compatibility

Old source retains the same meaning.

AST compatibility

The AST contract remains compatible where promised.

IR compatibility

Downstream semantic/IR mappings remain compatible where promised.

Runtime compatibility

Existing programs remain executable where required target capabilities exist.

Grammar compatibility alone does not guarantee runtime compatibility.


---

73. Versioning

Every declaration family must have:

introduction version;

current status;

stability status;

deprecation status;

removal version if applicable;

migration guidance where applicable.


Lifecycle:

experimental
    |
    v
accepted
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

No declaration syntax should disappear silently.


---

74. Feature manifests

For declarations that become substantial language features, create a corresponding feature contract under:

grammar/specification/features/

A feature manifest should define:

id
name
status
version
syntax
grammar
lexer tokens
AST nodes
semantic rules
IR mapping
compiler consumers
runtime consumers
capabilities
resource requirements
positive tests
negative tests
boundary tests
scalability tests
compatibility
hard-coding policy
diagnostics

This is the mechanism that makes a file independently completable.


---

75. Independent-file completion rule

A declaration file is not complete merely because ANTLR accepts it.

A file is complete only when all of these are known:

Purpose
Status
Ownership
Non-ownership
Inputs
Outputs
Dependencies
Upstream contracts
Downstream consumers
Public grammar contract
AST contract
Semantic contract
IR integration
Compiler integration
Runtime integration
Tooling integration
Cross-domain integration
Positive tests
Negative tests
Boundary tests
Scalability tests
Determinism tests
Compatibility
Diagnostics
Security
Performance
Hard-coding audit
Completion criteria

No future file should need to invent these contracts.


---

76. File-specific completion contracts

declarations.g4

Complete when:

exactly one declaration dispatcher exists;

all concrete declaration families have one owner;

imports use canonical grammar names;

tokenVocab = ZamaniLexer;

no concrete syntax is duplicated;

no recursive self-dispatch exists;

no circular imports exist;

resource/capability/domain delegates are correctly connected;

AST branches are known;

declaration tests pass.


constants.g4

Complete when:

constant syntax is independent;

canonical expression/type rules are reused;

AST mapping exists;

semantic boundary exists;

tests pass.


variables.g4

Complete when:

variable syntax is independent;

mutability is syntactic;

type/initializer reuse canonical rules;

ownership/memory remain downstream;

tests pass.


types.g4

Complete when:

named type syntax is complete;

canonical typeExpression is reused;

duplicate type-expression grammar is removed or formally delegated;

AST/semantic/compatibility contracts exist.


aliases.g4

Complete when:

canonical alias syntax is selected;

generic aliases are supported where specified;

canonical typeExpression is reused;

alias semantics remain downstream.


structs.g4

Complete when:

fields are arbitrary in number;

canonical types/expressions are reused;

source order is preserved;

AST mapping exists;

no layout decisions occur in grammar.


records.g4

Complete when:

record-specific syntax is clearly defined;

relationship to structs is explicit;

AST mapping is defined;

semantic/lowering contract exists.


enums.g4

Complete when:

variants are unbounded by grammar;

discriminants use canonical expressions;

semantic exhaustiveness is downstream.


unions.g4

Complete when:

union variants are unbounded;

payload forms are complete;

canonical lexer vocabulary is used;

semantic recursion/exhaustiveness remains downstream.


classes.g4

Complete when:

class members are fully specified;

canonical function/type/block grammar is reused;

no object-layout semantics are embedded;

AST contract exists.


interfaces.g4

Complete when:

interface contracts are syntactically complete;

inheritance/extension is defined;

semantic dispatch remains downstream.


traits.g4

Complete when:

trait syntax is complete;

generic/bound syntax is canonical;

trait solving remains downstream.


implementations.g4

Complete when:

implementation target syntax is complete;

generic implementations are supported;

coherence/resolution remains downstream.


resources.g4

Complete when:

resource intent is target-independent;

requirements/constraints/preferences/hints remain distinct;

canonical resource expression grammar is used;

no physical device selection is encoded.


The existing resource grammar explicitly intends this separation.

capabilities.g4

Complete when:

it delegates to grammar/core/capabilities.g4;

no capability syntax is duplicated;

declaration dispatch is integrated;

capability semantics remain downstream.


The existing file explicitly defines this adapter role.

domains.g4

Complete when:

domain syntax is independently defined;

domain names remain extensible;

domains do not create independent languages;

capability/resource integration is canonical;

domain semantics remain downstream.


attributes.g4

Complete when:

attribute syntax has one owner;

declaration usage is consistent;

attribute interpretation is registry/semantic-layer responsibility.



---

77. Hard-coding audit

Every declaration grammar must be searched for:

MAX_
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
MAX_FIELDS
MAX_VARIANTS
MAX_PARAMETERS
MAX_TYPES
MAX_MEMBERS
MAX_DIMENSIONS
MAX_RANK
MAX_WIDTH

Also search for structural hard-coding such as:

field1 field2 field3
variant1 variant2 variant3
parameter1 parameter2 parameter3
device1 device2 device3

where arbitrary repetition is semantically valid.


---

78. Hardware hard-coding audit

Reject grammar-level assumptions involving:

cpu0
gpu0
fpga0
qpu0
device0
node0
memory_bank0
physical_qubit0
physical_qubit1
register32
vector128
tensor_rank8

unless the syntax is explicitly a user-level identifier or literal and has no built-in machine interpretation.

The lexer must not decide whether such a resource exists.


---

79. Quantum hard-coding audit

Declaration grammars must not:

enumerate physical qubits;

enumerate QPU devices;

encode topology;

encode coupling maps;

encode calibration;

encode gate durations;

encode QEC implementation;

encode ZQN implementation;

construct quantum::ir.


Quantum semantics remain downstream.


---

80. No duplicated semantic IR

The declaration layer must never create:

DeclarationIR
QuantumDeclarationIR
HardwareDeclarationIR
ResourceIR
CapabilityIR
ClassIR
StructIR

merely because the grammar has corresponding syntax.

The parser produces parse-tree structure.

The frontend produces the canonical domain-neutral AST.

Semantic analysis produces the appropriate semantic representation.


---

81. Compiler integration

The compiler consumes declaration AST/semantic data.

It may perform:

name resolution;

type checking;

generic resolution;

trait/interface solving;

constant evaluation;

layout;

specialization;

optimization;

lowering;

target selection;

resource planning.


None of these operations belong in declaration parsing.


---

82. Runtime integration

The runtime consumes compiled/lowered representations.

Declaration syntax must not:

execute;

allocate physical resources;

discover devices;

select hardware;

schedule operations.


Runtime behavior must remain downstream.


---

83. Tooling integration

Every declaration family must eventually support:

syntax highlighting;

diagnostics;

formatter;

source navigation;

symbol indexing;

rename/refactoring;

documentation extraction;

IDE/LSP integration;

source maps.


Source spans must therefore be preserved.


---

84. Performance

The declaration grammar must be scalable without semantic limits.

Performance optimizations must not alter source semantics.

If implementation resource limits are required, they belong to explicit parser/compiler resource policy.

Examples:

maximum input bytes
parser memory budget
parser time budget
compiler memory budget
compiler time budget

These must never become grammar semantics.


---

85. No embedded Rust

All declaration grammars must remain declarative ANTLR grammars.

Forbidden:

@members { ... }

for semantic execution.

Forbidden:

{ Rust code }

actions.

Forbidden parser-time:

filesystem operations;

network calls;

process execution;

hardware queries;

runtime calls.


This guarantees safe Rust integration and deterministic parsing.


---

86. Repository-wide integration matrix

The declaration layer integrates with:

grammar/lexer/
        |
        v
grammar/core/
        |
        +--> names
        +--> paths
        +--> attributes
        +--> capabilities
        |
        v
grammar/types/
        |
        v
grammar/expressions/
        |
        v
grammar/declarations/
        |
        v
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
        |
        v
frontend AST
        |
        v
semantic model
        |
        +--> classical
        +--> quantum::ir
        +--> HDL/hardware
        +--> distributed
        +--> AI/data
        |
        v
compiler/lowering
        |
        v
routing/scheduling/resilience/QEC/ZQN
        |
        v
HAL
        |
        v
runtime

The declaration layer must remain reusable by all of these domains.


---

87. Migration order

Do not attempt to fix everything simultaneously.

Use this order.

Phase 1 — lexical authority

Complete:

grammar/lexer/

first.

Specifically reconcile:

keywords.g4
operators.g4
punctuation.g4
identifiers.g4
literals.g4
comments.g4
tokens.g4
ZamaniLexer.g4

with the Rust lexer contract.

The repository's lexer architecture already establishes tokens.g4 as the lexical composition boundary.

Phase 2 — core syntax

Complete:

grammar/core/

especially:

names
paths
attributes
visibility
modifiers
capabilities

Phase 3 — types

Complete:

grammar/types/

and establish one canonical typeExpression.

Phase 4 — expressions

Complete canonical expression syntax.

Phase 5 — concrete declarations

Complete independently:

constants
variables
types
aliases
structs
records
enums
unions
classes
interfaces
traits
implementations
resources
capabilities
domains
attributes

Phase 6 — dispatcher

Only after the concrete owners are independently complete:

declarations.g4

should be finalized.

Phase 7 — root integration

Integrate:

grammar/Zamani.g4
grammar/core/source-unit.g4

and eliminate duplicate declaration ownership.

Phase 8 — frontend integration

Verify:

parser -> src/frontend/ast/

for every declaration family.

Phase 9 — semantic integration

Verify:

AST -> semantic model

Phase 10 — IR integration

Verify:

semantic model -> canonical IR

with quantum lowering eventually reaching:

quantum::ir

Phase 11 — conformance

Run:

positive
negative
boundary
scalability
cross-domain
determinism
round-trip
compatibility

tests.


---

88. What must NOT be added to declarations.g4

Do not add:

quantum gate lists
QEC algorithms
ZQN algorithms
routing rules
scheduling rules
calibration
hardware discovery
device discovery
backend selection
compiler optimization
runtime execution
ABI rules
memory layout
physical qubit mapping
physical CPU mapping
GPU selection
FPGA selection
network topology
AI framework syntax
vendor APIs

Those belong elsewhere.


---

89. What must be expanded elsewhere

If declaration syntax needs functionality that does not currently exist, add it to its proper owner.

Examples:

new token
    -> grammar/lexer/

new reusable name syntax
    -> grammar/core/

new type form
    -> grammar/types/

new expression
    -> grammar/expressions/

new resource semantic
    -> grammar/resources/ or hardware/

new quantum syntax
    -> grammar/quantum/

new HDL syntax
    -> grammar/hdl/

new hardware intent
    -> grammar/hardware/

new distributed abstraction
    -> grammar/distributed/

new AI syntax
    -> grammar/ai/

new semantic capability
    -> grammar/core/capabilities.g4

new declaration family
    -> dedicated declaration .g4
    -> then one dispatcher integration

Do not put unrelated functionality into declarations.g4.


---

90. Definition of production-ready

grammar/declarations/ is production-ready only when:

Authority

[ ] One declaration dispatcher.

[ ] One owner per declaration family.

[ ] No duplicate root declaration grammar.

[ ] No duplicate source-unit declaration grammar.

[ ] Canonical lexer vocabulary.

[ ] No legacy ZamaniTokens dependency in production delegates.

[ ] No unexplained K_* vocabulary.


Declaration coverage

[ ] Constants.

[ ] Variables.

[ ] Named types.

[ ] Aliases.

[ ] Structs.

[ ] Records.

[ ] Enums.

[ ] Unions.

[ ] Classes.

[ ] Interfaces.

[ ] Traits.

[ ] Implementations.

[ ] Resources.

[ ] Capabilities.

[ ] Domains.

[ ] Attributes.


Architecture

[ ] No circular grammar dependencies.

[ ] Concrete syntax remains delegated.

[ ] Canonical types are reused.

[ ] Canonical expressions are reused.

[ ] Source order is preserved.

[ ] Source spans are preserved.

[ ] AST contracts exist.

[ ] Semantic contracts exist.

[ ] IR contracts exist.


POCO-REAF

[ ] No fixed CPU count.

[ ] No fixed GPU count.

[ ] No fixed FPGA count.

[ ] No fixed QPU count.

[ ] No fixed qubit count.

[ ] No fixed node count.

[ ] No fixed memory capacity.

[ ] No fixed tensor rank.

[ ] No fixed vector width.

[ ] No fixed field count.

[ ] No fixed variant count.

[ ] No fixed generic-parameter count.


Quantum

[ ] Quantum types flow through canonical type semantics.

[ ] No physical qubit mapping.

[ ] No QPU selection.

[ ] No topology.

[ ] No gate enumeration in declarations.

[ ] No QEC implementation.

[ ] No ZQN implementation.

[ ] quantum::ir remains downstream.


Hardware/HDL

[ ] No physical device IDs.

[ ] No physical addresses.

[ ] No fixed topology.

[ ] No backend selection.

[ ] Resource intent remains distinct from realization.


Safety

[ ] No Rust actions.

[ ] No unsafe.

[ ] No filesystem access.

[ ] No network access.

[ ] No process execution.

[ ] No hardware discovery.

[ ] No runtime execution.


Tests

[ ] Positive.

[ ] Negative.

[ ] Boundary.

[ ] Scalability.

[ ] Cross-domain.

[ ] Determinism.

[ ] Round-trip.

[ ] Compatibility.



---

91. Final declaration architecture

The finished declaration subsystem is:

Zamani source
                              |
                              v
                       canonical lexer
                              |
                              v
                         core syntax
                              |
               +--------------+--------------+
               |                             |
               v                             v
        canonical types              canonical expressions
               |                             |
               +--------------+--------------+
                              |
                              v
                     declarations.g4
                              |
       +----------+-----------+-----------+----------+
       |          |           |           |          |
       v          v           v           v          v
     values      types     aggregates   contracts   intent
       |          |           |           |          |
       |          |           |           |          |
 constants     types       structs      classes    resources
 variables     aliases     records      interfaces capabilities
                            enums        traits     domains
                            unions       impls
                              |
                              v
                     domain-neutral AST
                              |
                              v
                    structural validation
                              |
                              v
                     semantic analysis
                              |
          +-------------------+-------------------+
          |                   |                   |
          v                   v                   v
      classical          quantum semantics    HDL/hardware
      semantics                |              semantics
                              v
                         quantum::ir
                              |
                +-------------+-------------+
                |             |             |
                v             v             v
             optimize        QEC           ZQN
                |             |             |
                +-------------+-------------+
                              |
                              v
                     routing/scheduling
                              |
                              v
                             HAL
                              |
                              v
                           runtime


---

92. Final invariant

The entire declaration directory must preserve this rule:

> A Zamani declaration describes a source-level semantic entity, not an accidental limitation of the machine currently available.



Therefore:

one declaration
      |
      v
one source-level meaning
      |
      +----> tiny machine
      +----> embedded system
      +----> CPU
      +----> multicore CPU
      +----> GPU
      +----> FPGA
      +----> ASIC
      +----> QPU
      +----> simulator
      +----> accelerator
      +----> cluster
      +----> supercomputer
      +----> cloud
      +----> future substrate

subject only to genuine semantic requirements and the target's ability to satisfy them.


---

93. Immediate repository corrections required

Before declaring this README or the declaration subsystem complete, the following concrete repository issues must be resolved.

1. Fix declarations.g4

Remove the self-recursive:

domainDeclaration
    : domainDeclaration
    ;

and delegate to ZamaniDomains.

2. Normalize lexer vocabulary

All production declaration delegates must converge on:

tokenVocab = ZamaniLexer;

The current repository still contains grammars using ZamaniTokens/legacy vocabulary.

3. Normalize token names

Resolve conflicting names such as:

LPAREN vs LEFT_PAREN
RPAREN vs RIGHT_PAREN
EQUALS vs ASSIGN
SEMICOLON vs SEMI
IDENTIFIER vs IDENT

There must be exactly one canonical parser-visible token identity for each lexical concept.

4. Remove duplicated type-expression ownership

grammar/declarations/types.g4 currently documents grammar/types/ as the canonical type-expression owner but also contains forwarding implementations of type-expression rules.

Those must converge on the actual canonical type grammar.

5. Update this README's directory inventory

The old README omitted existing:

attributes.g4
capabilities.g4
classes.g4
domains.g4
records.g4
resources.g4

The actual repository contains them.

6. Integrate records.g4 and classes.g4

These are existing concrete declaration owners and must not remain outside the dispatcher.

7. Integrate resources and capabilities correctly

resources.g4 owns resource syntax; capabilities.g4 is an adapter to the canonical core capability grammar.

8. Keep domain syntax delegated

domains.g4 is already an independent ZamaniDomains grammar.

9. Audit root competitors

Audit:

grammar/Zamani.g4
grammar/core/source-unit.g4
grammar/antlr/ZamaniParser.g4

for duplicate declaration rules before removing them.

10. Do not rename the existing declaration files

The existing names are suitable. The required work is authority/conformance/integration, not mass renaming.


---

94. Completion statement

When this README's contracts are satisfied, the declaration directory can be considered independently complete only if:

lexer contract
      +
core contract
      +
type contract
      +
expression contract
      +
concrete declaration contracts
      +
dispatcher contract
      +
AST contract
      +
semantic contract
      +
IR contract
      +
compiler contract
      +
tooling contract
      +
tests
      +
compatibility
      +
hard-coding audit

all exist and agree.

The declaration subsystem then becomes a stable front-end boundary rather than another competing language implementation.

The governing principle is:

Declare meaning once.
Preserve meaning.
Do not encode today's machine.
Do not constrain tomorrow's machine.
Lower according to capabilities.
Realize according to available resources.

That is the declaration-layer foundation required for:

Zamani
From Atom to Everywhere

POCO-REAF
Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

 