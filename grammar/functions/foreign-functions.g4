/**
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/foreign-functions.g4
 *
 * Grammar:
 *     ForeignFunctions
 *
 * Status:
 *     Production parser grammar
 *
 * Purpose:
 *     Canonical source-level grammar for foreign/external function
 *     declarations and foreign-interface contracts.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Zamani source
 *      |
 *      v
 * lexer
 *      |
 *      v
 * ForeignFunctions grammar
 *      |
 *      v
 * canonical frontend AST
 *      |
 *      v
 * structural validation
 *      |
 *      v
 * semantic analysis
 *      |
 *      +--------------------+
 *      |                    |
 *      v                    v
 * classical IR          quantum::ir
 *      |                    |
 *      +----------+---------+
 *                 |
 *                 v
 *        optimization / routing /
 *        scheduling / hardware /
 *        runtime / interoperability
 *
 * This grammar describes the SOURCE-LEVEL INTERFACE.
 *
 * It does not execute foreign functions.
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * A foreign declaration describes an externally supplied computation.
 *
 * It does NOT describe:
 *
 *     - a particular CPU;
 *     - a particular GPU;
 *     - a particular FPGA;
 *     - a particular ASIC;
 *     - a particular QPU;
 *     - a physical qubit;
 *     - a register address;
 *     - a memory address;
 *     - a device ID;
 *     - a network node;
 *     - a vendor implementation;
 *     - a fixed ABI implementation;
 *     - a dynamic-library handle;
 *     - a runtime handle;
 *     - a scheduler;
 *     - a routing decision;
 *     - a calibration;
 *     - a quantum backend;
 *     - a QEC implementation;
 *     - a ZQN implementation.
 *
 * Those are downstream concerns.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Foreign declarations MUST remain portable descriptions of an interface.
 *
 * The same source declaration may ultimately be implemented by:
 *
 *     - a Zamani function;
 *     - a native library;
 *     - C;
 *     - C++;
 *     - another safe systems language;
 *     - an operating-system service;
 *     - an accelerator;
 *     - an FPGA;
 *     - an ASIC;
 *     - a quantum runtime;
 *     - a simulator;
 *     - a distributed service;
 *     - a future execution environment.
 *
 * The grammar therefore records interface requirements and declarations,
 * rather than binding the program to one implementation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - foreign interface declarations;
 *     - external function declarations;
 *     - foreign function names;
 *     - optional source/interface identity;
 *     - foreign parameter declarations;
 *     - foreign return-type attachment;
 *     - foreign declaration attributes;
 *     - foreign interface requirements;
 *     - foreign interface linkage metadata;
 *     - foreign symbol metadata;
 *     - foreign calling-convention spelling as SOURCE METADATA only;
 *     - foreign variadic declaration syntax;
 *     - foreign declaration version metadata;
 *     - foreign declaration capability requirements;
 *     - foreign declaration effect attachment;
 *     - foreign declaration contract attachment.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - general function declarations;
 *       -> grammar/functions/functions.g4
 *
 *     - function types;
 *       -> grammar/types/function-types.g4
 *
 *     - general types;
 *       -> grammar/types/
 *
 *     - expressions;
 *       -> grammar/expressions/
 *
 *     - statements;
 *       -> grammar/statements/
 *
 *     - modules/packages;
 *       -> grammar/modules/
 *
 *     - ABI implementation;
 *       -> compiler/interoperability/runtime layers
 *
 *     - dynamic library loading;
 *       -> runtime/interoperability layer
 *
 *     - linker behavior;
 *       -> compiler/linking layer
 *
 *     - calling-convention implementation;
 *       -> target/backend layer
 *
 *     - memory layout;
 *       -> target/type/code-generation layers
 *
 *     - hardware discovery;
 *       -> hardware abstraction layer
 *
 *     - quantum IR;
 *       -> quantum::ir
 *
 *     - QEC;
 *       -> quantum QEC subsystem
 *
 *     - ZQN;
 *       -> quantum ZQN subsystem
 *
 *     - routing;
 *       -> routing subsystem
 *
 *     - scheduling;
 *       -> scheduling subsystem
 *
 *     - optimization;
 *       -> optimization subsystem
 *
 *     - runtime dispatch;
 *       -> runtime
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar consumes:
 *
 *     grammar/lexer/tokens.g4
 *     grammar/core/names.g4
 *     grammar/core/qualified-names.g4
 *     grammar/types/
 *     grammar/effects/
 *     grammar/core/capabilities.g4
 *     grammar/core/requirements.g4
 *     grammar/core/constraints.g4
 *     grammar/core/attributes.g4
 *     grammar/expressions/
 *
 * through parser composition.
 *
 * It does NOT redefine those grammars.
 *
 * The composed parser MUST provide the following canonical rules:
 *
 *     qualifiedName
 *     typeExpression
 *     expression
 *     effectClause
 *     capabilityRequirement
 *     requirementClause
 *     constraintClause
 *     attribute
 *
 * If the repository chooses different canonical rule names, the composition
 * layer MUST provide compatibility aliases rather than duplicating the
 * underlying grammar.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar uses the canonical ZamaniTokens vocabulary.
 *
 * It MUST NOT define lexer tokens.
 *
 * Relevant canonical tokens include:
 *
 *     K_EXTERN
 *     K_FN
 *     K_PUB
 *     K_PUBLIC
 *     K_PRIVATE
 *     K_PROTECTED
 *     K_INTERNAL
 *     K_STATIC
 *     K_CONST
 *     K_MUT
 *     K_ASYNC
 *     K_SAFE
 *     K_UNSAFE
 *     K_VOLATILE
 *     K_INLINE
 *     K_FINAL
 *     K_ABSTRACT
 *     K_OVERRIDE
 *     K_WITH
 *     K_AS
 *     K_FROM
 *     K_USE
 *     K_IMPORT
 *
 *     IDENTIFIER
 *     STRING_LITERAL
 *     ELLIPSIS
 *     THIN_ARROW
 *     DOUBLE_COLON
 *     EQUAL_EQUAL
 *     COMMA
 *     DOT
 *     COLON
 *     SEMICOLON
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *
 * The lexer is authoritative for token spelling.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "Is this a syntactically valid foreign declaration?"
 *
 * Semantic analysis answers:
 *
 *     - Does the foreign symbol exist?
 *     - Does the source identity resolve?
 *     - Is the type compatible?
 *     - Is the selected ABI compatible?
 *     - Is the calling convention supported?
 *     - Are ownership rules compatible?
 *     - Are effects compatible?
 *     - Are capabilities available?
 *     - Are resource requirements satisfiable?
 *     - Is the target capable of providing the interface?
 *     - Is the external implementation trusted?
 *     - Is dynamic loading permitted?
 *     - Is the interface safe to invoke?
 *
 * The grammar MUST NOT answer those questions.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * The grammar introduces no executable foreign-function behavior.
 *
 * It cannot:
 *
 *     - load a library;
 *     - open a file;
 *     - access a network;
 *     - resolve a symbol;
 *     - execute native code;
 *     - invoke an ABI;
 *     - access hardware.
 *
 * Rust implementations consuming this grammar MUST use:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *
 * and MUST NOT require unsafe Rust.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are no grammar-level maximums for:
 *
 *     foreign interfaces;
 *     foreign functions;
 *     parameters;
 *     generic parameters;
 *     declarations;
 *     source size;
 *     interface metadata;
 *     implementation alternatives;
 *     capabilities;
 *     requirements;
 *     effects.
 *
 * Repetition is therefore represented structurally and bounded only by
 * implementation/resource policy.
 *
 * The grammar MUST NOT contain:
 *
 *     MAX_FOREIGN_FUNCTIONS
 *     MAX_PARAMETERS
 *     MAX_ABI_ENTRIES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_THREADS
 *
 * or equivalent machine-specific ceilings.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST be deterministic.
 *
 * This grammar MUST NOT depend on:
 *
 *     - wall-clock time;
 *     - randomness;
 *     - process state;
 *     - machine topology;
 *     - environment variables;
 *     - filesystem state;
 *     - network state;
 *     - backend availability.
 *
 * ============================================================================
 * SOURCE COMPATIBILITY
 * ============================================================================
 *
 * The canonical legacy Zamani form:
 *
 *     extern "library" {
 *         fn helper(x: int) -> int;
 *     }
 *
 * remains representable.
 *
 * A direct foreign binding may be represented as:
 *
 *     foreign library::helper(x);
 *
 * only where the surrounding expression/call grammar explicitly permits
 * foreign calls.
 *
 * This file owns the DECLARATION contract, not expression-call semantics.
 *
 * ============================================================================
 */

parser grammar ForeignFunctions;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. TOP-LEVEL FOREIGN DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     extern "source" {
 *         fn name(...);
 *     }
 *
 *     extern {
 *         fn name(...);
 *     }
 *
 *     extern "source" @attribute {
 *         fn name(...);
 *     }
 *
 * The source identifier is intentionally opaque at grammar level.
 *
 * It MUST NOT be interpreted as:
 *
 *     - a filesystem path;
 *     - a URL;
 *     - a dynamic-library filename;
 *     - a package;
 *     - a device;
 *     - a vendor;
 *     - a backend.
 *
 * Such interpretation belongs to semantic/linking policy.
 * ============================================================================
 */

foreignFunctionDeclaration
    : foreignDeclarationModifiers*
      K_EXTERN
      foreignSource?
      foreignDeclarationAttribute*
      LBRACE
      foreignFunctionMember*
      RBRACE
    ;


/* ============================================================================
 * 2. FOREIGN SOURCE
 * ============================================================================
 *
 * The source is an opaque source-level identity.
 *
 * Examples may include:
 *
 *     extern "libmath" { ... }
 *     extern "platform.math" { ... }
 *     extern "service.math" { ... }
 *     extern "quantum.interface" { ... }
 *
 * The grammar does not assign implementation semantics to the string.
 * ============================================================================
 */

foreignSource
    : STRING_LITERAL
    ;


/* ============================================================================
 * 3. FOREIGN DECLARATION MODIFIERS
 * ============================================================================
 *
 * These are source-level modifiers only.
 *
 * Their legality and meaning are semantic.
 *
 * No modifier here selects a physical target.
 * ============================================================================
 */

foreignDeclarationModifiers
    : K_PUB
    | K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    | K_STATIC
    | K_CONST
    | K_INLINE
    | K_FINAL
    | K_ABSTRACT
    | K_EXTERN
    ;


/* ============================================================================
 * 4. FOREIGN DECLARATION ATTRIBUTES
 * ============================================================================
 *
 * Attributes are deliberately delegated to the canonical attribute grammar.
 *
 * The composed parser supplies:
 *
 *     attribute
 *
 * This grammar does not create a second attribute language.
 * ============================================================================
 */

foreignDeclarationAttribute
    : attribute
    ;


/* ============================================================================
 * 5. FOREIGN INTERFACE MEMBERS
 * ============================================================================
 *
 * A foreign interface may contain function declarations.
 *
 * Future repository extensions MAY add additional interface members, but they
 * must be introduced explicitly rather than silently treating arbitrary source
 * text as an external declaration.
 * ============================================================================
 */

foreignFunctionMember
    : foreignFunction
    ;


/* ============================================================================
 * 6. FOREIGN FUNCTION
 * ============================================================================
 *
 * A foreign function is a declaration, never a definition.
 *
 * Therefore:
 *
 *     fn foo(...);
 *
 * is valid.
 *
 * A body:
 *
 *     { ... }
 *
 * is deliberately NOT accepted.
 *
 * Implementations are supplied elsewhere.
 * ============================================================================
 */

foreignFunction
    : foreignFunctionModifiers*
      K_FN
      foreignFunctionName
      foreignGenericParameters?
      LPAREN
      foreignParameterList?
      RPAREN
      foreignReturnClause?
      foreignEffectClause?
      foreignRequirementClause*
      foreignCapabilityClause*
      foreignFunctionAttribute*
      SEMICOLON
    ;


/* ============================================================================
 * 7. FOREIGN FUNCTION MODIFIERS
 * ============================================================================
 *
 * `extern` is intentionally accepted for compatibility with source that
 * repeats the external nature at function level.
 *
 * Semantic analysis MUST determine whether duplicate `extern` modifiers are
 * redundant, permitted, deprecated, or invalid for the selected language
 * version.
 * ============================================================================
 */

foreignFunctionModifiers
    : K_PUB
    | K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    | K_STATIC
    | K_CONST
    | K_MUT
    | K_ASYNC
    | K_VOLATILE
    | K_INLINE
    | K_FINAL
    | K_ABSTRACT
    | K_OVERRIDE
    | K_EXTERN
    | K_SAFE
    ;


/* ============================================================================
 * 8. FOREIGN FUNCTION NAME
 * ============================================================================
 */

foreignFunctionName
    : IDENTIFIER
    ;


/* ============================================================================
 * 9. FOREIGN GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic foreign interfaces remain symbolic.
 *
 * They do not represent physical resources.
 * ============================================================================
 */

foreignGenericParameters
    : LESS_THAN
      foreignGenericParameter
      (COMMA foreignGenericParameter)*
      COMMA?
      GREATER_THAN
    ;


foreignGenericParameter
    : IDENTIFIER
      foreignGenericParameterBound?
    ;


foreignGenericParameterBound
    : COLON
      typeExpression
    ;


/* ============================================================================
 * 10. FOREIGN PARAMETERS
 * ============================================================================
 *
 * Ordinary parameters.
 *
 * No fixed parameter limit is encoded.
 * ============================================================================
 */

foreignParameterList
    : foreignParameter
      (COMMA foreignParameter)*
      COMMA?
    ;


/* ============================================================================
 * 11. FOREIGN PARAMETER
 * ============================================================================
 *
 * The grammar deliberately uses canonical typeExpression.
 *
 * It does not introduce ABI-specific types such as:
 *
 *     c_int
 *     llvm_i32
 *     cuda_ptr
 *     qir_qubit
 *
 * Such representations belong to interoperability/type-lowering layers.
 * ============================================================================
 */

foreignParameter
    : foreignParameterModifier*
      foreignParameterName
      (COLON typeExpression)?
      foreignParameterDefault?
    ;


foreignParameterModifier
    : K_MUT
    | K_CONST
    | K_VOLATILE
    ;


foreignParameterName
    : IDENTIFIER
    ;


foreignParameterDefault
    : EQUAL_EQUAL
      expression
    ;


/* ============================================================================
 * 12. VARIADIC FOREIGN PARAMETERS
 * ============================================================================
 *
 * A variadic marker is source syntax only.
 *
 * Semantic analysis determines whether the selected external interface can
 * actually support variadic calls.
 *
 * No finite argument count is encoded.
 * ============================================================================
 */

foreignVariadicParameter
    : foreignParameterModifier*
      ELLIPSIS
      foreignParameterName
      (COLON typeExpression)?
    ;


/*
 * Explicit list form preserving source order.
 *
 * The semantic layer MUST enforce:
 *
 *     - placement rules;
 *     - uniqueness;
 *     - at most one variadic parameter where required;
 *     - compatibility with the resolved interface.
 */

foreignParameterListWithVariadic
    : foreignParameter
      (COMMA foreignParameter)*
      COMMA
      foreignVariadicParameter
      COMMA?
    | foreignVariadicParameter
    ;


/* ============================================================================
 * 13. FOREIGN RETURN TYPE
 * ============================================================================
 */

foreignReturnClause
    : THIN_ARROW
      typeExpression
    ;


/* ============================================================================
 * 14. FOREIGN EFFECTS
 * ============================================================================
 *
 * Effect syntax remains owned by grammar/effects/.
 *
 * The composition layer supplies effectClause.
 *
 * This wrapper prevents foreign-functions.g4 from creating another effect
 * language.
 * ============================================================================
 */

foreignEffectClause
    : effectClause
    ;


/* ============================================================================
 * 15. FOREIGN REQUIREMENTS
 * ============================================================================
 *
 * Requirements describe semantic prerequisites.
 *
 * They do NOT select a particular machine.
 * ============================================================================
 */

foreignRequirementClause
    : requirementClause
    ;


/* ============================================================================
 * 16. FOREIGN CAPABILITIES
 * ============================================================================
 *
 * Capabilities describe what an implementation must provide.
 *
 * A capability is not a device identifier.
 *
 * Example semantic distinction:
 *
 *     requires quantum
 *
 * is not equivalent to:
 *
 *     use qpu_17
 *
 * The latter belongs to deployment/target configuration.
 * ============================================================================
 */

foreignCapabilityClause
    : capabilityRequirement
    ;


/* ============================================================================
 * 17. EXPLICIT FOREIGN SYMBOL DECLARATION
 * ============================================================================
 *
 * This rule provides a structured symbol-binding form for implementations
 * that need to distinguish the Zamani source name from an external symbol.
 *
 * Example:
 *
 *     fn add(a: int, b: int) -> int
 *         symbol "external_add";
 *
 * The actual symbol-resolution policy remains semantic.
 *
 * This rule is optional syntax and must only be enabled by the composed
 * language profile if the canonical specification accepts symbol metadata.
 * ============================================================================
 */

foreignSymbolBinding
    : K_EXTERN
      IDENTIFIER
      DOUBLE_COLON
      STRING_LITERAL
    ;


/* ============================================================================
 * 18. EXPLICIT SOURCE-TO-SYMBOL BINDING
 * ============================================================================
 *
 * This is metadata, not execution.
 *
 * Example:
 *
 *     bind "zamani.math" :: "sqrt";
 *
 * The exact use of this production is intentionally left to the semantic
 * composition layer.
 * ============================================================================
 */

foreignSymbolReference
    : STRING_LITERAL
      DOUBLE_COLON
      IDENTIFIER
    ;


/* ============================================================================
 * 19. FOREIGN INTERFACE NAME
 * ============================================================================
 *
 * Qualified names are consumed through the canonical name grammar.
 *
 * This prevents foreign interfaces from creating a second namespace model.
 * ============================================================================
 */

foreignQualifiedName
    : qualifiedName
    ;


/* ============================================================================
 * 20. FOREIGN DECLARATION WITH QUALIFIED SOURCE
 * ============================================================================
 *
 * This is an explicit syntactic boundary for future interoperability profiles.
 *
 * The qualified name remains source metadata.
 * ============================================================================
 */

foreignNamedInterface
    : foreignQualifiedName
      LBRACE
      foreignFunctionMember*
      RBRACE
    ;


/* ============================================================================
 * 21. FOREIGN CALL INTEGRATION CONTRACT
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This file does NOT own the ordinary expression:
 *
 *     library::function(...)
 *
 * nor:
 *
 *     foreign library::function(...)
 *
 * Those are call-expression concerns and belong to the canonical expressions
 * / interoperability grammar.
 *
 * This prevents the same call syntax from being parsed independently by:
 *
 *     functions/foreign-functions.g4
 *     expressions/calls.g4
 *     interoperability/
 *
 * There must be one canonical call-expression production.
 *
 * ============================================================================
 */


/* ============================================================================
 * 22. ABI INTEGRATION CONTRACT
 * ============================================================================
 *
 * ABI syntax MUST remain declarative.
 *
 * If the language eventually permits:
 *
 *     abi "C"
 *     abi "system"
 *     calling_convention "..."
 *
 * those values MUST be represented as source metadata and interpreted by the
 * interoperability/compiler layer.
 *
 * They MUST NOT:
 *
 *     - generate native calls;
 *     - load libraries;
 *     - select hardware;
 *     - allocate memory;
 *     - access registers;
 *     - access physical addresses.
 *
 * The grammar deliberately does not encode a fixed list of vendor ABI names.
 *
 * This is necessary for POCO-REAF and future interoperability.
 *
 * ============================================================================
 */


/* ============================================================================
 * 23. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Foreign functions MAY expose quantum-related interfaces.
 *
 * Example:
 *
 *     extern "quantum.service" {
 *         fn execute(circuit: QuantumCircuit) -> Measurement;
 *     }
 *
 * This grammar does not define:
 *
 *     - QPU topology;
 *     - physical qubit IDs;
 *     - gate calibration;
 *     - QEC;
 *     - ZQN;
 *     - scheduling;
 *     - routing.
 *
 * If the type is quantum-specific, its canonical type grammar owns the syntax.
 *
 * Later semantic lowering MAY map an external call into:
 *
 *     quantum::ir
 *
 * where appropriate.
 *
 * The grammar itself MUST NEVER construct or duplicate quantum IR.
 *
 * ============================================================================
 */


/* ============================================================================
 * 24. HARDWARE INTEGRATION
 * ============================================================================
 *
 * Foreign functions MAY expose hardware-oriented interfaces.
 *
 * Example:
 *
 *     extern "accelerator" {
 *         fn transform(data: Tensor) -> Tensor;
 *     }
 *
 * The declaration does not specify:
 *
 *     GPU count;
 *     FPGA count;
 *     memory size;
 *     vector width;
 *     device address;
 *     PCI identifier;
 *     accelerator topology.
 *
 * Hardware capability resolution occurs downstream.
 *
 * ============================================================================
 */


/* ============================================================================
 * 25. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A foreign function MAY represent a distributed service.
 *
 * The grammar does not encode:
 *
 *     node count;
 *     node identity;
 *     topology;
 *     network address;
 *     cluster size;
 *     deployment location.
 *
 * Those are deployment/runtime concerns.
 *
 * ============================================================================
 */


/* ============================================================================
 * 26. OWNERSHIP / MEMORY INTEGRATION
 * ============================================================================
 *
 * Foreign parameters may use canonical Zamani ownership/type constructs.
 *
 * This grammar does not invent an ABI memory model.
 *
 * Semantic interoperability must determine whether:
 *
 *     ownership;
 *     borrowing;
 *     lifetime;
 *     mutability;
 *     transfer;
 *     copying;
 *     aliasing
 *
 * can be represented safely by the foreign interface.
 *
 * ============================================================================
 */


/* ============================================================================
 * 27. SECURITY INTEGRATION
 * ============================================================================
 *
 * A foreign declaration is an authority boundary.
 *
 * The grammar itself does not grant authority.
 *
 * Semantic analysis and runtime policy MUST determine:
 *
 *     - whether the interface is permitted;
 *     - whether the source is trusted;
 *     - whether execution is sandboxed;
 *     - whether capabilities are granted;
 *     - whether network/filesystem access is permitted;
 *     - whether the foreign implementation is allowed.
 *
 * A source declaration MUST NOT implicitly bypass the security model.
 *
 * ============================================================================
 */


/* ============================================================================
 * 28. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics SHOULD identify:
 *
 *     - missing foreign source;
 *     - malformed foreign function;
 *     - missing function name;
 *     - malformed parameter list;
 *     - malformed return clause;
 *     - malformed generic parameter list;
 *     - invalid declaration termination;
 *     - unexpected function body;
 *     - malformed foreign metadata.
 *
 * Semantic diagnostics belong to semantic analysis.
 *
 * Examples:
 *
 *     foreign symbol not found
 *     ABI incompatible
 *     capability unavailable
 *     foreign source not permitted
 *     foreign interface type incompatible
 *
 * MUST NOT be reported by this grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 29. AST INTEGRATION
 * ============================================================================
 *
 * The parser MUST lower:
 *
 *     foreignFunctionDeclaration
 *
 * into the repository's canonical external declaration AST.
 *
 * The current repository already defines an ExternalDeclaration concept for:
 *
 *     extern "source" { ... }
 *
 * and deliberately stores canonical child NodeIds rather than embedding
 * duplicate AST hierarchies.
 *
 * Therefore the parser integration contract is:
 *
 *     foreignFunctionDeclaration
 *              |
 *              v
 *     ExternalDeclaration
 *              |
 *              +---- NodeId -> external function declaration
 *              |
 *              +---- NodeId -> external function declaration
 *              |
 *              +---- ...
 *
 * This grammar MUST NOT introduce:
 *
 *     ForeignFunctionAst
 *     ForeignAbiAst
 *     ForeignRuntimeAst
 *
 * if equivalent canonical AST nodes already exist.
 *
 * ============================================================================
 */


/* ============================================================================
 * 30. SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Semantic analysis MUST resolve:
 *
 *     source identity;
 *     namespace;
 *     function identity;
 *     parameter types;
 *     result types;
 *     generic constraints;
 *     effects;
 *     requirements;
 *     capabilities;
 *     ownership;
 *     ABI compatibility;
 *     implementation availability;
 *     security policy.
 *
 * No such resolution occurs during parsing.
 *
 * ============================================================================
 */


/* ============================================================================
 * 31. IR INTEGRATION
 * ============================================================================
 *
 * A foreign declaration normally does not become an executable IR operation.
 *
 * It supplies interface metadata.
 *
 * A foreign call expression may subsequently lower into the appropriate
 * canonical IR representation.
 *
 * For quantum calls:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic resolution
 *       |
 *       v
 *     quantum::ir
 *
 * where the call is semantically a quantum operation.
 *
 * The grammar MUST NOT duplicate quantum::ir.
 *
 * ============================================================================
 */


/* ============================================================================
 * 32. COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler stages consume the resolved semantic declaration.
 *
 * Possible later stages include:
 *
 *     name resolution
 *     type checking
 *     effect checking
 *     capability checking
 *     ABI checking
 *     interface resolution
 *     linking
 *     lowering
 *     optimization
 *     scheduling
 *     routing
 *     target selection
 *     code generation
 *
 * None of those stages is owned by this grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 33. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime integration may resolve a foreign implementation dynamically or
 * statically according to runtime policy.
 *
 * This grammar imposes no requirement that the implementation be:
 *
 *     native;
 *     local;
 *     synchronous;
 *     CPU-based;
 *     classical;
 *     quantum;
 *     single-device;
 *     single-node.
 *
 * Runtime behavior is downstream.
 *
 * ============================================================================
 */


/* ============================================================================
 * 34. NO UNSAFE REQUIREMENT
 * ============================================================================
 *
 * Nothing in this grammar requires unsafe Rust.
 *
 * Foreign execution implementations MUST be isolated behind safe Rust
 * abstractions.
 *
 * This grammar therefore remains compatible with the repository requirement:
 *
 *     Rust 1.97 / 1.97.1
 *     Edition 2021
 *     no unsafe
 *
 * ============================================================================
 */


/* ============================================================================
 * 35. COMPATIBILITY
 * ============================================================================
 *
 * Existing canonical syntax:
 *
 *     extern "library" {
 *         fn foo(x: int) -> int;
 *     }
 *
 * MUST remain parseable.
 *
 * Existing direct-call syntax must remain owned by the appropriate expression
 * grammar rather than being duplicated here.
 *
 * Changes to this grammar MUST be classified as:
 *
 *     Equivalent
 *     Compatible Extension
 *     Deprecated
 *     Breaking
 *     Experimental
 *     Reserved
 *
 * according to:
 *
 *     grammar/specification/compatibility.md
 *
 * ============================================================================
 */


/* ============================================================================
 * 36. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The grammar MUST parse foreign interfaces containing:
 *
 *     zero functions;
 *     one function;
 *     many functions;
 *     many parameters;
 *     generic parameters;
 *     variadic declarations;
 *     long qualified names;
 *     large type expressions;
 *     large metadata sets.
 *
 * No artificial language maximum is permitted.
 *
 * Extremely large inputs may still be rejected by configurable parser/resource
 * policies. Such limits are implementation/resource limits, not grammar
 * semantics.
 *
 * ============================================================================
 */


/* ============================================================================
 * 37. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * Minimum positive examples:
 *
 *     extern "math" {
 *         fn sqrt(x: float) -> float;
 *     }
 *
 *     extern {
 *         fn log(x: float) -> float;
 *     }
 *
 *     extern "quantum.interface" {
 *         fn execute(circuit: QuantumCircuit) -> Measurement;
 *     }
 *
 *     extern "accelerator" {
 *         fn transform(data: Tensor) -> Tensor;
 *     }
 *
 *     extern "service" {
 *         fn request<T>(value: T) -> T;
 *     }
 *
 *     extern "native" {
 *         fn printf(...args: str) -> int;
 *     }
 *
 * Exact availability of QuantumCircuit, Measurement, Tensor, etc. is determined
 * by the canonical type grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 38. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * These MUST be rejected:
 *
 *     extern "x" {
 *         fn;
 *     }
 *
 *     extern "x" {
 *         fn foo()
 *     }
 *
 *     extern "x" {
 *         fn foo() { }
 *     }
 *
 *     extern "x" {
 *         fn foo(;
 *     }
 *
 *     extern "x" {
 *         fn foo() -> ;
 *     }
 *
 *     extern "x" {
 *         fn foo(...);
 *     }
 *
 *     extern "x" {
 *         fn foo(x: int) -> int = 1;
 *     }
 *
 * The final case is especially important:
 *
 * foreign functions are declarations, not definitions.
 *
 * ============================================================================
 */


/* ============================================================================
 * 39. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     - empty interface;
 *     - one declaration;
 *     - large declaration lists;
 *     - large parameter lists;
 *     - generic declarations;
 *     - variadic declarations;
 *     - deeply nested type expressions;
 *     - long source identities;
 *     - large attribute sets.
 *
 * The tests MUST NOT assume:
 *
 *     32 qubits;
 *     64 parameters;
 *     1024 functions;
 *     128 devices;
 *     any other arbitrary machine-derived limit.
 *
 * ============================================================================
 */


/* ============================================================================
 * 40. CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Foreign interfaces MUST be testable with:
 *
 *     classical + foreign
 *     quantum + foreign
 *     hybrid + foreign
 *     HDL + foreign
 *     hardware + foreign
 *     distributed + foreign
 *     AI + foreign
 *     networking + foreign
 *     security + foreign
 *
 * The grammar remains domain-neutral.
 *
 * ============================================================================
 */


/* ============================================================================
 * 41. ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where the repository provides a canonical source printer:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     printer
 *       |
 *       v
 *     parser
 *
 * MUST preserve the intended foreign-interface semantics.
 *
 * ============================================================================
 */


/* ============================================================================
 * 42. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_FOREIGN_FUNCTIONS
 *     MAX_PARAMETERS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_INTERFACES
 *     MAX_ABI_VARIANTS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_THREADS
 *     MAX_NODES
 *
 * Any implementation limit discovered in generated/parser/frontend code MUST
 * be classified separately as:
 *
 *     language requirement
 *     resource policy
 *     implementation limit
 *     target constraint
 *     accidental hard-coding
 *
 * Accidental hard-coding MUST be removed.
 *
 * ============================================================================
 */


/* ============================================================================
 * 43. DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     ZamaniTokens
 *          |
 *          v
 *     ForeignFunctions
 *          |
 *          v
 *     canonical AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +----------------------+
 *          |                      |
 *          v                      v
 *     classical IR           quantum::ir
 *          |                      |
 *          +----------+-----------+
 *                     |
 *                     v
 *              compiler pipeline
 *                     |
 *                     v
 *        runtime / hardware / deployment
 *
 * No reverse dependency is permitted.
 *
 * In particular:
 *
 *     foreign grammar -> runtime
 *
 * MUST NOT exist.
 *
 *     foreign grammar -> hardware
 *
 * MUST NOT exist.
 *
 *     foreign grammar -> quantum::ir
 *
 * MUST NOT exist.
 *
 * ============================================================================
 */


/* ============================================================================
 * 44. DEFINITION OF DONE
 * ============================================================================
 *
 * This grammar is complete only when:
 *
 * [ ] canonical token vocabulary is consumed;
 * [ ] no lexer tokens are duplicated;
 * [ ] foreign declarations parse deterministically;
 * [ ] external source identity is represented opaquely;
 * [ ] foreign functions cannot contain bodies;
 * [ ] parameters use canonical type expressions;
 * [ ] return types use canonical type expressions;
 * [ ] generic parameters use canonical type boundaries;
 * [ ] effects use the canonical effects grammar;
 * [ ] requirements use canonical requirement grammar;
 * [ ] capabilities use canonical capability grammar;
 * [ ] attributes use canonical attribute grammar;
 * [ ] no ABI implementation is encoded;
 * [ ] no hardware identity is encoded;
 * [ ] no machine size is encoded;
 * [ ] no fixed resource ceiling is encoded;
 * [ ] no unsafe Rust is required;
 * [ ] AST lowering uses the repository's canonical ExternalDeclaration;
 * [ ] child declarations use canonical AST node identity;
 * [ ] semantic resolution is downstream;
 * [ ] IR lowering is downstream;
 * [ ] runtime resolution is downstream;
 * [ ] positive tests exist;
 * [ ] negative tests exist;
 * [ ] boundary tests exist;
 * [ ] cross-domain tests exist;
 * [ ] scalability tests exist;
 * [ ] compatibility tests exist;
 * [ ] round-trip tests exist where supported;
 * [ ] documentation agrees with grammar authority;
 * [ ] legacy valid extern syntax remains compatible;
 * [ ] no duplicate foreign-call grammar exists elsewhere;
 * [ ] hard-coding audit passes.
 *
 * ============================================================================
 */