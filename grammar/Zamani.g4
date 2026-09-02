// Zamani.g4 — Comprehensive grammar for the Zamani language
// Includes control flow, type system, functions, async/await, generics, and more

grammar Zamani;

program : docComment? statement* EOF ;

// Top-level statements and declarations
statement
    : importStatement
    | invokeStatement
    | transcodeStatement
    | overrideStatement
    | langStatement
    | moduleStatement
    | exportStatement
    | pluginStatement
    | foreignFunctionCall
    | dataStatement
    | streamingData
    | databaseOperation
    | webService
    | interfaceStatement
    | errorStmt
    | functionDeclaration
    | variableDeclaration
    | namespaceStatement
    | conditionalStatement
    | loopStatement
    | tryCatchFinally
    | matchStatement
    | annotationStatement
    | asyncStatement
    | ';'
    ;

// ==========================================================================
// Documentation & Annotations
// ==========================================================================

docComment: (DOC_COMMENT)+ ;

annotationStatement: annotation+ statement ;

annotation: '@' IDENTIFIER ('(' annotationValue? ')')? ;

annotationValue
    : literal
    | IDENTIFIER
    | annotation
    | annotationValue ',' annotationValue
    ;

// ==========================================================================
// Import & Export Statements
// ==========================================================================

importStatement
    : IMPORT IDENTIFIER ';'
    | IMPORT IDENTIFIER 'as' IDENTIFIER ';'
    | IMPORT '{' importList '}' FROM STRING ';'
    | IMPORT '*' 'as' IDENTIFIER FROM STRING ';'
    ;

importList: IDENTIFIER (',' IDENTIFIER)* ;

exportStatement
    : EXPORT IDENTIFIER TO IDENTIFIER ';'
    | EXPORT IDENTIFIER ';'
    | EXPORT '{' exportList '}' ';'
    | EXPORT '*' FROM STRING ';'
    ;

exportList: IDENTIFIER ('as' IDENTIFIER)? (',' IDENTIFIER ('as' IDENTIFIER)?)* ;

// ==========================================================================
// Namespace & Visibility
// ==========================================================================

namespaceStatement: NAMESPACE IDENTIFIER block ;

visibilityModifier: 'public' | 'private' | 'protected' | 'internal' ;

// ==========================================================================
// Function Declarations
// ==========================================================================

functionDeclaration: visibilityModifier? annotation* FUNCTION IDENTIFIER genericParameters? '(' parameterList? ')' ('->' type)? block ;

lambdaExpression: '|' parameterList? '|' '=>' expression ;

asyncStatement: ASYNC functionDeclaration ;

// ==========================================================================
// Variable Declarations
// ==========================================================================

variableDeclaration
    : varKeyword IDENTIFIER (':' type)? '=' expression ';'
    | varKeyword IDENTIFIER ':' type ';'
    ;

varKeyword: 'let' | 'const' | 'var' ;

// ==========================================================================
// Control Flow Statements
// ==========================================================================

conditionalStatement
    : IF expression block (ELSE IF expression block)* (ELSE block)?
    ;

loopStatement
    : WHILE expression block
    | DO block WHILE expression ';'
    | FOR '(' varKeyword IDENTIFIER ('in' | 'of') expression ')' block
    | FOR '(' varKeyword IDENTIFIER ':' type '=' expression ';' expression ';' expression ')' block
    ;

loopControl: BREAK ';' | CONTINUE ';' ;

matchStatement: MATCH expression '{' matchCase+ '}' ;

matchCase: CASE pattern '=>' (expression | block) ;

pattern
    : literal
    | IDENTIFIER
    | IDENTIFIER '{' patternList '}'
    | pattern '|' pattern
    ;

patternList: IDENTIFIER (':' pattern)? (',' IDENTIFIER (':' pattern)?)* ;

// ==========================================================================
// Error Handling
// ==========================================================================

tryCatchFinally
    : TRY block (catchClause+ finallyClause? | finallyClause)
    ;

catchClause: CATCH '(' IDENTIFIER (':' type)? ')' block ;

finallyClause: FINALLY block ;

errorStmt: 'error' STRING ';' ;

// ==========================================================================
// Invoke & Interop Statements
// ==========================================================================

invokeStatement: INVOKE IDENTIFIER '::' IDENTIFIER '(' argumentList? ')' ';' ;

transcodeStatement: TRANSCODE IDENTIFIER '::' STRING TO IDENTIFIER ';' ;

overrideStatement: OVERRIDE IDENTIFIER '::' IDENTIFIER '(' parameterList? ')' block ;

foreignFunctionCall: FOREIGN IDENTIFIER '::' IDENTIFIER '(' argumentList? ')' ';' ;

// ==========================================================================
// Language & Module Statements
// ==========================================================================

langStatement: LANG IDENTIFIER block ;

moduleStatement: MODULE IDENTIFIER ('::' IDENTIFIER)? ';' ;

// ==========================================================================
// Plugin System
// ==========================================================================

pluginStatement: PLUGIN IDENTIFIER '{' pluginDefinition* '}' ;

pluginDefinition
    : LANGUAGE IDENTIFIER ';'
    | TRANSPILER IDENTIFIER ';'
    ;

// ==========================================================================
// Data & Serialization
// ==========================================================================

dataStatement
    : SERIALIZE expression TO dataFormat ';'
    | DESERIALIZE expression FROM dataFormat ';'
    ;

dataFormat: 'json' | 'xml' | 'messagepack' | 'protobuf' | 'avro' ;

streamingData: STREAM expression 'pipe' expression (YIELD)? ';' ;

// ==========================================================================
// Database & Web Service Operations
// ==========================================================================

databaseOperation: DATABASE '::' IDENTIFIER '(' argumentList? ')' ';' ;

webService: HTTP '::' IDENTIFIER '(' argumentList? ')' ';' ;

// ==========================================================================
// Interface & Struct Definitions
// ==========================================================================

interfaceStatement: INTERFACE IDENTIFIER genericParameters? '{' interfaceDefinition* '}' ;

interfaceDefinition
    : visibilityModifier? 'method' IDENTIFIER genericParameters? '(' parameterList? ')' ('->' type)? ';'
    | visibilityModifier? 'property' IDENTIFIER type ';'
    ;

structStatement: 'struct' IDENTIFIER genericParameters? '{' structDefinition* '}' ;

structDefinition: visibilityModifier? IDENTIFIER ':' type ';' ;

// ==========================================================================
// Generic Parameters
// ==========================================================================

genericParameters: '<' genericParameter (',' genericParameter)* '>' ;

genericParameter: IDENTIFIER ('extends' type)? ;

// ==========================================================================
// Block
// ==========================================================================

block: '{' statement* '}' ;

// ==========================================================================
// Expressions
// ==========================================================================

expression
    : primary
    | expression '.' IDENTIFIER                                    // Property access
    | expression '?.' IDENTIFIER                                   // Optional property access
    | expression '[' expression ']'                                // Indexing
    | expression '[' expression '..' expression ']'               // Range indexing
    | expression '(' argumentList? ')'                             // Function call
    | expression '::' IDENTIFIER '(' argumentList? ')'            // Module invocation
    | '...' expression                                              // Spread operator
    | '++' expression | expression '++'                            // Increment
    | '--' expression | expression '--'                            // Decrement
    | expression op_pow expression                                 // Power
    | expression op_mul expression                                 // Multiplication
    | expression op_add expression                                 // Addition
    | expression op_cmp expression                                 // Comparison
    | expression op_eq expression                                  // Equality
    | expression op_logic expression                               // Logical
    | expression '?' expression ':' expression                     // Ternary
    | AWAIT expression                                              // Await
    | YIELD expression                                              // Yield
    | lambdaExpression                                             // Lambda
    | '(' expression ')'                                           // Grouping
    ;

primary
    : IDENTIFIER
    | literal
    | templateString
    | arrayLiteral
    | mapLiteral
    | tupleLiteral
    | structLiteral
    ;

templateString: '`' (templatePart | templateExpression)* '`' ;

templatePart: ~[`$\\]+ | '\\' . ;

templateExpression: '${' expression '}' ;

arrayLiteral: '[' (expression (',' expression)*)? ']' ;

mapLiteral: '{' (mapEntry (',' mapEntry)*)? '}' ;

mapEntry: expression ':' expression ;

tupleLiteral: '(' expression (',' expression)+ ')' ;

structLiteral: IDENTIFIER '{' (IDENTIFIER ':' expression (',' IDENTIFIER ':' expression)*)? '}' ;

// Operators
op_pow: '**' | '^' ;
op_mul: '*' | '/' | '%' ;
op_add: '+' | '-' ;
op_cmp: '<' | '>' | '<=' | '>=' ;
op_eq: '==' | '!=' ;
op_logic: '&&' | '||' | 'and' | 'or' ;

// ==========================================================================
// Types
// ==========================================================================

type
    : IDENTIFIER
    | genericType
    | 'void'
    | 'null'
    | 'array' '<' type '>'
    | 'map' '<' type ',' type '>'
    | 'tuple' '<' type (',' type)* '>'
    | type '?'                                        // Optional type
    | type '|' type                                   // Union type
    | '(' type ')'
    ;

genericType: IDENTIFIER '<' type (',' type)* '>' ;

// ==========================================================================
// Arguments & Parameters
// ==========================================================================

argumentList
    : argument (',' argument)*
    | argument (',' argument)* ',' '...' IDENTIFIER
    ;

argument
    : expression
    | IDENTIFIER ':' expression
    | '...' expression
    ;

parameterList: parameter (',' parameter)* ;

parameter
    : visibilityModifier? IDENTIFIER ':' type ('=' literal)?
    | '...' IDENTIFIER ':' type
    ;

// ==========================================================================
// Literals
// ==========================================================================

literal
    : INTEGER
    | DECIMAL
    | STRING
    | BOOLEAN
    | 'null'
    | 'undefined'
    ;

// ==========================================================================
// Lexer
// ==========================================================================

// Keywords
IMPORT: 'import' ;
INVOKE: 'invoke' ;
TRANSCODE: 'transcode' ;
OVERRIDE: 'override' ;
LANG: 'lang' ;
MODULE: 'module' ;
EXPORT: 'export' ;
TO: 'to' ;
FROM: 'from' ;
PLUGIN: 'plugin' ;
LANGUAGE: 'language' ;
TRANSPILER: 'transpiler' ;
SELF: 'self' ;
FOREIGN: 'foreign' ;
INTEROP: 'interop' ;
CDECL: 'cdecl' ;
STDCALL: 'stdcall' ;
JAVA: 'java' ;
GC: 'gc' ;
CAST: 'cast' ;
CONVERT: 'convert' ;
DATA: 'data' ;
SERIALIZE: 'serialize' ;
DESERIALIZE: 'deserialize' ;
STREAM: 'stream' ;
DATABASE: 'db' ;
HTTP: 'http' ;
INTERFACE: 'interface' ;
NAMESPACE: 'namespace' ;
FUNCTION: 'function' ;
IF: 'if' ;
ELSE: 'else' ;
WHILE: 'while' ;
DO: 'do' ;
FOR: 'for' ;
BREAK: 'break' ;
CONTINUE: 'continue' ;
MATCH: 'match' ;
CASE: 'case' ;
TRY: 'try' ;
CATCH: 'catch' ;
FINALLY: 'finally' ;
ASYNC: 'async' ;
AWAIT: 'await' ;
YIELD: 'yield' ;

// Common tokens
IDENTIFIER: [a-zA-Z_][a-zA-Z_0-9]* ;
INTEGER: [0-9]+ ;
DECIMAL: [0-9]+ '.' [0-9]+ ;
BOOLEAN: 'true' | 'false' ;

// STRING — double-quoted with escape support
STRING: '"' ( '\\' . | ~["\\] )* '"' ;

// TEMPLATE_STRING — backtick-quoted
TEMPLATE_STRING: '`' ( '\\' . | ~[`\\] )* '`' ;

// Comments
DOC_COMMENT: '///' ~[\r\n]* | '/**' .*? '*/' ;
LINE_COMMENT: '//' ~[\r\n]* -> skip ;
COMMENT: '/*' .*? '*/' -> skip ;

// Operators and punctuation
LPAREN: '(' ;
RPAREN: ')' ;
LBRACE: '{' ;
RBRACE: '}' ;
LBRACKET: '[' ;
RBRACKET: ']' ;
LT: '<' ;
GT: '>' ;
COMMA: ',' ;
SEMI: ';' ;
COLONCOLON: '::' ;
COLON: ':' ;
DOT: '.' ;
PIPE: '|' ;
QUESTION: '?' ;
ARROW: '=>' ;
FATARROW: '->' ;
ELLIPSIS: '...' ;
DOUBLESTAR: '**' ;
DOUBLEAND: '&&' ;
DOUBLEOR: '||' ;
EQEQ: '==' ;
BANGEQ: '!=' ;
DOTQUESTION: '?.' ;
PLUSPLUS: '++' ;
MINUSMINUS: '--' ;
PLUSEQ: '+=' ;
MINUSEQ: '-=' ;
STAREQ: '*=' ;
SLASHEQ: '/=' ;
PERCENTEQ: '%=' ;
CARET: '^' ;

// Whitespace
WS: [ \t\r\n]+ -> skip ;
