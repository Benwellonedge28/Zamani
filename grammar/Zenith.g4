/*
 * Zenith.g4 — ANTLR4 combined grammar for the Zenith language.
 *
 * This is a formal, machine-checkable grammar derived directly from the
 * reference implementation (src/lexer.rs, src/parser.rs, src/ast/mod.rs)
 * and from GRAMMAR.md, which documents exactly what the real ZUTC
 * compiler accepts today. It is intended for tooling that wants a
 * standard ANTLR4 parser/lexer for Zenith (IDE plugins, syntax
 * highlighters, linters, alternative front-ends, etc.) without having to
 * reimplement the hand-written Rust recursive-descent parser.
 *
 * Status notes (kept in sync with GRAMMAR.md §1.4 and §6):
 *   - MTS_LITERAL is declared as a token kind in the reference lexer but
 *     not yet produced by it (`mts` currently lexes as a plain
 *     identifier). It is included here for forward compatibility but
 *     should be treated as reserved/planned.
 *   - `match` arms currently only accept an Expression as the pattern
 *     (no structural tuple/struct/enum destructuring yet). The `pattern`
 *     rule below is deliberately an alias of `expression` to mirror that.
 *   - Postfix `T?` optional-type sugar is not yet exposed by the
 *     reference parser; use `Optional<T>` / `Result<T, E>` generic forms.
 *
 * Build with (requires the ANTLR4 tool + your target's runtime, e.g.):
 *   antlr4 -Dlanguage=Python3 -visitor -o generated grammar/Zenith.g4
 *   antlr4 -Dlanguage=Java    -visitor -o generated grammar/Zenith.g4
 */

grammar Zenith;

// ═══════════════════════════════════════════════════════════════════════
// Parser rules
// ═══════════════════════════════════════════════════════════════════════

program
    : statement* EOF
    ;

// A plain identifier, OR one of the reserved keywords that the reference
// parser (src/parser.rs) empirically still accepts wherever an identifier
// is expected — e.g. path segments (parse_use reads `.literal` regardless
// of token type) and primary expressions (`this`, `self`, and the
// primitive/domain keywords all surface as plain Identifier AST nodes).
identLike
    : IDENT
    | THIS | SELF_LOWER | SELF_UPPER
    | INT_KW | FLOAT_KW | BOOL_KW | STR_KW | STRING_KW | CHAR_KW | VOID
    | QUANTUM | NANO | AGENT | CIRCUIT | EFFECT | HANDLE | REMEMBER | RECALL
    | LEARN | INFER | WISDOM | ZAMANI | SASA | ANCESTOR | LINEAR | AFFINE
    | LANGUAGE | MTS_KW | LEN | PRINT | PRINTLN | ASSERT | PANIC
    ;

// ─── Statements ───────────────────────────────────────────────────────────

statement
    : attribute? statementInner
    ;

attribute
    : '#' '[' .*? ']'
    ;

statementInner
    : letStmt
    | constStmt
    | functionDecl
    | returnStmt
    | breakStmt
    | continueStmt
    | whileStmt
    | forStmt
    | matchStmt
    | structDecl
    | enumDecl
    | traitDecl
    | implBlock
    | classDecl
    | interfaceDecl
    | moduleDecl
    | importStmt
    | useStmt
    | quantumCircuitDecl
    | nanoAgentDecl
    | sankofaRememberStmt
    | effectDecl
    | handleStmt
    | typeAliasDecl
    | unsafeBlock
    | wisdomStmt
    | languageDecl
    | expressionStmt
    ;

letStmt
    : ('let' | 'var') 'mut'? IDENT (':' typeExpr)? '=' expression ';'?
    ;

constStmt
    : 'const' IDENT (':' typeExpr)? '=' expression ';'?
    ;

returnStmt
    : 'return' expression? ';'?
    ;

breakStmt
    : 'break' ';'?
    ;

continueStmt
    : 'continue' ';'?
    ;

whileStmt
    : 'while' expression blockExpr
    ;

forStmt
    : 'for' IDENT 'in' expression blockExpr
    ;

matchStmt
    : matchExpr
    ;

matchExpr
    : 'match' expression matchBody
    ;

matchBody
    : '{' matchCase* '}'
    ;

matchCase
    : pattern '=>' expression ','?
    ;

// See file-header status note: structural destructuring is not yet
// supported by the reference parser, so `pattern` is just `expression`
// for now (literal patterns / identifier bindings / wildcard `_`).
pattern
    : expression
    ;

functionDecl
    : 'fn' IDENT typeParams? '(' params? ')' ('->' typeExpr)? blockExpr
    ;

typeParams
    : '<' IDENT (',' IDENT)* '>'
    ;

params
    : param (',' param)*
    ;

param
    : 'mut'? IDENT (':' typeExpr)? ('=' expression)?
    ;

structDecl
    : 'struct' IDENT typeParams? '{' structField* '}'
    ;

structField
    : 'public'? IDENT (':' typeExpr)? ','?
    ;

enumDecl
    : 'enum' IDENT typeParams? '{' enumVariant* '}'
    ;

enumVariant
    : IDENT enumVariantKind? ','?
    ;

enumVariantKind
    : '(' typeExpr (',' typeExpr)* ')'      # tupleVariant
    | '{' structField* '}'                  # structVariant
    ;

traitDecl
    : 'trait' IDENT typeParams? (':' typeExpr (',' typeExpr)*)? '{' traitItem* '}'
    ;

traitItem
    : traitMethod
    | traitAssocType
    ;

traitMethod
    : 'fn' IDENT typeParams? '(' params? ')' ('->' typeExpr)? (blockExpr | ';')
    ;

traitAssocType
    : 'type' IDENT (':' typeExpr)? ';'
    ;

implBlock
    : 'impl' typeParams? typeExpr ('for' typeExpr)? '{' implItem* '}'
    ;

implItem
    : functionDecl
    | traitAssocType
    ;

classDecl
    : 'class' IDENT (('extends' | 'implements') IDENT (',' IDENT)*)? '{' classMember* '}'
    ;

classMember
    : 'public'? 'static'?
      ( functionDecl
      | constructorDecl
      | fieldDecl
      )
    ;

constructorDecl
    : 'new' '(' params? ')' blockExpr
    ;

fieldDecl
    : IDENT (':' typeExpr)? ('=' expression)? (',' | ';')
    ;

interfaceDecl
    : 'interface' IDENT (':' IDENT (',' IDENT)*)? '{' interfaceMember* '}'
    ;

interfaceMember
    : traitMethod
    ;

moduleDecl
    : 'module' IDENT (blockExpr | ';')
    ;

importStmt
    : 'import' IDENT (('.' | '::') IDENT)* ';'?
    ;

useStmt
    : 'use' usePath ';'?
    ;

usePath
    : segment ('::' segment)* ('::' '*')?                       # useGlob
    | segment ('::' segment)* '::' '{' IDENT (',' IDENT)* '}'   # useNamed
    ;

segment
    : identLike
    ;

typeAliasDecl
    : 'type' IDENT typeParams? '=' typeExpr ';'?
    ;

unsafeBlock
    : 'unsafe' IDENT? blockExpr
    ;

// ─── Zenith-native statements ─────────────────────────────────────────────

quantumCircuitDecl
    : (('quantum' 'circuit') | 'circuit' | 'quantum') IDENT (blockExpr | expression)
    ;

nanoAgentDecl
    : (('nano' 'agent') | 'agent' | 'nano') IDENT (blockExpr | expression)
    ;

sankofaRememberStmt
    : 'remember' IDENT (':' typeExpr)? '=' expression ';'?
    ;

effectDecl
    : 'effect' IDENT (blockExpr | ';')
    ;

handleStmt
    : 'handle' IDENT blockExpr ('with' blockExpr)?
    ;

wisdomStmt
    : 'wisdom' IDENT ('=' expression)? ';'?
    ;

languageDecl
    : 'language' IDENT STRING? ';'?
    ;

expressionStmt
    : expression ';'?
    ;

// ─── Expressions (precedence climbing, lowest → highest) ──────────────────
// Mirrors GRAMMAR.md §4.1 / src/parser.rs Precedence::of. ANTLR4 resolves
// alternative order in a left-recursive rule as precedence (first =
// tightest-binding after left-recursion elimination is applied uniformly
// here from lowest to highest, matching the reference implementation).

expression
    : assignmentExpr
    ;

assignmentExpr
    : rangeExpr (assignOp assignmentExpr)?
    ;

assignOp
    : '=' | '+=' | '-=' | '*=' | '/='
    ;

rangeExpr
    : logicalOrExpr (('..' | '..=') logicalOrExpr)?
    ;

logicalOrExpr
    : logicalAndExpr (('||' | 'or') logicalAndExpr)*
    ;

logicalAndExpr
    : bitOrExpr (('&&' | 'and') bitOrExpr)*
    ;

bitOrExpr
    : bitXorExpr ('|' bitXorExpr)*
    ;

bitXorExpr
    : bitAndExpr ('^' bitAndExpr)*
    ;

bitAndExpr
    : equalityExpr ('&' equalityExpr)*
    ;

equalityExpr
    : comparisonExpr (('==' | '!=') comparisonExpr)*
    ;

comparisonExpr
    : shiftExpr (('<' | '<=' | '>' | '>=') shiftExpr)*
    ;

shiftExpr
    : sumExpr (('<<' | '>>') sumExpr)*
    ;

sumExpr
    : productExpr (('+' | '-') productExpr)*
    ;

productExpr
    : castExpr (('*' | '/' | '%') castExpr)*
    ;

// `as` cast and `:` type-ascription bind looser than prefix/call/index but
// are handled as a postfix-style wrapper around the prefix expression.
castExpr
    : prefixExpr (('as' | ':') typeExpr)*
    ;

prefixExpr
    : ('-' | '!' | '~' | '&' 'mut'? | '*') prefixExpr
    | postfixExpr
    ;

postfixExpr
    : primaryExpr postfixOp*
    ;

postfixOp
    : '(' args? ')'          # callOp
    | '[' expression ']'     # indexOp
    | '.' identLike ('(' args? ')')?   # memberOp
    | '?'                    # tryPropagateOp
    ;

args
    : expression (',' expression)*
    ;

primaryExpr
    : identLike structLiteralTail?  # identExpr
    | literal                     # literalExpr
    | '(' expression ')'          # parenExpr
    | '(' expression (',' expression)+ ')'   # tupleExpr
    | '[' (expression (',' expression)*)? ']'  # arrayExpr
    | blockExpr                   # blockValExpr
    | '|' params? '|' (blockExpr | expression)   # lambdaExpr
    | 'fn' '(' params? ')' ('->' typeExpr)? blockExpr   # anonFnExpr
    | ifExpr                       # ifValExpr
    | matchExpr                    # matchValExpr
    | loopExpr                     # loopValExpr
    | 'async' expression            # asyncExpr
    | 'await' expression            # awaitExpr
    | 'spawn' expression            # spawnExpr
    | 'new' IDENT ('(' args? ')')?  # newExpr
    | 'try' expression ('catch' ('(' (IDENT ':')? typeExpr ')')? blockExpr)*   # tryCatchExpr
    | recallExpr                    # recallValExpr
    | learnExpr                     # learnValExpr
    | performExpr                   # performValExpr
    | zamaniExpr                    # zamaniValExpr
    | sasaExpr                      # sasaValExpr
    | quantumOpExpr                 # quantumOpValExpr
    ;

structLiteralTail
    : '{' (identLike ':' expression ','?)* '}'
    ;

ifExpr
    : 'if' expression blockExpr ('else' (ifExpr | blockExpr))?
    ;

loopExpr
    : 'loop' blockExpr
    ;

blockExpr
    : '{' statement* '}'
    ;

recallExpr
    : 'recall' ('(' expression ')' | expression)
    ;

learnExpr
    : ('learn' | 'infer') 'from'? expression
    ;

performExpr
    : 'perform' expression
    ;

zamaniExpr
    : 'zamani' (blockExpr | expression)
    ;

sasaExpr
    : 'sasa' (blockExpr | expression)
    ;

quantumOpExpr
    : 'quantum' IDENT ('(' args? ')')?
    ;

literal
    : INTEGER
    | FLOAT
    | STRING
    | CHAR
    | BOOLEAN
    | NIL
    | QUANTUM_LITERAL
    | NANO_ANNOTATION
    | MTS_LITERAL
    ;

// ─── Type expressions ──────────────────────────────────────────────────────

typeExpr
    : (IDENT | INT_KW | FLOAT_KW | BOOL_KW | STR_KW | STRING_KW | CHAR_KW
       | VOID | SELF_LOWER | QUANTUM)
      ('<' typeExpr (',' typeExpr)* '>')?                                   # namedType
    | '(' ')'                                                          # unitType
    | '(' typeExpr ')'                                                 # parenType
    | '(' typeExpr (',' typeExpr)+ ')' ('->' typeExpr)?                # tupleOrFnType
    | 'fn' '(' (typeExpr (',' typeExpr)*)? ')' ('->' typeExpr)?        # fnType
    | '&' 'mut'? '[' typeExpr ']'                                      # sliceRefType
    | '&' 'mut'? typeExpr                                              # refType
    | '*' 'mut'? typeExpr                                              # rawPtrType
    | '[' typeExpr (';' expression)? ']'                               # arrayType
    | ('Self' | 'self')                                                # selfType
    ;

// ═══════════════════════════════════════════════════════════════════════
// Lexer rules
// ═══════════════════════════════════════════════════════════════════════

// ── Keywords (must precede IDENT so they win by ANTLR's longest-match +
//    first-declared-wins-on-tie rule) ───────────────────────────────────
LET: 'let'; VAR: 'var'; CONST: 'const'; MUT: 'mut'; FN: 'fn'; RETURN: 'return';
IF: 'if'; ELSE: 'else'; WHILE: 'while'; FOR: 'for'; IN: 'in'; LOOP: 'loop';
BREAK: 'break'; CONTINUE: 'continue'; MATCH: 'match'; WITH: 'with';
TRUE: 'true'; FALSE: 'false'; NIL_KW: 'nil'; NULL_KW: 'null';

STRUCT: 'struct'; ENUM: 'enum'; TRAIT: 'trait'; IMPL: 'impl'; CLASS: 'class';
INTERFACE: 'interface'; EXTENDS: 'extends'; IMPLEMENTS: 'implements';
PUBLIC: 'public'; PRIVATE: 'private'; PROTECTED: 'protected'; STATIC: 'static';
VIRTUAL: 'virtual'; OVERRIDE: 'override'; ABSTRACT: 'abstract'; NEW: 'new';
THIS: 'this'; SUPER: 'super'; TYPE: 'type'; SELF_LOWER: 'self'; SELF_UPPER: 'Self';
VOID: 'void'; INT_KW: 'int'; FLOAT_KW: 'float'; BOOL_KW: 'bool'; STR_KW: 'str';
STRING_KW: 'String'; CHAR_KW: 'char';

MODULE: 'module'; IMPORT: 'import'; USE: 'use'; FROM: 'from'; AS: 'as'; WHERE: 'where';

EFFECT: 'effect'; PERFORM: 'perform'; HANDLE: 'handle'; UNSAFE: 'unsafe';
TRY: 'try'; CATCH: 'catch'; THROW: 'throw'; YIELD: 'yield';

ASYNC: 'async'; AWAIT: 'await'; SPAWN: 'spawn';

QUANTUM: 'quantum'; CIRCUIT: 'circuit'; NANO: 'nano'; AGENT: 'agent';
REMEMBER: 'remember'; RECALL: 'recall'; LEARN: 'learn'; INFER: 'infer';
WISDOM: 'wisdom'; ZAMANI: 'zamani'; SASA: 'sasa'; ANCESTOR: 'ancestor';
LINEAR: 'linear'; AFFINE: 'affine'; LANGUAGE: 'language'; IS: 'is';
AND_KW: 'and'; OR_KW: 'or'; SIZEOF: 'sizeof'; LEN: 'len'; PRINT: 'print';
PRINTLN: 'println'; ASSERT: 'assert'; PANIC: 'panic'; SWITCH: 'switch';
CASE: 'case'; THEN: 'then'; MTS_KW: 'mts';

// ── Literals ────────────────────────────────────────────────────────────
BOOLEAN: TRUE | FALSE;
NIL: NIL_KW | NULL_KW;

INTEGER: DIGIT+;
FLOAT: DIGIT+ '.' DIGIT+;
STRING: '"' (ESC | ~["\\])* '"';
CHAR: '\'' (ESC | ~['\\]) '\'';

fragment ESC: '\\' [nrt0"'\\];
fragment DIGIT: [0-9];
fragment ALPHA: [a-zA-Z_];

// Zenith-native literals (see file-header status note re: MTS_LITERAL).
QUANTUM_LITERAL: '|' ('0' | '1' | '+' | '-') '\u27E9';
NANO_ANNOTATION: '@' IDENT ('(' ~[)]* ')')?;
MTS_LITERAL: 'mts' '[' ~[\]]* ']';

// ── Identifiers ─────────────────────────────────────────────────────────
IDENT: ALPHA (ALPHA | DIGIT)*;

// ── Punctuation & operators ─────────────────────────────────────────────
LPAREN: '('; RPAREN: ')'; LBRACE: '{'; RBRACE: '}'; LBRACK: '['; RBRACK: ']';
COMMA: ','; DOT: '.'; SEMI: ';'; COLON: ':'; ARROW: '->'; FATARROW: '=>';
COLONCOLON: '::'; TILDE: '~'; HASH: '#'; AT: '@';

PLUS: '+'; MINUS: '-'; STAR: '*'; SLASH: '/'; PERCENT: '%'; ASSIGN: '=';
EQ: '=='; NEQ: '!='; LT: '<'; GT: '>'; LE: '<='; GE: '>=';
ANDAND: '&&'; OROR: '||'; AMP: '&'; PIPE: '|'; CARET: '^';
SHL: '<<'; SHR: '>>';
PLUSEQ: '+='; MINUSEQ: '-='; STAREQ: '*='; SLASHEQ: '/=';
DOTDOT: '..'; DOTDOTEQ: '..=';
QUESTION: '?'; BANG: '!';

// ── Whitespace & comments ───────────────────────────────────────────────
LINE_COMMENT: '//' ~[\r\n]* -> skip;
BLOCK_COMMENT: '/*' .*? '*/' -> skip;
WS: [ \t\r\n]+ -> skip;
