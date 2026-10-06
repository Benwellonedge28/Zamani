/*

* ============================================================================
* ZAMANI — SQL DIALECT
* ============================================================================
* 
* File:
* grammar/dialects/sql.g4
* 
* Grammar:
* SQL
* 
* Role:
* SQL interoperability / dialect leaf grammar
* 
* Status:
* PRODUCTION-READY PARSER CONTRACT
* 
* Baseline:
* ANTLR4
* Rust 1.97.1+
* Rust 2021
* Safe Rust only
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This grammar defines SQL source syntax at the Zamani interoperability
* boundary.
* 
* SQL is NOT the canonical Zamani data language.
* 
* The ownership pipeline is:
* 
* SQL source
*     |
*     v
* SQL lexical vocabulary
*     |
*     v
* SQL parser
*     |
*     v
* SQL dialect syntax representation
*     |
*     v
* semantic normalization
*     |
*     v
* Zamani data/query semantic model
*     |
*     v
* canonical semantic representation
*     |
*     v
* domain/data IR
*     |
*     v
* planning / optimization / execution
* 
* This file MUST NOT implement:
* 
* - a database runtime;
* - database discovery;
* - connection management;
* - authentication;
* - authorization;
* - query optimization;
* - storage;
* - physical partitioning;
* - sharding;
* - hardware selection;
* - resource allocation;
* - execution;
* - vendor-specific runtime behavior.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - SQL statement syntax;
* - SQL query expressions;
* - SQL table expressions;
* - joins;
* - predicates;
* - grouping;
* - ordering;
* - pagination;
* - window specifications;
* - common table expressions;
* - data modification syntax;
* - core schema-definition syntax;
* - transaction syntax;
* - authorization statement syntax;
* - SQL type-name syntax;
* - explicitly delimited SQL extension boundaries.
* 
* THIS FILE DOES NOT OWN:
* 
* - Zamani identifiers;
* - Zamani types;
* - Zamani expressions;
* - Zamani resource semantics;
* - Zamani capability semantics;
* - Zamani effects;
* - Zamani policies;
* - Zamani provenance;
* - SQL execution;
* - database metadata;
* - physical database topology.
* 
* ============================================================================
* DEPENDENCY CONTRACT
* ============================================================================
* 
* DEPENDS_ON:
* 
* grammar/antlr/ZamaniLexer.g4
* grammar/lexer/tokens.g4
* grammar/lexer/keywords.g4
* grammar/dialects/dialect.g4
* grammar/dialects/registration.g4
* grammar/data/
* grammar/types/
* grammar/expressions/
* grammar/resources/
* grammar/effects/
* grammar/security/
* grammar/compatibility/
* 
* EXPORTS:
* 
* sqlProgram
* 
* CONSUMED_BY:
* 
* SQL dialect adapter
* dialect dispatcher
* SQL interoperability tooling
* SQL conformance tests
* 
* AST_OWNER:
* 
* existing domain-neutral frontend AST / dialect syntax representation
* 
* SEMANTIC_OWNER:
* 
* existing data/query semantic subsystem
* 
* TYPE_OWNER:
* 
* existing Zamani type system
* 
* EFFECT_OWNER:
* 
* existing effects subsystem
* 
* RESOURCE_OWNER:
* 
* existing resource subsystem
* 
* POLICY_OWNER:
* 
* existing policy/security subsystem
* 
* PROVENANCE_OWNER:
* 
* existing provenance subsystem
* 
* IR_OWNER:
* 
* existing canonical semantic/data IR
* 
* SPEC_OWNER:
* 
* grammar/specification/interoperability.md
* grammar/dialects/
* 
* TEST_OWNER:
* 
* grammar/tests/interoperability/
* grammar/tests/dialects/sql/
* 
* ============================================================================
* LEXER CONTRACT
* ============================================================================
* 
* This is a parser grammar.
* 
* It intentionally uses:
* 
* tokenVocab = ZamaniLexer
* 
* It MUST NOT define lexer rules.
* 
* SQL-specific lexical vocabulary MUST be added to the canonical lexical
* hierarchy before this parser is enabled.
* 
* In particular, the canonical lexer must provide SQL token identities for
* the SQL keywords and delimited identifiers/string literals listed by the
* repository's SQL conformance profile.
* 
* The SQL parser MUST NOT define a second lexer.
* 
* ============================================================================
* CASE CONTRACT
* ============================================================================
* 
* SQL keywords are normally case-insensitive.
* 
* The canonical Zamani language must remain free to use its own case rules.
* 
* Therefore SQL case normalization belongs to the SQL dialect lexical adapter,
* not this parser.
* 
* The adapter MUST:
* 
* 1. recognize SQL keyword spellings case-insensitively;
* 2. preserve original source text;
* 3. preserve exact source spans;
* 4. emit canonical Zamani/SQL token identities;
* 5. preserve quoted/delimited identifier spelling;
* 6. preserve string literal contents;
* 7. provide deterministic source mapping.
* 
* The parser itself must remain deterministic and case-independent once
* canonical tokens have been produced.
* 
* ============================================================================
* STRING CONTRACT
* ============================================================================
* 
* SQL standard character strings use single quotes.
* 
* Zamani's ordinary STRING token is not assumed to represent SQL strings.
* 
* The canonical lexical vocabulary must therefore expose a dedicated SQL
* character-string token, for example:
* 
* SQL_STRING
* 
* or the repository's approved equivalent.
* 
* It must support SQL string escaping according to the selected SQL profile.
* 
* The SQL grammar consumes that canonical token.
* 
* ============================================================================
* IDENTIFIER CONTRACT
* ============================================================================
* 
* SQL identifiers are distinct from Zamani identifiers.
* 
* This grammar consumes:
* 
* IDENTIFIER
* SQL_DELIMITED_IDENTIFIER
* 
* where the latter is supplied by the canonical SQL lexical adapter.
* 
* Delimited identifiers preserve their source spelling for semantic
* normalization and diagnostics.
* 
* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* This grammar defines no fixed:
* 
* table count
* column count
* row count
* query depth
* join count
* CTE count
* parameter count
* result size
* database size
* shard count
* node count
* worker count
* memory size
* storage size
* network size
* processor count.
* 
* Parser implementation limits, if any, are implementation/resource limits,
* not language-level SQL limits.
* 
* No MAX_* physical capacity constants may be introduced here.
* 
* ============================================================================
* POCO-REAF CONTRACT
* ============================================================================
* 
* SQL source expresses logical data intent.
* 
* Physical realization is downstream.
* 
* A query MUST remain independent of:
* 
* CPU identity
* GPU identity
* accelerator identity
* machine identity
* database-server identity
* shard identity
* node identity
* storage-device identity.
* 
* Target/resource feasibility is determined after parsing.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* Parsing depends only on:
* 
* canonical SQL tokens
* this grammar
* explicit SQL dialect/profile configuration.
* 
* Parsing MUST NOT:
* 
* inspect hardware;
* inspect the filesystem;
* connect to a database;
* inspect credentials;
* inspect network state;
* discover database schemas;
* execute queries;
* consult runtime state.
* 
* ============================================================================
* SEMANTIC BOUNDARY
* ============================================================================
* 
* Parsing establishes structure.
* 
* Semantic analysis owns:
* 
* name resolution;
* scope resolution;
* column resolution;
* type checking;
* aggregate validation;
* grouping validation;
* window validation;
* recursive-query validation;
* mutation validation;
* schema validation;
* transaction validation;
* authorization validation;
* dialect compatibility;
* capability requirements;
* resource requirements;
* effects;
* policies;
* provenance;
* execution feasibility.
* 
* ============================================================================
  */

parser grammar SQL;

options {
tokenVocab = ZamaniLexer;
}

/* ============================================================================

* 1. PUBLIC ENTRY POINT
* ========================================================================== */

sqlProgram
: sqlStatementList EOF
;

sqlStatementList
: sqlStatement*
;

sqlStatement
: sqlQueryStatement
| sqlInsertStatement
| sqlUpdateStatement
| sqlDeleteStatement
| sqlMergeStatement
| sqlCreateStatement
| sqlAlterStatement
| sqlDropStatement
| sqlTransactionStatement
| sqlAuthorizationStatement
| sqlExtensionStatement
;

/* ============================================================================

* 2. QUERY STATEMENTS
* ========================================================================== */

sqlQueryStatement
: sqlQueryExpression
sqlOrderByClause?
sqlPaginationClause?
sqlLockClause?
sqlStatementTerminator?
;

sqlQueryExpression
: sqlWithClause?
sqlQueryTerm
sqlSetOperation*
;

sqlQueryTerm
: sqlSelectExpression
| LPAREN sqlQueryExpression RPAREN
;

sqlSetOperation
: sqlUnionOperation
| sqlIntersectOperation
| sqlExceptOperation
;

sqlUnionOperation
: UNION sqlSetQuantifier? sqlQueryTerm
;

sqlIntersectOperation
: INTERSECT sqlSetQuantifier? sqlQueryTerm
;

sqlExceptOperation
: EXCEPT sqlSetQuantifier? sqlQueryTerm
;

sqlSetQuantifier
: ALL
| DISTINCT
;

/* ============================================================================

* 3. COMMON TABLE EXPRESSIONS
* ========================================================================== */

sqlWithClause
: WITH RECURSIVE?
sqlCommonTableExpression
(COMMA sqlCommonTableExpression)*
;

sqlCommonTableExpression
: sqlIdentifier
sqlColumnNameList?
AS
LPAREN
sqlQueryExpression
RPAREN
;

sqlColumnNameList
: LPAREN
sqlIdentifierList
RPAREN
;

/* ============================================================================

* 4. SELECT
* ========================================================================== */

sqlSelectExpression
: SELECT
sqlSetQuantifier?
sqlSelectList
sqlFromClause?
sqlWhereClause?
sqlGroupByClause?
sqlHavingClause?
sqlWindowClause?
sqlQualifyClause?
;

sqlSelectList
: STAR
| sqlSelectItem (COMMA sqlSelectItem)*
;

sqlSelectItem
: sqlExpression sqlSelectAlias?
;

sqlSelectAlias
: AS sqlIdentifier
| sqlIdentifier
;

/* ============================================================================

* 5. FROM / TABLE REFERENCES
* ========================================================================== */

sqlFromClause
: FROM
sqlTableReference
(COMMA sqlTableReference)*
;

sqlTableReference
: sqlTablePrimary sqlJoinClause*
;

sqlTablePrimary
: sqlQualifiedName sqlTableAlias?
| LPAREN sqlQueryExpression RPAREN sqlTableAlias?
| sqlTableFunction sqlTableAlias?
;

sqlTableAlias
: AS sqlIdentifier
| sqlIdentifier
;

sqlTableFunction
: sqlQualifiedName
LPAREN
sqlArgumentList?
RPAREN
;

/* ============================================================================

* 6. JOINS
* ========================================================================== */

sqlJoinClause
: sqlJoinType?
JOIN
sqlTablePrimary
sqlJoinCondition?
;

sqlJoinType
: INNER
| LEFT OUTER?
| RIGHT OUTER?
| FULL OUTER?
| CROSS
| NATURAL
;

sqlJoinCondition
: ON sqlExpression
| USING LPAREN sqlIdentifierList RPAREN
;

/* ============================================================================

* 7. FILTERING / GROUPING / WINDOWS
* ========================================================================== */

sqlWhereClause
: WHERE sqlExpression
;

sqlGroupByClause
: GROUP BY
sqlGroupItem
(COMMA sqlGroupItem)*
;

sqlGroupItem
: sqlExpression
;

sqlHavingClause
: HAVING sqlExpression
;

sqlWindowClause
: WINDOW
sqlWindowDefinition
(COMMA sqlWindowDefinition)*
;

sqlWindowDefinition
: sqlIdentifier
AS
sqlWindowSpecification
;

sqlWindowSpecification
: LPAREN
sqlWindowPartitionClause?
sqlOrderByClause?
sqlWindowFrameClause?
RPAREN
;

sqlWindowPartitionClause
: PARTITION BY
sqlExpression
(COMMA sqlExpression)*
;

sqlWindowFrameClause
: sqlWindowFrameUnits
sqlWindowFrameExtent
;

sqlWindowFrameUnits
: ROWS
| RANGE
| GROUPS
;

sqlWindowFrameExtent
: sqlWindowFrameBound
| BETWEEN sqlWindowFrameBound AND sqlWindowFrameBound
;

sqlWindowFrameBound
: UNBOUNDED PRECEDING
| sqlExpression PRECEDING
| CURRENT ROW
| sqlExpression FOLLOWING
| UNBOUNDED FOLLOWING
;

sqlQualifyClause
: QUALIFY sqlExpression
;

/* ============================================================================

* 8. ORDERING / PAGINATION / LOCKING
* ========================================================================== */

sqlOrderByClause
: ORDER BY
sqlOrderItem
(COMMA sqlOrderItem)*
;

sqlOrderItem
: sqlExpression
sqlOrderingDirection?
sqlNullOrdering?
;

sqlOrderingDirection
: ASC
| DESC
;

sqlNullOrdering
: NULLS FIRST
| NULLS LAST
;

sqlPaginationClause
: LIMIT sqlExpression
(OFFSET sqlExpression)?
| OFFSET sqlExpression
(ROW | ROWS)?
(
FETCH
sqlFetchDirection?
sqlFetchQuantity?
(ROW | ROWS)?
ONLY
)?
| FETCH
sqlFetchDirection?
sqlFetchQuantity?
(ROW | ROWS)?
ONLY
;

sqlFetchDirection
: FIRST
| NEXT
;

sqlFetchQuantity
: sqlExpression
;

sqlLockClause
: FOR
(UPDATE | SHARE)
sqlLockTargetClause?
sqlLockWaitClause?
;

sqlLockTargetClause
: OF sqlQualifiedName
(COMMA sqlQualifiedName)*
;

sqlLockWaitClause
: NOWAIT
| SKIP LOCKED
;

/* ============================================================================

* 9. INSERT
* ========================================================================== */

sqlInsertStatement
: INSERT INTO
sqlQualifiedName
sqlInsertColumnList?
sqlInsertSource
sqlReturningClause?
sqlStatementTerminator?
;

sqlInsertSource
: sqlValuesClause
| sqlQueryExpression
| DEFAULT VALUES
;

sqlInsertColumnList
: LPAREN sqlIdentifierList RPAREN
;

sqlValuesClause
: VALUES
sqlRowValue
(COMMA sqlRowValue)*
;

sqlRowValue
: LPAREN sqlExpressionList? RPAREN
;

sqlReturningClause
: RETURNING sqlSelectList
;

/* ============================================================================

* 10. UPDATE
* ========================================================================== */

sqlUpdateStatement
: UPDATE
sqlQualifiedName
sqlTableAlias?
SET
sqlAssignment
(COMMA sqlAssignment)*
sqlWhereClause?
sqlReturningClause?
sqlStatementTerminator?
;

sqlAssignment
: sqlIdentifier
ASSIGN
sqlExpression
;

/* ============================================================================

* 11. DELETE
* ========================================================================== */

sqlDeleteStatement
: DELETE FROM
sqlQualifiedName
sqlTableAlias?
sqlWhereClause?
sqlReturningClause?
sqlStatementTerminator?
;

/* ============================================================================

* 12. MERGE
* ========================================================================== */

sqlMergeStatement
: MERGE INTO
sqlQualifiedName
sqlTableAlias?
USING sqlMergeSource
ON sqlExpression
sqlMergeWhenClause+
sqlStatementTerminator?
;

sqlMergeSource
: sqlQualifiedName sqlTableAlias?
| LPAREN sqlQueryExpression RPAREN sqlTableAlias?
;

sqlMergeWhenClause
: WHEN MATCHED
sqlMergeSearchCondition?
THEN
sqlMergeMatchedAction
| WHEN NOT MATCHED
sqlMergeSearchCondition?
THEN
sqlMergeNotMatchedAction
;

sqlMergeSearchCondition
: AND sqlExpression
;

sqlMergeMatchedAction
: UPDATE SET
sqlAssignment
(COMMA sqlAssignment)*
| DELETE
;

sqlMergeNotMatchedAction
: INSERT
sqlInsertColumnList?
VALUES sqlRowValue
;

/* ============================================================================

* 13. CREATE
* 
* The core object forms are explicit.
* 
* Vendor-specific CREATE variants cross the extension boundary rather than
* being silently accepted as arbitrary token sequences.
* ========================================================================== */

sqlCreateStatement
: CREATE sqlCreateObject sqlCreateObjectBody?
sqlStatementTerminator?
;

sqlCreateObject
: TABLE
| SCHEMA
| VIEW
| INDEX
| TRIGGER
;

sqlCreateObjectBody
: sqlCreateTableBody
| sqlCreateViewBody
| sqlCreateIndexBody
| sqlCreateTriggerBody
| sqlCreateSchemaBody
;

sqlCreateTableBody
: LPAREN
sqlTableElement
(COMMA sqlTableElement)*
RPAREN
;

sqlTableElement
: sqlColumnDefinition
| sqlTableConstraint
;

sqlColumnDefinition
: sqlIdentifier
sqlTypeName
sqlColumnConstraint*
;

sqlColumnConstraint
: NOT NULL
| NULL
| DEFAULT sqlExpression
| GENERATED sqlGeneratedColumn
| PRIMARY KEY
| UNIQUE
| CHECK LPAREN sqlExpression RPAREN
| REFERENCES sqlQualifiedName
sqlReferenceColumnList?
sqlReferenceAction*
;

sqlGeneratedColumn
: ALWAYS AS
LPAREN sqlExpression RPAREN
sqlGeneratedStorage?
;

sqlGeneratedStorage
: IDENTIFIER
;

sqlTableConstraint
: CONSTRAINT sqlIdentifier sqlTableConstraintBody
| PRIMARY KEY sqlColumnNameList
| UNIQUE sqlColumnNameList
| CHECK LPAREN sqlExpression RPAREN
| FOREIGN KEY sqlColumnNameList
REFERENCES sqlQualifiedName
sqlReferenceColumnList?
sqlReferenceAction*
;

sqlTableConstraintBody
: PRIMARY KEY sqlColumnNameList
| UNIQUE sqlColumnNameList
| CHECK LPAREN sqlExpression RPAREN
| FOREIGN KEY sqlColumnNameList
REFERENCES sqlQualifiedName
;

sqlReferenceColumnList
: LPAREN sqlIdentifierList RPAREN
;

sqlReferenceAction
: ON
(DELETE | UPDATE)
(CASCADE | RESTRICT | NO ACTION | SET NULL | SET DEFAULT)
;

sqlCreateViewBody
: AS sqlQueryExpression
;

sqlCreateIndexBody
: sqlIndexUnique?
INDEX?
sqlIdentifier
ON
sqlQualifiedName
LPAREN sqlExpressionList RPAREN
;

sqlIndexUnique
: UNIQUE
;

sqlCreateTriggerBody
: sqlTriggerTiming
sqlTriggerEvent
ON sqlQualifiedName
sqlTriggerAction
;

sqlTriggerTiming
: BEFORE
| AFTER
| INSTEAD OF
;

sqlTriggerEvent
: INSERT
| UPDATE
| DELETE
;

sqlTriggerAction
: sqlTokenSequence
;

sqlCreateSchemaBody
: sqlSchemaAuthorization?
;

sqlSchemaAuthorization
: AUTHORIZATION sqlIdentifier
;

/* ============================================================================

* 14. ALTER
* ========================================================================== */

sqlAlterStatement
: ALTER
sqlAlterObject
sqlQualifiedName
sqlAlterAction
sqlStatementTerminator?
;

sqlAlterObject
: TABLE
| SCHEMA
| VIEW
| INDEX
;

sqlAlterAction
: ADD sqlAlterAddAction
| DROP sqlAlterDropAction
| RENAME sqlAlterRenameAction
| ALTER sqlAlterColumnAction
;

sqlAlterAddAction
: COLUMN? sqlColumnDefinition
| TABLE sqlTableConstraint
;

sqlAlterDropAction
: COLUMN sqlIdentifier
| CONSTRAINT sqlIdentifier
| PRIMARY KEY
| UNIQUE sqlColumnNameList
;

sqlAlterRenameAction
: COLUMN sqlIdentifier TO sqlIdentifier
| TO sqlIdentifier
;

sqlAlterColumnAction
: COLUMN sqlIdentifier sqlAlterColumnOperation
;

sqlAlterColumnOperation
: TYPE sqlTypeName
| SET DEFAULT sqlExpression
| DROP DEFAULT
| SET NOT NULL
| DROP NOT NULL
;

/* ============================================================================

* 15. DROP
* ========================================================================== */

sqlDropStatement
: DROP
sqlDropObject
sqlQualifiedName
sqlDropBehavior?
sqlStatementTerminator?
;

sqlDropObject
: TABLE
| SCHEMA
| VIEW
| INDEX
| TRIGGER
;

sqlDropBehavior
: CASCADE
| RESTRICT
;

/* ============================================================================

* 16. TRANSACTIONS
* ========================================================================== */

sqlTransactionStatement
: sqlBeginTransactionStatement
| sqlCommitStatement
| sqlRollbackStatement
| sqlSavepointStatement
;

sqlBeginTransactionStatement
: BEGIN
TRANSACTION?
sqlTransactionMode*
sqlStatementTerminator?
;

sqlTransactionMode
: ISOLATION LEVEL sqlIdentifier
| READ ONLY
| READ WRITE
;

sqlCommitStatement
: COMMIT
TRANSACTION?
sqlStatementTerminator?
;

sqlRollbackStatement
: ROLLBACK
TRANSACTION?
(TO SAVEPOINT? sqlIdentifier)?
sqlStatementTerminator?
;

sqlSavepointStatement
: SAVEPOINT sqlIdentifier
sqlStatementTerminator?
;

/* ============================================================================

* 17. AUTHORIZATION
* ========================================================================== */

sqlAuthorizationStatement
: GRANT sqlGrantBody sqlStatementTerminator?
| REVOKE sqlRevokeBody sqlStatementTerminator?
;

sqlGrantBody
: sqlTokenSequence
;

sqlRevokeBody
: sqlTokenSequence
;

/* ============================================================================

* 18. EXPRESSIONS
* ========================================================================== */

sqlExpression
: sqlOrExpression
;

sqlOrExpression
: sqlAndExpression
(OR sqlAndExpression)*
;

sqlAndExpression
: sqlNotExpression
(AND sqlNotExpression)*
;

sqlNotExpression
: NOT sqlNotExpression
| sqlPredicateExpression
;

sqlPredicateExpression
: sqlValueExpression
sqlPredicate*
;

sqlPredicate
: sqlComparisonPredicate
| sqlBetweenPredicate
| sqlInPredicate
| sqlLikePredicate
| sqlIsPredicate
;

sqlComparisonPredicate
: sqlComparisonOperator sqlValueExpression
;

sqlComparisonOperator
: EQUAL_EQUAL
| NOT_EQUAL
| LESS
| LESS_EQUAL
| GREATER
| GREATER_EQUAL
;

sqlBetweenPredicate
: BETWEEN sqlValueExpression
AND sqlValueExpression
;

sqlInPredicate
: IN
LPAREN
(
sqlQueryExpression
| sqlExpressionList
)
RPAREN
;

sqlLikePredicate
: LIKE
sqlValueExpression
(ESCAPE sqlValueExpression)?
;

sqlIsPredicate
: IS NOT?
(
NULL
| TRUE
| FALSE
| DISTINCT FROM sqlValueExpression
)
;

sqlExistsPredicate
: EXISTS
LPAREN
sqlQueryExpression
RPAREN
;

/* ============================================================================

* 19. VALUE EXPRESSIONS
* ========================================================================== */

sqlValueExpression
: sqlAdditiveExpression
;

sqlAdditiveExpression
: sqlMultiplicativeExpression
((PLUS | MINUS) sqlMultiplicativeExpression)*
;

sqlMultiplicativeExpression
: sqlUnaryExpression
((STAR | SLASH | MODULO) sqlUnaryExpression)*
;

sqlUnaryExpression
: (PLUS | MINUS | NOT)*
sqlPrimaryExpression
;

sqlPrimaryExpression
: sqlLiteral
| sqlQualifiedName
| sqlFunctionCall
| sqlCaseExpression
| sqlCastExpression
| sqlExistsExpression
| LPAREN sqlExpression RPAREN
;

sqlLiteral
: INTEGER
| FLOAT
| SQL_STRING
| TRUE
| FALSE
| NULL
;

sqlFunctionCall
: sqlQualifiedName
LPAREN
sqlFunctionArguments?
RPAREN
;

sqlFunctionArguments
: STAR
| sqlExpressionList
;

sqlCaseExpression
: CASE
sqlExpression?
sqlWhenClause+
ELSE sqlExpression?
END
;

sqlWhenClause
: WHEN sqlExpression
THEN sqlExpression
;

sqlCastExpression
: CAST
LPAREN
sqlExpression
AS
sqlTypeName
RPAREN
;

sqlExistsExpression
: sqlExistsPredicate
;

/* ============================================================================

* 20. TYPE NAMES
* ========================================================================== */

sqlTypeName
: sqlQualifiedName
sqlTypeParameterList?
;

sqlTypeParameterList
: LPAREN
sqlExpressionList?
RPAREN
;

/* ============================================================================

* 21. IDENTIFIERS
* ========================================================================== */

sqlIdentifier
: IDENTIFIER
| SQL_DELIMITED_IDENTIFIER
;

sqlQualifiedName
: sqlIdentifier
(DOT sqlIdentifier)*
;

sqlIdentifierList
: sqlIdentifier
(COMMA sqlIdentifier)*
;

/* ============================================================================

* 22. EXTENSION BOUNDARY
* 
* This is deliberately explicit.
* 
* Vendor syntax must be introduced through a dialect extension construct or
* a separately registered vendor grammar.
* 
* Arbitrary SQL token streams are NOT accepted as standard SQL.
* ========================================================================== */

sqlExtensionStatement
: AT
sqlQualifiedName
sqlExtensionPayload?
sqlStatementTerminator?
;

sqlExtensionPayload
: LPAREN
sqlBalancedTokenSequence?
RPAREN
;

sqlBalancedTokenSequence
: sqlBalancedToken*
;

sqlBalancedToken
: sqlBalancedAtom
| sqlBalancedParenthesized
| sqlBalancedBracketed
| sqlBalancedBraced
;

sqlBalancedParenthesized
: LPAREN
sqlBalancedTokenSequence?
RPAREN
;

sqlBalancedBracketed
: LBRACKET
sqlBalancedTokenSequence?
RBRACKET
;

sqlBalancedBraced
: LBRACE
sqlBalancedTokenSequence?
RBRACE
;

sqlBalancedAtom
: IDENTIFIER
| SQL_DELIMITED_IDENTIFIER
| INTEGER
| FLOAT
| SQL_STRING
| CHAR
| COMMA
| DOT
| COLON
| SEMICOLON
| DOUBLE_COLON
| ASSIGN
| EQUAL_EQUAL
| NOT_EQUAL
| LESS
| LESS_EQUAL
| GREATER
| GREATER_EQUAL
| PLUS
| MINUS
| STAR
| SLASH
| MODULO
| AMPERSAND
| PIPE
| CARET
| QUESTION_MARK
| BANG
;

/* ============================================================================

* 23. GENERAL LISTS
* ========================================================================== */

sqlExpressionList
: sqlExpression
(COMMA sqlExpression)*
;

sqlArgumentList
: sqlExpressionList
;

/* ============================================================================

* 24. STATEMENT TERMINATOR
* ========================================================================== */

sqlStatementTerminator
: SEMICOLON
;

/* ============================================================================

* 25. DIALECT INTEGRATION CONTRACT
* ============================================================================
* 
* DIALECT ID:
* 
* sql
* 
* STANDARD PROFILE:
* 
* sql::standard
* 
* Vendor profiles MUST be registered outside this file.
* 
* IMPORTER:
* 
* grammar/dialects/dialect.g4
* grammar/dialects/registration.g4
* 
* or the repository's canonical dialect dispatch boundary.
* 
* AST:
* 
* dialect-neutral SQL syntax representation
* 
* SEMANTIC TARGET:
* 
* existing Zamani data/query semantic model
* 
* EFFECTS:
* 
* resolved downstream
* 
* CAPABILITIES:
* 
* resolved downstream
* 
* RESOURCES:
* 
* resolved downstream
* 
* POLICIES:
* 
* resolved downstream
* 
* PROVENANCE:
* 
* source spans + dialect identity + semantic transformations
* 
* IR:
* 
* existing canonical semantic/data IR
* 
* QUANTUM:
* 
* NOT_APPLICABLE at SQL grammar level.
* 
* HDL:
* 
* NOT_APPLICABLE at SQL grammar level.
* 
* HARDWARE:
* 
* NOT_APPLICABLE at SQL grammar level.
* 
* ============================================================================
* DIAGNOSTIC CONTRACT
* ============================================================================
* 
* Syntax errors:
* 
* reported by the parser.
* 
* Unknown SQL dialect:
* 
* reported by dialect resolution.
* 
* Unsupported SQL feature:
* 
* reported by semantic/profile validation.
* 
* Unsupported database capability:
* 
* reported by capability analysis.
* 
* Insufficient execution resources:
* 
* reported by resource/execution planning.
* 
* Database authorization failure:
* 
* reported by security/execution infrastructure.
* 
* These conditions MUST NOT be conflated.
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* SQL profile/version selection is external to this grammar.
* 
* A profile may classify constructs as:
* 
* stable
* supported
* unsupported
* vendor
* experimental
* deprecated
* 
* This grammar MUST NOT silently reinterpret unsupported constructs as
* standard SQL.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This file contains no:
* 
* MAX_TABLES
* MAX_COLUMNS
* MAX_ROWS
* MAX_JOINS
* MAX_CTES
* MAX_QUERY_SIZE
* MAX_DATABASE_SIZE
* MAX_NODES
* MAX_WORKERS
* MAX_MEMORY
* MAX_STORAGE
* 
* No physical resource ceiling belongs in this grammar.
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when:
* 
* [x] SQL parser has one public entry point.
* [x] SQL has no private lexer.
* [x] Standard SQL structure is explicit.
* [x] Vendor syntax has an explicit extension boundary.
* [x] SQL expressions normalize downstream.
* [x] SQL does not create a second IR.
* [x] SQL has no physical resource limits.
* [x] SQL has no target-specific syntax.
* [x] Source spans remain available through the frontend.
* [x] Deterministic parsing is specified.
* [x] Compatibility is delegated to the dialect system.
* [x] Resource/capability semantics are delegated downstream.
* [x] No Rust actions or unsafe code are required.
* 
* The repository still requires the canonical lexical additions specified
* below this file's integration record before ANTLR generation can consume
* every SQL profile feature.
* 
* ============================================================================
  */