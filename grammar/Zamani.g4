// Zamani.g4 — Combined grammar for the Zamani language (updated)
// Merged selected lexical and syntactic features from the user's snippet.
// Platform-dependent features (cloud providers, containers, CI tools, etc.) were excluded per request.

grammar Zamani;

program : statement* EOF ;

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
    | ';'
    ;

// Import statement
importStatement: IMPORT IDENTIFIER ';' ;

// Invoke statement (module::func form)
invokeStatement: INVOKE IDENTIFIER '::' IDENTIFIER '(' argumentList? ')' ';' ;

// Transcode statement
transcodeStatement: TRANSCODE IDENTIFIER '::' STRING TO IDENTIFIER ';' ;

// Override (method override block)
overrideStatement: OVERRIDE IDENTIFIER '::' IDENTIFIER '(' parameterList? ')' block ;

// Lang statement (language-scoped block)
langStatement: LANG IDENTIFIER block ;

// Module statement
moduleStatement: MODULE IDENTIFIER ('::' IDENTIFIER)? ';' ;

// Export statement
exportStatement: EXPORT IDENTIFIER TO IDENTIFIER ';' ;

// Plugin statement
pluginStatement: PLUGIN IDENTIFIER '{' pluginDefinition* '}' ;

pluginDefinition
    : LANGUAGE IDENTIFIER ';'
    | TRANSPILER IDENTIFIER ';'
    ;

// Block
block: '{' statement* '}' ;

// Expressions (simple)
expression
    : IDENTIFIER
    | literal
    | invokeExpression
    | '(' expression ')'
    | expression op_mul expression
    | expression op_add expression
    | expression op_cmp expression
    | expression op_logic expression
    ;

op_mul: '*' | '/' | '%' ;
op_add: '+' | '-' ;
op_cmp: '==' | '!=' | '<' | '>' | '<=' | '>=' ;
op_logic: '&&' | '||' ;

invokeExpression: IDENTIFIER '::' IDENTIFIER '(' argumentList? ')' ;

argumentList: expression (',' expression)* ;

parameterList: parameter (',' parameter)* ;

parameter: type IDENTIFIER ;

// Basic type system snippet
type
    : IDENTIFIER
    | 'void'
    | 'array' '<' type '>'
    | 'map' '<' type ',' type '>'
    | '(' type ')'
    ;

literal
    : INTEGER
    | DECIMAL
    | STRING
    | BOOLEAN
    ;

// Foreign function interface
foreignFunctionCall: FOREIGN IDENTIFIER '::' IDENTIFIER '(' argumentList? ')' ';' ;

// Data serialization and deserialization
dataStatement
    : SERIALIZE expression TO dataFormat ';'
    | DESERIALIZE expression FROM dataFormat ';'
    ;

dataFormat: 'json' | 'xml' | 'messagepack' ;

// Streaming data
streamingData: STREAM expression 'pipe' expression ';' ;

// Database operations (generic DB token)
databaseOperation: DATABASE '::' IDENTIFIER '(' argumentList? ')' ';' ;

// Web services (HTTP as protocol)
webService: HTTP '::' IDENTIFIER '(' argumentList? ')' ';' ;

// Interface statement
interfaceStatement: INTERFACE IDENTIFIER '{' interfaceDefinition* '}' ;

interfaceDefinition
    : 'method' IDENTIFIER '(' parameterList? ')' ';'
    | 'property' IDENTIFIER type ';'
    ;

// Error handling
errorStmt: 'error' STRING ';' ;

// ==========================================================================
// Lexer
// ==========================================================================

IMPORT: 'import';
INVOKE: 'invoke';
TRANSCODE: 'transcode';
OVERRIDE: 'override';
LANG: 'lang';
MODULE: 'module';
EXPORT: 'export';
TO: 'to';
PLUGIN: 'plugin';
LANGUAGE: 'language';
TRANSPILER: 'transpiler';
SELF: 'self';
FOREIGN: 'foreign';
INTEROP: 'interop';
CDECL: 'cdecl';
STDCALL: 'stdcall';
JAVA: 'java';
GC: 'gc';
CAST: 'cast';
CONVERT: 'convert';
DATA: 'data';
SERIALIZE: 'serialize';
DESERIALIZE: 'deserialize';
STREAM: 'stream';
DATABASE: 'db';
HTTP: 'http';
// Excluded platform-specific tokens: AWS, AZURE, GCP, DOCKER, JENKINS, GIT, ANSIBLE
INTERFACE: 'interface';

// Common tokens
IDENTIFIER: [a-zA-Z_][a-zA-Z_0-9]* ;
INTEGER: [0-9]+ ;
DECIMAL: [0-9]+ '.' [0-9]+ ;
BOOLEAN: 'true' | 'false' ;

// STRING — double-quoted with escape support
STRING: '"' ( '\\' . | ~["\\] )* '"' ;

// Keywords that can also appear as parser literals (keep as tokens to avoid ambiguity)
TO: 'to';
FROM: 'from';

// Operators and punctuation
LPAREN: '(' ;
RPAREN: ')' ;
LBRACE: '{' ;
RBRACE: '}' ;
LT: '<' ;
GT: '>' ;
COMMA: ',' ;
SEMI: ';' ;
COLONCOLON: '::' ;
COLON: ':' ;
DOT: '.' ;
PIPE: '|' ;

// Whitespace and comments
LINE_COMMENT: '//' ~[\r\n]* -> skip ;
COMMENT: '/*' .*? '*/' -> skip ;
WS: [ \t\r\n]+ -> skip ;
