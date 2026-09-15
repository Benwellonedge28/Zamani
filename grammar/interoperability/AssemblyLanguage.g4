/*
 * ============================================================================
 * Zamani Universal Assembly Language Interoperability Grammar
 * ============================================================================
 *
 * Canonical ANTLR source:
 *
 *     grammar/interoperability/AssemblyLanguage.g4
 *
 * Compatibility source-tree name:
 *
 *     grammar/interoperability/assembly-language.g4
 *
 * IMPORTANT:
 * ANTLR grammar identifiers cannot contain '-'.
 *
 * Architectural role
 * ------------------
 *
 * Zamani source
 *      |
 *      v
 * canonical semantic frontend
 *      |
 *      v
 * canonical IR
 *      |
 *      +-----------------------------+
 *      |                             |
 *      v                             v
 * architecture-independent      target lowering
 * representation                      |
 *                                      v
 *                              machine representation
 *                                      |
 *                                      v
 *                              assembly backend
 *
 * This grammar exists at the interoperability boundary for assembly
 * representations. It does NOT make assembly language the semantic source
 * language of Zamani.
 *
 * OWNERSHIP
 * ---------
 *
 * This grammar owns the syntax required to represent architecture-specific
 * assembly-like source and interchange representations, including:
 *
 *   - assembly translation units;
 *   - sections;
 *   - symbols;
 *   - labels;
 *   - directives;
 *   - instruction statements;
 *   - instruction mnemonics;
 *   - generic operands;
 *   - registers;
 *   - immediates;
 *   - symbolic references;
 *   - memory/address expressions;
 *   - relocation expressions;
 *   - decorators/modifiers;
 *   - operand lists;
 *   - comments;
 *   - architecture/dialect metadata;
 *   - assembler-level expressions;
 *   - alignment/size/visibility/linkage directives;
 *   - macro-like source declarations where represented by the interchange
 *     grammar.
 *
 * This grammar deliberately supports an OPEN instruction vocabulary.
 *
 * It does NOT own:
 *
 *   - Zamani language semantics;
 *   - canonical Zamani IR;
 *   - classical IR;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - optimization algorithms;
 *   - instruction scheduling;
 *   - register allocation;
 *   - instruction selection;
 *   - hardware discovery;
 *   - target discovery;
 *   - physical device discovery;
 *   - ABI semantics;
 *   - calling-convention semantics;
 *   - linker implementation;
 *   - loader implementation;
 *   - binary encoding;
 *   - opcode numbers;
 *   - fixed register counts;
 *   - fixed register names;
 *   - fixed instruction sets;
 *   - fixed address widths;
 *   - fixed word sizes;
 *   - fixed architecture topology.
 *
 * Those concerns belong to ABI, target, compiler, hardware, runtime, and
 * binary-format subsystems.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Assembly is a target representation, not the portable semantic contract.
 *
 * A Zamani program must be capable of:
 *
 *     source
 *       |
 *       v
 *     semantic IR
 *       |
 *       +--> target A assembly
 *       +--> target B assembly
 *       +--> target C machine representation
 *       +--> future target representation
 *
 * Therefore this grammar never assumes a particular processor.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No grammar-level finite maximum exists for:
 *
 *   - source lines;
 *   - labels;
 *   - symbols;
 *   - sections;
 *   - instructions;
 *   - operands;
 *   - registers;
 *   - macros;
 *   - directives;
 *   - relocation expressions;
 *   - data declarations.
 *
 * The grammar deliberately does not define:
 *
 *   MAX_REGISTERS
 *   MAX_INSTRUCTIONS
 *   MAX_SECTIONS
 *   MAX_SYMBOLS
 *   MAX_ADDRESS_BITS
 *   MAX_WORD_SIZE
 *   MAX_OPERANDS
 *
 * Target-specific limits belong to target descriptions and semantic
 * validation.
 *
 * ============================================================================
 * ARCHITECTURE INDEPENDENCE
 * ============================================================================
 *
 * Do not encode:
 *
 *   x86-only registers
 *   ARM-only registers
 *   RISC-V-only registers
 *   fixed opcode tables
 *   fixed register counts
 *   fixed address widths
 *   fixed endian assumptions
 *
 * A register is syntactically a symbolic operand.
 *
 * A mnemonic is syntactically an identifier.
 *
 * Their actual meaning is supplied by the selected assembly dialect/target.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing is side-effect free.
 *
 * This grammar performs no:
 *
 *   - file access;
 *   - network access;
 *   - process execution;
 *   - assembler invocation;
 *   - linker invocation;
 *   - device access;
 *   - hardware discovery;
 *   - dynamic library loading.
 *
 * ============================================================================
 * RUST
 * ============================================================================
 *
 * Target repository:
 *
 *   Rust 1.97 / 1.97.1
 *   Rust 2021
 *   antlr-rust 0.3.0-beta
 *
 * No embedded Rust actions are used.
 * No semantic predicates are used.
 * No unsafe code is used.
 *
 * ============================================================================
 */

grammar AssemblyLanguage;

options {
    language = Rust;
}


/* ============================================================================
 * TRANSLATION UNIT
 * ========================================================================== */

assemblyUnit
    : assemblyItem* EOF
    ;


assemblyItem
    : dialectDirective
    | architectureDirective
    | sectionDeclaration
    | symbolDeclaration
    | labelDeclaration
    | directive
    | instruction
    | macroDeclaration
    | macroInvocation
    | emptyStatement
    ;


/* ============================================================================
 * DIALECT / TARGET METADATA
 * ========================================================================== */

/*
 * These declarations identify an external representation.
 *
 * They do NOT cause the parser to select a fixed machine architecture.
 *
 * Example:
 *
 *   .dialect "vendor.example.asm"
 *   .architecture "some.target"
 *
 * The semantic layer decides whether the declared dialect is supported.
 */

dialectDirective
    : DIALECT stringLiteral statementTerminator
    ;


architectureDirective
    : ARCHITECTURE stringLiteral statementTerminator
    ;


/* ============================================================================
 * SECTIONS
 * ========================================================================== */

sectionDeclaration
    : sectionHeader sectionAttribute* statementTerminator?
      sectionItem*
      sectionEnd?
    ;


sectionHeader
    : SECTION qualifiedName
    ;


sectionAttribute
    : identifier
    | stringLiteral
    | numericLiteral
    ;


sectionEnd
    : ENDSECTION
    ;


sectionItem
    : directive
    | symbolDeclaration
    | labelDeclaration
    | instruction
    | dataStatement
    | emptyStatement
    ;


/* ============================================================================
 * SYMBOLS
 * ========================================================================== */

symbolDeclaration
    : symbolVisibility?
      symbolBinding?
      symbolKind?
      identifier
      symbolSize?
      statementTerminator
    ;


symbolVisibility
    : GLOBAL
    | LOCAL
    | WEAK
    | HIDDEN
    | PROTECTED
    | INTERNAL
    ;


symbolBinding
    : BINDING
    ;


symbolKind
    : TYPE
    | OBJECT
    | FUNCTION
    | SECTION_KIND
    ;


symbolSize
    : SIZE expression
    ;


/* ============================================================================
 * LABELS
 * ========================================================================== */

labelDeclaration
    : labelName COLON
    ;


labelName
    : identifier
    ;


/* ============================================================================
 * INSTRUCTIONS
 * ========================================================================== */

/*
 * Instruction mnemonics intentionally have an open vocabulary.
 *
 * This is essential for POCO-REAF.
 *
 * The grammar must accept future instruction sets without changing the
 * language grammar merely because a processor vendor introduces a mnemonic.
 */

instruction
    : instructionPrefix*
      instructionMnemonic
      operandList?
      statementTerminator?
    ;


instructionPrefix
    : identifier
    ;


instructionMnemonic
    : identifier
    ;


/* ============================================================================
 * OPERANDS
 * ========================================================================== */

operandList
    : operand
      (COMMA operand)*
    ;


operand
    : registerOperand
    | immediateOperand
    | memoryOperand
    | symbolOperand
    | labelOperand
    | relocationOperand
    | expressionOperand
    | operandDecorator operand
    ;


operandDecorator
    : identifier
    | AT
    | HASH
    | PERCENT
    | DOLLAR
    ;


registerOperand
    : REGISTER_PREFIX? identifier
    ;


immediateOperand
    : IMMEDIATE_PREFIX expression
    ;


symbolOperand
    : qualifiedName
    ;


labelOperand
    : identifier
    ;


expressionOperand
    : expression
    ;


/* ============================================================================
 * MEMORY / ADDRESS OPERANDS
 * ========================================================================== */

/*
 * Memory syntax is deliberately generic.
 *
 * Examples that can be represented include forms conceptually equivalent to:
 *
 *   [base]
 *   [base + offset]
 *   [base + index * scale + displacement]
 *   symbol + displacement
 *   @symbol
 *
 * Exact architectural interpretation is downstream.
 */

memoryOperand
    : memoryPrefix*
      LBRACK
      memoryExpression
      RBRACK
    ;


memoryPrefix
    : identifier
    | SEGMENT_PREFIX
    ;


memoryExpression
    : memoryTerm
      (memoryOperator memoryTerm)*
    ;


memoryOperator
    : PLUS
    | MINUS
    | STAR
    ;


memoryTerm
    : registerOperand
    | numericLiteral
    | symbolOperand
    | expression
    ;


/* ============================================================================
 * RELOCATIONS
 * ========================================================================== */

relocationOperand
    : relocationModifier LPAREN expression RPAREN
    ;


relocationModifier
    : identifier
    ;


relocationExpression
    : relocationModifier LPAREN expression RPAREN
    ;


/* ============================================================================
 * DIRECTIVES
 * ========================================================================== */

directive
    : directiveName
      directiveArguments?
      statementTerminator?
    ;


directiveName
    : DIRECTIVE_PREFIX identifier
    | DOT identifier
    ;


directiveArguments
    : directiveArgument
      (COMMA directiveArgument)*
    ;


directiveArgument
    : stringLiteral
    | characterLiteral
    | numericLiteral
    | qualifiedName
    | expression
    | relocationExpression
    | bracketedDirectiveArgument
    ;


bracketedDirectiveArgument
    : LBRACK
      directiveArgument*
      RBRACK
    ;


/* ============================================================================
 * DATA DECLARATIONS
 * ========================================================================== */

dataStatement
    : dataDirective
      dataElementList?
      statementTerminator?
    ;


dataDirective
    : BYTE_DIRECTIVE
    | WORD_DIRECTIVE
    | DWORD_DIRECTIVE
    | QWORD_DIRECTIVE
    | OCTA_DIRECTIVE
    | FLOAT_DIRECTIVE
    | DOUBLE_DIRECTIVE
    | STRING_DIRECTIVE
    | ZERO_DIRECTIVE
    ;


dataElementList
    : dataElement
      (COMMA dataElement)*
    ;


dataElement
    : stringLiteral
    | characterLiteral
    | numericLiteral
    | expression
    ;


/* ============================================================================
 * MACROS
 * ========================================================================== */

/*
 * Macro syntax remains intentionally generic.
 *
 * Macro expansion is NOT performed by this grammar.
 */

macroDeclaration
    : MACRO identifier macroParameterList?
      statementTerminator?
      macroBody
      ENDM
    ;


macroParameterList
    : LPAREN
      macroParameter*
      RPAREN
    ;


macroParameter
    : identifier
    ;


macroBody
    : assemblyItem*
    ;


macroInvocation
    : identifier
      EXCLAMATION
      argumentList?
      statementTerminator?
    ;


/* ============================================================================
 * EXPRESSIONS
 * ============================================================================
 *
 * This is a deliberately small assembler-expression grammar.
 *
 * It must not replace Zamani's canonical expression grammar.
 *
 * It exists because assembly representations frequently need expressions for:
 *
 *   - offsets;
 *   - constants;
 *   - symbol arithmetic;
 *   - relocation expressions;
 *   - section sizes;
 *   - alignment;
 *   - data values.
 *
 * Semantic interpretation is delegated downstream.
 */

expression
    : logicalExpression
    ;


logicalExpression
    : comparisonExpression
      (logicalOperator comparisonExpression)*
    ;


logicalOperator
    : OR
    | XOR
    | AND
    ;


comparisonExpression
    : additiveExpression
      (comparisonOperator additiveExpression)*
    ;


comparisonOperator
    : EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


additiveExpression
    : multiplicativeExpression
      ((PLUS | MINUS) multiplicativeExpression)*
    ;


multiplicativeExpression
    : unaryExpression
      ((STAR | SLASH | PERCENT) unaryExpression)*
    ;


unaryExpression
    : unaryOperator* primaryExpression
    ;


unaryOperator
    : PLUS
    | MINUS
    | TILDE
    | EXCLAMATION
    ;


primaryExpression
    : numericLiteral
    | characterLiteral
    | symbolOperand
    | LPAREN expression RPAREN
    ;


/* ============================================================================
 * NAMES
 * ========================================================================== */

qualifiedName
    : identifier
      (NAMESPACE_SEPARATOR identifier)*
    ;


identifier
    : IDENTIFIER
    | ESCAPED_IDENTIFIER
    ;


/* ============================================================================
 * LITERALS
 * ========================================================================== */

numericLiteral
    : INTEGER_LITERAL
    | HEX_LITERAL
    | OCTAL_LITERAL
    | BINARY_LITERAL
    | FLOAT_LITERAL
    ;


stringLiteral
    : STRING_LITERAL
    ;


characterLiteral
    : CHARACTER_LITERAL
    ;


/* ============================================================================
 * STATEMENT TERMINATION
 * ========================================================================== */

statementTerminator
    : SEMICOLON
    ;


/* ============================================================================
 * EMPTY STATEMENTS
 * ========================================================================== */

emptyStatement
    : SEMICOLON
    ;


/* ============================================================================
 * LEXER
 * ========================================================================== */

/*
 * KEYWORDS
 *
 * Only structural assembly concepts are reserved.
 *
 * Instruction mnemonics and register names are NOT reserved keywords.
 * This prevents the grammar from embedding a finite ISA vocabulary.
 */

DIALECT
    : '.dialect'
    ;

ARCHITECTURE
    : '.architecture'
    ;

SECTION
    : '.section'
    ;

ENDSECTION
    : '.endsection'
    ;

GLOBAL
    : '.global'
    ;

LOCAL
    : '.local'
    ;

WEAK
    : '.weak'
    ;

HIDDEN
    : '.hidden'
    ;

PROTECTED
    : '.protected'
    ;

INTERNAL
    : '.internal'
    ;

BINDING
    : '.binding'
    ;

TYPE
    : '.type'
    ;

OBJECT
    : '.object'
    ;

FUNCTION
    : '.function'
    ;

SECTION_KIND
    : '.section_kind'
    ;

SIZE
    : '.size'
    ;

MACRO
    : '.macro'
    ;

ENDM
    : '.endm'
    ;

BYTE_DIRECTIVE
    : '.byte'
    ;

WORD_DIRECTIVE
    : '.word'
    ;

DWORD_DIRECTIVE
    : '.dword'
    ;

QWORD_DIRECTIVE
    : '.qword'
    ;

OCTA_DIRECTIVE
    : '.octa'
    ;

FLOAT_DIRECTIVE
    : '.float'
    ;

DOUBLE_DIRECTIVE
    : '.double'
    ;

STRING_DIRECTIVE
    : '.string'
    ;

ZERO_DIRECTIVE
    : '.zero'
    ;

REGISTER_PREFIX
    : '%'
    ;

IMMEDIATE_PREFIX
    : '$'
    ;

DIRECTIVE_PREFIX
    : '.'
    ;

SEGMENT_PREFIX
    : '@'
    ;

NAMESPACE_SEPARATOR
    : '::'
    ;


/* ============================================================================
 * PUNCTUATION / OPERATORS
 * ========================================================================== */

LPAREN
    : '('
    ;

RPAREN
    : ')'
    ;

LBRACK
    : '['
    ;

RBRACK
    : ']'
    ;

LBRACE
    : '{'
    ;

RBRACE
    : '}'
    ;

COMMA
    : ','
    ;

COLON
    : ':'
    ;

SEMICOLON
    : ';'
    ;

DOT
    : '.'
    ;

AT
    : '@'
    ;

HASH
    : '#'
    ;

DOLLAR
    : '$'
    ;

PLUS
    : '+'
    ;

MINUS
    : '-'
    ;

STAR
    : '*'
    ;

SLASH
    : '/'
    ;

PERCENT
    : '%'
    ;

TILDE
    : '~'
    ;

EXCLAMATION
    : '!'
    ;

OR
    : '|'
    ;

XOR
    : '^'
    ;

AND
    : '&'
    ;

EQUAL
    : '=='
    ;

NOT_EQUAL
    : '!='
    ;

LESS_EQUAL
    : '<='
    ;

GREATER_EQUAL
    : '>='
    ;

LESS
    : '<'
    ;

GREATER
    : '>'
    ;


/* ============================================================================
 * IDENTIFIERS
 * ========================================================================== */

ESCAPED_IDENTIFIER
    : '`'
      (~[`\\] | '\\' .)+
      '`'
    ;


IDENTIFIER
    : [a-zA-Z_]
      [a-zA-Z0-9_$]*
    ;


/* ============================================================================
 * NUMERIC LITERALS
 * ========================================================================== */

HEX_LITERAL
    : '0' [xX] [0-9a-fA-F]+
    ;


BINARY_LITERAL
    : '0' [bB] [01]+
    ;


OCTAL_LITERAL
    : '0' [oO] [0-7]+
    ;


FLOAT_LITERAL
    : [0-9]+
      '.'
      [0-9]+
      ([eE] [+-]? [0-9]+)?
    ;


INTEGER_LITERAL
    : [0-9]+
    ;


/* ============================================================================
 * STRING / CHARACTER LITERALS
 * ========================================================================== */

STRING_LITERAL
    : '"'
      (ESCAPE_SEQUENCE | ~["\\\r\n])*
      '"'
    ;


CHARACTER_LITERAL
    : '\''
      (ESCAPE_SEQUENCE | ~['\\\r\n])
      '\''
    ;


fragment ESCAPE_SEQUENCE
    : '\\'
      (
          'n'
        | 'r'
        | 't'
        | 'b'
        | 'f'
        | '0'
        | '\\'
        | '\''
        | '"'
        | 'x' HEX_DIGIT HEX_DIGIT
        | 'u' HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT
      )
    ;


fragment HEX_DIGIT
    : [0-9a-fA-F]
    ;


/* ============================================================================
 * COMMENTS
 * ========================================================================== */

LINE_COMMENT
    : '//' ~[\r\n]* -> channel(HIDDEN)
    ;


HASH_COMMENT
    : '#' ~[\r\n]* -> channel(HIDDEN)
    ;


BLOCK_COMMENT
    : '/*'
      .*?
      '*/'
      -> channel(HIDDEN)
    ;


/* ============================================================================
 * WHITESPACE
 * ========================================================================== */

WS
    : [ \t\r\n\f]+ -> channel(HIDDEN)
    ;