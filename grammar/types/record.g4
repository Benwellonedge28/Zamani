/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/record.g4
 *
 * Grammar:
 *     RecordTypes
 *
 * Status:
 *     Production-ready type-system integration component.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the TYPE-SYSTEM INTEGRATION boundary for record types.
 *
 * It does NOT define record declarations.
 *
 * Record declarations are owned exclusively by:
 *
 *     grammar/declarations/records.g4
 *
 * A declared record becomes a named source-level type. Its use in a type
 * position is therefore represented by the canonical named-type machinery.
 *
 * This file exists to give the modular type grammar a stable record-type
 * category without creating a second record syntax or a second AST type
 * hierarchy.
 *
 * ============================================================================
 * CRITICAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Zamani has one canonical source-level type representation:
 *
 *     TypeExpr
 *
 * Currently the repository represents named types as:
 *
 *     TypeExpr::Identifier
 *
 * and generic applications as:
 *
 *     TypeExpr::Generic
 *
 * There is currently no canonical:
 *
 *     TypeExpr::Record
 *     RecordTypeExpr
 *     RecordTypeNode
 *
 * Therefore this grammar MUST NOT introduce any of them.
 *
 * A record type such as:
 *
 *     Person
 *
 * or:
 *
 *     domain::Person
 *
 * or:
 *
 *     Person<T>
 *
 * is structurally parsed by the canonical named/generic type grammar.
 *
 * Whether `Person` denotes a record, struct, enum, class, interface, alias,
 * external type, or another nominal type is a SEMANTIC question.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the record-type integration category;
 *   - the boundary between record declarations and type usage;
 *   - documentation of record-type AST preservation;
 *   - record-type integration contracts;
 *   - compatibility of the record-type category;
 *   - prevention of duplicate record type syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - record declarations;
 *   - record declaration bodies;
 *   - record fields;
 *   - field visibility;
 *   - field initializers;
 *   - record literals;
 *   - expression member access;
 *   - identifiers;
 *   - qualified-name syntax;
 *   - generic argument syntax;
 *   - type-expression composition;
 *   - type inference;
 *   - nominal type resolution;
 *   - structural type checking;
 *   - record layout;
 *   - ABI layout;
 *   - serialization layout;
 *   - memory placement;
 *   - CPU/GPU/FPGA/ASIC/QPU selection;
 *   - distributed placement;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - runtime representation.
 *
 * ============================================================================
 * DECLARATION OWNERSHIP
 * ============================================================================
 *
 * Record declarations are exclusively owned by:
 *
 *     grammar/declarations/records.g4
 *
 * The declaration form is conceptually:
 *
 *     record Name {
 *         field: Type,
 *         ...
 *     }
 *
 * Generic records are declared through that grammar's generic parameter
 * machinery.
 *
 * This file MUST NOT redefine:
 *
 *     recordDeclaration
 *     recordBody
 *     recordField
 *
 * Doing so would create competing grammar authorities.
 *
 * ============================================================================
 * TYPE-USAGE OWNERSHIP
 * ============================================================================
 *
 * Once declared:
 *
 *     record Person {
 *         name: String,
 *         age: Integer,
 *     }
 *
 * the source-level type:
 *
 *     Person
 *
 * is a named type.
 *
 * The canonical type grammar therefore owns its syntax through:
 *
 *     namedType
 *
 * and generic application through:
 *
 *     genericType
 *
 * Examples:
 *
 *     Person
 *     domain::Person
 *     Person<T>
 *     domain::Person<T>
 *
 * must continue to use the canonical type-expression machinery.
 *
 * ============================================================================
 * WHY THIS FILE DOES NOT DEFINE:
 *
 *     recordType
 *         : RECORD identifier ...
 *
 * ============================================================================
 *
 * Such a rule would introduce a new source syntax:
 *
 *     record Person
 *
 * in a type position.
 *
 * That syntax is not required by the current language architecture and would
 * incorrectly conflate:
 *
 *     record declaration
 *
 * with:
 *
 *     record type reference.
 *
 * It would also require changes to:
 *
 *     TypeExpr
 *     type resolution
 *     generic resolution
 *     diagnostics
 *     semantic analysis
 *     formatter
 *     LSP
 *     documentation
 *     compatibility
 *
 * merely to identify a type whose semantic identity can already be represented
 * by the existing named-type model.
 *
 * Production architecture therefore keeps the distinction:
 *
 *     record declaration
 *         ->
 *     nominal type declaration
 *         ->
 *     named type reference
 *
 * ============================================================================
 * PUBLIC INTEGRATION RULE
 * ============================================================================
 *
 * The public adapter is:
 *
 *     recordType
 *
 * and delegates to the canonical named-type rule.
 *
 * This rule is intentionally NOT intended to become an alternative in
 * `typeExpression` while `namedType` already exists.
 *
 * It is a semantic/parser integration hook for modular consumers that need
 * to refer to "record type syntax" without introducing record-specific
 * surface syntax.
 *
 * ============================================================================
 */

parser grammar RecordTypes;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * RECORD TYPE INTEGRATION ADAPTER
 * ============================================================================
 *
 * A record type has no additional source-level marker.
 *
 * Its syntax is therefore the canonical named-type syntax.
 *
 * IMPORTANT:
 *
 * This rule MUST remain a thin delegation boundary.
 *
 * It must never grow record-specific lexical or structural syntax.
 */
recordType
    : namedType
    ;


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The parser cannot determine whether the resulting named type denotes:
 *
 *     - a record;
 *     - a struct;
 *     - an enum;
 *     - a class;
 *     - an interface;
 *     - a trait;
 *     - a type alias;
 *     - an imported/external type;
 *     - a domain-defined type.
 *
 * Name resolution determines the declaration associated with the type name.
 *
 * Therefore:
 *
 *     recordType
 *
 * carries no additional semantic information at parse time.
 *
 * Conceptual pipeline:
 *
 *     record declaration
 *          |
 *          v
 *     declaration environment
 *          |
 *          v
 *     named type reference
 *          |
 *          v
 *     name resolution
 *          |
 *          v
 *     nominal type identity
 *          |
 *          v
 *     semantic record definition
 *          |
 *          v
 *     canonical semantic type
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar MUST map through the existing canonical AST.
 *
 * Conceptually:
 *
 *     recordType
 *         ->
 *     namedType
 *         ->
 *     TypeExpr::Identifier
 *
 * For a generic record:
 *
 *     recordType
 *         ->
 *     named/generic type machinery
 *         ->
 *     TypeExpr::Generic
 *
 * No new AST variant is introduced.
 *
 * Existing repository AST definitions include:
 *
 *     TypeExpr::Identifier
 *     TypeExpr::Generic
 *
 * and these remain authoritative.
 *
 * ============================================================================
 * GENERIC RECORD CONTRACT
 * ============================================================================
 *
 * A declaration such as:
 *
 *     record Pair<T, U> {
 *         first: T,
 *         second: U,
 *     }
 *
 * is referenced using the existing generic type syntax:
 *
 *     Pair<A, B>
 *
 * The generic argument syntax belongs to:
 *
 *     grammar/types/generic.g4
 *
 * or the repository's canonical generic-type composition boundary.
 *
 * This file MUST NOT duplicate:
 *
 *     genericParameters
 *     typeArguments
 *     genericArgumentList
 *
 * ============================================================================
 * QUALIFIED RECORD TYPE CONTRACT
 * ============================================================================
 *
 * Records may be referenced through qualified names:
 *
 *     module::Person
 *     package::domain::Person
 *
 * Qualified-name syntax belongs to the canonical name/path grammar.
 *
 * This file MUST NOT define another qualified-name grammar.
 *
 * ============================================================================
 * RECORD FIELD CONTRACT
 * ============================================================================
 *
 * Record fields are declaration members.
 *
 * They are owned by:
 *
 *     grammar/declarations/records.g4
 *
 * The canonical field representation currently integrates with:
 *
 *     StructField
 *
 * in the frontend AST where the repository's shared aggregate-field model
 * requires it.
 *
 * This file MUST NOT define:
 *
 *     recordField
 *     recordFieldList
 *     structField
 *
 * because doing so would duplicate declaration-level field ownership.
 *
 * ============================================================================
 * RECORD LITERAL CONTRACT
 * ============================================================================
 *
 * A record type and a record value are different language concepts.
 *
 * Type:
 *
 *     Person
 *
 * Value:
 *
 *     Person {
 *         name: "Alice",
 *         age: 42,
 *     }
 *
 * or whatever canonical record-literal syntax is adopted by the expression
 * subsystem.
 *
 * Record values MUST NOT be parsed by this file.
 *
 * Record expressions belong to:
 *
 *     grammar/expressions/
 *     grammar/data/
 *
 * according to the repository's expression/data ownership model.
 *
 * The existing AST already contains:
 *
 *     Expression::Struct
 *
 * for aggregate construction.
 *
 * This file must not introduce:
 *
 *     Expression::Record
 *
 * merely because a record declaration exists.
 *
 * ============================================================================
 * TYPE IDENTITY
 * ============================================================================
 *
 * A declared record has nominal identity.
 *
 * For example:
 *
 *     record UserId {
 *         value: Integer,
 *     }
 *
 * and:
 *
 *     record ProductId {
 *         value: Integer,
 *     }
 *
 * are not automatically the same semantic type merely because their fields
 * are structurally identical.
 *
 * That distinction belongs to semantic type resolution.
 *
 * The parser preserves only the referenced type name.
 *
 * ============================================================================
 * STRUCTURAL VS NOMINAL SEMANTICS
 * ============================================================================
 *
 * This file deliberately does not decide whether a record participates in:
 *
 *     nominal typing;
 *     structural typing;
 *     coercion;
 *     subtyping;
 *     conversion;
 *     pattern compatibility;
 *     serialization compatibility.
 *
 * These are governed by:
 *
 *     grammar/spec/type-system.md
 *
 * and the semantic type subsystem.
 *
 * ============================================================================
 * GENERIC / PARAMETRIC INTEGRATION
 * ============================================================================
 *
 * Record declarations may contain generic parameters.
 *
 * Record type references therefore participate in the same parametric type
 * system as every other named type.
 *
 * Example:
 *
 *     record Buffer<T> {
 *         data: T,
 *     }
 *
 * references:
 *
 *     Buffer<Integer>
 *
 * and:
 *
 *     Buffer<Qubit>
 *
 * use the same canonical generic type mechanism.
 *
 * The record grammar does not know what `T` means after parsing.
 *
 * ============================================================================
 * DEPENDENT / VALUE-PARAMETERIZED INTEGRATION
 * ============================================================================
 *
 * If the language's canonical generic/dependent type system permits a record
 * to be parameterized by semantic values, those parameters are handled by
 * the canonical generic/dependent type machinery.
 *
 * Example conceptual type:
 *
 *     Register<N>
 *
 * where `N` is a semantic value parameter.
 *
 * This file does not evaluate `N`.
 *
 * It does not convert `N` to:
 *
 *     usize
 *
 * or any other host-specific representation.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * A record type expresses logical structure.
 *
 * It does not allocate physical resources.
 *
 * Therefore:
 *
 *     record DeviceState {
 *         ...
 *     }
 *
 * does NOT imply:
 *
 *     CPU allocation
 *     GPU allocation
 *     FPGA allocation
 *     ASIC allocation
 *     QPU allocation
 *     memory placement
 *     register allocation
 *     node placement
 *     network placement
 *
 * Resource requirements are expressed through the universal resource and
 * capability system and resolved downstream.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A record definition is source-level semantic intent.
 *
 * Its physical realization may differ across:
 *
 *     tiny embedded systems;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     QPUs;
 *     simulators;
 *     HPC systems;
 *     clusters;
 *     distributed systems;
 *     cloud systems;
 *     future architectures.
 *
 * The source-level record type must retain the same semantic meaning.
 *
 * Downstream stages may change:
 *
 *     representation;
 *     layout;
 *     storage;
 *     placement;
 *     serialization;
 *     transport;
 *     scheduling;
 *     execution strategy;
 *     ABI representation.
 *
 * None of those decisions belong in this grammar.
 *
 * ============================================================================
 * NO ARTIFICIAL LIMITS
 * ============================================================================
 *
 * This file MUST NOT impose limits on:
 *
 *     record count;
 *     record field count;
 *     field-name length;
 *     record nesting;
 *     generic arity;
 *     generic nesting;
 *     type nesting;
 *     memory;
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     ASIC count;
 *     accelerator count;
 *     QPU count;
 *     qubit count;
 *     node count;
 *     device count;
 *     tensor rank;
 *     network size;
 *     register width.
 *
 * In particular, this file must never define or reference language-semantic
 * constants such as:
 *
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
 * A compiler may have implementation resource policies.
 *
 * Those policies are not language semantics and must not be encoded here.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A record may contain quantum fields through its declaration:
 *
 *     record QuantumState {
 *         qubit: Qubit,
 *         classical: Integer,
 *     }
 *
 * The record type itself remains a normal named type.
 *
 * Quantum semantics are resolved after AST construction.
 *
 * The canonical quantum pipeline remains:
 *
 *     source
 *       ->
 *     AST
 *       ->
 *     semantic type analysis
 *       ->
 *     quantum semantics
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
 *     resilience/QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target realization
 *
 * This grammar MUST NOT import or depend on `quantum::ir`.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A record may describe logical hardware-facing data:
 *
 *     record SensorSample {
 *         value: Integer,
 *         status: Status,
 *     }
 *
 * or aggregate hardware-intent types.
 *
 * It does not define:
 *
 *     wires;
 *     physical registers;
 *     fixed bus widths;
 *     pin locations;
 *     device IDs;
 *     clock implementation;
 *     placement;
 *     routing.
 *
 * HDL syntax remains owned by:
 *
 *     grammar/hdl/
 *
 * Hardware realization remains owned by:
 *
 *     grammar/hardware/
 *
 * ============================================================================
 * DATA / AI / KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Records are general-purpose aggregates and can therefore represent data
 * used by:
 *
 *     data processing;
 *     knowledge systems;
 *     reasoning;
 *     learning;
 *     uncertainty;
 *     evidence;
 *     provenance;
 *     agents;
 *     classical computation;
 *     quantum-classical computation.
 *
 * No domain-specific record syntax is necessary.
 *
 * For example:
 *
 *     record Evidence<T> {
 *         claim: T,
 *         confidence: Float,
 *     }
 *
 * remains ordinary source-level type syntax.
 *
 * The semantics of confidence, provenance, reasoning, or learning are owned
 * by their respective semantic systems.
 *
 * ============================================================================
 * PATTERN-MATCHING INTEGRATION
 * ============================================================================
 *
 * Records may participate in structural pattern matching.
 *
 * Pattern syntax belongs to:
 *
 *     grammar/expressions/
 *     grammar/statements/
 *
 * The semantic checker resolves the pattern against the record declaration.
 *
 * This file does not define record patterns.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Record fields may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * expressions.
 *
 * Contract syntax is owned by the validation/contract subsystem.
 *
 * RecordTypes does not define or duplicate contract grammar.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Merely referring to a record type has no intrinsic effect.
 *
 * Effects may arise from operations involving record values, for example:
 *
 *     IO
 *     network
 *     mutation
 *     foreign
 *     native
 *     distributed
 *     randomness
 *     reflection
 *
 * Effect analysis remains downstream.
 *
 * ============================================================================
 * CAPABILITY / RESOURCE INTEGRATION
 * ============================================================================
 *
 * Record type definitions may be used inside resource/capability structures:
 *
 *     Resource<State>
 *     Capability<Policy>
 *     DeviceDescriptor
 *
 * This does not cause record parsing to inspect hardware.
 *
 * Capability negotiation remains downstream.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * A record declaration and its type references preserve source provenance
 * through the normal AST/source-span machinery.
 *
 * This grammar does not create provenance records itself.
 *
 * Provenance belongs to the compiler/semantic/toolchain provenance subsystem.
 *
 * ============================================================================
 * INTEROPERABILITY / ABI INTEGRATION
 * ============================================================================
 *
 * A record may eventually cross:
 *
 *     FFI
 *     ABI
 *     serialization
 *     networking
 *     distributed
 *     language-interoperability
 *
 * boundaries.
 *
 * This grammar does not determine layout.
 *
 * The semantic/backend layer must derive representation from the selected
 * target/interoperability contract.
 *
 * Therefore a record type must never implicitly mean:
 *
 *     #[repr(C)]
 *     packed
 *     aligned
 *     native-width
 *     pointer-width
 *
 * unless such semantics are explicitly represented by a separate
 * source-level declaration/modifier owned by the appropriate interoperability
 * grammar.
 *
 * ============================================================================
 * MEMORY / OWNERSHIP INTEGRATION
 * ============================================================================
 *
 * A record may contain:
 *
 *     references;
 *     pointers;
 *     linear values;
 *     affine values;
 *     quantum resources;
 *     resource handles.
 *
 * Ownership and lifetime checking remains semantic.
 *
 * This grammar merely preserves the nested type structure.
 *
 * ============================================================================
 * TYPE EQUALITY
 * ============================================================================
 *
 * Record type equality is not established by this parser.
 *
 * The semantic type system must distinguish:
 *
 *     source spelling;
 *     aliases;
 *     nominal identity;
 *     generic instantiation;
 *     substitutions;
 *     normalized forms.
 *
 * Source formatting must never affect semantic equality.
 *
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * This adapter introduces no special parser error handling.
 *
 * Errors in:
 *
 *     identifier syntax;
 *     qualified names;
 *     generic arguments;
 *     enclosing type syntax
 *
 * are diagnosed by the canonical grammar/parser.
 *
 * Errors such as:
 *
 *     unknown record type;
 *     wrong generic arity;
 *     invalid field access;
 *     incompatible record assignment;
 *     invalid field type;
 *
 * are semantic errors, not grammar errors.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware queries;
 *     - no mutable global state;
 *     - no runtime execution;
 *     - no target-specific behavior.
 *
 * Identical source/token input under the same grammar configuration produces
 * the same parse structure.
 *
 * ============================================================================
 * SAFETY / RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * The consuming implementation must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and the repository requirement:
 *
 *     no unsafe Rust.
 *
 * Rust source implementing the AST/parser/semantic layers should enforce:
 *
 *     #![forbid(unsafe_code)]
 *
 * where appropriate.
 *
 * This grammar does not require unsafe functionality.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     canonical identifier grammar
 *     canonical type/name grammar
 *     grammar/declarations/records.g4
 *
 * The declaration grammar is a semantic counterpart, not a parser dependency
 * that should create cyclic grammar imports.
 *
 * EXPORTS:
 *
 *     recordType
 *
 * CONSUMED_BY:
 *
 *     modular type composition/tooling where a record-specific type category
 *     is required.
 *
 * AST_OWNER:
 *
 *     existing canonical frontend TypeExpr representation.
 *
 * SEMANTIC_OWNER:
 *
 *     canonical semantic type system.
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/type-system.md
 *     grammar/spec/data.md
 *
 * DECLARATION_OWNER:
 *
 *     grammar/declarations/records.g4
 *
 * IR_OWNER:
 *
 *     canonical semantic/IR layers.
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/types/
 *     repository parser/semantic integration tests.
 *
 * ============================================================================
 * IMPORTANT GRAMMAR-COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file MUST NOT be added as a competing alternative alongside:
 *
 *     namedType
 *
 * in the canonical `typeExpression` rule.
 *
 * Doing:
 *
 *     typeExpression
 *         : namedType
 *         | recordType
 *         | ...
 *
 * would make:
 *
 *     recordType -> namedType
 *
 * redundant and could create unnecessary ambiguity or duplicate parse paths.
 *
 * Instead, the canonical type grammar should continue to own:
 *
 *     namedType
 *
 * directly.
 *
 * This file is available as a modular semantic/category adapter for consumers
 * that need a record-specific parser rule without changing the universal type
 * syntax.
 *
 * ============================================================================
 * REQUIRED DECLARATION INTEGRATION
 * ============================================================================
 *
 * `grammar/declarations/records.g4` remains the sole declaration owner.
 *
 * It must retain:
 *
 *     recordDeclaration
 *     recordBody
 *     recordField
 *
 * and must consume the canonical:
 *
 *     identifier
 *     genericParameters
 *     whereClause
 *     typeExpression
 *
 * rules rather than importing this file merely to parse declaration fields.
 *
 * ============================================================================
 * REQUIRED TYPES INTEGRATION
 * ============================================================================
 *
 * `grammar/types/types.g4` remains the canonical type composition owner.
 *
 * It should continue to resolve record references through:
 *
 *     namedType
 *
 * rather than creating a special source spelling.
 *
 * This ensures that:
 *
 *     record;
 *     struct;
 *     enum;
 *     class;
 *     interface;
 *     trait;
 *     alias;
 *     external type;
 *
 * all participate in the same source-level named-type mechanism.
 *
 * ============================================================================
 * REQUIRED COMPOSITE-TYPE INTEGRATION
 * ============================================================================
 *
 * `grammar/types/composite-types.g4` must not add record declarations to its
 * composite type alternatives.
 *
 * A named record is not a structural composite syntax category.
 *
 * It is a nominal type reference.
 *
 * Therefore:
 *
 *     RecordName
 *
 * remains a named type.
 *
 * A structural aggregate such as:
 *
 *     (A, B)
 *
 * remains a tuple.
 *
 * This distinction prevents record syntax from being confused with:
 *
 *     tuple;
 *     array;
 *     option;
 *     result;
 *     function;
 *     slice.
 *
 * ============================================================================
 * REQUIRED DATA INTEGRATION
 * ============================================================================
 *
 * `grammar/data/schemas.g4` may refer to record declarations and types but
 * MUST NOT redefine:
 *
 *     recordDeclaration
 *     recordType
 *     recordField
 *     typeExpression
 *
 * Data/schema semantics must consume the canonical record/type model.
 *
 * ============================================================================
 * REQUIRED EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Record construction/access remains in expression/data grammar.
 *
 * The existing AST's:
 *
 *     Expression::Struct
 *
 * remains the aggregate-expression representation unless and until the
 * language specification explicitly introduces a distinct record-expression
 * node.
 *
 * Such a change must happen in the AST/specification first; this file must
 * not silently create one.
 *
 * ============================================================================
 * REQUIRED AST INTEGRATION
 * ============================================================================
 *
 * For:
 *
 *     Person
 *
 * expected source AST representation:
 *
 *     TypeExpr::Identifier(Person)
 *
 * For:
 *
 *     Person<T>
 *
 * expected representation:
 *
 *     TypeExpr::Generic(
 *         TypeExpr::Identifier(Person),
 *         [T]
 *     )
 *
 * The exact construction remains the responsibility of the existing AST
 * builder/parser integration.
 *
 * This grammar does not contain embedded AST construction code.
 *
 * ============================================================================
 * REQUIRED SEMANTIC INTEGRATION
 * ============================================================================
 *
 * After parsing:
 *
 *     TypeExpr::Identifier("Person")
 *
 * the semantic layer resolves `Person`.
 *
 * If it resolves to a record declaration:
 *
 *     record Person { ... }
 *
 * the semantic type becomes the canonical semantic representation of that
 * record.
 *
 * The semantic layer is responsible for:
 *
 *     - declaration lookup;
 *     - generic argument validation;
 *     - alias expansion;
 *     - nominal identity;
 *     - field metadata;
 *     - field visibility;
 *     - recursive type validation;
 *     - ownership/resource checks;
 *     - capability checks where applicable;
 *     - effect checks where applicable;
 *     - domain-specific constraints.
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * Canonical flow:
 *
 *     record source declaration
 *          ->
 *     AST declaration
 *          ->
 *     semantic record type
 *          ->
 *     canonical semantic representation
 *          ->
 *     IR
 *          ->
 *     optimization
 *          ->
 *     target lowering
 *
 * A record containing quantum fields may eventually participate in:
 *
 *     quantum::ir
 *
 * but only after semantic/domain lowering determines that quantum semantics
 * are actually involved.
 *
 * This grammar must never import or construct quantum::ir.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * The following are NOT determined here:
 *
 *     field offsets;
 *     alignment;
 *     padding;
 *     memory bank;
 *     register allocation;
 *     cache placement;
 *     GPU memory space;
 *     FPGA storage primitive;
 *     ASIC implementation;
 *     QPU representation;
 *     network serialization;
 *     distributed placement.
 *
 * Those are backend/interoperability concerns.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Because this file delegates to the canonical named-type grammar, scalability
 * is inherited from the universal type system rather than represented by
 * record-specific finite enumeration.
 *
 * There is no record-specific limit on:
 *
 *     record declarations;
 *     record references;
 *     generic arguments;
 *     type nesting;
 *     program size;
 *     machine size.
 *
 * Any practical compiler protection belongs to implementation policy.
 *
 * A resource-exhausted compiler must distinguish:
 *
 *     implementation resource exhaustion
 *
 * from:
 *
 *     source-language type invalidity.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing source:
 *
 *     record Person { ... }
 *
 * remains owned by:
 *
 *     grammar/declarations/records.g4
 *
 * Existing type references:
 *
 *     Person
 *     module::Person
 *
 * remain valid through the canonical named-type grammar.
 *
 * No source spelling is changed by this file.
 *
 * This file therefore has zero required source-language migration cost.
 *
 * ============================================================================
 * NEGATIVE REQUIREMENTS
 * ============================================================================
 *
 * This file MUST NOT:
 *
 *     - introduce `record` as a type-use keyword;
 *     - duplicate record declarations;
 *     - duplicate field syntax;
 *     - create RecordTypeExpr;
 *     - create RecordTypeNode;
 *     - create a record-specific IR;
 *     - inspect hardware;
 *     - inspect resources;
 *     - allocate memory;
 *     - allocate qubits;
 *     - select a backend;
 *     - define ABI layout;
 *     - define serialization layout;
 *     - contain embedded Rust;
 *     - contain semantic predicates;
 *     - contain unsafe code;
 *     - define fixed record capacities.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The adapter itself should have parser-conformance coverage for delegation:
 *
 * POSITIVE:
 *
 *     Person
 *     domain::Person
 *     package::domain::Person
 *
 * GENERIC:
 *
 *     Person<T>
 *     Person<A, B>
 *     domain::Person<A, B>
 *
 * NESTED:
 *
 *     Option<Person>
 *     Result<Person, Error>
 *     (Person, Other)
 *     [Person]
 *
 * CROSS-DOMAIN:
 *
 *     QuantumRecord
 *     DeviceState
 *     TensorRecord
 *     DistributedState
 *     AgentState
 *
 * These are ordinary named types and must remain compatible with the canonical
 * type-expression parser.
 *
 * SEMANTIC POSITIVE:
 *
 *     record Person {
 *         name: String,
 *     }
 *
 *     let p: Person = ...;
 *
 * must resolve `Person` to the declared record.
 *
 * GENERIC SEMANTIC POSITIVE:
 *
 *     record Pair<T, U> {
 *         first: T,
 *         second: U,
 *     }
 *
 *     let p: Pair<Integer, String> = ...;
 *
 * must resolve through the canonical generic type system.
 *
 * NEGATIVE SEMANTIC:
 *
 *     UnknownRecord
 *
 * must produce a semantic unknown-type diagnostic, not a special parser
 * diagnostic.
 *
 * WRONG GENERIC ARITY:
 *
 *     Pair<Integer>
 *
 * must be rejected by generic semantic validation when the declaration
 * requires more arguments.
 *
 * INVALID FIELD:
 *
 *     p.nonexistent
 *
 * must be diagnosed by name/member semantic analysis, not this grammar.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must demonstrate that record types can participate in structures of
 * increasing size without grammar-defined ceilings.
 *
 * Examples:
 *
 *     record Node<T> {
 *         value: T,
 *         next: Option<Node<T>>,
 *     }
 *
 * and large program-generated collections of record declarations/references.
 *
 * Tests may use explicit implementation resource budgets, but those budgets
 * must never become grammar constants or source-language restrictions.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Identical source and lexer configuration must produce identical parse trees.
 *
 * The record-type adapter must introduce no semantic predicates or
 * target-dependent behavior that could make parsing nondeterministic.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS:
 *
 *     [x] no machine capacity;
 *     [x] no hardware capacity;
 *     [x] no quantum capacity;
 *     [x] no field-count limit;
 *     [x] no generic-arity limit;
 *     [x] no type-depth limit;
 *     [x] no memory limit;
 *     [x] no fixed representation;
 *     [x] no fixed ABI;
 *     [x] no vendor dependency;
 *     [x] no target-specific syntax;
 *     [x] no embedded Rust;
 *     [x] no unsafe code;
 *     [x] no second AST type hierarchy;
 *     [x] no second record declaration grammar;
 *     [x] no second record literal grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * THIS FILE IS COMPLETE WHEN:
 *
 *     [x] RecordTypes parser grammar exists.
 *     [x] `recordType` delegates to canonical `namedType`.
 *     [x] No record declaration syntax is duplicated.
 *     [x] No record field syntax is duplicated.
 *     [x] No new TypeExpr variant is required.
 *     [x] Generic records use canonical generic syntax.
 *     [x] Qualified records use canonical name syntax.
 *     [x] Record literals remain outside this file.
 *     [x] Record semantics remain downstream.
 *     [x] No hardware assumptions exist.
 *     [x] No resource limits exist.
 *     [x] No unsafe Rust is required.
 *     [x] Quantum integration remains downstream.
 *     [x] HDL integration remains downstream.
 *     [x] Data/AI integration remains generic.
 *     [x] POCO-REAF is preserved.
 *
 * REPOSITORY INTEGRATION IS COMPLETE WHEN:
 *
 *     [ ] `grammar/declarations/records.g4` remains the sole record-declaration
 *         owner.
 *
 *     [ ] `grammar/types/types.g4` continues to parse record references through
 *         `namedType`.
 *
 *     [ ] `grammar/types/generic.g4` remains the sole generic type-argument
 *         owner.
 *
 *     [ ] `grammar/core/names.g4` remains the sole qualified-name owner.
 *
 *     [ ] AST construction maps record references to existing TypeExpr
 *         variants.
 *
 *     [ ] semantic resolution identifies the referenced declaration as a
 *         record when appropriate.
 *
 *     [ ] `grammar/data/schemas.g4` consumes rather than duplicates record
 *         syntax.
 *
 *     [ ] record parser/semantic tests pass.
 *
 *     [ ] nested/generic/cross-domain tests pass.
 *
 *     [ ] scalability tests pass.
 *
 *     [ ] deterministic parsing tests pass.
 *
 *     [ ] no duplicate canonical record grammar remains elsewhere.
 *
 * ============================================================================
 * FINAL DESIGN DECISION
 * ============================================================================
 *
 * The universal Zamani type architecture intentionally does NOT need a
 * special source spelling for every declaration kind.
 *
 * The correct relationship is:
 *
 *     record declaration
 *             |
 *             v
 *       named semantic type
 *             |
 *             v
 *       canonical namedType
 *             |
 *             v
 *          TypeExpr
 *             |
 *             v
 *      semantic type system
 *             |
 *       +-----+------+----------------+
 *       |            |                |
 *    classical    quantum::ir       HDL/other
 *       |            |                |
 *       +------------+----------------+
 *                    |
 *                 canonical
 *                    IR
 *                    |
 *          optimization/lowering
 *                    |
 *       routing/scheduling/resilience
 *                    |
 *               target realization
 *
 * This preserves a single type system, a single AST representation, and
 * target-independent record semantics while allowing record values to be used
 * throughout classical, quantum, HDL, AI, data, distributed, networking and
 * future computational domains.
 *
 * ============================================================================
 */