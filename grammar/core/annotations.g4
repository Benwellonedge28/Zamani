/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/core/annotations.g4
 *
 * GRAMMAR
 * -------
 * Annotations
 *
 * STATUS
 * ------
 * CANONICAL CORE PARSER COMPONENT
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the canonical parser-level owner of Zamani annotation syntax.
 *
 * An annotation is source-level metadata attached to another syntactic
 * construct. This grammar defines the STRUCTURE of an annotation only.
 *
 * Examples:
 *
 *     @inline
 *     @deprecated
 *     @quantum
 *     @resource(...)
 *     @capability(...)
 *     @custom::annotation(...)
 *
 * The annotation name and its arguments remain structurally extensible.
 *
 * New annotation names therefore do NOT require modification of this grammar.
 *
 *
 * OWNS
 * ----
 *
 * This file owns:
 *
 *     - annotation syntax;
 *     - annotation marker consumption;
 *     - annotation names;
 *     - annotation argument syntax;
 *     - positional annotation arguments;
 *     - named annotation arguments;
 *     - annotation value syntax;
 *     - structured annotation lists;
 *     - structured annotation maps;
 *     - nested annotation values;
 *     - annotation sequences;
 *     - optional annotation sequences.
 *
 *
 * DOES NOT OWN
 * ------------
 *
 * This file does NOT own:
 *
 *     - lexical token definitions;
 *     - identifier lexical rules;
 *     - keyword spelling;
 *     - literal lexical rules;
 *     - attributes;
 *     - metadata attachment policy;
 *     - pragmas;
 *     - compiler directives;
 *     - declaration attachment rules;
 *     - function attachment rules;
 *     - type attachment rules;
 *     - module attachment rules;
 *     - quantum semantics;
 *     - classical semantics;
 *     - HDL semantics;
 *     - hardware realization;
 *     - resource discovery;
 *     - capability discovery;
 *     - topology;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - backend selection;
 *     - runtime execution;
 *     - AST construction;
 *     - semantic analysis;
 *     - IR construction.
 *
 *
 * DEPENDS_ON
 * ----------
 *
 * Canonical lexer vocabulary:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical source-name grammar:
 *
 *     grammar/core/names.g4
 *
 * Imported grammar:
 *
 *     Names
 *
 *
 * EXPORTS
 * -------
 *
 * Public parser rules:
 *
 *     annotation
 *     annotations
 *     annotationList
 *     optionalAnnotations
 *     annotationPrefix
 *     optionalAnnotationPrefix
 *     annotationName
 *     annotationArguments
 *     annotationArgumentList
 *     annotationArgument
 *     annotationArgumentName
 *     annotationValue
 *     annotationScalarValue
 *     annotationListValue
 *     annotationValueList
 *     annotationMapValue
 *     annotationMapEntryList
 *     annotationMapEntry
 *
 *
 * CONSUMED_BY
 * -----------
 *
 * Core/parser composition and any construct that permits annotations:
 *
 *     grammar/antlr/ZamaniParser.g4
 *     grammar/core/*
 *     grammar/declarations/*
 *     grammar/functions/*
 *     grammar/modules/*
 *     grammar/types/*
 *     grammar/statements/*
 *     grammar/expressions/*
 *     grammar/classical/*
 *     grammar/quantum/*
 *     grammar/hybrid/*
 *     grammar/hdl/*
 *     grammar/hardware/*
 *     grammar/distributed/*
 *     grammar/ai/*
 *     grammar/data/*
 *     grammar/networking/*
 *     grammar/security/*
 *     grammar/resources/*
 *     grammar/interoperability/*
 *     grammar/dialects/*
 *     grammar/metaprogramming/*
 *
 * Consumers MUST reuse these rules rather than redefining annotation syntax.
 *
 *
 * AST_OWNER
 * ---------
 *
 * Existing Zamani frontend AST layer.
 *
 * This grammar creates ANTLR parser contexts only.
 *
 * The AST representation must preserve:
 *
 *     - annotation name;
 *     - annotation argument ordering;
 *     - named/positional distinction;
 *     - literal source representation;
 *     - nested structure;
 *     - source spans;
 *     - source ordering.
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Semantic analysis / annotation registry.
 *
 * Semantic analysis determines:
 *
 *     - whether an annotation exists;
 *     - whether it is permitted in the current context;
 *     - whether its arguments are valid;
 *     - argument types;
 *     - duplicate argument policy;
 *     - annotation version;
 *     - annotation owner;
 *     - semantic effect;
 *     - capability implications;
 *     - resource implications;
 *     - policy implications;
 *     - provenance implications.
 *
 *
 * TYPE_OWNER
 * ----------
 *
 * The annotation grammar does not own annotation argument types.
 *
 * Annotation schemas and semantic validation determine whether a value is:
 *
 *     string
 *     integer
 *     float
 *     boolean
 *     character
 *     symbolic name
 *     list
 *     map
 *     nested annotation
 *     another supported literal/value.
 *
 *
 * EFFECT_OWNER
 * ------------
 *
 * This grammar has no effects.
 *
 * An annotation may DECLARE or DESCRIBE effects, but the grammar itself never
 * grants an effect.
 *
 * For example:
 *
 *     @requires_effect(...)
 *
 * is syntax only.
 *
 * Effect checking belongs to the semantic effect subsystem.
 *
 *
 * CAPABILITY_OWNER
 * ----------------
 *
 * An annotation may describe capabilities or capability requirements.
 *
 * The grammar does not resolve them.
 *
 * Example:
 *
 *     @requires(capability = "quantum.measurement")
 *
 * remains source syntax until semantic capability analysis.
 *
 *
 * RESOURCE_OWNER
 * --------------
 *
 * Annotations may carry resource requirements, preferences, constraints or
 * metadata.
 *
 * This grammar does not inspect actual resources.
 *
 * It does not know:
 *
 *     CPU count
 *     GPU count
 *     QPU count
 *     qubit count
 *     memory capacity
 *     node count
 *     device count
 *     topology
 *     tensor capacity
 *
 *
 * CONTRACT_OWNER
 * --------------
 *
 * Annotation values may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * but contract semantics are owned by the validation/contract subsystem.
 *
 *
 * POLICY_OWNER
 * ------------
 *
 * Annotation syntax may carry policy declarations or references.
 *
 * Policy interpretation belongs to the policy/security/execution subsystems.
 *
 *
 * PROVENANCE_OWNER
 * ----------------
 *
 * The parser must preserve annotation source spans and ordering so semantic
 * provenance can record:
 *
 *     source
 *     annotation
 *     interpretation
 *     transformation
 *     decision
 *     verification
 *
 *
 * IR_OWNER
 * --------
 *
 * This grammar owns no IR.
 *
 * An annotation may later become semantic metadata attached to:
 *
 *     canonical semantic model
 *     classical IR
 *     quantum::ir
 *     HDL/hardware representation
 *     distributed representation
 *     execution plan
 *     deployment metadata
 *
 * depending on semantic ownership.
 *
 *
 * QUANTUM BOUNDARY
 * ----------------
 *
 * Annotation syntax remains domain-neutral.
 *
 * Quantum annotations may eventually contribute semantic metadata to:
 *
 *     quantum::ir
 *
 * but this grammar does not know:
 *
 *     physical qubits
 *     gate sets
 *     coupling maps
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     QPU identity.
 *
 *
 * HDL BOUNDARY
 * ------------
 *
 * HDL annotations may describe source intent, verification intent, timing
 * intent, synthesis intent or other metadata.
 *
 * Actual hardware realization remains downstream.
 *
 *
 * BACKEND BOUNDARY
 * ----------------
 *
 * Annotation syntax MUST NOT select a backend.
 *
 * A source annotation may express intent, for example:
 *
 *     @portable
 *     @requires(...)
 *     @prefer(...)
 *
 * but target realization is determined downstream by:
 *
 *     semantic analysis
 *     capability negotiation
 *     resource analysis
 *     compilation
 *     lowering
 *     scheduling
 *     routing
 *     HAL
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It MUST NOT define lexer rules.
 *
 * The canonical parser vocabulary is:
 *
 *     ZamaniLexer
 *
 * The canonical annotation marker is:
 *
 *     AT
 *
 * `AT` is lexically the `@` marker.
 *
 * Annotation names remain canonical IDENTIFIER/name constructs.
 *
 * Canonical literal tokens consumed here are the tokens exported by
 * ZamaniLexer, including the repository's canonical:
 *
 *     INTEGER
 *     FLOAT
 *     STRING
 *     CHAR
 *     TRUE
 *     FALSE
 *
 * No token aliases such as:
 *
 *     ANNOTATION_MARKER
 *     INTEGER_LITERAL
 *     FLOAT_LITERAL
 *     CHARACTER_LITERAL
 *     BOOLEAN_LITERAL
 *
 * are introduced here.
 *
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * The grammar is intentionally a parser grammar:
 *
 *     parser grammar Annotations;
 *
 * It consumes:
 *
 *     ZamaniLexer
 *
 * and imports:
 *
 *     Names
 *
 * This establishes the dependency:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     Annotations
 *          |
 *          v
 *     annotation syntax
 *
 * Names are reused rather than reimplemented.
 *
 *
 * ============================================================================
 * ANNOTATION MODEL
 * ============================================================================
 *
 * Canonical surface forms:
 *
 *     @name
 *
 *     @name()
 *
 *     @name(value)
 *
 *     @name(value1, value2)
 *
 *     @name(key = value)
 *
 *     @name(key = value, other = value)
 *
 *     @namespace::name
 *
 *     @namespace::name(...)
 *
 * Nested values are supported:
 *
 *     @outer(@inner)
 *
 *     @outer([@a, @b])
 *
 *     @outer({
 *         first = @a,
 *         second = @b
 *     })
 *
 * The exact semantic legality of these forms belongs downstream.
 *
 *
 * ============================================================================
 * ANNOTATION NAME EXTENSIBILITY
 * ============================================================================
 *
 * Annotation names are NOT enumerated.
 *
 * Therefore this grammar does not contain:
 *
 *     quantumAnnotation
 *     gpuAnnotation
 *     fpgaAnnotation
 *     qecAnnotation
 *     aiAnnotation
 *     vendorAnnotation
 *     hardwareAnnotation
 *
 * as closed keyword inventories.
 *
 * Future annotation namespaces remain representable through qualified names.
 *
 * Examples:
 *
 *     @quantum::logical
 *     @hardware::pipeline
 *     @distributed::replicated
 *     @ai::differentiable
 *     @security::sandbox
 *     @future::domain::annotation
 *
 * These names are resolved semantically.
 *
 *
 * ============================================================================
 * POSITIONAL / NAMED ARGUMENT CONTRACT
 * ============================================================================
 *
 * Positional argument:
 *
 *     @example(value)
 *
 * Named argument:
 *
 *     @example(key = value)
 *
 * The parser preserves which form was written.
 *
 * The semantic layer determines:
 *
 *     - whether positional arguments are permitted;
 *     - whether named arguments are permitted;
 *     - whether both may coexist;
 *     - required argument names;
 *     - duplicate names;
 *     - argument ordering;
 *     - argument types.
 *
 * The grammar deliberately does not silently reorder arguments.
 *
 *
 * ============================================================================
 * TRAILING COMMA CONTRACT
 * ============================================================================
 *
 * Trailing commas are accepted:
 *
 *     @example(a, b,)
 *
 *     @example(
 *         a = 1,
 *         b = 2,
 *     )
 *
 * This improves source stability and formatter compatibility.
 *
 *
 * ============================================================================
 * VALUE CONTRACT
 * ============================================================================
 *
 * Annotation values are structural values, not arbitrary runtime expressions.
 *
 * Supported categories are:
 *
 *     scalar literal
 *     symbolic qualified name
 *     nested annotation
 *     list
 *     map
 *
 * Runtime expression syntax does NOT belong here.
 *
 * This prevents annotations from silently becoming a second executable
 * expression language.
 *
 * If future Zamani requires compile-time expressions in annotations, that
 * capability must be introduced explicitly through the metaprogramming /
 * compile-time expression architecture.
 *
 *
 * ============================================================================
 * SCALAR VALUE CONTRACT
 * ============================================================================
 *
 * The canonical source-level scalar forms are:
 *
 *     INTEGER
 *     FLOAT
 *     STRING
 *     CHAR
 *     TRUE
 *     FALSE
 *
 * Numeric magnitude, precision and representation remain semantic concerns.
 *
 * A source integer does NOT imply:
 *
 *     8-bit
 *     16-bit
 *     32-bit
 *     64-bit
 *     128-bit
 *     native machine integer
 *
 * A source floating literal does NOT imply a particular hardware format.
 *
 *
 * ============================================================================
 * SYMBOLIC VALUE CONTRACT
 * ============================================================================
 *
 * Qualified names may be used as annotation values:
 *
 *     @target(classical::portable)
 *
 *     @operation(quantum::measurement)
 *
 *     @policy(security::restricted)
 *
 * These are unresolved symbolic references until semantic analysis.
 *
 *
 * ============================================================================
 * LIST CONTRACT
 * ============================================================================
 *
 * Lists may contain arbitrary annotation values:
 *
 *     @example([a, b, c])
 *
 *     @example([1, 2, 3])
 *
 *     @example([@a, @b])
 *
 *     @example([[1, 2], [3, 4]])
 *
 * No fixed list length is encoded.
 *
 *
 * ============================================================================
 * MAP CONTRACT
 * ============================================================================
 *
 * Maps use local identifier keys:
 *
 *     @example({
 *         mode = "portable",
 *         policy = security::restricted
 *     })
 *
 * Map keys remain syntactic identifiers.
 *
 * Semantic validation determines:
 *
 *     - recognized keys;
 *     - duplicate-key behavior;
 *     - required keys;
 *     - key/value types;
 *     - ordering significance.
 *
 *
 * ============================================================================
 * NESTING CONTRACT
 * ============================================================================
 *
 * Annotation values are recursively compositional.
 *
 * There is no grammar-level nesting constant.
 *
 * Therefore the grammar does not define:
 *
 *     MAX_ANNOTATION_DEPTH
 *     MAX_ANNOTATION_ARGUMENTS
 *     MAX_ANNOTATION_VALUES
 *     MAX_ANNOTATION_LIST_SIZE
 *     MAX_ANNOTATION_MAP_SIZE
 *
 * Actual parser/compiler resource limits remain implementation limits.
 *
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * This grammar contains no machine-dependent limits.
 *
 * It imposes no universal limit on:
 *
 *     annotations per construct
 *     annotation arguments
 *     list elements
 *     map entries
 *     nested annotation structures
 *     qualified-name depth
 *     source declarations
 *     modules
 *     functions
 *     types
 *     qubits
 *     processors
 *     accelerators
 *     nodes
 *     memory
 *     tensor rank
 *     devices
 *
 * No constants such as the following are permitted:
 *
 *     MAX_ANNOTATIONS
 *     MAX_ANNOTATION_ARGUMENTS
 *     MAX_ANNOTATION_DEPTH
 *     MAX_ANNOTATION_LIST_SIZE
 *     MAX_ANNOTATION_MAP_SIZE
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * "Infinity" means no artificial language-level machine ceiling.
 *
 * It does not mean physical resources are infinite.
 *
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token stream
 *     grammar
 *     language/compatibility configuration
 *
 * It does NOT depend on:
 *
 *     hardware
 *     resource availability
 *     filesystem state
 *     network state
 *     runtime state
 *     scheduler state
 *     randomness
 *     wall-clock time.
 *
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The parser must preserve:
 *
 *     annotation order
 *     argument order
 *     named/positional distinction
 *     qualified-name segment order
 *     literal source spelling
 *     nested structure
 *     source spans.
 *
 * Semantic normalization belongs downstream.
 *
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics should distinguish:
 *
 *     missing annotation name
 *     malformed argument list
 *     missing argument value
 *     missing closing delimiter
 *     malformed map entry
 *     invalid delimiter placement.
 *
 * Semantic diagnostics should distinguish:
 *
 *     unknown annotation
 *     annotation not valid for context
 *     unknown named argument
 *     duplicate named argument
 *     invalid annotation value type
 *     unsupported annotation version
 *     conflicting annotation semantics.
 *
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing source forms:
 *
 *     @name
 *     @name(...)
 *
 * remain supported.
 *
 * Annotation names remain identifiers/qualified names rather than a closed
 * keyword list.
 *
 * Any future annotation semantic change must be versioned through:
 *
 *     grammar/compatibility/
 *
 * rather than changing the syntactic interpretation silently.
 *
 *
 * ============================================================================
 * TEST OWNER
 * ============================================================================
 *
 * Recommended test location:
 *
 *     grammar/tests/core/annotations/
 *
 * Required categories:
 *
 *     lexical integration
 *     parser positive
 *     parser negative
 *     boundary
 *     nesting
 *     scalability
 *     cross-domain
 *     compatibility
 *     determinism
 *     source-preservation
 *
 *
 * ============================================================================
 * SPEC OWNER
 * ============================================================================
 *
 * Normative human specification:
 *
 *     grammar/specification/
 *
 * Machine-checkable specification:
 *
 *     grammar/spec/
 *
 * Core syntax status should be reflected in:
 *
 *     grammar/grammar.md
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] It uses parser grammar syntax.
 *
 *     [ ] It consumes ZamaniLexer.
 *
 *     [ ] It imports Names.
 *
 *     [ ] It defines no lexer tokens.
 *
 *     [ ] It uses the canonical AT token.
 *
 *     [ ] It uses the canonical IDENTIFIER/name rules.
 *
 *     [ ] It supports bare annotations.
 *
 *     [ ] It supports empty argument lists.
 *
 *     [ ] It supports positional arguments.
 *
 *     [ ] It supports named arguments.
 *
 *     [ ] It supports mixed argument forms structurally.
 *
 *     [ ] It supports trailing commas.
 *
 *     [ ] It supports qualified annotation names.
 *
 *     [ ] It supports scalar values.
 *
 *     [ ] It supports symbolic values.
 *
 *     [ ] It supports nested annotations.
 *
 *     [ ] It supports lists.
 *
 *     [ ] It supports maps.
 *
 *     [ ] It imposes no artificial resource limits.
 *
 *     [ ] It does not enumerate domain-specific annotation names.
 *
 *     [ ] It does not select hardware.
 *
 *     [ ] It does not create quantum IR.
 *
 *     [ ] It does not create HDL IR.
 *
 *     [ ] It does not perform semantic validation.
 *
 *     [ ] It preserves source structure.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Cross-domain tests exist.
 *
 *     [ ] Compatibility tests exist.
 *
 *     [ ] Rust 1.97 / 1.97.1 integration succeeds.
 *
 *     [ ] No unsafe Rust is required.
 *
 * ============================================================================
 */

parser grammar Annotations;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/*
 * ============================================================================
 * 1. PUBLIC ANNOTATION ENTRY POINTS
 * ============================================================================
 */

/*
 * Exactly one annotation.
 */
annotation
    : AT annotationName annotationArguments?
    ;


/*
 * One or more annotations.
 *
 * No artificial maximum is encoded.
 */
annotations
    : annotation+
    ;


/*
 * Alias-style public list rule retained for reusable consumers.
 */
annotationList
    : annotation+
    ;


/*
 * Zero or more annotations.
 */
optionalAnnotations
    : annotation*
    ;


/*
 * Annotation prefix.
 *
 * Consumers decide where an annotation prefix is legal.
 */
annotationPrefix
    : annotationList
    ;


/*
 * Optional annotation prefix.
 */
optionalAnnotationPrefix
    : optionalAnnotations
    ;


/*
 * ============================================================================
 * 2. ANNOTATION NAME
 * ============================================================================
 *
 * Annotation names reuse the canonical source-level qualified-name grammar.
 *
 * Examples:
 *
 *     @inline
 *     @quantum::logical
 *     @hardware::pipeline
 *     @future::domain::annotation
 */
annotationName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 3. ANNOTATION ARGUMENTS
 * ============================================================================
 *
 * Empty argument lists are legal:
 *
 *     @name()
 */
annotationArguments
    : LPAREN annotationArgumentList? RPAREN
    ;


/*
 * Comma-separated argument list with an optional trailing comma.
 */
annotationArgumentList
    : annotationArgument (COMMA annotationArgument)* COMMA?
    ;


/*
 * ============================================================================
 * 4. ANNOTATION ARGUMENT
 * ============================================================================
 *
 * Named arguments are deliberately placed first.
 *
 * This gives the parser an immediately distinguishable structural prefix:
 *
 *     IDENTIFIER ASSIGN
 *
 * before considering a positional value.
 *
 * Examples:
 *
 *     @example(name = value)
 *
 *     @example(value)
 *
 *     @example(name = value, other)
 *
 * Whether mixing named and positional arguments is semantically legal is
 * decided by the annotation schema/semantic layer.
 */
annotationArgument
    : annotationArgumentName ASSIGN annotationValue
    | annotationValue
    ;


/*
 * Named argument keys are local identifiers.
 *
 * Qualification belongs to the annotation name itself.
 */
annotationArgumentName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 5. ANNOTATION VALUES
 * ============================================================================
 *
 * Values are deliberately structural rather than arbitrary expressions.
 */
annotationValue
    : annotationScalarValue
    | annotationName
    | annotation
    | annotationListValue
    | annotationMapValue
    ;


/*
 * ============================================================================
 * 6. SCALAR VALUES
 * ============================================================================
 *
 * These are the canonical source-level literal tokens exposed by ZamaniLexer.
 */
annotationScalarValue
    : INTEGER
    | FLOAT
    | STRING
    | CHAR
    | TRUE
    | FALSE
    ;


/*
 * ============================================================================
 * 7. LIST VALUES
 * ============================================================================
 *
 * Examples:
 *
 *     @example([a, b, c])
 *
 *     @example([1, 2, 3])
 *
 *     @example([@a, @b])
 *
 *     @example([[1, 2], [3, 4]])
 */
annotationListValue
    : LBRACKET annotationValueList? RBRACKET
    ;


annotationValueList
    : annotationValue (COMMA annotationValue)* COMMA?
    ;


/*
 * ============================================================================
 * 8. MAP VALUES
 * ============================================================================
 *
 * Examples:
 *
 *     @example({
 *         mode = "portable"
 *     })
 *
 *     @example({
 *         policy = security::restricted,
 *         capability = quantum::measurement
 *     })
 *
 * Keys are local identifiers.
 *
 * Duplicate-key policy is semantic.
 */
annotationMapValue
    : LBRACE annotationMapEntryList? RBRACE
    ;


annotationMapEntryList
    : annotationMapEntry (COMMA annotationMapEntry)* COMMA?
    ;


annotationMapEntry
    : annotationArgumentName ASSIGN annotationValue
    ;


/*
 * ============================================================================
 * 9. STRUCTURAL INTEGRATION NOTES
 * ============================================================================
 *
 * The following source constructs are examples of consumers:
 *
 *     @inline
 *     fn compute() { ... }
 *
 *     @quantum::logical
 *     qubit q;
 *
 *     @hardware::portable
 *     resource r;
 *
 *     @requires(
 *         capability = "quantum.measurement"
 *     )
 *
 *     @policy(
 *         execution = portable
 *     )
 *
 * These examples demonstrate syntax only.
 *
 * The semantic owners determine whether each annotation exists and whether
 * it is legal.
 *
 * ============================================================================
 * DOMAIN INTEGRATION
 * ============================================================================
 *
 * CLASSICAL
 * ---------
 *
 * Classical declarations may consume:
 *
 *     optionalAnnotationPrefix
 *
 * before declarations, functions, types, data structures or operations.
 *
 *
 * QUANTUM
 * -------
 *
 * Quantum declarations and operations may consume the same annotation rules.
 *
 * Annotation semantics may eventually contribute metadata to:
 *
 *     quantum semantic model
 *         ->
 *     quantum::ir
 *
 * The annotation grammar itself remains quantum-backend independent.
 *
 *
 * HYBRID
 * ------
 *
 * Hybrid constructs use the same annotation representation.
 *
 * No second hybrid annotation system is introduced.
 *
 *
 * HDL / HARDWARE
 * --------------
 *
 * HDL and hardware constructs reuse:
 *
 *     annotationPrefix
 *
 * for source-level metadata and intent.
 *
 * Physical implementation remains downstream.
 *
 *
 * AI / DATA
 * --------
 *
 * AI/data constructs can annotate:
 *
 *     models
 *     datasets
 *     inference
 *     learning
 *     provenance
 *     reproducibility
 *     policies
 *
 * without adding new parser rules for each application.
 *
 *
 * DISTRIBUTED / NETWORKING
 * ------------------------
 *
 * Distributed/network constructs may annotate:
 *
 *     services
 *     actors
 *     channels
 *     policies
 *     deployment intent
 *
 * Actual placement remains downstream.
 *
 *
 * SECURITY
 * --------
 *
 * Security annotations may describe:
 *
 *     trust
 *     sandboxing
 *     authorization
 *     policy
 *
 * The annotation grammar does not itself grant security privileges.
 *
 *
 * METAPROGRAMMING
 * ---------------
 *
 * Compile-time/reflection systems may consume annotation syntax as metadata.
 *
 * Such consumers remain responsible for authorization/effect rules.
 *
 *
 * ============================================================================
 * POCO-REAF INTEGRATION
 * ============================================================================
 *
 * Annotation syntax describes source intent or metadata.
 *
 * It does not bind the program to:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     cluster
 *     cloud
 *     vendor
 *     device identifier
 *     physical topology.
 *
 * Therefore:
 *
 *     @requires(...)
 *
 * can remain unchanged while semantic compilation determines whether a
 * particular realization satisfies the requirement.
 *
 * A target that cannot satisfy a semantic requirement must produce an explicit
 * capability/resource diagnostic rather than silently changing the annotation
 * meaning.
 *
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This grammar contains no Rust actions or predicates.
 *
 * The generated parser is consumed by the existing Zamani Rust frontend.
 *
 * Required implementation baseline:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The compiler/runtime implementation must use safe Rust only.
 *
 * This grammar requires no `unsafe`.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_ANNOTATIONS
 *     MAX_ANNOTATION_ARGUMENTS
 *     MAX_ANNOTATION_DEPTH
 *     MAX_ANNOTATION_LIST_SIZE
 *     MAX_ANNOTATION_MAP_SIZE
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It contains no fixed hardware topology.
 *
 * It contains no finite quantum-operation catalogue.
 *
 * It contains no target-specific backend selection.
 *
 *
 * ============================================================================
 * REQUIRED TEST MATRIX
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     @name
 *     @name()
 *     @name(1)
 *     @name("value")
 *     @name(true)
 *     @name(false)
 *     @name(foo)
 *     @name(namespace::value)
 *     @namespace::name
 *     @namespace::name()
 *     @name(key = 1)
 *     @name(key = "value")
 *     @name(a, b, c)
 *     @name(a, b,)
 *     @name(key = value,)
 *     @name([a, b, c])
 *     @name({key = value})
 *     @name(@nested)
 *     @outer(@inner(value))
 *     @name([@a, @b])
 *     @name({
 *         first = @a,
 *         second = @b,
 *     })
 *
 *
 * NEGATIVE
 * --------
 *
 *     @
 *     @()
 *     @name(
 *     @name)
 *     @name(=value)
 *     @name(key =)
 *     @name(key value)
 *     @name(,)
 *     @name(a,,b)
 *     @name({=value})
 *     @name({key})
 *     @name({key = })
 *
 *
 * BOUNDARY
 * --------
 *
 *     deeply qualified annotation names
 *     deeply nested annotation values
 *     large argument lists
 *     large lists
 *     large maps
 *     nested lists/maps
 *     nested annotations
 *     mixed positional/named arguments
 *     Unicode identifiers
 *     large numeric source literals
 *
 * Boundary tests must validate resource exhaustion behavior without encoding
 * a language-level artificial maximum.
 *
 *
 * CROSS-DOMAIN
 * -----------
 *
 * Annotations must parse in programs involving:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     data
 *     distributed
 *     networking
 *     security
 *     metaprogramming
 *     interoperability
 *     future dialects
 *
 *
 * DETERMINISM
 * -----------
 *
 * Identical token streams must produce identical parse structure.
 *
 *
 * SOURCE PRESERVATION
 * -------------------
 *
 * Annotation ordering, argument ordering and nested structure must survive:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *
 *
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * Before marking this file complete:
 *
 *     [ ] Canonical ZamaniLexer is available.
 *
 *     [ ] AT is exported by the canonical lexer.
 *
 *     [ ] IDENTIFIER is exported by the canonical lexer.
 *
 *     [ ] INTEGER is exported by the canonical lexer.
 *
 *     [ ] FLOAT is exported by the canonical lexer.
 *
 *     [ ] STRING is exported by the canonical lexer.
 *
 *     [ ] CHAR is exported by the canonical lexer.
 *
 *     [ ] TRUE is exported by the canonical lexer.
 *
 *     [ ] FALSE is exported by the canonical lexer.
 *
 *     [ ] Names exports qualifiedName.
 *
 *     [ ] ZamaniParser/Core imports this grammar through the canonical
 *         composition hierarchy.
 *
 *     [ ] Consumers use annotationPrefix rather than redefining annotation
 *         syntax.
 *
 *     [ ] AST construction preserves source structure.
 *
 *     [ ] Semantic annotation registry validates annotation identity.
 *
 *     [ ] Semantic validation owns argument schemas.
 *
 *     [ ] Resource/capability consumers interpret metadata downstream.
 *
 *     [ ] Quantum consumers cross the semantic quantum::ir boundary only
 *         after semantic analysis.
 *
 *     [ ] No annotation grammar contains target-specific resource limits.
 *
 *     [ ] Positive tests pass.
 *
 *     [ ] Negative tests pass.
 *
 *     [ ] Boundary tests pass.
 *
 *     [ ] Cross-domain tests pass.
 *
 *     [ ] Compatibility tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 *     [ ] Rust 1.97 / 1.97.1 integration passes.
 *
 *     [ ] Safe-Rust requirement remains satisfied.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * This file defines WHAT ANNOTATIONS LOOK LIKE.
 *
 * It does not define WHAT ANNOTATIONS MEAN.
 *
 * That separation is what allows the same annotation syntax to serve:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     data
 *     distributed
 *     networking
 *     security
 *     metaprogramming
 *     future computational domains
 *
 * without creating a second language or imposing hardware ceilings.
 *
 * ============================================================================
 */