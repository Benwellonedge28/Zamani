Zamani Keyword Specification

Path: "grammar/lexer/keywords.md"
Status: Normative
Scope: Canonical lexical keyword contract
Language: Zamani
Grammar baseline: ANTLR4
Implementation baseline: Rust 1.97 / Rust 1.97.1
Safety requirement: Safe Rust only; no "unsafe" Rust implementation
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the authoritative contract for Zamani lexical keywords.

It establishes:

- which source spellings are reserved;
- which token names represent those spellings;
- which vocabulary belongs to the lexical layer;
- which vocabulary must remain ordinary identifiers;
- how contextual vocabulary is handled;
- how keyword changes affect compatibility;
- how keywords integrate with "keywords.g4";
- how keywords integrate with "identifiers.g4";
- how keywords integrate with "tokens.g4";
- how keywords integrate with "grammar/antlr/ZamaniLexer.g4";
- how keywords integrate with "Zamani.g4";
- how keywords integrate with "src/lexer.rs";
- how keywords integrate with parser, AST, semantic analysis, IR, compiler, runtime, tooling, and interoperability;
- how the keyword system remains scalable without encoding hardware or implementation limits.

This file is normative for the policy and inventory of keywords.

The executable lexical rules remain in:

grammar/lexer/keywords.g4

The complete assembled lexer remains the responsibility of the canonical lexer assembly.

---

2. Architectural position

The lexical pipeline is:

source text
    |
    v
Unicode/source decoding
    |
    v
keyword vocabulary
    |
    +--> keywords.g4
    |
    +--> identifiers.g4
    |
    +--> literals
    |
    +--> operators
    |
    +--> punctuation
    |
    v
canonical Zamani lexer
    |
    v
parser
    |
    v
domain-neutral AST
    |
    v
structural + semantic analysis
    |
    v
canonical semantic model
    |
    +--------------------+--------------------+
    |                    |                    |
    v                    v                    v
classical IR        quantum::ir        HDL/Hardware IR
    |                    |                    |
    +--------------------+--------------------+
                         |
                         v
               optimization / lowering
                         |
                routing / scheduling
                         |
              resilience / QEC / ZQN
                         |
                        HAL
                         |
                  target realization

Keywords terminate their responsibility at lexical classification.

A keyword MUST NOT:

- perform semantic analysis;
- resolve a symbol;
- select a compiler backend;
- select hardware;
- discover hardware;
- select a QPU;
- select a CPU;
- select a GPU;
- select an FPGA;
- select a device;
- assign physical qubits;
- perform routing;
- perform scheduling;
- perform QEC;
- perform ZQN analysis;
- perform calibration;
- allocate runtime resources;
- impose resource limits.

The existing "Zamani.g4" explicitly establishes this source-syntax boundary and keeps "quantum::ir" as the canonical quantum semantic boundary.

---

3. Authority hierarchy

Keyword authority is ordered as follows:

grammar/specification/
        |
        v
grammar/spec/
        |
        v
grammar/lexer/keywords.md
        |
        v
grammar/lexer/keywords.g4
        |
        v
grammar/antlr/ZamaniLexer.g4
        |
        v
src/lexer.rs
        |
        v
parser / AST / semantic implementation

More precisely:

3.1 Language specification

The language specification determines whether a spelling is supposed to be reserved.

3.2 "keywords.md"

This file records the normative keyword inventory and integration contract.

3.3 "keywords.g4"

This file implements the lexical keyword inventory.

3.4 "ZamaniLexer.g4"

This assembles the lexical components into the canonical lexer.

3.5 Rust lexer

"src/lexer.rs" must implement the same lexical contract when the Rust-native lexer is used.

The current Rust lexer already contains an explicit keyword token family, including core, quantum, Sankofa/MTS, OOP, system, and type-related vocabulary.

3.6 Parser

Parser grammars consume canonical lexer tokens.

Parser grammars MUST NOT independently redefine the keyword vocabulary.

---

4. What is a keyword?

A Zamani keyword is a source spelling reserved by the language because its lexical classification is required to distinguish language syntax from ordinary identifiers.

A keyword is not merely:

- a popular API name;
- a library function;
- a mathematical function;
- a quantum gate;
- a hardware name;
- a vendor name;
- a backend name;
- a framework name;
- a device name;
- a capability name.

For example:

fn
let
module
quantum
measure

may be keywords because they participate directly in language syntax.

But:

H
X
CNOT
RX
CUDA
ROCm
TensorFlow
Qiskit
NVIDIA
AMD
Intel
IBM
vendor_gate
my_qpu

MUST remain identifiers unless a future normative specification explicitly reserves one of those spellings.

---

5. Keyword classes

Zamani keywords are divided into these categories:

1. Core declarations and bindings
2. Control flow
3. Pattern matching
4. Modules and packages
5. Types
6. Visibility
7. Object-oriented constructs
8. Generics and constraints
9. Concurrency
10. Effects
11. Contracts
12. Quantum syntax
13. Quantum semantic-intent vocabulary
14. Classical/data computation
15. Hardware/software co-design
16. Resource and capability intent
17. Distributed computation
18. AI/ML
19. Data
20. Networking
21. Security
22. Compilation and execution
23. Metaprogramming
24. Dialects and language versioning
25. Sankofa/temporal vocabulary
26. Nano/domain vocabulary
27. Built-in type names
28. Boolean/null literals
29. Diagnostics/runtime constructs
30. Interoperability vocabulary

Not every domain concept becomes a keyword.

The default design is:

«Prefer identifiers and semantic data over expanding the reserved-word set.»

---

6. Core keywords

The following are stable core lexical vocabulary:

fn
let
var
mut
const
return

Canonical token names:

FN
LET
VAR
MUT
CONST
RETURN

These belong to the core language and are consumed by:

grammar/functions/
grammar/statements/
grammar/declarations/
grammar/types/

---

7. Control-flow keywords

if
else
for
in
while
loop
break
continue
match
case
when
yield

Canonical token names:

IF
ELSE
FOR
IN
WHILE
LOOP
BREAK
CONTINUE
MATCH
CASE
WHEN
YIELD

The lexer only identifies them.

Control-flow semantics belong to the statement and semantic layers.

---

8. Module and package keywords

module
import
export
use
from
as
package
depends

Canonical tokens:

MODULE
IMPORT
EXPORT
USE
FROM
AS
PACKAGE
DEPENDS

"depends" is reserved only if the package/dependency syntax requires lexical distinction.

Otherwise dependency names, package names, repository names, registry names, and versions remain ordinary data.

---

9. Declaration/type keywords

type
struct
enum
trait
impl
class
interface
record

Canonical tokens:

TYPE
STRUCT
ENUM
TRAIT
IMPL
CLASS
INTERFACE
RECORD

These are syntactic declaration markers.

They do not determine implementation layout.

For example:

struct

does not imply:

- fixed field count;
- fixed alignment;
- fixed ABI;
- fixed memory size.

Those are downstream semantic/backend concerns.

---

10. Visibility keywords

pub
public
private
protected
internal

Canonical tokens:

PUB
PUBLIC
PRIVATE
PROTECTED
INTERNAL

If future language evolution establishes one canonical spelling, existing spellings MUST be handled through the compatibility/deprecation system rather than silently changing their meaning.

---

11. Object-oriented keywords

static
override
virtual
abstract
final
extends
implements
this
self
super
new

Canonical tokens:

STATIC
OVERRIDE
VIRTUAL
ABSTRACT
FINAL
EXTENDS
IMPLEMENTS
THIS
SELF
SUPER
NEW

"self" and "this" remain distinct lexical spellings.

Their semantic relationship is determined by the relevant language construct.

---

12. Generic and constraint keywords

where

Canonical token:

WHERE

Generic parameters themselves remain identifiers.

For example:

T
N
Shape
Backend
Capability

are not keywords.

This is important for unbounded generic programming.

---

13. Concurrency keywords

async
await
spawn
parallel

Canonical tokens:

ASYNC
AWAIT
SPAWN
PARALLEL

These keywords express language-level concurrency intent.

They MUST NOT encode:

thread count
core count
CPU count
GPU count
node count
worker count

For example:

parallel

means parallel execution semantics.

It does not mean:

parallel_on_8_threads

or:

parallel_on_64_cores

unless those values are explicitly expressed as program requirements.

---

14. Error-handling keywords

try
catch
finally
throw
handle

Canonical tokens:

TRY
CATCH
FINALLY
THROW
HANDLE

Error-handling implementation is downstream.

The lexical layer does not define:

- exception representation;
- stack layout;
- runtime ABI;
- unwinding implementation;
- recovery mechanism.

---

15. Effects and contracts

Keywords:

effect
effects
with
requires
ensures
invariant

Canonical tokens:

EFFECT
EFFECTS
WITH
REQUIRES
ENSURES
INVARIANT

These provide syntactic entry points for:

grammar/effects/
grammar/spec/
semantic analysis

"requires" MUST NOT be interpreted as a physical resource allocation instruction.

For example:

requires capability("quantum.measurement")

expresses a requirement.

It does not select a particular device.

---

16. Quantum keywords

Stable quantum syntax vocabulary:

quantum
circuit
Qubit
apply
measure
reset
barrier
control
adjoint
inverse
observe

Canonical tokens:

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

These keywords express quantum-language constructs.

They do NOT constitute a complete quantum gate vocabulary.

---

17. Quantum operations are not automatically keywords

The following MUST remain identifiers unless explicitly promoted by a future language specification:

H
X
Y
Z
S
T
CNOT
CZ
SWAP
RX
RY
RZ
U
U1
U2
U3
Toffoli
custom_gate
vendor_gate

This is intentional.

The language must support:

apply H to q
apply custom_gate to q
apply vendor.operation to q

without requiring a new compiler release every time a new operation appears.

This is essential for POCO-REAF and future hardware independence.

---

18. Quantum semantic vocabulary

The following existing language vocabulary may be reserved where it has genuine syntax-level meaning:

entangle
noise
fidelity
surface
code
logical
parity

Canonical tokens:

ENTANGLE
NOISE
FIDELITY
SURFACE
CODE
LOGICAL
PARITY

However:

surface_code
surface
code

must not cause QEC execution during lexing.

QEC remains downstream.

The intended pipeline remains:

quantum source
    |
    v
domain-neutral AST
    |
    v
quantum semantic analysis
    |
    v
quantum::ir
    |
    v
QEC / optimization / routing / scheduling / ZQN / HAL

---

19. QEC, ZQN and HAL separation

Keywords MUST NOT turn the lexer into a QEC, ZQN, routing, scheduling, or HAL implementation.

For example:

logical
parity
noise
fidelity

identify source constructs only.

They do not:

- construct QEC codes;
- calculate syndrome data;
- select stabilizers;
- perform decoding;
- select physical qubits;
- choose coupling topology;
- schedule gates;
- inspect hardware;
- calibrate devices.

The corresponding repository subsystems remain authoritative for those responsibilities.

---

20. Built-in quantum type vocabulary

The current language surface reserves:

Qubit

and supports quantum type forms such as:

Qubit
Qubit<...>
QRegister<...>
QState<...>
QuantumRegister<...>
LogicalQubit
LogicalRegister<...>

The lexical layer should reserve only spellings that genuinely need lexical distinction.

Generic register names remain identifiers.

No keyword may establish:

MAX_QUBITS
MAX_QREGISTER_SIZE
MAX_QSTATE_SIZE

or any equivalent universal limit.

---

21. Nano and agent vocabulary

Existing domain vocabulary:

nano
agent
perform
learn
infer

Canonical tokens:

NANO
AGENT
PERFORM
LEARN
INFER

These are language-level constructs only.

They do not hard-code:

- number of agents;
- number of atoms;
- number of molecules;
- material size;
- agent population;
- computational resources.

---

22. Sankofa and temporal vocabulary

Existing Zamani/Sankofa-oriented vocabulary:

mts
zamani
sasa
remember
recall
wisdom

Canonical tokens:

MTS
ZAMANI
SASA
REMEMBER
RECALL
WISDOM

These keywords identify language constructs.

They do not impose:

- a maximum history length;
- a maximum number of timelines;
- a fixed timestamp width;
- a maximum number of branches;
- a fixed memory size.

Temporal semantics belong to the semantic/execution layers.

---

23. Language and metaprogramming

Reserved vocabulary:

language
macro
extern

Canonical tokens:

LANGUAGE
MACRO
EXTERN

Additional metaprogramming concepts should remain identifiers unless their syntax requires reserved lexical distinction.

---

24. Type-system vocabulary

The following existing spellings are reserved where required by the current type syntax:

Result
Never

Canonical tokens:

RESULT
NEVER

The type system MUST NOT use keywords to encode physical representation.

For example:

int
float

do not imply a particular machine register width.

---

25. Mathematical symbols and mathematical names

Mathematical concepts should generally remain semantic identifiers or operators rather than becoming an ever-growing keyword list.

For example:

sin
cos
tan
exp
log
sqrt
fft
svd
eig
gradient
integral
derivative

should normally remain identifiers.

Likewise:

matrix
tensor
vector
polynomial
distribution

should be language keywords only when they have language-level syntax.

Otherwise they remain type/library/semantic names.

This prevents the lexer from becoming a mathematical dictionary.

---

26. "Pi" and "Sigma"

The current Rust lexer has explicit "KeywordPi", "SigmaSymbol", and "PiSymbol" vocabulary.

These must be separated conceptually:

Π
Σ

are mathematical/symbolic lexical forms when used as symbols.

Pi
Sigma

are ordinary identifier spellings unless the language specification explicitly gives them reserved-word status.

A mathematical name must not become a keyword merely because it is common.

---

27. Type names

The current language surface contains:

void
int
float
bool
str
string
char

Canonical tokens:

VOID
INT
FLOAT_TYPE
BOOL_TYPE
STR_TYPE
STRING_TYPE
CHAR_TYPE

These spellings are lexical conveniences.

They do not establish:

- bit width;
- register width;
- ABI;
- machine representation;
- target-specific precision.

Precision and representation are semantic/type-system concerns.

---

28. Boolean and null literals

Reserved literal spellings:

true
false
nil
null

Canonical tokens:

TRUE
FALSE
NIL
NULL

These are not ordinary identifiers.

Their semantic types are determined downstream.

---

29. Core built-in operation spellings

Existing language compatibility requires:

print
println
assert
panic
len
sizeof

These may remain reserved where the current parser requires lexical distinction.

Canonical tokens:

PRINT
PRINTLN
ASSERT
PANIC
LEN
SIZEOF

However, this category must remain small.

General libraries MUST NOT add keywords merely because they expose functions.

For example:

fft()
matmul()
sort()
hash()
serialize()
deserialize()

should remain ordinary callable names unless a future specification proves that lexical reservation is necessary.

---

30. System and domain vocabulary

Existing vocabulary includes:

omniversal
simulate
synthesize
deploy
alignment
containment
trust
knowledge
generative
sovereignty
goal
bionano
reality
nlp
system

These remain reserved only where they participate in actual Zamani syntax.

They MUST NOT be interpreted by the lexer as:

- deployment commands;
- cloud actions;
- hardware operations;
- simulation commands;
- AI framework selection;
- backend selection.

Lexical recognition has no side effects.

---

31. Advanced vocabulary

Existing compatibility vocabulary includes:

asi
aesi
asesi
admin
payment
gateway
graphics
video
adjust
versioning
copyright
notice
legal
action
tailor
business

These require special treatment.

They MUST NOT automatically be considered core language semantics merely because they exist in historical/extended grammar material.

The lifecycle is:

historical/proposed vocabulary
        |
        v
feature specification
        |
        v
semantic contract
        |
        v
AST contract
        |
        v
canonical grammar
        |
        v
conformance tests
        |
        v
stable keyword

Until that process is completed, the vocabulary should be treated as compatibility/extended language surface rather than silently promoted to new core semantics.

This prevents "Zamani-Grammar.md" from becoming an uncontrolled second language authority.

---

32. "unsafe"

The spelling:

unsafe

currently exists in the repository's lexer and statement grammar.

This must be explicitly distinguished from the implementation requirement:

«The Zamani compiler implementation uses safe Rust only and contains no Rust "unsafe".»

The two concepts are not equivalent.

If Zamani retains "unsafe" as a source-language construct for compatibility or foreign-language interoperability, it must have an explicit semantic specification.

If Zamani's normative language model is intentionally safe-only, then "unsafe" should remain a reserved compatibility spelling but be rejected by core semantic validation.

It MUST NOT silently become valid merely because the lexer recognizes it.

This allows the repository to remove unsafe source semantics later without breaking lexical compatibility.

---

33. Rust interoperability

Rust-specific vocabulary MUST NOT leak into core Zamani.

For example:

unsafe
async
trait
impl
extern

may occur because Zamani interoperates with Rust.

That does not mean Zamani's core semantics are Rust.

The interoperability layer owns Rust-specific interpretation:

grammar/interoperability/rust.g4

The keyword layer only provides lexical classification.

---

34. Contextual keywords

Not every language word should become a permanently reserved keyword.

A contextual keyword is a spelling that behaves like a keyword only in a specific syntactic position.

Examples may include future constructs such as:

model
move
switch
then
ancestor

which already appear in the Rust lexer vocabulary but are not uniformly represented by the current "keywords.g4".

The production rule is:

«If the parser can distinguish a construct structurally without globally reserving a spelling, prefer contextual treatment.»

This preserves source compatibility.

For example, if:

model

can safely remain a user-defined identifier outside a model declaration, it should not necessarily become a globally reserved keyword.

A contextual keyword must have:

- a defined syntactic context;
- an AST mapping;
- a semantic mapping;
- a compatibility rule;
- positive tests;
- negative tests;
- identifier-collision tests.

---

35. Keyword versus identifier rule

The following rule is mandatory:

If a spelling is not required to distinguish language syntax,
it should remain an identifier.

Therefore these are normally identifiers:

H
X
CNOT
RX
GPU
CPU
FPGA
QPU
CUDA
ROCm
LLVM
MLIR
QIR
OpenQASM
Qiskit
TensorFlow
PyTorch
NVIDIA
AMD
Intel
IBM
Google
Amazon
Microsoft
device
backend
accelerator

This is essential for vendor independence and future extensibility.

---

36. No hardware keywords

Zamani MUST NOT create keywords such as:

cpu0
cpu1
gpu0
gpu1
qpu0
qpu1
fpga0
fpga1
qubit0
qubit1
node0
node1

Those are identifiers and/or target metadata.

Likewise, keywords must never encode:

MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_QUBITS
MAX_NODES
MAX_THREADS
MAX_MEMORY
MAX_DEVICES

---

37. No resource-count keywords

The language must not contain resource-count vocabulary that accidentally establishes universal limits.

Invalid design:

eight_threads
sixteen_cores
sixty_four_qubits
four_gpus

Valid semantic model:

requires capability("parallel.compute")
requires resource("qubit") >= n
requires memory >= required_memory

The numeric values are program/resource semantics, not keyword vocabulary.

---

38. No topology keywords

Keywords MUST NOT encode:

ring
mesh8
grid16
star32
torus64
coupler17
qubit17

Topology is target/resource metadata.

A portable program may express topology requirements or capabilities, but physical realization belongs downstream.

---

39. Keyword ordering and ANTLR integration

The canonical lexer must ensure that keyword tokens and identifier matching do not produce accidental collisions.

The repository's identifier grammar explicitly states that keywords are not duplicated there and that keyword recognition belongs to the canonical lexer assembly.

The integration contract is therefore:

keywords.g4
identifiers.g4
literals.g4
operators.g4
punctuation.g4
comments.g4
unicode.g4
...
        |
        v
grammar/antlr/ZamaniLexer.g4
        |
        v
parser grammars

No parser grammar should recreate:

FN : 'fn';
LET : 'let';

or equivalent keyword definitions.

---

40. Token-name stability

Token names are part of the parser compatibility contract.

For example:

FN
LET
QUANTUM
QUBIT
MEASURE
MODULE
TYPE

must not be renamed casually.

Changing:

FN

to:

FUNCTION

is a compatibility-affecting change even if both recognize:

fn

Token renames require:

1. compatibility analysis;
2. parser impact analysis;
3. Rust lexer impact analysis;
4. generated lexer regeneration;
5. AST/parser conformance;
6. migration documentation;
7. tests.

---

41. Keyword spelling is case-sensitive

Zamani keyword matching is case-sensitive.

Therefore:

fn

is a keyword.

These are not automatically equivalent:

Fn
FN
fN

They remain identifiers unless separately reserved.

This matches the identifier system's case-sensitive contract.

---

42. Unicode keywords

Unicode may be supported for identifiers and symbols, but a keyword must have one canonical source spelling.

Do not create multiple visually equivalent keyword spellings merely through Unicode normalization.

For example, if a future symbolic keyword is introduced, its:

- Unicode code points;
- normalization policy;
- display form;
- source form;
- token name;
- compatibility behavior

must be explicitly specified.

Keyword recognition must not silently normalize or mutate source text.

---

43. Keyword escaping

If Zamani eventually supports escaped identifiers, the specification must define whether a keyword can be escaped into an identifier.

For example:

`module`

or another future escaping form.

This decision belongs in:

grammar/spec/lexical.md
grammar/lexer/identifiers.g4
grammar/lexer/keywords.md

It must not be invented independently by the lexer implementation.

Until such a mechanism is formally specified, keyword spellings remain reserved.

---

44. Keyword collision policy

If a new keyword would conflict with an existing identifier used in valid source code, the language must not silently break that source.

The compatibility process is:

candidate keyword
       |
       v
identifier collision analysis
       |
       +---- high compatibility risk ----> contextual keyword
       |
       +---- low compatibility risk -----> reserved keyword
       |
       v
versioned language specification

This is especially important for the broad historical vocabulary in "Zamani-Grammar.md".

---

45. Keyword promotion lifecycle

A proposed word MUST NOT become a stable keyword simply because it appears in design documentation.

Lifecycle:

PROPOSED
   |
   v
SPECIFIED
   |
   v
EXPERIMENTAL
   |
   v
CONFORMANCE TESTED
   |
   v
STABLE

Possible removal path:

STABLE
   |
   v
DEPRECATED
   |
   v
REMOVED

Every promotion must update:

grammar/lexer/keywords.md
grammar/lexer/keywords.g4
grammar/spec/lexical.md
grammar/spec/compatibility.md
grammar/grammar.md
src/lexer.rs
parser contracts
tests

Only files actually affected by the change should be modified; unrelated files must not be touched.

---

46. "Zamani-Grammar.md" relationship

"grammar/Zamani-Grammar.md" may contain historical, proposed, experimental, or broad language vocabulary.

It is not sufficient authority to reserve a keyword.

The authority path is:

Zamani-Grammar.md
       |
       v
feature proposal
       |
       v
specification
       |
       v
keyword contract
       |
       v
keywords.g4
       |
       v
canonical lexer

This prevents the historical/aspirational grammar from silently expanding the reserved-word set.

---

47. "grammar.md" relationship

"grammar/grammar.md" describes implementation conformance.

It must report whether each keyword is:

SPECIFIED
IMPLEMENTED
PARTIALLY_IMPLEMENTED
EXPERIMENTAL
DEPRECATED
NOT_IMPLEMENTED

It must not invent an independent keyword inventory.

---

48. "grammar/lexer/tokens.g4" relationship

"tokens.g4" owns token declarations where required by the lexer architecture.

"keywords.g4" owns keyword spelling recognition.

They must not create competing definitions.

If both files describe a keyword token, they must have:

same spelling
same canonical token identity
same compatibility status
same documentation reference

There must be exactly one lexical owner for the spelling.

---

49. "grammar/lexer/identifiers.g4" relationship

"identifiers.g4" owns:

IDENTIFIER

and identifier character classes.

It must not duplicate keyword definitions.

The existing identifier contract already explicitly establishes this separation.

---

50. "grammar/lexer/operators.g4" relationship

Operators are not keywords.

For example:

+
-
*
/
==
!=
<=
>=
&&
||
>>
<<

belong to the operator system.

A symbolic mathematical operator must not be converted into a word keyword simply for convenience.

---

51. "grammar/lexer/punctuation.g4" relationship

Punctuation remains independently owned.

Examples:

(
)
[
]
{
}
,
.
:
;
@

are not keyword vocabulary.

---

52. "grammar/lexer/literals.g4" relationship

Literal values remain literal tokens.

Examples:

123
3.14
"hello"
'c'
true
false

The boolean words may be reserved lexical tokens, but their value semantics belong downstream.

---

53. Parser integration

The parser must consume the canonical keyword tokens.

A parser production should conceptually be:

functionDeclaration
    : FN identifier ...
    ;

rather than defining:

'fn'

independently in multiple parser grammars.

This creates one stable lexical-to-parser contract.

---

54. AST integration

Keywords do not automatically become AST nodes.

For example:

FN

normally contributes to the construction of a function declaration AST node.

The AST should represent:

FunctionDeclaration

not:

FnKeywordNode

Likewise:

MEASURE

should contribute to the appropriate generic/domain-neutral operation representation rather than creating a lexer-derived quantum IR.

---

55. Quantum AST integration

Quantum keywords must eventually lower through the existing domain-neutral operation model.

Conceptually:

APPLY
+
identifier/custom operation
+
targets
+
parameters
+
modifiers
        |
        v
generic AST Operation
        |
        v
semantic quantum operation
        |
        v
quantum::ir

No keyword file may introduce a second quantum IR.

---

56. Semantic integration

The semantic analyzer decides what a keyword-bearing construct means.

For example:

requires

may introduce a resource/capability requirement.

But the lexer does not determine whether that requirement is:

- satisfiable;
- portable;
- available;
- preferred;
- violated.

Those are semantic/resource-analysis responsibilities.

---

57. Resource integration

Keywords such as:

requires
capability
resource
parallel
deploy

must never encode actual hardware values.

The distinction is:

keyword
    |
    v
syntax
    |
    v
semantic requirement
    |
    v
resource/capability model
    |
    v
target realization

This is the foundation for POCO-REAF.

---

58. Compiler integration

The compiler may use keyword-derived semantic information for:

- optimization;
- lowering;
- specialization;
- target selection;
- scheduling;
- routing;
- deployment.

But the lexer must remain target-independent.

The same source keyword must tokenize identically regardless of whether compilation eventually targets:

tiny embedded system
CPU
GPU
FPGA
ASIC
QPU
cluster
cloud
supercomputer
future architecture

---

59. Runtime integration

Runtime behavior is outside the keyword layer.

No keyword may directly:

allocate memory
start a thread
start a GPU kernel
open a socket
select a QPU
measure hardware
perform calibration
execute QEC

The runtime receives already-analyzed program semantics.

---

60. Tooling integration

Tooling should obtain keyword information from the canonical vocabulary.

This includes:

- syntax highlighting;
- completion;
- diagnostics;
- formatter;
- parser tooling;
- language server;
- documentation generation;
- semantic indexing.

Tooling MUST NOT maintain independent keyword lists.

The generated/editor vocabulary should be derived from the canonical keyword contract.

---

61. Interoperability integration

Foreign-language vocabulary must remain scoped to interoperability.

For example:

rust
c
cpp
python
wasm
qasm
qir

must not become core keywords merely because Zamani interoperates with those ecosystems.

The interoperability layer determines where foreign syntax may appear.

---

62. Dialect integration

A dialect may introduce additional keywords only if the dialect mechanism explicitly permits lexical extensions.

Every dialect keyword must declare:

dialect name
dialect version
keyword spelling
token name
scope
syntax extension
AST mapping
semantic mapping
compatibility policy

Dialect keywords must not silently become core Zamani keywords.

---

63. Versioning

Keyword changes are language-version changes when they affect valid source.

Each keyword must therefore be associated with a language-version policy.

Possible statuses:

stable
experimental
contextual
deprecated
reserved-for-future
compatibility-only
dialect-only

A spelling marked:

reserved-for-future

must not necessarily have semantic syntax yet.

---

64. Reserved-for-future vocabulary

Zamani may reserve carefully selected future words to avoid future breaking changes.

However, the reserved set must remain intentionally small.

Do not reserve every conceivable concept.

In particular, do not reserve:

every quantum gate
every algorithm
every AI model
every hardware vendor
every accelerator
every protocol
every mathematical function

That would make the language less extensible.

---

65. POCO-REAF requirements

The keyword system satisfies POCO-REAF only if all of the following remain true:

Program semantics are portable

Keywords express computation and intent.

Hardware is discovered later

Keywords do not identify mandatory physical devices unless the program explicitly requires target-specific semantics.

Resource scale is unbounded

No keyword establishes a universal maximum resource count.

Domain expansion does not require keyword explosion

New operations can normally be identifiers or semantic capabilities.

Future hardware does not require rewriting existing programs

Unknown future devices can consume the same semantic program representation.

---

66. Scalability

Keyword syntax itself is constant with respect to computational scale.

A program can conceptually grow from:

one value

to:

many values

to:

many devices

to:

many nodes

to:

many quantum resources

without adding new keywords.

Scale belongs to:

program data
type parameters
generic parameters
resource requirements
capabilities
runtime resources
compiler decisions

not to the keyword vocabulary.

---

67. Hard-coding audit

The keyword system MUST NOT contain:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_DEVICES
MAX_ACCELERATORS
MAX_TIMELINES
MAX_PROCESSES
MAX_NETWORK_LINKS
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSIONS
MAX_REGISTER_WIDTH

It must also not encode fixed device identifiers such as:

GPU0
GPU1
QPU0
QPU1
CPU0
NODE0
QUBIT0

as language keywords.

---

68. Determinism

Keyword classification must be deterministic.

Given the same:

language version
dialect set
source text

the lexer must produce the same keyword classification.

No keyword recognition may depend on:

- hardware;
- environment variables;
- installed GPUs;
- installed QPUs;
- operating system;
- network;
- runtime state;
- random state.

---

69. Diagnostics

Unknown or invalid keyword-like spellings must generate diagnostics at the correct layer.

For example:

fun

when only:

fn

is reserved should not be silently converted into "FN".

The diagnostic system should be able to say that the spelling is unknown or suggest the appropriate keyword without changing the token stream implicitly.

Keyword diagnostics must preserve:

- source span;
- source spelling;
- language version;
- active dialect;
- relevant compatibility information.

---

70. Security

Keyword processing must not perform source rewriting or hidden normalization.

It must preserve source spelling and source spans.

Unicode security policy belongs jointly to:

grammar/lexer/unicode.g4
grammar/lexer/identifiers.g4
grammar/spec/lexical.md

Invisible or ambiguous characters must not silently create distinct keyword spellings.

---

71. Performance

Keyword recognition must remain efficient for arbitrarily large source files subject to available resources.

The language specification must not impose an artificial maximum number of keywords as a scalability mechanism.

Implementation may use:

- generated lexer tables;
- deterministic lookup;
- perfect hashing;
- trie-like structures;
- other safe implementation techniques.

The implementation must remain safe Rust.

No "unsafe" Rust is permitted.

---

72. Rust 1.97 / 1.97.1 requirement

The Rust implementation must compile against the repository's supported baseline:

Rust 1.97
Rust 1.97.1

The keyword contract itself is language/toolchain independent.

The Rust implementation must use safe Rust only.

No keyword feature may require:

unsafe { ... }

or an "unsafe" implementation dependency.

---

73. Tests required

Every stable keyword requires at least:

Positive lexical test

keyword

must produce the expected token.

Identifier collision test

A similar non-keyword spelling must remain an identifier.

Case test

Keyword
KEYWORD
keyword

must behave according to the case-sensitivity specification.

Boundary test

The keyword embedded in a longer identifier must not accidentally tokenize as a keyword.

Example:

fn_value

must remain an identifier.

Parser integration test

The token must be accepted in its intended syntactic context.

Negative test

The keyword must be rejected where the syntax does not allow it.

Compatibility test

The behavior must remain correct across supported language versions.

---

74. Keyword test matrix

The repository should maintain a generated conformance matrix covering at minimum:

core
control-flow
modules
types
visibility
OOP
generics
concurrency
effects
contracts
quantum
hybrid
HDL
hardware
resources
distributed
AI
data
networking
security
compile
execution
interoperability
dialects
macros
metaprogramming
Sankofa
MTS
nano

Each row should identify:

spelling
token
status
language version
dialect
grammar owner
parser consumers
AST mapping
semantic mapping
tests
compatibility status

---

75. Keyword completeness audit

The following must be reconciled before the keyword subsystem is considered complete:

grammar/lexer/keywords.md
grammar/lexer/keywords.g4
grammar/lexer/tokens.g4
grammar/antlr/ZamaniLexer.g4
grammar/antlr/*
grammar/Zamani.g4
grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/grammar.md
src/lexer.rs
src/parser.rs
src/frontend/ast/*
grammar/tests/*

The current Rust lexer already exposes vocabulary that is broader than the current keyword grammar, including "ancestor", "model", "move", "switch", and "then".

Those differences must be resolved explicitly rather than by silently deleting token variants.

For each discrepancy choose one:

CORE KEYWORD
CONTEXTUAL KEYWORD
DIALECT KEYWORD
COMPATIBILITY KEYWORD
ORDINARY IDENTIFIER
DEPRECATED
REMOVE

The decision must be recorded in the appropriate specification/compatibility contract.

---

76. Existing "keywords.g4" integration

"grammar/lexer/keywords.g4" remains the executable keyword grammar.

It should not be renamed.

It should be brought into exact conformance with this document.

The current file already establishes the correct architectural idea: it owns reserved keyword recognition, not identifiers, literals, operators, AST, semantic analysis, QEC, ZQN, routing, scheduling, hardware discovery, or runtime behavior.

That separation must be preserved.

---

77. Existing "identifiers.g4" integration

"grammar/lexer/identifiers.g4" remains the identifier authority.

It must continue to:

recognize identifier shape

and must not:

recognize keyword meaning

Its current design already explicitly prevents keyword duplication.

---

78. Existing "src/lexer.rs" integration

"src/lexer.rs" must eventually provide the same observable lexical behavior as the canonical grammar.

Its existing "TokenType" enum contains a substantial keyword set.

The completion criterion is:

keywords.md
      =
keywords.g4
      =
canonical ZamaniLexer
      =
Rust lexer observable tokenization

subject only to representation differences.

No source spelling may accidentally tokenize differently between the ANTLR and Rust frontends.

---

79. Generated artifacts

Generated lexer/parser files are implementation artifacts.

They must not become keyword authority.

The authoritative chain remains:

specification
    |
    v
keywords.md
    |
    v
keywords.g4
    |
    v
canonical lexer
    |
    v
generated artifacts

Generated artifacts must be reproducible.

---

80. Completion criteria

"grammar/lexer/keywords.md" is complete when:

- [x] keyword ownership is defined;
- [x] identifier ownership is defined;
- [x] operator ownership is defined;
- [x] punctuation ownership is defined;
- [x] literal ownership is defined;
- [x] quantum keyword policy is defined;
- [x] quantum gate policy is defined;
- [x] hardware keyword policy is defined;
- [x] resource-limit prohibition is defined;
- [x] POCO-REAF contract is defined;
- [x] Unicode policy boundary is defined;
- [x] case-sensitivity policy is defined;
- [x] contextual keyword policy is defined;
- [x] compatibility policy is defined;
- [x] token-name stability is defined;
- [x] parser integration is defined;
- [x] AST integration is defined;
- [x] semantic integration is defined;
- [x] IR integration is defined;
- [x] "quantum::ir" boundary is preserved;
- [x] compiler integration is defined;
- [x] runtime integration is defined;
- [x] tooling integration is defined;
- [x] dialect integration is defined;
- [x] interoperability integration is defined;
- [x] Rust 1.97/1.97.1 compatibility is defined;
- [x] safe-Rust-only requirement is defined;
- [x] hard-coding audit is defined;
- [x] determinism requirements are defined;
- [x] diagnostics requirements are defined;
- [x] test requirements are defined;
- [x] existing vocabulary discrepancies have an explicit resolution process.

The implementation is not complete until the corresponding executable lexer and Rust lexer conform to this contract.

---

81. Final integration contract

The completed keyword subsystem is:

                 LANGUAGE SPECIFICATION
                         |
                         v
              grammar/lexer/keywords.md
                         |
                         v
              grammar/lexer/keywords.g4
                         |
              +----------+----------+
              |                     |
              v                     v
       ZamaniLexer.g4          Rust lexer
              |                     |
              +----------+----------+
                         |
                         v
                    TOKEN STREAM
                         |
                         v
                      PARSER
                         |
                         v
                 DOMAIN-NEUTRAL AST
                         |
                         v
              SEMANTIC ANALYSIS
                         |
                         v
                CANONICAL IR MODEL
                         |
            +------------+------------+
            |            |            |
            v            v            v
       Classical    quantum::ir    HDL/Hardware
            |            |            |
            +------------+------------+
                         |
                         v
               Optimization/Lowering
                         |
                         v
              Routing/Scheduling
                         |
                         v
                Resilience/QEC/ZQN
                         |
                         v
                         HAL
                         |
                         v
                  TARGET REALIZATION

The keyword system therefore remains deliberately small, stable, target-independent, deterministic, extensible, and semantically neutral.

The central rule is:

«Reserve only what the language must reserve. Everything else remains data, an identifier, a capability, a library operation, a dialect extension, or downstream implementation detail.»

That rule is what prevents the keyword layer from becoming a hidden scalability limit and allows Zamani to grow from tiny classical programs through quantum, hybrid, HDL, AI, distributed, accelerator, and future computational systems without continually changing the lexical core.