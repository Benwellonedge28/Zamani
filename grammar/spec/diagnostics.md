Zamani Diagnostic Specification

Path: "grammar/spec/diagnostics.md"
Language: Zamani
Specification role: Normative diagnostic, error-reporting, recovery, tooling, and conformance contract
Specification status: Production architecture
Specification version: 1.0
Implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Rust safety requirement: Production compiler implementation MUST use safe Rust; Rust "unsafe" is prohibited
Portability principle: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability principle: No artificial language-level machine, hardware, topology, or resource ceiling
Canonical quantum semantic boundary: "quantum::ir"

---

0. Purpose

This document defines the normative diagnostic contract for Zamani.

Diagnostics are part of the language implementation contract.

A production Zamani compiler MUST be able to communicate, in a structured and deterministic way:

- what went wrong;
- where it happened;
- which language phase detected it;
- why it is invalid;
- what source construct was involved;
- what was expected where applicable;
- what was actually encountered where applicable;
- whether the issue is recoverable;
- whether compilation can continue safely;
- whether the issue is semantic or merely target/resource dependent;
- whether the issue is an error, warning, informational message, hint, or note;
- whether a fix can be suggested;
- whether the issue is version-specific;
- whether the issue affects portability;
- whether the issue originates from a downstream target rather than the Zamani source itself.

Diagnostics MUST integrate consistently across:

Source
  ↓
Source decoding
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
Ownership/resource/capability analysis
  ↓
Domain semantic analysis
  ↓
Canonical semantic model
  ↓
IR generation
  ↓
IR verification
  ↓
Optimization/lowering
  ↓
Routing/scheduling/resilience/QEC/ZQN
  ↓
HAL
  ↓
Target realization
  ↓
Runtime/execution

The diagnostic architecture MUST NOT become a second semantic system.

Diagnostics report the result of the responsible subsystem. They do not redefine that subsystem's rules.

---

1. Scope

This specification owns:

- diagnostic severity;
- diagnostic identity;
- diagnostic codes;
- diagnostic categories;
- diagnostic phases;
- source locations;
- source spans;
- primary labels;
- secondary labels;
- notes;
- help text;
- related information;
- machine-readable data;
- structured suggestions;
- fix-it representation;
- recovery classification;
- deterministic ordering;
- deduplication;
- diagnostic rendering;
- diagnostic serialization;
- diagnostic compatibility;
- diagnostic testing;
- diagnostic budgets;
- diagnostic scalability;
- IDE/editor behavior;
- batch compiler behavior;
- library/API behavior;
- cross-phase diagnostic propagation.

This specification does not own:

- lexical grammar;
- parser grammar;
- type rules;
- effect rules;
- ownership rules;
- resource allocation;
- quantum semantics;
- QEC;
- ZQN;
- routing;
- scheduling;
- HAL;
- target implementation;
- runtime implementation.

Those systems remain authoritative for their own semantics.

---

2. Authority and Repository Integration

The diagnostic specification integrates with the existing repository authority model.

The relevant relationship is:

grammar/spec/diagnostics.md
        │
        ├────────────── lexical diagnostics
        ├────────────── parser diagnostics
        ├────────────── semantic diagnostics
        ├────────────── resource diagnostics
        ├────────────── domain diagnostics
        ├────────────── IR diagnostics
        └────────────── target diagnostics
                         │
                         ▼
                    diagnostics API
                         │
             ┌───────────┼───────────┐
             ▼           ▼           ▼
          compiler      CLI         tooling
             │                       │
             ▼                       ▼
          stderr/JSON             LSP/IDE

The existing repository authority model remains:

grammar/spec/lexical.md
        ↓
grammar/spec/syntax.md
        ↓
grammar/spec/semantics.md
        ↓
grammar/spec/diagnostics.md
        ↓
grammar/spec/compatibility.md
        ↓
Zamani.g4
        ↓
src/lexer.rs
        ↓
src/parser.rs
        ↓
src/frontend/ast/
        ↓
semantic analysis
        ↓
canonical semantic model
        ↓
IR

"grammar/diagnostics.md" MUST NOT become a competing diagnostic specification if this file is established as the normative location.

"grammar/grammar.md" MAY document currently implemented diagnostic behavior, but it MUST NOT override this specification.

"grammar/Zamani-Grammar.md" MAY contain proposed diagnostics, but those proposals are not normative until promoted.

---

3. Diagnostic Design Principles

Every production diagnostic MUST satisfy the following principles.

3.1 Correctness

The diagnostic MUST correspond to a real condition detected by the responsible subsystem.

The compiler MUST NOT manufacture an error merely because a target lacks a particular implementation strategy when another valid realization exists.

3.2 Determinism

For identical:

- source;
- language version;
- compiler configuration;
- diagnostic configuration;
- semantic inputs;
- target constraints;

diagnostic identity, severity, source locations, and ordering MUST be deterministic.

Diagnostics MUST NOT depend on:

- hash-map iteration order;
- filesystem traversal order;
- CPU count;
- GPU availability;
- QPU availability;
- network timing;
- wall-clock time;
- random state;
- thread scheduling;
- target enumeration order.

3.3 Actionability

A diagnostic SHOULD explain what the programmer needs to know to correct the problem.

3.4 Precision

The compiler SHOULD identify the smallest useful source region.

A diagnostic SHOULD NOT highlight an entire source file when one token or expression is sufficient.

3.5 Stability

Diagnostic codes MUST be more stable than human-readable wording.

Tooling MUST consume diagnostic codes and structured fields rather than parsing prose.

3.6 Separation of concerns

The diagnostic system MUST distinguish:

language invalidity
        ≠
semantic invalidity
        ≠
resource unsatisfiability
        ≠
target incompatibility
        ≠
runtime failure

This distinction is essential to POCO-REAF.

---

4. Diagnostic Severity

Zamani defines the following severities.

4.1 Error

The requested operation cannot produce a valid result under the current compilation conditions.

An error normally prevents successful compilation of the affected compilation unit.

Examples:

unknown identifier
invalid syntax
type mismatch
invalid ownership operation
unsatisfied mandatory capability
invalid quantum operation
invalid HDL connection

4.2 Warning

The program is potentially valid, but the compiler identifies a condition that may indicate a defect, portability issue, deprecated behavior, or undesirable construction.

Warnings MUST NOT silently alter program semantics.

4.3 Information

Informational output communicates compiler state or non-error information.

Examples:

feature is experimental
target-specific optimization applied
fallback realization selected

Informational diagnostics MUST NOT be required for program correctness.

4.4 Hint

A hint suggests a possible improvement.

A hint MUST NOT imply that the program is invalid.

4.5 Note

A note provides context for another diagnostic.

A note MUST NOT normally appear as an independent diagnostic.

---

5. Diagnostic Identity

Every diagnostic MUST have a stable machine-readable code.

The canonical conceptual structure is:

ZMN-<DOMAIN>-<CONDITION>

Examples:

ZMN-LEX-INVALID-UTF8
ZMN-LEX-INVALID-NUMBER
ZMN-PARSE-UNEXPECTED-TOKEN
ZMN-NAME-UNKNOWN-IDENTIFIER
ZMN-TYPE-MISMATCH
ZMN-EFFECT-UNDECLARED
ZMN-RESOURCE-UNSATISFIED
ZMN-CAPABILITY-MISSING
ZMN-QUANTUM-INVALID-OPERATION
ZMN-QUANTUM-RESOURCE-UNSATISFIED
ZMN-HDL-INVALID-CONNECTION
ZMN-IR-LOWERING-UNSUPPORTED
ZMN-IR-VERIFICATION-FAILED
ZMN-TARGET-UNSUPPORTED
ZMN-COMPAT-DEPRECATED
ZMN-SAFETY-UNSAFE-DISALLOWED

The exact code registry MUST be maintained centrally.

No subsystem may invent incompatible code formats.

---

6. Diagnostic Code Stability

Diagnostic codes are compatibility identifiers.

Once a diagnostic code is released as stable:

- its fundamental meaning MUST NOT be silently changed;
- its category MUST NOT be silently changed;
- its severity MUST NOT be changed if doing so breaks tooling assumptions;
- its structured payload schema MUST remain backward compatible where practical.

Human-readable wording MAY improve without changing the diagnostic code.

For example:

ZMN-TYPE-MISMATCH

may change its wording between compiler releases, but tooling continues to identify the same class of condition through the code.

---

7. Diagnostic Domains

The following domains are normative.

LEX
PARSE
NAME
MODULE
TYPE
EFFECT
OWNERSHIP
MEMORY
RESOURCE
CAPABILITY
CLASSICAL
QUANTUM
HYBRID
HDL
HARDWARE
DISTRIBUTED
AI
DATA
NETWORK
SECURITY
MACRO
META
DIALECT
INTEROP
SEMANTIC
IR
OPT
ROUTE
SCHEDULE
QEC
ZQN
HAL
TARGET
RUNTIME
COMPAT
SAFETY
TOOL
INTERNAL

A domain MAY contain additional codes without changing the overall diagnostic architecture.

---

8. Diagnostic Phases

Diagnostics MUST identify the phase that produced them.

Canonical phases are:

SOURCE
LEXICAL
PARSE
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
RESILIENCE
QEC
ZQN
HAL
TARGET
RUNTIME
TOOLING
COMPATIBILITY

The phase is machine-readable.

It MUST NOT be inferred from the diagnostic message text.

---

9. Diagnostic Structure

The conceptual diagnostic structure is:

Diagnostic {
    code
    severity
    phase
    message
    primary_span
    labels
    notes
    help
    related_information
    suggestions
    machine_data
    origin
    applicability
}

The exact Rust representation may differ.

The semantic fields MUST remain available.

---

10. Message

Every diagnostic MUST have a concise primary message.

Messages SHOULD:

- describe the condition;
- avoid unnecessary implementation details;
- avoid internal Rust terminology;
- avoid unstable memory addresses;
- avoid nondeterministic identifiers;
- avoid target-specific details unless target context is relevant.

Messages SHOULD describe what the compiler knows.

Bad:

parser failed somehow

Good:

expected an expression after `=`

---

11. Primary Span

An error MUST have a primary source span whenever a source location exists.

The primary span identifies the source region most directly responsible for the diagnostic.

Examples:

let x = ;
        ^

or:

apply H to q[unknown];
              ^^^^^^^

For errors caused by multiple locations, the most causally significant location MUST be primary.

---

12. Secondary Spans

A diagnostic MAY contain secondary spans.

Secondary spans explain relationships.

Example:

error: `x` is already defined in this scope
  --> current.z:10:5
   |
10 | let x = 2;
   |     ^ second definition

note: previous definition is here
  --> current.z:4:5
   |
 4 | let x = 1;
   |     ^ first definition

Secondary spans MUST be explicitly labeled.

---

13. Source Span Model

Source spans MUST be represented independently from rendered line/column formatting.

The canonical semantic representation SHOULD be based on source identity plus byte offsets:

SourceFileId
start_byte
end_byte

Line and column are derived presentation information.

The compiler MUST NOT store only human-readable line/column information because:

- Unicode changes byte/column relationships;
- tabs affect visual columns;
- source edits require stable source mapping;
- tooling needs precise ranges;
- multiple files may participate in one diagnostic.

---

14. Zero-Length Spans

Zero-length spans MAY be used for insertion diagnostics.

Example:

fn calculate(x: Int)
                 ^
                 expected `{`

The diagnostic MUST distinguish:

zero-length insertion location

from:

empty source range caused by malformed input

---

15. Multi-File Diagnostics

Diagnostics MAY reference multiple files.

Examples:

- imported module error;
- type declaration mismatch;
- macro expansion;
- foreign interface mismatch;
- generated-source mapping;
- cross-module resource requirement;
- conflicting declarations.

Each span MUST include its source identity.

A diagnostic MUST NOT assume that all source locations belong to one file.

---

16. Source Maps

Generated or transformed source MUST preserve mappings back to the originating source whenever practical.

This applies to:

- macros;
- generated code;
- metaprogramming;
- dialect lowering;
- source transformations;
- compiler-generated declarations.

When an error originates in generated code, the compiler SHOULD report the originating user source first and provide generated-source information as secondary context.

---

17. Expected and Actual Information

Parser and semantic diagnostics SHOULD expose structured expected/actual information.

Conceptually:

expected:
    expression

actual:
    `;`

This information MUST be machine-readable where practical.

Human-readable messages MUST NOT be the only representation.

---

18. Suggestions

Suggestions are structured guidance.

Examples:

consider importing `math`

or:

consider changing `Int` to `Float`

A suggestion MUST NOT be represented merely as prose when an exact machine-applicable edit is known.

---

19. Fix-Its

A fix-it consists conceptually of:

replacement {
    source_file
    span
    replacement_text
}

A fix-it MAY contain multiple edits.

All edits MUST be:

- source-specific;
- deterministic;
- non-overlapping unless explicitly supported;
- applicable to the source version for which they were generated.

The compiler MUST NOT emit a fix-it that silently changes program semantics merely to suppress a diagnostic.

---

20. Safe Fix-Its

A fix-it is considered safe only when the compiler has sufficient information to know that the replacement addresses the diagnosed condition without introducing an unrelated semantic change.

Examples of generally safe fixes:

missing closing delimiter

where the insertion is unambiguous.

Examples requiring caution:

change type from A to B

if multiple semantic interpretations exist.

Unsafe or uncertain suggestions SHOULD be emitted as ordinary help text rather than automatic edits.

---

21. Notes

Notes provide explanatory context.

Examples:

note: quantum operations are lowered through `quantum::ir`

or:

note: this capability is required by the selected execution policy

Notes MUST NOT contain essential machine-readable information that tooling needs to understand the diagnostic.

---

22. Help Text

Help text SHOULD describe an actionable path forward.

Examples:

help: declare the required capability explicitly

or:

help: use a logical qubit resource instead of a physical qubit identifier

Help MUST NOT imply that a target limitation is a language limitation.

---

23. Resource and Capability Diagnostics

Resource diagnostics are particularly important to POCO-REAF.

The compiler MUST distinguish:

invalid program

from:

valid program whose requirements cannot be satisfied by this target

For example:

requires qubits >= n

may be semantically valid even when a selected target cannot provide enough resources.

The correct diagnostic is therefore target/resource-specific:

ZMN-RESOURCE-UNSATISFIED

rather than:

ZMN-QUANTUM-INVALID

unless the program itself violates quantum semantics.

---

24. Resource Diagnostic Payload

A resource-unsatisfied diagnostic SHOULD expose structured information such as:

resource_kind
requested
available
unit
capability
target
constraint
scope
alternatives

Example conceptual payload:

resource_kind: quantum.qubits
requested: n
available: 128
unit: qubit
target: selected-target

The values MUST NOT be reduced to fixed-width host representations merely for diagnostic convenience.

---

25. Resource Scalability

The diagnostic system MUST NOT impose artificial language limits.

It MUST NOT define:

MAX_DIAGNOSTICS = fixed universal language limit
MAX_RESOURCE_QUANTITY
MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_NODES
MAX_THREADS

A compiler implementation MAY enforce configurable resource budgets.

Those budgets MUST be classified separately from language validity.

For example:

ZMN-RESOURCE-COMPILER-BUDGET

is different from:

ZMN-PARSE-INVALID-SYNTAX

---

26. Diagnostic Budgets

A compiler MAY enforce budgets such as:

- maximum diagnostic count;
- maximum diagnostic storage;
- maximum source-analysis memory;
- maximum compilation time;
- maximum output size.

When a budget is exhausted, the compiler MUST report a resource-budget condition rather than pretending that the source is invalid.

Example:

ZMN-RESOURCE-DIAGNOSTIC-BUDGET

The compiler SHOULD indicate that additional diagnostics were suppressed.

Example:

additional diagnostics were suppressed because the diagnostic budget was exhausted

---

27. Diagnostic Overflow

If the number of diagnostics exceeds the configured reporting budget:

1. diagnostics MUST remain deterministic;
2. the first selected diagnostics MUST follow deterministic ordering;
3. suppression MUST be reported;
4. the compiler MUST NOT claim that no additional problems exist;
5. tooling MUST be able to distinguish suppression from absence of diagnostics.

---

28. Lexical Diagnostics

Lexical diagnostics include, where applicable:

ZMN-LEX-INVALID-UTF8
ZMN-LEX-INVALID-CHARACTER
ZMN-LEX-INVALID-IDENTIFIER
ZMN-LEX-INVALID-NUMBER
ZMN-LEX-INVALID-STRING
ZMN-LEX-INVALID-ESCAPE
ZMN-LEX-UNTERMINATED-STRING
ZMN-LEX-UNTERMINATED-COMMENT
ZMN-LEX-INVALID-OPERATOR
ZMN-LEX-RESERVED-SPELLING

The lexer MUST preserve sufficient source information for downstream diagnostics.

---

29. Parser Diagnostics

Parser diagnostics include:

ZMN-PARSE-UNEXPECTED-TOKEN
ZMN-PARSE-EXPECTED-TOKEN
ZMN-PARSE-EXPECTED-EXPRESSION
ZMN-PARSE-EXPECTED-TYPE
ZMN-PARSE-EXPECTED-DECLARATION
ZMN-PARSE-UNTERMINATED-BLOCK
ZMN-PARSE-UNTERMINATED-DELIMITER
ZMN-PARSE-INVALID-STRUCTURE
ZMN-PARSE-AMBIGUOUS-CONSTRUCT

Parser diagnostics MUST identify the relevant source location.

---

30. Parser Recovery

Parser recovery exists to permit additional diagnostics.

Recovery MUST NOT convert invalid syntax into a semantically valid program.

The parser MAY construct an error-tolerant intermediate representation for IDE/tooling purposes.

Such a representation MUST be distinguishable from a valid production AST.

Production compilation MUST NOT treat parser error placeholders as valid semantic constructs.

---

31. Recovery Boundaries

The parser SHOULD recover at stable synchronization points such as:

- statement boundaries;
- declaration boundaries;
- block delimiters;
- module boundaries;
- expression delimiters;
- commas;
- semicolons;
- closing delimiters.

Recovery MUST be deterministic.

The compiler MUST avoid cascades where one missing delimiter causes hundreds of unrelated diagnostics whenever a more precise recovery strategy is available.

---

32. AST Diagnostics

AST construction SHOULD produce diagnostics only for structural invariants that cannot be represented by parser errors.

Examples:

ZMN-AST-INVALID-INVARIANT
ZMN-AST-MISSING-SPAN
ZMN-AST-INVALID-NODE

An AST diagnostic generally indicates an implementation/conformance defect rather than ordinary user syntax.

Such diagnostics SHOULD be classified as internal/compiler errors unless the malformed AST originated from an explicitly error-tolerant frontend mode.

---

33. Name Resolution Diagnostics

Name-related diagnostics include:

ZMN-NAME-UNKNOWN-IDENTIFIER
ZMN-NAME-DUPLICATE-DEFINITION
ZMN-NAME-AMBIGUOUS
ZMN-NAME-OUT-OF-SCOPE
ZMN-NAME-INVALID-IMPORT
ZMN-NAME-PRIVATE-ACCESS

Name diagnostics SHOULD identify:

- unresolved name;
- relevant scope;
- candidate declarations where useful;
- declaration source locations where relevant.

---

34. Module Diagnostics

Module diagnostics include:

ZMN-MODULE-NOT-FOUND
ZMN-MODULE-CYCLE
ZMN-MODULE-DUPLICATE
ZMN-MODULE-VERSION-MISMATCH
ZMN-MODULE-INVALID-EXPORT
ZMN-MODULE-INVALID-IMPORT

Filesystem or package-resolution failures MUST be distinguished from invalid language syntax.

---

35. Type Diagnostics

Type diagnostics include:

ZMN-TYPE-MISMATCH
ZMN-TYPE-UNKNOWN
ZMN-TYPE-ARGUMENT-MISMATCH
ZMN-TYPE-RETURN-MISMATCH
ZMN-TYPE-CONSTRAINT-UNSATISFIED
ZMN-TYPE-INFERENCE-AMBIGUOUS
ZMN-TYPE-INVALID-CONVERSION
ZMN-TYPE-INVALID-OPERATION

A type diagnostic MUST identify the relevant source expressions when possible.

---

36. Effect Diagnostics

Effect diagnostics include:

ZMN-EFFECT-UNDECLARED
ZMN-EFFECT-NOT-AVAILABLE
ZMN-EFFECT-MISMATCH
ZMN-EFFECT-HANDLER-MISSING
ZMN-EFFECT-HANDLER-INCOMPATIBLE

Effects MUST be diagnosed at the semantic level.

The parser MUST NOT attempt to determine whether an effect is physically executable.

---

37. Ownership and Memory Diagnostics

Ownership/memory diagnostics include:

ZMN-OWNERSHIP-MOVE
ZMN-OWNERSHIP-USE-AFTER-MOVE
ZMN-OWNERSHIP-BORROW
ZMN-OWNERSHIP-CONFLICT
ZMN-MEMORY-INVALID-REGION
ZMN-MEMORY-INVALID-LIFETIME
ZMN-MEMORY-INVALID-ADDRESS-SPACE
ZMN-MEMORY-CAPABILITY

Physical memory placement is not a source-language ownership error unless the language semantics explicitly require it.

---

38. Capability Diagnostics

Capability diagnostics include:

ZMN-CAPABILITY-MISSING
ZMN-CAPABILITY-INCOMPATIBLE
ZMN-CAPABILITY-CONSTRAINT
ZMN-CAPABILITY-VERSION
ZMN-CAPABILITY-NEGOTIATION

A missing target capability MUST NOT automatically imply invalid Zamani source.

The diagnostic classification MUST identify whether the failure occurs during:

- semantic validation;
- compilation;
- target selection;
- deployment;
- runtime.

---

39. Classical Computing Diagnostics

Classical-domain diagnostics MAY include:

ZMN-CLASSICAL-INVALID-OPERATION
ZMN-CLASSICAL-NUMERIC-RANGE
ZMN-CLASSICAL-PRECISION
ZMN-CLASSICAL-SHAPE
ZMN-CLASSICAL-DIMENSION
ZMN-CLASSICAL-ALGORITHM-CONSTRAINT

A mathematical or numerical limitation MUST NOT be confused with a parser limitation.

For example, an integer that cannot fit a selected target representation is not necessarily an invalid lexical integer.

---

40. Quantum Diagnostics

Quantum diagnostics MUST preserve the distinction between:

invalid quantum semantics

and:

valid quantum semantics that cannot be realized by a selected target

Canonical examples include:

ZMN-QUANTUM-INVALID-OPERATION
ZMN-QUANTUM-INVALID-TARGET
ZMN-QUANTUM-INVALID-ARITY
ZMN-QUANTUM-INVALID-PARAMETER
ZMN-QUANTUM-INVALID-CONTROL
ZMN-QUANTUM-INVALID-ADJOINT
ZMN-QUANTUM-INVALID-MEASUREMENT
ZMN-QUANTUM-INVALID-STATE
ZMN-QUANTUM-RESOURCE-UNSATISFIED
ZMN-QUANTUM-CAPABILITY-MISSING
ZMN-QUANTUM-LOWERING-UNSUPPORTED

Quantum diagnostics MUST NOT establish an independent quantum IR.

The semantic path remains:

Zamani quantum syntax
        ↓
frontend AST
        ↓
semantic quantum model
        ↓
quantum::ir
        ↓
verification
        ↓
optimization
        ↓
routing
        ↓
scheduling
        ↓
QEC / resilience
        ↓
ZQN
        ↓
HAL
        ↓
target realization

---

41. Quantum Operation Names

The diagnostic architecture MUST support generic quantum operations.

The compiler MUST NOT require the diagnostic registry to enumerate every possible gate.

For example, diagnostics MUST be able to report:

operation `custom_operation` requires 2 targets

without requiring:

ZMN-QUANTUM-CUSTOM-OPERATION-X

for every operation name.

Operation identity belongs in structured diagnostic data.

---

42. Quantum Resource Diagnostics

If a valid program requires:

n logical qubits

but a target provides fewer resources, the compiler SHOULD report:

ZMN-QUANTUM-RESOURCE-UNSATISFIED

with structured fields such as:

requested = n
available = ...
resource = logical_qubits
target = ...

The compiler MUST NOT silently:

- delete qubits;
- reduce the computation;
- change the algorithm;
- reinterpret logical qubits as physical qubits;
- remove operations.

---

43. QEC Diagnostics

QEC diagnostics belong to the QEC subsystem.

Examples:

ZMN-QEC-REQUIREMENT-UNSATISFIED
ZMN-QEC-CODE-INCOMPATIBLE
ZMN-QEC-RESOURCE-UNSATISFIED
ZMN-QEC-LOWERING-FAILED

The grammar does not implement QEC.

The diagnostic merely reports QEC semantic or realization conditions.

---

44. ZQN Diagnostics

ZQN diagnostics belong to fault/noise semantics.

Examples:

ZMN-ZQN-MODEL-INCOMPATIBLE
ZMN-ZQN-NOISE-CONSTRAINT
ZMN-ZQN-FAULT-BUDGET
ZMN-ZQN-VERIFICATION

ZQN diagnostics MUST NOT be used to redefine quantum syntax.

---

45. HDL Diagnostics

HDL diagnostics include:

ZMN-HDL-INVALID-PORT
ZMN-HDL-INVALID-CONNECTION
ZMN-HDL-WIDTH-MISMATCH
ZMN-HDL-CLOCK-CONSTRAINT
ZMN-HDL-RESET-CONSTRAINT
ZMN-HDL-TIMING-CONSTRAINT
ZMN-HDL-INVALID-NET
ZMN-HDL-INVALID-STATE
ZMN-HDL-SYNTHESIS-UNSUPPORTED
ZMN-HDL-VERIFICATION-FAILED

HDL width diagnostics MUST distinguish:

program-defined width mismatch

from:

target-specific hardware width limitation

The language MUST NOT introduce artificial universal register-width limits.

---

46. Distributed Diagnostics

Distributed diagnostics include:

ZMN-DISTRIBUTED-PLACEMENT
ZMN-DISTRIBUTED-CONSISTENCY
ZMN-DISTRIBUTED-COMMUNICATION
ZMN-DISTRIBUTED-RESOURCE
ZMN-DISTRIBUTED-CAPABILITY
ZMN-DISTRIBUTED-TOPOLOGY
ZMN-DISTRIBUTED-FAULT-TOLERANCE

A target with insufficient nodes MUST NOT make a portable program syntactically invalid.

---

47. AI and Data Diagnostics

AI/data diagnostics include:

ZMN-AI-SHAPE-MISMATCH
ZMN-AI-TYPE-MISMATCH
ZMN-AI-CAPABILITY
ZMN-AI-RESOURCE
ZMN-AI-MODEL-CONSTRAINT
ZMN-DATA-SCHEMA-MISMATCH
ZMN-DATA-SHAPE-MISMATCH
ZMN-DATA-STREAM-CONSTRAINT
ZMN-DATA-PROVENANCE

Framework-specific failures MUST remain distinguishable from Zamani language errors.

---

48. Networking Diagnostics

Networking diagnostics include:

ZMN-NETWORK-INVALID-ENDPOINT
ZMN-NETWORK-PROTOCOL
ZMN-NETWORK-CAPABILITY
ZMN-NETWORK-ROUTING
ZMN-NETWORK-ADDRESS
ZMN-NETWORK-SECURITY
ZMN-NETWORK-TIMEOUT

Runtime network failures MUST NOT be misreported as compile-time language errors.

---

49. Security Diagnostics

Security diagnostics include:

ZMN-SECURITY-UNAUTHORIZED
ZMN-SECURITY-CAPABILITY
ZMN-SECURITY-POLICY
ZMN-SECURITY-KEY
ZMN-SECURITY-CRYPTOGRAPHIC-CONSTRAINT
ZMN-SECURITY-TRUST
ZMN-SECURITY-PROVENANCE

Sensitive information MUST NOT be leaked into diagnostics.

Diagnostics MUST NOT expose:

- secret keys;
- credentials;
- authentication tokens;
- private memory;
- confidential source content beyond the required source span.

---

50. Interoperability Diagnostics

Interop diagnostics include:

ZMN-INTEROP-ABI
ZMN-INTEROP-TYPE
ZMN-INTEROP-CALLING-CONVENTION
ZMN-INTEROP-FOREIGN-SYMBOL
ZMN-INTEROP-SERIALIZATION
ZMN-INTEROP-FORMAT
ZMN-INTEROP-VERSION

External formats such as OpenQASM, QIR, HDL formats, C/C++, Rust, WebAssembly, or other representations MUST NOT become competing semantic authorities.

---

51. IR Diagnostics

IR generation diagnostics include:

ZMN-IR-LOWERING-UNSUPPORTED
ZMN-IR-LOWERING-INVALID
ZMN-IR-LOWERING-LOSS
ZMN-IR-INVALID-REFERENCE
ZMN-IR-INVALID-TYPE
ZMN-IR-INVALID-EFFECT
ZMN-IR-INVALID-RESOURCE

The compiler MUST NEVER silently discard source semantics during lowering.

If a construct cannot be represented:

source
 ↓
AST
 ↓
semantic model
 ↓
IR lowering

the compiler MUST produce a structured diagnostic.

---

52. IR Verification Diagnostics

IR verification diagnostics include:

ZMN-IR-VERIFY-INVARIANT
ZMN-IR-VERIFY-TYPE
ZMN-IR-VERIFY-CONTROL
ZMN-IR-VERIFY-RESOURCE
ZMN-IR-VERIFY-EFFECT
ZMN-IR-VERIFY-QUANTUM
ZMN-IR-VERIFY-OWNERSHIP

An IR verifier failure indicates either:

1. an invalid semantic transformation; or
2. an implementation defect.

The compiler MUST NOT convert verifier failures into ordinary user warnings.

---

53. Optimization Diagnostics

Optimization diagnostics SHOULD generally be informational unless an optimization violates a required semantic constraint.

Examples:

ZMN-OPT-REJECTED
ZMN-OPT-CONSTRAINT
ZMN-OPT-FAILED

Optimization failure MUST NOT normally cause compilation failure if an unoptimized valid realization remains possible.

---

54. Routing Diagnostics

Routing diagnostics include:

ZMN-ROUTE-UNAVAILABLE
ZMN-ROUTE-CONSTRAINT
ZMN-ROUTE-RESOURCE
ZMN-ROUTE-TOPOLOGY
ZMN-ROUTE-CAPABILITY

A routing failure is not automatically a source-language error.

The compiler SHOULD distinguish:

no valid realization for selected target

from:

program itself violates a routing-independent semantic rule

---

55. Scheduling Diagnostics

Scheduling diagnostics include:

ZMN-SCHEDULE-RESOURCE
ZMN-SCHEDULE-DEPENDENCY
ZMN-SCHEDULE-TIMING
ZMN-SCHEDULE-CAPABILITY
ZMN-SCHEDULE-DEADLINE

Scheduling MUST remain downstream of language semantics.

---

56. HAL Diagnostics

HAL diagnostics describe target capability/state.

Examples:

ZMN-HAL-DEVICE-UNAVAILABLE
ZMN-HAL-CAPABILITY
ZMN-HAL-CALIBRATION
ZMN-HAL-RESOURCE
ZMN-HAL-STATE

HAL diagnostics MUST NOT redefine source syntax.

---

57. Target Diagnostics

Target diagnostics include:

ZMN-TARGET-UNSUPPORTED
ZMN-TARGET-CAPABILITY
ZMN-TARGET-RESOURCE
ZMN-TARGET-CONSTRAINT
ZMN-TARGET-ABI
ZMN-TARGET-VERSION
ZMN-TARGET-REALIZATION

These diagnostics are essential to POCO-REAF.

A program may be:

valid Zamani

while simultaneously being:

not realizable on target X

Those states MUST remain distinguishable.

---

58. Runtime Diagnostics

Runtime diagnostics include:

ZMN-RUNTIME-RESOURCE
ZMN-RUNTIME-CAPABILITY
ZMN-RUNTIME-IO
ZMN-RUNTIME-NETWORK
ZMN-RUNTIME-QUANTUM
ZMN-RUNTIME-FAULT
ZMN-RUNTIME-CANCELLATION
ZMN-RUNTIME-TIMEOUT
ZMN-RUNTIME-FAILURE

Runtime failures MUST NOT be presented as compile-time syntax errors.

---

59. Compatibility Diagnostics

Compatibility diagnostics include:

ZMN-COMPAT-DEPRECATED
ZMN-COMPAT-REMOVED
ZMN-COMPAT-RESERVED
ZMN-COMPAT-VERSION
ZMN-COMPAT-FEATURE-GATE
ZMN-COMPAT-SYNTAX
ZMN-COMPAT-SEMANTICS

A removed feature MUST produce a deterministic diagnostic rather than being silently reinterpreted.

---

60. Safety Diagnostics

The compiler implementation MUST use safe Rust.

The following are prohibited in production compiler implementation:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

The corresponding repository policy MUST be independently checked by tooling.

For the Zamani source language, the existing "unsafe" vocabulary MUST be treated according to the canonical lexical/compatibility policy.

If the language reserves "unsafe" without providing an executable source-level unsafe model, the diagnostic SHOULD be:

ZMN-SAFETY-UNSAFE-DISALLOWED

The compiler MUST NOT claim that merely parsing an "unsafe" construct provides a safety guarantee.

---

61. Internal Compiler Diagnostics

Internal errors must be distinguishable from user errors.

Examples:

ZMN-INTERNAL-AST-INVARIANT
ZMN-INTERNAL-IR-INVARIANT
ZMN-INTERNAL-DIAGNOSTIC-INVARIANT
ZMN-INTERNAL-UNEXPECTED-STATE

Internal diagnostics SHOULD include:

- compiler version;
- phase;
- stable internal identifier;
- relevant source span;
- reproducibility information;
- non-sensitive context.

They MUST NOT expose secrets or unstable memory addresses.

---

62. No Panics for User Input

Malformed user input MUST NOT require Rust "panic!" as the ordinary diagnostic mechanism.

User-controlled invalid input MUST be represented through structured diagnostics or defined errors.

A panic MAY indicate an internal compiler defect, but production compiler code SHOULD minimize uncontrolled panics.

---

63. Diagnostic Ordering

Diagnostics MUST have deterministic ordering.

The canonical ordering SHOULD prioritize:

1. source file order;
2. source byte offset;
3. diagnostic phase;
4. diagnostic severity;
5. stable diagnostic code;
6. deterministic internal sequence.

Implementations MAY use another ordering only if the ordering is explicitly standardized and deterministic.

Hash iteration order MUST NOT determine diagnostic ordering.

---

64. Diagnostic Deduplication

The compiler MAY deduplicate equivalent diagnostics.

Deduplication MUST NOT hide distinct source locations or distinct semantic causes.

Two diagnostics SHOULD be considered equivalent only when their structured identity and relevant source context are equivalent.

---

65. Cascading Errors

The compiler SHOULD prevent cascading diagnostics when one root cause explains multiple downstream failures.

For example:

missing closing `}`

SHOULD NOT produce hundreds of unrelated:

unknown identifier
invalid statement
invalid expression

diagnostics caused solely by parser desynchronization.

When cascading diagnostics are still useful, they SHOULD identify their dependency on the primary error.

---

66. Diagnostic Suppression

Diagnostics MAY be suppressed through explicit configuration.

Suppression MUST NOT alter semantic analysis.

A suppressed diagnostic remains semantically detected; only its presentation is changed.

Suppression configuration MUST be:

- explicit;
- version-aware;
- deterministic;
- tool-readable.

---

67. Warnings and Portability

A warning MAY identify target coupling.

For example:

ZMN-TARGET-PORTABILITY

may indicate that a program explicitly requested a target-specific capability.

The warning MUST NOT claim that target-specific code is invalid if the language permits it.

The distinction is:

portable

versus:

explicitly target-dependent

rather than:

valid

versus:

invalid

---

68. Hard-Coding Diagnostics

The grammar/validation infrastructure SHOULD detect prohibited universal hardware constants.

Examples include:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_TIMELINES
MAX_DEVICES

Such detection MUST distinguish:

program-defined constant

from:

compiler-defined universal ceiling

For example:

let matrix_size = 1024;

is not inherently an error.

A compiler implementation containing:

const MAX_QUBITS: usize = 1024;

as the universal language limit is an architectural violation.

---

69. Requirement/Capability Diagnostic Separation

The compiler MUST distinguish:

requires capability X

from:

target provides capability Y

and:

target selected implementation Z

A diagnostic SHOULD identify which layer failed.

Example:

ZMN-CAPABILITY-MISSING
required capability `quantum.mid_circuit_measurement`
selected target does not provide the capability

This is preferable to:

invalid quantum program

when the source is semantically valid.

---

70. Diagnostic Data for Resource Negotiation

Resource/capability diagnostics SHOULD support machine-readable alternatives.

For example:

alternatives:
  - target with capability X
  - distributed realization
  - simulator
  - logical-resource realization

These alternatives are advisory.

The diagnostic system MUST NOT itself choose the target.

---

71. No Hidden Semantic Repair

Diagnostics MUST NOT silently repair semantics.

The compiler MUST NOT:

- delete an invalid operation;
- replace an unknown quantum operation with a no-op;
- truncate a resource quantity;
- reduce tensor dimensions;
- reduce qubit counts;
- ignore unsupported modifiers;
- drop effect annotations;
- drop resource requirements;
- silently choose another type;
- silently select another target.

If a recovery transformation exists, it MUST be explicitly represented and constrained.

---

72. Error-Tolerant IDE Mode

The compiler MAY support error-tolerant analysis for:

- IDEs;
- language servers;
- syntax highlighting;
- code completion;
- incremental analysis.

In that mode:

invalid source

MAY produce:

error-tolerant AST

but the AST MUST preserve the fact that errors occurred.

IDE recovery MUST NOT imply production compilation success.

---

73. Incremental Diagnostics

Incremental compilation MAY reuse diagnostics from unchanged regions.

Reused diagnostics MUST remain valid under the current:

- source version;
- language version;
- configuration;
- dependency state;
- semantic environment.

The compiler MUST invalidate diagnostics when their semantic dependencies change.

---

74. Parallel Diagnostics

Compiler phases MAY execute in parallel.

Parallelism MUST NOT affect:

- diagnostic identity;
- severity;
- source span;
- ordering;
- message content;
- deduplication.

A compiler running on 1 CPU and a compiler running on many CPUs MUST produce semantically equivalent diagnostics.

---

75. Distributed Diagnostics

Distributed compilation MAY collect diagnostics from multiple workers.

The final aggregation MUST be deterministic.

Worker identity MUST NOT accidentally become diagnostic identity.

Diagnostics SHOULD be ordered using source/phase/code information rather than worker completion time.

---

76. Source Version in Diagnostics

Every diagnostic SHOULD be associated with the language version used to interpret the source.

This is important for:

- deprecated syntax;
- removed syntax;
- reserved keywords;
- feature gates;
- semantic changes;
- compatibility migrations.

---

77. Dialect Diagnostics

Dialect diagnostics include:

ZMN-DIALECT-UNKNOWN
ZMN-DIALECT-VERSION
ZMN-DIALECT-INCOMPATIBLE
ZMN-DIALECT-FEATURE
ZMN-DIALECT-SEMANTIC

A dialect MUST declare:

- identity;
- version;
- syntax extensions;
- semantic extensions;
- AST mapping;
- IR mapping;
- compatibility rules.

An undeclared dialect extension MUST NOT silently change core Zamani semantics.

---

78. Macro Diagnostics

Macro diagnostics include:

ZMN-MACRO-INVALID
ZMN-MACRO-EXPANSION
ZMN-MACRO-HYGIENE
ZMN-MACRO-RECURSION
ZMN-MACRO-CAPABILITY
ZMN-MACRO-UNSUPPORTED

Macro-generated errors SHOULD point to the originating invocation when possible.

If an error originates inside macro definition code, the definition location SHOULD be included as related information.

---

79. Metaprogramming Diagnostics

Metaprogramming diagnostics include:

ZMN-META-INVALID
ZMN-META-EVALUATION
ZMN-META-TYPE
ZMN-META-CAPABILITY
ZMN-META-GENERATION
ZMN-META-RECURSION

Compile-time metaprogramming MUST NOT bypass semantic validation.

---

80. Diagnostics and Canonical AST

Every user-visible diagnostic generated before or during AST construction SHOULD retain enough information to map to the target-independent frontend AST.

The AST MUST preserve source spans for:

- declarations;
- statements;
- expressions;
- types;
- attributes;
- modifiers;
- quantum operations;
- HDL constructs;
- resource declarations;
- capability requirements.

The diagnostic system MUST NOT depend on parser stack state after AST construction.

---

81. Diagnostics and Semantic Model

Semantic diagnostics MUST operate on semantic facts rather than parser-specific token patterns whenever possible.

For example:

type mismatch

belongs to type analysis.

It should not be reported as:

unexpected token

merely because the parser happened to encounter the type conflict.

---

82. Diagnostics and Canonical IR

Every source-level construct that survives semantic analysis MUST either:

1. lower correctly into the appropriate canonical IR; or
2. generate an explicit lowering diagnostic.

No semantic information may disappear silently.

For quantum constructs:

source
 ↓
AST
 ↓
semantic quantum operation
 ↓
quantum::ir

A failure must be reported at the appropriate boundary.

---

83. Diagnostic Preservation Through Lowering

When an optimization or lowering pass transforms a construct, diagnostics SHOULD preserve source provenance.

For example:

source operation
 ↓
canonical IR operation
 ↓
optimized operation
 ↓
target operation

A target failure SHOULD still be traceable to the originating source operation.

---

84. Diagnostic Provenance

Every diagnostic SHOULD identify its origin.

Conceptually:

origin {
    phase
    subsystem
    source_context
}

The origin MUST be stable enough for debugging and tooling.

---

85. Serialization

Diagnostics SHOULD support a structured serialization format suitable for:

- CLI tools;
- CI;
- editors;
- LSP;
- build systems;
- automated conformance testing;
- machine-readable compiler APIs.

The serialized form SHOULD contain:

code
severity
phase
message
primary span
secondary spans
notes
help
suggestions
source identity
related information
structured data

Human-readable output is a presentation layer over the same diagnostic model.

---

86. CLI Rendering

Human-readable CLI output SHOULD use:

error[ZMN-TYPE-MISMATCH]: ...
  --> file.zm:line:column
   |
...

The exact visual formatting MAY evolve.

The machine-readable code MUST remain stable.

---

87. JSON/Structured Rendering

Structured output MUST NOT require consumers to parse human-readable prose.

Example conceptual representation:

{
  "code": "ZMN-TYPE-MISMATCH",
  "severity": "error",
  "phase": "type",
  "message": "argument has type `Float`, expected `Int`",
  "primary_span": {
    "file": "example.zm",
    "start": 120,
    "end": 125
  }
}

The exact serialization schema belongs to the tooling/API implementation, but these semantic fields MUST remain representable.

---

88. LSP/IDE Integration

Language-server diagnostics SHOULD map directly from the canonical diagnostic model.

The LSP layer MUST NOT invent a second diagnostic taxonomy.

The mapping is:

Zamani Diagnostic
        ↓
LSP Diagnostic

not:

compiler diagnostic
        ↓
independently reconstructed IDE diagnostic

Fix-its SHOULD map to code actions.

Notes and related locations SHOULD map to appropriate IDE information.

---

89. Diagnostic Privacy

Diagnostics MUST be safe to emit in normal build logs.

They MUST NOT accidentally expose:

- passwords;
- secrets;
- private keys;
- tokens;
- confidential credentials;
- private environment variables;
- hidden source files;
- unrelated user data.

When external systems provide sensitive values, diagnostics SHOULD redact them.

---

90. Path Handling

Diagnostics SHOULD avoid embedding host-specific absolute paths when reproducibility is required.

Where possible, diagnostics SHOULD use stable source identifiers or workspace-relative paths.

The same source should not produce semantically different diagnostics merely because it was compiled from:

/home/user/project

versus:

/build/agent/project

---

91. Environment Independence

Diagnostics MUST NOT depend on:

- locale;
- timezone;
- operating-system message catalogs;
- terminal width for semantic content;
- CPU architecture;
- available accelerators;
- network timing.

Presentation MAY adapt to terminal capabilities, but the underlying diagnostic must remain identical.

---

92. Deterministic Numeric Formatting

Diagnostic formatting of quantities MUST be deterministic.

Large values MUST NOT overflow merely because they are being formatted.

Resource quantities SHOULD retain their semantic representation where necessary.

For example:

requested: 999999999999999999999999

must not silently wrap into a smaller host integer.

---

93. Diagnostics for Arbitrarily Large Programs

The diagnostic model MUST be conceptually scalable from:

one token
one statement
one file

to:

large modules
large projects
large distributed programs
large quantum programs
large HDL designs
large data/tensor programs

The language itself MUST NOT define a finite universal diagnostic count or source-size semantic ceiling.

Implementations MAY enforce operational budgets.

---

94. Diagnostic Resource Limits

Implementation limits MUST use distinct diagnostic codes.

Examples:

ZMN-RESOURCE-SOURCE-BUDGET
ZMN-RESOURCE-MEMORY-BUDGET
ZMN-RESOURCE-TIME-BUDGET
ZMN-RESOURCE-DIAGNOSTIC-BUDGET
ZMN-RESOURCE-ANALYSIS-BUDGET

These mean:

implementation cannot continue under the current budget

not:

program is invalid Zamani

---

95. Compatibility With "grammar/spec/lexical.md"

The lexical specification owns lexical behavior.

Diagnostics MUST therefore preserve its distinctions between:

- invalid UTF-8;
- invalid characters;
- identifiers;
- keywords;
- literals;
- operators;
- comments;
- source spans.

Lexical errors MUST originate in the lexical layer whenever the failure is genuinely lexical.

The diagnostics layer supplies their standardized identity and presentation.

---

96. Compatibility With "grammar/spec/syntax.md"

The syntax specification owns syntactic validity.

Parser diagnostics MUST correspond to syntax rules defined there.

The diagnostic system MUST NOT add grammar rules through error messages.

For example:

expected expression

does not itself define what an expression is.

---

97. Compatibility With "grammar/spec/semantics.md"

Semantic diagnostics MUST use the semantic distinctions established by the semantic specification.

In particular:

semantic invalidity

must remain distinct from:

resource unsatisfiability
target incompatibility
implementation failure

This distinction is required for POCO-REAF.

---

98. Compatibility With "grammar/spec/compatibility.md"

The compatibility specification owns:

- stable features;
- experimental features;
- deprecated features;
- removed features;
- reserved syntax;
- language versions;
- migration policy.

Diagnostics provide the concrete reporting mechanism.

For example:

ZMN-COMPAT-DEPRECATED

reports a deprecated feature.

It does not define when the feature became deprecated.

---

99. Compatibility With "grammar/grammar.md"

"grammar/grammar.md" SHOULD record diagnostic support for every implemented feature.

For each feature:

syntax
lexer
parser
AST
semantic
IR
diagnostics
tests
status

A feature MUST NOT be marked complete if it has no defined diagnostics for invalid forms where invalid forms are possible.

---

100. Compatibility With "grammar/Zamani-Grammar.md"

"Zamani-Grammar.md" may contain proposed or aspirational constructs.

Diagnostics for such constructs MUST NOT be treated as stable until the feature is promoted.

A proposed construct may therefore have:

SPECIFIED
EXPERIMENTAL
RESERVED

status without implying stable compiler support.

---

101. Compatibility With "grammar/Zamani.g4"

"Zamani.g4" is the ANTLR grammar composition root.

ANTLR parser errors MUST map into the canonical diagnostic model where ANTLR is used for tooling or conformance.

ANTLR-specific wording MUST NOT become the Zamani diagnostic contract.

---

102. Compatibility With "src/lexer.rs"

"src/lexer.rs" owns executable lexical scanning.

It MUST produce or map to the canonical diagnostic model.

It MUST preserve:

- source file identity;
- byte spans;
- token context;
- lexical error identity.

The lexer MUST NOT silently swallow malformed input.

---

103. Compatibility With "src/parser.rs"

"src/parser.rs" owns executable syntax recognition.

It MUST map parser failures into structured diagnostics.

The parser MUST preserve enough source information for:

- primary spans;
- expected tokens;
- actual tokens;
- recovery context.

---

104. Compatibility With "src/frontend/ast/"

The frontend AST MUST preserve source spans for constructs that can generate later diagnostics.

AST construction MUST NOT discard source information required by:

- semantic errors;
- type errors;
- quantum errors;
- resource errors;
- IR errors;
- target errors.

---

105. Compatibility With Quantum Frontend

Quantum frontend formats such as OpenQASM MUST translate their errors into the same diagnostic system.

An OpenQASM parsing failure MUST NOT create a separate incompatible diagnostic taxonomy.

The relationship is:

OpenQASM source
 ↓
OpenQASM frontend
 ↓
Zamani frontend semantic representation
 ↓
quantum::ir

Diagnostics MUST preserve the original source location where possible.

---

106. Compatibility With "quantum::ir"

"quantum::ir" remains the canonical quantum semantic boundary.

The diagnostic system MUST NOT create another competing quantum semantic representation.

Diagnostics around quantum IR MUST refer to the canonical IR operation or source construct.

---

107. Compatibility With QEC

QEC owns QEC semantics.

Diagnostics MUST identify QEC conditions without embedding QEC implementation into grammar.

---

108. Compatibility With Routing

Routing owns physical realization.

Routing failures MUST be reported as routing/target/resource conditions unless the source itself contains an invalid routing-specific semantic construct.

---

109. Compatibility With Scheduling

Scheduling owns execution order and timing realization.

Scheduling diagnostics MUST NOT redefine source-level control-flow semantics.

---

110. Compatibility With ZQN

ZQN owns fault/noise semantics.

ZQN diagnostics MUST remain distinct from:

- parser errors;
- syntax errors;
- generic quantum syntax errors.

---

111. Compatibility With HAL

HAL owns actual device capabilities and state.

HAL diagnostics MAY include dynamic target state, but MUST distinguish:

language validity

from:

current device availability

---

112. Diagnostics and POCO-REAF

The diagnostic system is a critical part of POCO-REAF.

It MUST preserve the distinction:

Program valid
       +
Target unsuitable

rather than incorrectly reporting:

Program invalid

This allows the same source program to be compiled or executed against:

- CPUs;
- GPUs;
- FPGAs;
- QPUs;
- simulators;
- distributed systems;
- embedded systems;
- future computational substrates.

A target failure MUST NOT force source rewriting unless the semantic requirements genuinely cannot be satisfied.

---

113. Diagnostic Categories for Portability

The compiler SHOULD distinguish:

ZMN-SEMANTIC-INVALID
ZMN-CAPABILITY-MISSING
ZMN-RESOURCE-UNSATISFIED
ZMN-TARGET-UNSUPPORTED
ZMN-ROUTE-UNAVAILABLE
ZMN-SCHEDULE-UNSATISFIED
ZMN-HAL-UNAVAILABLE

These represent different layers of failure.

---

114. No Hardware-Specific Error Semantics in Core Grammar

The core grammar MUST NOT need diagnostic codes for every physical device.

For example, it must not require:

ZMN-QPU-IBM-X
ZMN-GPU-NVIDIA-Y
ZMN-FPGA-VENDOR-Z

as core language diagnostics.

Vendor/device details belong in target/HAL diagnostics and structured target metadata.

---

115. Diagnostic Testing Contract

Every diagnostic MUST have tests appropriate to its phase.

At minimum:

positive behavior
negative behavior
source-span behavior
determinism
serialization
compatibility

Where relevant:

boundary
scalability
resource exhaustion
recovery
cross-file
cross-domain
target mismatch

---

116. Negative Tests

Negative tests MUST verify:

- diagnostic code;
- severity;
- primary source location;
- relevant structured data;
- expected recovery behavior.

Tests SHOULD NOT depend unnecessarily on the exact human-readable wording.

---

117. Diagnostic Golden Tests

Golden tests MAY verify rendered output.

However, stable tests SHOULD prefer structured diagnostic assertions.

For example:

code == ZMN-TYPE-MISMATCH
severity == error
primary_span == ...

is more stable than requiring every punctuation mark in the human-readable message to remain identical.

---

118. Determinism Tests

The repository MUST include tests proving that the same source produces equivalent diagnostics under:

- different thread counts;
- different hash-map iteration order;
- repeated compilation;
- different target enumeration order;
- different worker scheduling.

The diagnostic set and ordering MUST remain deterministic.

---

119. Scalability Tests

Diagnostics MUST be tested against increasingly large inputs.

Tests SHOULD cover:

tiny program
small program
large program
very large program
large diagnostic set
large resource quantities
large module graph
large quantum operation graph
large HDL design
large distributed topology

The tests MUST distinguish:

implementation resource exhaustion

from:

language invalidity

---

120. Boundary Tests

Boundary tests MUST cover:

- empty source;
- one token;
- one-character identifiers;
- maximum implementation-supported line/column values;
- Unicode boundaries;
- malformed UTF-8;
- nested constructs;
- large numeric literals;
- large source spans;
- zero-length spans;
- multi-file diagnostics;
- generated-source mappings.

Any implementation-specific finite boundary MUST be documented.

---

121. Compatibility Tests

Compatibility tests MUST verify:

- stable diagnostic codes remain stable;
- deprecated features produce the correct diagnostic;
- removed features are not silently reinterpreted;
- reserved syntax remains reserved;
- language-version differences are deterministic.

---

122. Cross-Domain Tests

At least one diagnostic integration test SHOULD exist for each major domain:

classical
quantum
hybrid
HDL
hardware
distributed
AI
data
networking
security
memory
concurrency
effects
interop
macros
metaprogramming
dialects

---

123. Quantum Diagnostic Tests

Quantum diagnostics MUST cover:

- invalid operation arity;
- invalid parameters;
- invalid controls;
- invalid measurement;
- invalid state usage;
- invalid classical feed-forward;
- unsupported target capability;
- insufficient quantum resources;
- QEC requirement failure;
- routing failure;
- scheduling failure;
- ZQN constraint failure;
- HAL/device availability failure.

These cases MUST remain distinguishable.

---

124. HDL Diagnostic Tests

HDL diagnostics MUST cover:

- port mismatch;
- signal mismatch;
- width mismatch;
- invalid clocking;
- invalid reset;
- timing constraint failure;
- state-machine error;
- synthesis limitation;
- target realization limitation.

---

125. Resource Diagnostic Tests

Resource tests MUST distinguish:

resource requirement valid

from:

resource requirement unsatisfied

and:

compiler resource budget exhausted

These MUST never collapse into one generic error.

---

126. Diagnostic Conformance Matrix

Every language feature SHOULD be traceable through:

Feature
 ├── specification
 ├── lexical rules
 ├── syntax
 ├── AST
 ├── semantic rules
 ├── diagnostics
 ├── IR mapping
 ├── compiler integration
 ├── runtime integration
 └── tests

A production feature is incomplete if its invalid states cannot be diagnosed appropriately.

---

127. Feature Manifest Integration

Where feature manifests are introduced under:

grammar/specification/features/

each feature manifest SHOULD identify:

diagnostics:
  - code
  - severity
  - phase
  - condition
  - tests

This creates a closed feature contract.

Example:

quantum-operations
    ↓
grammar
    ↓
AST
    ↓
semantic model
    ↓
quantum::ir
    ↓
diagnostics
    ↓
tests

No later file should need to invent missing diagnostic semantics.

---

128. Completion Contract for This File

"grammar/spec/diagnostics.md" is complete only when:

- diagnostic ownership is defined;
- diagnostic authority is defined;
- severity is defined;
- diagnostic codes are defined;
- domains are defined;
- phases are defined;
- source spans are defined;
- primary labels are defined;
- secondary labels are defined;
- notes are defined;
- help is defined;
- suggestions are defined;
- fix-its are defined;
- recovery is defined;
- deterministic ordering is defined;
- deduplication is defined;
- resource diagnostics are defined;
- capability diagnostics are defined;
- target diagnostics are defined;
- runtime diagnostics are defined;
- quantum diagnostics are defined;
- QEC integration is defined;
- ZQN integration is defined;
- routing integration is defined;
- scheduling integration is defined;
- HAL integration is defined;
- HDL diagnostics are defined;
- distributed diagnostics are defined;
- AI/data diagnostics are defined;
- networking diagnostics are defined;
- security diagnostics are defined;
- interoperability diagnostics are defined;
- dialect diagnostics are defined;
- macro diagnostics are defined;
- metaprogramming diagnostics are defined;
- compatibility diagnostics are defined;
- safe-Rust requirements are defined;
- privacy requirements are defined;
- serialization is defined;
- CLI integration is defined;
- IDE/LSP integration is defined;
- scalability requirements are defined;
- diagnostic resource budgets are separated from language limits;
- deterministic behavior is defined;
- negative tests are defined;
- boundary tests are defined;
- scalability tests are defined;
- compatibility tests are defined;
- cross-domain tests are defined;
- AST integration is defined;
- semantic integration is defined;
- IR integration is defined;
- compiler integration is defined;
- runtime integration is defined.

---

129. Final Diagnostic Architecture

The complete production diagnostic architecture is:

                         Zamani Source
                              │
                              ▼
                       Source decoding
                              │
                    ┌─────────┴─────────┐
                    ▼                   ▼
                 success              error
                    │                   │
                    │          ZMN-LEX-...
                    ▼
                   Lexer
                    │
                    ▼
                 Parser
                    │
          ┌─────────┴─────────┐
          ▼                   ▼
       success              error
          │                   │
          │            ZMN-PARSE-...
          ▼
       Frontend AST
          │
          ▼
   Structural validation
          │
          ▼
   Name/module resolution
          │
          ▼
      Type analysis
          │
          ▼
     Effect analysis
          │
          ▼
 Ownership/resource/capability
          │
          ▼
   Domain semantic analysis
          │
      ┌───┼──────────────────────┐
      ▼   ▼                      ▼
 Classical Quantum               HDL
          │
          ▼
   Canonical semantic model
          │
          ├───────────────┐
          ▼               ▼
    Classical IR      quantum::ir
          │               │
          └───────┬───────┘
                  ▼
            IR verification
                  │
                  ▼
             Optimization
                  │
       ┌──────────┼───────────┐
       ▼          ▼           ▼
    Routing   Scheduling   Resilience
       │          │           │
       └──────────┼───────────┘
                  ▼
                 QEC
                  │
                  ▼
                 ZQN
                  │
                  ▼
                 HAL
                  │
                  ▼
          Target realization
                  │
                  ▼
              Runtime

At every boundary:

valid
  │
  ├── continue
  │
  └── structured diagnostic

There MUST be no silent semantic loss.

There MUST be no silent target substitution.

There MUST be no silent resource truncation.

There MUST be no silent quantum-operation deletion.

There MUST be no silent hardware-limit reinterpretation.

There MUST be no diagnostic taxonomy that competes with the language's semantic architecture.

---

130. Production Invariants

The following invariants are mandatory.

Invariant 1 — One diagnostic model

All compiler phases use one canonical diagnostic model.

Invariant 2 — Stable codes

Tooling consumes stable diagnostic codes rather than parsing prose.

Invariant 3 — Source provenance

Diagnostics retain source provenance whenever source exists.

Invariant 4 — Determinism

Equivalent compilation conditions produce deterministic diagnostics.

Invariant 5 — No silent loss

Unsupported constructs produce explicit diagnostics.

Invariant 6 — No false invalidity

Target/resource limitations are not reported as source invalidity unless the source itself violates a semantic requirement.

Invariant 7 — No artificial scale limit

The diagnostic specification does not impose universal limits on program or resource magnitude.

Invariant 8 — Explicit implementation budgets

Compiler resource budgets are distinguished from language limits.

Invariant 9 — Safe Rust

Production compiler implementation uses Rust 1.97/1.97.1-compatible safe Rust only.

Invariant 10 — Quantum boundary

Quantum diagnostics integrate with "quantum::ir"; they do not create a competing quantum IR.

Invariant 11 — Target independence

Core diagnostics do not encode individual physical devices as language constructs.

Invariant 12 — Compatibility

Diagnostic behavior is version-aware and migration-aware.

Invariant 13 — Tooling interoperability

CLI, CI, IDE, LSP, and machine-readable consumers use the same underlying diagnostic information.

Invariant 14 — Privacy

Diagnostics do not leak secrets or unrelated private data.

Invariant 15 — Recoverability

Error recovery is explicitly distinguished from successful compilation.

---

131. Final Integration Table

File / subsystem| Diagnostic responsibility
"grammar/spec/diagnostics.md"| Normative diagnostic contract
"grammar/spec/lexical.md"| Lexical conditions being diagnosed
"grammar/spec/syntax.md"| Syntax conditions being diagnosed
"grammar/spec/semantics.md"| Semantic conditions being diagnosed
"grammar/spec/compatibility.md"| Version/deprecation compatibility
"grammar/DESIGN.md"| Architectural diagnostic integration
"grammar/Zamani.g4"| Syntax source for parser diagnostics
"grammar/grammar.md"| Implementation-conformance documentation
"grammar/Zamani-Grammar.md"| Historical/proposed diagnostic material
"src/lexer.rs"| Executable lexical diagnostics
"src/parser.rs"| Executable syntax diagnostics
"src/frontend/ast/"| Source-preserving structural representation
semantic analysis| Name/type/effect/resource/capability diagnostics
"quantum::ir"| Canonical quantum semantic diagnostic boundary
QEC| QEC diagnostics
ZQN| fault/noise diagnostics
routing| physical realization diagnostics
scheduling| timing/resource scheduling diagnostics
HAL| target capability/state diagnostics
IR verifier| IR invariant diagnostics
compiler| lowering/optimization/target diagnostics
runtime| execution diagnostics
"grammar/validation/"| automated diagnostic conformance
"grammar/tests/"| diagnostic positive/negative/boundary/scalability tests
CLI| human-readable rendering
tooling/LSP| structured editor diagnostics

---

132. Final Acceptance Criteria

This specification is production-ready when the repository can demonstrate the following:

source
  ↓
diagnostic-capable lexer
  ↓
diagnostic-capable parser
  ↓
source-preserving AST
  ↓
diagnostic-capable semantic analysis
  ↓
resource/capability diagnostics
  ↓
domain diagnostics
  ↓
canonical IR diagnostics
  ↓
IR verification diagnostics
  ↓
optimization/lowering diagnostics
  ↓
routing/scheduling/QEC/ZQN/HAL diagnostics
  ↓
target/runtime diagnostics

and when:

same source
+
same language version
+
same semantic configuration
+
same target constraints

produces deterministic structured diagnostics regardless of:

CPU count
thread count
GPU availability
QPU availability
filesystem ordering
hash iteration order
worker completion order
network timing

while still allowing the implementation to scale from tiny programs to arbitrarily large computations subject only to actual semantic requirements and available compiler/runtime/target resources.

The essential rule is:

«A diagnostic reports a failure of a specific contract; it must never become the mechanism by which one layer silently changes the meaning owned by another layer.»

Therefore the complete production boundary is:

Language specification
        ↓
Lexical specification
        ↓
Syntax specification
        ↓
Diagnostics specification
        ↓
Zamani.g4
        ↓
Rust lexer
        ↓
Rust parser
        ↓
Frontend AST
        ↓
Semantic model
        ↓
Classical / quantum / HDL / hybrid / other domain semantics
        ↓
Canonical IR
        ↓
Verification
        ↓
Optimization
        ↓
Routing / scheduling / QEC / resilience / ZQN
        ↓
HAL
        ↓
Target realization
        ↓
Runtime

with diagnostics available at every boundary, without creating a second language, second semantic model, second quantum IR, or artificial scalability ceiling.