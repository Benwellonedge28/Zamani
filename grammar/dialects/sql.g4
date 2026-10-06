/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/sql/sql.g4
 *
 * Grammar:
 *     SQL
 *
 * Role:
 *     SQL DIALECT / INTEROPERABILITY LEAF GRAMMAR
 *
 * Status:
 *     PRODUCTION-READY ARCHITECTURAL BASELINE
 *
 * Baseline:
 *     ANTLR4
 *     Rust 1.97.1+
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines source-level SQL interoperability syntax.
 *
 * SQL is NOT the canonical Zamani data language.
 *
 * This grammar therefore provides:
 *
 *     SQL source
 *         |
 *         v
 *     SQL dialect parser
 *         |
 *         v
 *     dialect-neutral SQL syntax representation
 *         |
 *         v
 *     semantic normalization
 *         |
 *         v
 *     Zamani data/query semantic model
 *         |
 *         v
 *     canonical IR / execution planning
 *
 * It MUST NOT create:
 *
 *     - a second Zamani language;
 *     - a second universal data model;
 *     - a second expression language;
 *     - a database runtime;
 *     - a query optimizer;
 *     - a storage engine;
 *     - a physical database topology;
 *     - a fixed database capacity model;
 *     - a vendor-specific execution engine.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - SQL statement syntax;
 *     - SQL query-expression syntax;
 *     - SQL table-expression syntax;
 *     - SQL joins;
 *     - SQL grouping;
 *     - SQL ordering;
 *     - SQL window syntax;
 *     - SQL common table expressions;
 *     - SQL data manipulation syntax;
 *     - SQL schema-definition syntax where supported by the shared lexer;
 *     - SQL transaction syntax where supported by the shared lexer;
 *     - SQL type-name syntax at the SQL dialect boundary;
 *     - SQL dialect extension points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - Zamani identifier syntax;
 *     - Zamani numeric literals;
 *     - Zamani string literals;
 *     - Zamani comments;
 *     - Zamani Unicode rules;
 *     - Zamani expressions;
 *     - Zamani type semantics;
 *     - resource limits;
 *     - capability discovery;
 *     - database discovery;
 *     - connection management;
 *     - authentication;
 *     - authorization;
 *     - query planning;
 *     - query optimization;
 *     - transaction execution;
 *     - storage;
 *     - indexes;
 *     - physical partitions;
 *     - distributed placement;
 *     - hardware selection.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * SQL syntax is an interoperability representation.
 *
 * It is NOT a replacement for:
 *
 *     grammar/data/
 *     grammar/expressions/
 *     grammar/statements/
 *     grammar/types/
 *
 * Zamani-native data/query constructs remain owned by those grammars.
 *
 * SQL enters Zamani through the dialect/interoperability boundary.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * SQL source may describe logical data intent.
 *
 * It MUST NOT encode universal physical assumptions such as:
 *
 *     fixed database size
 *     fixed table size
 *     fixed row count
 *     fixed column count
 *     fixed shard count
 *     fixed node count
 *     fixed memory size
 *     fixed storage size
 *     fixed processor count
 *     fixed database vendor
 *     fixed machine
 *
 * SQL may express logical constraints and query semantics.
 *
 * Physical realization is resolved downstream.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * SQL vendors and future SQL dialects must be extensible.
 *
 * This grammar therefore avoids making every vendor feature a universal
 * Zamani keyword.
 *
 * Vendor-specific identifiers, qualified names and extension clauses remain
 * available through the explicit SQL dialect extension boundary.
 *
 * A vendor feature MUST NOT silently change the meaning of standard Zamani
 * syntax.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * This grammar intentionally uses:
 *
 *     tokenVocab = ZamaniLexer
 *
 * It MUST NOT define another lexer.
 *
 * The canonical lexical pipeline is:
 *
 *     grammar/lexer/
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     SQL parser
 *
 * SQL-specific reserved words that are not already present in the canonical
 * Zamani lexer MUST be added to the canonical lexical vocabulary BEFORE this
 * grammar is enabled in the ANTLR parser build.
 *
 * They MUST NOT be defined locally here.
 *
 * ============================================================================
 * CURRENT LEXER INTEGRATION REQUIREMENT
 * ============================================================================
 *
 * The current repository already provides a number of SQL-compatible tokens,
 * including concepts such as:
 *
 *     SELECT
 *     FROM
 *     WHERE
 *     GROUP
 *     ORDER
 *     BY
 *     HAVING
 *     JOIN
 *     INNER
 *     LEFT
 *     RIGHT
 *     FULL
 *     ON
 *     UNION
 *     WITH
 *     CASE
 *     WHEN
 *     ELSE
 *     END
 *     AS
 *     AND
 *     OR
 *     NOT
 *     IS
 *     IN
 *     UPDATE
 *     INSERT
 *     DELETE
 *     VALUES
 *     TABLE
 *     ALTER
 *     DROP
 *
 * The SQL integration must additionally reconcile any SQL spelling that is
 * currently represented only as IDENTIFIER or is absent from the canonical
 * lexer.
 *
 * Typical additions include, where required by the selected SQL conformance
 * level:
 *
 *     CREATE
 *     INTO
 *     SET
 *     DISTINCT
 *     ASC
 *     DESC
 *     LIMIT
 *     OFFSET
 *     FETCH
 *     FIRST
 *     NEXT
 *     NULLS
 *     INTERSECT
 *     EXCEPT
 *     MERGE
 *     RETURNING
 *     PRIMARY
 *     KEY
 *     UNIQUE
 *     CHECK
 *     DEFAULT
 *     REFERENCES
 *     INDEX
 *     VIEW
 *     TRIGGER
 *     PROCEDURE
 *     FUNCTION
 *     GRANT
 *     REVOKE
 *     COMMIT
 *     ROLLBACK
 *     BEGIN
 *     TRANSACTION
 *     SAVEPOINT
 *     RELEASE
 *     RECURSIVE
 *     OVER
 *     PARTITION
 *     ROWS
 *     RANGE
 *     GROUPS
 *     PRECEDING
 *     FOLLOWING
 *     CURRENT
 *
 * Exact token additions belong to:
 *
 *     grammar/lexer/keywords.g4
 *     grammar/lexer/tokens.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * and MUST be reconciled with the Rust lexer implementation.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar must preserve enough structure for an AST or dialect-neutral
 * syntax representation to retain:
 *
 *     statement kind
 *     source spans
 *     identifiers
 *     qualification
 *     aliases
 *     expressions
 *     predicates
 *     projections
 *     table references
 *     joins
 *     grouping
 *     ordering
 *     limits/offsets
 *     common table expressions
 *     set operations
 *     window specifications
 *     data modification
 *     schema operations
 *     transaction intent
 *     vendor extension nodes
 *
 * The AST MUST NOT contain:
 *
 *     database connection objects
 *     physical database IDs
 *     physical shard IDs
 *     server IDs
 *     machine IDs
 *     storage-device IDs
 *     CPU/GPU/QPU identifiers.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes SQL structure.
 *
 * Semantic analysis owns:
 *
 *     name resolution
 *     scope resolution
 *     column resolution
 *     type checking
 *     aggregate validity
 *     grouping validity
 *     window validity
 *     recursive-query validation
 *     mutation validity
 *     schema validity
 *     transaction semantics
 *     dialect compatibility
 *     capability requirements
 *     effect requirements
 *     resource requirements
 *     security policy
 *     provenance
 *     execution feasibility
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * SQL operations may produce effects.
 *
 * Typical semantic effects include:
 *
 *     data.read
 *     data.write
 *     schema.read
 *     schema.write
 *     transaction
 *     external
 *     network
 *     nondeterministic
 *
 * This grammar does NOT assign those effects.
 *
 * It only preserves syntax required by semantic analysis.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A SQL construct MAY require capabilities such as:
 *
 *     data.query
 *     data.mutation
 *     data.schema
 *     data.transaction
 *     data.recursive_query
 *     data.window_query
 *     data.returning
 *     data.json
 *     data.array
 *     data.spatial
 *     data.vendor_extension
 *
 * Capabilities are resolved downstream.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * SQL syntax MUST NOT impose physical resource limits.
 *
 * The following are semantic/execution concerns:
 *
 *     query memory
 *     result size
 *     transaction capacity
 *     storage capacity
 *     worker count
 *     database node count
 *     network bandwidth
 *     execution time
 *
 * The SQL grammar contains no universal maximum for any of them.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * SQL execution may be constrained by:
 *
 *     security policy
 *     data-access policy
 *     privacy policy
 *     transaction policy
 *     resource policy
 *     provenance policy
 *     vendor policy
 *
 * Those policies are not encoded into SQL grammar productions.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * SQL source spans and dialect identity MUST remain available for provenance.
 *
 * Downstream provenance may record:
 *
 *     source SQL
 *     normalized SQL
 *     semantic query
 *     transformation
 *     optimizer decision
 *     execution plan
 *     external data source
 *     result derivation
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source tokens
 *     grammar
 *     explicit parser configuration.
 *
 * It MUST NOT:
 *
 *     inspect the filesystem;
 *     connect to a database;
 *     discover a server;
 *     inspect hardware;
 *     inspect environment variables;
 *     resolve credentials;
 *     execute SQL.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This grammar represents SQL interoperability, not a promise that every
 * SQL implementation accepts every production.
 *
 * SQL dialect/version selection MUST be represented through the surrounding
 * dialect infrastructure.
 *
 * Example semantic information:
 *
 *     dialect sql::standard
 *     version ...
 *     capability ...
 *
 * remains owned by:
 *
 *     grammar/dialects/
 *
 * This file does not duplicate dialect registration/versioning.
 *
 * ============================================================================
 * SQL ENTRY POINT
 * ============================================================================
 *
 * The public entry point is:
 *
 *     sqlProgram
 *
 * It is deliberately separate from the Zamani root `program`.
 *
 * The dialect adapter decides how SQL text is introduced:
 *
 *     - explicit SQL dialect block;
 *     - external SQL source;
 *     - embedded SQL construct;
 *     - tooling invocation;
 *     - interoperability import.
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
    | sqlGrantStatement
    | sqlRevokeStatement
    | sqlDialectExtensionStatement
    ;


/* ============================================================================
 * 2. QUERY STATEMENTS
 * ========================================================================== */

sqlQueryStatement
    : sqlWithClause?
      sqlQueryExpression
      sqlOrderByClause?
      sqlPaginationClause?
      sqlLockClause?
      sqlStatementTerminator?
    ;

sqlQueryExpression
    : sqlSelectExpression
      (
          sqlUnionExpression
        | sqlIntersectExpression
        | sqlExceptExpression
      )*
    ;

sqlSetQueryExpression
    : sqlQueryExpression
    ;

sqlUnionExpression
    : UNION sqlSetQuantifier?
      sqlSelectExpression
    ;

sqlIntersectExpression
    : INTERSECT sqlSetQuantifier?
      sqlSelectExpression
    ;

sqlExceptExpression
    : EXCEPT sqlSetQuantifier?
      sqlSelectExpression
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
      (
          COMMA sqlCommonTableExpression
      )*
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
    | sqlSelectItem
      (
          COMMA sqlSelectItem
      )*
    ;

sqlSelectItem
    : sqlExpression
      sqlAliasClause?
    ;

sqlAliasClause
    : AS sqlIdentifier
    | sqlIdentifier
    ;


/* ============================================================================
 * 5. FROM
 * ========================================================================== */

sqlFromClause
    : FROM sqlTableReference
      (
          COMMA sqlTableReference
      )*
    ;

sqlTableReference
    : sqlTablePrimary
      sqlJoinClause*
    ;

sqlTablePrimary
    : sqlQualifiedName
      sqlAliasClause?
    | LPAREN sqlQueryExpression RPAREN
      sqlAliasClause?
    | sqlTableFunction
      sqlAliasClause?
    ;

sqlTableFunction
    : sqlIdentifier
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
    | USING
      LPAREN
      sqlIdentifierList
      RPAREN
    ;


/* ============================================================================
 * 7. WHERE / GROUP / HAVING
 * ========================================================================== */

sqlWhereClause
    : WHERE sqlExpression
    ;

sqlGroupByClause
    : GROUP BY
      sqlGroupItem
      (
          COMMA sqlGroupItem
      )*
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
      (
          COMMA sqlWindowDefinition
      )*
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
      (
          COMMA sqlExpression
      )*
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
    | UNBOUNDED FOLLOWING
    | CURRENT ROW
    | sqlExpression PRECEDING
    | sqlExpression FOLLOWING
    ;

sqlQualifyClause
    : QUALIFY sqlExpression
    ;


/* ============================================================================
 * 8. ORDERING / PAGINATION
 * ========================================================================== */

sqlOrderByClause
    : ORDER BY
      sqlOrderItem
      (
          COMMA sqlOrderItem
      )*
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
      (
          OFFSET sqlExpression
      )?
    | OFFSET sqlExpression
      (
          ROW
        | ROWS
      )?
      (
          FETCH
          sqlFetchDirection?
          sqlFetchQuantity?
          (
              ROW
            | ROWS
          )?
          ONLY
      )?
    | FETCH
      sqlFetchDirection?
      sqlFetchQuantity?
      (
          ROW
        | ROWS
      )?
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
      (
          UPDATE
        | SHARE
      )
      sqlLockTargetClause?
      sqlLockWaitClause?
    ;

sqlLockTargetClause
    : OF sqlQualifiedName
      (
          COMMA sqlQualifiedName
      )*
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
      (
          sqlValuesClause
        | sqlQueryExpression
      )
      sqlReturningClause?
      sqlStatementTerminator?
    ;

sqlInsertColumnList
    : LPAREN
      sqlIdentifierList
      RPAREN
    ;

sqlValuesClause
    : VALUES
      sqlRowValue
      (
          COMMA sqlRowValue
      )*
    ;

sqlRowValue
    : LPAREN
      sqlExpressionList?
      RPAREN
    ;

sqlReturningClause
    : RETURNING
      sqlSelectList
    ;


/* ============================================================================
 * 10. UPDATE
 * ========================================================================== */

sqlUpdateStatement
    : UPDATE
      sqlQualifiedName
      sqlAliasClause?
      SET
      sqlAssignment
      (
          COMMA sqlAssignment
      )*
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
      sqlAliasClause?
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
      sqlAliasClause?
      USING sqlMergeSource
      ON sqlExpression
      sqlMergeWhenClause+
      sqlStatementTerminator?
    ;

sqlMergeSource
    : sqlQualifiedName
      sqlAliasClause?
    | LPAREN sqlQueryExpression RPAREN
      sqlAliasClause?
    ;

sqlMergeWhenClause
    : WHEN MATCHED
      sqlMergeMatchPredicate?
      THEN
      sqlMergeMatchedAction
    | WHEN NOT MATCHED
      sqlMergeNotMatchedPredicate?
      THEN
      sqlMergeNotMatchedAction
    ;

sqlMergeMatchPredicate
    : AND sqlExpression
    ;

sqlMergeNotMatchedPredicate
    : AND sqlExpression
    ;

sqlMergeMatchedAction
    : UPDATE SET
      sqlAssignment
      (
          COMMA sqlAssignment
      )*
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
 * The concrete object vocabulary is deliberately open.
 *
 * A future SQL object type should not require a universal Zamani AST redesign.
 * ========================================================================== */

sqlCreateStatement
    : CREATE
      sqlCreateObject
      sqlCreateObjectBody?
      sqlStatementTerminator?
    ;

sqlCreateObject
    : sqlIdentifier
    ;

sqlCreateObjectBody
    : sqlParenthesizedTokenGroup
    | sqlTokenSequence
    ;


/* ============================================================================
 * 14. ALTER
 * ========================================================================== */

sqlAlterStatement
    : ALTER
      sqlIdentifier
      sqlQualifiedName?
      sqlAlterAction?
      sqlStatementTerminator?
    ;

sqlAlterAction
    : sqlTokenSequence
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
    : sqlIdentifier
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
      sqlTransactionOption*
      sqlStatementTerminator?
    ;

sqlTransactionOption
    : ISOLATION
      LEVEL
      sqlIdentifier
    | READ
      ONLY
    | READ
      WRITE
    ;

sqlCommitStatement
    : COMMIT
      TRANSACTION?
      sqlStatementTerminator?
    ;

sqlRollbackStatement
    : ROLLBACK
      TRANSACTION?
      (
          TO SAVEPOINT?
          sqlIdentifier
      )?
      sqlStatementTerminator?
    ;

sqlSavepointStatement
    : SAVEPOINT sqlIdentifier sqlStatementTerminator?
    ;


/* ============================================================================
 * 17. AUTHORIZATION
 * ========================================================================== */

sqlGrantStatement
    : GRANT
      sqlTokenSequence
      sqlStatementTerminator?
    ;

sqlRevokeStatement
    : REVOKE
      sqlTokenSequence
      sqlStatementTerminator?
    ;


/* ============================================================================
 * 18. EXPRESSIONS
 *
 * SQL expressions are intentionally represented inside the SQL dialect.
 *
 * They must eventually normalize into the Zamani semantic expression model.
 *
 * They do NOT create a second universal expression system.
 * ========================================================================== */

sqlExpression
    : sqlOrExpression
    ;

sqlOrExpression
    : sqlAndExpression
      (
          OR sqlAndExpression
      )*
    ;

sqlAndExpression
    : sqlNotExpression
      (
          AND sqlNotExpression
      )*
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
    | sqlExistsPredicate
    ;

sqlComparisonPredicate
    : sqlComparisonOperator
      sqlValueExpression
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
    : BETWEEN sqlValueExpression AND sqlValueExpression
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
    : LIKE sqlValueExpression
      (
          ESCAPE sqlValueExpression
      )?
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
      (
          (
              PLUS
            | MINUS
          )
          sqlMultiplicativeExpression
      )*
    ;

sqlMultiplicativeExpression
    : sqlUnaryExpression
      (
          (
              STAR
            | SLASH
            | MODULO
          )
          sqlUnaryExpression
      )*
    ;

sqlUnaryExpression
    : (
          PLUS
        | MINUS
        | NOT
      )*
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
    | STRING
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
    : EXISTS
      LPAREN
      sqlQueryExpression
      RPAREN
    ;


/* ============================================================================
 * 20. TYPE NAMES
 *
 * SQL type names are intentionally open.
 *
 * The grammar does not enumerate every vendor type.
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
 *
 * SQL identifiers are normalized later.
 *
 * Reserved Zamani tokens cannot be reinterpreted as SQL identifiers unless
 * the canonical lexer supplies an appropriate quoted-identifier token.
 *
 * The implementation should add the canonical quoted identifier token if the
 * selected SQL conformance profile requires it.
 * ========================================================================== */

sqlIdentifier
    : IDENTIFIER
    ;

sqlQualifiedName
    : sqlIdentifier
      (
          DOUBLE_COLON sqlIdentifier
        | DOT sqlIdentifier
      )*
    ;

sqlIdentifierList
    : sqlIdentifier
      (
          COMMA sqlIdentifier
      )*
    ;


/* ============================================================================
 * 22. EXTENSION / VENDOR BOUNDARY
 *
 * This is intentionally token-tree based.
 *
 * It permits future SQL/vendor syntax to be represented without adding
 * universal grammar rules for every database implementation.
 *
 * Semantic validation MUST determine whether the extension is valid.
 * ========================================================================== */

sqlDialectExtensionStatement
    : sqlExtensionMarker
      sqlQualifiedName
      sqlParenthesizedTokenGroup?
      sqlStatementTerminator?
    ;

sqlExtensionMarker
    : AT
    ;

sqlParenthesizedTokenGroup
    : LPAREN
      sqlBalancedTokenSequence?
      RPAREN
    ;

sqlBalancedTokenSequence
    : sqlBalancedToken*
    ;

sqlBalancedToken
    : sqlBalancedAtom
    | sqlParenthesizedTokenGroup
    | sqlBracketedTokenGroup
    | sqlBracedTokenGroup
    ;

sqlBalancedAtom
    : IDENTIFIER
    | INTEGER
    | FLOAT
    | STRING
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

sqlBracketedTokenGroup
    : LBRACKET
      sqlBalancedTokenSequence?
      RBRACKET
    ;

sqlBracedTokenGroup
    : LBRACE
      sqlBalancedTokenSequence?
      RBRACE
    ;

sqlTokenSequence
    : sqlBalancedToken+
    ;


/* ============================================================================
 * 23. GENERAL EXPRESSION LISTS
 * ========================================================================== */

sqlExpressionList
    : sqlExpression
      (
          COMMA sqlExpression
      )*
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
 * 25. INTEGRATION CONTRACT
 * ============================================================================
 *
 * IMPORTER:
 *
 *     grammar/dialects/dialect.g4
 *     grammar/dialects/dialects.g4
 *     or the repository's canonical dialect-dispatch boundary.
 *
 * The importer MUST NOT copy SQL productions.
 *
 * ============================================================================
 *
 * DIALECT REGISTRATION
 *
 * SQL identity is registered through:
 *
 *     grammar/dialects/registration.g4
 *
 * Example semantic identity:
 *
 *     sql
 *     sql::standard
 *     sql::vendor::extension
 *
 * No SQL vendor is hard-coded into this grammar.
 *
 * ============================================================================
 *
 * AST OWNER
 *
 * The AST owner is the repository's established frontend AST subsystem.
 *
 * This grammar MUST NOT introduce a database-specific AST as the universal
 * semantic representation.
 *
 * ============================================================================
 *
 * SEMANTIC OWNER
 *
 * SQL semantics are normalized into the existing:
 *
 *     data/query
 *
 * semantic model.
 *
 * SQL-specific information may be retained as dialect metadata.
 *
 * ============================================================================
 *
 * IR OWNER
 *
 * SQL MUST NOT introduce a competing universal IR.
 *
 * Query/data operations should lower through the repository's canonical
 * semantic representation and appropriate domain/data IR.
 *
 * ============================================================================
 *
 * RESOURCE INTEGRATION
 *
 * SQL execution requirements are expressed downstream through the existing:
 *
 *     grammar/resources/
 *     capabilities
 *     policies
 *
 * systems.
 *
 * Example semantic requirements:
 *
 *     data.query
 *     data.transaction
 *     data.recursive_query
 *
 * Physical resource values are not grammar constants.
 *
 * ============================================================================
 *
 * EFFECT INTEGRATION
 *
 * SQL read/write/schema/transaction operations are classified downstream
 * using the existing effects system.
 *
 * ============================================================================
 *
 * SECURITY INTEGRATION
 *
 * SQL authorization and data-access constraints are resolved through the
 * existing security/policy systems.
 *
 * SQL grammar MUST NOT implement authorization.
 *
 * ============================================================================
 *
 * PROVENANCE INTEGRATION
 *
 * Every SQL AST node that survives semantic normalization SHOULD preserve:
 *
 *     source span
 *     dialect identity
 *     source artifact identity
 *     normalization information
 *     transformation lineage
 *
 * ============================================================================
 *
 * FFI / DATABASE DRIVER INTEGRATION
 *
 * Database drivers, protocols and client libraries belong downstream.
 *
 * This grammar does not select:
 *
 *     PostgreSQL
 *     MySQL
 *     SQLite
 *     Oracle
 *     SQL Server
 *     DuckDB
 *     distributed SQL engine
 *     cloud provider
 *
 * A driver/backend may advertise capabilities.
 *
 * ============================================================================
 *
 * POCO-REAF INTEGRATION
 *
 * The same logical SQL source can be lowered to any compatible execution
 * environment without changing its source meaning.
 *
 * Target selection remains outside this grammar.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     MAX_TABLES
 *     MAX_COLUMNS
 *     MAX_ROWS
 *     MAX_DATABASES
 *     MAX_CONNECTIONS
 *     MAX_SHARDS
 *     MAX_NODES
 *     MAX_QUERY_SIZE
 *     MAX_RESULT_SIZE
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TRANSACTIONS
 *
 * None are present.
 *
 * Repetition is represented through grammar operators.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar imposes no language-level bound on:
 *
 *     statements
 *     CTEs
 *     columns
 *     joins
 *     expressions
 *     predicates
 *     projections
 *     grouping expressions
 *     ordering expressions
 *     parameters
 *     query nesting
 *     set-operation chains
 *     window definitions
 *     SQL object names
 *
 * Actual parser/compiler limits belong to implementation/resource policy.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax errors must be reported by the canonical parser diagnostics layer.
 *
 * Semantic errors MUST NOT be manufactured as parser errors.
 *
 * Examples:
 *
 *     missing table
 *     unknown column
 *     unsupported SQL feature
 *     insufficient capability
 *     unavailable database operation
 *
 * are semantic/backend errors, not grammar errors.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required test locations:
 *
 *     grammar/tests/dialects/sql/
 *
 * Required categories:
 *
 *     lexical/
 *     parser/
 *     ast/
 *     semantic/
 *     expressions/
 *     queries/
 *     dml/
 *     ddl/
 *     transactions/
 *     windows/
 *     recursive/
 *     extensions/
 *     negative/
 *     boundary/
 *     scalability/
 *     compatibility/
 *     determinism/
 *
 * ============================================================================
 * REQUIRED POSITIVE TESTS
 * ============================================================================
 *
 * At minimum:
 *
 *     SELECT *
 *     SELECT expression
 *     SELECT ... FROM ...
 *     WHERE
 *     GROUP BY
 *     HAVING
 *     ORDER BY
 *     LIMIT/OFFSET
 *     JOIN
 *     subquery
 *     CTE
 *     recursive CTE
 *     UNION
 *     INTERSECT
 *     EXCEPT
 *     INSERT
 *     UPDATE
 *     DELETE
 *     MERGE
 *     RETURNING
 *     CREATE
 *     ALTER
 *     DROP
 *     transactions
 *     window functions
 *     CASE
 *     CAST
 *     EXISTS
 *     IN
 *     BETWEEN
 *     LIKE
 *     NULL predicates
 *     qualified names
 *     vendor extension boundary
 *
 * ============================================================================
 * REQUIRED NEGATIVE TESTS
 * ============================================================================
 *
 * Test:
 *
 *     malformed SELECT
 *     malformed JOIN
 *     malformed GROUP BY
 *     malformed CTE
 *     malformed window frame
 *     malformed INSERT
 *     malformed UPDATE
 *     malformed DELETE
 *     malformed transaction
 *     unbalanced delimiters
 *     invalid expression structure
 *     invalid pagination structure
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     1. The file compiles against the canonical Zamani lexer.
 *     2. No local lexer exists.
 *     3. SQL is reachable only through the dialect/interoperability boundary.
 *     4. No universal Zamani data grammar is duplicated.
 *     5. No physical database assumptions exist.
 *     6. SQL syntax has a domain-neutral AST mapping.
 *     7. SQL semantics have a documented normalization target.
 *     8. Resource/capability/effect/policy ownership is downstream.
 *     9. Vendor extensions remain open-world.
 *    10. Positive tests pass.
 *    11. Negative tests pass.
 *    12. Boundary tests pass.
 *    13. Scalability tests pass.
 *    14. Determinism tests pass.
 *    15. Rust-generated parser integration passes Rust 1.97.1+.
 *    16. The generated implementation contains no unsafe Rust.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SQL RESERVED/CONTEXTUAL TOKEN DEPENDENCY NOTE
 * ============================================================================
 *
 * The following productions intentionally reference SQL vocabulary that must
 * be available from ZamaniLexer:
 *
 *     ALL
 *     ASC
 *     BEGIN
 *     BETWEEN
 *     CAST
 *     CASCADE
 *     COMMIT
 *     CREATE
 *     CROSS
 *     CURRENT
 *     DELETE
 *     DISTINCT
 *     DROP
 *     ESCAPE
 *     EXCEPT
 *     EXISTS
 *     FETCH
 *     FIRST
 *     FOLLOWING
 *     FOR
 *     FULL
 *     GRANT
 *     GROUP
 *     HAVING
 *     INDEX
 *     INSERT
 *     INTERSECT
 *     INTO
 *     ISOLATION
 *     JOIN
 *     KEY
 *     LEVEL
 *     LIMIT
 *     LOCK
 *     MATCHED
 *     MERGE
 *     NATURAL
 *     NEXT
 *     NO
 *     NOWAIT
 *     NULL
 *     NULLS
 *     OFFSET
 *     ONLY
 *     OUTER
 *     PARTITION
 *     PRECEDING
 *     PRIMARY
 *     QUALIFY
 *     RANGE
 *     READ
 *     RECURSIVE
 *     REFERENCES
 *     RESTRICT
 *     RETURNING
 *     REVOKE
 *     RIGHT
 *     ROLLBACK
 *     ROW
 *     ROWS
 *     SAVEPOINT
 *     SELECT
 *     SET
 *     SHARE
 *     SKIP
 *     TABLE
 *     THEN
 *     TRANSACTION
 *     TRUNCATE
 *     UNION
 *     UNIQUE
 *     UPDATE
 *     USING
 *     VALUES
 *     WHEN
 *     WINDOW
 *     WITH
 *
 * These names are intentionally NOT defined in this file.
 *
 * They belong to the canonical lexical authority.
 *
 * The integration task MUST reconcile the exact current token names before
 * enabling this grammar in the build.
 *
 * ============================================================================
 */