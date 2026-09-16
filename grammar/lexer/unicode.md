Zamani Unicode Specification

Path: "grammar/lexer/unicode.md"
Status: Normative / Production Architecture
Specification Version: 1.0
Language: Zamani
Implementation Baseline: Rust 1.97 / Rust 1.97.1
Rust Edition: 2021
Safety: Safe Rust only; "unsafe" is prohibited
Source Encoding: UTF-8
Primary Grammar Authority: "grammar/Zamani.g4"
Lexical Authority: "grammar/lexer/"
Identifier Contract: "grammar/lexer/identifiers.md"
Literal Contract: "grammar/lexer/literals.md"
String Contract: "grammar/lexer/string-literals.g4"
Token Contract: "grammar/lexer/tokens.md"
Lexical Specification: "grammar/specification/lexical.md"
Formal Lexical Contract: "grammar/spec/lexical.md"
Source-Span Contract: "grammar/spec/source-spans.md"
Implementation Reference: "src/lexer.rs"
Parser Consumer: "src/parser.rs"
AST Consumer: "src/frontend/ast/" and the repository's canonical AST implementation
Canonical Quantum Semantic Boundary: "quantum::ir"

---

0. Purpose

This document defines the normative Unicode contract for Zamani.

It establishes how Unicode is handled throughout the lexical boundary without making Unicode behavior dependent on:

- CPU architecture;
- operating system;
- host locale;
- machine word size;
- compiler backend;
- GPU;
- FPGA;
- QPU;
- physical qubit;
- hardware topology;
- memory capacity;
- runtime implementation;
- deployment environment.

Unicode handling is part of the portable source-language layer.

The fundamental pipeline is:

source bytes
    │
    ▼
UTF-8 validation
    │
    ▼
Unicode scalar-value stream
    │
    ├── identifiers
    ├── keywords
    ├── literals
    ├── operators
    ├── punctuation
    └── comments
    │
    ▼
tokens + source spans
    │
    ▼
parser
    │
    ▼
domain-neutral AST
    │
    ▼
semantic analysis
    │
    ▼
canonical semantic model / IR
    │
    ├── classical IR
    ├── quantum::ir
    └── HDL / hardware / domain IR

Unicode processing MUST stop being a hidden source-rewriting mechanism.

The lexer MUST recognize source.

It MUST NOT silently rewrite source.

---

1. Architectural Position

Unicode is a foundational lexical concern.

It therefore sits below:

- identifiers;
- keywords;
- literals;
- comments;
- expressions;
- declarations;
- quantum syntax;
- HDL syntax;
- semantic analysis;
- AST construction.

The dependency direction is:

grammar/DESIGN.md
        │
        ▼
grammar/specification/lexical.md
        │
        ▼
grammar/spec/lexical.md
        │
        ▼
grammar/lexer/unicode.md
        │
        ├───────────────┬────────────────┐
        ▼               ▼                ▼
identifiers.md    literals.md      comments.md
        │               │                │
        └───────────────┼────────────────┘
                        ▼
                 lexer / tokens
                        │
                        ▼
                     parser
                        │
                        ▼
                 domain-neutral AST

Unicode MUST NOT create a second semantic pipeline.

---

2. Scope

This specification owns:

1. source encoding;
2. UTF-8 validity;
3. Unicode scalar-value handling;
4. Unicode code-point terminology;
5. Unicode identifier classification;
6. Unicode identifier stability;
7. normalization policy;
8. case handling;
9. combining marks;
10. format characters;
11. Unicode whitespace;
12. Unicode line boundaries;
13. Unicode escapes at the lexical boundary;
14. Unicode literal validation boundaries;
15. Unicode source-span requirements;
16. Unicode diagnostics;
17. Unicode security considerations;
18. Unicode versioning;
19. Unicode reproducibility;
20. Unicode conformance testing;
21. Unicode scalability requirements;
22. Unicode integration with the Rust lexer;
23. Unicode integration with ANTLR;
24. Unicode integration with AST/source spans;
25. Unicode compatibility requirements.

This specification does not own:

- symbol resolution;
- type checking;
- name lookup;
- string allocation;
- string interning;
- string encoding at runtime;
- Unicode normalization of runtime data;
- localization;
- collation;
- sorting;
- case-insensitive application behavior;
- Unicode rendering;
- terminal display;
- font selection;
- grapheme segmentation as a semantic operation;
- quantum semantics;
- QEC;
- ZQN;
- scheduling;
- routing;
- HAL;
- backend-specific encoding.

Those belong to downstream contracts.

---

3. Normative Terms

The following terms are normative:

- MUST — mandatory.
- MUST NOT — prohibited.
- SHOULD — recommended unless a documented compatibility reason exists.
- SHOULD NOT — discouraged unless a documented compatibility reason exists.
- MAY — permitted.
- LANGUAGE RULE — a rule that determines whether source is valid Zamani.
- IMPLEMENTATION LIMIT — a limit imposed by a compiler, tool, runtime, operating system, or resource budget.
- SOURCE SPELLING — the exact Unicode source representation supplied by the programmer.
- SCALAR VALUE — a Unicode scalar value, excluding UTF-16 surrogate code points.
- GRAPHEME CLUSTER — a user-perceived character sequence; it is not equivalent to a Unicode scalar value.
- CODE POINT — a Unicode code point; not every code point is a Unicode scalar value.

---

4. Fundamental Unicode Model

Zamani source is UTF-8.

The canonical representation is:

UTF-8 bytes
    ↓
Unicode scalar values
    ↓
lexical analysis

The compiler MUST NOT interpret source using:

- ASCII as the complete character model;
- Latin-1;
- Windows-1252;
- locale-specific encodings;
- host-dependent encodings;
- terminal encodings;
- platform-default encodings.

ASCII remains a valid subset of Zamani source.

Unicode is not an optional extension.

---

5. UTF-8 Is the Source Encoding

All conforming Zamani source input MUST be interpreted as UTF-8.

A source-input API accepting raw bytes MUST validate UTF-8 before ordinary lexical processing.

Invalid UTF-8 MUST produce a source-encoding diagnostic.

Invalid UTF-8 MUST NOT be:

- silently replaced with U+FFFD;
- discarded;
- transliterated;
- interpreted using another encoding;
- converted according to the host locale;
- treated as arbitrary identifier data.

For example, invalid UTF-8 bytes MUST NOT become a valid identifier merely because the host operating system can decode them using another character encoding.

---

6. Unicode Scalar Values

After successful UTF-8 decoding, lexical processing operates on Unicode scalar values.

Unicode scalar values are:

U+0000 .. U+D7FF
U+E000 .. U+10FFFF

The surrogate range:

U+D800 .. U+DFFF

is not a Unicode scalar-value range.

The lexer and literal semantic layer MUST preserve this distinction.

A Unicode escape that denotes a surrogate MUST NOT silently become a valid scalar.

---

7. Code Point vs Scalar Value

Zamani tooling MUST distinguish:

Unicode code point

from:

Unicode scalar value

This distinction is mandatory for:

- "\u{...}" escapes;
- character literals;
- Unicode diagnostics;
- source processing;
- AST literal validation.

A code point value outside the Unicode scalar-value range MUST NOT be represented as a valid Unicode scalar.

---

8. No UTF-16 Code-Unit Model

The core Zamani language MUST NOT define characters as UTF-16 code units.

In particular:

char

MUST NOT implicitly mean:

16-bit UTF-16 code unit

A character semantic value MUST represent a Unicode scalar value unless a separate byte/code-unit type explicitly defines another representation.

This avoids target-dependent character semantics.

---

9. Unicode Version

Unicode classification is version-sensitive.

Therefore the Zamani language specification MUST identify the normative Unicode data/version associated with each Zamani language version.

The implementation MUST NOT silently select Unicode behavior based on:

- operating system Unicode tables;
- host library version;
- terminal;
- locale;
- editor;
- filesystem;
- platform.

The Unicode version used for lexical classification SHOULD be available through compiler/tooling metadata.

For example:

Zamani language version
Unicode data version
lexer implementation version

SHOULD be independently identifiable.

---

10. Unicode Version Changes

A Unicode-data upgrade MUST be treated as a compatibility-sensitive change.

When Unicode character properties change:

1. the language specification MUST document the change;
2. identifier conformance tests MUST be updated;
3. lexical conformance tests MUST be updated;
4. compatibility implications MUST be recorded;
5. the compiler MUST NOT silently reinterpret existing source under an undocumented policy.

A Unicode upgrade MUST NOT accidentally create a second language version hidden inside the compiler.

---

11. Deterministic Unicode Classification

Unicode classification MUST be deterministic.

The same:

source bytes
+
Zamani language version
+
Unicode specification version
+
lexical configuration

MUST produce the same lexical classification.

The result MUST NOT depend on:

host OS
host locale
terminal
editor
filesystem
CPU
GPU
QPU

---

12. Identifier Integration

The identifier contract already establishes Unicode-aware identifiers and recommends Unicode identifier properties equivalent to "XID_Start" and "XID_Continue".

This document therefore does not redefine identifier syntax.

Instead:

unicode.md
      │
      ▼
identifiers.md
      │
      ▼
identifier token

"identifiers.md" owns:

- identifier start;
- identifier continuation;
- identifier spelling;
- identifier comparison;
- keyword separation;
- identifier-specific normalization policy.

"unicode.md" owns the underlying Unicode model.

No domain grammar may define its own Unicode identifier rules.

---

13. Identifier Classification

The implementation MUST NOT use:

[A-Za-z_][A-Za-z0-9_]*

as the complete Zamani identifier definition.

That is only an ASCII subset.

Unicode-aware identifier classification MUST be used.

The implementation MUST NOT create a finite list such as:

Latin
Greek
Cyrillic
Arabic
Chinese
...

as the complete set of supported scripts.

Unicode support must remain extensible through the normative Unicode property model.

---

14. Combining Marks

Combining marks MUST be handled according to the identifier property contract.

A combining mark MUST NOT automatically become an identifier-start character.

A combining mark MAY participate in an identifier continuation when permitted by the normative Unicode identifier properties.

This prevents malformed or visually misleading identifiers while retaining legitimate Unicode identifiers.

---

15. Unicode Normalization

Zamani MUST preserve source spelling.

The lexer MUST NOT silently perform:

- NFC normalization;
- NFD normalization;
- NFKC normalization;
- NFKD normalization;
- locale-specific normalization;
- transliteration.

For example, visually similar source representations MUST remain distinguishable unless the language version explicitly defines their semantic equivalence.

The source:

é

MUST NOT silently become:

e + combining acute accent

and vice versa.

---

16. Source Identity vs Semantic Identity

The following are separate concepts:

source identity
semantic identifier identity

Source identity means:

«exactly what Unicode source spelling the programmer supplied.»

Semantic identifier identity means:

«whether two identifiers denote the same language entity.»

The lexer MUST preserve source identity.

The identifier specification determines semantic identifier comparison.

This distinction is required for:

- diagnostics;
- source maps;
- formatting;
- refactoring;
- reproducible builds;
- provenance;
- IDE tooling.

---

17. No Hidden Normalization

The compiler MUST NOT perform:

source
→ normalize
→ silently lex normalized source

Instead:

source
→ preserve source
→ lex source
→ explicitly apply any specified semantic comparison policy

If a future Zamani language version introduces normalized identifier equality, that must be an explicit versioned language rule.

---

18. Case Sensitivity

Zamani identifiers are case-sensitive.

Unicode case folding MUST NOT be performed implicitly.

The following remain distinct:

value
Value
VALUE

The same principle applies to Unicode identifiers.

The lexer MUST preserve the original case.

Keyword matching MUST follow the canonical keyword contract.

---

19. Unicode Keywords

Unicode MUST NOT cause arbitrary keyword aliases to appear.

For example, an implementation MUST NOT silently treat:

function

as equivalent to a translated or Unicode-script representation of the same word.

Keywords are lexical language syntax.

They are owned by:

grammar/lexer/keywords.md

and the canonical grammar.

Unicode provides the character model; it does not invent keyword aliases.

---

20. Built-in Names vs Keywords

Unicode processing MUST preserve the distinction between:

keyword
built-in name
user identifier

A semantic type such as:

Qubit

does not need to become a special lexer token merely because it is built in.

This keeps Unicode processing independent of domain semantics.

---

21. Unicode Whitespace

Unicode whitespace MUST be explicitly classified.

The lexer MUST distinguish:

- language whitespace;
- line terminators;
- comments;
- significant punctuation;
- characters that merely resemble whitespace.

The implementation MUST NOT use the host locale's whitespace definition.

---

22. ASCII Space

U+0020 SPACE is ordinary horizontal whitespace.

It MUST NOT be semantically significant unless a separate language feature explicitly gives whitespace semantic meaning.

---

23. Tab

U+0009 CHARACTER TABULATION is whitespace.

Tabs MUST NOT alter source-byte offsets.

Tabs MUST NOT alter lexical identity.

Display width is a tooling/rendering concern.

A compiler MUST NOT interpret:

tab = 4 spaces

or:

tab = 8 spaces

as part of source semantics unless a future language version explicitly introduces indentation semantics.

---

24. Non-Breaking Space

U+00A0 NO-BREAK SPACE MUST NOT silently become ordinary source whitespace unless explicitly specified.

This is important because visually:

a b

and:

a<NBSP>b

may look similar while containing different code points.

The lexer SHOULD diagnose unexpected Unicode whitespace when it occurs where ordinary ASCII whitespace was expected.

It MUST NOT silently rewrite it.

---

25. Other Unicode Space Characters

Characters classified as Unicode space separators MUST be handled according to the normative whitespace specification.

They MUST NOT automatically be treated as ordinary ASCII spaces merely because they render as spaces.

If Zamani chooses to accept additional Unicode whitespace, the accepted set MUST be:

- explicitly specified;
- deterministic;
- versioned;
- tested.

---

26. Zero-Width Characters

Invisible Unicode characters require special treatment.

The lexer MUST NOT automatically accept arbitrary zero-width or format characters inside identifiers.

Characters such as:

- zero-width space;
- zero-width non-joiner;
- zero-width joiner;
- word joiner;
- bidi controls;

must be handled according to explicit Unicode and security policy.

Invisible characters MUST NOT silently create two visually identical but semantically unrelated names without tooling being able to identify the difference.

---

27. Unicode Format Characters

Unicode format characters MUST NOT automatically be treated as identifier characters.

If a future Zamani version explicitly permits a particular format character, that permission MUST be documented in:

grammar/lexer/unicode.md
grammar/lexer/identifiers.md
grammar/specification/lexical.md
grammar/spec/lexical.md

and covered by conformance tests.

---

28. Bidirectional Controls

Bidirectional Unicode controls can affect how source appears when rendered.

The lexer MUST preserve their actual source identity.

The compiler/toolchain SHOULD provide security diagnostics for suspicious bidirectional controls.

The compiler MUST NOT reorder source based on visual display direction.

Logical source order remains authoritative.

---

29. Confusable Characters

Zamani MUST NOT reject a valid identifier solely because it is visually confusable with another identifier.

For example:

foo

and an identifier containing a visually similar character from another script may be distinct.

However, tooling SHOULD provide warnings for suspicious confusable identifiers.

The warning MUST NOT change the program's meaning.

---

30. Homoglyphs Are Not Equivalence

Visual similarity does not imply semantic equality.

The compiler MUST NOT perform:

Latin a == Cyrillic а

or equivalent homoglyph folding.

Identifiers are compared according to the language identifier contract, not according to visual appearance.

---

31. Grapheme Clusters

A grapheme cluster is not necessarily one Unicode scalar value.

For example, what users perceive as one character can consist of multiple scalar values.

Therefore the lexer MUST NOT define:

character = grapheme cluster

unless a specific language construct explicitly requires grapheme semantics.

Grapheme segmentation belongs to text-processing semantics, not basic lexical identity.

---

32. Source Spans and Unicode

The existing source-span contract establishes byte-based canonical positions and explicitly distinguishes byte columns, Unicode scalar columns, grapheme columns, display columns, and UTF-16 positions.

Unicode therefore MUST integrate with source spans as follows:

canonical source identity
        =
FileId + UTF-8 byte offset

Line/column coordinates are derived.

They MUST NOT replace byte-based source identity.

---

33. UTF-8 Byte Boundaries

A source span MUST NOT normally begin or end inside a UTF-8 encoded scalar value.

For example, if:

π

occupies multiple UTF-8 bytes, a user-visible span covering "π" must cover the complete encoded sequence.

Internal byte scanning may inspect individual bytes.

User-visible source spans MUST remain valid UTF-8 boundaries.

---

34. Safe Rust Source Slicing

The Rust implementation MUST use safe UTF-8-aware operations.

It MUST NOT use unchecked UTF-8 slicing.

It MUST NOT use:

unsafe

for Unicode processing.

It MUST NOT assume that:

byte offset == character index

or:

byte length == character count

---

35. Unicode Source Position

The canonical position model is:

FileId + byte offset

A derived diagnostic position may include:

line
column

The implementation MUST document whether diagnostic columns are:

- Unicode scalar-value columns;
- UTF-16 columns;
- byte columns.

The compiler's canonical source identity remains byte-based.

---

36. UTF-16 Tooling Integration

Some external tooling protocols use UTF-16 coordinates.

Zamani tooling MUST convert between:

canonical UTF-8 byte offsets

and:

tool-specific UTF-16 positions

at the tooling boundary.

The compiler MUST NOT change its internal source model to UTF-16 merely to satisfy an external editor protocol.

---

37. Grapheme-Based Tooling

Editor features MAY use grapheme clusters for cursor movement or display.

That MUST remain a tooling concern.

A grapheme cursor position MUST NOT become the compiler's canonical token position.

---

38. Line Endings

Unicode processing must integrate with the source-span line-ending contract.

At minimum, the compiler MUST handle:

LF
CRLF

deterministically.

If standalone CR is accepted as a line terminator, its behavior MUST be explicitly specified.

The host operating system MUST NOT determine source semantics.

---

39. Unicode Line Separators

Unicode includes characters such as:

U+2028 LINE SEPARATOR
U+2029 PARAGRAPH SEPARATOR

These MUST NOT silently become ordinary Zamani newlines merely because another programming language treats them as such.

Their treatment must be explicitly defined by the lexical specification.

If they are not language line terminators, they must not silently alter statement boundaries.

---

40. Unicode in Comments

Unicode comments remain source text.

Comments MUST preserve their original Unicode spelling for:

- diagnostics;
- source maps;
- documentation extraction;
- tooling;
- formatting;
- provenance.

Comment processing MUST NOT normalize source text.

Comment Unicode behavior is coordinated with:

grammar/lexer/comments.md

---

41. Unicode in String Literals

The existing string-literal grammar explicitly keeps Unicode normalization outside ordinary string lexical recognition and preserves source representation rather than immediately constructing a runtime string value.

This specification therefore establishes:

unicode.md
      │
      ▼
string-literals.g4
      │
      ▼
STRING token
      │
      ▼
literal semantic decoding

The lexer MUST NOT normalize ordinary string contents.

---

42. String Source vs String Value

For:

"é"

the lexer preserves the source representation.

For:

"\u00E9"

the lexer preserves the escape spelling.

These are not required to have identical source spellings.

Whether they produce equal semantic string values is a semantic literal-processing decision.

The lexer MUST NOT silently rewrite one into the other.

---

43. Unicode Escapes

Unicode escape syntax is owned jointly by the literal contract and string/character literal grammar.

Unicode processing MUST distinguish:

escape syntax

from:

decoded Unicode scalar value

For example:

\u{03BB}

is source syntax.

The resulting scalar value is:

U+03BB

The lexer MUST NOT silently replace the source spelling with the decoded character.

---

44. Unicode Escape Validity

A syntactically formed Unicode escape does not automatically imply a valid Unicode scalar value.

The semantic literal layer MUST reject values such as:

\u{D800}

because U+D800 is a surrogate.

Likewise:

\u{110000}

is outside the Unicode scalar range.

Such values MUST NOT be:

- wrapped;
- truncated;
- replaced;
- normalized;
- converted to U+FFFD silently.

---

45. Fixed-Width Unicode Escapes

If the language accepts:

\uXXXX

the lexical layer recognizes the syntax.

Semantic validation MUST determine whether the resulting value is valid for the literal type.

The implementation MUST NOT confuse:

UTF-16 code unit

with:

Unicode scalar value

---

46. Braced Unicode Escapes

If the language accepts:

\u{HEX_DIGITS}

the lexical scanner MUST preserve the complete source representation.

The semantic layer determines whether the numeric value denotes a valid Unicode scalar.

The implementation MUST NOT impose an arbitrary hardware-derived maximum on the number of source digits.

Malformed escape syntax MUST be rejected deterministically.

---

47. Unicode Escape Normalization

Unicode escape processing MUST NOT perform normalization.

For example:

\u{00E9}

must not silently become:

e + combining acute accent

or vice versa.

Escape decoding and Unicode normalization are separate operations.

---

48. Character Literals

Character literal semantics MUST use Unicode scalar values unless the language explicitly defines a separate byte/code-unit literal.

Therefore:

'a'
'π'
'量'
'\u{1F600}'

may represent one Unicode scalar each.

A character literal containing multiple Unicode scalar values MUST NOT silently become a single character.

Grapheme clusters do not change this rule.

---

49. Byte Literals

Byte literals are not Unicode scalar literals.

A byte literal represents an explicitly defined byte value.

The compiler MUST NOT silently interpret arbitrary Unicode characters as single bytes.

If a Unicode character requires multiple UTF-8 bytes, those bytes must be represented according to the byte-literal specification.

---

50. Numeric Literals and Unicode Digits

The core numeric literal syntax SHOULD use the explicitly specified Zamani digit alphabet.

The compiler MUST NOT silently accept every Unicode character with Unicode property "Nd" as a decimal digit unless the numeric-literal specification explicitly defines that behavior.

This prevents visually similar numeric representations from creating accidental lexical equivalence.

For example, a Unicode decimal digit from another script MUST NOT automatically be treated as ASCII:

0
1
2
...
9

unless the language specification explicitly says so.

---

51. Unicode Mathematical Symbols

Unicode mathematical symbols MAY appear where the grammar explicitly permits them.

Their presence MUST NOT automatically create operators.

For example, a visually mathematical symbol is not automatically equivalent to:

+
-
*
/
=

Operator spelling belongs to:

grammar/lexer/operators.md

Unicode merely supplies the character universe.

---

52. Unicode Quantum Symbols

Quantum syntax may use Unicode symbols such as:

|ψ⟩
⟨ψ|
⊗
†

but Unicode support MUST NOT cause the lexer to invent quantum semantics.

Quantum syntax belongs to:

grammar/quantum/
grammar/lexer/quantum-literals.md

The Unicode contract only guarantees that the source characters can be represented and lexed deterministically where the grammar permits them.

The canonical semantic boundary remains:

quantum::ir

not the Unicode lexer.

---

53. Bra-Ket Syntax

Characters such as:

⟨
⟩

must remain ordinary Unicode source characters unless the quantum grammar gives them structural meaning.

The Unicode lexer MUST NOT hard-code a finite list such as:

|0⟩
|1⟩
|+⟩
|-⟩

as the complete universe of quantum states.

This preserves generalized quantum syntax and POCO-REAF scalability.

---

54. Unicode HDL Symbols

HDL syntax may use Unicode symbols if explicitly specified.

However, Unicode MUST NOT turn hardware-specific notation into universal language semantics.

Hardware meaning remains owned by:

grammar/hdl/
grammar/hardware/
grammar/resources/

---

55. Unicode in AI/Data/Scientific Domains

Unicode names and textual data MUST be usable across:

- AI/ML;
- data;
- scientific computing;
- tensor computing;
- symbolic mathematics;
- networking;
- security;
- distributed systems;
- classical computing;
- quantum computing;
- HDL.

No domain may define a separate Unicode universe.

---

56. Unicode and Domain Keywords

A domain grammar MUST NOT redefine Unicode identifier rules.

For example:

grammar/quantum/
grammar/ai/
grammar/hdl/
grammar/networking/

must all use the common Unicode lexical contract.

This prevents:

quantum Unicode rules
AI Unicode rules
HDL Unicode rules

from becoming incompatible sublanguages.

---

57. Unicode and Dialects

A dialect MAY introduce additional Unicode syntax only through the dialect contract.

A dialect MUST declare:

- Unicode characters introduced;
- lexical role;
- token mapping;
- parser mapping;
- semantic mapping;
- compatibility;
- source-span behavior;
- diagnostics;
- normalization behavior;
- security implications.

A dialect MUST NOT silently modify core Unicode identifier behavior.

---

58. Unicode and Macros

Macros MUST preserve Unicode source identity.

Macro expansion MUST NOT silently normalize identifiers or literal contents.

If a macro constructs source from Unicode data, the generated source MUST pass through the same lexical Unicode contract.

Macro-generated identifiers MUST obey the same identifier rules as source-written identifiers.

---

59. Unicode and Metaprogramming

Reflection and code generation MUST NOT bypass Unicode validation.

Generated source must remain valid Zamani source according to the same Unicode version and lexical rules.

A code generator MUST NOT rely on the host locale.

---

60. Unicode and Interoperability

Foreign formats may use:

- UTF-8;
- UTF-16;
- UTF-32;
- legacy encodings;
- escaped Unicode;
- language-specific identifiers.

Interoperability layers MUST perform explicit conversion.

The Zamani core lexer MUST receive canonical UTF-8 source.

A foreign format MUST NOT redefine Zamani's Unicode semantics.

---

61. Unicode and Filesystems

Filesystem names are not Zamani identifiers.

The compiler MUST NOT assume that:

filesystem normalization
=
identifier normalization

A path supplied by an operating system may have platform-specific Unicode behavior.

That behavior belongs to the filesystem/path layer.

The language's source identifiers remain governed by this specification.

---

62. Unicode and Locale

The Zamani compiler MUST NOT use the process locale to determine:

- identifier validity;
- keyword matching;
- case sensitivity;
- numeric parsing;
- whitespace classification;
- Unicode normalization.

Compiler behavior must remain deterministic across locales.

---

63. Unicode and Case Conversion

Compiler correctness MUST NOT depend on locale-sensitive case conversion.

In particular, the compiler MUST NOT use host-locale rules for deciding whether two identifiers are equal.

Case behavior is defined by the Zamani language specification.

---

64. Unicode Security

Unicode processing is a security boundary.

The toolchain SHOULD detect or warn about:

- confusable identifiers;
- suspicious mixed-script identifiers;
- bidi controls;
- invisible characters;
- unexpected format characters;
- suspicious normalization differences.

Security diagnostics MUST NOT change program semantics.

---

65. Mixed Scripts

Mixed-script identifiers MAY be valid.

The compiler MUST NOT reject them merely because multiple scripts occur.

For example, legitimate multilingual programs must remain possible.

However, tooling SHOULD detect suspicious mixed-script patterns.

The distinction is:

language validity

versus:

security warning

These MUST NOT be conflated.

---

66. Invisible Characters

Invisible characters that are not explicitly permitted by the identifier rules SHOULD produce diagnostics or warnings when encountered in sensitive lexical positions.

The compiler MUST NOT silently delete them.

Deleting invisible characters would alter source identity and could create security vulnerabilities.

---

67. Unicode Security Must Not Become a Language Limit

Security tooling MUST NOT impose arbitrary limits such as:

only ASCII identifiers
only Latin identifiers
maximum Unicode code points
maximum scripts per file
maximum combining marks

unless such a rule is explicitly adopted as language syntax.

Security policy and language validity remain separate.

---

68. Deterministic Diagnostics

Unicode diagnostics MUST identify the relevant source span.

For example:

invalid UTF-8
invalid Unicode scalar value
unsupported Unicode format character
invalid identifier start
invalid identifier continuation
unexpected Unicode whitespace
suspicious bidirectional control

Diagnostics MUST point to the smallest useful source span where practical.

---

69. Diagnostic Source Preservation

Diagnostics MUST display the original source spelling.

The compiler MUST NOT display only a normalized or transliterated form when the distinction matters.

For example, if two identifiers differ by Unicode normalization, diagnostics SHOULD make the actual source difference visible.

---

70. Unicode and Source Maps

Source maps MUST use canonical source spans.

Unicode transformations MUST NOT invalidate source provenance.

Any transformation that changes source representation must carry an explicit mapping.

This applies to:

- macros;
- generated code;
- formatting;
- code generation;
- interpolation;
- foreign-language conversion.

---

71. Unicode and AST

The AST MUST retain enough information to preserve Unicode source identity.

For an identifier:

source spelling
source span
semantic name representation

must remain distinguishable.

For a literal:

source spelling
literal kind
source span
decoded semantic value

must not be conflated.

The AST MUST NOT require Unicode normalization merely to represent source.

---

72. Unicode and Semantic Analysis

Semantic analysis MAY apply a language-defined identifier comparison policy.

It MUST NOT silently change the source representation.

Semantic analysis MUST also preserve source provenance.

---

73. Unicode and Canonical IR

Canonical IRs must not depend on source Unicode encoding unless source provenance is intentionally preserved.

A semantic identifier may be represented internally in a canonical form, but that does not permit source spelling to be discarded when diagnostics or provenance require it.

For quantum programs:

Unicode source
    ↓
lexer
    ↓
AST
    ↓
semantic quantum representation
    ↓
quantum::ir

Unicode MUST NOT create a second quantum IR.

---

74. Unicode and POCO-REAF

Unicode behavior must support:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever.»

The same Zamani source must have the same Unicode lexical meaning regardless of:

- CPU;
- GPU;
- FPGA;
- QPU;
- accelerator;
- embedded target;
- cloud machine;
- distributed cluster;
- operating system;
- filesystem implementation.

Only actual resource availability and explicitly documented implementation policies may affect whether compilation or execution succeeds.

---

75. No Unicode Hardware Limits

This specification MUST NOT define:

MAX_UNICODE_IDENTIFIER_LENGTH
MAX_UNICODE_SOURCE_SIZE
MAX_UNICODE_FILE_SIZE
MAX_UNICODE_STRING_SIZE
MAX_UNICODE_COMBINING_MARKS
MAX_UNICODE_SCRIPT_COUNT

as language limits.

The absence of such constants is intentional.

---

76. Scalability

Unicode processing must scale from:

one ASCII character

to:

very large Unicode source programs

subject only to:

- available memory;
- available storage;
- compiler resource budgets;
- implementation architecture.

The language itself MUST NOT impose arbitrary finite source limits merely because an implementation currently uses a particular integer type or buffer size.

---

77. Large Unicode Source

The lexer SHOULD process source in linear time.

It SHOULD avoid:

- repeated normalization;
- repeated whole-source copying;
- per-character heap allocation;
- quadratic string concatenation;
- regex behavior with catastrophic backtracking.

Unicode classification SHOULD be O(1) or amortized O(1) per scalar value for the supported property lookup model.

---

78. Streaming and Incremental Processing

A production implementation MAY process Unicode source incrementally or in chunks.

Chunk boundaries MUST NOT alter lexical meaning.

A Unicode scalar encoded across bytes MUST be handled correctly when input is streamed.

A chunk boundary MUST NOT become a semantic boundary.

---

79. Incremental Compilation

Incremental compilation MUST preserve Unicode semantics.

Editing:

ASCII source

into:

Unicode source

must invalidate the appropriate lexical and downstream dependencies.

The compiler MUST NOT reuse stale lexical classifications after Unicode-relevant source changes.

---

80. Unicode Cache Keys

Caches MUST NOT use a lossy representation of Unicode source as the semantic cache key.

A cache identity SHOULD incorporate:

source identity/content
language version
Unicode specification version
relevant lexical configuration

This prevents Unicode-version changes from producing stale compiler results.

---

81. Unicode Reproducibility

Reproducible compilation requires deterministic Unicode classification.

A build MUST NOT depend on the host operating system's Unicode database unless that database is explicitly part of the build contract.

The preferred model is:

Zamani version
+
specified Unicode data
=
deterministic lexical behavior

---

82. Unicode and Formatting

Formatters MUST preserve semantic Unicode identity.

A formatter MAY change:

whitespace
line breaks
indentation

where language semantics permit.

It MUST NOT silently normalize identifiers.

It MUST NOT silently rewrite Unicode literals.

---

83. Unicode and Pretty Printing

Pretty printers SHOULD preserve source meaning.

If a canonical printer chooses normalized formatting for presentation, it MUST NOT cause two semantically distinct identifiers to become identical.

---

84. Unicode and Documentation

Documentation comments may contain arbitrary valid Unicode subject to source encoding rules.

Documentation tooling MUST preserve Unicode text.

It MUST NOT assume ASCII.

---

85. Unicode and Diagnostics

Diagnostics may contain Unicode.

The diagnostic engine MUST correctly render:

- source snippets;
- Unicode identifiers;
- Unicode literals;
- Unicode operators;
- Unicode error locations.

The compiler MUST distinguish source-byte offsets from display columns.

---

86. Unicode and Terminal Rendering

Terminal rendering is not source semantics.

A terminal may:

- use different fonts;
- render combining marks differently;
- have different bidi behavior;
- use different display widths.

The compiler MUST NOT depend on terminal appearance.

---

87. Unicode and IDE/LSP

IDE/LSP integration MUST translate from canonical Zamani positions into protocol-specific positions.

The canonical compiler model remains:

FileId + UTF-8 byte offset

An IDE may require:

UTF-16 line/character

That conversion belongs at the boundary.

---

88. Unicode and Diagnostics Across Platforms

The same source should produce semantically equivalent diagnostics on:

- Linux;
- Windows;
- macOS;
- embedded development environments;
- CI;
- containers;
- remote compilation environments.

Path rendering may differ.

Unicode lexical meaning must not.

---

89. Unicode and Error Recovery

Malformed Unicode MUST NOT cause undefined lexer behavior.

The lexer MUST recover deterministically where recovery is supported.

Malformed UTF-8 MUST be rejected at the source boundary.

Malformed Unicode syntax MUST produce structured lexical diagnostics.

The lexer MUST NOT:

- panic;
- invoke undefined behavior;
- access invalid memory;
- silently skip arbitrary bytes;
- enter an unbounded recovery loop.

---

90. Safe Rust Requirement

The Rust implementation MUST use safe Rust only.

Production Unicode handling MUST NOT use:

unsafe

or unchecked conversion APIs whose safety depends on manually maintained invariants.

The implementation SHOULD use:

- "char";
- "str";
- safe UTF-8 iteration;
- safe indexing patterns;
- explicit "Result";
- structured diagnostics;
- deterministic Unicode-property tables.

---

91. Rust Version

The implementation baseline is:

Rust 1.97 / Rust 1.97.1
Rust 2021

Unicode implementation choices MUST remain compatible with the repository's Rust baseline.

No newer compiler feature may become an accidental requirement.

---

92. No Panic on User Unicode

Malformed source is input, not a programmer error in the compiler.

The lexer MUST NOT rely on:

unwrap()
expect()
unreachable!()

for malformed user-controlled Unicode input.

Where malformed Unicode can occur, the implementation MUST return a structured error.

---

93. No Locale-Dependent Rust Behavior

Rust Unicode handling MUST NOT be wrapped in locale-dependent behavior.

The compiler must explicitly define:

- Unicode classification;
- normalization policy;
- case policy;
- whitespace policy;
- line-ending policy.

---

94. ANTLR Integration

"grammar/Zamani.g4" is the canonical grammar composition/root grammar.

Unicode rules MUST integrate into the ANTLR architecture without creating a competing root grammar.

The preferred structure is:

Zamani.g4
    │
    └── canonical lexer/parser composition
             │
             ├── identifiers
             ├── literals
             ├── strings
             ├── comments
             └── Unicode character handling

No domain grammar may define an alternative Unicode universe.

---

95. ANTLR Unicode Character Classes

ANTLR lexical rules MAY use Unicode-aware character/property facilities where supported by the selected ANTLR target.

However, the Rust implementation and ANTLR grammar MUST agree semantically.

ANTLR acceptance MUST NOT become a separate language definition.

The conformance target is:

same source
→ same lexical classification

---

96. Rust Lexer vs ANTLR

If "src/lexer.rs" and ANTLR produce different Unicode tokenization, that is a conformance failure.

The difference MUST be resolved through the canonical lexical specification.

The implementation MUST NOT declare one behavior correct merely because one lexer happens to accept it.

---

97. Unicode Property Data

Unicode property data used by the compiler SHOULD be pinned to the language's normative Unicode version.

The implementation MUST avoid silently depending on whichever Unicode database happens to be installed on the host.

If a dependency supplies Unicode properties, its version and data behavior MUST be compatible with the Zamani language version.

---

98. Unicode Property Fallback

The compiler MUST NOT silently fall back from:

Unicode-aware classification

to:

ASCII classification

when Unicode data is unavailable.

Such a fallback would create two different Zamani lexical languages.

The compiler should fail clearly if required Unicode data is unavailable.

---

99. Unicode Normalization Libraries

A Unicode normalization library MAY be used downstream if normalization is explicitly required by a semantic feature.

The lexer MUST NOT normalize source merely because such a library is available.

Normalization is an explicit operation, not an implicit compiler side effect.

---

100. Unicode and Source Identity

The exact source spelling MUST remain available where needed for:

- diagnostics;
- AST provenance;
- source maps;
- formatting;
- refactoring;
- reproducible builds;
- debugging;
- IDE features.

Unicode source MUST never be irreversibly rewritten at lexical recognition.

---

101. Unicode and Tokens

The token contract requires token identity to remain associated with:

- token kind;
- source spelling/lexeme;
- source span;
- downstream AST/semantic information.

Unicode MUST therefore not cause token text to be replaced by a normalized spelling.

A token MAY additionally carry a decoded or classified representation, but that representation MUST NOT replace the original source information.

---

102. Unicode and Literals

"grammar/lexer/literals.md" owns the complete literal contract.

"grammar/lexer/string-literals.g4" owns ordinary string token syntax.

"grammar/lexer/character-literals.g4" owns character literal syntax.

"unicode.md" owns the Unicode rules used by those literal contracts.

This prevents:

literals.md
string-literals.g4
character-literals.g4
unicode.md

from independently defining conflicting Unicode behavior.

---

103. Unicode and Keywords

"grammar/lexer/keywords.md" owns the keyword registry.

Unicode MUST NOT duplicate the keyword list.

The Unicode contract only establishes how the source characters making up keywords are represented and compared.

---

104. Unicode and Operators

"grammar/lexer/operators.md" owns operator spelling and precedence.

Unicode symbols are operators only if explicitly registered there.

Unicode MUST NOT automatically transform visually similar characters into ASCII operators.

---

105. Unicode and Comments

"grammar/lexer/comments.md" owns comment syntax.

Unicode determines character validity and source representation.

Comment semantics remain separate.

---

106. Unicode and Delimiters

"grammar/lexer/delimiters.md" owns delimiter syntax.

Unicode characters may be delimiters where explicitly defined.

No Unicode character becomes a delimiter merely because it resembles an existing ASCII delimiter.

---

107. Unicode and Interpolation

If interpolation is introduced, it must integrate with:

grammar/lexer/interpolation.md
grammar/expressions/
grammar/frontend/AST

Interpolation MUST preserve source spans across Unicode text and embedded expressions.

Nested interpolation depth MUST NOT be hard-coded as a language limit.

---

108. Unicode and Quantum Literals

Quantum literal syntax belongs to:

grammar/lexer/quantum-literals.md
grammar/quantum/

Unicode provides characters such as:

⟨
⟩
ψ
⊗
†

where permitted.

The quantum grammar must remain generalized.

It MUST NOT encode a finite Unicode list of possible quantum states.

---

109. Unicode and Hardware/HDL

Unicode may appear in:

- signal names;
- module names;
- annotations;
- documentation;
- symbolic mathematical expressions;

where the relevant grammar permits it.

Unicode MUST NOT encode target-specific physical limits.

For example, a Unicode identifier cannot implicitly mean:

physical_qubit_0

unless that is explicit source semantics.

---

110. Unicode and Resources

Resource names and capability names must obey the common identifier contract.

Unicode MUST NOT create machine-specific resource identifiers.

The portable source should express:

capability("...")

or equivalent semantic requirements.

Actual device identity belongs downstream.

---

111. Unicode and Distributed Computing

Unicode node/service/process names may be legal identifiers.

The grammar MUST NOT impose a finite number of Unicode-named nodes.

The number of resources remains determined by program semantics and available resources.

---

112. Unicode and AI/Data

AI model names, dataset names, tensor labels, schema names, and symbolic variables must use the common Unicode rules.

No AI subsystem may create a separate Unicode identifier model.

---

113. Unicode and Security

Security-sensitive names SHOULD preserve exact source spelling.

Security tools SHOULD expose:

- code points;
- Unicode names;
- normalization differences;
- confusable information;
- bidi controls.

Such information is diagnostic metadata.

It MUST NOT silently mutate the program.

---

114. Unicode and Cryptography

Cryptographic input is data, not automatically source identifiers.

The compiler MUST NOT normalize arbitrary string/byte data before cryptographic operations.

For example, a cryptographic hash over source-level string data must operate according to the explicit semantic representation of that data.

Unicode normalization MUST NOT be silently inserted into cryptographic semantics.

---

115. Unicode and Networking

Network protocol strings may have protocol-specific Unicode rules.

Those rules belong to the networking domain.

They MUST NOT alter the core Zamani Unicode source model.

For example:

hostname syntax
URL syntax
DNS rules
HTTP rules

must remain domain/protocol semantics.

---

116. Unicode and Interoperability

When translating Zamani source into another language:

Zamani Unicode source
        ↓
canonical semantic representation
        ↓
foreign representation

The interoperability layer is responsible for target-specific identifier restrictions.

The core Zamani source MUST NOT be constrained merely because one target language has a narrower identifier set.

---

117. Foreign Identifier Escaping

If a target language cannot represent a Zamani Unicode identifier directly, the interoperability layer MUST perform explicit escaping or name mapping.

The mapping MUST preserve:

- semantic identity;
- uniqueness;
- provenance;
- reversibility where required.

The core language remains Unicode-capable.

---

118. Unicode and Name Mangling

Backend name mangling MUST NOT modify source identifier semantics.

Mangling is an implementation representation.

Source spelling remains available through provenance metadata.

---

119. Unicode and Serialization

Serialized compiler structures MUST define their Unicode encoding explicitly.

The preferred representation is UTF-8 for textual interchange.

Binary formats MUST explicitly define:

- encoding;
- length;
- validity;
- version;
- compatibility.

---

120. Unicode and Version Compatibility

A change to Unicode classification that changes whether source is legal is a language compatibility change.

Such changes MUST be documented under:

grammar/compatibility/
grammar/spec/versioning.md

A Unicode upgrade MUST NOT silently change the meaning of existing source.

---

121. Backward Compatibility

When new Unicode characters become valid identifier characters under a newer language version, existing source MUST retain its meaning.

If a previously valid identifier becomes invalid, the compatibility mechanism MUST explicitly document that change.

Silent source breakage is prohibited.

---

122. Forward Compatibility

Older compilers MAY reject Unicode syntax introduced by newer language versions.

They MUST produce a useful diagnostic rather than silently reinterpret it as a different program.

---

123. Unicode Feature Gating

If Unicode behavior changes incompatibly, the change MAY be protected by an explicit language-version or feature gate.

Feature gating MUST NOT be based on:

host OS
host architecture
hardware

---

124. Unicode Conformance

Unicode conformance tests MUST verify at minimum:

UTF-8

- valid ASCII;
- valid multi-byte UTF-8;
- invalid UTF-8;
- boundary scalar values;
- malformed sequences.

Identifiers

- ASCII;
- Unicode letters;
- combining marks;
- Unicode digits in continuation positions;
- invalid starts;
- invalid continuations;
- underscore;
- mixed scripts;
- long identifiers.

Whitespace

- ASCII space;
- tab;
- CR;
- LF;
- CRLF;
- Unicode spaces;
- unexpected non-breaking spaces.

Security

- bidi controls;
- zero-width characters;
- confusables;
- mixed scripts.

Literals

- Unicode string contents;
- Unicode escapes;
- surrogate escapes;
- maximum scalar;
- invalid scalar;
- combining sequences.

Source spans

- multi-byte characters;
- line/column calculations;
- UTF-8 boundary correctness;
- EOF;
- CRLF.

---

125. Required Positive Tests

At minimum:

let π = 3.14;
let λ = 1;
let 量子 = 42;
let состояние = 1;
let données = 2;
let بيانات = 3;

provided these names satisfy the identifier contract.

Unicode strings:

"π"
"量子"
"😀"
"é"
"\u{03BB}"

must preserve source identity.

---

126. Required Negative Tests

The conformance suite MUST include:

- invalid UTF-8;
- malformed UTF-8 continuation;
- invalid Unicode scalar escape;
- surrogate escape;
- out-of-range scalar;
- invalid identifier start;
- invalid invisible character where prohibited;
- malformed bidi control usage where diagnosed;
- invalid Unicode whitespace where not permitted.

---

127. Boundary Tests

Boundary tests MUST include:

U+0000
U+007F
U+0080
U+07FF
U+0800
U+D7FF
U+D800
U+DFFF
U+E000
U+FFFF
U+10000
U+10FFFF
U+110000

The values outside the Unicode scalar range must be rejected where scalar semantics apply.

---

128. UTF-8 Boundary Tests

Test all UTF-8 sequence lengths:

1-byte
2-byte
3-byte
4-byte

and verify:

byte offset
scalar position
line
column
span

remain correct.

---

129. Normalization Tests

At minimum test visually similar source with distinct Unicode representations.

For example:

NFC form
NFD form

must remain distinguishable at source level.

If semantic identifier normalization is not enabled for the language version, they must not silently become the same identifier.

---

130. Confusable Tests

Include source containing visually confusable identifiers.

Verify:

1. lexical identity remains distinct;
2. semantic identity follows identifier rules;
3. security tooling may warn;
4. the compiler does not silently merge them.

---

131. Bidi Tests

Include Unicode bidirectional controls in test inputs.

Verify:

- source bytes remain unchanged;
- lexical boundaries remain deterministic;
- security diagnostics are emitted where configured;
- semantic interpretation is not reordered according to visual presentation.

---

132. Large-Identifier Tests

Test identifiers substantially larger than typical machine word sizes.

The test MUST verify that the grammar itself does not reject them due to arbitrary fixed limits.

If an implementation resource budget rejects the source, the diagnostic MUST identify resource exhaustion rather than language invalidity.

---

133. Large-Unicode-Source Tests

The test suite SHOULD generate large Unicode source inputs.

The purpose is to verify:

- linear behavior;
- absence of quadratic copying;
- stable spans;
- deterministic tokenization;
- bounded diagnostic behavior;
- resource-budget correctness.

---

134. Determinism Tests

Given identical:

source
language version
Unicode version
lexer configuration

the lexer MUST produce identical:

tokens
spans
diagnostics

across supported host environments.

---

135. Property-Based Testing

Unicode lexical processing SHOULD use property-based/fuzz testing.

Useful properties include:

valid UTF-8 does not panic

token spans remain ordered

source slicing never crosses invalid UTF-8 boundaries

lexer output is deterministic

Unicode normalization is not performed implicitly

invalid UTF-8 never becomes valid source

---

136. Fuzzing

The Unicode lexer SHOULD be fuzz-tested with:

- random valid Unicode;
- random invalid UTF-8;
- combining sequences;
- bidi controls;
- long identifiers;
- long strings;
- unusual whitespace;
- malformed escapes;
- mixed scripts.

Fuzzing MUST verify that malformed source cannot trigger:

- panic;
- undefined behavior;
- infinite loop;
- unbounded recursion;
- memory unsafety.

---

137. Resource-Bounded Unicode Processing

Implementations MAY provide configurable resource budgets for:

- source size;
- token size;
- diagnostic size;
- AST memory;
- compilation memory;
- processing time.

These are implementation controls.

They MUST NOT be embedded as language-level Unicode limits.

---

138. No Artificial Unicode Constants

The following patterns are prohibited as universal language rules:

MAX_UNICODE_CHARS
MAX_UNICODE_BYTES
MAX_IDENTIFIER_CODEPOINTS
MAX_COMBINING_MARKS
MAX_SCRIPTS
MAX_SOURCE_UNICODE

unless a future specification explicitly adopts them as semantic language rules.

---

139. Performance Contract

Unicode processing SHOULD be:

O(n)

with respect to source length.

The implementation SHOULD avoid repeated full-source normalization.

Unicode classification SHOULD use efficient property lookup.

The lexer SHOULD avoid allocating a new object for every Unicode scalar value.

---

140. Memory Contract

Unicode processing MUST NOT require source-size-proportional auxiliary allocations beyond what the chosen lexer architecture actually needs.

Where practical:

source slice/span

should be retained instead of copying large Unicode strings.

Decoded semantic values may be allocated later where required.

---

141. No Premature Decoding

The lexer SHOULD NOT eagerly decode every Unicode literal into a target-specific runtime representation.

Instead:

source spelling
→ token
→ AST literal
→ semantic decoding

This preserves:

- exact source;
- diagnostics;
- reproducibility;
- target independence.

---

142. No Premature Normalization

Likewise:

source
→ normalize
→ tokenize

is prohibited.

The correct model is:

source
→ validate
→ tokenize
→ semantic processing where explicitly required

---

143. Unicode and Constant Folding

Constant folding MUST preserve semantic Unicode values.

However, constant folding MUST NOT rewrite source spans.

For example:

"\u{03BB}"

may semantically fold to a Unicode scalar/string value while the source remains the original escape spelling.

---

144. Unicode and Serialization of AST

AST serialization SHOULD preserve both:

source spelling

and:

semantic value

when required by the AST contract.

A serializer MUST NOT serialize only a normalized value if doing so would destroy source provenance.

---

145. Unicode and Reproducible Builds

Build reproducibility requires stable Unicode property data.

The build should record the relevant:

language version
Unicode version
lexer version
grammar version

where practical.

---

146. Unicode and Generated Code

Generated Zamani source MUST use UTF-8.

Generated source MUST obey the same Unicode specification.

Generators MUST NOT depend on the host locale.

---

147. Unicode and Macro-Generated Identifiers

A macro generating:

identifier

must generate a valid Zamani identifier under the same Unicode rules.

It MUST NOT bypass identifier validation.

---

148. Unicode and Reflection

Reflection APIs that expose source identifiers SHOULD expose:

- original spelling;
- source span/provenance;
- semantic identity.

Reflection MUST NOT assume ASCII.

---

149. Unicode and Diagnostics Serialization

Machine-readable diagnostics MUST preserve Unicode without lossy transliteration.

A diagnostic protocol MAY provide escaped representations in addition to the original Unicode text.

The original source representation remains authoritative.

---

150. Unicode and JSON/Configuration

If Zamani tooling exchanges Unicode through JSON or similar formats, escaping is a serialization concern.

It MUST NOT alter the underlying semantic source.

For example:

\u03BB

in a JSON transport can represent:

λ

without changing the source-level Unicode contract.

---

151. Unicode and Package Metadata

Package metadata may contain Unicode names and descriptions.

Package identifiers and package display names must remain distinct concepts.

Filesystem/package-manager restrictions must not redefine core source identifier semantics.

---

152. Unicode and Network Transport

Source transported over a network MUST retain byte-for-byte identity when source identity matters.

Transport encoding conversion MUST be explicit.

A network transport MUST NOT silently normalize source.

---

153. Unicode and Distributed Compilation

Distributed compiler workers MUST use compatible:

Zamani language version
Unicode version
lexical configuration

otherwise the compilation result may diverge.

This must be part of compiler provenance where distributed compilation is supported.

---

154. Unicode and Remote Caches

Remote compilation caches MUST include Unicode-relevant compiler configuration in cache identity.

A cache generated with one Unicode classification version MUST NOT be silently reused by an incompatible compiler configuration.

---

155. Unicode and Embedded Targets

Embedded targets do not define the source Unicode subset.

A program may contain Unicode source even when the final runtime has limited text capabilities.

Compilation may lower or encode the semantic representation according to target capabilities.

Source syntax remains Unicode-capable.

---

156. Unicode and Hardware Acceleration

GPU, FPGA, QPU, accelerator, and specialized hardware capabilities MUST NOT alter source Unicode semantics.

Unicode is resolved before target-specific lowering.

---

157. Unicode and Quantum Compilation

Quantum compilation MUST receive Unicode-resolved semantic constructs through the normal frontend.

Unicode processing MUST NOT perform:

- gate decomposition;
- qubit allocation;
- physical mapping;
- routing;
- scheduling;
- QEC;
- noise analysis.

Those remain downstream responsibilities.

---

158. Unicode and QEC/ZQN/HAL

Unicode is outside the ownership of:

- QEC;
- ZQN;
- HAL.

These systems consume semantic information after frontend processing.

The Unicode lexer MUST NOT become coupled to hardware fault semantics.

---

159. Unicode and HDL Compilation

Unicode processing ends at lexical/semantic source handling.

HDL synthesis, simulation, timing, placement, routing, and physical realization remain downstream.

Unicode MUST NOT encode physical resource limits.

---

160. Unicode and POCO-REAF Portability

The language must allow:

same source
        ↓
different compiler targets
        ↓
different hardware resources

without changing Unicode meaning.

Target adaptation occurs downstream.

---

161. Integration Matrix

File| Unicode responsibility
"grammar/DESIGN.md"| Overall authority and architecture
"grammar/README.md"| Navigation and ownership
"grammar/specification/lexical.md"| Human-readable normative lexical model
"grammar/spec/lexical.md"| Formal lexical contract
"grammar/lexer/unicode.md"| This complete Unicode contract
"grammar/lexer/identifiers.md"| Identifier-specific Unicode rules
"grammar/lexer/keywords.md"| Keyword registry
"grammar/lexer/tokens.md"| Token representation
"grammar/lexer/literals.md"| Complete literal contract
"grammar/lexer/string-literals.g4"| Ordinary string lexical syntax
"grammar/lexer/character-literals.g4"| Character lexical syntax
"grammar/lexer/comments.md"| Comment behavior
"grammar/lexer/delimiters.md"| Delimiters
"grammar/lexer/operators.md"| Operators
"grammar/lexer/quantum-literals.md"| Quantum-specific literal syntax
"grammar/Zamani.g4"| Canonical grammar composition
"src/lexer.rs"| Executable lexical implementation
"src/parser.rs"| Parser integration
"src/frontend/ast/"| AST representation
source-map implementation| Byte-based provenance
semantic analysis| Semantic Unicode/name interpretation
canonical IR| Target-independent semantic representation
"quantum::ir"| Canonical quantum semantic boundary
"grammar/validation/"| Conformance and hard-coding validation
"grammar/compatibility/"| Version/migration policy
"grammar/tests/"| Executable conformance

---

162. Ownership Boundaries

This file OWNS:

- UTF-8 source model;
- Unicode scalar model;
- Unicode version policy;
- Unicode lexical classification foundation;
- Unicode normalization boundary;
- Unicode security boundary;
- Unicode/source-span integration;
- Unicode portability.

This file DOES NOT OWN:

- identifier grammar details;
- keyword lists;
- string delimiters;
- character literal delimiters;
- numeric syntax;
- parser grammar;
- AST structure;
- semantic type rules;
- quantum semantics;
- hardware semantics.

This separation prevents specification duplication.

---

163. Upstream Contracts

This file depends on:

grammar/DESIGN.md
grammar/specification/lexical.md
grammar/spec/lexical.md
grammar/spec/source-spans.md

It must remain compatible with the repository's established:

- source encoding;
- token model;
- source-span model;
- AST architecture;
- safe-Rust requirement;
- POCO-REAF architecture.

---

164. Downstream Consumers

The downstream consumers are:

src/lexer.rs
src/parser.rs
src/frontend/ast/
semantic analysis
diagnostics
source maps
formatter
LSP/IDE tooling
macro system
metaprogramming
compiler
canonical semantic IR
quantum::ir
HDL/hardware IR

No downstream consumer may redefine Unicode semantics.

---

165. Public Contract

A conforming Zamani Unicode implementation MUST:

1. accept valid UTF-8;
2. reject invalid UTF-8 deterministically;
3. preserve source spelling;
4. avoid implicit normalization;
5. use deterministic Unicode classification;
6. respect the language's specified Unicode version;
7. preserve Unicode-aware source spans;
8. distinguish scalar values from code units;
9. avoid locale-dependent behavior;
10. use safe Rust;
11. avoid arbitrary language-level Unicode limits;
12. produce deterministic diagnostics;
13. remain compatible with the canonical grammar;
14. integrate with the common identifier/literal contracts.

---

166. Negative Contract

A conforming implementation MUST NOT:

- silently replace invalid UTF-8;
- silently normalize source;
- silently transliterate identifiers;
- use host locale for language semantics;
- use ASCII-only identifiers as the universal rule;
- treat UTF-16 code units as Zamani characters;
- treat grapheme clusters as lexical characters by default;
- silently fold confusables;
- silently remove invisible characters;
- silently reorder bidi text;
- hard-code machine-size Unicode limits;
- introduce domain-specific Unicode grammars;
- use "unsafe";
- panic on ordinary malformed source.

---

167. Hard-Coding Audit

The following patterns MUST be rejected during review if they represent universal language limits:

MAX_UNICODE_LENGTH
MAX_IDENTIFIER_LENGTH
MAX_SOURCE_UNICODE
MAX_CODEPOINTS
MAX_GRAPHEMES
MAX_SCRIPTS
MAX_COMBINING_MARKS
MAX_UTF8_BYTES

A finite implementation buffer is acceptable only as an explicit resource policy.

It must not become a grammar rule.

---

168. Completion Criteria

"grammar/lexer/unicode.md" is complete when all of the following are true:

- [x] UTF-8 is explicitly defined.
- [x] Unicode scalar values are distinguished from code points.
- [x] UTF-16 code units are not the core character model.
- [x] Unicode normalization policy is explicit.
- [x] source spelling is preserved.
- [x] identifier integration is defined.
- [x] keyword integration is defined.
- [x] literal integration is defined.
- [x] string integration is defined.
- [x] character integration is defined.
- [x] whitespace integration is defined.
- [x] source-span integration is defined.
- [x] diagnostics integration is defined.
- [x] security behavior is defined.
- [x] Unicode versioning is defined.
- [x] compatibility behavior is defined.
- [x] ANTLR integration is defined.
- [x] Rust integration is defined.
- [x] safe-Rust requirement is explicit.
- [x] scalability requirements are explicit.
- [x] hard-coding prohibition is explicit.
- [x] deterministic behavior is explicit.
- [x] testing requirements are explicit.
- [x] quantum integration boundary is explicit.
- [x] HDL/hardware boundary is explicit.
- [x] AI/data/distributed integration is explicit.
- [x] interoperability is explicit.
- [x] macro/metaprogramming behavior is explicit.
- [x] POCO-REAF is preserved.

---

169. Required Repository Follow-Up Integration

This file is independently complete, but the repository must enforce these existing-file relationships.

"grammar/lexer/identifiers.md"

It MUST reference this file for:

- Unicode character model;
- Unicode version;
- normalization;
- confusable handling;
- format characters;
- bidi handling.

It remains the owner of identifier-specific syntax.

"grammar/lexer/literals.md"

It MUST reference this file for:

- Unicode scalar validity;
- Unicode escape semantics;
- source preservation;
- Unicode normalization;
- Unicode literal diagnostics.

It remains the umbrella literal contract.

"grammar/lexer/string-literals.g4"

It MUST continue to own ordinary string token syntax.

It MUST NOT introduce an independent Unicode normalization policy.

The existing design already keeps normalization outside string lexical recognition.

"grammar/spec/source-spans.md"

This file MUST remain the source-location authority.

Unicode source positions must continue to use the existing byte-based canonical model.

"grammar/specification/lexical.md"

It should reference this file rather than duplicate the complete Unicode rules.

"grammar/spec/lexical.md"

It should reference this file as the detailed Unicode implementation contract.

"grammar/lexer/README.md"

It should list:

unicode.md

as the normative Unicode contract.

"grammar/grammar.md"

It should identify Unicode support as part of lexical conformance.

"grammar/Zamani-Grammar.md"

Unicode proposals contained there MUST NOT override this specification automatically.

"grammar/Zamani.g4"

The root grammar remains the composition authority. Unicode rules MUST NOT create another competing root grammar. The current root already establishes the target-independent architecture and safe-Rust policy.

---

170. Final Architectural Rule

The complete Zamani Unicode architecture is:

                 Zamani Source
                       │
                       ▼
                  UTF-8 bytes
                       │
                       ▼
              Unicode validation
                       │
                       ▼
             Unicode scalar stream
                       │
          ┌────────────┼────────────┐
          │            │            │
          ▼            ▼            ▼
     identifiers    literals     operators
          │            │            │
          └────────────┼────────────┘
                       ▼
                 canonical lexer
                       │
                       ▼
                 tokens + spans
                       │
                       ▼
                    parser
                       │
                       ▼
              domain-neutral AST
                       │
                       ▼
           structural + semantic analysis
                       │
                       ▼
             canonical semantic model
                       │
       ┌───────────────┼────────────────┐
       │               │                │
       ▼               ▼                ▼
 Classical IR      quantum::ir      HDL/Hardware IR
       │               │                │
       └───────────────┼────────────────┘
                       ▼
                  optimization
                       │
             routing / scheduling
                       │
              resilience / QEC / ZQN
                       │
                      HAL
                       │
                target realization

The essential invariant is:

«Unicode belongs to the portable source-language boundary. It must never become a hidden source-rewriting layer, a hardware-dependent restriction, or a second semantic system.»

The result is a Unicode model that can support tiny programs and arbitrarily large source programs subject to actual resource availability, while preserving exact source identity, deterministic compilation, safe Rust, source provenance, and the POCO-REAF architecture.