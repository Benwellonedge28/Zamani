/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/core/attributes.g4
 *
 * GRAMMAR
 * -------
 * Attributes
 *
 * STATUS
 * ------
 * CANONICAL / PRODUCTION
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * RUST BASELINE
 * -------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 *
 * SAFETY
 * ------
 * This grammar contains no embedded Rust actions, semantic predicates,
 * filesystem access, network access, process execution, runtime callbacks,
 * or unsafe code.
 *
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the canonical parser-level owner of generic SOURCE-LEVEL
 * ATTRIBUTE syntax.
 *
 * An attribute is structured metadata attached to another source construct.
 *
 * Canonical forms include:
 *
 *     @name
 *     @name(...)
 *     @namespace::name
 *     @namespace::name(...)
 *
 * Attributes are intentionally generic.
 *
 * This grammar defines:
 *
 *     WHAT AN ATTRIBUTE LOOKS LIKE
 *
 * It does NOT define:
 *
 *     WHAT AN ATTRIBUTE MEANS
 *
 * Attribute meaning is determined by downstream semantic analysis,
 * attribute registries, specifications, capability analysis, resource
 * analysis, policy analysis, effect analysis, domain semantics, and tooling.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The attribute pipeline is:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       |-- AT
 *       |-- IDENTIFIER
 *       |-- DOUBLE_COLON
 *       |-- LPAREN / RPAREN
 *       |-- LBRACKET / RBRACKET
 *       |-- LBRACE / RBRACE
 *       |-- COMMA
 *       |-- ASSIGN
 *       |-- literal tokens
 *       |
 *       v
 *     Names
 *       |
 *       v
 *     Attributes
 *       |
 *       +--> declarations
 *       +--> types
 *       +--> functions
 *       +--> modules
 *       +--> effects
 *       +--> memory
 *       +--> concurrency
 *       +--> classical
 *       +--> quantum
 *       +--> hybrid
 *       +--> HDL
 *       +--> hardware
 *       +--> distributed
 *       +--> AI
 *       +--> data
 *       +--> networking
 *       +--> security
 *       +--> resources
 *       +--> compilation
 *       +--> execution
 *       +--> interoperability
 *       +--> dialects
 *       +--> macros
 *       +--> metaprogramming
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> type checking
 *       +--> effect checking
 *       +--> capability checking
 *       +--> resource checking
 *       +--> contract checking
 *       +--> policy checking
 *       +--> provenance
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +--> classical representation
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed representation
 *       +--> other domain representations
 *       |
 *       v
 *     optimization / lowering / routing / scheduling
 *       |
 *       v
 *     target realization
 *
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     - generic attribute syntax;
 *     - attribute attachment syntax;
 *     - attribute lists;
 *     - optional attribute prefixes;
 *     - attribute names;
 *     - qualified attribute names;
 *     - attribute argument lists;
 *     - named attribute arguments;
 *     - positional attribute arguments;
 *     - attribute argument names;
 *     - structural attribute values;
 *     - scalar attribute values;
 *     - symbolic attribute values;
 *     - list attribute values;
 *     - map attribute values;
 *     - map entries;
 *     - nested attribute values;
 *     - reusable attribute grammar wrappers.
 *
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     - lexical token definitions;
 *     - identifier recognition;
 *     - Unicode identifier classification;
 *     - keyword spelling;
 *     - literal lexical spelling;
 *     - operators;
 *     - punctuation definitions;
 *     - comments;
 *     - whitespace;
 *     - paths;
 *     - URLs;
 *     - module resolution;
 *     - namespace resolution;
 *     - symbol resolution;
 *     - type checking;
 *     - effect checking;
 *     - capability resolution;
 *     - resource resolution;
 *     - policy interpretation;
 *     - contract interpretation;
 *     - provenance records;
 *     - runtime evaluation;
 *     - compiler optimization;
 *     - target selection;
 *     - device discovery;
 *     - physical placement;
 *     - quantum routing;
 *     - quantum scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - HDL synthesis;
 *     - hardware realization;
 *     - deployment.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *
 * The parser-facing lexical vocabulary is the canonical ZamaniLexer.
 *
 * This file therefore uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It does NOT consume ZamaniTokens directly.
 *
 *
 * EXPORTS
 * -------
 *
 *     attribute
 *     attributeList
 *     optionalAttributes
 *     attributePrefix
 *     optionalAttributePrefix
 *
 *     attributeName
 *     qualifiedAttributeName
 *
 *     attributeArguments
 *     optionalAttributeArguments
 *     attributeArgumentList
 *     attributeArgument
 *     attributeNamedArgument
 *     attributePositionalArgument
 *     attributeArgumentName
 *
 *     attributeValue
 *     optionalAttributeValue
 *     attributeScalarValue
 *     attributeNameValue
 *     attributeListValue
 *     attributeValueList
 *     attributeMapValue
 *     attributeMapEntryList
 *     attributeMapEntry
 *     attributeMapKey
 *     attributeNestedValue
 *
 *
 * CONSUMED_BY
 * -----------
 *
 * Any grammar that attaches metadata to a source construct may consume the
 * exported attribute rules.
 *
 * Typical consumers include:
 *
 *     grammar/declarations/
 *     grammar/types/
 *     grammar/functions/
 *     grammar/modules/
 *     grammar/effects/
 *     grammar/memory/
 *     grammar/concurrency/
 *     grammar/classical/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/distributed/
 *     grammar/ai/
 *     grammar/data/
 *     grammar/networking/
 *     grammar/security/
 *     grammar/resources/
 *     grammar/compile/
 *     grammar/execution/
 *     grammar/interoperability/
 *     grammar/dialects/
 *     grammar/macros/
 *     grammar/metaprogramming/
 *
 * Consumers MUST reuse these rules instead of recreating generic attribute
 * syntax.
 *
 *
 * AST_OWNER
 * ---------
 *
 * The grammar creates ANTLR parser contexts only.
 *
 * The domain-neutral frontend AST owns the actual attribute representation.
 *
 * The AST should preserve at least:
 *
 *     - source span;
 *     - source ordering;
 *     - attribute name;
 *     - qualification;
 *     - argument ordering;
 *     - positional/named distinction;
 *     - complete value structure;
 *     - duplicate map entries;
 *     - nested attribute structure.
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Semantic analysis owns:
 *
 *     - attribute definition lookup;
 *     - namespace resolution;
 *     - attachment validation;
 *     - argument validation;
 *     - value validation;
 *     - duplicate-key policy;
 *     - type validation;
 *     - capability interpretation;
 *     - resource interpretation;
 *     - effect interpretation;
 *     - policy interpretation;
 *     - contract interpretation;
 *     - portability analysis;
 *     - target-specific validation.
 *
 *
 * IR_OWNER
 * --------
 *
 * This grammar owns NO IR.
 *
 * Attribute information may later become:
 *
 *     - semantic metadata;
 *     - capability requirements;
 *     - resource requirements;
 *     - optimization metadata;
 *     - verification metadata;
 *     - provenance;
 *     - execution policy;
 *     - interoperability metadata;
 *     - quantum semantic metadata;
 *     - HDL/hardware intent.
 *
 * Attributes MUST NOT become a separate universal IR.
 *
 *
 * TEST_OWNER
 * ----------
 *
 * Recommended tests:
 *
 *     grammar/tests/core/attributes/
 *
 * Tests should also participate in:
 *
 *     grammar/tests/lexical/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/scalability/
 *     grammar/tests/compatibility/
 *     grammar/tests/boundary/
 *
 *
 * SPEC_OWNER
 * ----------
 *
 * Normative language specification:
 *
 *     grammar/specification/
 *
 * Machine-checkable contracts:
 *
 *     grammar/spec/
 *
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This parser grammar defines NO lexer rules.
 *
 * Required lexical concepts are supplied by the canonical lexer:
 *
 *     AT
 *     IDENTIFIER
 *     DOUBLE_COLON
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     ASSIGN
 *
 * Literal tokens consumed here are also supplied by the canonical lexer.
 *
 * This file MUST NOT define:
 *
 *     IDENTIFIER
 *     AT
 *     DOUBLE_COLON
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     ASSIGN
 *
 * or any other lexer rule.
 *
 *
 * ============================================================================
 * NAME CONTRACT
 * ============================================================================
 *
 * Names are owned by:
 *
 *     grammar/core/names.g4
 *
 * This grammar imports Names and reuses:
 *
 *     identifier
 *     qualifiedName
 *
 * directly.
 *
 * Attribute names therefore participate in the same name system as:
 *
 *     declarations;
 *     functions;
 *     modules;
 *     types;
 *     resources;
 *     capabilities;
 *     policies;
 *     quantum objects;
 *     hardware abstractions;
 *     data objects;
 *     AI models;
 *     dialect symbols.
 *
 * No separate attribute identifier system exists.
 *
 *
 * ============================================================================
 * ATTRIBUTE / ANNOTATION BOUNDARY
 * ============================================================================
 *
 * The repository also contains:
 *
 *     grammar/core/annotations.g4
 *
 * Attribute and annotation syntax MUST NOT evolve into two incompatible
 * generic metadata syntaxes.
 *
 * This file owns the generic ATTRIBUTE parser structure.
 *
 * Annotation syntax remains owned by its designated grammar.
 *
 * If the language specification eventually declares annotations and attributes
 * semantically equivalent, one may become a compatibility layer around the
 * other without creating another generic syntax.
 *
 * This file does not attempt to resolve that semantic policy.
 *
 *
 * ============================================================================
 * ATTRIBUTE ATTACHMENT
 * ============================================================================
 *
 * An attribute consists of:
 *
 *     AT
 *     attribute name
 *     optional argument list
 *
 * Examples:
 *
 *     @inline
 *     @pure
 *     @quantum::logical
 *     @hardware::accelerator
 *     @resource(...)
 *     @policy(...)
 *     @provenance(...)
 *
 * Attribute meaning is NOT inferred by this grammar.
 *
 * ============================================================================
 * ATTRIBUTE LIST
 * ============================================================================
 *
 * A non-empty attribute list contains one or more attributes.
 *
 * An optional attribute prefix contains zero or more attributes.
 *
 * This distinction is intentional:
 *
 *     attributeList
 *         -> one or more
 *
 *     optionalAttributes
 *         -> zero or more
 *
 * The previous implementation incorrectly defined `optionalAttributes` as a
 * single mandatory attribute. That is corrected here.
 *
 *
 * ============================================================================
 * ARGUMENT MODEL
 * ============================================================================
 *
 * Attributes support:
 *
 *     positional arguments
 *     named arguments
 *
 * Examples:
 *
 *     @example(value)
 *
 *     @example(first, second)
 *
 *     @example(mode = "fast")
 *
 *     @example(mode = "fast", level = 2)
 *
 * Mixed positional and named arguments are syntactically representable.
 *
 * Whether a particular attribute permits mixing, requires ordering, or
 * requires uniqueness is a semantic/schema rule.
 *
 *
 * ============================================================================
 * ATTRIBUTE VALUES
 * ============================================================================
 *
 * Attribute values are intentionally structural.
 *
 * Supported value classes are:
 *
 *     scalar literal
 *     symbolic name
 *     list
 *     map
 *     nested attribute
 *
 * This prevents attributes from becoming an accidental second runtime
 * expression language.
 *
 * Arbitrary expressions are therefore NOT accepted here.
 *
 * If compile-time expressions are eventually permitted in attributes, they
 * must be introduced through an explicitly specified compile-time expression
 * boundary owned by the canonical expression/metaprogramming subsystem.
 *
 * This grammar must not independently reproduce expression precedence.
 *
 *
 * ============================================================================
 * SCALAR VALUES
 * ============================================================================
 *
 * Scalar lexical values currently consumed by this grammar are:
 *
 *     INTEGER_LITERAL
 *     FLOAT_LITERAL
 *     STRING_LITERAL
 *     CHARACTER_LITERAL
 *     TRUE
 *     FALSE
 *
 * The lexer owns these token definitions.
 *
 * Numeric literals remain source-level values and do not imply:
 *
 *     integer width;
 *     floating-point width;
 *     register width;
 *     SIMD width;
 *     GPU width;
 *     accelerator format;
 *     target ABI.
 *
 * Boolean literals do not imply a particular machine representation.
 *
 *
 * ============================================================================
 * SYMBOLIC VALUES
 * ============================================================================
 *
 * A symbolic attribute value is a canonical qualified name.
 *
 * Examples:
 *
 *     @requires(capability = quantum::measurement)
 *     @target(model = hardware::accelerator)
 *     @policy(name = execution::recovery)
 *
 * This grammar does not resolve the name.
 *
 *
 * ============================================================================
 * STRUCTURED LIST VALUES
 * ============================================================================
 *
 * Lists support arbitrary source cardinality.
 *
 * Examples:
 *
 *     @targets([cpu, gpu, qpu])
 *
 *     @features([
 *         quantum::measurement,
 *         quantum::control,
 *         tensor::compute,
 *     ])
 *
 * A list may contain nested values.
 *
 * Empty lists are syntactically valid.
 *
 * Trailing commas are accepted.
 *
 *
 * ============================================================================
 * STRUCTURED MAP VALUES
 * ============================================================================
 *
 * Maps support arbitrary source cardinality.
 *
 * Examples:
 *
 *     @resource({
 *         memory = required_memory,
 *         capability = tensor::compute,
 *     })
 *
 *     @policy({
 *         fallback = execution::recover,
 *         mode = "deterministic",
 *     })
 *
 * Map keys may be:
 *
 *     identifier
 *     string literal
 *
 * Duplicate keys are structurally preserved.
 *
 * Semantic validation determines whether duplicates are permitted.
 *
 * The parser MUST NOT silently overwrite an earlier entry.
 *
 *
 * ============================================================================
 * NESTED ATTRIBUTES
 * ============================================================================
 *
 * Nested attributes are structural values.
 *
 * Examples:
 *
 *     @outer(@inner)
 *
 *     @outer(@quantum::resource(...))
 *
 * Whether a particular attribute permits nested attributes is a semantic
 * attribute-schema decision.
 *
 *
 * ============================================================================
 * TRAILING COMMA CONTRACT
 * ============================================================================
 *
 * Trailing commas are accepted in:
 *
 *     attribute argument lists;
 *     attribute list values;
 *     attribute map values.
 *
 * Examples:
 *
 *     @example(
 *         first,
 *         second,
 *     )
 *
 *     @example([
 *         first,
 *         second,
 *     ])
 *
 *     @example({
 *         first = value,
 *         second = value,
 *     })
 *
 * This is syntax only.
 *
 * Semantic validation remains downstream.
 *
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains NO finite language-level limits for:
 *
 *     attribute count;
 *     argument count;
 *     list cardinality;
 *     map cardinality;
 *     qualification depth;
 *     nesting depth;
 *     declaration count;
 *     module count;
 *     function count;
 *     resource count;
 *     capability count;
 *     device count;
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     accelerator count;
 *     QPU count;
 *     qubit count;
 *     node count;
 *     tensor rank;
 *     memory capacity.
 *
 * ANTLR repetition operators express unbounded language structure.
 *
 * Actual implementation limits caused by:
 *
 *     available memory;
 *     parser implementation;
 *     source size;
 *     compiler resources;
 *     diagnostic resources;
 *     operating-system constraints;
 *     target resources;
 *
 * are implementation/resource concerns rather than language-level ceilings.
 *
 * A finite implementation limit MUST NOT be encoded into this grammar as a
 * semantic restriction.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * FORBIDDEN
 * ---------
 *
 * This file MUST NOT contain machine/domain capacity constants such as:
 *
 *     MAX_ATTRIBUTES
 *     MAX_ATTRIBUTE_ARGUMENTS
 *     MAX_ATTRIBUTE_DEPTH
 *     MAX_ATTRIBUTE_NAME_LENGTH
 *     MAX_NAMESPACE_DEPTH
 *     MAX_LIST_LENGTH
 *     MAX_MAP_ENTRIES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *
 * It MUST NOT define fixed domain categories such as:
 *
 *     QuantumAttribute
 *     GPUAttribute
 *     FPGAAttribute
 *     QECAttribute
 *     HardwareAttribute
 *
 * merely to enumerate current technologies.
 *
 * New namespaces and attribute names must remain possible without changing
 * this generic attribute grammar.
 *
 *
 * ============================================================================
 * SEMANTIC SEPARATION
 * ============================================================================
 *
 * Attribute syntax MUST NOT be confused with semantic authority.
 *
 * For example:
 *
 *     @requires(qubits = required_qubits)
 *
 * is syntactically metadata.
 *
 * It does not mean:
 *
 *     select physical qubits;
 *     reserve physical qubits;
 *     choose a QPU;
 *     establish topology;
 *     perform routing;
 *     perform scheduling.
 *
 * Likewise:
 *
 *     @prefer(accelerator = quantum)
 *
 * does not select a physical device.
 *
 * Those decisions belong downstream to resource analysis, capability
 * negotiation, planning, routing, scheduling, lowering, deployment, and
 * runtime systems.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum-related attributes use the same generic attribute syntax.
 *
 * Examples:
 *
 *     @quantum::logical
 *
 *     @quantum::resource(...)
 *
 *     @quantum::capability(...)
 *
 *     @qec::require(...)
 *
 *     @noise::budget(...)
 *
 * This grammar does not determine their meaning.
 *
 * The quantum pipeline remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       +--> optimization
 *       +--> decomposition
 *       +--> routing
 *       +--> scheduling
 *       +--> resilience / QEC
 *       +--> ZQN
 *       +--> HAL
 *       |
 *       v
 *     target realization
 *
 * This grammar MUST NOT introduce:
 *
 *     physical qubit identifiers;
 *     physical topology;
 *     calibration;
 *     QPU selection;
 *     routing decisions;
 *     scheduling decisions;
 *     a quantum gate catalogue.
 *
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical constructs may use attributes for:
 *
 *     optimization intent;
 *     purity;
 *     execution policy;
 *     numerical intent;
 *     tensor intent;
 *     resource requirements;
 *     provenance;
 *     contracts;
 *     verification metadata;
 *     interoperability metadata.
 *
 * The grammar remains domain-neutral.
 *
 *
 * ============================================================================
 * AI / REASONING / LEARNING INTEGRATION
 * ============================================================================
 *
 * Attributes may carry metadata associated with:
 *
 *     reasoning;
 *     inference;
 *     deduction;
 *     knowledge;
 *     learning;
 *     adaptation;
 *     uncertainty;
 *     evidence;
 *     explanation;
 *     decisions;
 *     agents;
 *     neural-symbolic composition;
 *     provenance.
 *
 * This file does NOT define those semantics.
 *
 * It only provides the generic syntax through which such metadata may be
 * attached.
 *
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Contract systems may use attributes for metadata associated with:
 *
 *     requires;
 *     ensures;
 *     invariant;
 *     assume;
 *     guarantee;
 *     property;
 *     evidence;
 *     verification.
 *
 * Attribute syntax does not replace the contract grammar.
 *
 * Contract conditions remain owned by:
 *
 *     grammar/validation/
 *
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Attributes may express metadata concerning:
 *
 *     requirements;
 *     capabilities;
 *     constraints;
 *     preferences;
 *     budgets;
 *     hints;
 *     negotiation.
 *
 * Examples:
 *
 *     @requires(capability = tensor::compute)
 *
 *     @requires(resource = memory)
 *
 *     @prefer(accelerator = quantum)
 *
 * These remain semantic descriptions.
 *
 * They do not directly select or reserve hardware.
 *
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Attributes may describe effect metadata such as:
 *
 *     IO;
 *     network;
 *     mutation;
 *     randomness;
 *     measurement;
 *     foreign;
 *     native;
 *     distributed;
 *     learning;
 *     adaptation;
 *     reflection;
 *     simulation;
 *     code generation.
 *
 * Effect ownership remains in:
 *
 *     grammar/effects/
 *
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Attributes may carry policy metadata concerning:
 *
 *     permissions;
 *     prohibitions;
 *     requirements;
 *     constraints;
 *     preferences;
 *     fallback;
 *     adaptation;
 *     deployment;
 *     execution;
 *     security.
 *
 * Policy interpretation remains downstream.
 *
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Attribute source spans and ordering must be preserved so downstream
 * provenance can record:
 *
 *     source;
 *     containing construct;
 *     attribute;
 *     transformation;
 *     verification;
 *     decision;
 *     evidence.
 *
 * This grammar does not construct provenance records.
 *
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware constructs may consume generic attributes for:
 *
 *     hardware intent;
 *     timing intent;
 *     verification metadata;
 *     synthesis hints;
 *     resource intent;
 *     interface metadata;
 *     accelerator intent.
 *
 * This grammar MUST NOT encode:
 *
 *     fixed bus width;
 *     fixed register width;
 *     fixed device count;
 *     fixed memory capacity;
 *     physical placement;
 *     technology node;
 *     vendor device identity.
 *
 * Hardware semantics remain downstream.
 *
 *
 * ============================================================================
 * DISTRIBUTED / NETWORKING INTEGRATION
 * ============================================================================
 *
 * Attributes may describe:
 *
 *     execution intent;
 *     service metadata;
 *     consistency intent;
 *     reliability intent;
 *     topology requirements;
 *     network capability requirements;
 *     security policy.
 *
 * This grammar does not perform discovery or topology resolution.
 *
 *
 * ============================================================================
 * FFI / ABI INTEGRATION
 * ============================================================================
 *
 * Attributes may annotate:
 *
 *     foreign declarations;
 *     ABI metadata;
 *     calling conventions;
 *     linkage;
 *     data-layout intent;
 *     interoperability requirements.
 *
 * FFI and ABI semantics remain owned by the interoperability subsystem.
 *
 * An attribute MUST NOT itself execute a foreign function.
 *
 *
 * ============================================================================
 * METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Attributes may annotate:
 *
 *     compile-time generation;
 *     reflection;
 *     introspection;
 *     generated declarations;
 *     dialect metadata;
 *     syntax-tree transformations.
 *
 * The attribute grammar remains inert.
 *
 * It does not execute compile-time code.
 *
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Attribute source is untrusted input.
 *
 * Parsing an attribute MUST NOT:
 *
 *     execute commands;
 *     access files;
 *     access the network;
 *     access environment variables;
 *     access secrets;
 *     discover hardware;
 *     execute code;
 *     grant capabilities;
 *     grant permissions.
 *
 * An attribute spelling such as:
 *
 *     @allow(...)
 *
 * is not itself an authorization decision.
 *
 * Security semantics belong to the security/policy subsystem.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no randomness;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime callbacks;
 *     - no hardware discovery;
 *     - no target selection;
 *     - no time-dependent behavior.
 *
 * For the same canonical token stream and parser configuration, the grammar
 * produces equivalent parse-tree structure.
 *
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Structural errors belong to the parser.
 *
 * Examples include:
 *
 *     @
 *     @name(
 *     @name(value
 *     @name(,)
 *     @name(= value)
 *     @name([)
 *     @name({)
 *     @name({ key = })
 *
 * Semantic errors belong downstream.
 *
 * Examples include:
 *
 *     unknown attribute;
 *     attribute not allowed at attachment point;
 *     unknown named argument;
 *     duplicate argument;
 *     duplicate map key when prohibited;
 *     invalid value type;
 *     unsatisfied capability;
 *     unsatisfied resource requirement;
 *     forbidden policy;
 *     invalid effect declaration.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Generic attribute syntax is a language-level interface.
 *
 * Existing consumers must migrate toward these exported rules rather than
 * defining another generic attribute syntax.
 *
 * Context-specific grammars may narrow:
 *
 *     attribute names;
 *     attachment points;
 *     argument schemas;
 *     cardinality;
 *     allowed values.
 *
 * They MUST NOT redefine:
 *
 *     @name
 *     @name(...)
 *     @namespace::name
 *     @namespace::name(...)
 *
 * with a competing generic grammar.
 *
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * Generated parser integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The implementation MUST use safe Rust.
 *
 * This grammar requires no `unsafe`.
 *
 *
 * ============================================================================
 * ATTRIBUTE GRAMMAR
 * ============================================================================
 */

/*
 * ============================================================================
 * ANTLR PARSER DECLARATION
 * ============================================================================
 *
 * The canonical parser-facing token vocabulary is ZamaniLexer.
 *
 * This keeps the parser hierarchy independent from individual lexer component
 * grammars and from the lexical composition grammar.
 * ============================================================================
 */

parser grammar Attributes;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Names is the canonical owner of:
 *
 *     identifier
 *     qualifiedName
 *     name lists
 *     aliases
 *     symbolic references
 *
 * Attribute syntax reuses those rules.
 */

import Names;


/*
 * ============================================================================
 * ATTRIBUTE ATTACHMENT
 * ============================================================================
 */

/**
 * One complete attribute.
 *
 * Examples:
 *
 *     @inline
 *     @inline()
 *     @quantum::logical
 *     @quantum::resource(memory = required_memory)
 */
attribute
    : AT attributeName attributeArguments?
    ;


/**
 * One or more attributes.
 *
 * Examples:
 *
 *     @inline
 *     @inline @pure
 *     @quantum::logical @resource(...)
 *
 * There is no grammar-defined finite cardinality.
 */
attributeList
    : attribute+
    ;


/**
 * Zero or more attributes.
 *
 * This is the genuinely optional form.
 *
 * Empty input is accepted.
 */
optionalAttributes
    : attribute*
    ;


/**
 * Required attribute prefix.
 *
 * At least one attribute is required.
 */
attributePrefix
    : attributeList
    ;


/**
 * Optional attribute prefix.
 *
 * Zero or more attributes are accepted.
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
 * Generic attribute name.
 *
 * Name syntax is delegated to the canonical Names grammar.
 */
attributeName
    : qualifiedAttributeName
    ;


/**
 * Qualified attribute name.
 *
 * Examples:
 *
 *     @name
 *     @namespace::name
 *     @a::b::c
 *
 * Qualification depth is not bounded by this grammar.
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
 * Both forms are valid:
 *
 *     @name()
 *
 *     @name(value)
 *
 * Empty argument lists are structurally valid.
 */
attributeArguments
    : LPAREN attributeArgumentList? RPAREN
    ;


/**
 * Optional wrapper around attributeArguments.
 *
 * This rule is useful to consumers that need explicit optionality.
 */
optionalAttributeArguments
    : attributeArguments?
    ;


/**
 * Non-empty comma-separated attribute argument list.
 *
 * A trailing comma is permitted.
 *
 * Examples:
 *
 *     value
 *
 *     value1, value2
 *
 *     value1, value2,
 */
attributeArgumentList
    : attributeArgument (COMMA attributeArgument)* COMMA?
    ;


/**
 * One attribute argument.
 *
 * Named arguments are considered before positional arguments so the parser
 * can recognize:
 *
 *     name = value
 *
 * as one structured argument.
 */
attributeArgument
    : attributeNamedArgument
    | attributePositionalArgument
    ;


/**
 * Named attribute argument.
 *
 * Example:
 *
 *     mode = "deterministic"
 */
attributeNamedArgument
    : attributeArgumentName ASSIGN attributeValue
    ;


/**
 * Positional attribute argument.
 *
 * Example:
 *
 *     "deterministic"
 */
attributePositionalArgument
    : attributeValue
    ;


/**
 * Local argument name.
 *
 * An argument key is deliberately a simple identifier rather than a qualified
 * name. Namespace semantics for an attribute's argument schema belong to
 * semantic validation.
 */
attributeArgumentName
    : identifier
    ;


/*
 * ============================================================================
 * OPTIONAL ARGUMENT/VALUE WRAPPERS
 * ============================================================================
 */

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
 * ATTRIBUTE VALUES
 * ============================================================================
 */

/**
 * Generic structural attribute value.
 *
 * Attribute values deliberately do not consume the general Zamani expression
 * grammar.
 *
 * This prevents attributes from becoming a second runtime expression language.
 */
attributeValue
    : attributeScalarValue
    | attributeNameValue
    | attributeListValue
    | attributeMapValue
    | attributeNestedValue
    ;


/**
 * Scalar attribute value.
 *
 * Token definitions are owned by the canonical lexer.
 *
 * Current canonical scalar literal vocabulary:
 *
 *     INTEGER_LITERAL
 *     FLOAT_LITERAL
 *     STRING_LITERAL
 *     CHARACTER_LITERAL
 *     TRUE
 *     FALSE
 *
 * The parser does not assign semantic types or machine representations.
 */
attributeScalarValue
    : INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | CHARACTER_LITERAL
    | TRUE
    | FALSE
    ;


/**
 * Symbolic attribute value.
 *
 * Examples:
 *
 *     quantum::measurement
 *     tensor::compute
 *     hardware::accelerator
 *     execution::recover
 *
 * Resolution occurs downstream.
 */
attributeNameValue
    : qualifiedName
    ;


/*
 * ============================================================================
 * LIST VALUES
 * ============================================================================
 */

/**
 * Structured list value.
 *
 * Empty lists are valid:
 *
 *     []
 *
 * Non-empty lists may contain any structural attribute value.
 *
 * Trailing commas are accepted.
 */
attributeListValue
    : LBRACKET attributeValueList? RBRACKET
    ;


/**
 * Non-empty list contents.
 *
 * Examples:
 *
 *     [a]
 *     [a, b]
 *     [a, b,]
 */
attributeValueList
    : attributeValue (COMMA attributeValue)* COMMA?
    ;


/*
 * ============================================================================
 * MAP VALUES
 * ============================================================================
 */

/**
 * Structured map value.
 *
 * Empty maps are valid:
 *
 *     {}
 *
 * Non-empty maps contain key/value entries.
 *
 * Trailing commas are accepted.
 */
attributeMapValue
    : LBRACE attributeMapEntryList? RBRACE
    ;


/**
 * Non-empty map-entry list.
 *
 * Duplicate keys are deliberately preserved structurally.
 *
 * Semantic analysis decides whether duplicate keys are legal.
 */
attributeMapEntryList
    : attributeMapEntry (COMMA attributeMapEntry)* COMMA?
    ;


/**
 * One map entry.
 *
 * Examples:
 *
 *     mode = "fast"
 *     "mode" = "fast"
 */
attributeMapEntry
    : attributeMapKey ASSIGN attributeValue
    ;


/**
 * Map key.
 *
 * Keys may be simple identifiers or string literals.
 *
 * String keys permit externally defined schemas without requiring a new
 * grammar for every schema vocabulary.
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
 * Nested attribute used as an attribute value.
 *
 * Examples:
 *
 *     @outer(@inner)
 *
 *     @outer(@quantum::resource(memory = required_memory))
 */
attributeNestedValue
    : attribute
    ;


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * CONSUMER INTEGRATION
 * --------------------
 *
 * A consuming grammar should attach attributes using:
 *
 *     optionalAttributePrefix
 *
 * when attributes are optional.
 *
 * It should use:
 *
 *     attributePrefix
 *
 * when at least one attribute is required.
 *
 * It should use:
 *
 *     attribute
 *
 * when exactly one attribute is expected.
 *
 * It should NOT reproduce:
 *
 *     AT identifier
 *     AT qualifiedName
 *     AT identifier (...)
 *     AT qualifiedName (...)
 *
 * locally.
 *
 *
 * ============================================================================
 * NAMES INTEGRATION
 * ============================================================================
 *
 * Name ownership:
 *
 *     Attributes
 *         |
 *         v
 *     Names
 *         |
 *         v
 *     canonical lexer
 *
 * This ensures:
 *
 *     @a
 *     @a::b
 *     @a::b::c
 *
 * use exactly the same name syntax as the rest of Zamani.
 *
 * Adding a new namespace does not require changing this grammar.
 *
 *
 * ============================================================================
 * LEXER INTEGRATION
 * ============================================================================
 *
 * The dependency is:
 *
 *     ZamaniLexer
 *         |
 *         +--> canonical lexical composition
 *                 |
 *                 +--> identifiers
 *                 +--> punctuation
 *                 +--> literals
 *                 +--> keywords
 *                 +--> operators
 *                 +--> annotations
 *                 +--> comments
 *                 +--> whitespace
 *
 * This grammar must never import an individual literal lexer merely to obtain
 * a token.
 *
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * Recommended conceptual AST model:
 *
 *     Attribute
 *         name
 *         arguments[]
 *         source_span
 *
 *     AttributeArgument
 *         kind
 *         name?
 *         value
 *         source_span
 *
 *     AttributeValue
 *         Scalar
 *         Name
 *         List
 *         Map
 *         NestedAttribute
 *         source_span
 *
 *     AttributeMapEntry
 *         key
 *         value
 *         source_span
 *
 * The AST implementation may use different concrete Rust names.
 *
 * The important invariant is preservation of the complete source structure.
 *
 * In particular, map entries MUST NOT be represented only by a host-language
 * map if doing so would silently discard duplicate source keys before semantic
 * validation.
 *
 *
 * ============================================================================
 * SEMANTIC INTEGRATION
 * ============================================================================
 *
 * After parsing:
 *
 *     attribute
 *         |
 *         v
 *     structural validation
 *         |
 *         v
 *     attribute resolution
 *         |
 *         +--> attachment validation
 *         +--> argument validation
 *         +--> value validation
 *         +--> type validation
 *         +--> capability validation
 *         +--> resource validation
 *         +--> effect validation
 *         +--> policy validation
 *         +--> contract validation
 *         +--> portability validation
 *         |
 *         v
 *     semantic model
 *
 * The parser does none of these semantic operations.
 *
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * No IR is created here.
 *
 * Depending on semantic meaning, an attribute may contribute to:
 *
 *     semantic metadata
 *     classical representation
 *     quantum::ir
 *     HDL/hardware representation
 *     distributed representation
 *     interoperability representation
 *     optimization metadata
 *     provenance
 *     policy metadata
 *
 * The attribute grammar remains unchanged when a new backend is introduced.
 *
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Attributes are source-level descriptions.
 *
 * They must remain independent of whether the program eventually runs on:
 *
 *     embedded hardware
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC system
 *     cluster
 *     distributed system
 *     cloud infrastructure
 *     heterogeneous hardware
 *     future architectures.
 *
 * Resource and capability analysis determines whether and how the target can
 * realize the program.
 *
 *
 * ============================================================================
 * POCO-REAF INTEGRATION
 * ============================================================================
 *
 * The attribute grammar supports the source-level expression of portable
 * intent without encoding target capacities.
 *
 * For example:
 *
 *     @requires(capability = tensor::compute)
 *
 * expresses a semantic requirement.
 *
 * It does NOT mean:
 *
 *     use a particular GPU;
 *     use a particular accelerator;
 *     use a fixed number of processing units.
 *
 * Likewise:
 *
 *     @requires(resource = memory)
 *
 * does not encode a fixed memory capacity.
 *
 * The same attribute syntax therefore remains usable as the eventual
 * realization changes scale.
 *
 *
 * ============================================================================
 * SECURITY / TRUST BOUNDARY
 * ============================================================================
 *
 * Parsed attributes are untrusted source data until validated.
 *
 * Parsing success MUST NOT imply:
 *
 *     authorization;
 *     permission;
 *     capability ownership;
 *     resource reservation;
 *     trusted provenance;
 *     executable intent.
 *
 * These properties require explicit downstream validation.
 *
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given:
 *
 *     identical token stream
 *     identical grammar version
 *     identical parser configuration
 *
 * the parser must produce equivalent attribute parse-tree structure.
 *
 * No rule depends on:
 *
 *     current time;
 *     randomness;
 *     hardware;
 *     filesystem state;
 *     network state;
 *     environment state;
 *     runtime state;
 *     scheduler state;
 *     target availability.
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must verify arbitrary structural repetition for:
 *
 *     attributes;
 *     arguments;
 *     list values;
 *     map entries;
 *     qualified names;
 *     nested attributes.
 *
 * Tests MUST NOT assert an artificial language-level maximum.
 *
 * Resource-constrained tests may choose finite test sizes, but those sizes are
 * test parameters rather than language limits.
 *
 *
 * ============================================================================
 * REQUIRED POSITIVE TESTS
 * ============================================================================
 *
 * Minimal:
 *
 *     @a
 *
 * Empty arguments:
 *
 *     @a()
 *
 * Qualified:
 *
 *     @a::b
 *
 * Qualified with arguments:
 *
 *     @a::b(value)
 *
 * Named:
 *
 *     @a(mode = "fast")
 *
 * Multiple named:
 *
 *     @a(mode = "fast", level = 2)
 *
 * Mixed:
 *
 *     @a(first, mode = "fast")
 *
 * Trailing comma:
 *
 *     @a(first, mode = "fast",)
 *
 * List:
 *
 *     @a([cpu, gpu, qpu])
 *
 * Empty list:
 *
 *     @a([])
 *
 * List trailing comma:
 *
 *     @a([cpu, gpu,])
 *
 * Map:
 *
 *     @a({mode = "fast", level = 2})
 *
 * Empty map:
 *
 *     @a({})
 *
 * Map trailing comma:
 *
 *     @a({mode = "fast", level = 2,})
 *
 * String map key:
 *
 *     @a({"mode" = "fast"})
 *
 * Nested:
 *
 *     @a(@b)
 *
 * Nested with arguments:
 *
 *     @a(@b(mode = "fast"))
 *
 * Boolean:
 *
 *     @a(enabled = true)
 *
 *     @a(enabled = false)
 *
 * Numeric:
 *
 *     @a(level = 42)
 *
 *     @a(threshold = 1.25)
 *
 * Character:
 *
 *     @a(symbol = 'x')
 *
 * Symbolic:
 *
 *     @a(capability = quantum::measurement)
 *
 * Deep qualification:
 *
 *     @a(a::b::c::d)
 *
 *
 * ============================================================================
 * REQUIRED NEGATIVE TESTS
 * ============================================================================
 *
 * Missing name:
 *
 *     @
 *
 * Missing closing parenthesis:
 *
 *     @a(
 *
 * Missing value:
 *
 *     @a(mode =)
 *
 * Missing key:
 *
 *     @a(=value)
 *
 * Missing closing list delimiter:
 *
 *     @a([value)
 *
 * Missing closing map delimiter:
 *
 *     @a({key = value)
 *
 * Missing map value:
 *
 *     @a({key =})
 *
 * Missing map key:
 *
 *     @a({=value})
 *
 * Invalid separator:
 *
 *     @a(value value)
 *
 * Unterminated nested attribute:
 *
 *     @a(@b(
 *
 * These are parser-structure failures.
 *
 *
 * ============================================================================
 * REQUIRED SEMANTIC NEGATIVE TESTS
 * ============================================================================
 *
 * These MUST NOT be encoded as parser restrictions:
 *
 *     unknown attribute;
 *     attribute attached to an invalid construct;
 *     unknown named argument;
 *     duplicate named argument where prohibited;
 *     duplicate map key where prohibited;
 *     invalid attribute value type;
 *     unsatisfied capability;
 *     unsatisfied resource requirement;
 *     prohibited policy;
 *     invalid effect;
 *     invalid contract metadata;
 *     unavailable target capability.
 *
 *
 * ============================================================================
 * REQUIRED BOUNDARY TESTS
 * ============================================================================
 *
 *     @a
 *     @a()
 *     @a(value)
 *     @a(value,)
 *     @a(value1, value2)
 *     @a(value1, value2,)
 *     @a(mode = value)
 *     @a([value])
 *     @a([value,])
 *     @a({key = value})
 *     @a({key = value,})
 *     @a(@b)
 *     @a(@b())
 *     @a(@b(value))
 *     @a(a::b)
 *     @a(a::b::c)
 *
 * Also test adjacency with:
 *
 *     declarations;
 *     functions;
 *     types;
 *     expressions;
 *     statements;
 *     modules;
 *     quantum constructs;
 *     HDL constructs;
 *     resource declarations;
 *     policy declarations;
 *     contracts.
 *
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * The same attribute syntax must be usable with metadata associated with:
 *
 *     classical computation;
 *     numerical computation;
 *     tensor computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL;
 *     hardware intent;
 *     accelerators;
 *     AI;
 *     reasoning;
 *     learning;
 *     adaptation;
 *     uncertainty;
 *     knowledge;
 *     agents;
 *     distributed computation;
 *     networking;
 *     security;
 *     FFI;
 *     ABI;
 *     simulation;
 *     metaprogramming;
 *     compilation.
 *
 * The grammar itself remains unchanged across these domains.
 *
 *
 * ============================================================================
 * COMPATIBILITY TEST CONTRACT
 * ============================================================================
 *
 * Verify that:
 *
 *     @name
 *     @name(...)
 *     @namespace::name
 *     @namespace::name(...)
 *
 * remain stable.
 *
 * Verify that identifiers beginning with attribute names remain identifiers
 * when they are lexically identifiers.
 *
 * Verify that:
 *
 *     true
 *     false
 *
 * continue to use the canonical TRUE/FALSE lexer tokens.
 *
 * Verify that attributes do not introduce another generic identifier syntax.
 *
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where a formatter exists:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     formatter
 *       ->
 *     parser
 *
 * must preserve attribute semantics.
 *
 * Source ordering of attributes and arguments must not be lost.
 *
 * Duplicate map entries must not be silently discarded during round-trip
 * processing.
 *
 *
 * ============================================================================
 * HARD-CODING COMPLETION AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     NO MAX_* language constants.
 *     NO machine capacity assumptions.
 *     NO CPU count.
 *     NO GPU count.
 *     NO FPGA count.
 *     NO accelerator count.
 *     NO QPU count.
 *     NO qubit count.
 *     NO node count.
 *     NO memory capacity.
 *     NO register width.
 *     NO tensor-rank ceiling.
 *     NO topology.
 *     NO physical device identifier.
 *     NO vendor-specific implementation.
 *     NO quantum gate catalogue.
 *     NO backend selection.
 *     NO routing.
 *     NO scheduling.
 *     NO calibration.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     [ ] It is the canonical generic attribute parser grammar.
 *
 *     [ ] It uses tokenVocab = ZamaniLexer.
 *
 *     [ ] It imports Names.
 *
 *     [ ] It does not define lexer rules.
 *
 *     [ ] It does not duplicate identifier syntax.
 *
 *     [ ] It does not duplicate qualified-name syntax.
 *
 *     [ ] It does not duplicate expression syntax.
 *
 *     [ ] It does not duplicate literal lexer rules.
 *
 *     [ ] It correctly recognizes TRUE and FALSE.
 *
 *     [ ] optionalAttributes is genuinely optional.
 *
 *     [ ] attributeList is non-empty.
 *
 *     [ ] Empty attribute argument lists are accepted.
 *
 *     [ ] Named arguments are supported.
 *
 *     [ ] Positional arguments are supported.
 *
 *     [ ] Trailing argument commas are supported.
 *
 *     [ ] Lists are supported.
 *
 *     [ ] Empty lists are supported.
 *
 *     [ ] Trailing list commas are supported.
 *
 *     [ ] Maps are supported.
 *
 *     [ ] Empty maps are supported.
 *
 *     [ ] Trailing map commas are supported.
 *
 *     [ ] String map keys are supported.
 *
 *     [ ] Duplicate map keys remain structurally representable.
 *
 *     [ ] Nested attributes are supported.
 *
 *     [ ] Attribute qualification is delegated to Names.
 *
 *     [ ] No arbitrary runtime expression grammar is introduced.
 *
 *     [ ] No semantic interpretation occurs in the parser.
 *
 *     [ ] No capability is granted by parsing.
 *
 *     [ ] No resource is selected by parsing.
 *
 *     [ ] No hardware is selected by parsing.
 *
 *     [ ] No quantum topology is encoded.
 *
 *     [ ] No QEC implementation is encoded.
 *
 *     [ ] No ZQN implementation is encoded.
 *
 *     [ ] No HAL implementation is encoded.
 *
 *     [ ] No backend-specific representation is encoded.
 *
 *     [ ] No artificial language-level capacity is encoded.
 *
 *     [ ] No Rust actions exist.
 *
 *     [ ] No unsafe Rust is required.
 *
 *     [ ] Rust 1.97 integration is verified.
 *
 *     [ ] Rust 1.97.1 integration is verified.
 *
 *     [ ] ANTLR generation succeeds.
 *
 *     [ ] Imported grammar resolution succeeds.
 *
 *     [ ] Positive tests pass.
 *
 *     [ ] Negative tests pass.
 *
 *     [ ] Semantic negative tests pass downstream.
 *
 *     [ ] Boundary tests pass.
 *
 *     [ ] Cross-domain tests pass.
 *
 *     [ ] Scalability tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 *     [ ] Round-trip tests pass.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Attributes are a generic source-level metadata mechanism.
 *
 * They are not:
 *
 *     a hardware language;
 *     a quantum language;
 *     an AI language;
 *     a resource allocator;
 *     a policy engine;
 *     a runtime;
 *     an IR;
 *     a backend.
 *
 * Their purpose is to preserve structured source intent across:
 *
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     semantic analysis
 *       ->
 *     canonical semantic model
 *       ->
 *     domain IR
 *       ->
 *     target realization.
 *
 * The grammar therefore remains stable while the set of computational domains,
 * hardware architectures, capabilities, resources, policies, models and
 * implementations grows.
 *
 * This is essential to Zamani's:
 *
 *     Program_Once
 *       ->
 *     Compile_Once
 *       ->
 *     Run_Everywhere
 *       ->
 *     Run_Anywhere
 *       ->
 *     Forever
 *
 * architecture.
 * ============================================================================
 */