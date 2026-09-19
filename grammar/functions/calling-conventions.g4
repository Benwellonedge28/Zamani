/**
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/calling-conventions.g4
 *
 * Grammar:
 *     CallingConventions
 *
 * Status:
 *     Production function-grammar integration contract
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR 4
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     2021
 *
 * Safety:
 *     Safe Rust only.
 *     No unsafe Rust is required or permitted by the Zamani compiler.
 *
 * Architectural objective:
 *
 *     Program Once, Compile Once, Run Everywhere, Anywhere, Forever
 *     (POCO-REAF)
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file owns ONLY the function-level SOURCE SYNTAX for attaching a
 * calling-convention requirement/reference to a Zamani function declaration.
 *
 * It deliberately does NOT implement or define an ABI.
 *
 * The distinction is fundamental:
 *
 *     calling-convention syntax
 *             !=
 *     calling-convention semantics
 *             !=
 *     ABI implementation
 *             !=
 *     target lowering
 *
 * This grammar therefore provides a narrow integration boundary between:
 *
 *     grammar/functions/
 *
 * and:
 *
 *     grammar/interoperability/abi.g4
 *
 * The ABI grammar remains the canonical owner of ABI/calling-convention
 * contract semantics.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     functions.g4             calling-conventions.g4
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                 domain-neutral AST
 *                        |
 *                        v
 *                semantic analysis
 *                        |
 *                        v
 *                  ABI contract
 *                        |
 *                        v
 *              ABI/calling-convention
 *                    resolution
 *                        |
 *                        v
 *                  target lowering
 *                        |
 *                        v
 *                compiler/backend
 *                        |
 *                        v
 *                    runtime
 *
 * The function grammar MUST NOT bypass the semantic/ABI layers.
 *
 * ============================================================================
 *
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *   - the reusable function-level calling-convention attachment syntax;
 *   - the syntactic representation of an open-ended convention reference;
 *   - optional calling-convention arguments/metadata;
 *   - composition of calling-convention syntax into function declarations;
 *   - source-level validation boundaries that can be expressed syntactically;
 *   - integration contracts with functions.g4 and foreign-functions.g4.
 *
 * ============================================================================
 *
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *   - ABI semantics;
 *   - ABI compatibility;
 *   - register allocation;
 *   - register names;
 *   - stack layout;
 *   - stack cleanup;
 *   - parameter passing implementation;
 *   - return-value implementation;
 *   - machine word size;
 *   - pointer width;
 *   - CPU architecture;
 *   - GPU architecture;
 *   - FPGA architecture;
 *   - QPU architecture;
 *   - operating-system ABI;
 *   - linker implementation;
 *   - object-file format;
 *   - symbol resolution;
 *   - dynamic loading;
 *   - hardware discovery;
 *   - target selection;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime execution;
 *   - physical resources;
 *   - resource allocation;
 *   - quantum IR;
 *   - classical IR;
 *   - HDL IR.
 *
 * Those concerns belong to their canonical downstream owners.
 *
 * ============================================================================
 *
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one semantic authority for calling-convention/ABI
 * contracts.
 *
 * That authority is:
 *
 *     grammar/interoperability/abi.g4
 *
 * This file MUST NOT recreate:
 *
 *     abiDeclaration
 *     abiConventionDeclaration
 *     abiProfileDeclaration
 *     abiLinkageDeclaration
 *     ABI type layout
 *     ABI pass-mode semantics
 *     ABI compatibility semantics
 *
 * Instead, this file references the convention symbol and provides the
 * function-level attachment point.
 *
 * ============================================================================
 *
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * `functions.g4` is the canonical composition grammar for ordinary functions.
 *
 * Calling conventions are nevertheless interoperability/ABI concerns.
 *
 * Keeping the reusable attachment syntax here provides:
 *
 *     functions.g4
 *             |
 *             +--> calling-convention attachment
 *                              |
 *                              v
 *                    interoperability/abi.g4
 *
 * without making `functions.g4` an ABI grammar.
 *
 * This is intentionally a small boundary file rather than a second
 * implementation of ABI syntax.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * A calling-convention reference expresses a source-level interoperability
 * requirement.
 *
 * It MUST NOT select a physical machine.
 *
 * Therefore the following distinctions are mandatory:
 *
 *     calling convention
 *         != CPU
 *         != core
 *         != register
 *         != device
 *         != node
 *         != QPU
 *         != GPU
 *         != FPGA
 *         != physical address
 *
 * For example, a symbolic convention such as:
 *
 *     "c"
 *
 * identifies an interoperability convention.
 *
 * It does NOT mean:
 *
 *     use CPU 0
 *
 * or:
 *
 *     use x86_64
 *
 * or:
 *
 *     use register RAX
 *
 * or:
 *
 *     use a particular operating system.
 *
 * Those are downstream target-resolution concerns.
 *
 * ============================================================================
 *
 * OPEN-WORLD REQUIREMENT
 * ============================================================================
 *
 * The grammar MUST NOT enumerate a finite list such as:
 *
 *     c
 *     cdecl
 *     stdcall
 *     fastcall
 *     thiscall
 *     sysv64
 *     win64
 *     aapcs
 *     vectorcall
 *     gpu
 *     qpu
 *
 * as permanent universal grammar alternatives.
 *
 * A new convention must be representable without modifying this grammar.
 *
 * This is essential for:
 *
 *   - future architectures;
 *   - new ABIs;
 *   - vendor-neutral interoperability;
 *   - embedded systems;
 *   - accelerators;
 *   - distributed boundaries;
 *   - quantum/classical boundaries;
 *   - future computational substrates.
 *
 * ============================================================================
 *
 * CANONICAL REPRESENTATION
 * ============================================================================
 *
 * The source-level convention reference is deliberately represented as:
 *
 *     STRING_LITERAL
 *
 * or:
 *
 *     qualifiedName
 *
 * Examples:
 *
 *     calling_convention = "c"
 *
 *     calling_convention = "stdcall"
 *
 *     calling_convention = platform::native
 *
 *     calling_convention = vendor::extension::convention
 *
 * The grammar does not decide whether the referenced convention exists.
 *
 * Semantic analysis performs that validation.
 *
 * ============================================================================
 *
 * FUNCTION-LEVEL SYNTAX
 * ============================================================================
 *
 * The canonical source form is:
 *
 *     fn compute(value: T) -> R
 *         calling_convention = "..."
 *         {
 *             ...
 *         }
 *
 * or, for a declaration:
 *
 *     fn compute(value: T) -> R
 *         calling_convention = "..."
 *         ;
 *
 * The exact placement relative to other function metadata is determined by
 * the function composition grammar.
 *
 * This file only defines the reusable clause.
 *
 * ============================================================================
 *
 * FOREIGN-FUNCTION INTEGRATION
 * ============================================================================
 *
 * A foreign declaration may use the same semantic convention reference:
 *
 *     extern "foreign" {
 *         fn compute(value: T) -> R
 *             calling_convention = "..."
 *             ;
 *     }
 *
 * `foreign-functions.g4` remains responsible for foreign-function
 * declaration structure.
 *
 * This file MUST NOT duplicate foreign-function declarations.
 *
 * ============================================================================
 *
 * NO HARDWARE LIMITS
 * ============================================================================
 *
 * This grammar contains no language-level limits for:
 *
 *   - number of functions;
 *   - number of parameters;
 *   - number of calling-convention declarations;
 *   - number of ABI profiles;
 *   - number of targets;
 *   - number of devices;
 *   - number of processors;
 *   - number of cores;
 *   - number of threads;
 *   - number of nodes;
 *   - number of qubits;
 *   - amount of memory;
 *   - number of accelerators.
 *
 * Repetition is represented structurally rather than by enumerated rules.
 *
 * Practical parser/compiler limits are implementation resource-policy
 * concerns and MUST NOT become source-language semantics.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this construct MUST depend only on:
 *
 *   - source text;
 *   - canonical token stream;
 *   - grammar version.
 *
 * Parsing MUST NOT depend on:
 *
 *   - hardware availability;
 *   - operating system;
 *   - filesystem state;
 *   - network state;
 *   - environment variables;
 *   - installed libraries;
 *   - linker state;
 *   - runtime state;
 *   - device discovery;
 *   - scheduler state;
 *   - calibration state.
 *
 * Whether a convention can actually be satisfied is a later semantic/target
 * question.
 *
 * ============================================================================
 *
 * SAFETY
 * ============================================================================
 *
 * This file contains grammar declarations only.
 *
 * It performs no:
 *
 *   - filesystem access;
 *   - network access;
 *   - process execution;
 *   - dynamic library loading;
 *   - symbol lookup;
 *   - hardware discovery;
 *   - ABI negotiation;
 *   - runtime invocation.
 *
 * Rust code generated/used by the compiler must target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *
 * and MUST contain no unsafe code.
 *
 * ============================================================================
 *
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This parser grammar consumes the canonical Zamani lexer vocabulary.
 *
 * It MUST NOT declare lexer rules.
 *
 * It therefore does not introduce:
 *
 *     CALLING
 *     CONVENTION
 *     ABI
 *     C
 *     STDCALL
 *     SYSV
 *     WIN64
 *     AAPCS
 *
 * as hard-coded lexical tokens.
 *
 * Convention identifiers remain ordinary identifiers or string literals.
 *
 * ============================================================================
 *
 * SHARED GRAMMAR CONTRACT
 * ============================================================================
 *
 * The composition layer supplies:
 *
 *     qualifiedName
 *
 * This file does not redefine:
 *
 *     identifier
 *     qualifiedName
 *     typeExpression
 *     expression
 *     attribute
 *     effectClause
 *     contractClause
 *     resource requirement
 *     capability requirement
 *
 * ============================================================================
 *
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar CallingConventions;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * 1. CALLING-CONVENTION CLAUSE
 * ============================================================================
 *
 * This is the only canonical function-level construct owned by this file.
 *
 * The key is intentionally represented as an IDENTIFIER rather than a
 * reserved keyword.
 *
 * Semantic validation MUST require the canonical spelling:
 *
 *     calling_convention
 *
 * This keeps the lexical vocabulary open and avoids adding a universal
 * reserved word merely for interoperability metadata.
 *
 * The semantic layer may also support versioned aliases through the normal
 * compatibility mechanism.
 *
 * ============================================================================
 */

functionCallingConventionClause
    : callingConventionKey
      EQUALS
      callingConventionReference
    ;


/*
 * ============================================================================
 * 2. CANONICAL KEY
 * ============================================================================
 *
 * The parser intentionally accepts IDENTIFIER here because the current
 * canonical lexer does not reserve `calling_convention` as a keyword.
 *
 * Semantic validation MUST reject arbitrary identifiers at this boundary.
 *
 * This separation prevents this grammar from creating a second lexical
 * authority.
 *
 * ============================================================================
 */

callingConventionKey
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 3. CONVENTION REFERENCE
 * ============================================================================
 *
 * A convention is a symbolic semantic reference.
 *
 * Both forms are supported:
 *
 *     "c"
 *
 * and:
 *
 *     platform::native
 *
 * The exact interpretation is owned by ABI semantic resolution.
 *
 * ============================================================================
 */

callingConventionReference
    : STRING_LITERAL
    | qualifiedName
    ;


/*
 * ============================================================================
 * 4. OPTIONAL FUNCTION CALLING-CONVENTION LIST
 * ============================================================================
 *
 * A function has one effective calling-convention requirement.
 *
 * Therefore this grammar intentionally provides a singular clause rather than
 * an unbounded list of potentially conflicting conventions.
 *
 * Multiple ABI requirements belong in the canonical ABI contract model,
 * where compatibility and composition can be checked semantically.
 *
 * ============================================================================
 */

functionCallingConventionAttachment
    : functionCallingConventionClause
    ;


/*
 * ============================================================================
 * 5. DECLARATION-LEVEL ADAPTER
 * ============================================================================
 *
 * This rule exists for grammar composition.
 *
 * `functions.g4` may consume:
 *
 *     functionCallingConventionAttachment?
 *
 * at the function metadata boundary.
 *
 * The function grammar remains responsible for the complete declaration.
 *
 * ============================================================================
 */

functionCallingConventionMetadata
    : functionCallingConventionAttachment
    ;


/*
 * ============================================================================
 * 6. FOREIGN-FUNCTION ADAPTER
 * ============================================================================
 *
 * Foreign functions may consume the same clause.
 *
 * `foreign-functions.g4` remains responsible for:
 *
 *     extern
 *     foreign source
 *     foreign members
 *     foreign function signatures
 *
 * This file supplies only the calling-convention clause.
 *
 * ============================================================================
 */

foreignCallingConventionMetadata
    : functionCallingConventionClause
    ;


/*
 * ============================================================================
 * 7. ABI BRIDGE
 * ============================================================================
 *
 * The parsed reference must map downstream to the canonical ABI semantic
 * model.
 *
 * Conceptually:
 *
 *     functionCallingConventionClause
 *                 |
 *                 v
 *         domain-neutral AST
 *                 |
 *                 v
 *       semantic ABI reference
 *                 |
 *                 v
 *       interoperability/ABI
 *                 |
 *                 v
 *       target resolution
 *
 * This grammar does not invoke the ABI grammar directly at runtime.
 *
 * Grammar composition and semantic lowering are separate concerns.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 8. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing success means only:
 *
 *     the source has the syntactic shape
 *
 *         identifier = symbolic-reference
 *
 * at the calling-convention attachment point.
 *
 * Semantic analysis must additionally establish:
 *
 *   - the key is `calling_convention`;
 *   - the reference is well-formed;
 *   - the referenced convention exists when required;
 *   - the convention is compatible with the enclosing ABI;
 *   - the convention is compatible with the function signature;
 *   - the convention is legal for the selected interoperability boundary;
 *   - incompatible duplicate declarations are diagnosed;
 *   - target support is checked downstream;
 *   - no unsupported target is silently selected.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 9. FUNCTION SIGNATURE INTEGRATION
 * ============================================================================
 *
 * This construct does not own:
 *
 *     parameter types;
 *     return types;
 *     generic parameters;
 *     effects;
 *     contracts;
 *     expressions;
 *     statements;
 *     blocks.
 *
 * Those remain owned by:
 *
 *     grammar/functions/parameters.g4
 *     grammar/functions/returns.g4
 *     grammar/functions/generics.g4
 *     grammar/functions/contracts.g4
 *     grammar/effects/
 *     grammar/expressions/
 *     grammar/statements/
 *
 * The calling-convention clause merely attaches metadata to the complete
 * function signature.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 10. ABI INTEGRATION
 * ============================================================================
 *
 * Canonical ABI ownership:
 *
 *     grammar/interoperability/abi.g4
 *
 * That grammar already provides:
 *
 *     ABI identity
 *     ABI profiles
 *     calling-convention declarations
 *     linkage
 *     symbol metadata
 *     representation metadata
 *     compatibility
 *     requirements
 *
 * This file MUST NOT reproduce those productions.
 *
 * Instead:
 *
 *     functionCallingConventionClause
 *         |
 *         v
 *     symbolic ABI reference
 *         |
 *         v
 *     ABI semantic resolver
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 11. FOREIGN FUNCTIONS
 * ============================================================================
 *
 * `foreign-functions.g4` already provides foreign declaration structure and
 * symbolic ABI metadata.
 *
 * The integration rule is:
 *
 *     foreign declaration
 *          |
 *          +--> language
 *          +--> ABI
 *          +--> linkage
 *          +--> calling convention
 *          +--> attributes
 *          +--> requirements
 *          +--> capability metadata
 *
 * No foreign-language-specific calling-convention list belongs here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 12. RUST ABI INTEROPERABILITY
 * ============================================================================
 *
 * Rust 1.97/1.97.1 is the implementation baseline for ZUTC.
 *
 * That does NOT make Rust's ABI vocabulary the Zamani language vocabulary.
 *
 * If a target/backend needs a Rust ABI reference, it may be represented as:
 *
 *     calling_convention = "Rust"
 *
 * or another canonical ABI-qualified semantic reference.
 *
 * The grammar must not enumerate Rust's target-specific ABI variants.
 *
 * The semantic/compiler layer determines whether a requested convention can
 * be lowered for the selected target.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 13. C / C++ / FOREIGN ABI INTEROPERABILITY
 * ============================================================================
 *
 * The grammar deliberately remains open-world.
 *
 * Examples of semantically possible references include:
 *
 *     calling_convention = "C"
 *     calling_convention = "cdecl"
 *     calling_convention = "stdcall"
 *     calling_convention = "sysv64"
 *     calling_convention = "win64"
 *     calling_convention = "aapcs"
 *
 * These are examples of DATA, not grammar alternatives.
 *
 * A future convention:
 *
 *     calling_convention = "future_architecture_convention"
 *
 * must remain syntactically representable without editing this file.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 14. EMBEDDED / HDL / ACCELERATOR INTEROPERABILITY
 * ============================================================================
 *
 * Calling conventions may eventually describe boundaries involving:
 *
 *     embedded firmware;
 *     HDL-generated interfaces;
 *     accelerators;
 *     FPGA fabric;
 *     ASIC interfaces;
 *     DSPs;
 *     GPUs;
 *     QPUs;
 *     distributed services;
 *     future computational substrates.
 *
 * This grammar does not create separate constructs for those domains.
 *
 * The convention remains symbolic.
 *
 * Domain-specific semantics belong downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 15. QUANTUM INTEGRATION
 * ============================================================================
 *
 * A function that crosses a classical/quantum boundary may carry a calling
 * convention or interoperability requirement.
 *
 * Example:
 *
 *     fn quantum_entry(q: Qubit) -> Measurement
 *         calling_convention = "..."
 *     {
 *         ...
 *     }
 *
 * The function grammar does not define:
 *
 *     Qubit
 *     Measurement
 *     physical qubits
 *     gate sets
 *     topology
 *     calibration
 *     scheduling
 *     QEC
 *     ZQN
 *
 * Quantum semantics continue through:
 *
 *     domain-neutral AST
 *         ->
 *     semantic analysis
 *         ->
 *     quantum::ir
 *         ->
 *     optimization
 *         ->
 *     routing
 *         ->
 *     scheduling
 *         ->
 *     QEC/resilience
 *         ->
 *     ZQN
 *         ->
 *     HAL
 *         ->
 *     target realization
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 16. HARDWARE / RESOURCE INTEGRATION
 * ============================================================================
 *
 * Calling convention syntax MUST NOT imply resource allocation.
 *
 * It must never mean:
 *
 *     use core 0
 *     use GPU 0
 *     use QPU 0
 *     use register 0
 *     use node 0
 *
 * Resource/capability requirements remain separate semantic concepts.
 *
 * This preserves the distinction:
 *
 *     interoperability requirement
 *             !=
 *     resource requirement
 *             !=
 *     target selection
 *             !=
 *     physical placement
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 17. FUNCTION-TYPE INTEGRATION
 * ============================================================================
 *
 * This file does NOT modify function-type syntax.
 *
 * A function type may eventually carry ABI metadata if the canonical type
 * specification explicitly permits it.
 *
 * That decision belongs to:
 *
 *     grammar/types/function.g4
 *
 * and the type-system specification.
 *
 * Do not silently make every function type ABI-specific merely because a
 * function declaration can carry a calling convention.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 18. GENERICS
 * ============================================================================
 *
 * Calling-convention references do not impose generic arity limits.
 *
 * Generic parameters remain owned by:
 *
 *     grammar/functions/generics.g4
 *
 * Generic substitution, specialization and monomorphization remain semantic
 * and compiler concerns.
 *
 * A generic function may therefore carry one symbolic convention requirement
 * without changing the generic grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. EFFECTS
 * ============================================================================
 *
 * Calling convention is not an effect.
 *
 * Do not transform:
 *
 *     calling_convention = "..."
 *
 * into:
 *
 *     effects(...)
 *
 * unless the semantic specification explicitly defines such a relationship.
 *
 * ABI compatibility and effect capability are separate semantic dimensions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. CONTRACTS
 * ============================================================================
 *
 * Calling-convention metadata does not replace:
 *
 *     requires(...)
 *     ensures(...)
 *     invariant(...)
 *
 * Contracts remain owned by:
 *
 *     grammar/functions/contracts.g4
 *
 * A contract may constrain an interoperability boundary semantically, but
 * this grammar must not duplicate contract syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. SOURCE SPANS
 * ============================================================================
 *
 * Every occurrence of:
 *
 *     functionCallingConventionClause
 *
 * must retain source span information through the frontend AST.
 *
 * At minimum, downstream diagnostics need to identify:
 *
 *     key span
 *     reference span
 *     complete clause span
 *
 * The grammar does not calculate source locations.
 *
 * The canonical frontend source-span infrastructure owns that responsibility.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must represent the construct as generic metadata rather
 * than introducing an ABI-specific execution object.
 *
 * Conceptually:
 *
 *     Attribute / Metadata
 *         key:
 *             calling_convention
 *         value:
 *             symbolic reference
 *
 * or an equivalent domain-neutral function metadata node.
 *
 * The exact Rust AST type is owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT require:
 *
 *     QuantumCallingConvention
 *     CpuCallingConvention
 *     GpuCallingConvention
 *     HdlCallingConvention
 *
 * AST variants.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. SEMANTIC MODEL CONTRACT
 * ============================================================================
 *
 * After AST construction:
 *
 *     calling_convention
 *             |
 *             v
 *     semantic ABI reference
 *
 * The semantic model is responsible for:
 *
 *   - canonicalization;
 *   - validation;
 *   - version resolution;
 *   - ABI compatibility;
 *   - duplicate/conflict detection;
 *   - target capability checking;
 *   - diagnostic generation.
 *
 * Unknown conventions may remain unresolved until a target/interoperability
 * context requires resolution, provided the language specification permits
 * open-world declarations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. IR CONTRACT
 * ============================================================================
 *
 * This grammar does not create a CallingConventionIR.
 *
 * ABI/calling-convention metadata must be attached to the canonical semantic
 * representation consumed by the appropriate compiler/IR layer.
 *
 * In particular:
 *
 *     NO FunctionCallingConventionIR
 *     NO QuantumCallingConventionIR
 *     NO HardwareCallingConventionIR
 *
 * should be introduced merely because this grammar exists.
 *
 * If a canonical IR requires a calling-convention field, the field must be
 * owned by that canonical IR's existing function/call/external-symbol model.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. QUANTUM IR CONTRACT
 * ============================================================================
 *
 * This file MUST NOT create a quantum calling-convention representation.
 *
 * Quantum semantic lowering remains:
 *
 *     source
 *       ->
 *     frontend AST
 *       ->
 *     semantic analysis
 *       ->
 *     quantum::ir
 *
 * The existing quantum IR already has a function calling-convention concept.
 *
 * That existing representation is consumed downstream.
 *
 * This grammar only supplies source syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler lowering may transform:
 *
 *     symbolic convention
 *          ->
 *     validated ABI identity
 *          ->
 *     target-specific ABI lowering
 *
 * Target lowering may determine:
 *
 *     parameter passing
 *     return passing
 *     register/stack usage
 *     linkage
 *     symbol naming
 *     object format
 *     unwind behavior
 *     target-specific interoperability
 *
 * None of those decisions belong to this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. RUNTIME CONTRACT
 * ============================================================================
 *
 * The runtime MUST NOT reinterpret raw parser strings as executable ABI
 * instructions.
 *
 * ABI metadata must already have passed semantic/compiler validation before
 * runtime consumption.
 *
 * Parsing is never authorization for:
 *
 *     dynamic loading
 *     symbol resolution
 *     foreign execution
 *     hardware access
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. SECURITY CONTRACT
 * ============================================================================
 *
 * Calling-convention metadata must not grant capabilities.
 *
 * For example:
 *
 *     calling_convention = "native"
 *
 * does NOT grant:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     hardware access
 *     foreign-library loading
 *
 * Capability/effect/security analysis remains authoritative.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The spelling:
 *
 *     calling_convention
 *
 * is source-level metadata.
 *
 * If the language later standardizes a shorter spelling or a reserved keyword,
 * that change must go through:
 *
 *     grammar/compatibility/
 *
 * and the language-version policy.
 *
 * Do not silently reinterpret an existing identifier.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. NEGATIVE CASES
 * ============================================================================
 *
 * The following must be rejected semantically or structurally as appropriate:
 *
 *     calling_convention
 *
 *     calling_convention =
 *
 *     calling_convention = ;
 *
 *     calling_convention = ()
 *
 *     calling_convention = {}
 *
 *     calling_convention = "a" "b"
 *
 * if the canonical expression grammar does not permit such a reference.
 *
 * Multiple conflicting convention clauses on the same function must also be
 * rejected by semantic validation rather than silently selecting one.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. BOUNDARY CASES
 * ============================================================================
 *
 * The following must remain representable:
 *
 *     calling_convention = "c"
 *
 *     calling_convention = "cdecl"
 *
 *     calling_convention = "stdcall"
 *
 *     calling_convention = "sysv64"
 *
 *     calling_convention = "win64"
 *
 *     calling_convention = "aapcs"
 *
 *     calling_convention = "future::convention"
 *
 *     calling_convention = "vendor.future.convention"
 *
 *     calling_convention = "custom-convention"
 *
 * The grammar must not need modification when a new convention is introduced.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. SCALABILITY TESTS
 * ============================================================================
 *
 * Tests must verify that the construct remains valid independent of:
 *
 *     number of functions;
 *     number of parameters;
 *     generic arity;
 *     number of modules;
 *     machine size;
 *     number of processors;
 *     number of accelerators;
 *     number of QPUs;
 *     number of nodes;
 *     amount of memory.
 *
 * The convention reference itself contains no finite numeric machine limit.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. DETERMINISTIC TESTS
 * ============================================================================
 *
 * Identical source and grammar version must produce identical:
 *
 *     token sequence;
 *     parse tree;
 *     AST metadata;
 *
 * independent of:
 *
 *     target hardware;
 *     environment;
 *     runtime;
 *     linker;
 *     filesystem;
 *     network.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * The same syntax must remain usable for:
 *
 *     classical functions;
 *     quantum/classical functions;
 *     HDL/software boundaries;
 *     accelerator interfaces;
 *     embedded functions;
 *     distributed interfaces;
 *     foreign functions.
 *
 * Domain semantics remain downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain universal grammar rules encoding:
 *
 *     MAX_CPU
 *     MAX_GPU
 *     MAX_FPGA
 *     MAX_QPU
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_NODES
 *     MAX_REGISTERS
 *     MAX_MEMORY
 *     MAX_PARAMETERS
 *     MAX_FUNCTIONS
 *
 * It also MUST NOT enumerate physical device IDs or register names.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] functions.g4 can consume functionCallingConventionAttachment.
 *
 * [ ] foreign-functions.g4 can consume the same semantic convention reference
 *     without duplicating its grammar.
 *
 * [ ] interoperability/abi.g4 remains the canonical ABI semantic contract
 *     grammar.
 *
 * [ ] no calling-convention keyword list is hard-coded here.
 *
 * [ ] no target architecture list is hard-coded here.
 *
 * [ ] no register names are hard-coded here.
 *
 * [ ] no machine capacities are hard-coded here.
 *
 * [ ] no hardware identifiers are hard-coded here.
 *
 * [ ] no second ABI grammar is introduced.
 *
 * [ ] no second calling-convention IR is introduced.
 *
 * [ ] AST mapping is domain-neutral.
 *
 * [ ] semantic resolution is downstream.
 *
 * [ ] source spans survive the frontend.
 *
 * [ ] unknown future conventions remain syntactically representable.
 *
 * [ ] negative tests exist.
 *
 * [ ] boundary tests exist.
 *
 * [ ] scalability tests exist.
 *
 * [ ] determinism tests exist.
 *
 * [ ] cross-domain tests exist.
 *
 * [ ] compatibility tests exist.
 *
 * [ ] Rust integration remains compatible with Rust 1.97/1.97.1.
 *
 * [ ] Rust implementation remains safe Rust with no unsafe code.
 *
 * ============================================================================
 */