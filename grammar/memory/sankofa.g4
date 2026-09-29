/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/sankofa.g4
 *
 * Grammar:
 *     Sankofa
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Status:
 *     CANONICAL SANKOFA MEMORY-DOMAIN GRAMMAR
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     2021
 *
 * Safety:
 *     Safe Rust only.
 *
 *     This grammar contains no Rust actions and requires no unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the source-language grammar for Zamani's Sankofa memory
 * facilities.
 *
 * Sankofa is a memory/knowledge/temporal facility of Zamani.
 *
 * It provides source-level constructs for concepts including:
 *
 *     remember
 *     recall
 *     learn
 *     infer
 *     wisdom
 *     zamani
 *     sasa
 *     temporal state
 *     history
 *     provenance
 *     ancestry/lineage
 *     knowledge
 *     conclusions
 *     consensus
 *     inter-memory communication
 *
 * The grammar describes SOURCE INTENT ONLY.
 *
 * It does NOT implement:
 *
 *     memory storage
 *     persistence
 *     databases
 *     caches
 *     retrieval algorithms
 *     learning algorithms
 *     inference engines
 *     consensus algorithms
 *     temporal execution
 *     distributed replication
 *     provenance storage
 *     trust computation
 *     cryptography
 *     AI models
 *     quantum computation
 *     hardware selection
 *     scheduling
 *     routing
 *     resource discovery
 *     runtime execution
 *
 * Those responsibilities belong downstream.
 *
 * ============================================================================
 * CORE ARCHITECTURAL RULE
 * ============================================================================
 *
 * Sankofa is NOT a second language.
 *
 * It is a memory-domain extension of Zamani.
 *
 * Therefore:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic Sankofa model
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     compiler/runtime realization
 *
 * No Sankofa-specific competing AST root or IR is introduced here.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Sankofa MUST preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A Sankofa program may therefore be realized on:
 *
 *     tiny embedded systems
 *     desktop systems
 *     servers
 *     multicore systems
 *     GPUs
 *     accelerators
 *     FPGA systems
 *     ASIC systems
 *     quantum-classical systems
 *     simulators
 *     HPC systems
 *     distributed systems
 *     clusters
 *     cloud systems
 *     future computational substrates
 *
 * The source grammar MUST NOT encode assumptions about the target machine.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There is intentionally NO universal upper bound for:
 *
 *     memories
 *     memory records
 *     remembered values
 *     recalled values
 *     knowledge items
 *     learning events
 *     inference steps
 *     history entries
 *     provenance records
 *     temporal states
 *     timelines
 *     branches
 *     ancestors
 *     descendants
 *     consensus participants
 *     memory domains
 *     inter-memory links
 *     namespaces
 *     identifiers
 *     program size
 *     expression size
 *
 * The grammar MUST NOT define constants such as:
 *
 *     MAX_MEMORIES
 *     MAX_MEMORY_ITEMS
 *     MAX_RECALLS
 *     MAX_LEARNING_STEPS
 *     MAX_INFERENCE_STEPS
 *     MAX_HISTORY
 *     MAX_TIMELINES
 *     MAX_ANCESTORS
 *     MAX_DESCENDANTS
 *     MAX_CONSENSUS_PARTICIPANTS
 *     MAX_KNOWLEDGE
 *     MAX_MEMORY_SIZE
 *     MAX_MEMORY_DEPTH
 *
 * Nor may it encode:
 *
 *     32-bit memory
 *     64-bit memory
 *     fixed history depth
 *     fixed timeline count
 *     fixed branch count
 *     fixed knowledge capacity
 *
 * Physical and implementation limits remain resource/capability facts.
 *
 * ============================================================================
 * PROGRAM VALUE VS IMPLEMENTATION LIMIT
 * ============================================================================
 *
 * A numeric value in source is ordinary program semantics.
 *
 * For example:
 *
 *     remember count = 1024;
 *
 * does NOT mean that Sankofa has a maximum of 1024 memories.
 *
 * Likewise:
 *
 *     remember depth = n;
 *
 * may be an application-level value.
 *
 * The grammar MUST NOT reinterpret such values as compiler ceilings.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Language specification:
 *
 *     grammar/specification/
 *
 * Memory architecture:
 *
 *     grammar/memory/README.md
 *
 * Generic memory foundation:
 *
 *     grammar/memory/memory.g4
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical keyword vocabulary:
 *
 *     grammar/lexer/keywords.g4
 *
 * Canonical expressions:
 *
 *     grammar/expressions/
 *
 * Canonical types:
 *
 *     grammar/types/
 *
 * Resource semantics:
 *
 *     grammar/resources/
 *
 * Temporal type semantics:
 *
 *     grammar/types/temporal.g4
 *
 * Compatibility:
 *
 *     grammar/compatibility/
 *
 * Validation:
 *
 *     grammar/validation/
 *
 * Existing Rust frontend:
 *
 *     src/lexer.rs
 *     src/parser.rs
 *     src/ast/
 *     src/semantic.rs
 *     src/ir_gen.rs
 *
 * Existing Sankofa-related downstream consumers:
 *
 *     src/toolchain/causality_checker.rs
 *     src/compiler/monomorphizer.rs
 *
 * ============================================================================
 * TOKEN AUTHORITY
 * ============================================================================
 *
 * This file MUST consume the canonical ZamaniLexer vocabulary.
 *
 * It MUST NOT define lexer rules.
 *
 * Existing canonical Sankofa-related tokens include:
 *
 *     MTS
 *     ZAMANI
 *     SASA
 *     REMEMBER
 *     RECALL
 *     WISDOM
 *     LEARN
 *     INFER
 *
 * This grammar deliberately does NOT invent an ANCESTOR token because the
 * canonical ANTLR keyword grammar currently does not define one.
 *
 * The Rust lexer currently contains additional Sankofa-oriented vocabulary.
 * Such lexer/parser divergence must be resolved through the normal lexical
 * compatibility process rather than silently introducing a second token
 * authority here.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Sankofa operations are NOT represented as an exhaustive enumeration.
 *
 * This is intentionally prohibited:
 *
 *     sankofaOperation
 *         : REMEMBER
 *         | RECALL
 *         | LEARN
 *         | INFER
 *         | WISDOM
 *         | ...
 *
 * as the complete operation universe.
 *
 * The language must remain extensible.
 *
 * Stable language-level keywords provide syntactic anchors.
 *
 * Domain-specific and future operations may be represented through qualified
 * operation names and the canonical operation/expression model where the
 * surrounding grammar permits them.
 *
 * ============================================================================
 * AST PRINCIPLE
 * ============================================================================
 *
 * Syntax must preserve enough information to construct a domain-neutral AST.
 *
 * A Sankofa AST representation must preserve, where applicable:
 *
 *     source span
 *     operation identity
 *     namespace/path
 *     subject
 *     value
 *     query
 *     target
 *     parameters
 *     named arguments
 *     temporal qualifier
 *     provenance
 *     confidence
 *     policy
 *     resource requirements
 *     capability requirements
 *     constraints
 *     preferences
 *     hints
 *     metadata
 *
 * The grammar MUST NOT construct backend-specific nodes.
 *
 * In particular, it must not require:
 *
 *     DatabaseMemoryNode
 *     RedisMemoryNode
 *     GPUCacheMemoryNode
 *     QPUMemoryNode
 *     PhysicalMemoryNode
 *     DistributedNodeMemory
 *
 * ============================================================================
 * SEMANTIC PRINCIPLE
 * ============================================================================
 *
 * The semantic layer determines:
 *
 *     whether a Sankofa operation exists;
 *     whether its operands are valid;
 *     whether the referenced memory exists;
 *     whether temporal relations are valid;
 *     whether provenance is valid;
 *     whether trust/confidence requirements are satisfied;
 *     whether ownership/lifetime rules are satisfied;
 *     whether resource requirements can be satisfied;
 *     whether capabilities are available;
 *     whether a learning/inference operation is valid;
 *     whether a consensus operation is valid;
 *     whether a history operation is causally valid.
 *
 * The grammar does not perform these checks.
 *
 * ============================================================================
 * IR PRINCIPLE
 * ============================================================================
 *
 * Sankofa MUST NOT create a competing universal IR.
 *
 * The semantic path is:
 *
 *     Sankofa syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic Sankofa model
 *          |
 *          v
 *     canonical semantic representation / IR
 *
 * Existing downstream IR structures may be extended or adapted as required,
 * but the source grammar itself does not define a second IR.
 *
 * ============================================================================
 * TEMPORAL PRINCIPLE
 * ============================================================================
 *
 * Sankofa's temporal vocabulary describes semantic temporal relationships.
 *
 * It does NOT imply a physical clock.
 *
 * In particular:
 *
 *     zamani
 *     sasa
 *     past
 *     present
 *     history
 *     temporal state
 *
 * must not be interpreted by this grammar as:
 *
 *     CPU cycles
 *     wall-clock time
 *     fixed-width timestamps
 *     scheduler ticks
 *     hardware clock domains
 *
 * A concrete duration remains ordinary program data governed by the canonical
 * temporal/type system.
 *
 * ============================================================================
 * PROVENANCE PRINCIPLE
 * ============================================================================
 *
 * Memory and knowledge can have provenance.
 *
 * Provenance may describe:
 *
 *     source
 *     origin
 *     creator
 *     derivation
 *     transformation
 *     observation
 *     evidence
 *     parent
 *     temporal relation
 *     version
 *     confidence
 *     trust metadata
 *
 * Provenance syntax is metadata/intent.
 *
 * It does not itself establish that provenance is truthful.
 *
 * Verification belongs downstream.
 *
 * ============================================================================
 * TRUST / CONFIDENCE PRINCIPLE
 * ============================================================================
 *
 * Confidence and trust are semantic properties.
 *
 * The grammar may carry their source-level values or expressions.
 *
 * The grammar MUST NOT:
 *
 *     invent a universal confidence scale;
 *     require floating point;
 *     require a particular probability representation;
 *     declare a universal trust algorithm.
 *
 * Therefore:
 *
 *     confidence = expression
 *
 * is preferable to a fixed parser-level confidence representation.
 *
 * ============================================================================
 * CONSENSUS PRINCIPLE
 * ============================================================================
 *
 * Consensus is distinct from:
 *
 *     replication
 *     acknowledgement
 *     consistency
 *     ordering
 *     durability
 *     authorization
 *     trust
 *     availability
 *
 * Sankofa grammar may express consensus intent.
 *
 * It must not encode a particular consensus algorithm.
 *
 * The grammar must not assume:
 *
 *     leader election
 *     quorum size
 *     fixed participant count
 *     voting algorithm
 *     Byzantine threshold
 *     network topology
 *
 * Those are semantic/resource/runtime concerns.
 *
 * ============================================================================
 * LEARNING PRINCIPLE
 * ============================================================================
 *
 * `learn` expresses source-level learning intent.
 *
 * It does not mean that the parser:
 *
 *     trains a model;
 *     executes gradient descent;
 *     mutates persistent memory;
 *     accesses a dataset;
 *     invokes an AI framework.
 *
 * Learning semantics are downstream.
 *
 * ============================================================================
 * INFERENCE PRINCIPLE
 * ============================================================================
 *
 * `infer` expresses inference intent.
 *
 * It does not select:
 *
 *     a model;
 *     a hardware accelerator;
 *     a runtime;
 *     a solver;
 *     a neural framework;
 *     a symbolic engine.
 *
 * Those are semantic/backend concerns.
 *
 * ============================================================================
 * WISDOM PRINCIPLE
 * ============================================================================
 *
 * `wisdom` is a language-level knowledge declaration/operation already present
 * in the existing Rust AST and parser.
 *
 * Existing implementation:
 *
 *     Statement::Wisdom(Span, String, Expression)
 *
 * is therefore preserved as the initial compatibility representation.
 *
 * The grammar does not redefine the AST.
 *
 * ============================================================================
 * REMEMBER PRINCIPLE
 * ============================================================================
 *
 * Existing Rust implementation:
 *
 *     Statement::SankofaMemory(Span, String, Expression)
 *
 * currently supports the established source form:
 *
 *     remember name = expression;
 *
 * An optional type annotation is retained by this grammar because the
 * existing Rust parser already accepts:
 *
 *     remember name: Type = expression;
 *
 * The type information MUST eventually be preserved by the AST rather than
 * silently discarded.
 *
 * ============================================================================
 * RECALL / LEARN / INFER
 * ============================================================================
 *
 * These operations already have canonical lexical tokens but the current Rust
 * parser does not yet provide complete statement-level implementations for
 * them.
 *
 * This grammar therefore defines their canonical source contract so that
 * parser/AST/semantic work can converge on one specification.
 *
 * The implementation status MUST remain visible in grammar/grammar.md until
 * the Rust frontend implements the corresponding AST and semantic contracts.
 *
 * ============================================================================
 * SYNTAX DESIGN
 * ============================================================================
 *
 * The stable forms are intentionally small.
 *
 * REMEMBER:
 *
 *     remember name = expression;
 *     remember name: Type = expression;
 *
 * RECALL:
 *
 *     recall expression;
 *     recall expression;
 *         with named arguments;
 *
 * LEARN:
 *
 *     learn expression;
 *     learn expression from expression;
 *     learn expression with named arguments;
 *
 * INFER:
 *
 *     infer expression;
 *     infer expression from expression;
 *     infer expression with named arguments;
 *
 * WISDOM:
 *
 *     wisdom name;
 *     wisdom name = expression;
 *
 * These forms intentionally use the universal expression grammar instead of
 * creating a second Sankofa expression language.
 *
 * ============================================================================
 * OPEN EXTENSION FORM
 * ============================================================================
 *
 * Future Sankofa facilities may use qualified semantic operation names through
 * the canonical operation model where such syntax is admitted by the complete
 * Zamani grammar.
 *
 * Examples conceptually include:
 *
 *     sankofa::history(...)
 *     sankofa::provenance(...)
 *     sankofa::consensus(...)
 *     sankofa::lineage(...)
 *     sankofa::knowledge(...)
 *
 * The grammar does not hard-code these as an exhaustive operation list.
 *
 * ============================================================================
 * TEMPORAL QUALIFIERS
 * ============================================================================
 *
 * The existing lexical vocabulary includes:
 *
 *     ZAMANI
 *     SASA
 *     MTS
 *
 * This file provides explicit attachment points for temporal intent without
 * assigning physical time semantics.
 *
 * ============================================================================
 * MEMORY INTEGRATION
 * ============================================================================
 *
 * Sankofa builds on:
 *
 *     grammar/memory/memory.g4
 *
 * It may consume:
 *
 *     memoryPlace
 *     memoryQualifiedName
 *     memoryArgumentList
 *     memoryIntent
 *     memoryLifetime
 *     memoryRegion
 *     memorySpace
 *
 * where those rules are available through the canonical composed grammar.
 *
 * Sankofa does not redefine them.
 *
 * ============================================================================
 * OWNERSHIP / LIFETIME
 * ============================================================================
 *
 * A remembered value may have ownership/lifetime semantics.
 *
 * Those remain owned by:
 *
 *     grammar/memory/ownership.g4
 *     grammar/memory/borrowing.g4
 *     grammar/memory/lifetimes.g4
 *
 * Sankofa may attach those concepts through the generic memory model.
 *
 * It does not create a second ownership system.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Sankofa operations may require:
 *
 *     memory
 *     persistent storage
 *     compute
 *     inference capability
 *     learning capability
 *     temporal capability
 *     provenance capability
 *     consensus capability
 *     distributed communication
 *
 * Requirements must remain symbolic.
 *
 * Example:
 *
 *     requires capability("memory.persistence")
 *
 * is source intent.
 *
 * It is not a promise that the target possesses that capability.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Sankofa syntax must not name physical implementation resources as part of
 * its universal semantics.
 *
 * It must not require:
 *
 *     CPU 0
 *     GPU 0
 *     memory bank 0
 *     node 0
 *     device 0
 *     accelerator 0
 *
 * unless such physical identity is explicitly part of a downstream target
 * realization language.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Sankofa memory may be:
 *
 *     local
 *     shared
 *     replicated
 *     partitioned
 *     distributed
 *     persistent
 *
 * The grammar expresses intent.
 *
 * Actual placement and replication belong to:
 *
 *     grammar/distributed/
 *     grammar/resources/
 *     grammar/hardware/
 *     compiler
 *     runtime
 *
 * ============================================================================
 * AI INTEGRATION
 * ============================================================================
 *
 * Sankofa may participate in AI computation.
 *
 * The grammar does not create an AI memory implementation.
 *
 * AI semantics remain governed by:
 *
 *     grammar/ai/
 *
 * Sankofa provides memory/knowledge intent that AI semantic analysis may
 * consume.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Sankofa may store or recall classical data associated with quantum
 * computation.
 *
 * It may participate in hybrid computation.
 *
 * It does NOT define:
 *
 *     qubit storage;
 *     quantum state representation;
 *     quantum gates;
 *     quantum routing;
 *     QEC;
 *     ZQN;
 *     physical qubit allocation.
 *
 * Quantum semantics remain governed by:
 *
 *     grammar/quantum/
 *     quantum::ir
 *
 * Hybrid composition remains governed by:
 *
 *     grammar/hybrid/
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Sankofa memory intent may eventually be realized using:
 *
 *     registers
 *     SRAM
 *     DRAM
 *     persistent storage
 *     FPGA memory
 *     accelerator memory
 *     distributed memory
 *
 * but none of these are universal Sankofa grammar limits.
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Memory may contain sensitive or protected information.
 *
 * Sankofa syntax may carry security metadata through generic attributes,
 * policies, capabilities, or semantic metadata.
 *
 * It must not implement cryptography.
 *
 * Security semantics remain governed by:
 *
 *     grammar/security/
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST be deterministic.
 *
 * Identical:
 *
 *     source
 *     grammar version
 *     dialect configuration
 *
 * must produce the same parse structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     current time
 *     system time
 *     random numbers
 *     hardware
 *     memory contents
 *     network state
 *     filesystem state
 *     target availability
 *     runtime state
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * The grammar must allow the parser to preserve source spans for:
 *
 *     keyword
 *     name
 *     type
 *     expression
 *     temporal qualifier
 *     argument list
 *     operation
 *     declaration
 *
 * Semantic diagnostics belong downstream.
 *
 * Parser diagnostics should identify:
 *
 *     missing name
 *     missing assignment
 *     malformed expression
 *     malformed argument list
 *     malformed temporal clause
 *     malformed source construct
 *
 * They must not claim that a target lacks memory or compute resources.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions
 *     no predicates
 *     no executable code
 *     no filesystem access
 *     no network access
 *     no environment inspection
 *     no hardware discovery
 *     no secret access
 *     no runtime execution
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing source compatibility:
 *
 *     remember name = expression;
 *
 * MUST remain valid.
 *
 * Existing source compatibility:
 *
 *     remember name: Type = expression;
 *
 * SHOULD remain valid once the type information is preserved through the
 * frontend AST.
 *
 * Existing source compatibility:
 *
 *     wisdom name;
 *     wisdom name = expression;
 *
 * MUST remain representable.
 *
 * New RECALL/LEARN/INFER syntax must not be marked IMPLEMENTED merely because
 * this grammar specifies it.
 *
 * grammar/grammar.md must report actual frontend implementation status.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This file is a parser grammar.
 *
 * It consumes the canonical:
 *
 *     ZamaniLexer
 *
 * It must NOT define:
 *
 *     IDENTIFIER
 *     INTEGER
 *     STRING
 *     operators
 *     punctuation
 *
 * It must reuse the universal grammar rules exposed by the canonical parser
 * composition.
 *
 * The memory dispatcher is responsible for composing this grammar.
 *
 * Recommended composition:
 *
 *     grammar/memory/Memory.g4
 *              |
 *              +--> memory.g4
 *              +--> sankofa.g4
 *              +--> temporal-memory components
 *              +--> history components
 *              +--> provenance components
 *              +--> consensus components
 *
 * However, there must remain only ONE parser-level `Memory` composition
 * authority.
 *
 * ============================================================================
 * IMPORTANT IMPORT DIRECTION
 * ============================================================================
 *
 * This file MUST NOT import Memory.
 *
 * Doing so would create:
 *
 *     Memory -> Sankofa -> Memory
 *
 * circular composition.
 *
 * The canonical Memory dispatcher owns Sankofa composition.
 *
 * Therefore this file remains a leaf/domain grammar and relies on canonical
 * rules supplied by the composed parser hierarchy.
 *
 * ============================================================================
 * FILE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     sankofaConstruct
 *     sankofaStatement
 *     sankofaExpression
 *     sankofaRemember
 *     sankofaRecall
 *     sankofaLearn
 *     sankofaInfer
 *     sankofaWisdom
 *     sankofaTemporalQualifier
 *     sankofaTemporalReference
 *     sankofaSource
 *     sankofaTarget
 *     sankofaFromClause
 *     sankofaWithClause
 *     sankofaArgumentList
 *     sankofaArgument
 *     sankofaNamedArgument
 *     sankofaAssignmentTarget
 *     sankofaExtensionOperation
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     expressionList
 *     typeExpression
 *     memoryPlace
 *     memoryLifetime
 *     memoryRegion
 *     memorySpace
 *     resource
 *     capability
 *     provenance implementation
 *     history implementation
 *     consensus implementation
 *     learning implementation
 *     inference implementation
 *     storage implementation
 *     runtime implementation
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * sankofaRemember
 *     ->
 *     existing Statement::SankofaMemory compatibility representation
 *
 * Future AST evolution MUST preserve:
 *
 *     source span
 *     name
 *     optional type
 *     value
 *     attributes/metadata
 *
 * sankofaWisdom
 *     ->
 *     existing Statement::Wisdom compatibility representation
 *
 * Future AST evolution MUST preserve:
 *
 *     source span
 *     name
 *     optional value
 *     metadata
 *
 * sankofaRecall
 * sankofaLearn
 * sankofaInfer
 *
 * MUST map to domain-neutral operation/statement representation rather than
 * backend-specific nodes.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * `remember`:
 *
 *     establishes or updates a semantic memory binding.
 *
 * `recall`:
 *
 *     requests retrieval of information.
 *
 * `learn`:
 *
 *     expresses learning/update intent.
 *
 * `infer`:
 *
 *     expresses inference/reasoning intent.
 *
 * `wisdom`:
 *
 *     declares or records a higher-level knowledge/wisdom value.
 *
 * Temporal qualifiers:
 *
 *     select semantic temporal context.
 *
 * None of these constructs directly execute runtime behavior during parsing.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Sankofa operations may carry resource/capability requirements through
 * surrounding universal resource syntax.
 *
 * The grammar does not decide whether a requirement can be satisfied.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Sankofa syntax must remain compatible with provenance metadata supplied by
 * the memory/provenance grammar.
 *
 * The parser preserves metadata.
 *
 * Verification belongs downstream.
 *
 * ============================================================================
 * CONSENSUS CONTRACT
 * ============================================================================
 *
 * Consensus-related Sankofa facilities must consume the repository's generic
 * consensus/resource/distributed semantic contracts.
 *
 * This grammar does not select a consensus algorithm.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Malformed Sankofa syntax must be rejected structurally.
 *
 * Examples:
 *
 *     remember;
 *     remember = value;
 *     remember name;
 *     remember name = ;
 *     wisdom;
 *     wisdom = value;
 *     recall ;
 *     learn ;
 *     infer ;
 *
 * Whether an otherwise syntactically valid memory name exists, whether an
 * expression has a valid type, whether a temporal reference exists, and
 * whether resources are available are semantic questions.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must verify:
 *
 *     one memory
 *     many memories
 *     symbolic memory values
 *     large expressions
 *     nested expressions
 *     temporal references
 *     repeated recall
 *     repeated learning
 *     repeated inference
 *     large provenance chains
 *     large histories
 *     distributed memory
 *     persistent memory
 *
 * Tests MUST NOT define an artificial universal maximum.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Identical source must produce identical parse structure.
 *
 * This must remain true regardless of:
 *
 *     machine size
 *     target architecture
 *     number of processors
 *     memory capacity
 *     runtime availability
 *     network state
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Validation must reject or flag language-level introduction of:
 *
 *     MAX_MEMORIES
 *     MAX_MEMORY_SIZE
 *     MAX_HISTORY
 *     MAX_TIMELINES
 *     MAX_ANCESTORS
 *     MAX_KNOWLEDGE
 *     MAX_RECALLS
 *     MAX_LEARNING_STEPS
 *     MAX_INFERENCE_STEPS
 *     MAX_CONSENSUS_PARTICIPANTS
 *
 * and equivalent hidden limits.
 *
 * A numeric source value is NOT itself a violation.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar requires no Rust implementation details.
 *
 * The consuming implementation MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST use safe Rust only.
 *
 * No `unsafe` is required by this grammar.
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete as a grammar contract when:
 *
 * [x] Sankofa has one canonical grammar file.
 * [x] Existing remember syntax is preserved.
 * [x] Existing wisdom syntax is preserved.
 * [x] Recall has a defined source contract.
 * [x] Learn has a defined source contract.
 * [x] Infer has a defined source contract.
 * [x] Temporal attachment points are defined.
 * [x] Universal expressions are reused.
 * [x] Universal types are reused.
 * [x] Memory foundation is reused.
 * [x] No lexer rules are duplicated.
 * [x] No second AST authority is created.
 * [x] No second IR is created.
 * [x] No runtime behavior occurs during parsing.
 * [x] No physical hardware is selected.
 * [x] No fixed resource ceiling is imposed.
 * [x] No fixed timeline ceiling is imposed.
 * [x] No fixed history ceiling is imposed.
 * [x] No fixed knowledge ceiling is imposed.
 * [x] Provenance remains extensible.
 * [x] Consensus remains algorithm-independent.
 * [x] Learning remains implementation-independent.
 * [x] Inference remains implementation-independent.
 * [x] Quantum integration is downstream.
 * [x] Classical integration is downstream.
 * [x] AI integration is downstream.
 * [x] Distributed integration is downstream.
 * [x] HDL/hardware integration is downstream.
 * [x] Resource/capability analysis is downstream.
 * [x] Determinism is explicit.
 * [x] Diagnostics are explicit.
 * [x] Compatibility is explicit.
 * [x] Safe-Rust requirements are explicit.
 * [x] Integration ownership is explicit.
 *
 * ============================================================================
 * IMPORTANT IMPLEMENTATION STATUS
 * ============================================================================
 *
 * This grammar file being complete does NOT mean that every construct is
 * already implemented by the Rust frontend.
 *
 * At the repository revision audited for this file:
 *
 *     remember -> existing Rust AST/parser/semantic/IR support
 *     wisdom   -> existing Rust AST/parser support
 *     recall   -> lexical support exists; parser/AST integration remains
 *     learn    -> lexical support exists; parser/AST integration remains
 *     infer    -> lexical support exists; parser/AST integration remains
 *
 * grammar/grammar.md must report those implementation states independently.
 *
 * ============================================================================
 * CANONICAL SANKOFA GRAMMAR
 * ============================================================================
 */

parser grammar Sankofa;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Memory's canonical dispatcher should invoke:
 *
 *     sankofaConstruct
 *
 * This file does not create a program root.
 * ========================================================================== */

sankofaConstruct
    : sankofaStatement
    | sankofaExpression
    ;


/* ============================================================================
 * 2. STATEMENTS
 * ========================================================================== */

sankofaStatement
    : sankofaRemember
    | sankofaRecall
    | sankofaLearn
    | sankofaInfer
    | sankofaWisdom
    ;


/* ============================================================================
 * 3. EXPRESSIONS
 * ========================================================================== */

sankofaExpression
    : sankofaRecallExpression
    | sankofaLearnExpression
    | sankofaInferExpression
    ;


/* ============================================================================
 * 4. REMEMBER
 * ============================================================================
 *
 * Compatibility forms:
 *
 *     remember name = expression;
 *     remember name: Type = expression;
 *
 * The optional type is syntax only.
 *
 * Semantic validation belongs downstream.
 * ========================================================================== */

sankofaRemember
    : REMEMBER
      sankofaAssignmentTarget
      sankofaTypeAnnotation?
      ASSIGN
      expression
      SEMICOLON
    ;


sankofaAssignmentTarget
    : identifier
    ;


sankofaTypeAnnotation
    : COLON
      typeExpression
    ;


/* ============================================================================
 * 5. RECALL
 * ============================================================================
 *
 * Statement forms:
 *
 *     recall expression;
 *     recall expression from expression;
 *     recall expression with (...);
 *
 * The expression is deliberately generic.
 *
 * The semantic layer determines whether the expression denotes:
 *
 *     memory
 *     key
 *     query
 *     temporal reference
 *     knowledge
 *     another valid Sankofa source
 * ========================================================================== */

sankofaRecall
    : RECALL
      expression
      sankofaFromClause?
      sankofaWithClause?
      SEMICOLON
    ;


sankofaRecallExpression
    : RECALL
      expression
      sankofaFromClause?
      sankofaWithClause?
    ;


/* ============================================================================
 * 6. LEARN
 * ============================================================================
 *
 * Statement:
 *
 *     learn expression;
 *     learn expression from expression;
 *     learn expression with (...);
 *
 * Expression form:
 *
 *     learn expression
 * ========================================================================== */

sankofaLearn
    : LEARN
      expression
      sankofaFromClause?
      sankofaWithClause?
      SEMICOLON
    ;


sankofaLearnExpression
    : LEARN
      expression
      sankofaFromClause?
      sankofaWithClause?
    ;


/* ============================================================================
 * 7. INFER
 * ========================================================================== */

sankofaInfer
    : INFER
      expression
      sankofaFromClause?
      sankofaWithClause?
      SEMICOLON
    ;


sankofaInferExpression
    : INFER
      expression
      sankofaFromClause?
      sankofaWithClause?
    ;


/* ============================================================================
 * 8. WISDOM
 * ============================================================================
 *
 * Existing compatibility forms:
 *
 *     wisdom name;
 *     wisdom name = expression;
 *
 * The existing Rust parser represents the no-assignment form as a null
 * expression.
 *
 * Future AST evolution may represent the optional value explicitly.
 * ========================================================================== */

sankofaWisdom
    : WISDOM
      identifier
      (
          ASSIGN
          expression
      )?
      SEMICOLON?
    ;


/* ============================================================================
 * 9. FROM CLAUSE
 * ========================================================================== */

sankofaFromClause
    : FROM
      expression
    ;


/* ============================================================================
 * 10. WITH CLAUSE
 * ============================================================================
 *
 * Named arguments are represented using ordinary identifiers.
 *
 * There is no closed property list.
 * ========================================================================== */

sankofaWithClause
    : WITH
      LPAREN
      sankofaArgumentList?
      RPAREN
    ;


sankofaArgumentList
    : sankofaArgument
      (COMMA sankofaArgument)*
      COMMA?
    ;


sankofaArgument
    : sankofaNamedArgument
    | expression
    ;


sankofaNamedArgument
    : identifier
      ASSIGN
      expression
    ;


/* ============================================================================
 * 11. TEMPORAL QUALIFICATION
 * ============================================================================
 *
 * This is a reusable semantic attachment point.
 *
 * The exact temporal semantic model remains owned by the temporal type /
 * execution specifications.
 *
 * Examples conceptually:
 *
 *     zamani
 *     sasa
 *
 * are lexical vocabulary, but their semantic interpretation belongs downstream.
 *
 * ========================================================================== */

sankofaTemporalQualifier
    : ZAMANI
    | SASA
    | MTS
    ;


sankofaTemporalReference
    : sankofaTemporalQualifier
    | expression
    ;


/* ============================================================================
 * 12. SOURCE / TARGET
 * ============================================================================
 *
 * These rules deliberately remain expression-based.
 * ========================================================================== */

sankofaSource
    : expression
    ;


sankofaTarget
    : expression
    ;


/* ============================================================================
 * 13. OPEN EXTENSION OPERATION
 * ============================================================================
 *
 * Future Sankofa operations must not require modification of a closed
 * operation enumeration.
 *
 * A qualified name plus arguments is sufficient to identify an open semantic
 * operation.
 *
 * The operation's existence and meaning are semantic questions.
 * ========================================================================== */

sankofaExtensionOperation
    : qualifiedName
      LPAREN
      sankofaArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 14. EXTENSION ARGUMENTS
 * ============================================================================
 *
 * Kept separate so future domain extensions can attach:
 *
 *     provenance
 *     temporal state
 *     confidence
 *     resource requirements
 *     capabilities
 *     policy
 *
 * without changing the universal expression grammar.
 * ========================================================================== */

sankofaMetadata
    : AT
      qualifiedName
      (
          LPAREN
          sankofaArgumentList?
          RPAREN
      )?
    ;


/* ============================================================================
 * 15. EXTENSIBLE OPERATION BUNDLE
 * ============================================================================
 *
 * This rule is intended for composition by future Sankofa-specific semantic
 * grammars.
 *
 * It does not make the listed concepts a closed vocabulary.
 * ========================================================================== */

sankofaOperation
    : sankofaExtensionOperation
    ;


/* ============================================================================
 * 16. TEMPORAL MEMORY INTENT
 * ============================================================================
 *
 * Reusable attachment point for:
 *
 *     remember
 *     recall
 *     learn
 *     infer
 *     wisdom
 *
 * Temporal interpretation remains semantic.
 * ========================================================================== */

sankofaTemporalIntent
    : sankofaTemporalQualifier
    ;


/* ============================================================================
 * 17. FINAL INTEGRATION INVARIANT
 * ============================================================================
 *
 * The Sankofa grammar provides syntax only.
 *
 * The semantic pipeline remains:
 *
 *     Sankofa source
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     Sankofa semantic analysis
 *          |
 *          +--> memory semantics
 *          +--> temporal semantics
 *          +--> provenance semantics
 *          +--> knowledge semantics
 *          +--> learning semantics
 *          +--> inference semantics
 *          +--> consensus semantics
 *          +--> resource semantics
 *          +--> capability semantics
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     compiler/runtime/backend
 *
 * No parser rule in this file may bypass that pipeline.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */