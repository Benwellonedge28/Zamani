/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/linear.g4
 *
 * Grammar:
 *     LinearTypes
 *
 * Status:
 *     PRODUCTION
 *
 * Language role:
 *     Source-level linear type qualifier.
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns exactly one source-level construct:
 *
 *     linear
 *
 * as a type qualifier.
 *
 * It deliberately does NOT parse the type that follows the qualifier.
 *
 * Complete type expressions are owned by:
 *
 *     grammar/types/types.g4
 *
 * Therefore:
 *
 *     linear T
 *
 * is composed by the canonical type grammar as:
 *
 *     linearQualifier + typeExpression
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Linearity is a type-system property describing permitted value usage.
 *
 * This grammar records only the source-level qualifier.
 *
 * It does NOT decide:
 *
 *     - whether a particular type may be linear;
 *     - whether a value must be consumed;
 *     - whether a value may be copied;
 *     - whether a value may be moved;
 *     - whether aliases are permitted;
 *     - whether destruction consumes a value;
 *     - whether a value is resource-bearing;
 *     - whether a value requires a capability;
 *     - whether a value requires a physical resource;
 *     - whether a value requires a quantum resource;
 *     - whether a value maps to hardware;
 *     - whether a value is executable;
 *     - whether a value is schedulable.
 *
 * Those decisions belong to downstream semantic analysis.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     linearQualifier
 *
 * THIS FILE DOES NOT OWN:
 *
 *     typeExpression
 *     typeCore
 *     typePostfix
 *     typePrefix
 *     affineQualifier
 *     references
 *     pointers
 *     generics
 *     tuples
 *     arrays
 *     slices
 *     functions
 *     dependent types
 *     associated types
 *     type classes
 *     quantum types
 *     classical types
 *     HDL types
 *     hardware types
 *     resource types
 *     capability types
 *     effects
 *     contracts
 *     policies
 *     provenance
 *     ownership analysis
 *     borrow analysis
 *     move analysis
 *     copy analysis
 *     resource allocation
 *     capability negotiation
 *     target selection
 *     optimization
 *     lowering
 *     routing
 *     scheduling
 *     resilience
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through the canonical token vocabulary:
 *
 *     LINEAR
 *
 * The lexer token is ultimately owned by the repository's lexical subsystem.
 *
 * This grammar MUST NOT define another:
 *
 *     LINEAR
 *     LINEAR_KEYWORD
 *     LINEAR_TYPE
 *     'linear'
 *
 * token.
 *
 * EXPORTS:
 *
 *     linearQualifier
 *
 * CONSUMED_BY:
 *
 *     grammar/types/types.g4
 *
 * Indirectly consumed by:
 *
 *     grammar/Zamani.g4
 *     canonical parser/frontend
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It MUST use:
 *
 *     tokenVocab = ZamaniLexer
 *
 * It MUST NOT import:
 *
 *     Types
 *
 * or any other grammar that owns complete type expressions.
 *
 * Dependency direction is:
 *
 *     LinearTypes
 *          |
 *          | exports linearQualifier
 *          v
 *     Type
 *          |
 *          | owns typeExpression
 *          v
 *     canonical frontend
 *
 * NOT:
 *
 *     Type -> LinearTypes -> Type
 *
 * The latter would create a circular grammar dependency.
 *
 * ============================================================================
 * PUBLIC GRAMMAR API
 * ============================================================================
 *
 * `linearQualifier` consumes exactly one canonical LINEAR token.
 *
 * The rule intentionally does not consume the operand type.
 *
 * Example composition:
 *
 *     linear T
 *
 * is handled by the importing type grammar:
 *
 *     typeExpression
 *         : typePrefix* typeCore typePostfix*
 *         ;
 *
 *     typePrefix
 *         : linearTypePrefix
 *         | ...
 *         ;
 *
 *     linearTypePrefix
 *         : linearQualifier
 *         ;
 *
 * The result is structurally:
 *
 *     linearQualifier
 *     typeExpression
 *
 * ============================================================================
 * QUALIFIER OWNERSHIP
 * ============================================================================
 *
 * `linear.g4` owns:
 *
 *     linearQualifier
 *
 * `affine.g4` owns:
 *
 *     affineQualifier
 *
 * They are intentionally independent.
 *
 * The canonical type orchestrator combines them.
 *
 * This prevents:
 *
 *     linear.g4
 *
 * from becoming an ownership-discipline grammar that silently owns affine
 * syntax as well.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar LinearTypes;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * LINEAR QUALIFIER
 * ============================================================================
 *
 * Canonical source spelling:
 *
 *     linear
 *
 * Canonical lexer token:
 *
 *     LINEAR
 *
 * The operand type is intentionally NOT consumed here.
 *
 * Correct:
 *
 *     linearQualifier
 *         : LINEAR
 *         ;
 *
 * Incorrect:
 *
 *     linearQualifier
 *         : LINEAR typeExpression
 *         ;
 *
 * The incorrect form would make this component depend on the complete type
 * grammar and would violate the repository's single-owner architecture.
 *
 * ============================================================================
 */

linearQualifier
    : LINEAR
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does not construct an AST.
 *
 * The canonical frontend AST already provides:
 *
 *     TypeExpr::Linear(Box<TypeExpr>)
 *
 * Therefore the complete source construct:
 *
 *     linear T
 *
 * is represented downstream as conceptually:
 *
 *     TypeExpr::Linear(T)
 *
 * No additional:
 *
 *     LinearType
 *     LinearTypeExpr
 *     LinearNode
 *
 * AST hierarchy is introduced here.
 *
 * Source spans for the LINEAR token must remain available through the normal
 * parser/frontend source-context machinery.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The grammar establishes only syntactic presence of the linear qualifier.
 *
 * Semantic analysis owns:
 *
 *     - linear type validity;
 *     - linear-use checking;
 *     - move checking;
 *     - copy checking;
 *     - destruction/consumption checking;
 *     - control-flow ownership joins;
 *     - function argument ownership;
 *     - function return ownership;
 *     - generic substitution;
 *     - reference interaction;
 *     - pointer interaction;
 *     - resource interaction;
 *     - capability interaction;
 *     - effect interaction.
 *
 * In particular, this grammar MUST NOT contain semantic predicates that try
 * to determine whether the qualified type is valid.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The operand of `linear` is whatever complete type expression the canonical
 * type system accepts.
 *
 * This means the grammar does not need to know the universe of types.
 *
 * Examples that may be accepted by the surrounding type grammar include:
 *
 *     linear T
 *     linear User
 *     linear module::User
 *     linear Vec<T>
 *     linear Result<T, E>
 *     linear (T, U)
 *     linear [T]
 *     linear fn(T) -> U
 *     linear &T
 *     linear &mut T
 *     linear Qubit
 *     linear Resource<T>
 *     linear Tensor<T, Shape>
 *
 * Whether any particular combination is semantically legal is decided after
 * parsing.
 *
 * No finite type whitelist is permitted here.
 *
 * ============================================================================
 * GENERIC / SYMBOLIC SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no limit on:
 *
 *     - generic arity;
 *     - identifier length;
 *     - qualified-name depth;
 *     - type nesting;
 *     - symbolic dimensions;
 *     - resource quantities;
 *     - quantum quantities;
 *     - hardware quantities;
 *     - tensor dimensions.
 *
 * It contains no implementation capacity constants.
 *
 * It therefore does not impose a language-level machine-size ceiling.
 *
 * Actual compiler, memory, parser-stack, execution, and target-resource
 * limitations remain implementation/resource concerns rather than grammar
 * semantics.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Applying the `linear` qualifier is not itself an execution effect.
 *
 * It does not imply:
 *
 *     IO
 *     mutation
 *     allocation
 *     deallocation
 *     networking
 *     randomness
 *     native execution
 *     foreign execution
 *     quantum measurement
 *     learning
 *     adaptation
 *     reflection
 *     simulation
 *
 * Effects belong to the canonical effect system and to the operations that
 * use the resulting value.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Parsing `linear` requires no hardware or execution capability.
 *
 * A type qualified by `linear` may later participate in a semantic capability
 * requirement.
 *
 * For example, a user-defined type might semantically represent a resource
 * requiring some capability.
 *
 * Such requirements belong to:
 *
 *     grammar/resources/
 *
 * and the semantic capability/resource systems.
 *
 * This grammar MUST NOT contain capability names.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * A linear value can represent a resource-bearing abstraction, but the
 * qualifier does not allocate or reserve a resource.
 *
 * For example:
 *
 *     linear Resource<T>
 *
 * expresses source-level type information only.
 *
 * It does not mean:
 *
 *     allocate hardware;
 *     reserve memory;
 *     reserve a processor;
 *     reserve a GPU;
 *     reserve a QPU;
 *     reserve a node;
 *     reserve a device.
 *
 * Resource feasibility remains downstream.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * The linear qualifier may be referenced by operations governed by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * However, none of those contract constructs belong in this file.
 *
 * Contract syntax and verification remain owned by the validation/contract
 * subsystem.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may constrain operations involving linear values.
 *
 * Examples include policies concerning:
 *
 *     transfer
 *     persistence
 *     duplication
 *     external calls
 *     distribution
 *     adaptation
 *
 * Policy syntax and evaluation remain outside this grammar.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parser/frontend must preserve the source context of LINEAR.
 *
 * Downstream provenance may record:
 *
 *     source declaration
 *     source qualifier
 *     semantic interpretation
 *     validation
 *     transformation
 *     optimization
 *     lowering
 *
 * This grammar does not create provenance records.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Linearity is useful for values whose semantic domain includes quantum
 * resources.
 *
 * Examples:
 *
 *     linear Qubit
 *     linear LogicalQubit
 *     linear QuantumResource<T>
 *
 * remain ordinary source-level type expressions.
 *
 * This grammar MUST NOT encode:
 *
 *     physical qubit identifiers
 *     QPU identifiers
 *     coupling maps
 *     topology
 *     gate sets
 *     decomposition
 *     routing
 *     scheduling
 *     calibration
 *     error-correction strategy
 *     resilience strategy
 *
 * If a linear type participates in quantum computation, downstream processing
 * remains:
 *
 *     source
 *       |
 *       v
 *     AST TypeExpr::Linear
 *       |
 *       v
 *     semantic type analysis
 *       |
 *       v
 *     quantum semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition/routing/scheduling
 *       |
 *       v
 *     resilience/QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * No linear-specific quantum IR is created.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A linear-qualified type may represent an HDL, hardware, accelerator, or
 * device resource abstraction if the canonical type system permits it.
 *
 * This grammar does not identify a physical implementation.
 *
 * It MUST NOT encode:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     ASIC capacity
 *     node count
 *     memory capacity
 *     register width
 *     network size
 *     device count
 *
 * Hardware realization belongs to hardware/resource/capability analysis and
 * target lowering.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The source-level meaning of:
 *
 *     linear T
 *
 * is independent of the size or identity of the machine on which T is
 * eventually realized.
 *
 * Therefore this grammar contains no artificial machine ceiling.
 *
 * It MUST NOT define or imply constants such as:
 *
 *     MAX_LINEAR_VALUES
 *     MAX_LINEAR_TYPES
 *     MAX_TYPE_DEPTH
 *     MAX_GENERIC_ARITY
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * No resource number is encoded by this qualifier.
 *
 * A program remains source-compatible across target scales as long as its
 * semantic requirements can be satisfied by the selected realization.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing this construct must depend only on:
 *
 *     - input token stream;
 *     - grammar version;
 *     - lexer vocabulary;
 *     - parser configuration.
 *
 * It MUST NOT depend on:
 *
 *     - available CPU count;
 *     - available GPU count;
 *     - available QPU;
 *     - machine memory;
 *     - network topology;
 *     - filesystem state;
 *     - wall-clock time;
 *     - randomness;
 *     - scheduler state;
 *     - target vendor.
 *
 * Identical source and identical language configuration must produce identical
 * parser structure.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar is declarative and side-effect free.
 *
 * It contains:
 *
 *     - no embedded Rust;
 *     - no target-language actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment inspection;
 *     - no command execution;
 *     - no dynamic evaluation.
 *
 * Consequently it does not introduce an unsafe Rust requirement.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Stable source spelling:
 *
 *     linear
 *
 * Stable lexer token:
 *
 *     LINEAR
 *
 * Stable parser rule:
 *
 *     linearQualifier
 *
 * Stable AST destination:
 *
 *     TypeExpr::Linear
 *
 * The previous implementation exposed additional rule names and duplicated
 * qualifier definitions. Those are intentionally removed from the production
 * grammar because they are not required by the current canonical type
 * orchestrator.
 *
 * In particular, this file does NOT retain a compatibility alias that creates
 * another ownership/linearity rule unless an actual production consumer
 * requires that alias.
 *
 * ============================================================================
 * REQUIRED INTEGRATION WITH `grammar/types/types.g4`
 * ============================================================================
 *
 * The canonical type grammar already defines:
 *
 *     typeExpression
 *     typePrefix
 *     linearTypePrefix
 *
 * with the integration boundary:
 *
 *     linearTypePrefix
 *         : linearQualifier
 *         ;
 *
 * The production grammar composition must import LinearTypes into the
 * canonical Type grammar using the repository's ANTLR composition mechanism.
 *
 * Conceptually:
 *
 *     Type
 *       |
 *       +--> LinearTypes
 *       |       |
 *       |       +--> linearQualifier
 *       |
 *       +--> typeExpression
 *
 * `types.g4` remains the owner of complete type composition.
 *
 * `linear.g4` remains the owner of the LINEAR qualifier.
 *
 * Neither file should absorb the other's ownership.
 *
 * ============================================================================
 * REQUIRED INTEGRATION WITH `grammar/types/affine.g4`
 * ============================================================================
 *
 * `affine.g4` independently owns:
 *
 *     affineQualifier
 *
 * This file MUST NOT reference AFFINE.
 *
 * The canonical type orchestrator combines:
 *
 *     linearQualifier
 *
 * and:
 *
 *     affineQualifier
 *
 * at the type-prefix level.
 *
 * ============================================================================
 * REQUIRED INTEGRATION WITH THE LEXER
 * ============================================================================
 *
 * The lexer subsystem owns the spelling:
 *
 *     linear
 *
 * and emits:
 *
 *     LINEAR
 *
 * This parser grammar consumes that token through:
 *
 *     tokenVocab = ZamaniLexer
 *
 * No lexer change is required merely to make this parser component correct,
 * provided the canonical lexical pipeline continues exporting LINEAR.
 *
 * ============================================================================
 * AST / SEMANTIC / IR BOUNDARY
 * ============================================================================
 *
 * The complete pipeline is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     LinearTypes.linearQualifier
 *       |
 *       v
 *     Type.typeExpression
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     TypeExpr::Linear
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic type analysis
 *       |
 *       +--> ownership/linearity analysis
 *       +--> resource analysis
 *       +--> capability analysis
 *       +--> effect analysis
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware semantics
 *       +--> other domain IR
 *       |
 *       v
 *     target-independent optimization
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     target realization
 *
 * This file owns only the parser node between the lexer and canonical type
 * composition.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Tests for this component should be driven through the canonical type
 * expression entry point wherever possible.
 *
 * POSITIVE STRUCTURAL CASES:
 *
 *     linear T
 *     linear User
 *     linear module::User
 *     linear Vec<T>
 *     linear Result<T, E>
 *     linear (T, U)
 *     linear [T]
 *     linear fn(T) -> U
 *     linear &T
 *     linear &mut T
 *     linear Qubit
 *     linear Resource<T>
 *
 * The exact accepted forms depend on the canonical type grammar.
 *
 * NEGATIVE STRUCTURAL CASES:
 *
 *     linear
 *     linear 123
 *     linear (
 *     linear <
 *     linear [
 *     linear &
 *     linear *
 *
 * These are complete-type failures owned by the enclosing type grammar.
 *
 * QUALIFIER ISOLATION:
 *
 *     affine T
 *
 * must be handled by AffineTypes, not by this grammar.
 *
 * CROSS-DOMAIN:
 *
 *     linear Qubit
 *     linear Resource<Qubit>
 *     linear Tensor<T, Shape>
 *     linear HardwareResource
 *
 * must not require this grammar to know what those names mean.
 *
 * SCALABILITY:
 *
 * Test arbitrarily large symbolic/nested type structures subject only to
 * external parser/compiler resource policies.
 *
 * No test may establish a language-level maximum.
 *
 * DETERMINISM:
 *
 * Repeated parsing of identical source under identical parser configuration
 * must produce identical parser structure.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Allowed fixed language element:
 *
 *     LINEAR
 *
 * No physical resource is named.
 *
 * No finite domain inventory is named.
 *
 * No capacity is encoded.
 *
 * No hardware limit is encoded.
 *
 * No vendor operation is encoded.
 *
 * No quantum topology is encoded.
 *
 * No tensor-rank ceiling is encoded.
 *
 * No generic-arity ceiling is encoded.
 *
 * No type-depth ceiling is encoded.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when all of the following are true:
 *
 *     [x] Exactly one parser grammar declaration exists.
 *     [x] Grammar name is LinearTypes.
 *     [x] tokenVocab is ZamaniLexer.
 *     [x] Exactly one canonical linearQualifier rule exists.
 *     [x] linearQualifier consumes only LINEAR.
 *     [x] No AFFINE token is owned here.
 *     [x] No typeExpression is defined here.
 *     [x] No typeCore is defined here.
 *     [x] No lexer rule is defined here.
 *     [x] No embedded Rust exists.
 *     [x] No semantic predicates exist.
 *     [x] No AST implementation is duplicated.
 *     [x] No IR is created.
 *     [x] No resource limit is encoded.
 *     [x] No hardware target is encoded.
 *     [x] Quantum semantics remain downstream.
 *     [x] HDL semantics remain downstream.
 *     [x] Resource/capability semantics remain downstream.
 *     [x] Effect semantics remain downstream.
 *     [x] Contract semantics remain downstream.
 *     [x] Policy semantics remain downstream.
 *     [x] Provenance remains downstream.
 *     [x] TypeExpr::Linear remains the canonical AST destination.
 *     [x] Rust 1.97+ compatibility is preserved.
 *     [x] Safe Rust remains sufficient.
 *
 * ============================================================================
 */