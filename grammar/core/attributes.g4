/**

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* File:
* grammar/core/attributes.g4
* 
* Status:
* Production parser component.
* 
* Grammar technology:
* ANTLR4 parser grammar.
* 
* Rust baseline:
* Rust 1.97 / Rust 1.97.1.
* 
* Safety:
* This grammar contains no embedded Rust actions, semantic predicates,
* filesystem access, network access, process execution, runtime callbacks,
* or unsafe code.
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file is the canonical reusable grammar for SOURCE-LEVEL ATTRIBUTES.
* 
* An attribute is structured metadata attached to another Zamani construct.
* 
* Conceptually:
* 
* @attribute
* @attribute(...)
* @namespace::attribute
* @namespace::attribute(...)
* 
* Attribute syntax is deliberately generic.  The meaning of an attribute is
* determined after parsing by the semantic/attribute registry and by the
* construct to which the attribute is attached.
* 
* This file therefore defines WHAT AN ATTRIBUTE LOOKS LIKE, not WHAT AN
* ATTRIBUTE MEANS.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* source
*   |
*   v
* canonical lexer
*   |
*   | AT
*   | IDENTIFIER
*   | DOUBLE_COLON
*   | LPAREN / RPAREN
*   | LBRACKET / RBRACKET
*   | LBRACE / RBRACE
*   | COMMA
*   | ASSIGN
*   | literal tokens
*   |
*   v
* grammar/core/names.g4
*   |
*   v
* grammar/core/attributes.g4
*   |
*   +--> declarations
*   +--> types
*   +--> functions
*   +--> modules
*   +--> effects
*   +--> memory
*   +--> concurrency
*   +--> classical
*   +--> quantum
*   +--> hybrid
*   +--> HDL
*   +--> hardware
*   +--> distributed
*   +--> AI
*   +--> data
*   +--> networking
*   +--> security
*   +--> resources
*   +--> compile
*   +--> execution
*   +--> interoperability
*   +--> dialects
*   +--> macros
*   +--> metaprogramming
*   |
*   v
* frontend AST
*   |
*   v
* structural / semantic analysis
*   |
*   +--> attribute validation
*   +--> capability interpretation
*   +--> resource interpretation
*   +--> compiler metadata
*   +--> optimization metadata
*   +--> interoperability metadata
*   +--> domain metadata
*   |
*   v
* canonical semantic model / IR
* 
* Attributes MUST NOT directly lower to a hardware implementation merely
* because an attribute happens to contain hardware-related vocabulary.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - generic attribute attachment syntax;
* - attribute lists;
* - attribute names;
* - qualified attribute names;
* - attribute argument lists;
* - positional attribute arguments;
* - named attribute arguments;
* - attribute scalar values;
* - structured attribute lists;
* - structured attribute maps;
* - nested attribute values;
* - optional attribute prefixes;
* - reusable attribute wrappers.
* 
* THIS FILE DOES NOT OWN:
* 
* - identifier lexical syntax;
* - keyword spelling;
* - literal lexical spelling;
* - Unicode classification;
* - comments;
* - whitespace;
* - paths;
* - URLs;
* - filesystem locations;
* - hardware addresses;
* - device discovery;
* - resource discovery;
* - capability negotiation;
* - module resolution;
* - symbol resolution;
* - type checking;
* - expression precedence;
* - runtime evaluation;
* - compiler optimization;
* - scheduling;
* - routing;
* - QEC;
* - ZQN;
* - HAL;
* - physical qubit placement;
* - classical IR;
* - quantum::ir;
* - HDL/hardware IR;
* - target selection;
* - deployment;
* - runtime behavior.
* 
* ============================================================================
* SINGLE SOURCE OF TRUTH
* ============================================================================
* 
* Lexical tokens are owned by:
* 
* grammar/lexer/tokens.g4
* 
* In particular, this grammar consumes canonical lexical concepts rather than
* inventing attribute-specific lexer tokens.
* 
* The canonical parser-level name grammar is:
* 
* grammar/core/names.g4
* 
* and the qualified-reference integration layer is:
* 
* grammar/core/qualified-names.g4
* 
* This file MUST NOT redefine:
* 
* IDENTIFIER
* qualifiedName
* keyword rules
* literal lexer rules
* punctuation lexer rules
* 
* ============================================================================
* WHY THIS FILE DOES NOT DEFINE ATTRIBUTE-SPECIFIC TOKENS
* ============================================================================
* 
* The previous implementation used conceptual tokens such as:
* 
* ATTRIBUTE_START
* ATTRIBUTE_NAMESPACE_SEPARATOR
* ATTRIBUTE_LPAREN
* ATTRIBUTE_RPAREN
* ATTRIBUTE_COMMA
* ATTRIBUTE_ASSIGN
* ATTRIBUTE_LBRACKET
* ATTRIBUTE_RBRACKET
* ATTRIBUTE_LBRACE
* ATTRIBUTE_RBRACE
* 
* Those are not independently meaningful lexical concepts.
* 
* An attribute uses ordinary Zamani punctuation:
* 
* @
* ::
* (
* )
* ,
* =
* [
* ]
* {
* }
* 
* Therefore the parser consumes the canonical tokens:
* 
* AT
* DOUBLE_COLON
* LPAREN
* RPAREN
* COMMA
* ASSIGN
* LBRACKET
* RBRACKET
* LBRACE
* RBRACE
* 
* This prevents the attribute grammar from creating a second lexical
* vocabulary.
* 
* ============================================================================
* NAME INTEGRATION
* ============================================================================
* 
* Names are owned by Names.
* 
* This file imports:
* 
* Names
* 
* and reuses:
* 
* identifier
* qualifiedName
* 
* directly.
* 
* It MUST NOT redeclare:
* 
* identifier
* qualifiedName
* 
* A qualified attribute such as:
* 
* @quantum::resource
* 
* is therefore structurally the same name system used elsewhere in Zamani.
* 
* ============================================================================
* ATTRIBUTE VS ANNOTATION
* ============================================================================
* 
* Zamani currently contains both:
* 
* grammar/core/annotations.g4
* 
* and:
* 
* grammar/core/attributes.g4
* 
* They overlap syntactically.
* 
* This file establishes ATTRIBUTE syntax as a reusable metadata attachment
* interface while preserving the existing annotation grammar for compatibility.
* 
* The two concepts MUST NOT silently acquire different identifier, name,
* punctuation, or literal grammars.
* 
* Their semantic distinction, if retained, belongs to the AST/specification
* layer:
* 
* annotation
*     -> language-defined source annotation concept
* 
* attribute
*     -> structured metadata attached to a construct
* 
* If the language specification eventually declares them semantically
* identical, a later compatibility migration may make one a compatibility
* wrapper around the other. That migration must not create a third syntax.
* 
* ============================================================================
* ATTRIBUTE ATTACHMENT
* ============================================================================
* 
* This file defines the reusable attribute itself.
* 
* It deliberately does NOT decide which constructs permit attributes.
* 
* Examples of potential consumers:
* 
* declaration
* type
* function
* parameter
* module
* effect
* memory declaration
* concurrent task
* classical operation
* quantum operation
* hybrid boundary
* HDL module
* hardware description
* resource requirement
* compiler configuration
* execution policy
* interoperability declaration
* dialect declaration
* 
* The owning grammar decides whether:
* 
* optionalAttributePrefix
* 
* is permitted at a particular attachment point.
* 
* ============================================================================
* POCO-REAF CONTRACT
* ============================================================================
* 
* Attribute syntax is independent of execution hardware.
* 
* There is NO grammar-level limit on:
* 
* - attribute count;
* - attribute-name length;
* - qualification depth;
* - argument count;
* - list size;
* - map size;
* - nested attribute depth;
* - declaration count;
* - module count;
* - resource count;
* - capability count;
* - device count;
* - CPU count;
* - GPU count;
* - FPGA count;
* - QPU count;
* - qubit count;
* - node count;
* - tensor rank;
* - memory size.
* 
* Repetition is represented structurally using ANTLR repetition operators.
* 
* Any implementation limit needed to protect parser memory, stack usage,
* source size, diagnostics, compilation time, or runtime resources is an
* implementation/resource policy, NOT a language-level grammar limit.
* 
* ============================================================================
* HARD-CODING PROHIBITION
* ============================================================================
* 
* This grammar MUST NOT contain:
* 
* MAX_ATTRIBUTES
* MAX_ATTRIBUTE_ARGUMENTS
* MAX_ATTRIBUTE_DEPTH
* MAX_ATTRIBUTE_NAME_LENGTH
* MAX_NAMESPACE_DEPTH
* MAX_QUBITS
* MAX_CPUS
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_QPUS
* MAX_NODES
* MAX_MEMORY
* MAX_TENSOR_RANK
* MAX_DEVICES
* 
* It also MUST NOT create fixed domain-specific attribute categories such as:
* 
* QuantumAttribute
* GPUAttribute
* FPGAAttribute
* QECAttribute
* ZQNAttribute
* 
* merely to enumerate known concepts.
* 
* New attribute namespaces must remain possible without editing this grammar.
* 
* ============================================================================
* SEMANTIC REQUIREMENT / CAPABILITY / PREFERENCE / IMPLEMENTATION SEPARATION
* ============================================================================
* 
* Attributes may eventually carry semantic information such as:
* 
* requirement
* capability
* preference
* hint
* constraint
* policy
* metadata
* 
* This grammar does not decide which category an attribute belongs to.
* 
* For example:
* 
* @requires(qubits = n)
* 
* can describe a semantic requirement.
* 
* It MUST NOT be interpreted by this grammar as:
* 
* use physical qubits 0..n-1
* 
* Likewise:
* 
* @prefer(accelerator = quantum)
* 
* is not a physical placement decision.
* 
* Physical placement, routing, scheduling, calibration, topology, device
* selection and deployment remain downstream responsibilities.
* 
* ============================================================================
* ATTRIBUTE VALUES ARE STRUCTURAL
* ============================================================================
* 
* Attribute values are intentionally restricted to static structural forms:
* 
* literals
* names
* lists
* maps
* nested attributes
* 
* This prevents attributes from silently becoming a second runtime expression
* language.
* 
* If Zamani requires arbitrary compile-time expressions in attributes, that
* capability must be introduced through an explicit compile-time-expression
* contract and integrated with the canonical expression grammar. It MUST NOT
* be approximated by inventing a second expression grammar here.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The frontend AST should preserve the complete source structure.
* 
* Recommended conceptual representation:
* 
* Attribute
*     name
*     arguments[]
*     source_span
* 
* AttributeArgument
*     positional | named
*     value
*     source_span
* 
* AttributeValue
*     literal
*     name
*     list
*     map
*     nested_attribute
*     source_span
* 
* AttributeMapEntry
*     key
*     value
*     source_span
* 
* The exact Rust AST type names belong to:
* 
* src/frontend/ast/
* 
* This grammar MUST NOT depend on those Rust types.
* 
* Source spans MUST survive from lexer/parser into the AST so diagnostics,
* formatting, IDE tooling, provenance and semantic validation can identify
* the exact attribute and value that produced an error.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* After parsing, semantic analysis is responsible for:
* 
* - resolving the attribute namespace;
* - identifying the attribute definition;
* - validating attachment context;
* - validating argument names;
* - validating argument cardinality;
* - validating value types;
* - detecting duplicate keys where prohibited;
* - resolving referenced names;
* - checking capability requirements;
* - checking resource requirements;
* - checking effect requirements;
* - checking domain-specific rules;
* - deciding whether an attribute is portable;
* - deciding whether an attribute is a compiler hint;
* - deciding whether an attribute is a hard requirement;
* - rejecting unsupported target-specific semantics.
* 
* The parser MUST NOT perform those operations.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* Attributes are metadata, not an independent universal IR.
* 
* Depending on semantic meaning, validated attributes may become:
* 
* - AST metadata;
* - semantic annotations;
* - capability requirements;
* - resource requirements;
* - optimization metadata;
* - provenance;
* - verification metadata;
* - interoperability metadata;
* - execution policy metadata;
* - quantum operation metadata;
* - HDL/hardware intent metadata.
* 
* A quantum-related attribute may eventually contribute metadata to the
* canonical quantum semantic representation / quantum::ir.
* 
* This grammar MUST NOT define a second quantum IR.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* Quantum attributes may use ordinary generic syntax:
* 
* @quantum::logical
* @quantum::resource(...)
* @quantum::capability(...)
* @qec::require(...)
* @noise::budget(...)
* 
* The parser does not determine whether those attributes are meaningful.
* 
* Semantic quantum processing remains downstream:
* 
* attribute syntax
*     |
*     v
* frontend AST
*     |
*     v
* semantic validation
*     |
*     v
* quantum semantic representation
*     |
*     v
* quantum::ir
*     |
*     +--> optimization
*     +--> QEC
*     +--> ZQN
*     +--> routing
*     +--> scheduling
*     +--> HAL
* 
* No physical qubit identifier, device topology, calibration value, or QPU
* selection is implied merely by an attribute name.
* 
* ============================================================================
* CLASSICAL / HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* The same syntax can represent metadata for:
* 
* classical computation
* numerical computation
* tensor computation
* HDL
* hardware/software co-design
* accelerator intent
* distributed execution
* networking
* AI/ML
* security
* interoperability
* 
* The attribute grammar remains domain-neutral.
* 
* Domain grammars define attachment points and semantic validators, not new
* general-purpose attribute syntaxes.
* 
* ============================================================================
* NESTED VALUES
* ============================================================================
* 
* Nested attributes are supported:
* 
* @outer(@inner(...))
* 
* This is structural syntax only.
* 
* Whether nested attributes are legal for a particular attribute is semantic.
* 
* ============================================================================
* TRAILING COMMAS
* ============================================================================
* 
* Trailing commas are accepted in attribute argument lists, list values and
* map values:
* 
* @example(
*     first,
*     second,
* )
* 
* @example([
*     first,
*     second,
* ])
* 
* @example({
*     first = value,
*     second = value,
* })
* 
* This is source-compatible with generated code and reduces source churn.
* 
* ============================================================================
* DUPLICATE MAP KEYS
* ============================================================================
* 
* This grammar permits duplicate keys structurally:
* 
* @example({
*     mode = "a",
*     mode = "b",
* })
* 
* Whether duplicates are legal is a semantic/schema decision.
* 
* The parser MUST preserve the entries rather than silently overwriting one.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* This grammar contains:
* 
* - no semantic predicates;
* - no actions;
* - no runtime callbacks;
* - no filesystem access;
* - no network access;
* - no randomness;
* - no hardware discovery;
* - no target selection.
* 
* Given the same token stream, parsing is deterministic.
* 
* ============================================================================
* SECURITY
* ============================================================================
* 
* Attribute syntax is untrusted source input.
* 
* The grammar itself performs no:
* 
* - command execution;
* - file access;
* - network access;
* - environment access;
* - secret access;
* - dynamic code execution.
* 
* Attribute values MUST remain inert syntax until explicitly validated by
* downstream semantic/compiler stages.
* 
* Attribute names and values MUST NOT be treated as trusted authorization
* decisions merely because they parsed successfully.
* 
* ============================================================================
* COMPATIBILITY
* ============================================================================
* 
* Existing specialized attribute grammars such as:
* 
* grammar/modules/module-attributes.g4
* 
* may continue to exist as context-specific compatibility grammars.
* 
* They SHOULD delegate generic attribute structure to this canonical core
* grammar rather than creating incompatible generic attribute syntax.
* 
* Context-specific grammars may narrow:
* 
* allowed names
* allowed values
* attachment points
* cardinality
* 
* but MUST NOT redefine the general "@name(...)" syntax.
* 
* ============================================================================
* RELATIONSHIP TO core/annotations.g4
* ============================================================================
* 
* "core/annotations.g4" currently provides a closely related source-level
* construct.
* 
* It must not become a third independent identifier/literal/punctuation
* system.
* 
* The long-term integration contract is:
* 
* canonical lexer
*       |
*       +------------------+
*       |                  |
*       v                  v
* annotation syntax   attribute syntax
*       |                  |
*       +--------+---------+
*                |
*                v
*         shared Names
*                |
*                v
*         shared AST/value
*             contracts
* 
* If the language specification later determines that annotations and
* attributes are semantically identical, one may become a compatibility
* wrapper around the other without changing source syntax.
* 
* ============================================================================
* RUST CONTRACT
* ============================================================================
* 
* This file contains no Rust implementation code.
* 
* Generated parser integration MUST:
* 
* - compile with Rust 1.97;
* - compile with Rust 1.97.1;
* - remain compatible with the repository's ANTLR Rust integration;
* - require no `unsafe`;
* - preserve source spans;
* - preserve deterministic parsing;
* - avoid machine-specific assumptions.
* 
* Repository Rust safety policy remains downstream of this grammar.
* 
* ============================================================================
* PUBLIC RULE CONTRACT
* ============================================================================
* 
* The following rules are intentionally public/reusable:
* 
* attribute
* attributeList
* optionalAttributes
* attributePrefix
* optionalAttributePrefix
* attributeName
* qualifiedAttributeName
* attributeArguments
* attributeArgumentList
* attributeArgument
* attributeNamedArgument
* attributePositionalArgument
* attributeArgumentName
* attributeValue
* attributeScalarValue
* attributeNameValue
* attributeListValue
* attributeValueList
* attributeMapValue
* attributeMapEntryList
* attributeMapEntry
* attributeNestedValue
* 
* Downstream grammars SHOULD consume these rules rather than reproducing
* attribute syntax.
* 
* ============================================================================
  */

parser grammar Attributes;

options {
tokenVocab = ZamaniTokens;
}

import Names;

/*

* ============================================================================
* ATTRIBUTE ATTACHMENT
* ============================================================================
  */

/**

* One attribute.
* 
* Canonical forms:
* 
* @name
* @name(...)
* @namespace::name
* @namespace::name(...)

*/
attribute
: AT attributeName attributeArguments?
;

/**

* One or more attributes.
* 
* There is no finite language-level maximum.
  */
  attributeList
  : attribute+
  ;

/**

* Zero or more attributes.
  /
  optionalAttributes
  : attribute
  ;

/**

* Required attribute prefix.
* 
* Useful when a consuming construct has already decided that at least one
* attribute must be present.
  */
  attributePrefix
  : attributeList
  ;

/**

* Optional attribute prefix.
  */
  optionalAttributePrefix
  : optionalAttributes
  ;

/*

* ============================================================================
* ATTRIBUTE NAMES
* ============================================================================
  */

/**

* Attribute names use the canonical Zamani name system.
* 
* Examples:
* 
* @inline
* @quantum
* @quantum::logical
* @hardware::capability

*/
attributeName
: qualifiedAttributeName
;

/**

* Qualified attribute name.
* 
* "qualifiedName" remains owned by core/names.g4.
  */
  qualifiedAttributeName
  : qualifiedName
  ;

/*

* ============================================================================
* ATTRIBUTE ARGUMENTS
* ============================================================================
  */

/**

* Parenthesized attribute argument list.
* 
* Empty argument lists are legal:
* 
* @attribute()

*/
attributeArguments
: LPAREN attributeArgumentList? RPAREN
;

/**

* Comma-separated attribute arguments.
* 
* Trailing comma is accepted.
  /
  attributeArgumentList
  : attributeArgument
  (COMMA attributeArgument)
  COMMA?
  ;

/**

* An attribute argument is either named or positional.
* 
* Examples:
* 
* @attribute(value)
* 
* @attribute(name = value)

*/
attributeArgument
: attributeNamedArgument
| attributePositionalArgument
;

/**

* Named attribute argument.
  */
  attributeNamedArgument
  : attributeArgumentName ASSIGN attributeValue
  ;

/**

* Positional attribute argument.
  */
  attributePositionalArgument
  : attributeValue
  ;

/**

* Named argument keys are local identifiers.
* 
* The attribute itself may be qualified, but an argument key is not a second
* namespace-resolution construct.
  */
  attributeArgumentName
  : identifier
  ;

/*

* ============================================================================
* ATTRIBUTE VALUES
* ============================================================================
  */

/**

* Generic attribute value.
* 
* Attribute values intentionally remain structural and non-executable.
  */
  attributeValue
  : attributeScalarValue
  | attributeNameValue
  | attributeListValue
  | attributeMapValue
  | attributeNestedValue
  ;

/**

* Scalar attribute values.
* 
* The exact lexical spelling is owned by ZamaniTokens.
* 
* K_TRUE/K_FALSE/K_NULL/K_NIL are keyword tokens from the canonical lexer.
  */
  attributeScalarValue
  : INTEGER_LITERAL
  | FLOAT_LITERAL
  | STRING_LITERAL
  | CHARACTER_LITERAL
  | K_TRUE
  | K_FALSE
  | K_NULL
  | K_NIL
  ;

/**

* A symbolic attribute value.
* 
* Examples:
* 
* @requires(capability = quantum::mid_circuit_measurement)
* 
* @target(model = hardware::accelerator)
* 
* The name is not resolved by this grammar.
  */
  attributeNameValue
  : qualifiedName
  ;

/*

* ============================================================================
* STRUCTURED LIST VALUES
* ============================================================================
  */

/**

* Ordered structured attribute value.
* 
* Examples:
* 
* @targets([cpu, gpu, qpu])
* 
* @features([
*     quantum::measurement,
*     quantum::control,
* ])

*/
attributeListValue
: LBRACKET attributeValueList? RBRACKET
;

/**

* Elements of an attribute list.
* 
* No finite cardinality is imposed.
  /
  attributeValueList
  : attributeValue
  (COMMA attributeValue)
  COMMA?
  ;

/*

* ============================================================================
* STRUCTURED MAP VALUES
* ============================================================================
  */

/**

* Key/value structured attribute.
* 
* Example:
* 
* @resource({
*     memory = 4GiB,
*     accelerator = quantum,
* })
* 
* The grammar only preserves structure. Semantic validation determines
* whether the keys and values are legal.
  */
  attributeMapValue
  : LBRACE attributeMapEntryList? RBRACE
  ;

/**

* Map entries.
* 
* Duplicate keys are intentionally preserved for semantic validation.
  /
  attributeMapEntryList
  : attributeMapEntry
  (COMMA attributeMapEntry)
  COMMA?
  ;

/**

* One map entry.
* 
* Keys may be ordinary identifiers or string keys.
* 
* String keys are useful for externally defined schemas and future dialects.
  */
  attributeMapEntry
  : attributeMapKey ASSIGN attributeValue
  ;

/**

* Attribute map key.
  */
  attributeMapKey
  : identifier
  | STRING_LITERAL
  ;

/*

* ============================================================================
* NESTED ATTRIBUTES
* ============================================================================
  */

/**

* Nested attribute value.
* 
* Examples:
* 
* @outer(@inner)
* 
* @outer(@quantum::resource(...))

*/
attributeNestedValue
: attribute
;

/*

* ============================================================================
* OPTIONAL VALUE WRAPPERS
* ============================================================================
* 
* These wrappers are useful to downstream grammars and avoid repeated
* nullable expressions.
  */

/**

* Optional attribute arguments.
  */
  optionalAttributeArguments
  : attributeArguments?
  ;

/**

* Optional attribute name.
  */
  optionalAttributeName
  : attributeName?
  ;

/**

* Optional attribute value.
  */
  optionalAttributeValue
  : attributeValue?
  ;

/*

* ============================================================================
* INTEGRATION WRAPPERS
* ============================================================================
* 
* These rules intentionally carry no additional semantics.
* 
* They provide stable names for downstream grammar components without forcing
* those components to know the internal structure of the generic attribute
* grammar.
  */

/**

* Generic metadata attribute.
  */
  metadataAttribute
  : attribute
  ;

/**

* Generic compiler attribute.
* 
* The semantic layer determines whether the attribute is actually a compiler
* directive, hint, requirement, or ordinary metadata.
  */
  compilerAttribute
  : attribute
  ;

/**

* Generic resource/capability attribute.
  */
  resourceAttribute
  : attribute
  ;

/**

* Generic quantum attribute.
  */
  quantumAttribute
  : attribute
  ;

/**

* Generic HDL/hardware attribute.
  */
  hardwareAttribute
  : attribute
  ;

/**

* Generic interoperability attribute.
  */
  interoperabilityAttribute
  : attribute
  ;

/*

* ============================================================================
* COMPLETION CONTRACT
* ============================================================================
* 
* This grammar is complete when:
* 
* [x] The canonical lexer vocabulary is used.
* [x] "ZamaniTokens" is used instead of the obsolete "ZamaniLexer" reference.
* [x] Generic punctuation is reused.
* [x] "AT" is reused rather than creating ATTRIBUTE_START.
* [x] "DOUBLE_COLON" is reused through "Names".
* [x] "IDENTIFIER" is reused through "Names".
* [x] Qualified names are owned by "Names".
* [x] No identifier grammar is duplicated.
* [x] No literal lexer rules are duplicated.
* [x] No expression grammar is duplicated.
* [x] Attribute arguments support positional values.
* [x] Attribute arguments support named values.
* [x] Empty argument lists are legal.
* [x] Trailing commas are supported.
* [x] Structured list values are supported.
* [x] Structured map values are supported.
* [x] Nested attributes are supported.
* [x] Duplicate map keys remain available for semantic diagnostics.
* [x] Attribute names remain extensible.
* [x] Domain-specific attribute names are not hard-coded.
* [x] No hardware resource limit is encoded.
* [x] No quantum resource limit is encoded.
* [x] No fixed namespace depth is encoded.
* [x] No fixed attribute count is encoded.
* [x] No fixed argument count is encoded.
* [x] No fixed collection size is encoded.
* [x] No target selection is performed.
* [x] No physical placement is performed.
* [x] No QEC implementation is performed.
* [x] No ZQN implementation is performed.
* [x] No routing/scheduling is performed.
* [x] "quantum::ir" remains downstream.
* [x] AST mapping is specified independently of this grammar.
* [x] Semantic validation remains downstream.
* [x] Source spans remain preservable.
* [x] The grammar is deterministic.
* [x] The grammar contains no Rust or unsafe code.
* [x] Rust 1.97 / 1.97.1 integration is specified.
* 
* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* Positive syntax tests:
* 
* @inline
* @deprecated
* @quantum
* @quantum::logical
* @hardware::capability
* 
* @inline()
* @resource(memory = 4GiB)
* @requires(capability = quantum::measurement)
* @compile(optimize = "portable")
* 
* @features([
*     quantum::measurement,
*     quantum::reset,
*     quantum::control,
* ])
* 
* @resource({
*     memory = "required",
*     accelerator = quantum,
* })
* 
* @outer(@inner)
* 
* @outer(
*     @inner,
*     mode = "portable",
* )
* 
* Negative syntax tests:
* 
* @
* @::name
* @name::
* @:::
* @name(=value)
* @name(key=)
* @name([)
* @name({)
* @name([value,)
* 
* Boundary tests:
* 
* @a
* @very_long_attribute_name
* @a::b::c::d::e
* @a(v)
* @a(v1, v2, v3, ...)
* @a([v1, v2, ...])
* @a({k1=v1, k2=v2, ...})
* deeply nested attribute structures
* 
* Scalability tests:
* 
* - arbitrarily many attributes;
* - arbitrarily many arguments;
* - arbitrarily many list elements;
* - arbitrarily many map entries;
* - arbitrarily deep qualified names;
* - arbitrarily deep nested attribute structures;
* - large source programs;
* - tiny source programs.
* 
* The tests MUST demonstrate that no language-level finite resource constant
* is required.
* 
* Compatibility tests:
* 
* - module attributes;
* - declaration attributes;
* - function attributes;
* - quantum attributes;
* - HDL attributes;
* - hardware attributes;
* - resource/capability attributes;
* - dialect attributes;
* - interoperability metadata.
* 
* AST tests MUST verify:
* 
* attribute name
* argument ordering
* named-vs-positional distinction
* nested values
* list ordering
* map entry ordering
* duplicate map keys
* source spans
* 
* Semantic tests MUST verify that syntax alone does not grant:
* 
* capability
* authorization
* hardware access
* resource availability
* quantum-device access
* compiler privileges
* runtime privileges
* 
* ============================================================================
  */