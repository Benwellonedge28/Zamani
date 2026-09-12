/**
 * Zamani — Hardware Literal Lexer Component
 *
 * File:
 *   grammar/lexer/hardware-literals.g4
 *
 * Purpose:
 *   Defines lexical forms that are intrinsically hardware-oriented while
 *   remaining independent of any particular machine, vendor, architecture,
 *   device identifier, topology, resource count, or implementation.
 *
 * Architectural ownership:
 *   - Owns hardware-specific literal syntax.
 *   - Does NOT own hardware semantics.
 *   - Does NOT discover hardware.
 *   - Does NOT encode physical machine limits.
 *   - Does NOT define hardware types.
 *   - Does NOT define hardware modules, ports, signals, clocks, timing,
 *     targets, resources, capabilities, placement, or deployment semantics.
 *
 * Integration model:
 *   This file is intended to be composed into the canonical Zamani lexer.
 *   It must not become a second lexer authority.
 *
 * Important:
 *   - No embedded target-specific Rust code.
 *   - No semantic predicates.
 *   - No actions.
 *   - No filesystem/network access.
 *   - No fixed hardware limits.
 *   - No fixed device counts.
 *   - No fixed address widths.
 *   - No fixed register widths.
 *   - No fixed topology sizes.
 *
 * Rust:
 *   The generated lexer is consumed by the Zamani Rust frontend.
 *   This grammar itself contains no Rust code and requires no unsafe code.
 *
 * Scalability:
 *   Repetition operators intentionally have no artificial upper bound.
 *   Actual resource limits are enforced by semantic analysis, compilation,
 *   resource management, scheduling, or the target/runtime layers.
 *
 * --------------------------------------------------------------------------
 * LITERAL OWNERSHIP
 * --------------------------------------------------------------------------
 *
 * Hardware literals are restricted to lexical values whose meaning is
 * hardware-oriented but whose magnitude is not tied to a particular machine.
 *
 * Examples:
 *
 *   128bit
 *   64bit
 *   1024bit
 *
 *   4byte
 *   64byte
 *   1KiB
 *   16MiB
 *   2GiB
 *
 *   10ns
 *   250ps
 *   1us
 *
 * NOTE:
 *   Numeric, size, and duration literal grammars may own generic forms.
 *   The canonical lexer must establish exactly one owner for overlapping
 *   lexical forms. This file therefore uses a hardware-specific prefix for
 *   forms that must be distinguishable from generic quantities.
 *
 * Canonical hardware quantity forms:
 *
 *   hw.width(...)
 *   hw.addr(...)
 *   hw.align(...)
 *   hw.bank(...)
 *   hw.lane(...)
 *   hw.port(...)
 *
 * are intentionally NOT lexed here as single literals. They are identifiers,
 * punctuation, and expressions handled by parser/semantic layers.
 *
 * This avoids making hardware syntax inseparable from the language grammar.
 */


/*
 * ==========================================================================
 * HARDWARE WIDTH LITERALS
 * ==========================================================================
 *
 * Represents a width/precision quantity without specifying which hardware
 * resource owns it.
 *
 * Valid examples:
 *
 *   8bit
 *   32bit
 *   64bit
 *   128bit
 *   1024bit
 *
 * There is deliberately no MAX_WIDTH.
 *
 * The semantic layer decides whether a particular target can support the
 * requested width.
 *
 * The suffix is deliberately lexical and case-sensitive.
 */
HARDWARE_WIDTH_LITERAL
    : DECIMAL_INTEGER 'bit'
    ;


/*
 * ==========================================================================
 * HARDWARE BYTE-WIDTH LITERALS
 * ==========================================================================
 *
 * Represents a byte-oriented hardware quantity.
 *
 * Examples:
 *
 *   1byte
 *   8byte
 *   64byte
 *
 * This is not a memory-capacity declaration. It is only a lexical quantity.
 *
 * Large values remain syntactically valid and are checked later against
 * language/resource/target constraints.
 */
HARDWARE_BYTE_WIDTH_LITERAL
    : DECIMAL_INTEGER 'byte'
    ;


/*
 * ==========================================================================
 * HARDWARE ALIGNMENT LITERALS
 * ==========================================================================
 *
 * Alignment is represented as a quantity rather than a fixed machine
 * property.
 *
 * Examples:
 *
 *   4align
 *   16align
 *   64align
 *
 * Whether a particular alignment is legal is a semantic/target question.
 */
HARDWARE_ALIGNMENT_LITERAL
    : DECIMAL_INTEGER 'align'
    ;


/*
 * ==========================================================================
 * HARDWARE ADDRESS LITERALS
 * ==========================================================================
 *
 * Hardware addresses are represented lexically without assuming a fixed
 * address width.
 *
 * Canonical syntax:
 *
 *   @0x1000
 *   @0x100000000
 *
 * The '@' prefix provides an explicit hardware-address lexical namespace.
 *
 * IMPORTANT:
 *   A source-level address is inherently target-specific. Therefore this
 *   literal MUST NOT be interpreted as a universally portable resource
 *   identity.
 *
 * POCO-REAF:
 *   Portable programs should normally use symbolic resources, capabilities,
 *   bindings, or target-independent references rather than raw addresses.
 *
 * Semantic analysis must therefore distinguish:
 *
 *   source-level hardware address
 *
 * from:
 *
 *   portable resource identity.
 *
 * No maximum address width is encoded here.
 */
HARDWARE_ADDRESS_LITERAL
    : '@' HEX_INTEGER
    ;


/*
 * ==========================================================================
 * HARDWARE REGISTER INDEX LITERALS
 * ==========================================================================
 *
 * Register indexes are numeric quantities.
 *
 * This rule exists only for an explicitly hardware-qualified lexical form.
 *
 * Example:
 *
 *   %0
 *   %7
 *   %1024
 *
 * The grammar imposes no maximum register index.
 *
 * NOTE:
 *   The semantic layer must determine whether a target actually exposes
 *   such a register. The lexer must never encode a register-count limit.
 */
HARDWARE_REGISTER_INDEX_LITERAL
    : '%' DECIMAL_INTEGER
    ;


/*
 * ==========================================================================
 * HARDWARE BANK INDEX LITERALS
 * ==========================================================================
 *
 * Represents a hardware bank/index quantity without declaring that a target
 * has any particular number of banks.
 *
 * Examples:
 *
 *   #0
 *   #1
 *   #1024
 *
 * The '#' namespace is reserved here specifically for an explicitly
 * hardware-qualified bank/index literal.
 *
 * If the canonical Zamani language already assigns '#' another lexical
 * meaning, the canonical lexer must resolve the conflict by assigning this
 * rule a different explicit hardware namespace rather than introducing
 * semantic predicates.
 */
HARDWARE_BANK_INDEX_LITERAL
    : '#' DECIMAL_INTEGER
    ;


/*
 * ==========================================================================
 * HARDWARE LANE INDEX LITERALS
 * ==========================================================================
 *
 * Represents an explicitly qualified hardware lane/index.
 *
 * Canonical form:
 *
 *   lane[0]
 *   lane[7]
 *   lane[1024]
 *
 * This rule intentionally does NOT lex the complete construct as one token.
 *
 * The parser should instead compose:
 *
 *   identifier + '[' + integer + ']'
 *
 * so that lane names remain extensible and the language does not hard-code
 * a particular lane namespace.
 *
 * Therefore no lexer token is emitted here.
 */


/*
 * ==========================================================================
 * HARDWARE RESOURCE IDENTIFIERS
 * ==========================================================================
 *
 * Resource identifiers are NOT literals owned by this file.
 *
 * Examples such as:
 *
 *   cpu0
 *   gpu0
 *   fpga0
 *   qpu0
 *   device42
 *
 * must remain ordinary identifiers unless the semantic layer explicitly
 * classifies them as hardware resources.
 *
 * This is intentional.
 *
 * Hard-coding resource names into the lexer would violate POCO-REAF.
 *
 * The lexer must never contain rules such as:
 *
 *   CPU0
 *   GPU0
 *   QPU0
 *   FPGA0
 *
 * or fixed resource-count assumptions.
 */


/*
 * ==========================================================================
 * HEXADECIMAL INTEGER FRAGMENT
 * ==========================================================================
 *
 * Internal fragment used by HARDWARE_ADDRESS_LITERAL.
 *
 * This fragment intentionally has no maximum width.
 *
 * A resource/target layer can reject an address that is invalid for a
 * particular target without changing the language grammar.
 */
fragment HEX_INTEGER
    : '0' [xX] HEX_DIGIT+
    ;


/*
 * ==========================================================================
 * DECIMAL INTEGER FRAGMENT
 * ==========================================================================
 *
 * Internal decimal quantity.
 *
 * This fragment is intentionally unbounded by the grammar.
 *
 * Generic integer literals should normally be owned by
 *
 *   grammar/lexer/numeric-literals.g4
 *
 * if that file exists in the canonical lexer architecture.
 *
 * If the canonical lexer already exports a shared integer token, replace
 * this fragment with that shared lexical contract rather than maintaining
 * two independent numeric grammars.
 */
fragment DECIMAL_INTEGER
    : DECIMAL_DIGIT+
    ;


/*
 * ==========================================================================
 * DIGIT FRAGMENTS
 * ==========================================================================
 */

fragment DECIMAL_DIGIT
    : [0-9]
    ;

fragment HEX_DIGIT
    : [0-9a-fA-F]
    ;


/*
 * ==========================================================================
 * INTEGRATION CONTRACT
 * ==========================================================================
 *
 * Canonical ownership:
 *
 *   ZamaniLexer
 *       |
 *       +-- generic tokens
 *       +-- keywords
 *       +-- identifiers
 *       +-- generic literals
 *       +-- hardware literals  <-- this component
 *       +-- quantum literals
 *       +-- other domain tokens
 *
 * This file must NOT become an independently competing lexer authority.
 *
 * The canonical lexer must ensure that:
 *
 *   1. Each lexical sequence has one intended token owner.
 *   2. Generic numeric rules do not accidentally consume a hardware literal
 *      before hardware-specific rules can recognize it.
 *   3. Hardware-specific rules do not steal ordinary language syntax.
 *   4. Token names remain stable once published.
 *   5. Parser grammar references the canonical token vocabulary.
 *
 *
 * ==========================================================================
 * NON-OWNERSHIP CONTRACT
 * ==========================================================================
 *
 * The following concepts belong elsewhere:
 *
 *   Hardware type             -> grammar/hardware/
 *   Hardware module           -> grammar/hdl/
 *   Port                      -> grammar/hdl/
 *   Signal                    -> grammar/hdl/
 *   Clock                     -> grammar/hdl/
 *   Timing semantics          -> grammar/hdl/ + semantic layer
 *   Target                    -> grammar/hardware/ or grammar/compile/
 *   Resource                  -> grammar/resources/
 *   Capability                -> grammar/core/ or grammar/resources/
 *   Constraint                -> grammar/core/ or grammar/resources/
 *   Placement                 -> grammar/hardware/ or grammar/execution/
 *   Deployment                -> grammar/execution/
 *   Device discovery          -> runtime/hardware layer
 *   Calibration               -> hardware subsystem
 *   Topology                  -> hardware subsystem
 *   Scheduling                -> scheduling subsystem
 *   Optimization              -> optimization subsystem
 *   Quantum semantics         -> quantum semantic layer / quantum::ir
 *
 *
 * ==========================================================================
 * POCO-REAF CONTRACT
 * ==========================================================================
 *
 * These literals describe quantities, not guaranteed machine properties.
 *
 * For example:
 *
 *   1024bit
 *
 * means that the source expresses a 1024-bit quantity.
 *
 * It does NOT mean:
 *
 *   the machine has 1024-bit registers
 *   the machine has 1024-bit ALUs
 *   the machine has a 1024-bit bus
 *   the machine has a 1024-bit physical resource
 *
 * Those conclusions belong to semantic analysis and target lowering.
 *
 *
 * ==========================================================================
 * SCALABILITY CONTRACT
 * ==========================================================================
 *
 * There are no grammar-level limits for:
 *
 *   address width
 *   register index
 *   bank index
 *   quantity magnitude
 *   number of hardware resources
 *   number of devices
 *   number of cores
 *   number of lanes
 *   number of banks
 *   number of accelerators
 *
 * Repetition is intentionally represented with '+' rather than a bounded
 * repetition such as '{1,64}'.
 *
 * Actual limits may be imposed by:
 *
 *   parser implementation resource limits
 *   integer representation
 *   semantic validation
 *   compiler limits
 *   target capabilities
 *   resource availability
 *   runtime policy
 *
 * Those are not language-grammar limits.
 *
 *
 * ==========================================================================
 * DETERMINISM CONTRACT
 * ==========================================================================
 *
 * This grammar contains:
 *
 *   no semantic predicates
 *   no target queries
 *   no runtime queries
 *   no filesystem access
 *   no network access
 *   no random behavior
 *   no time-dependent behavior
 *
 * Therefore lexical recognition is deterministic.
 *
 *
 * ==========================================================================
 * ERROR CONTRACT
 * ==========================================================================
 *
 * Malformed hardware literals must be diagnosed by the canonical lexer and
 * diagnostic infrastructure.
 *
 * Examples:
 *
 *   @
 *   @0x
 *   % 
 *   #
 *
 * must not be silently converted into hardware semantics.
 *
 * Invalid target-specific quantities must be rejected later by semantic
 * validation rather than encoded as arbitrary lexer limits.
 *
 *
 * ==========================================================================
 * COMPATIBILITY CONTRACT
 * ==========================================================================
 *
 * Once these token names are consumed by parser/AST tooling, renaming them
 * is a language/tooling compatibility change.
 *
 * Changes require:
 *
 *   grammar/specification/language-version.md
 *   grammar/compatibility/
 *   parser/AST migration
 *   lexer regression tests
 *   documentation updates
 *
 * Existing valid generic numeric syntax must remain valid unless the language
 * specification explicitly changes it.
 *
 *
 * ==========================================================================
 * SECURITY CONTRACT
 * ==========================================================================
 *
 * Hardware address literals can expose target-specific implementation
 * details.
 *
 * They therefore must not automatically imply:
 *
 *   privileged access
 *   memory access
 *   device access
 *   DMA
 *   MMIO
 *   register access
 *   arbitrary execution
 *
 * Authorization and capability checking belong to semantic/compiler/runtime
 * layers.
 *
 *
 * ==========================================================================
 * RUST CONTRACT
 * ==========================================================================
 *
 * This grammar uses no embedded Rust.
 *
 * Generated Rust code must be built under the repository's Rust toolchain
 * policy (Rust 1.97 / 1.97.1 as specified by the project).
 *
 * No handwritten unsafe code is introduced by this grammar.
 *
 *
 * ==========================================================================
 * TEST CONTRACT
 * ==========================================================================
 *
 * Required lexical tests:
 *
 * Positive:
 *
 *   1bit
 *   8bit
 *   32bit
 *   128bit
 *   1024bit
 *
 *   1byte
 *   8byte
 *   64byte
 *
 *   4align
 *   16align
 *   4096align
 *
 *   @0x0
 *   @0x1
 *   @0x1000
 *   @0xFFFFFFFF
 *   @0xFFFFFFFFFFFFFFFFFFFFFFFF
 *
 *   %0
 *   %1
 *   %1024
 *
 *   #0
 *   #1
 *   #1024
 *
 * Negative:
 *
 *   @
 *   @0x
 *   @0y10
 *   % 
 *   #
 *   1BIT
 *   1BYTE
 *   1ALIGN
 *
 * Boundary/scalability:
 *
 *   Very large decimal quantity.
 *   Very large hexadecimal address.
 *   Very large register index.
 *   Very large bank index.
 *
 * Cross-domain:
 *
 *   hardware quantity + classical expression
 *   hardware quantity + quantum expression
 *   hardware quantity + HDL construct
 *   hardware quantity + resource requirement
 *   hardware quantity + compile target
 *
 * Determinism:
 *
 *   identical source -> identical token sequence
 *
 * Compatibility:
 *
 *   all previously valid non-hardware literals continue to tokenize under
 *   their canonical owners.
 *
 *
 * ==========================================================================
 * COMPLETION CRITERIA
 * ==========================================================================
 *
 * This file is complete only when:
 *
 *   [ ] Its token ownership is recorded in lexer/tokens.g4 or the canonical
 *       lexer specification.
 *
 *   [ ] It does not duplicate generic numeric literal ownership.
 *
 *   [ ] It does not duplicate identifiers.
 *
 *   [ ] It does not encode hardware counts.
 *
 *   [ ] It does not encode hardware topology.
 *
 *   [ ] It does not encode vendor/device identities.
 *
 *   [ ] It does not encode target-specific capabilities.
 *
 *   [ ] It contains no embedded Rust.
 *
 *   [ ] It contains no unsafe code.
 *
 *   [ ] It is deterministic.
 *
 *   [ ] Its token names are stable.
 *
 *   [ ] Parser integration is defined before parser implementation.
 *
 *   [ ] AST integration is defined before AST implementation.
 *
 *   [ ] Semantic validation distinguishes portable quantities from
 *       target-specific requirements.
 *
 *   [ ] Resource/capability layers own actual hardware limits.
 *
 *   [ ] Hardware address literals cannot silently grant hardware access.
 *
 *   [ ] Positive tests exist.
 *
 *   [ ] Negative tests exist.
 *
 *   [ ] Boundary tests exist.
 *
 *   [ ] Scalability tests exist.
 *
 *   [ ] Cross-domain tests exist.
 *
 *   [ ] Determinism tests exist.
 *
 *   [ ] Documentation describes the syntax.
 *
 *   [ ] No downstream file requires this grammar to be reopened merely
 *       because another domain grammar is subsequently implemented.
 */