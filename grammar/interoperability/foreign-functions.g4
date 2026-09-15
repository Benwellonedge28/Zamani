/*
 * ============================================================================
 * Zamani — Universal Foreign-Function / External-Interface Grammar
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/foreign-functions.g4
 *
 * Purpose:
 *     Defines the source-language syntax for declaring and invoking external
 *     callable interfaces without coupling Zamani source semantics to a
 *     particular ABI, processor, operating system, vendor, device, runtime,
 *     library format, programming language, or hardware topology.
 *
 * Architectural position:
 *
 *     Zamani source
 *          |
 *          v
 *       lexer
 *          |
 *          v
 *       parser
 *          |
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *   ExternalDeclaration              external call
 *          |                               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *                 semantic analysis
 *                          |
 *             +------------+-------------+
 *             |            |             |
 *             v            v             v
 *          type/ABI   capability      effects/
 *          resolution  resolution     resources
 *             |            |             |
 *             +------------+-------------+
 *                          |
 *                          v
 *                       ZUIR/IR
 *                          |
 *                          v
 *                    linking/runtime
 *
 * Ownership:
 *   This grammar owns SOURCE SYNTAX only.
 *
 * It does NOT own:
 *   - ABI definitions;
 *   - ABI compatibility;
 *   - calling-convention implementation;
 *   - dynamic library loading;
 *   - filesystem resolution;
 *   - network resolution;
 *   - symbol lookup;
 *   - linker implementation;
 *   - process execution;
 *   - runtime handles;
 *   - device handles;
 *   - CPU/GPU/QPU selection;
 *   - physical qubit IDs;
 *   - hardware topology;
 *   - scheduling;
 *   - routing;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - optimization;
 *   - canonical quantum IR;
 *   - canonical classical IR.
 *
 * POCO-REAF:
 *   External interfaces express a callable CONTRACT, not a machine.
 *
 *   They must therefore permit the same Zamani source interface to resolve
 *   differently on different targets, provided the target satisfies the
 *   declared semantic contract.
 *
 * Scalability:
 *   No source-level maximum is imposed on:
 *   - declarations;
 *   - parameters;
 *   - arguments;
 *   - interfaces;
 *   - external implementations;
 *   - resource counts;
 *   - devices;
 *   - nodes;
 *   - cores;
 *   - threads;
 *   - qubits;
 *   - memory;
 *   - address spaces.
 *
 * Security:
 *   Parsing an external declaration or call MUST NOT execute, load, resolve,
 *   open, connect to, or otherwise access the external implementation.
 *
 * Rust:
 *   Downstream compiler/runtime implementation target:
 *   - Rust 1.97
 *   - Rust 1.97.1
 *   - Edition 2021
 *   - unsafe code forbidden
 *
 * ============================================================================
 *
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar is intended to be imported by:
 *
 *     grammar/interoperability/interoperability.g4
 *
 * and ultimately composed into the authoritative Zamani grammar.
 *
 * The composing grammar MUST provide or import these foundational rules:
 *
 *     identifier
 *     qualifiedName
 *     stringLiteral
 *     typeExpr
 *     expression
 *     argumentList
 *     parameterList
 *     parameter
 *     visibilityModifier
 *     modifier
 *     annotation
 *     attribute
 *
 * The names above are contracts. If the repository chooses different names,
 * the adapter/composition grammar must map them; this file must not duplicate
 * their definitions.
 *
 * This file deliberately does not define a second identifier, type system,
 * expression grammar, annotation grammar, or argument grammar.
 *
 * ============================================================================
 */

parser grammar ForeignFunctions;

/*
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * The actual composition root supplies the shared lexical vocabulary and
 * foundational parser rules.
 *
 * This grammar therefore expects the importing grammar to make the following
 * token/rule names available:
 *
 *   identifier
 *   qualifiedName
 *   stringLiteral
 *   typeExpr
 *   expression
 *   argumentList
 *   parameterList
 *   parameter
 *   visibilityModifier
 *   modifier
 *   annotation
 *   attribute
 *
 * Do not introduce local duplicate definitions for those constructs.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TOP-LEVEL EXTERNAL INTERFACE DECLARATIONS
 * ============================================================================
 *
 * Two source forms are supported:
 *
 *   extern "source" { ... }
 *
 * and:
 *
 *   extern fn name(...) -> type;
 *
 * The first declares a named external interface/source containing one or more
 * callable declarations.
 *
 * The second declares a single external callable without requiring the source
 * identity to be fixed at source level.
 *
 * Neither form selects a concrete machine.
 * ============================================================================
 */

foreignDeclaration
    : foreignInterfaceDeclaration
    | foreignFunctionDeclaration
    ;


/*
 * External interface/source declaration.
 *
 * Examples:
 *
 *   extern "math" {
 *       fn sin(x: Real) -> Real;
 *       fn cos(x: Real) -> Real;
 *   }
 *
 *   extern "quantum-runtime" {
 *       fn submit(program: QuantumProgram) -> Result;
 *   }
 *
 *   extern "hdl-runtime" {
 *       fn configure(interface: HardwareInterface) -> Result;
 *   }
 *
 * The string is an opaque source-level identifier.
 *
 * It MUST NOT be interpreted by the grammar as:
 *   - a filesystem path;
 *   - a shared-library filename;
 *   - a URL;
 *   - a vendor name;
 *   - an ABI;
 *   - a device ID;
 *   - a backend ID.
 *
 * Those meanings are assigned by later semantic policy.
 */
foreignInterfaceDeclaration
    : attribute* visibilityModifier?
      'extern'
      stringLiteral
      '{'
      foreignMember*
      '}'
    ;


/*
 * An interface may be empty during incremental parsing/tooling workflows.
 *
 * Semantic analysis may impose stronger requirements where a complete
 * compilation unit is required.
 */
foreignMember
    : attribute* foreignFunctionDeclaration
    | attribute* foreignTypeDeclaration
    | attribute* foreignConstantDeclaration
    ;


/*
 * ============================================================================
 * EXTERNAL FUNCTION DECLARATIONS
 * ============================================================================
 *
 * These declarations describe callable signatures only.
 *
 * They do not describe:
 *   - register layouts;
 *   - stack layouts;
 *   - machine calling sequences;
 *   - binary symbol addresses;
 *   - platform-specific handles;
 *   - physical hardware.
 *
 * ABI lowering is a downstream compiler concern.
 * ============================================================================
 */

foreignFunctionDeclaration
    : visibilityModifier?
      modifier*
      'extern'
      'fn'
      identifier
      genericParameterClause?
      '(' parameterList? ')'
      foreignReturnClause?
      foreignEffectClause?
      foreignRequirementClause?
      foreignAttributeBlock?
      ';'
    ;


/*
 * Return type is optional to preserve compatibility with procedures/functions
 * whose return value is semantically absent.
 */
foreignReturnClause
    : '->' typeExpr
    ;


/*
 * Effects describe semantic behavior, not implementation mechanics.
 *
 * Examples of valid downstream effect concepts include:
 *
 *   io
 *   network
 *   device
 *   quantum
 *   hardware
 *   nondeterministic
 *   blocking
 *   distributed
 *
 * The actual effect vocabulary belongs to the canonical effects grammar.
 */
foreignEffectClause
    : 'with'
      'effects'
      '{'
      qualifiedName
      (',' qualifiedName)*
      '}'
    ;


/*
 * Requirements describe semantic/capability prerequisites.
 *
 * A requirement is deliberately different from:
 *
 *   target selection
 *   placement
 *   scheduling
 *   hardware discovery
 *   resource allocation
 *
 * Those are downstream concerns.
 */
foreignRequirementClause
    : 'requires'
      '{'
      foreignRequirement
      (',' foreignRequirement)*
      '}'
    ;


foreignRequirement
    : qualifiedName
      ( '=' expression )?
    ;


/*
 * Optional attribute block allows interoperability-specific metadata without
 * putting ABI/vendor/backend concepts into the core grammar.
 *
 * Example:
 *
 *   attributes {
 *       calling_convention = "..."
 *       linkage = "..."
 *   }
 *
 * The semantic layer decides whether an attribute is recognized and valid.
 */
foreignAttributeBlock
    : 'attributes'
      '{'
      foreignAttribute*
      '}'
    ;


foreignAttribute
    : identifier
      ( '=' expression )?
      ';'
    ;


/*
 * ============================================================================
 * GENERICS
 * ============================================================================
 *
 * Generic external interfaces allow a single declaration to describe an
 * interface over abstract types/resources without encoding concrete machine
 * sizes.
 *
 * The semantic type system owns generic constraints.
 * ============================================================================
 */

genericParameterClause
    : '<'
      genericParameter
      (',' genericParameter)*
      '>'
    ;


genericParameter
    : identifier
      genericParameterConstraint*
    ;


genericParameterConstraint
    : ':'
      qualifiedName
    ;


/*
 * ============================================================================
 * FOREIGN TYPE DECLARATIONS
 * ============================================================================
 *
 * An external type is an opaque source-level type identity.
 *
 * It MUST NOT contain:
 *   - raw machine addresses;
 *   - ABI-specific layout;
 *   - register counts;
 *   - physical device state;
 *   - pointers to runtime objects.
 *
 * Layout and representation are resolved later.
 * ============================================================================
 */

foreignTypeDeclaration
    : 'type'
      identifier
      foreignTypeParameters?
      foreignTypeRepresentation?
      ';'
    ;


foreignTypeParameters
    : '<'
      identifier
      (',' identifier)*
      '>'
    ;


foreignTypeRepresentation
    : ':'
      'opaque'
    | ':'
      typeExpr
    ;


/*
 * ============================================================================
 * FOREIGN CONSTANT DECLARATIONS
 * ============================================================================
 *
 * Constants are semantic declarations. Their actual storage/linkage is
 * determined later.
 * ============================================================================
 */

foreignConstantDeclaration
    : 'const'
      identifier
      ':'
      typeExpr
      ';'
    ;


/*
 * ============================================================================
 * CALL EXPRESSIONS
 * ============================================================================
 *
 * Calls remain ordinary semantic calls after parsing.
 *
 * The grammar provides explicit foreign-call syntax for cases where the
 * programmer intentionally crosses an external interface boundary.
 *
 * The call does not itself load or invoke anything.
 * ============================================================================
 */

foreignCallExpression
    : 'foreign'
      qualifiedName
      '('
      argumentList?
      ')'
    ;


/*
 * Explicit source-qualified call.
 *
 * Example:
 *
 *   foreign "math"::sin(x)
 *
 * The source identifier remains syntactic metadata.
 * Its interpretation is deferred.
 */
qualifiedForeignCallExpression
    : 'foreign'
      stringLiteral
      '::'
      qualifiedName
      '('
      argumentList?
      ')'
    ;


/*
 * ABI-independent explicit FFI call.
 *
 * The interface name and callable name are source-level identifiers.
 *
 * No binary/library format is implied.
 */
ffiCallExpression
    : 'ffi'
      'call'
      qualifiedName
      '('
      argumentList?
      ')'
    ;


/*
 * ============================================================================
 * STATEMENT FORMS
 * ============================================================================
 *
 * These forms support statement-oriented interoperability where the language's
 * ordinary expression grammar does not consume the returned value.
 * ============================================================================
 */

foreignCallStatement
    : foreignCallExpression ';'
    | qualifiedForeignCallExpression ';'
    | ffiCallExpression ';'
    ;


/*
 * ============================================================================
 * EXTERNAL FUNCTION POINTER / CALLABLE VALUE
 * ============================================================================
 *
 * External callable values may be passed around without binding them to a
 * concrete machine representation.
 * ============================================================================
 */

foreignCallableExpression
    : 'foreign'
      'ref'
      qualifiedName
    ;


/*
 * ============================================================================
 * LINKAGE DECLARATION
 * ============================================================================
 *
 * Linkage describes a semantic association between an external declaration
 * and an implementation identity.
 *
 * It does NOT load the implementation.
 *
 * The implementation may eventually be:
 *
 *   native code
 *   another language
 *   service
 *   accelerator
 *   quantum runtime
 *   HDL-generated component
 *   distributed endpoint
 *   future execution mechanism
 *
 * The actual resolver owns interpretation.
 * ============================================================================
 */

foreignLinkageDeclaration
    : 'link'
      qualifiedName
      foreignLinkageBody
    ;


foreignLinkageBody
    : '{'
      foreignLinkageItem*
      '}'
    ;


foreignLinkageItem
    : 'name' '=' stringLiteral ';'
    | 'kind' '=' qualifiedName ';'
    | 'version' '=' stringLiteral ';'
    | 'interface' '=' qualifiedName ';'
    | 'requires' '=' expression ';'
    | 'attribute' identifier '=' expression ';'
    ;


/*
 * ============================================================================
 * EXTERNAL RESOURCE REQUIREMENTS
 * ============================================================================
 *
 * A foreign interface may state semantic resource requirements.
 *
 * Example:
 *
 *   requires {
 *       quantum;
 *       network;
 *   }
 *
 * or:
 *
 *   requires {
 *       capability("...");
 *   }
 *
 * The grammar does not contain a fixed vocabulary of machine resources.
 * Resource semantics belong to the canonical resource/capability model.
 * ============================================================================
 */

foreignResourceRequirement
    : 'requires'
      '('
      expression
      ')'
    ;


/*
 * ============================================================================
 * CONDITIONAL AVAILABILITY
 * ============================================================================
 *
 * Availability is expressed as a semantic predicate.
 *
 * This is deliberately not:
 *
 *   if target == "x86"
 *
 * because source semantics must remain portable.
 *
 * A later capability/target resolver may evaluate the predicate against the
 * available compilation/execution environment.
 * ============================================================================
 */

foreignAvailabilityClause
    : 'available'
      'when'
      expression
    ;


/*
 * ============================================================================
 * VERSION / COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Version information is declarative metadata. It is not a requirement to
 * select a particular implementation unless semantic policy says so.
 * ============================================================================
 */

foreignCompatibilityClause
    : 'compatible'
      'with'
      foreignCompatibilityRequirement
    ;


foreignCompatibilityRequirement
    : qualifiedName
    | stringLiteral
    | expression
    ;


/*
 * ============================================================================
 * DECLARATION GROUP
 * ============================================================================
 *
 * Provides a reusable grouping form for tooling and semantic analysis.
 * ============================================================================
 */

foreignDeclarationGroup
    : 'foreign'
      'interface'
      identifier
      '{'
      foreignMember*
      '}'
    ;


/*
 * ============================================================================
 * CALL TARGET QUALIFICATION
 * ============================================================================
 *
 * Qualified names are deliberately delegated to the canonical name/path
 * grammar. This prevents this file from defining a second namespace model.
 * ============================================================================
 */

foreignTarget
    : qualifiedName
    | stringLiteral
      '::'
      qualifiedName
    ;


/*
 * ============================================================================
 * INTEGRATION NOTES
 * ============================================================================
 *
 * 1. LEXER
 * --------------------------------------------------------------------------
 * Required lexical tokens are supplied by the canonical Zamani lexer:
 *
 *   extern
 *   fn
 *   foreign
 *   ffi
 *   call
 *   ref
 *   interface
 *   type
 *   const
 *   opaque
 *   link
 *   name
 *   kind
 *   version
 *   compatible
 *   available
 *   when
 *   requires
 *   with
 *   effects
 *   attributes
 *
 * These must be introduced into the canonical keyword/token layer rather than
 * duplicated in this grammar.
 *
 *
 * 2. AST
 * --------------------------------------------------------------------------
 * `foreignInterfaceDeclaration` lowers to the repository's canonical
 * `ExternalDeclaration`.
 *
 * Individual external function/type/constant members must use the repository's
 * canonical declaration nodes rather than a parallel FFI AST hierarchy.
 *
 * This matches `src/frontend/ast/node/declarations/extern.rs`, whose contract
 * identifies `ExternalDeclaration` as the authoritative source-level
 * representation.
 *
 *
 * 3. EXPRESSIONS
 * --------------------------------------------------------------------------
 * `foreignCallExpression`,
 * `qualifiedForeignCallExpression`,
 * and `ffiCallExpression`
 * are source-level call forms.
 *
 * They must ultimately lower into the canonical call representation.
 *
 * This grammar MUST NOT introduce a second call IR.
 *
 *
 * 4. TYPES
 * --------------------------------------------------------------------------
 * `typeExpr` is imported from the canonical type grammar.
 *
 * Foreign functions therefore use the same Zamani type system as native
 * functions unless semantic analysis explicitly marks an opaque/external
 * representation.
 *
 *
 * 5. EFFECTS
 * --------------------------------------------------------------------------
 * `foreignEffectClause` integrates with the canonical effects subsystem.
 *
 * It does not define the effects themselves.
 *
 *
 * 6. CAPABILITIES / RESOURCES
 * --------------------------------------------------------------------------
 * `foreignRequirementClause` and related forms provide source-level
 * requirements.
 *
 * They do not perform hardware discovery.
 *
 * They do not select:
 *
 *   CPU
 *   GPU
 *   FPGA
 *   QPU
 *   node
 *   device
 *   topology
 *
 *
 * 7. QUANTUM
 * --------------------------------------------------------------------------
 * A foreign declaration may describe an interface implemented by a quantum
 * runtime, simulator, service, accelerator, or other backend.
 *
 * This grammar must never contain:
 *
 *   physical qubit IDs
 *   QPU handles
 *   coupling maps
 *   calibration
 *   pulse schedules
 *   QEC implementation
 *   ZQN noise models
 *
 * Quantum semantics are lowered through the canonical quantum IR boundary.
 *
 *
 * 8. HDL / HARDWARE
 * --------------------------------------------------------------------------
 * Foreign interfaces may represent HDL-generated components or hardware
 * services.
 *
 * The grammar does not encode a fixed number of ports, devices, registers,
 * lanes, cores, or accelerators.
 *
 *
 * 9. DISTRIBUTED COMPUTING
 * --------------------------------------------------------------------------
 * An external interface may ultimately resolve to a distributed service.
 *
 * Network transport, endpoint resolution, authentication, retries, placement,
 * scheduling and resilience remain downstream.
 *
 *
 * 10. SECURITY
 * --------------------------------------------------------------------------
 * Merely parsing or constructing a foreign declaration is inert.
 *
 * External resolution MUST be an explicit later compiler/runtime operation
 * subject to:
 *
 *   capability checks
 *   trust policy
 *   provenance
 *   sandbox policy
 *   permissions
 *   ABI/type validation
 *   resource policy
 *   runtime policy
 *
 *
 * 11. LINKER
 * --------------------------------------------------------------------------
 * Linkage metadata is consumed by the linking/target layer.
 *
 * This grammar does not assume ELF, PE, Mach-O, WASM, shared objects,
 * static archives, RPC, firmware, quantum services, FPGA bitstreams,
 * or any other concrete representation.
 *
 *
 * 12. RUNTIME
 * --------------------------------------------------------------------------
 * Runtime receives a resolved callable/interface contract.
 *
 * Runtime MUST NOT depend on parser-specific node structure.
 *
 *
 * 13. POCO-REAF
 * --------------------------------------------------------------------------
 * A Zamani program may declare:
 *
 *     extern fn compute(input: Data) -> Result;
 *
 * without declaring:
 *
 *     CPU model
 *     GPU model
 *     QPU model
 *     FPGA model
 *     number of devices
 *     memory size
 *     machine topology
 *
 * The same declaration can therefore participate in different compilation
 * and execution environments.
 *
 *
 * ============================================================================
 * NON-OWNERSHIP GUARANTEES
 * ============================================================================
 *
 * This grammar does not own:
 *
 *   ABI semantics
 *   calling convention semantics
 *   binary formats
 *   symbol resolution
 *   dynamic loading
 *   filesystem access
 *   networking
 *   credentials
 *   authentication
 *   hardware discovery
 *   hardware topology
 *   device allocation
 *   scheduling
 *   routing
 *   optimization
 *   QEC
 *   ZQN
 *   resilience
 *   quantum IR
 *   classical IR
 *   runtime state
 *
 *
 * ============================================================================
 * SCALABILITY GUARANTEES
 * ============================================================================
 *
 * No finite grammar constant limits:
 *
 *   number of interfaces
 *   number of functions
 *   number of parameters
 *   number of arguments
 *   number of external implementations
 *   number of resource requirements
 *   number of capabilities
 *   number of targets
 *
 * Any practical limits belong to configurable parser/compiler resource
 * policies and must not be encoded as language semantics.
 *
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic for a fixed lexer/token stream.
 *
 * This grammar contains:
 *
 *   no randomness
 *   no timestamps
 *   no external I/O
 *   no environment reads
 *   no filesystem reads
 *   no network access
 *   no target discovery
 *
 *
 * ============================================================================
 * ERROR-RECOVERY CONTRACT
 * ============================================================================
 *
 * The parser should produce ordinary ANTLR syntax diagnostics for malformed
 * declarations.
 *
 * Semantic diagnostics such as:
 *
 *   unknown external symbol
 *   incompatible type
 *   unsupported capability
 *   unavailable implementation
 *   ABI mismatch
 *   prohibited external effect
 *
 * belong to semantic analysis and MUST NOT be represented by grammar actions.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar intentionally contains no:
 *
 *   MAX_FUNCTIONS
 *   MAX_PARAMETERS
 *   MAX_ARGUMENTS
 *   MAX_DEVICES
 *   MAX_CORES
 *   MAX_THREADS
 *   MAX_QUBITS
 *   MAX_PORTS
 *   MAX_NODES
 *   MAX_MEMORY
 *   MAX_TARGETS
 *
 * There are no machine-specific constants.
 *
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The following tests must exist under:
 *
 *     grammar/tests/interoperability/
 *
 * Positive:
 *
 *   extern "math" {
 *       fn sin(x: Real) -> Real;
 *   }
 *
 *   extern fn compute(input: Data) -> Result;
 *
 *   foreign math::sin(x);
 *
 *   foreign "math"::sin(x);
 *
 *   ffi call compute(value);
 *
 *   extern "quantum-runtime" {
 *       fn submit(program: QuantumProgram) -> Result
 *           with effects { quantum };
 *   }
 *
 *   extern "hardware-runtime" {
 *       fn execute(program: Program) -> Result
 *           requires { hardware };
 *   }
 *
 * Generic:
 *
 *   extern "runtime" {
 *       fn map<T: Callable>(value: T) -> T;
 *   }
 *
 * Negative:
 *
 *   extern;
 *
 *   extern fn;
 *
 *   foreign;
 *
 *   foreign "source";
 *
 *   extern "source" { fn broken( -> Result; }
 *
 * Boundary:
 *
 *   zero external members
 *   one external member
 *   many members
 *   many parameters
 *   deeply qualified names
 *   nested generic constraints
 *
 * Cross-domain:
 *
 *   classical + foreign
 *   quantum + foreign
 *   hybrid + foreign
 *   HDL + foreign
 *   hardware + foreign
 *   distributed + foreign
 *   AI + foreign
 *
 * POCO-REAF:
 *
 *   The same source interface must parse identically without embedding a
 *   machine-specific target.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] The canonical lexer supplies all required tokens.
 * [ ] The canonical identifier/name grammar is reused.
 * [ ] The canonical expression grammar is reused.
 * [ ] The canonical type grammar is reused.
 * [ ] The canonical parameter grammar is reused.
 * [ ] The canonical argument grammar is reused.
 * [ ] No duplicate FFI AST is introduced.
 * [ ] External declarations lower to ExternalDeclaration.
 * [ ] External members use canonical declaration nodes.
 * [ ] Foreign calls lower to canonical call representation.
 * [ ] Effects integrate with the effects subsystem.
 * [ ] Requirements integrate with capabilities/resources.
 * [ ] ABI interpretation remains downstream.
 * [ ] Linking remains downstream.
 * [ ] Runtime loading remains downstream.
 * [ ] Hardware selection remains downstream.
 * [ ] Quantum hardware remains downstream.
 * [ ] QEC remains downstream.
 * [ ] ZQN remains downstream.
 * [ ] Resilience remains downstream.
 * [ ] No fixed machine/resource limits exist.
 * [ ] No filesystem/network access occurs during parsing.
 * [ ] No grammar action executes external code.
 * [ ] Positive tests exist.
 * [ ] Negative tests exist.
 * [ ] Boundary tests exist.
 * [ ] Cross-domain tests exist.
 * [ ] POCO-REAF tests exist.
 * [ ] ANTLR generation succeeds.
 * [ ] The Rust frontend implementation remains Rust 1.97/1.97.1 compatible.
 * [ ] The Rust implementation contains no unsafe code.
 */