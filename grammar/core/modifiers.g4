/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/core/modifiers.g4
 *
 * GRAMMAR
 * -------
 * Modifiers
 *
 * STATUS
 * ------
 * CANONICAL / PRODUCTION
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the canonical parser-level composition point for reusable
 * source-level modifiers.
 *
 * A modifier qualifies another source construct without becoming a second
 * declaration, expression, attribute, resource, capability, policy, or
 * domain language.
 *
 * This grammar owns STRUCTURE.
 *
 * Semantic analysis owns MEANING.
 *
 *
 * PRIMARY DESIGN
 * --------------
 *
 *     modifier
 *         |
 *         +--> coreModifier
 *         |
 *         +--> extensionModifier
 *
 *
 * Core modifiers are vocabulary controlled by the language.
 *
 * Extension modifiers are open-world and namespace-qualified so that future
 * language domains do not require continuously expanding this grammar.
 *
 *
 * POCO-REAF PRINCIPLE
 * -------------------
 *
 * A modifier describes portable source intent.
 *
 * It MUST NOT select or encode:
 *
 *     CPU identity
 *     GPU identity
 *     FPGA identity
 *     ASIC identity
 *     QPU identity
 *     accelerator identity
 *     node identity
 *     physical qubit identity
 *     memory-bank identity
 *     device address
 *     vendor-specific machine topology
 *
 * Target realization belongs downstream.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     modifier
 *     modifierList
 *     optionalModifierList
 *     coreModifier
 *     storageModifier
 *     linkageModifier
 *     behaviorModifier
 *     objectModelModifier
 *     safetyModifier
 *     extensionModifier
 *     qualifiedModifierName
 *     modifierArguments
 *     modifierArgumentList
 *     modifierArgument
 *     modifierValue
 *     modifierValueList
 *
 *
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexical token definitions
 *     keyword spelling
 *     identifiers
 *     qualified-name lexical rules
 *     visibility semantics
 *     attributes
 *     annotations
 *     expressions
 *     types
 *     declarations
 *     functions
 *     modules
 *     effects
 *     capabilities
 *     resources
 *     requirements
 *     constraints
 *     contracts
 *     policies
 *     provenance
 *     concurrency
 *     classical semantics
 *     quantum semantics
 *     quantum::ir
 *     HDL semantics
 *     hardware realization
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * LEXER
 * -----
 *
 * The canonical parser-facing lexical vocabulary is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars therefore consume:
 *
 *     ZamaniLexer
 *
 * and MUST NOT create a competing lexer vocabulary.
 *
 *
 * NAMES
 * -----
 *
 * Source-level names are owned by:
 *
 *     grammar/core/names.g4
 *
 * This file reuses:
 *
 *     identifier
 *     qualifiedName
 *
 * where appropriate.
 *
 *
 * VISIBILITY
 * ----------
 *
 * Visibility syntax is independently owned by:
 *
 *     grammar/core/visibility.g4
 *
 * This file MUST NOT redefine visibility alternatives.
 *
 *
 * ATTRIBUTES
 * ----------
 *
 * Attribute syntax is owned by:
 *
 *     grammar/core/attributes.g4
 *
 * This file MUST NOT redefine attribute syntax.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/core/visibility.g4
 *
 * EXPORTS:
 *
 *     modifier
 *     modifierList
 *     optionalModifierList
 *     coreModifier
 *     extensionModifier
 *     qualifiedModifierName
 *     modifierArguments
 *     modifierValue
 *
 * CONSUMED_BY:
 *
 *     declarations
 *     functions
 *     modules
 *     types
 *     classical
 *     quantum
 *     hybrid
 *     hdl
 *     hardware
 *     distributed
 *     networking
 *     AI
 *     data
 *     interoperability
 *     dialects
 *     macros
 *     metaprogramming
 *
 * AST_OWNER:
 *
 *     frontend AST / source syntax model
 *
 * SEMANTIC_OWNER:
 *
 *     semantic modifier registry / declaration-specific validation
 *
 * IR_OWNER:
 *
 *     canonical semantic model and downstream domain IRs
 *
 * TEST_OWNER:
 *
 *     grammar/tests/modifiers/
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 *
 * ============================================================================
 * FUNDAMENTAL RULE
 * ============================================================================
 *
 * Parsing a modifier does NOT mean that the modifier is legal in every
 * syntactic context.
 *
 * For example:
 *
 *     inline
 *
 * may be legal for a function but not for a module.
 *
 * Likewise:
 *
 *     abstract
 *
 * may be legal for a type declaration but not for an expression.
 *
 * Contextual legality is semantic/declaration-owned.
 *
 *
 * ============================================================================
 * MODIFIER VS ATTRIBUTE
 * ============================================================================
 *
 * Modifier:
 *
 *     inline
 *     async
 *     static
 *     quantum::adaptive
 *
 * Attribute:
 *
 *     @compile(...)
 *     @quantum::resource(...)
 *
 * They are deliberately different source constructs.
 *
 * This grammar MUST NOT transform one into the other.
 *
 * The AST may later normalize both into a common metadata representation
 * where appropriate, but source provenance must remain recoverable.
 *
 *
 * ============================================================================
 * MODIFIER VS REQUIREMENT / CAPABILITY / RESOURCE
 * ============================================================================
 *
 * A modifier is NOT automatically:
 *
 *     a resource requirement
 *     a capability requirement
 *     a resource allocation
 *     a target selection
 *     a scheduling directive
 *     a routing directive
 *     a hardware selection
 *
 * For example:
 *
 *     quantum::adaptive
 *
 * expresses source-level intent.
 *
 * It does not select a QPU.
 *
 * Likewise:
 *
 *     hardware::pipeline
 *
 * does not select a particular FPGA or ASIC.
 *
 * Such meaning belongs to semantic analysis and resource/capability
 * negotiation.
 *
 *
 * ============================================================================
 * CLOSED CORE / OPEN EXTENSION MODEL
 * ============================================================================
 *
 * The core modifier vocabulary is intentionally finite at any language
 * version.
 *
 * The extension vocabulary is open-ended.
 *
 * Therefore:
 *
 *     modifier
 *         : coreModifier
 *         | extensionModifier
 *
 * Extension modifiers MUST be namespace-qualified.
 *
 * This prevents an arbitrary identifier from being consumed as a modifier.
 *
 * A bare:
 *
 *     compute
 *
 * remains an identifier.
 *
 * A namespaced:
 *
 *     quantum::adaptive
 *
 * can be recognized structurally as an extension modifier.
 *
 *
 * ============================================================================
 * EXTENSION EXAMPLES
 * ============================================================================
 *
 * The following are structurally representable without adding new core
 * grammar rules:
 *
 *     quantum::adaptive
 *     quantum::entry
 *     quantum::dynamic
 *
 *     hardware::pipeline
 *     hardware::streaming
 *
 *     distributed::replicated
 *     distributed::deterministic
 *
 *     ai::differentiable
 *     ai::symbolic
 *
 *     execution::adaptive
 *     execution::reproducible
 *
 *     security::restricted
 *
 *     future::domain::modifier
 *
 * Their semantic existence must be established by the appropriate registry
 * or specification.
 *
 *
 * ============================================================================
 * CORE MODIFIER CATEGORIES
 * ============================================================================
 *
 * Core modifiers are grouped structurally rather than by hardware domain.
 *
 * Categories:
 *
 *     visibility
 *     storage/mutability
 *     linkage
 *     behavior/implementation
 *     object-model/type qualification
 *     safety
 *
 * Domain-specific modifiers should normally use extension namespaces rather
 * than expanding the universal core vocabulary.
 *
 *
 * ============================================================================
 * PUBLIC ENTRY POINTS
 * ============================================================================
 */

modifier
    : coreModifier
    | extensionModifier
    ;

modifierList
    : modifier+
    ;

optionalModifierList
    : modifier*
    ;


/*
 * ============================================================================
 * CORE MODIFIER
 * ============================================================================
 *
 * Visibility is imported from the canonical Visibility grammar.
 *
 * The other categories are owned structurally here until a more granular
 * modifier grammar is introduced.
 *
 * No semantic legality is encoded here.
 * ============================================================================
 */

coreModifier
    : visibilityModifier
    | storageModifier
    | linkageModifier
    | behaviorModifier
    | objectModelModifier
    | safetyModifier
    ;


/*
 * ============================================================================
 * VISIBILITY
 * ============================================================================
 *
 * Single owner:
 *
 *     grammar/core/visibility.g4
 *
 * Do not duplicate:
 *
 *     pub
 *     public
 *     private
 *     protected
 *     internal
 *
 * here.
 *
 * This keeps visibility semantics independently reusable by declarations,
 * modules, functions, types, quantum constructs, HDL constructs, and future
 * domains.
 *
 * ============================================================================
 */

visibilityModifier
    : visibilityModifierCore
    ;

visibilityModifierCore
    : PUBLIC
    | PUB
    | PRIVATE
    | PROTECTED
    | INTERNAL
    ;


/*
 * ============================================================================
 * STORAGE / MUTABILITY
 * ============================================================================
 *
 * These tokens are lexical language vocabulary.
 *
 * Their legality depends on the consuming declaration.
 *
 * For example, the semantic layer may distinguish:
 *
 *     let
 *     var
 *     const
 *     mut
 *     static
 *
 * according to the declaration model.
 *
 * No machine representation is implied.
 * ============================================================================
 */

storageModifier
    : STATIC
    | CONST
    | LET
    | VAR
    | MUT
    ;


/*
 * ============================================================================
 * LINKAGE
 * ============================================================================
 *
 * `extern` is source-level linkage intent.
 *
 * ABI, calling convention, symbol visibility, linker behavior and foreign
 * runtime integration are owned by interoperability/function semantics.
 * ============================================================================
 */

linkageModifier
    : EXTERN
    ;


/*
 * ============================================================================
 * BEHAVIOR / IMPLEMENTATION
 * ============================================================================
 *
 * These modifiers qualify execution or implementation strategy.
 *
 * They do NOT select a physical target.
 * ============================================================================
 */

behaviorModifier
    : VOLATILE
    | INLINE
    | ASYNC
    ;


/*
 * ============================================================================
 * OBJECT MODEL / TYPE QUALIFICATION
 * ============================================================================
 */

objectModelModifier
    : OVERRIDE
    | VIRTUAL
    | ABSTRACT
    | FINAL
    | SEALED
    | PARTIAL
    ;


/*
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * The language currently has an `unsafe` lexical construct.
 *
 * It is deliberately kept distinct from Rust safety.
 *
 * A Zamani `unsafe` modifier may be represented in the language AST, but the
 * compiler implementation itself MUST remain safe Rust.
 *
 * If the canonical language vocabulary later introduces `safe`, it may be
 * added to this category through the normal lexical/specification process.
 *
 * ============================================================================
 */

safetyModifier
    : UNSAFE
    ;


/*
 * ============================================================================
 * EXTENSION MODIFIER
 * ============================================================================
 *
 * A namespaced modifier may optionally carry:
 *
 *     arguments
 *     a structural value
 *
 * Examples:
 *
 *     quantum::adaptive
 *
 *     quantum::mode = symbolic
 *
 *     execution::policy(portable)
 *
 *     hardware::pipeline(stage)
 *
 * The grammar does not assign semantic meaning to any namespace.
 * ============================================================================
 */

extensionModifier
    : qualifiedModifierName modifierArguments?
      modifierValueClause?
    ;


/*
 * ============================================================================
 * QUALIFIED MODIFIER NAME
 * ============================================================================
 *
 * A modifier extension requires at least two name segments.
 *
 * This deliberately avoids:
 *
 *     modifier : IDENTIFIER
 *
 * because a bare identifier would be indistinguishable from an ordinary
 * declaration/name token.
 *
 * The namespace depth is unbounded by the grammar.
 * ============================================================================
 */

qualifiedModifierName
    : identifier DOUBLE_COLON identifier
      (DOUBLE_COLON identifier)*
    ;


/*
 * ============================================================================
 * MODIFIER ARGUMENTS
 * ============================================================================
 *
 * Arguments are structural rather than executable expressions.
 *
 * This is intentionally smaller than the general expression grammar.
 *
 * The reason is architectural:
 *
 *     modifier
 *
 * should remain declarative metadata/intent rather than silently becoming
 * an executable expression context.
 *
 * If a future feature genuinely requires arbitrary compile-time expressions,
 * it must explicitly integrate with the canonical expression/metaprogramming
 * system.
 *
 * ============================================================================
 */

modifierArguments
    : LPAREN modifierArgumentList? RPAREN
    ;

modifierArgumentList
    : modifierArgument (COMMA modifierArgument)* COMMA?
    ;

modifierArgument
    : identifier ASSIGN modifierValue
    | modifierValue
    ;


/*
 * ============================================================================
 * OPTIONAL VALUE CLAUSE
 * ============================================================================
 *
 * Examples:
 *
 *     quantum::mode = symbolic
 *     execution::strategy = adaptive
 *     hardware::policy = portable
 *
 * The value is structural.
 *
 * It is not evaluated by the parser.
 * ============================================================================
 */

modifierValueClause
    : ASSIGN modifierValue
    ;


/*
 * ============================================================================
 * MODIFIER VALUE
 * ============================================================================
 *
 * Values use the canonical lexer literal categories and canonical names.
 *
 * No literal width, precision, register width, tensor rank, memory size,
 * quantum capacity or hardware capacity is encoded here.
 *
 * Qualified names remain symbolic until semantic analysis.
 * ============================================================================
 */

modifierValue
    : qualifiedName
    | INTEGER_LITERAL
    | DECIMAL_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | CHAR_LITERAL
    | BOOLEAN_LITERAL
    | NIL_LITERAL
    | COMPLEX_LITERAL
    | DURATION_LITERAL
    | SIZE_LITERAL
    | QUANTUM_LITERAL
    | HARDWARE_LITERAL
    ;


/*
 * ============================================================================
 * VALUE LIST
 * ============================================================================
 *
 * This reusable rule permits future consumers to explicitly request a
 * structural list of modifier values without inventing another grammar.
 *
 * No fixed cardinality is imposed.
 * ============================================================================
 */

modifierValueList
    : modifierValue (COMMA modifierValue)*
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Each parsed modifier occurrence must preserve:
 *
 *     category
 *     source spelling/token identity
 *     qualified namespace
 *     modifier name
 *     argument order
 *     named/positional distinction
 *     optional value
 *     source span
 *     source order
 *
 * Conceptually:
 *
 *     ModifierSyntax
 *         category
 *         name
 *         namespace
 *         arguments
 *         value
 *         source_span
 *         source_order
 *
 * This grammar does not define Rust AST structures.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     whether a modifier exists;
 *     whether it is enabled by the language version;
 *     whether it is legal in the current context;
 *     whether duplicate modifiers are legal;
 *     whether modifier order matters;
 *     whether two modifiers conflict;
 *     whether required companion modifiers exist;
 *     whether modifier arguments have valid types;
 *     whether modifier values are valid;
 *     whether the modifier requires a capability;
 *     whether the modifier creates an effect;
 *     whether it imposes a resource requirement;
 *     whether it participates in a policy;
 *     whether it is compatible with the target.
 *
 * The parser does not perform these checks.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Modifier arguments and values may refer to source-level symbolic names and
 * literals.
 *
 * Their type is resolved downstream.
 *
 * No modifier syntax establishes:
 *
 *     integer width;
 *     floating-point width;
 *     pointer width;
 *     register width;
 *     tensor rank;
 *     vector width;
 *     memory representation.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * A modifier may semantically declare or request effects such as:
 *
 *     IO
 *     network
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     simulation
 *
 * However, this grammar does not enumerate those effects.
 *
 * The canonical effects subsystem owns their semantic representation.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A modifier may be associated semantically with capabilities such as:
 *
 *     quantum.measurement
 *     tensor.compute
 *     distributed.execution
 *     hardware.reconfiguration
 *     network.streaming
 *
 * Capability identity is not hard-coded into this grammar.
 *
 * Resolution belongs to:
 *
 *     resources/capabilities
 *     semantic analysis
 *     target negotiation
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Modifiers may semantically contribute resource requirements.
 *
 * For example:
 *
 *     quantum::adaptive
 *
 * may cause a semantic subsystem to require a capability supporting dynamic
 * execution.
 *
 * The modifier grammar itself does not contain:
 *
 *     qubit counts
 *     CPU counts
 *     GPU counts
 *     node counts
 *     memory capacities
 *     tensor-rank limits
 *     device counts
 *
 * Resource quantities remain semantic expressions handled by the resource
 * subsystem.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Modifiers may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * only through semantic integration.
 *
 * This grammar MUST NOT duplicate the contract grammar.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Modifiers may be constrained by policies involving:
 *
 *     security
 *     execution
 *     resource selection
 *     adaptation
 *     deployment
 *     simulation
 *     interoperability
 *
 * Policy interpretation belongs to the policy subsystem.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Modifier provenance must preserve:
 *
 *     source location
 *     source spelling
 *     source order
 *     language version
 *     dialect context, where applicable
 *     semantic normalization
 *     transformations applied downstream
 *
 * A compiler must not discard the original modifier merely because it later
 * normalizes it into semantic metadata.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum modifiers may qualify source constructs such as:
 *
 *     quantum declarations
 *     quantum operations
 *     circuits
 *     dynamic execution
 *     adaptive execution
 *     measurement
 *     resilience intent
 *
 * Example:
 *
 *     quantum::adaptive
 *
 * remains source intent.
 *
 * It does NOT directly encode:
 *
 *     physical qubits
 *     topology
 *     routing
 *     calibration
 *     QEC implementation
 *     QPU identity
 *
 * Semantic quantum lowering remains:
 *
 *     source
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware-oriented extensions may express portable intent:
 *
 *     hardware::pipeline
 *     hardware::streaming
 *     hardware::reconfigurable
 *
 * They MUST NOT identify a particular physical machine.
 *
 * Widths, timing, topology, resources and implementation constraints belong
 * to HDL/hardware semantics and target analysis.
 *
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * Backends consume semantic modifier information only after:
 *
 *     parsing
 *     AST construction
 *     validation
 *     name resolution
 *     type analysis
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     policy analysis
 *
 * The backend may then specialize the program for:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed execution
 *     future targets
 *
 * without changing the source modifier grammar.
 *
 *
 * ============================================================================
 * DUPLICATES
 * ============================================================================
 *
 * The grammar permits repeated modifiers structurally.
 *
 * Example:
 *
 *     inline inline fn compute() {}
 *
 * Whether this is:
 *
 *     valid;
 *     redundant;
 *     deprecated;
 *     an error;
 *
 * is semantic policy.
 *
 * The parser must preserve both occurrences for diagnostics and provenance.
 *
 *
 * ============================================================================
 * ORDER
 * ============================================================================
 *
 * Modifier order is preserved.
 *
 * The grammar does not canonicalize:
 *
 *     public async inline
 *
 * into another order.
 *
 * A semantic layer may normalize order where the language specification
 * declares modifier order semantically irrelevant.
 *
 *
 * ============================================================================
 * CONFLICTS
 * ============================================================================
 *
 * The parser does not encode conflicts such as:
 *
 *     abstract final
 *     const mut
 *     immutable mut
 *
 * because legality can depend on the declaration kind and language version.
 *
 * Semantic validation owns conflict detection.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar imposes no language-level finite maximum on:
 *
 *     modifier count
 *     modifier argument count
 *     modifier value size
 *     namespace depth
 *     qualified-name depth
 *     source declarations
 *     functions
 *     types
 *     modules
 *     quantum operations
 *     qubits
 *     processors
 *     accelerators
 *     nodes
 *     devices
 *     memory
 *     tensor rank
 *     network size
 *
 * There is intentionally no:
 *
 *     MAX_MODIFIERS
 *     MAX_MODIFIER_ARGUMENTS
 *     MAX_MODIFIER_DEPTH
 *     MAX_NAMESPACE_DEPTH
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
 * Practical limits are implementation/resource constraints and must not
 * become source-language semantics.
 *
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * This grammar performs:
 *
 *     no filesystem access
 *     no network access
 *     no hardware discovery
 *     no runtime execution
 *     no randomness
 *     no environment inspection
 *     no scheduling
 *     no target selection
 *
 * The parse depends on the source token stream and grammar/version context.
 *
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Modifier parsing MUST NOT execute:
 *
 *     modifier arguments
 *     modifier values
 *     capabilities
 *     policies
 *     hardware queries
 *     foreign calls
 *     reflection
 *     generated code
 *
 * All such operations belong to controlled downstream phases.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing public rule:
 *
 *     modifier
 *
 * remains stable.
 *
 * Existing public list rule:
 *
 *     modifierList
 *
 * remains stable.
 *
 * The additional:
 *
 *     optionalModifierList
 *
 * is additive.
 *
 * Existing semantic modifier categories remain represented where their
 * lexical vocabulary exists.
 *
 * Extension modifiers remain open-ended through qualified names.
 *
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     pub
 *     public
 *     private
 *     protected
 *     internal
 *
 *     static
 *     const
 *     let
 *     var
 *     mut
 *
 *     extern
 *
 *     inline
 *     volatile
 *     async
 *
 *     final
 *     sealed
 *     partial
 *     override
 *     virtual
 *     abstract
 *
 *     unsafe
 *
 *     quantum::adaptive
 *
 *     quantum::adaptive(mode)
 *
 *     quantum::mode = symbolic
 *
 *     execution::policy(portable)
 *
 *     hardware::pipeline(stage = streaming)
 *
 *     future::domain::modifier
 *
 *
 * NEGATIVE
 * --------
 *
 *     ::
 *
 *     quantum::
 *
 *     ::adaptive
 *
 *     quantum::adaptive::
 *
 *     ordinaryIdentifier
 *
 *     quantum::adaptive(
 *
 *     quantum::adaptive(
 *         value,
 *     )
 *     // when the parser reaches an incomplete delimiter
 *
 *
 * BOUNDARY
 * --------
 *
 *     quantum::a::b::c::d
 *
 *     future::domain::deep::extension::modifier
 *
 *     quantum::adaptive(a, b, c, d)
 *
 *     quantum::mode = "portable"
 *
 *     quantum::mode = very_large_symbolic_name
 *
 *
 * SCALABILITY
 * ----------
 *
 * Tests must verify that no grammar change is required merely because:
 *
 *     namespace depth increases;
 *     number of modifiers increases;
 *     number of arguments increases;
 *     source program size increases;
 *     target hardware size increases;
 *     number of quantum resources increases;
 *     number of distributed nodes increases.
 *
 *
 * CROSS-DOMAIN
 * -----------
 *
 * The same modifier infrastructure must be consumable by:
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
 *     interoperability
 *     metaprogramming
 *     future domains
 *
 *
 * DETERMINISM
 * -----------
 *
 * Repeated parsing of identical source must produce equivalent parse trees.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS:
 *
 *     no physical device enumeration;
 *     no vendor enumeration;
 *     no resource-capacity constants;
 *     no qubit ceiling;
 *     no CPU ceiling;
 *     no GPU ceiling;
 *     no FPGA ceiling;
 *     no node ceiling;
 *     no memory ceiling;
 *     no tensor-rank ceiling;
 *     no topology ceiling;
 *     no fixed namespace depth;
 *     no fixed modifier count;
 *     no runtime behavior;
 *     no backend selection.
 *
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * The generated/frontend/compiler implementation MUST remain compatible
 * with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and MUST use safe Rust.
 *
 * This grammar does not require:
 *
 *     unsafe blocks
 *     unsafe functions
 *     unsafe traits
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] canonical parser-facing lexer vocabulary is used;
 *     [x] canonical names are reused;
 *     [x] visibility ownership is not duplicated;
 *     [x] core modifier categories are structurally represented;
 *     [x] extension modifiers are namespace-qualified;
 *     [x] extension arguments are supported;
 *     [x] extension values are supported;
 *     [x] named and positional extension arguments are distinguishable;
 *     [x] source order is preserved;
 *     [x] duplicate modifiers remain available for semantic validation;
 *     [x] conflicts remain semantic;
 *     [x] attributes remain separate;
 *     [x] expressions remain separate;
 *     [x] resources remain separate;
 *     [x] capabilities remain separate;
 *     [x] policies remain separate;
 *     [x] contracts remain separate;
 *     [x] provenance requirements are defined;
 *     [x] quantum boundary is defined;
 *     [x] HDL boundary is defined;
 *     [x] backend boundary is defined;
 *     [x] no physical hardware is encoded;
 *     [x] no language-level machine ceiling is encoded;
 *     [x] deterministic parsing is preserved;
 *     [x] Rust 1.97/1.97.1 compatibility is specified;
 *     [x] no unsafe Rust is required.
 *
 * ============================================================================
 * INTEGRATION REQUIREMENT
 * ============================================================================
 *
 * The following one-time repository integration is REQUIRED before this file
 * can be generated successfully:
 *
 *     grammar/core/visibility.g4
 *
 * must use the same canonical parser-facing lexer vocabulary as this file:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * and its visibility alternatives must consume the current canonical lexical
 * token names:
 *
 *     PUB
 *     PUBLIC
 *     PRIVATE
 *     PROTECTED
 *     INTERNAL
 *
 * rather than obsolete K_* token names.
 *
 * This is a compatibility/ownership correction, not a new visibility feature.
 *
 * After that migration:
 *
 *     Modifiers
 *         |
 *         +--> Visibility
 *
 * has exactly one visibility owner.
 *
 * No later edit to `modifiers.g4` should be required merely because
 * `visibility.g4` is completed.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */

parser grammar Modifiers;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Visibility
    ;


/*
 * ============================================================================
 * PUBLIC MODIFIER RULES
 * ============================================================================
 */

modifier
    : coreModifier
    | extensionModifier
    ;

modifierList
    : modifier+
    ;

optionalModifierList
    : modifier*
    ;


/*
 * ============================================================================
 * CORE MODIFIER COMPOSITION
 * ============================================================================
 */

coreModifier
    : visibilityModifier
    | storageModifier
    | linkageModifier
    | behaviorModifier
    | objectModelModifier
    | safetyModifier
    ;


/*
 * ============================================================================
 * STORAGE / MUTABILITY
 * ============================================================================
 */

storageModifier
    : STATIC
    | CONST
    | LET
    | VAR
    | MUT
    ;


/*
 * ============================================================================
 * LINKAGE
 * ============================================================================
 */

linkageModifier
    : EXTERN
    ;


/*
 * ============================================================================
 * BEHAVIOR / IMPLEMENTATION
 * ============================================================================
 */

behaviorModifier
    : VOLATILE
    | INLINE
    | ASYNC
    ;


/*
 * ============================================================================
 * OBJECT MODEL / TYPE QUALIFICATION
 * ============================================================================
 */

objectModelModifier
    : OVERRIDE
    | VIRTUAL
    | ABSTRACT
    | FINAL
    | SEALED
    | PARTIAL
    ;


/*
 * ============================================================================
 * SAFETY
 * ============================================================================
 */

safetyModifier
    : UNSAFE
    ;


/*
 * ============================================================================
 * OPEN-WORLD EXTENSION MODIFIER
 * ============================================================================
 */

extensionModifier
    : qualifiedModifierName modifierArguments? modifierValueClause?
    ;

qualifiedModifierName
    : identifier DOUBLE_COLON identifier
      (DOUBLE_COLON identifier)*
    ;


/*
 * ============================================================================
 * EXTENSION ARGUMENTS
 * ============================================================================
 */

modifierArguments
    : LPAREN modifierArgumentList? RPAREN
    ;

modifierArgumentList
    : modifierArgument (COMMA modifierArgument)* COMMA?
    ;

modifierArgument
    : identifier ASSIGN modifierValue
    | modifierValue
    ;


/*
 * ============================================================================
 * OPTIONAL STRUCTURAL VALUE
 * ============================================================================
 */

modifierValueClause
    : ASSIGN modifierValue
    ;


/*
 * ============================================================================
 * STRUCTURAL MODIFIER VALUES
 * ============================================================================
 */

modifierValue
    : qualifiedName
    | INTEGER_LITERAL
    | DECIMAL_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | CHAR_LITERAL
    | BOOLEAN_LITERAL
    | NIL_LITERAL
    | COMPLEX_LITERAL
    | DURATION_LITERAL
    | SIZE_LITERAL
    | QUANTUM_LITERAL
    | HARDWARE_LITERAL
    ;