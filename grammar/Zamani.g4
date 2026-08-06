/*
 * Zamani.g4 — ANTLR4 combined grammar for the Zenith language.
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
 *   antlr4 -Dlanguage=Python3 -visitor -o generated grammar/Zamani.g4
 *   antlr4 -Dlanguage=Java    -visitor -o generated grammar/Zamani.g4
 */

grammar Zamani;

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


[05/08, 19:31] Benwellonedge: import os

output_path = '/mnt/agents/output/Zamani.g4'
os.makedirs('/mnt/agents/output', exist_ok=True)

# Write the grammar file in chunks
with open(output_path, 'w', encoding='utf-8') as f:
    f.write('''// Zamani.g4 — Comprehensive ANTLR4 combined grammar for the Zamani language.
//
// This grammar unifies all documented features from:
//   - GRAMMAR.md (reference implementation)
//   - ZAMANI_GRAMMAR.md (NIMBUS v3.0 Universal Trinity + UBUNTU + OOP + Omniversal)
//   - Zamani.g4 (ANTLR4 baseline)
//
// It covers: core language, OOP, quantum, nano, MTS, Sankofa, algebraic effects,
// dependent types, session types, meta-programming, HDL, distributed/cloud computing,
// cryptography, on-device AI, self-evolution, actor model, AI/AGI governance,
// omniversal simulation, green computing, developer relations, and higher-kinded types.
//
// Build:
//   antlr4 -Dlanguage=Python3 -visitor -listener -o generated grammar/Zamani.g4
//   antlr4 -Dlanguage=Java   -visitor -listener -o generated grammar/Zamani.g4
//
// NOTE: Many advanced features (omniversal, AGI governance, self-evolution, etc.)
// are syntactically modeled here as documented but may not yet have runtime
// semantics in the reference compiler.

grammar Zamani;

// ═══════════════════════════════════════════════════════════════════════════════
// Parser Rules — Program Structure
// ═══════════════════════════════════════════════════════════════════════════════

program
    : declaration* EOF
    ;

declaration
    : moduleDecl
    | importDecl
    | exportDecl
    | functionDecl
    | structDecl
    | enumDecl
    | traitDecl
    | implDecl
    | typeAliasDecl
    | constDecl
    | classDecl
    | interfaceDecl
    | recordDecl
    | quantumCircuitDecl
    | nanoAgentDecl
    | languageDecl
    | effectDecl
    | mtsDecl
    | sankofaDecl
    | agentDecl
    | cognitiveBlock
    | metaBlock
    | hdlModuleDecl
    | cloudDecl
    | distributedDecl
    | onDeviceAgentDecl
    | selfEvolveDecl
    | optPassDecl
    | targetPlatform
    | runtimeDecl
    | actorDecl
    | aiSystemDecl
    | agiSystemDecl
    | asiSystemDecl
    | aesiSystemDecl
    | asesiSystemDecl
    | adminInterfaceDecl
    | paymentGatewayDecl
    | userFeedbackDecl
    | copyrightNoticeDecl
    | tailorMadeFeatureDecl
    | programOnceDecl
    | maliciousIdeaDetection
    | userBlockingDecl
    | legalActionDecl
    | sandboxDecl
    | omniversalSimulationDecl
    | omniversalCodeSynthDecl
    | omniversalDeployDecl
    | omniversalAlignmentDecl
    | omniversalContainmentDecl
    | omniversalTrustDecl
    | omniversalKnowledgeDecl
    | omniversalGenerativeDecl
    | omniversalSovereigntyDecl
    | omniversalGoalDecl
    | omniversalBioNanoDecl
    | omniversalRealityDecl
    | omniversalNlpDecl
    | chatArchitectDecl
    | greenComputingAttr
    | thermalOptDecl
    | resourceConserveDecl
    | selfDiscoverDecl
    | developerAnalyticsDecl
    | licenseTrackingDecl
    | deploymentDecl
    | versionReleaseDecl
    | lspServerDecl
    | typeClassDecl
    | typeClassInstance
    | higherKindedTypeDecl
    | selfAdjustDecl
    | selfVersioningDecl
    | extensionMethodDecl
    | extensionPropertyDecl
    | extensionIndexerDecl
    | extensionOperatorDecl
    | macroDecl
    | domainSpecificLanguageDecl
    | aspectDecl
    | typeProviderDecl
    | dataParallelismDecl
    | concurrentDataStructureDecl
    | messageHandlerDecl
    | musicDecl
    | roboticsDecl
    | deepLearningDecl
    | graphicsDecl
    | videoDecl
    | tensorDecl
    | matrixDecl
    | vectorDecl
    | mlModelDecl
    | quantumMlBlock
    | explainableRlBlock
    | explainableDeepLearningBlock
    | knowledgeGraphBlock
    | probabilisticGraphicalModelBlock
    | transferLearningBlock
    | multiAgentBlock
    | autonomousSystemBlock
    | graphModelingBlock
    | advancedNlpBlock
    | cognitiveArchitectureBlock
    | aiForBusinessBlock
    | vrArInteractionBlock
    | imageVideoAnalysisBlock
    | fileScopedType
    | attributeDecl
    | statement
    ;
''')

print("Part 1 written successfully")
[05/08, 19:33] Benwellonedge: with open(output_path, 'a', encoding='utf-8') as f:
    f.write('''
// ═══════════════════════════════════════════════════════════════════════════════
// Module System
// ═══════════════════════════════════════════════════════════════════════════════

moduleDecl
    : 'module' ident ('::' ident)* blockExpr
    | 'module' ident ('::' ident)* ';'
    ;

importDecl
    : 'import' modulePath ('as' ident)? ';'?
    ;

exportDecl
    : 'export' ident ('to' ident)? ';'?
    ;

modulePath
    : ident ('::' ident | '.' ident)*
    ;

useStmt
    : 'use' usePath ';'?
    ;

usePath
    : segment ('::' segment)* ('::' '*')?
    | segment ('::' segment)* '::' '{' ident (',' ident)* '}'
    ;

segment
    : ident
    ;

globalUsing
    : 'global' 'using' ident ';'
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Functions & Control Flow
// ═══════════════════════════════════════════════════════════════════════════════

functionDecl
    : modifiers? 'fn' ident typeParams? '(' params? ')' ('->' typeExpr)? ('with' effectList)? blockExpr
    ;

params
    : param (',' param)*
    ;

param
    : 'mut'? ident (':' typeExpr)? ('=' expression)?
    | '...' typeExpr ident
    ;

modifiers
    : modifier+
    ;

modifier
    : 'pub' | 'private' | 'protected' | 'static' | 'const' | 'async'
    | 'unsafe' | 'inline' | 'override' | 'final' | 'abstract' | 'virtual'
    | 'sealed' | 'partial' | 'file' | 'required' | 'init'
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
    : 'for' ident 'in' expression blockExpr
    ;

loopExpr
    : 'loop' blockExpr
    ;

ifExpr
    : 'if' expression blockExpr ('else' (ifExpr | blockExpr))?
    ;

matchStmt
    : matchExpr
    ;

matchExpr
    : 'match' expression '{' matchCase* '}'
    ;

matchCase
    : 'case' pattern ('when' expression)? '=>' expression ','?
    | pattern ('when' expression)? '=>' expression ','?
    ;

pattern
    : ident
    | literal
    | '_'
    | '(' pattern (',' pattern)* ')'
    | '[' pattern (',' pattern)* ']'
    | '[' pattern (',' pattern)* '...' pattern ']'
    | pattern '|' pattern
    | ident ':' typeExpr
    ;

unsafeBlock
    : 'unsafe' ident? blockExpr
    | 'unsafe' '!' '(' 'evas' ':' expression ')' blockExpr
    ;

throwStmt
    : 'throw' expression ';'?
    ;

tryCatchStmt
    : 'try' blockExpr catchClause* finallyClause?
    ;

catchClause
    : 'catch' ('(' param ')')? blockExpr
    ;

finallyClause
    : 'finally' blockExpr
    ;

blockExpr
    : '{' statement* '}'
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Variable Declarations
// ═══════════════════════════════════════════════════════════════════════════════

letStmt
    : ('let' | 'var') 'mut'? ident (':' typeExpr)? '=' expression ';'?
    ;

constStmt
    : 'const' ident (':' typeExpr)? '=' expression ';'?
    ;

constDecl
    : 'const' ident (':' typeExpr)? '=' expression ';'?
    ;
''')

print("Part 2 written successfully")
[05/08, 19:33] Benwellonedge: with open(output_path, 'a', encoding='utf-8') as f:
    f.write('''
// ═══════════════════════════════════════════════════════════════════════════════
// Expressions (Precedence Climbing)
// ═══════════════════════════════════════════════════════════════════════════════

expression
    : assignmentExpr
    ;

assignmentExpr
    : rangeExpr (assignOp assignmentExpr)?
    ;

assignOp
    : '=' | '+=' | '-=' | '*=' | '/=' | '%=' | '&=' | '|=' | '^=' | '<<=' | '>>='
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
    : comparisonExpr (('==' | '!=' | '===' | '!==') comparisonExpr)*
    ;

comparisonExpr
    : shiftExpr (('<' | '<=' | '>' | '>=' | 'instanceof' | 'is' | 'has') shiftExpr)*
    ;

shiftExpr
    : sumExpr (('<<' | '>>' | '>>>') sumExpr)*
    ;

sumExpr
    : productExpr (('+' | '-') productExpr)*
    ;

productExpr
    : castExpr (('*' | '/' | '%') castExpr)*
    ;

castExpr
    : prefixExpr (('as' | ':') typeExpr)*
    ;

prefixExpr
    : ('-' | '!' | '~' | '&' 'mut'? | '*' | '++' | '--') prefixExpr
    | postfixExpr
    ;

postfixExpr
    : primaryExpr postfixOp*
    ;

postfixOp
    : '(' args? ')'                    # callOp
    | '[' expression ']'               # indexOp
    | '.' ident ('(' args? ')')?       # memberOp
    | '?'                              # tryPropagateOp
    | '++'                             # postIncOp
    | '--'                             # postDecOp
    ;

args
    : expression (',' expression)*
    | namedArgument (',' namedArgument)*
    ;

namedArgument
    : ident '=' expression
    ;

primaryExpr
    : ident structLiteralTail?         # identExpr
    | literal                          # literalExpr
    | '(' expression ')'               # parenExpr
    | '(' expression (',' expression)+ ')' # tupleExpr
    | '[' (expression (',' expression)*)? ']' # arrayExpr
    | 'map' '{' (expression '=>' expression (',' expression '=>' expression)*)? '}' # mapExpr
    | blockExpr                        # blockValExpr
    | '|' params? '|' (blockExpr | expression) # lambdaExpr
    | 'fn' '(' params? ')' ('->' typeExpr)? blockExpr # anonFnExpr
    | ifExpr                           # ifValExpr
    | matchExpr                        # matchValExpr
    | loopExpr                         # loopValExpr
    | 'async' expression               # asyncExpr
    | 'await' expression               # awaitExpr
    | 'spawn' expression               # spawnExpr
    | 'new' ident typeArgs? '(' args? ')' # newExpr
    | 'try' expression catchClause*    # tryCatchExpr
    | 'yield' expression?              # yieldExpr
    | recallExpr                       # recallValExpr
    | learnExpr                        # learnValExpr
    | performExpr                      # performValExpr
    | zamaniExpr                       # zamaniValExpr
    | sasaExpr                         # sasaValExpr
    | quantumOpExpr                    # quantumOpValExpr
    | nanoExpr                         # nanoValExpr
    | mtsExpr                          # mtsValExpr
    | consensusExpr                    # consensusValExpr
    | ancestorCall                     # ancestorValExpr
    | withExpr                         # withValExpr
    | mopExpr                          # mopValExpr
    | macroCall                        # macroCallExpr
    | 'super' ('.' ident | '(' args? ')')? # superExpr
    | 'this'                           # thisExpr
    | 'self'                           # selfExpr
    | interpolatedString               # interpStringExpr
    ;

structLiteralTail
    : '{' (ident ':' expression ','?)* '}'
    ;

recallExpr
    : 'recall' ('(' expression ')' | expression)
    ;

learnExpr
    : ('learn' | 'infer') 'from'? expression ('with' 'weight' expression)?
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
    : 'quantum' ident ('(' args? ')')?
    | 'superpose' '(' expression (',' expression)* ')'
    | 'entangle' '(' ident ',' ident ')'
    ;

nanoExpr
    : nanoLit
    | 'assemble' '(' expression ')'
    | 'deploy' '(' expression ')'
    ;

mtsExpr
    : mtsLit
    | 'parallel' '(' blockExpr ')'
    | 'speculative' '(' blockExpr ')'
    | 'counterfactual' '(' expression ',' blockExpr ')'
    ;

consensusExpr
    : 'consensus' '[' exprList ']' 'vote' expression
    ;

ancestorCall
    : 'ancestral' ident '(' args? ')' ';'?
    ;

withExpr
    : expression 'with' '[' effectList ']'
    | expression 'with' '{' (ident ':' expression ';'?)* '}'
    ;

mopExpr
    : 'reflect' '(' expression ')'
    | 'introspect' '(' ident ')'
    | 'meta_eval' '(' expression ')'
    | 'quote' '{' statement* '}'
    | 'unquote' '(' expression ')'
    | 'splice' '(' expression ')'
    ;

macroCall
    : ident '!' '(' args? ')'
    ;

exprList
    : expression (',' expression)*
    ;

expressionStmt
    : expression ';'?
    ;
''')

print("Part 3 written successfully")
[05/08, 19:34] Benwellonedge: with open(output_path, 'a', encoding='utf-8') as f:
    f.write('''
// ═══════════════════════════════════════════════════════════════════════════════
// Literals
// ═══════════════════════════════════════════════════════════════════════════════

literal
    : INTEGER
    | FLOAT
    | STRING
    | CHAR
    | BOOLEAN
    | NIL
    | quantumLit
    | nanoLit
    | mtsLit
    | rawStringLit
    | utf8StringLit
    ;

quantumLit
    : '|' ('0' | '1' | '+' | '-' | ident) '\\u27E9'
    ;

nanoLit
    : '@atom' '(' ident ':' ORBITAL ')'
    | '@molecule' '(' FORMULA ')'
    ;

mtsLit
    : 'mts' '[' INTEGER ']'
    ;

interpolatedString
    : '$' STRING
    ;

rawStringLit
    : 'r' '#'* STRING
    ;

utf8StringLit
    : 'u8' STRING
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════════

typeExpr
    : baseType ('<' typeExpr (',' typeExpr)* '>')?   # namedType
    | '(' ')'                                         # unitType
    | '(' typeExpr ')'                                # parenType
    | '(' typeExpr (',' typeExpr)+ ')' ('->' typeExpr)? # tupleOrFnType
    | 'fn' '(' (typeExpr (',' typeExpr)*)? ')' ('->' typeExpr)? # fnType
    | '&' 'mut'? '[' typeExpr ']'                     # sliceRefType
    | '&' 'mut'? typeExpr                             # refType
    | '*' 'mut'? typeExpr                             # rawPtrType
    | '[' typeExpr (';' expression)? ']'              # arrayType
    | 'Self' | 'self'                                 # selfType
    | typeExpr '?'                                    # nullableType
    | 'Box' '<' typeExpr '>'                          # boxedType
    | 'linear' typeExpr                               # linearType
    | 'affine' typeExpr                               # affineType
    | 'session' '{' sessionOp* '}'                    # sessionType
    | piType                                          # piTypeExpr
    | sigmaType                                       # sigmaTypeExpr
    | identityType                                    # identityTypeExpr
    | 'Type_0' | 'Type_1' | 'Type_2' | 'Type_N'      # universeType
    | 'Kind' | 'Sort' | 'Prop'                        # metaType
    | quantumType                                     # quantumTypeExpr
    | nanoType                                        # nanoTypeExpr
    | mtsType                                         # mtsTypeExpr
    | sankofaType                                     # sankofaTypeExpr
    | cognitiveType                                   # cognitiveTypeExpr
    | typeExpr 'with' 'effects' '{' effectName (',' effectName)* '}' # effectfulType
    | 'hkt' '<' typeParam '.' typeExpr '>'            # higherKindedTypeExpr
    | 'exists' typeParam '.' typeExpr                 # existentialType
    | 'singleton' typeExpr                            # singletonType
    | typeExpr '.' ident                              # pathDepType
    ;

baseType
    : 'void' | 'int' | 'float' | 'bool' | 'string' | 'char' | 'bytes'
    | 'i8' | 'i16' | 'i32' | 'i64' | 'i128'
    | 'u8' | 'u16' | 'u32' | 'u64' | 'u128'
    | 'f32' | 'f64' | 'usize' | 'isize'
    | ident
    ;

typeParams
    : '<' typeParam (',' typeParam)* '>'
    ;

typeParam
    : ('out' | 'in' | '*')? ident (':' typeConstraint)?
    ;

typeConstraint
    : typeExpr ('+' typeExpr)*
    ;

typeArgs
    : '<' typeExpr (',' typeExpr)* '>'
    ;

piType
    : ('\\u03A0' | 'Pi') '(' ident ':' typeExpr ')' typeExpr
    ;

sigmaType
    : ('\\u03A3' | 'Sigma') '(' ident ':' typeExpr ')' typeExpr
    ;

identityType
    : 'Id' '(' typeExpr ',' expression ',' expression ')'
    ;

sessionOp
    : 'send' typeExpr
    | 'recv' typeExpr
    | 'offer' '{' sessionBranch* '}'
    | 'choice' '{' sessionBranch* '}'
    | 'close'
    ;

sessionBranch
    : ident '->' typeExpr
    ;

quantumType
    : 'Qubit'
    | 'QReg' '[' expression ']'
    | 'Superposition' '<' typeExpr '>'
    | 'Entangled' '<' typeExpr ',' typeExpr '>'
    | 'QMeasured' '<' typeExpr '>'
    | 'QArray' '<' typeExpr ',' expression '>'
    ;

nanoType
    : 'Atom' '<' typeExpr '>'
    | 'Molecule' '<' typeExpr '>'
    | 'NanoAgent' '<' typeExpr '>'
    | 'Archaeve' '<' typeExpr '>'
    ;

mtsType
    : 'MtsSlice' '<' expression '>'
    ;

sankofaType
    : 'History' '<' typeExpr ',' expression '>'
    | 'ConsensusTrue' '<' typeExpr '>'
    | 'InterMemory' '<' STRING ',' typeExpr '>'
    ;

cognitiveType
    : 'CognitiveState' '<' typeExpr '>'
    | 'Consciousness' '<' typeExpr '>'
    | 'Biological' '<' typeExpr '>'
    | 'Neural' '<' typeExpr '>'
    | 'MemoryBank' '<' typeExpr '>'
    | 'AgentType'
    | 'NarrowAI' | 'AGI' | 'ASI' | 'AESI' | 'ASESI'
    ;
''')

print("Part 4 written successfully")
[05/08, 19:34] Benwellonedge: with open(output_path, 'a', encoding='utf-8') as f:
    f.write('''
// ═══════════════════════════════════════════════════════════════════════════════
// Structs, Enums, Traits, Impls
// ═══════════════════════════════════════════════════════════════════════════════

structDecl
    : 'struct' ident typeParams? '{' structField* '}'
    ;

structField
    : modifiers? typeExpr ident ('=' expression)? ','?
    ;

enumDecl
    : 'enum' ident typeParams? (':' typeExpr)? '{' enumVariant* '}'
    ;

enumVariant
    : ident enumVariantKind? ','?
    ;

enumVariantKind
    : '(' typeExpr (',' typeExpr)* ')'    # tupleVariant
    | '{' structField* '}'                # structVariant
    ;

traitDecl
    : 'trait' ident typeParams? (':' typeExpr (',' typeExpr)*)? '{' traitItem* '}'
    ;

traitItem
    : traitMethod
    | traitAssocType
    | constantDef
    ;

traitMethod
    : 'fn' ident typeParams? '(' params? ')' ('->' typeExpr)? (blockExpr | ';')
    ;

traitAssocType
    : 'type' ident (':' typeExpr)? ';'
    ;

constantDef
    : 'const' ident ':' typeExpr '=' expression ';'
    ;

implDecl
    : 'impl' typeParams? typeExpr ('for' typeExpr)? '{' implItem* '}'
    ;

implItem
    : functionDecl
    | traitAssocType
    | constantDef
    ;

typeAliasDecl
    : 'type' ident typeParams? '=' typeExpr ';'?
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// OOP — Classes, Interfaces, Records
// ═══════════════════════════════════════════════════════════════════════════════

classDecl
    : modifiers? 'class' ident typeParams?
      extendsClause? implementsClause? permitsClause?
      '{' classMember* '}'
    | 'abstract' 'class' ident '{' classMember* '}'
    | 'final' 'class' ident extendsClause? '{' classMember* '}'
    | 'sealed' 'class' ident extendsClause? permitsClause? '{' classMember* '}'
    | 'static' 'class' ident '{' classMember* '}'
    | 'partial' 'class' ident '{' classMember* '}'
    | 'file' 'class' ident '{' classMember* '}'
    ;

extendsClause
    : ('extends' | 'implements') ident (',' ident)*
    ;

implementsClause
    : 'implements' ident (',' ident)*
    ;

permitsClause
    : 'permits' ident (',' ident)*
    ;

classMember
    : modifiers? typeExpr ident ('=' expression)? ';'                    # propertyDef
    | modifiers? 'fn' ident typeParams? '(' params? ')' ('->' typeExpr)? ('with' effectList)? blockExpr # methodDef
    | 'new' '(' params? ')' blockExpr                                    # constructorDef
    | 'static' modifiers? typeExpr ident ('=' expression)? ';'           # staticPropertyDef
    | 'static' modifiers? 'fn' ident '(' params? ')' ('->' typeExpr)? blockExpr # staticMethodDef
    | 'abstract' 'fn' ident '(' params? ')' ('->' typeExpr)? ';'         # abstractMethodDef
    | 'abstract' modifiers? typeExpr ident ';'                           # abstractPropertyDef
    | 'operator' OPERATOR '(' params? ')' blockExpr                      # operatorOverload
    | 'explicit' 'operator' typeExpr '(' params? ')' blockExpr           # explicitOperatorOverload
    | 'implicit' 'operator' typeExpr '(' params? ')' blockExpr           # implicitOperatorOverload
    | 'event' typeExpr ident ';'                                         # eventDef
    | 'delegate' ('->' typeExpr)? ident '(' params? ')' ';'              # delegateDef
    | 'this' '[' params? ']' '{' getter ';' setter? ';' '}'             # indexerDef
    | 'static' 'this' '[' params? ']' '{' getter ';' setter? ';' '}'    # staticIndexerDef
    | 'async' 'this' '[' params? ']' '{' getter ';' setter? ';' '}'     # asyncIndexerDef
    | 'init' modifiers? typeExpr ident ';'                               # initOnlyProperty
    | 'required' modifiers? typeExpr ident ';'                           # requiredProperty
    | 'class' ident '{' classMember* '}'                                 # innerClassDef
    | 'async' modifiers? 'fn' ident '(' params? ')' ('->' typeExpr)? blockExpr # asyncMethodDef
    | 'async' modifiers? typeExpr ident ';'                              # asyncPropertyDef
    | 'async' 'stream' typeExpr ';'                                      # asyncStream
    | 'async' 'iterator' typeExpr ';'                                    # asyncIterator
    | 'async' 'dispose' blockExpr                                        # asyncDispose
    | 'fn' ident '(' params? ')' ('->' typeExpr)? '=>' expression        # exprBodiedMember
    ;

getter
    : 'get' blockExpr
    ;

setter
    : 'set' blockExpr
    ;

interfaceDecl
    : 'interface' ident typeParams? extendsClause? '{' interfaceMember* '}'
    | 'sealed' 'interface' ident '{' interfaceMember* '}'
    ;

interfaceMember
    : 'fn' ident typeParams? '(' params? ')' ('->' typeExpr)? (blockExpr | ';') # interfaceMethod
    | 'default' 'fn' ident '(' params? ')' ('->' typeExpr)? blockExpr            # defaultInterfaceMethod
    | 'static' 'fn' ident '(' params? ')' ('->' typeExpr)? blockExpr             # staticInterfaceMethod
    | 'private' 'fn' ident '(' params? ')' ('->' typeExpr)? blockExpr            # privateInterfaceMethod
    | 'async' 'fn' ident '(' params? ')' ('->' typeExpr)? blockExpr              # asyncInterfaceMethod
    ;

recordDecl
    : 'record' ident typeParams? '(' params? ')' extendsClause? implementsClause? '{' recordBody? '}'
    | 'record' 'struct' ident '(' params? ')' '{' recordBody? '}'
    ;

recordBody
    : classMember*
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Extension Methods / Properties / Indexers / Operators
// ═══════════════════════════════════════════════════════════════════════════════

extensionMethodDecl
    : 'extension' 'fn' ident '(' params? ')' ('->' typeExpr)? ('where' typeConstraint)? blockExpr
    ;

extensionPropertyDecl
    : 'extension' typeExpr ident '{' getter ';' setter? ';' '}'
    ;

extensionIndexerDecl
    : 'extension' typeExpr 'this' '[' params? ']' '{' getter ';' setter? ';' '}'
    ;

extensionOperatorDecl
    : 'extension' 'operator' OPERATOR '(' params? ')' ('where' typeConstraint)? blockExpr
    ;
''')

print("Part 5 written successfully")
[05/08, 19:35] Benwellonedge: with open(output_path, 'a', encoding='utf-8') as f:
    f.write('''
// ═══════════════════════════════════════════════════════════════════════════════
// Quantum Computing
// ═══════════════════════════════════════════════════════════════════════════════

quantumCircuitDecl
    : ('quantum' 'circuit' | 'circuit' | 'quantum') ident '(' params? ')' blockExpr
    ;

quantumStmt
    : quantumGate
    | quantumMeasure
    | quantumReset
    | quantumBarrier
    ;

quantumGate
    : ('Hadamard' | 'CNOT' | 'PauliX' | 'PauliY' | 'PauliZ' | 'T' | 'S' | 'Swap') '(' ident (',' ident)* ')'
    | ('hadamard' | 'cnot' | 'paulix' | 'pauliy' | 'pauliz' | 't' | 's' | 'swap') '(' ident (',' ident)* ')'
    ;

quantumMeasure
    : 'measure' ident ('->' ident)?
    ;

quantumReset
    : 'reset' ident
    ;

quantumBarrier
    : 'barrier' (ident (',' ident)*)?
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Nano-Agent Computing
// ═══════════════════════════════════════════════════════════════════════════════

nanoAgentDecl
    : ('nano' 'agent' | 'agent' | 'nano') ident '{' nanoAgentBody '}'
    ;

nanoAgentBody
    : nanoCapability* nanoBehavior* nanoProtocol*
    ;

nanoCapability
    : 'capability' ident '(' params? ')' blockExpr
    ;

nanoBehavior
    : 'behavior' ident '(' params? ')' blockExpr
    ;

nanoProtocol
    : 'protocol' ident '{' protocolRule* '}'
    ;

protocolRule
    : 'on' ident '->' blockExpr
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Multi-Timeline System (MTS)
// ═══════════════════════════════════════════════════════════════════════════════

mtsDecl
    : 'mts' 'timeline' ident '{' mtsBody '}'
    ;

mtsBody
    : mtsSlice* mtsOperation*
    ;

mtsSlice
    : 'slice' ident '[' INTEGER ']' blockExpr
    ;

mtsOperation
    : 'fork' '(' ident ')'
    | 'merge' '(' ident ')'
    | 'observe' '(' ident ')'
    | 'rewind' '(' INTEGER ')'
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Sankofa Memory System
// ═══════════════════════════════════════════════════════════════════════════════

sankofaDecl
    : memoryDecl
    | wisdomDecl
    | historyDecl
    | consensusDecl
    | interMemoryDecl
    ;

memoryDecl
    : 'remember' ident ':' typeExpr '=' expression ';'?
    ;

wisdomDecl
    : 'wisdom' ident '{' wisdomBody '}'
    ;

wisdomBody
    : (premiseDecl | inferenceRule | wisdomStmt)*
    ;

premiseDecl
    : 'premise' ident ':' typeExpr '=' expression ';'
    ;

inferenceRule
    : 'rule' ident '(' params? ')' '->' blockExpr
    ;

wisdomStmt
    : 'conclude' expression ';'
    ;

historyDecl
    : 'history' ident '{' statement* '}'
    ;

consensusDecl
    : 'consensus' ident '{' statement* '}'
    ;

interMemoryDecl
    : 'intermemory' ident '{' statement* '}'
    ;

sankofaRememberStmt
    : 'remember' ident (':' typeExpr)? '=' expression ';'?
    ;

learnStmt
    : 'learn' 'from' expression ('with' 'weight' expression)? ';'
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Algebraic Effects
// ═══════════════════════════════════════════════════════════════════════════════

effectDecl
    : 'effect' ident typeParams? '(' params? ')' ('->' typeExpr)? ';'?
    ;

effectList
    : '[' effectName (',' effectName)* ']'
    ;

effectName
    : ident
    ;

handleStmt
    : 'handle' expression '{' effectHandler* '}' ('with' blockExpr)?
    ;

effectHandler
    : 'case' effectName '(' params? ')' '->' blockExpr
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Meta-Programming
// ═══════════════════════════════════════════════════════════════════════════════

languageDecl
    : 'language' ident ('=' STRING)? ('{' grammarRule* '}')? ';'?
    ;

grammarRule
    : ident ':' STRING ';'
    ;

metaBlock
    : 'meta' '{' statement* '}'
    ;

invokeStmt
    : 'invoke' modulePath '(' args? ')' ';'?
    ;

transcodeStmt
    : 'transcode' ident '::' STRING 'to' ident ';'?
    ;

overrideStmt
    : 'override' ident '::' ident '(' params? ')' blockExpr
    ;

langStmt
    : 'lang' ident blockExpr
    ;

pluginStmt
    : 'plugin' ident '{' pluginDefinition* '}'
    ;

pluginDefinition
    : ('language' | 'transpiler') ident ';'
    ;

macroDecl
    : 'macro' ident '(' params? ')' blockExpr
    ;

domainSpecificLanguageDecl
    : 'domain' ident (':' typeExpr)? ('module' ident)? ('type' typeExpr)? ('plugin' ident)? ('provider' ident)? '{' statement* '}'
    ;

aspectDecl
    : 'aspect' ident ('extends' ident)? ('depends' ident)? ('compose' ident)? ('weave' ident)? ('domain' ident)? '{' statement* '}'
    ;

typeProviderDecl
    : 'provider' ident '(' typeParams? ')' '{' statement* '}' ('cache' ident)? ('inject' ident)? ('metadata' typeExpr)? ('aspect' ident)?
    ;
''')

print("Part 6 written successfully")
[05/08, 19:35] Benwellonedge: with open(output_path, 'a', encoding='utf-8') as f:
    f.write('''
// ═══════════════════════════════════════════════════════════════════════════════
// HDL (Hardware Description Language)
// ═══════════════════════════════════════════════════════════════════════════════

hdlModuleDecl
    : 'hdl' 'module' ident '{' hdlBody '}'
    ;

hdlBody
    : hdlPort* hdlComponent* hdlLogicGate* hdlSignal* hdlAssignment*
    ;

hdlPort
    : 'port' hdlPortDirection ident ':' hdlPortType
    ;

hdlPortDirection
    : 'input' | 'output' | 'inout'
    ;

hdlPortType
    : 'wire' | 'reg' | 'logic' | 'quantum' | 'nano' | typeExpr
    ;

hdlComponent
    : 'component' ident '{' hdlPort* '}'
    ;

hdlLogicGate
    : 'gate' hdlLogicGateType '(' ident (',' ident)* ')'
    | 'gate' quantumGateType '(' ident (',' ident)* ')'
    ;

hdlLogicGateType
    : 'AND' | 'OR' | 'NOT' | 'XOR' | 'NAND' | 'NOR' | 'MUX' | 'DEMUX'
    ;

quantumGateType
    : 'Hadamard' | 'CNOT' | 'Toffoli' | 'Fredkin' | 'PauliX' | 'PauliY' | 'PauliZ'
    ;

hdlSignal
    : 'signal' ident ':' hdlPortType
    ;

hdlAssignment
    : 'assign' ident '=' hdlExpression ';'
    ;

hdlExpression
    : ident
    | hdlLogicGate
    | '(' hdlExpression ')'
    | hdlExpression ('&' | '|' | '^' | '~') hdlExpression
    ;

externalHdlImport
    : 'import_hdl' '(' STRING ',' ident ')'
    ;

externalHdlLink
    : 'link_hdl' '(' ident ',' ident ')'
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Distributed & Cloud Computing
// ═══════════════════════════════════════════════════════════════════════════════

distributedDecl
    : 'distributed' 'node' ident '{' distributedBody '}'
    ;

distributedBody
    : (serviceHandle | remoteCall | distributedOperation)*
    ;

serviceHandle
    : 'service' ident 'at' STRING
    ;

remoteCall
    : 'remote' ident '::' ident '(' args? ')' ';'?
    ;

distributedOperation
    : 'teleport' '(' ident ',' STRING ')'
    | 'migrate' '(' ident ',' STRING ')'
    | 'dsm' '(' ident ',' STRING ')'
    | 'dmts' '(' ident ',' STRING ')'
    ;

cloudDecl
    : cloudPlatform
    | container
    | devOpsTool
    | databaseOp
    | webService
    ;

cloudPlatform
    : ('AWS' | 'Azure' | 'GCP') '::' ident '(' args? ')' ';'?
    ;

container
    : 'Docker' '::' ident '(' args? ')' ';'?
    ;

devOpsTool
    : 'Jenkins' '::' ident '(' args? ')' ';'?
    | 'Git' '::' ident '(' args? ')' ';'?
    | 'Ansible' '::' ident '(' args? ')' ';'?
    ;

databaseOp
    : 'Database' '::' ident '(' args? ')' ';'?
    ;

webService
    : 'HTTP' '::' ident '(' args? ')' ';'?
    ;

dataStmt
    : ('serialize' | 'deserialize') expression ('to' | 'from') dataFormat ';'?
    ;

dataFormat
    : 'json' | 'xml' | 'messagepack' | 'protobuf' | 'cbor'
    ;

streamingData
    : 'stream' expression 'pipe' expression ';'
    ;

foreignFunctionCall
    : 'foreign' ident '::' ident '(' args? ')' ';'?
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Cryptography
// ═══════════════════════════════════════════════════════════════════════════════

cryptoExpr
    : 'encrypt' '(' expression ',' cryptoAlgo ')'
    | 'decrypt' '(' expression ',' cryptoAlgo ')'
    | 'encrypt_homomorphic' '(' expression ',' cryptoKey ')'
    | 'decrypt_homomorphic' '(' expression ',' cryptoKey ')'
    | 'encrypt_layered' '(' expression ',' cryptoAlgoList ')'
    | 'encrypt_quantum_safe' '(' expression ',' cryptoKey ')'
    ;

cryptoAlgo
    : 'AES256' | 'ChaCha20' | 'RSA4096' | 'Kyber' | 'Dilithium' | 'SPHINCS+'
    ;

cryptoAlgoList
    : '[' cryptoAlgo (',' cryptoAlgo)* ']'
    ;

cryptoKey
    : 'key' ident
    | 'generate_key' '(' typeExpr ')'
    ;

zkExpr
    : 'generate_zk_proof' '(' expression ',' expression ')'
    | 'verify_zk_proof' '(' expression ',' expression ')'
    ;

smcExpr
    : 'secure_multi_party_compute' '(' exprList ')'
    ;

keyManagement
    : 'request_key' '(' typeExpr ')'
    | 'rotate_key' '(' ident ')'
    | 'revoke_key' '(' ident ')'
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// On-Device / Edge AI
// ═══════════════════════════════════════════════════════════════════════════════

onDeviceAgentDecl
    : 'on_device' 'agent' ident '{' onDeviceAgentBody '}'
    | 'on_device_agent' ident '{' onDeviceAgentBody '}'
    ;

onDeviceAgentBody
    : (agentCapability | agentBehavior | deviceConstraint | selfPreservationProtocol | offlineProtocol)*
    ;

deviceConstraint
    : 'requires' deviceSpec (',' deviceSpec)* ';'
    ;

deviceSpec
    : 'memory' '>=' expression
    | 'power' '>=' expression
    | 'storage' '>=' expression
    | 'processor' '=' typeExpr
    ;

selfPreservationProtocol
    : 'self_preserve' blockExpr
    | 'self_preservation' '{' preservationProtocol* '}'
    ;

preservationProtocol
    : 'protocol' STRING
    ;

offlineProtocol
    : 'offline' blockExpr
    ;

offlineAgent
    : 'offline' 'agent' ident '{' offlineBody '}'
    ;

offlineBody
    : (localResourceSpec | mtsLocalSpec | lifecycleSpec)*
    ;

localResourceSpec
    : 'local_resources' '=' '[' identList ']'
    ;

mtsLocalSpec
    : 'local_mts' '=' 'true'
    ;

lifecycleSpec
    : 'lifecycle' '{' lifecycleAction* '}'
    ;

lifecycleAction
    : ('monitor' | 'update' | 'manage') ident
    ;
''')

print("Part 7 written successfully")
[05/08, 19:36] Benwellonedge: with open(output_path, 'a', encoding='utf-8') as f:
    f.write('''
// ═══════════════════════════════════════════════════════════════════════════════
// Self-Evolving / Autonomous Features
// ═══════════════════════════════════════════════════════════════════════════════

selfEvolveDecl
    : 'self_evolve' '{' evolveBody '}'
    ;

evolveBody
    : (monitorRule | optimizeRule | patchRule | verifyRule)*
    ;

monitorRule
    : 'monitor' expression '->' blockExpr
    ;

optimizeRule
    : 'optimize' expression '->' blockExpr
    ;

patchRule
    : 'patch' expression '->' blockExpr
    ;

verifyRule
    : 'verify' expression '->' blockExpr
    ;

autoCodeGen
    : 'autonomously_generate' '(' expression ')' '->' blockExpr
    ;

autoOptimize
    : 'autonomously_optimize' '(' expression ')' '->' blockExpr
    ;

autoVerify
    : 'autonomously_verify' '(' expression ')' '->' blockExpr
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Optimization & Compilation Targets
// ═══════════════════════════════════════════════════════════════════════════════

optPassDecl
    : 'optimization' 'pass' ident '{' optPassBody '}'
    ;

optPassBody
    : ('target' optTarget ';'?)* ('strategy' optStrategy ';'?)* blockExpr?
    ;

optTarget
    : 'classical' | 'quantum' | 'nano' | 'neuromorphic' | 'GPU' | 'FPGA'
    | 'SIMD' | 'WASM' | 'USSD' | 'edge' | 'stellar'
    ;

optStrategy
    : 'DCE' | 'CSE' | 'inlining' | 'loop_unroll' | 'constant_fold'
    | 'quantum_gate_opt' | 'nano_atp_efficiency' | 'ai_quantization'
    | 'thermal_throttle_prevention' | 'simd_vectorization'
    | 'gpu_offloading' | 'quantum_error_correction'
    ;

targetPlatform
    : 'target' platformSpec
    ;

platformSpec
    : 'x86_64' | 'ARM64' | 'RISC-V' | 'WASM' | 'LLVM_IR' | 'bare_metal'
    | 'Android' | 'iOS' | 'cloud' | 'IoT' | 'USSD' | 'FPGA'
    | 'quantum' | 'nano' | 'neuromorphic' | 'stellar'
    | 'Tariro_Runtime' | 'Z_MMP'
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Runtime Features (POCO-REAF)
// ═══════════════════════════════════════════════════════════════════════════════

runtimeDecl
    : 'runtime' 'configure' '{' runtimeConfig* '}'
    ;

runtimeConfig
    : 'gc' '=' ('enabled' | 'disabled' | 'hybrid') ';'?
    | 'self_heal' '=' BOOLEAN ';'?
    | 'quantum_sim' '=' BOOLEAN ';'?
    | 'nano_orchestration' '=' BOOLEAN ';'?
    | 'effect_dispatch' '=' ('eager' | 'lazy' | 'batched') ';'?
    | 'scheduler' '=' ('preemptive' | 'cooperative' | 'quantum') ';'?
    ;

spawnExpr
    : 'spawn' blockExpr
    ;

channelExpr
    : 'channel' '<' typeExpr '>'
    ;

selectStmt
    : 'select' '{' selectCase* '}'
    ;

selectCase
    : 'case' expression '->' blockExpr
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Actor Model
// ═══════════════════════════════════════════════════════════════════════════════

actorDecl
    : 'actor' ident '{' actorBody '}'
    ;

actorBody
    : (actorMessage | actorBehavior | actorState | concurrentExpr)*
    ;

actorMessage
    : 'message' ident '(' params? ')' blockExpr
    ;

actorBehavior
    : 'behavior' ident blockExpr
    ;

actorState
    : 'state' ident ':' typeExpr '=' expression ';'
    ;

concurrentExpr
    : 'concurrent' expression blockExpr
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// AI / Cognitive / AGI Governance
// ═══════════════════════════════════════════════════════════════════════════════

cognitiveBlock
    : 'cognitive_architecture' ident '{' statement* '}'
    ;

aiDomainBlock
    : aiDomain ident '{' statement* '}'
    ;

aiDomain
    : 'machine_learning' | 'deep_learning' | 'neural_network'
    | 'graph_based_deep_learning' | 'causal_discovery' | 'causal_reasoning'
    | 'probabilistic_modeling' | 'advanced_computer_vision'
    | 'predictive_analytics' | 'prescriptive_analytics' | 'data_visualization'
    | 'human_ai_collaboration' | 'advanced_human_ai_collaboration'
    | 'explainability_and_transparency' | 'safety_and_security'
    | 'ethics_and_governance' | 'quantum_machine_learning'
    | 'cognitive_architectures' | 'human_computer_interaction'
    | 'data_analytics' | 'time_series_ml' | 'time_series_forecasting'
    | 'time_series_analysis' | 'text_generation' | 'sentiment_analysis'
    | 'language_translation' | 'object_detection' | 'object_recognition'
    | 'image_classification' | 'autonomous_navigation' | 'autonomous_robotics'
    | 'autonomous_systems' | 'model_explainability' | 'ai_safety_and_security'
    | 'ai_ethics_and_governance' | 'quantum_optimization'
    | 'recommendation_systems' | 'quantum_computing' | 'blockchain_technology'
    | 'iot' | 'cybersecurity' | 'data_science' | 'natural_language_generation'
    | 'nl_generation' | 'advanced_robotics' | 'transfer_learning'
    | 'explainable_reinforcement_learning' | 'graph_neural_network'
    | 'advanced_natural_language_processing' | 'advanced_natural_language_generation'
    | 'advanced_computer_vision_for_object_recognition'
    | 'advanced_computer_vision_for_object_detection'
    | 'advanced_computer_vision_for_image_classification'
    | 'advanced_robotics_for_autonomous_systems'
    | 'advanced_robotics_for_autonomous_navigation'
    | 'advanced_explainability_and_transparency_for_ai_models'
    | 'advanced_safety_and_security_for_ai_systems'
    | 'advanced_ethics_and_governance_for_ai_development'
    | 'advanced_quantum_computing_for_optimization_problems'
    | 'advanced_quantum_computing_for_machine_learning'
    | 'advanced_causal_reasoning_for_decision_making'
    | 'advanced_cognitive_architectures_for_human_ai_collaboration'
    | 'advanced_machine_learning_for_recommendation_systems'
    | 'advanced_nlp_for_sentiment_analysis' | 'advanced_nlp_for_text_generation'
    | 'advanced_nlp_for_language_translation'
    | 'advanced_machine_learning_for_time_series_data'
    | 'advanced_machine_learning_for_time_series_forecasting'
    | 'advanced_machine_learning_for_time_series_analysis'
    ;

agentDecl
    : 'agent' ident (':' agentType)? '{' agentBody '}'
    ;

agentBody
    : (agentCapability | agentBehavior | agentGoal | agentConstraint)*
    ;

agentGoal
    : 'goal' expression ';'
    ;

agentConstraint
    : 'constraint' expression ';'
    ;

agentType
    : 'NarrowAI' | 'AGI' | 'ASI' | 'AESI' | 'ASESI'
    ;

humanAgiCollaboration
    : 'human_ai_collaboration' ident '{' statement* '}'
    ;

aiSystemDecl
    : 'ai' ident '{' aiSystemBody '}'
    ;

aiSystemBody
    : (aiTypeDef | aiCapabilityDef | explainStmt | transparentStmt | selfVersioningDecl)*
    ;

aiTypeDef
    : 'type' ident '=' ('narrow' | 'general' | 'super') ';'
    ;

aiCapabilityDef
    : 'capability' ident '{' statement* '}'
    ;

agiSystemDecl
    : 'agi' ident '{' agiSystemBody '}'
    ;

agiSystemBody
    : (agiCapabilityDef | agiLearningDef)*
    ;

agiCapabilityDef
    : 'capability' ident '{' statement* '}'
    ;

agiLearningDef
    : 'learning' ident '{' statement* '}'
    ;

asiSystemDecl
    : 'asi' ident '{' asiSystemBody '}'
    ;

asiSystemBody
    : (asiCapabilityDef | asiSelfImprovementDef)*
    ;

asiSelfImprovementDef
    : 'self_improvement' ident '{' statement* '}'
    ;

aesiSystemDecl
    : 'aesi' ident '{' aesiSystemBody '}'
    ;

aesiSystemBody
    : (aesiCapabilityDef | aesiTransformationDef)*
    ;

aesiTransformationDef
    : 'transformation' ident '{' statement* '}'
    ;

asesiSystemDecl
    : 'asesi' ident '{' asesiSystemBody '}'
    ;

asesiSystemBody
    : (asesiCapabilityDef | asesiOmnipotenceDef)*
    ;

asesiOmnipotenceDef
    : 'omnipotence' ident '{' statement* '}'
    ;
''')

print("Part 8 written successfully")
[05/08, 19:37] Benwellonedge: with open(output_path, 'a', encoding='utf-8') as f:
    f.write('''
// ═══════════════════════════════════════════════════════════════════════════════
// Safety / Security / Ethics / Governance Attributes
// ═══════════════════════════════════════════════════════════════════════════════

evasCert
    : 'unsafe' '!' '(' 'evas' ':' expression ')' blockExpr
    ;

safetyAttr
    : '#safety' '(' attrArgs ')'
    ;

securityAttr
    : '#security' '(' attrArgs ')'
    ;

ethicsAttr
    : '#ethics' '(' attrArgs ')'
    ;

governanceAttr
    : '#governance' '(' attrArgs ')'
    ;

complianceAttr
    : '#compliance' '(' 'standard' '=' STRING ',' 'certified_by' '=' STRING ')'
    ;

greenComputingAttr
    : '#green' '(' greenArgs ')'
    ;

attrArgs
    : attrArg (',' attrArg)*
    ;

attrArg
    : ident '=' expression
    ;

greenArgs
    : 'minimize_power' '=' expression
    | 'minimize_heat' '=' expression
    | 'minimize_water' '=' expression
    | 'maximize_efficiency' '=' expression
    ;

attributeDecl
    : '@' ident '(' annotationArgs? ')'
    | '#' '[' expression* ']'
    ;

annotationArgs
    : annotationArg (',' annotationArg)*
    ;

annotationArg
    : ident '=' expression
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Administration & Governance Blocks
// ═══════════════════════════════════════════════════════════════════════════════

adminInterfaceDecl
    : 'admin' ident '{' adminInterfaceBody '}'
    ;

adminInterfaceBody
    : (changeLogDisplay | suggestionInput | hybridDef | interfaceDef)*
    ;

changeLogDisplay
    : 'display' 'changes' '{' changeLogBody '}'
    ;

changeLogBody
    : changeLogEntry*
    ;

changeLogEntry
    : 'change' ident 'made' 'by' ident ';'
    ;

suggestionInput
    : 'input' 'suggestions' '{' suggestionBody '}'
    ;

suggestionBody
    : suggestionEntry*
    ;

suggestionEntry
    : 'suggestion' ident 'from' ident ';'
    ;

paymentGatewayDecl
    : 'payment' ident '{' paymentGatewayBody '}'
    ;

paymentGatewayBody
    : (paymentMethodDef | paymentVerificationDef)*
    ;

paymentMethodDef
    : 'method' ident '{' statement* '}'
    ;

paymentVerificationDef
    : 'verify' ident '{' statement* '}'
    ;

userFeedbackDecl
    : 'feedback' ident '{' userFeedbackBody '}'
    ;

userFeedbackBody
    : (feedbackInputDef | feedbackValidationDef)*
    ;

feedbackInputDef
    : 'input' 'feedback' '{' statement* '}'
    ;

feedbackValidationDef
    : 'validate' 'feedback' '{' statement* '}'
    ;

copyrightNoticeDecl
    : 'copyright' ident '{' copyrightNoticeBody '}'
    ;

copyrightNoticeBody
    : copyrightNoticeStatement*
    ;

copyrightNoticeStatement
    : 'copyright' ident 'owned' 'by' ident ';'
    ;

tailorMadeFeatureDecl
    : 'feature' ident '{' tailorMadeFeatureBody '}'
    ;

tailorMadeFeatureBody
    : (featureDef | featureCustomizationDef)*
    ;

featureDef
    : 'define' 'feature' ident '{' statement* '}'
    ;

featureCustomizationDef
    : 'customize' 'feature' ident '{' statement* '}'
    ;

programOnceDecl
    : 'program_once' ident '{' programOnceBody '}'
    ;

programOnceBody
    : (systemConfigDef | systemLogicDef)*
    ;

systemConfigDef
    : 'config' ident '{' statement* '}'
    ;

systemLogicDef
    : 'logic' ident '{' statement* '}'
    ;

maliciousIdeaDetection
    : 'malicious' 'idea' 'detection' '{' maliciousIdeaBody '}'
    ;

maliciousIdeaBody
    : (ideaAnalysisDef | ideaBlockingDef)*
    ;

ideaAnalysisDef
    : 'analyze' 'idea' ident '{' statement* '}'
    ;

ideaBlockingDef
    : 'block' 'idea' ident '{' statement* '}'
    ;

userBlockingDecl
    : 'block' 'user' ident '{' userBlockingBody '}'
    ;

userBlockingBody
    : (userIdentificationDef | userBlockingActionDef)*
    ;

userIdentificationDef
    : 'identify' 'user' ident '{' statement* '}'
    ;

userBlockingActionDef
    : 'block' 'user' ident '{' statement* '}'
    ;

legalActionDecl
    : 'legal' 'action' ident '{' legalActionBody '}'
    ;

legalActionBody
    : (legalProceedingDef | legalNoticeDef)*
    ;

legalProceedingDef
    : 'proceeding' 'legal' ident '{' statement* '}'
    ;

legalNoticeDef
    : 'notice' 'legal' ident '{' statement* '}'
    ;

sandboxDecl
    : 'sandbox' ident '{' sandboxBody '}'
    ;

sandboxBody
    : (simulationDef | testingDef)*
    ;

simulationDef
    : 'simulate' ident '{' statement* '}'
    ;

testingDef
    : 'test' ident '{' statement* '}'
    ;
''')

print("Part 9 written successfully")
[05/08, 19:39] Benwellonedge: // ═══════════════════════════════════════════════════════════════════════════════
// Omniversal System
// ═══════════════════════════════════════════════════════════════════════════════

omniversalSimulationDecl
    : 'omniversal_simulate' ident '{' statement* '}'
    ;

omniversalCodeSynthDecl
    : 'omniversal_synthesize' ident '{' statement* '}'
    ;

omniversalDeployDecl
    : 'omniversal_deploy' ident '{' statement* '}'
    ;

omniversalAlignmentDecl
    : 'omniversal_alignment' ident '{' statement* '}'
    ;

omniversalContainmentDecl
    : 'omniversal_containment' ident '{' statement* '}'
    ;

omniversalTrustDecl
    : 'omniversal_trust' ident '{' statement* '}'
    ;

omniversalKnowledgeDecl
    : 'omniversal_knowledge' ident '{' statement* '}'
    ;

omniversalGenerativeDecl
    : 'omniversal_generate' ident '{' statement* '}'
    ;

omniversalSovereigntyDecl
    : 'omniversal_sovereignty' ident '{' statement* '}'
    ;

omniversalGoalDecl
    : 'omniversal_goal' ident '{' statement* '}'
    ;

omniversalBioNanoDecl
    : 'omniversal_bionano' ident '{' statement* '}'
    ;

omniversalRealityDecl
    : 'omniversal_reality' ident '{' statement* '}'
    ;

omniversalNlpDecl
    : 'omniversal_nlp' ident '{' statement* '}'
    ;

multiverseSimulation
    : 'multiverse' '{' multiverseBody '}'
    ;

multiverseBody
    : universeCount universeParams*
    ;

universeCount
    : 'universes' '=' INTEGER
    ;

universeParams
    : 'universe' ident '{' universeBody '}'
    ;

universeBody
    : 'physics' '=' STRING
    | 'rules' '=' STRING
    | 'initial_state' '=' STRING
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Conversational Code Synthesis
// ═══════════════════════════════════════════════════════════════════════════════

chatArchitectDecl
    : 'chat_agent' ident '{' chatAgentBody '}'
    ;

chatAgentBody
    : (chatCapability | chatContext | chatSynthesis)*
    ;

chatCapability
    : 'capability' ident blockExpr
    ;

chatContext
    : 'context' ident blockExpr
    ;

chatSynthesis
    : 'synthesize' expression '->' blockExpr
    ;

nlCodeGenExpr
    : 'nl_generate' expression 'as' typeExpr
    ;

nlInterpretExpr
    : 'nl_interpret' expression
    ;

nlTranslateExpr
    : 'nl_translate' expression 'to' typeExpr
    ;

documentationGeneration
    : 'document' ident '{' documentationBody '}'
    ;

documentationBody
    : docFormatSpec? docContentSpec? docModalSpec?
    ;

docFormatSpec
    : 'format' '=' ('document' | 'book' | 'article' | 'report' | 'journal' | 'news' | 'interactive_web')
    ;

docContentSpec
    : 'content' '=' STRING
    ;

docModalSpec
    : 'multimodal' '=' ('text' | 'diagrams' | 'images' | 'video' | 'interactive')
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Green Computing / Energy Optimization
// ═══════════════════════════════════════════════════════════════════════════════

thermalOptDecl
    : 'thermal_optimize' '{' thermalOptBody '}'
    | 'thermal' '{' thermalBody '}'
    ;

thermalOptBody
    : ('threshold' expression ';'?)* ('strategy' optStrategy ';'?)* blockExpr?
    ;

thermalBody
    : thermalGoal? throttlePrevention?
    ;

thermalGoal
    : 'max_temp' '=' expression
    ;

throttlePrevention
    : 'prevent_throttling' '=' 'true'
    ;

resourceConserveDecl
    : 'conserve' '(' resourceType (',' resourceType)* ')' blockExpr
    | 'conserve' '{' conserveBody '}'
    ;

conserveBody
    : conserveTarget*
    ;

conserveTarget
    : ('power' | 'water' | 'carbon_footprint' | 'heat') '=' expression
    ;

resourceType
    : 'power' | 'water' | 'heat' | 'compute' | 'memory' | 'storage' | 'network'
    ;

energyAware
    : 'energy_aware' '{' energyBody '}'
    ;

energyBody
    : energyGoal? energyStrategy? dvfsHint?
    ;

energyGoal
    : 'goal' '=' ('minimize_power' | 'minimize_heat' | 'minimize_water' | 'minimize_carbon' | 'maximize_efficiency')
    ;

energyStrategy
    : 'strategy' '=' ('AOT' | 'JIT' | 'nano_compile' | 'edge_compile')
    ;

dvfsHint
    : 'dvfs' '(' 'clock' '=' expression ',' 'voltage' '=' expression ')'
    ;

cloudEnergy
    : 'cloud_energy' '{' cloudEnergyBody '}'
    ;

cloudEnergyBody
    : ('renewable' '=' 'true') | ('pue' '=' expression) | ('data_center' '=' STRING)
    ;

resourceMatching
    : 'match_resources' '{' resourceMatchBody '}'
    ;

resourceMatchBody
    : ('parallel' '=>' 'GPU') | ('numerical' '=>' 'FPGA') | ('quantum' '=>' 'QPU') | ('nano' '=>' 'NACU')
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Developer Relations & Self-Discovery
// ═══════════════════════════════════════════════════════════════════════════════

selfDiscoverDecl
    : 'self_discover' '{' discoverConfig* '}'
    | 'self_discover' '{' discoveryBody '}'
    ;

discoverConfig
    : 'ide_detect' STRING ';'?
    | 'language_server' '=' BOOLEAN ';'?
    | 'auto_suggest' '=' BOOLEAN ';'?
    | 'context_aware' '=' BOOLEAN ';'?
    | 'proactive_assist' '=' BOOLEAN ';'?
    ;

discoveryBody
    : ideDetection? proactiveIntro? contextualAssist?
    ;

ideDetection
    : 'detect_ide' '=' ('VSCode' | 'IntelliJ' | 'Vim' | 'Emacs' | 'Eclipse' | 'ZBE')
    ;

proactiveIntro
    : 'introduce' STRING
    ;

contextualAssist
    : 'assist' '{' assistBody '}'
    ;

assistBody
    : 'trigger' '=' ('file_type' | 'language_server' | 'project_structure')
    | 'action' '=' STRING
    ;

developerAnalyticsDecl
    : 'developer_analytics' '{' analyticsConfig* '}'
    | 'analytics' '{' analyticsBody '}'
    ;

analyticsConfig
    : 'track_developers' '=' BOOLEAN ';'?
    | 'track_companies' '=' BOOLEAN ';'?
    | 'track_countries' '=' BOOLEAN ';'?
    | 'track_licenses' '=' BOOLEAN ';'?
    | 'track_usage' '=' BOOLEAN ';'?
    ;

analyticsBody
    : developerCount? companyCount? countryList? licenseData? versionData?
    ;

developerCount
    : 'developers' '=' INTEGER
    ;

companyCount
    : 'companies' '=' INTEGER
    ;

countryList
    : 'countries' '=' '[' stringList ']'
    ;

licenseData
    : 'licenses' '{' licenseEntry* '}'
    ;

licenseEntry
    : 'license' ident '=' STRING
    ;

versionData
    : 'versions' '{' versionEntry* '}'
    ;

versionEntry
    : 'version' ident 'deployed' 'by' ident
    ;

licenseTrackingDecl
    : 'license' ident '{' licenseBody '}'
    ;

licenseBody
    : ('developer' ident ';'?)* ('company' ident ';'?)* ('country' STRING ';'?)* ('type' STRING ';'?)* ('expires' STRING ';'?)?
    ;

deploymentDecl
    : 'deploy' ident '{' deploymentBody '}'
    ;

deploymentBody
    : ('target' '=' STRING)? ('version' '=' STRING)? ('release' '=' 'true')?
    ;

versionReleaseDecl
    : 'release' ident '{' releaseBody '}'
    ;

releaseBody
    : ('version' '=' STRING)? ('changelog' '=' STRING)? ('rollback' '=' 'true')?
    ;

lspServerDecl
    : 'lsp' '{' lspBody '}'
    ;

lspBody
    : ('completion' '=' 'true')? ('diagnostics' '=' 'true')? ('go_to_def' '=' 'true')? ('refactoring' '=' 'true')? ('hover' '=' 'true')?
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Higher-Kinded Types & Type Classes
// ═══════════════════════════════════════════════════════════════════════════════

higherKindedTypeDecl
    : 'hkt' '<' typeParam '.' typeExpr '>'
    ;

typeClassDecl
    : 'typeclass' ident typeParams? '{' typeClassBody '}'
    ;

typeClassBody
    : (typeClassMethod | typeClassAssocType)*
    ;

typeClassMethod
    : 'fn' ident '(' params? ')' ('->' typeExpr)? ';'
    ;

typeClassAssocType
    : 'associated' 'type' ident (':' typeConstraint)? ';'
    ;

typeClassInstance
    : 'instance' typeParams? ident 'for' typeExpr '{' typeClassInstanceBody '}'
    ;

typeClassInstanceBody
    : functionDecl*
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Self-Adjustment & Self-Versioning
// ═══════════════════════════════════════════════════════════════════════════════

selfAdjustDecl
    : 'self_adjust' ident '{' selfAdjustBody '}'
    ;

selfAdjustBody
    : (adjustmentRule | adjustmentLogic)*
    ;

adjustmentRule
    : 'rule' ident '{' 'when' expression 'then' expression '}'
    ;

adjustmentLogic
    : 'logic' ident '{' statement* '}'
    ;

selfVersioningDecl
    : 'version' ident '{' versioningBody '}'
    | 'self_version' ident '{' versioningBody '}'
    ;

versioningBody
    : (versionRecord | versionChangelog)*
    ;

versionRecord
    : 'record' ident '{' versionRecordEntry* '}'
    ;

versionRecordEntry
    : 'version' ident 'created' 'by' ident 'at' STRING ';'
    ;

versionChangelog
    : 'changelog' ident '{' changelogEntry* '}'
    ;

changelogEntry
    : 'change' ident 'made' 'by' ident 'at' STRING ';'
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Additional AI / ML / Domain Blocks
// ═══════════════════════════════════════════════════════════════════════════════

quantumMlBlock
    : 'quantum_ml' ident '{' statement* '}'
    ;

explainableRlBlock
    : 'explainable_rl' ident '{' statement* '}'
    ;

explainableDeepLearningBlock
    : 'explainable_deep_learning' ident '{' statement* '}'
    ;

knowledgeGraphBlock
    : 'knowledge_graph' ident '{' statement* '}'
    ;

probabilisticGraphicalModelBlock
    : 'probabilistic_graphical_model' ident '{' statement* '}'
    ;

transferLearningBlock
    : 'transfer_learning' ident '{' statement* '}'
    ;

multiAgentBlock
    : 'multi_agent' ident '{' statement* '}'
    ;

autonomousSystemBlock
    : 'autonomous_system' ident '{' statement* '}'
    ;

graphModelingBlock
    : 'graph' ident '{' statement* '}'
    ;

advancedNlpBlock
    : 'nlp' expression ';'
    ;

cognitiveArchitectureBlock
    : 'cognitive' ident '{' statement* '}'
    ;

aiForBusinessBlock
    : 'ai_for_business' ident '{' statement* '}'
    ;

vrArInteractionBlock
    : 'vr_ar_interaction' ident '{' statement* '}'
    ;

imageVideoAnalysisBlock
    : 'image_video_analysis' ident '{' statement* '}'
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Tensor / Matrix / Vector / ML / Robotics / Media
// ═══════════════════════════════════════════════════════════════════════════════

tensorDecl
    : 'tensor' '[' dimensionList ']' ident ';'
    ;

dimensionList
    : dimension (',' dimension)*
    ;

dimension
    : 'dim' '=' INTEGER
    | 'dimension' '=' INTEGER
    | 'dimensions' '=' '[' integerList ']'
    ;

matrixDecl
    : 'matrix' '[' INTEGER 'x' INTEGER ']' ident ';'
    ;

vectorDecl
    : 'vector' '[' INTEGER ']' ident ';'
    | 'vector_space' '{' vectorSpaceBody '}'
    ;

vectorSpaceBody
    : ('vector' ident | 'function' ident)*
    ;

mlModelDecl
    : 'model' ident '{' mlModelBody '}'
    ;

mlModelBody
    : 'train' 'on' expression
    | 'transform' 'with' expression
    | 'evaluate' 'with' expression
    ;

musicDecl
    : 'music' ident '{' musicBody '}'
    ;

musicBody
    : 'melody' '=' STRING
    | 'harmony' '=' STRING
    | 'tempo' '=' INTEGER
    | 'key' '=' STRING
    ;

roboticsDecl
    : 'robot' ident '{' roboticsBody '}'
    ;

roboticsBody
    : 'actuator' '=' STRING
    | 'sensor' '=' STRING
    | 'control_loop' '=' STRING
    ;

deepLearningDecl
    : 'dl' ident '{' dlBody '}'
    ;

dlBody
    : 'nn' ident '{' nnBody '}'
    ;

nnBody
    : 'layer' ident '{' layerBody '}'
    ;

layerBody
    : 'activation' '=' STRING
    | 'neurons' '=' INTEGER
    | 'dropout' '=' expression
    ;

graphicsDecl
    : 'graphics' ident '{' graphicsBody '}'
    ;

graphicsBody
    : 'render' '=' STRING
    | 'shader' '=' STRING
    | 'frame' '=' INTEGER
    ;

videoDecl
    : 'video' ident '{' videoBody '}'
    ;

videoBody
    : 'codec' '=' STRING
    | 'resolution' '=' STRING
    | 'fps' '=' INTEGER
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Data Parallelism & Concurrency
// ═══════════════════════════════════════════════════════════════════════════════

dataParallelismDecl
    : 'parallel' 'data' ident '{' statement* '}'
    ;

concurrentDataStructureDecl
    : 'concurrent' 'data' ident '{' classMember* '}'
    ;

messageHandlerDecl
    : 'handler' ident '(' params? ')' blockExpr
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Human Interface Devices
// ═══════════════════════════════════════════════════════════════════════════════

hidManager
    : 'hid' ident '{' hidBody '}'
    ;

hidBody
    : (hidDeviceSpec | hidGesture | hidBci | hidEyeTracking | hidTouch | hidHaptic)*
    ;

hidDeviceSpec
    : 'device' '=' ('GUI' | 'CLI' | 'VCI' | 'GESTURE' | 'BCI' | 'EYE_TRACKING' | 'TOUCH' | 'HAPTIC')
    ;

hidGesture
    : 'gesture' '{' gestureBody '}'
    ;

gestureBody
    : 'sign_language' '=' ('ASL' | 'BSL' | 'CSL')
    | 'tracking' '=' STRING
    ;

hidBci
    : 'bci' '{' bciBody '}'
    ;

bciBody
    : 'neural_command' '=' STRING
    | 'neural_feedback' '=' STRING
    ;

hidEyeTracking
    : 'eye_tracking' '{' eyeBody '}'
    ;

eyeBody
    : 'gaze_point' '=' expression
    | 'dwell_time' '=' expression
    ;

hidTouch
    : 'touch' '{' touchBody '}'
    ;

touchBody
    : 'multi_touch' '=' 'true'
    | 'pressure' '=' expression
    ;

hidHaptic
    : 'haptic' '{' hapticBody '}'
    ;

hapticBody
    : 'pattern' '=' STRING
    | 'intensity' '=' expression
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Helper Rules
// ═══════════════════════════════════════════════════════════════════════════════

ident
    : IDENT
    | THIS
    | SELF_LOWER
    | SELF_UPPER
    | INT_KW
    | FLOAT_KW
    | BOOL_KW
    | STR_KW
    | STRING_KW
    | CHAR_KW
    | VOID
    | QUANTUM
    | NANO
    | AGENT
    | CIRCUIT
    | EFFECT
    | HANDLE
    | REMEMBER
    | RECALL
    | LEARN
    | INFER
    | WISDOM
    | ZAMANI
    | SASA
    | ANCESTOR
    | LINEAR
    | AFFINE
    | LANGUAGE
    | MTS_KW
    | LEN
    | PRINT
    | PRINTLN
    | ASSERT
    | PANIC
    ;

identList
    : ident (',' ident)*
    ;

stringList
    : STRING (',' STRING)*
    ;

integerList
    : INTEGER (',' INTEGER)*
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Lexer Rules
// ═══════════════════════════════════════════════════════════════════════════════

// ── Keywords (must precede IDENT) ─────────────────────────────────────────────
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
EXPORT: 'export'; GLOBAL: 'global';

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

// Extended keywords from ZAMANI_GRAMMAR.md
HADAMARD: 'Hadamard'; CNOT: 'CNOT'; PAULIX: 'PauliX'; PAULIY: 'PauliY'; PAULIZ: 'PauliZ';
TOFFOLI: 'Toffoli'; FREDKIN: 'Fredkin';
ATOM: 'atom'; MOLECULE: 'molecule'; ASSEMBLE: 'assemble'; DEPLOY: 'deploy';
SUPERPOSE: 'superpose'; ENTANGLE: 'entangle'; MEASURE: 'measure'; RESET: 'reset';
BARRIER: 'barrier'; FORK: 'fork'; MERGE: 'merge'; OBSERVE: 'observe'; REWIND: 'rewind';
PARALLEL: 'parallel'; SPECULATIVE: 'speculative'; COUNTERFACTUAL: 'counterfactual';
CONSENSUS: 'consensus'; VOTE: 'vote'; ANCESTRAL: 'ancestral';
PI: 'Pi'; SIGMA: 'Sigma'; ID: 'Id'; TYPE0: 'Type_0'; TYPE1: 'Type_1'; TYPE2: 'Type_2'; TYPEN: 'Type_N';
KIND: 'Kind'; SORT: 'Sort'; PROP: 'Prop'; SESSION: 'session'; SEND: 'send'; RECV: 'recv'; OFFER: 'offer';
CHOICE: 'choice'; CLOSE: 'close'; HKT: 'hkt'; EXISTS: 'exists'; SINGLETON: 'singleton';
COGNITIVESTATE: 'CognitiveState'; CONSCIOUSNESS: 'Consciousness'; BIOLOGICAL: 'Biological';
NEURAL: 'Neural'; MEMORYBANK: 'MemoryBank'; AGENTTYPE: 'AgentType';
NARROWAI: 'NarrowAI'; AGI_KW: 'AGI'; ASI_KW: 'ASI'; AESI_KW: 'AESI'; ASESI_KW: 'ASESI';
META: 'meta'; INVOKE: 'invoke'; TRANSCODE: 'transcode'; PLUGIN: 'plugin'; TRANSPILER: 'transpiler';
MACRO: 'macro'; REFLECT: 'reflect'; INTROSPECT: 'introspect'; META_EVAL: 'meta_eval';
QUOTE: 'quote'; UNQUOTE: 'unquote'; SPLICE: 'splice'; DOMAIN: 'domain'; ASPECT: 'aspect';
PROVIDER: 'provider'; CACHE: 'cache'; INJECT: 'inject'; METADATA: 'metadata';
COMPOSE: 'compose'; WEAVE: 'weave'; DEPENDS: 'depends';

// HDL / Cloud / Distributed
HDL: 'hdl'; PORT: 'port'; INPUT: 'input'; OUTPUT: 'output'; INOUT: 'inout';
WIRE: 'wire'; REG: 'reg'; LOGIC: 'logic'; GATE: 'gate'; AND: 'AND'; OR: 'OR'; NOT: 'NOT';
XOR: 'XOR'; NAND: 'NAND'; NOR: 'NOR'; MUX: 'MUX'; DEMUX: 'DEMUX'; SIGNAL: 'signal'; ASSIGN: 'assign';
DISTRIBUTED: 'distributed'; NODE: 'node'; SERVICE: 'service'; AT: 'at'; REMOTE: 'remote';
TELEPORT: 'teleport'; MIGRATE: 'migrate'; DSM: 'dsm'; DMTS: 'dmts';
AWS: 'AWS'; AZURE: 'Azure'; GCP: 'GCP'; DOCKER: 'Docker'; JENKINS: 'Jenkins'; GIT: 'Git'; ANSIBLE: 'Ansible';
DATABASE: 'Database'; HTTP: 'HTTP'; SERIALIZE: 'serialize'; DESERIALIZE: 'deserialize';
JSON: 'json'; XML: 'xml'; MESSAGEPACK: 'messagepack'; PROTOBUF: 'protobuf'; CBOR: 'cbor';
STREAM: 'stream'; PIPE: 'pipe'; FOREIGN: 'foreign';

// Crypto
ENCRYPT: 'encrypt'; DECRYPT: 'decrypt'; ENCRYPT_HOMOMORPHIC: 'encrypt_homomorphic';
DECRYPT_HOMOMORPHIC: 'decrypt_homomorphic'; ENCRYPT_LAYERED: 'encrypt_layered';
ENCRYPT_QUANTUM_SAFE: 'encrypt_quantum_safe'; AES256: 'AES256'; CHACHA20: 'ChaCha20';
RSA4096: 'RSA4096'; KYBER: 'Kyber'; DILITHIUM: 'Dilithium'; SPHINCS: 'SPHINCS+';
GENERATE_ZK_PROOF: 'generate_zk_proof'; VERIFY_ZK_PROOF: 'verify_zk_proof';
SECURE_MULTI_PARTY_COMPUTE: 'secure_multi_party_compute'; REQUEST_KEY: 'request_key';
ROTATE_KEY: 'rotate_key'; REVOKE_KEY: 'revoke_key'; GENERATE_KEY: 'generate_key';

// On-device / Edge
ON_DEVICE: 'on_device'; ON_DEVICE_AGENT: 'on_device_agent'; REQUIRES: 'requires';
MEMORY: 'memory'; POWER: 'power'; STORAGE: 'storage'; PROCESSOR: 'processor';
SELF_PRESERVE: 'self_preserve'; SELF_PRESERVATION: 'self_preservation'; OFFLINE: 'offline';
LOCAL_RESOURCES: 'local_resources'; LOCAL_MTS: 'local_mts'; LIFECYCLE: 'lifecycle';
MONITOR: 'monitor'; UPDATE: 'update'; MANAGE: 'manage';

// Self-evolution
SELF_EVOLVE: 'self_evolve'; OPTIMIZE: 'optimize'; PATCH: 'patch'; VERIFY: 'verify';
AUTONOMOUSLY_GENERATE: 'autonomously_generate'; AUTONOMOUSLY_OPTIMIZE: 'autonomously_optimize';
AUTONOMOUSLY_VERIFY: 'autonomously_verify';

// Optimization / Target
OPTIMIZATION: 'optimization'; PASS: 'pass'; TARGET: 'target'; STRATEGY: 'strategy';
CLASSICAL: 'classical'; NEUROMORPHIC: 'neuromorphic'; GPU: 'GPU'; FPGA_KW: 'FPGA';
SIMD: 'SIMD'; WASM: 'WASM'; USSD: 'USSD'; EDGE: 'edge'; STELLAR: 'stellar';
DCE: 'DCE'; CSE: 'CSE'; INLINING: 'inlining'; LOOP_UNROLL: 'loop_unroll';
CONSTANT_FOLD: 'constant_fold'; QUANTUM_GATE_OPT: 'quantum_gate_opt';
NANO_ATP_EFFICIENCY: 'nano_atp_efficiency'; AI_QUANTIZATION: 'ai_quantization';
THERMAL_THROTTLE_PREVENTION: 'thermal_throttle_prevention'; SIMD_VECTORIZATION: 'simd_vectorization';
GPU_OFFLOADING: 'gpu_offloading'; QUANTUM_ERROR_CORRECTION: 'quantum_error_correction';
ARM64: 'ARM64'; RISCV: 'RISC-V'; LLVM_IR: 'LLVM_IR'; BARE_METAL: 'bare_metal';
ANDROID: 'Android'; IOS: 'iOS'; CLOUD: 'cloud'; IOT: 'IoT';
TARIRO_RUNTIME: 'Tariro_Runtime'; Z_MMP: 'Z_MMP';

// Runtime
RUNTIME: 'runtime'; CONFIGURE: 'configure'; GC: 'gc'; ENABLED: 'enabled';
DISABLED: 'disabled'; HYBRID: 'hybrid'; SELF_HEAL: 'self_heal'; QUANTUM_SIM: 'quantum_sim';
NANO_ORCHESTRATION: 'nano_orchestration'; EFFECT_DISPATCH: 'effect_dispatch';
EAGER: 'eager'; LAZY: 'lazy'; BATCHED: 'batched'; SCHEDULER: 'scheduler';
PREEMPTIVE: 'preemptive'; COOPERATIVE: 'cooperative'; CHANNEL: 'channel'; SELECT: 'select';

// Actor
ACTOR: 'actor'; MESSAGE: 'message'; BEHAVIOR: 'behavior'; STATE_KW: 'state';
CONCURRENT: 'concurrent';

// AI / Cognitive
COGNITIVE_ARCHITECTURE: 'cognitive_architecture'; MACHINE_LEARNING: 'machine_learning';
DEEP_LEARNING: 'deep_learning'; NEURAL_NETWORK: 'neural_network';
GRAPH_BASED_DEEP_LEARNING: 'graph_based_deep_learning'; CAUSAL_DISCOVERY: 'causal_discovery';
CAUSAL_REASONING: 'causal_reasoning'; PROBABILISTIC_MODELING: 'probabilistic_modeling';
ADVANCED_COMPUTER_VISION: 'advanced_computer_vision'; PREDICTIVE_ANALYTICS: 'predictive_analytics';
PRESCRIPTIVE_ANALYTICS: 'prescriptive_analytics'; DATA_VISUALIZATION: 'data_visualization';
HUMAN_AI_COLLABORATION: 'human_ai_collaboration'; ADVANCED_HUMAN_AI_COLLABORATION: 'advanced_human_ai_collaboration';
EXPLAINABILITY_AND_TRANSPARENCY: 'explainability_and_transparency'; SAFETY_AND_SECURITY: 'safety_and_security';
ETHICS_AND_GOVERNANCE: 'ethics_and_governance'; QUANTUM_MACHINE_LEARNING: 'quantum_machine_learning';
COGNITIVE_ARCHITECTURES: 'cognitive_architectures'; HUMAN_COMPUTER_INTERACTION: 'human_computer_interaction';
DATA_ANALYTICS: 'data_analytics'; TIME_SERIES_ML: 'time_series_ml';
TIME_SERIES_FORECASTING: 'time_series_forecasting'; TIME_SERIES_ANALYSIS: 'time_series_analysis';
TEXT_GENERATION: 'text_generation'; SENTIMENT_ANALYSIS: 'sentiment_analysis';
LANGUAGE_TRANSLATION: 'language_translation'; OBJECT_DETECTION: 'object_detection';
OBJECT_RECOGNITION: 'object_recognition'; IMAGE_CLASSIFICATION: 'image_classification';
AUTONOMOUS_NAVIGATION: 'autonomous_navigation'; AUTONOMOUS_ROBOTICS: 'autonomous_robotics';
AUTONOMOUS_SYSTEMS: 'autonomous_systems'; MODEL_EXPLAINABILITY: 'model_explainability';
AI_SAFETY_AND_SECURITY: 'ai_safety_and_security'; AI_ETHICS_AND_GOVERNANCE: 'ai_ethics_and_governance';
QUANTUM_OPTIMIZATION: 'quantum_optimization'; RECOMMENDATION_SYSTEMS: 'recommendation_systems';
QUANTUM_COMPUTING: 'quantum_computing'; BLOCKCHAIN_TECHNOLOGY: 'blockchain_technology';
IOT_KW: 'iot'; CYBERSECURITY: 'cybersecurity'; DATA_SCIENCE: 'data_science';
NATURAL_LANGUAGE_GENERATION: 'natural_language_generation'; NL_GENERATION: 'nl_generation';
ADVANCED_ROBOTICS: 'advanced_robotics'; TRANSFER_LEARNING: 'transfer_learning';
EXPLAINABLE_REINFORCEMENT_LEARNING: 'explainable_reinforcement_learning';
GRAPH_NEURAL_NETWORK: 'graph_neural_network';
ADVANCED_NATURAL_LANGUAGE_PROCESSING: 'advanced_natural_language_processing';
ADVANCED_NATURAL_LANGUAGE_GENERATION: 'advanced_natural_language_generation';
ADVANCED_COMPUTER_VISION_FOR_OBJECT_RECOGNITION: 'advanced_computer_vision_for_object_recognition';
ADVANCED_COMPUTER_VISION_FOR_OBJECT_DETECTION: 'advanced_computer_vision_for_object_detection';
ADVANCED_COMPUTER_VISION_FOR_IMAGE_CLASSIFICATION: 'advanced_computer_vision_for_image_classification';
ADVANCED_ROBOTICS_FOR_AUTONOMOUS_SYSTEMS: 'advanced_robotics_for_autonomous_systems';
ADVANCED_ROBOTICS_FOR_AUTONOMOUS_NAVIGATION: 'advanced_robotics_for_autonomous_navigation';
ADVANCED_EXPLAINABILITY_AND_TRANSPARENCY_FOR_AI_MODELS: 'advanced_explainability_and_transparency_for_ai_models';
ADVANCED_SAFETY_AND_SECURITY_FOR_AI_SYSTEMS: 'advanced_safety_and_security_for_ai_systems';
ADVANCED_ETHICS_AND_GOVERNANCE_FOR_AI_DEVELOPMENT: 'advanced_ethics_and_governance_for_ai_development';
ADVANCED_QUANTUM_COMPUTING_FOR_OPTIMIZATION_PROBLEMS: 'advanced_quantum_computing_for_optimization_problems';
ADVANCED_QUANTUM_COMPUTING_FOR_MACHINE_LEARNING: 'advanced_quantum_computing_for_machine_learning';
ADVANCED_CAUSAL_REASONING_FOR_DECISION_MAKING: 'advanced_causal_reasoning_for_decision_making';
ADVANCED_COGNITIVE_ARCHITECTURES_FOR_HUMAN_AI_COLLABORATION: 'advanced_cognitive_architectures_for_human_ai_collaboration';
ADVANCED_MACHINE_LEARNING_FOR_RECOMMENDATION_SYSTEMS: 'advanced_machine_learning_for_recommendation_systems';
ADVANCED_NLP_FOR_SENTIMENT_ANALYSIS: 'advanced_nlp_for_sentiment_analysis';
ADVANCED_NLP_FOR_TEXT_GENERATION: 'advanced_nlp_for_text_generation';
ADVANCED_NLP_FOR_LANGUAGE_TRANSLATION: 'advanced_nlp_for_language_translation';
ADVANCED_MACHINE_LEARNING_FOR_TIME_SERIES_DATA: 'advanced_machine_learning_for_time_series_data';
ADVANCED_MACHINE_LEARNING_FOR_TIME_SERIES_FORECASTING: 'advanced_machine_learning_for_time_series_forecasting';
ADVANCED_MACHINE_LEARNING_FOR_TIME_SERIES_ANALYSIS: 'advanced_machine_learning_for_time_series_analysis';
GOAL_KW: 'goal'; CONSTRAINT: 'constraint'; CAPABILITY: 'capability';
EXPLAIN: 'explain'; TRANSPARENT: 'transparent'; DECISION: 'decision'; MADE: 'made'; BY: 'by'; REASON: 'reason';
LOG: 'log'; NARROW: 'narrow'; GENERAL: 'general'; SUPER: 'super'; LEARNING: 'learning';
SELF_IMPROVEMENT: 'self_improvement'; TRANSFORMATION: 'transformation'; OMNIPOTENCE: 'omnipotence';

// Safety / Governance
EVAS: 'evas'; SAFETY: 'safety'; SECURITY: 'security'; ETHICS: 'ethics'; GOVERNANCE: 'governance';
COMPLIANCE: 'compliance'; STANDARD: 'standard'; CERTIFIED_BY: 'certified_by';
GREEN: 'green'; MINIMIZE_POWER: 'minimize_power'; MINIMIZE_HEAT: 'minimize_heat';
MINIMIZE_WATER: 'minimize_water'; MAXIMIZE_EFFICIENCY: 'maximize_efficiency';

// Administration
ADMIN: 'admin'; DISPLAY: 'display'; CHANGES: 'changes'; INPUT: 'input'; SUGGESTIONS: 'suggestions';
SUGGESTION: 'suggestion'; PAYMENT: 'payment'; METHOD: 'method'; VERIFY: 'verify'; FEEDBACK: 'feedback';
VALIDATE: 'validate'; COPYRIGHT: 'copyright'; OWNED: 'owned'; FEATURE: 'feature'; DEFINE: 'define';
CUSTOMIZE: 'customize'; PROGRAM_ONCE: 'program_once'; SYSTEM_CONFIG: 'config'; SYSTEM_LOGIC: 'logic';
MALICIOUS: 'malicious'; IDEA: 'idea'; DETECTION: 'detection'; ANALYZE: 'analyze'; BLOCK: 'block';
USER: 'user'; IDENTIFY: 'identify'; LEGAL: 'legal'; ACTION: 'action'; PROCEEDING: 'proceeding';
NOTICE: 'notice'; SANDBOX: 'sandbox'; SIMULATE: 'simulate'; TEST: 'test';

// Omniversal
OMNIVERSAL_SIMULATE: 'omniversal_simulate'; OMNIVERSAL_SYNTHESIZE: 'omniversal_synthesize';
OMNIVERSAL_DEPLOY: 'omniversal_deploy'; OMNIVERSAL_ALIGNMENT: 'omniversal_alignment';
OMNIVERSAL_CONTAINMENT: 'omniversal_containment'; OMNIVERSAL_TRUST: 'omniversal_trust';
OMNIVERSAL_KNOWLEDGE: 'omniversal_knowledge'; OMNIVERSAL_GENERATE: 'omniversal_generate';
OMNIVERSAL_SOVEREIGNTY: 'omniversal_sovereignty'; OMNIVERSAL_GOAL: 'omniversal_goal';
OMNIVERSAL_BIONANO: 'omniversal_bionano'; OMNIVERSAL_REALITY: 'omniversal_reality';
OMNIVERSAL_NLP: 'omniversal_nlp'; MULTIVERSE: 'multiverse'; UNIVERSES: 'universes';
UNIVERSE: 'universe'; PHYSICS: 'physics'; RULES: 'rules'; INITIAL_STATE: 'initial_state';

// Conversational / NL
CHAT_AGENT: 'chat_agent'; CONTEXT: 'context'; SYNTHESIZE: 'synthesize';
NL_GENERATE: 'nl_generate'; NL_INTERPRET: 'nl_interpret'; NL_TRANSLATE: 'nl_translate';
DOCUMENT: 'document'; FORMAT: 'format'; BOOK: 'book'; ARTICLE: 'article'; REPORT: 'report';
JOURNAL: 'journal'; NEWS: 'news'; INTERACTIVE_WEB: 'interactive_web'; CONTENT: 'content';
MULTIMODAL: 'multimodal'; DIAGRAMS: 'diagrams'; IMAGES: 'images'; VIDEO: 'video'; INTERACTIVE: 'interactive';

// Green / Energy
THERMAL_OPTIMIZE: 'thermal_optimize'; THERMAL: 'thermal'; THRESHOLD: 'threshold';
MAX_TEMP: 'max_temp'; PREVENT_THROTTLING: 'prevent_throttling'; CONSERVE: 'conserve';
CARBON_FOOTPRINT: 'carbon_footprint'; ENERGY_AWARE: 'energy_aware'; MINIMIZE_CARBON: 'minimize_carbon';
AOT: 'AOT'; JIT: 'JIT'; NANO_COMPILE: 'nano_compile'; EDGE_COMPILE: 'edge_compile';
DVFS: 'dvfs'; CLOCK: 'clock'; VOLTAGE: 'voltage'; CLOUD_ENERGY: 'cloud_energy';
RENEWABLE: 'renewable'; PUE: 'pue'; DATA_CENTER: 'data_center'; MATCH_RESOURCES: 'match_resources';
QPU: 'QPU'; NACU: 'NACU';

// Developer Relations
SELF_DISCOVER: 'self_discover'; IDE_DETECT: 'ide_detect'; LANGUAGE_SERVER: 'language_server';
AUTO_SUGGEST: 'auto_suggest'; CONTEXT_AWARE: 'context_aware'; PROACTIVE_ASSIST: 'proactive_assist';
DETECT_IDE: 'detect_ide'; VSCODE: 'VSCode'; INTELLIJ: 'IntelliJ'; VIM: 'Vim'; EMACS: 'Emacs'; ECLIPSE: 'Eclipse'; ZBE: 'ZBE';
INTRODUCE: 'introduce'; ASSIST: 'assist'; TRIGGER: 'trigger'; FILE_TYPE: 'file_type'; PROJECT_STRUCTURE: 'project_structure';
DEVELOPER_ANALYTICS: 'developer_analytics'; ANALYTICS: 'analytics'; TRACK_DEVELOPERS: 'track_developers';
TRACK_COMPANIES: 'track_companies'; TRACK_COUNTRIES: 'track_countries'; TRACK_LICENSES: 'track_licenses';
TRACK_USAGE: 'track_usage'; DEVELOPERS: 'developers'; COMPANIES: 'companies'; COUNTRIES: 'countries';
LICENSES: 'licenses'; VERSIONS: 'versions'; DEPLOYED: 'deployed'; DEPLOY: 'deploy'; RELEASE: 'release';
CHANGELOG: 'changelog'; ROLLBACK: 'rollback'; LSP: 'lsp'; COMPLETION: 'completion'; DIAGNOSTICS: 'diagnostics';
GO_TO_DEF: 'go_to_def'; REFACTORING: 'refactoring'; HOVER: 'hover'; EXPIRES: 'expires';

// Higher-kinded / Type Classes
TYPECLASS: 'typeclass'; ASSOCIATED: 'associated'; INSTANCE: 'instance'; FOR: 'for';

// Self-adjustment
SELF_ADJUST: 'self_adjust'; WHEN: 'when'; THEN: 'then'; SELF_VERSION: 'self_version';
CREATED: 'created'; ON: 'on'; AT: 'at'; RECORD: 'record';

// Additional
TENSOR: 'tensor'; MATRIX: 'matrix'; VECTOR: 'vector'; VECTOR_SPACE: 'vector_space';
MODEL: 'model'; TRAIN: 'train'; EVALUATE: 'evaluate'; TRANSFORM: 'transform';
MUSIC: 'music'; MELODY: 'melody'; HARMONY: 'harmony'; TEMPO: 'tempo'; KEY: 'key';
ROBOT: 'robot'; ACTUATOR: 'actuator'; SENSOR: 'sensor'; CONTROL_LOOP: 'control_loop';
DL: 'dl'; NN: 'nn'; LAYER: 'layer'; ACTIVATION: 'activation'; NEURONS: 'neurons'; DROPOUT: 'dropout';
GRAPHICS: 'graphics'; RENDER: 'render'; SHADER: 'shader'; FRAME: 'frame'; CODEC: 'codec';
RESOLUTION: 'resolution'; FPS: 'fps'; HIGH_DIMENSIONAL: 'high_dimensional';
DIM: 'dim'; DIMENSION: 'dimension'; DIMENSIONS: 'dimensions';
INFINITY: 'infinity'; INFINITE: 'infinite'; PRECISION: 'precision'; SCALE: 'scale';
GENERATION: 'generation'; GENERATE: 'generate'; CODE: 'code'; TARGET: 'target';
PERFORM: 'perform'; TASK: 'task'; OPERATION: 'operation'; PRIORITY: 'priority'; DEADLINE: 'deadline';
ADD: 'add'; SUBTRACT: 'subtract'; MULTIPLY: 'multiply'; DIVIDE: 'divide';
INSERT: 'insert'; DELETE: 'delete'; CONNECT: 'connect'; TO: 'to'; RESUME: 'resume'; COORDINATE: 'coordinate';
VAR: 'var'; ARRAY: 'array'; LIST: 'list'; OBJECT: 'object'; PROPERTY: 'property'; HANDLER: 'handler';
POINT: 'point'; X: 'x'; Y: 'y'; ELEMENT: 'element'; BASIS: 'basis';
FILE_SCOPED: 'file'; REQUIRED: 'required'; INIT: 'init'; PARTIAL: 'partial'; SEALED: 'sealed';
FINAL: 'final'; EXPLICIT: 'explicit'; IMPLICIT: 'implicit'; EVENT: 'event'; DELEGATE: 'delegate';
GET: 'get'; SET: 'set'; OUT: 'out'; IN_KW: 'in'; DEFAULT: 'default'; WHERE: 'where';
FILE: 'file'; GLOBAL: 'global'; AS: 'as'; IS: 'is'; HAS: 'has'; REF: 'ref'; MOVE: 'move'; BOX: 'box';
TYPEOF: 'typeof'; SIZEOF: 'sizeof'; ALIGNOF: 'alignof'; OFFSETOF: 'offsetof'; INSTANCEOF: 'instanceof';
SIZE: 'size'; ALIGN: 'align'; OFFSET: 'offset'; TYPE_LEVEL: 'type'; KIND_KW: 'kind';
LINEARIZATION: 'linearization'; VARIANCE: 'variance'; MEMOIZE: 'memoize';
PATH_DEPENDENT: 'path'; EXISTENTIAL: 'existential'; TYPE_CONSTRUCTOR: 'type_constructor';
TYPE_FAMILY: 'type_family'; ASSOCIATED_TYPE: 'associated_type'; SELF_RECURSIVE: 'self_recursive';
BOUNDED_RECURSION: 'bounded_recursion'; BOUNDED: 'bounded'; RECURSION: 'recursion';
CONTEXT_DEPENDENT: 'context_dependent'; FUNCTIONAL_DEPENDENCY: 'functional_dependency';
TYPE_PROVIDER: 'type_provider'; TYPE_SAFE: 'type_safe'; METAPROGRAMMING: 'metaprogramming';
SQL: 'sql'; IMMUTABLE: 'immutable'; SAFE: 'safe'; POINTER: 'pointer'; VALUE: 'value';
DATA_STRUCTURE: 'data_structure'; PARALLEL: 'parallel'; CONCURRENT: 'concurrent';
MESSAGE_HANDLER: 'message_handler'; FUNCTION_COMPOSITION: 'function_composition';
DATA_INTERCHANGE: 'data_interchange'; SERIALIZE: 'serialize'; DESERIALIZE: 'deserialize';
RAW_PTR: 'raw_ptr'; SLICE: 'slice'; UNIT: 'unit'; NEVER: 'never'; OPTION: 'Option'; RESULT: 'Result';
OWNED: 'owned'; BORROWED: 'borrowed'; SHARED: 'shared'; UNIQUE: 'unique'; PIN: 'Pin';
FUTURE: 'Future'; STREAM: 'stream'; ITERATOR: 'iterator'; INTO_ITER: 'IntoIter';
FROM: 'from'; INTO: 'into'; TRY_FROM: 'try_from'; TRY_INTO: 'try_into'; DEFAULT_KW: 'default';
CLONE: 'clone'; COPY: 'copy'; DROP: 'drop'; SEND: 'Send'; SYNC: 'Sync'; STATIC_LIFETIME: 'static';
CONST_KW: 'const'; LET: 'let'; MUT: 'mut'; STATIC: 'static'; EXTERN: 'extern'; CRATE: 'crate';
SUPER: 'super'; SELF: 'self'; TRUE: 'true'; FALSE: 'false'; MATCH: 'match';
IF: 'if'; ELSE: 'else'; WHILE: 'while'; FOR: 'for'; LOOP: 'loop'; RETURN: 'return';
BREAK: 'break'; CONTINUE: 'continue'; YIELD: 'yield'; AWAIT: 'await'; ASYNC: 'async';
TRY: 'try'; CATCH: 'catch'; THROW: 'throw'; PANIC: 'panic'; ASSERT: 'assert';
UNSAFE: 'unsafe'; MOD: 'mod'; USE: 'use'; PUB: 'pub'; PRIV: 'priv'; PRIV2: 'private';
PROC: 'proc'; MACRO_RULES: 'macro_rules'; UNION: 'union'; TYPE: 'type'; ENUM: 'enum';
STRUCT: 'struct'; TRAIT: 'trait'; IMPL: 'impl'; FN: 'fn'; CONST: 'const'; STATIC: 'static';
LET: 'let'; REF: 'ref'; MUT: 'mut'; WHERE: 'where'; AS: 'as'; MOVE: 'move';
DYN: 'dyn'; ABSTRACT: 'abstract'; BECOME: 'become'; DO: 'do'; FINAL: 'final';
OVERRIDE: 'override'; PRIV: 'priv'; PRIV2: 'private'; PUB: 'pub'; TYPEOF: 'typeof';
UNSIZED: 'unsized'; VIRTUAL: 'virtual'; YIELD: 'yield'; BOX: 'box'; UNION: 'union';
TRAIT2: 'trait'; IMPL2: 'impl'; FN2: 'fn'; CONST2: 'const'; STATIC2: 'static';
LET2: 'let'; REF2: 'ref'; MUT2: 'mut'; WHERE2: 'where'; AS2: 'as'; MOVE2: 'move';
DYN2: 'dyn'; ABSTRACT2: 'abstract'; BECOME2: 'become'; DO2: 'do'; FINAL2: 'final';
OVERRIDE2: 'override'; PRIV3: 'priv'; PRIV4: 'private'; PUB2: 'pub'; TYPEOF2: 'typeof';
UNSIZED2: 'unsized'; VIRTUAL2: 'virtual'; YIELD2: 'yield'; BOX2: 'box'; UNION2: 'union';

// ── Literals ────────────────────────────────────────────────────────────
BOOLEAN: TRUE | FALSE;
NIL: NIL_KW | NULL_KW;

INTEGER: DIGIT+ | '0x' HEX_DIGIT+ | '0b' BIN_DIGIT+ | '0o' OCT_DIGIT+;
FLOAT: DIGIT+ '.' DIGIT+ ('e' ('+' | '-')? DIGIT+)?;

STRING: '"' (ESC | ~["\\])* '"';
CHAR: '\'' (ESC | ~['\\]) '\'';

fragment ESC: '\\' [nrt0"'\\];
fragment DIGIT: [0-9];
fragment HEX_DIGIT: [0-9a-fA-F];
fragment BIN_DIGIT: [01];
fragment OCT_DIGIT: [0-7];
fragment ALPHA: [a-zA-Z_];

// Zenith-native literals
QUANTUM_LITERAL: '|' ('0' | '1' | '+' | '-') '\\u27E9';
NANO_ANNOTATION: '@' IDENT ('(' ~[)]* ')')?;
MTS_LITERAL: 'mts' '[' ~[\\]]* ']';

// ── Identifiers ─────────────────────────────────────────────────────────
IDENT: ALPHA (ALPHA | DIGIT)*;

// ── Punctuation & operators ─────────────────────────────────────────────
LPAREN: '('; RPAREN: ')'; LBRACE: '{'; RBRACE: '}'; LBRACK: '['; RBRACK: ']';
COMMA: ','; DOT: '.'; SEMI: ';'; COLON: ':'; ARROW: '->'; FATARROW: '=>';
COLONCOLON: '::'; TILDE: '~'; HASH: '#'; AT: '@'; BANG: '!';

PLUS: '+'; MINUS: '-'; STAR: '*'; SLASH: '/'; PERCENT: '%'; ASSIGN: '=';
EQ: '=='; NEQ: '!='; LT: '<'; GT: '>'; LE: '<='; GE: '>=';
ANDAND: '&&'; OROR: '||'; AMP: '&'; PIPE: '|'; CARET: '^';
SHL: '<<'; SHR: '>>'; SHRU: '>>>';
PLUSEQ: '+='; MINUSEQ: '-='; STAREQ: '*='; SLASHEQ: '/=';
DOTDOT: '..'; DOTDOTEQ: '..=';
QUESTION: '?'; DOLLAR: '$';

// ── Whitespace & comments ───────────────────────────────────────────────
LINE_COMMENT: '//' ~[\r\n]* -> skip;
BLOCK_COMMENT: '/*' .*? '*/' -> skip;
DOC_COMMENT: '///' ~[\r\n]* -> skip;
WS: [ \t\r\n]+ -> skip;