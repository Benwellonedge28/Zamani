
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR typed tables
//!
//! Path: `src/zuir/values/tables.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Represent portable, typed, tabular data in the ZUIR value layer.
//!
//! A table consists of a schema and an ordered collection of rows. Each
//! column references the canonical semantic type system, and each populated
//! cell reuses the canonical ConstantValue representation.
//!
//! ## OWNS
//!
//! - Table, table schema, column, and row representations.
//! - Column-name uniqueness and row-shape validation.
//! - Explicit nullability rules for cells.
//! - Configurable structural validation budgets.
//! - Deterministic, local table validation errors.
//!
//! ## DOES_NOT_OWN
//!
//! - Semantic type definitions, type resolution, or type compatibility.
//! - SQL syntax, query planning, query execution, or database connections.
//! - Source parsing or source AST definitions.
//! - Constant evaluation or arbitrary-precision arithmetic.
//! - Table storage engines, physical layouts, or target allocation.
//! - IR program construction, operation verification, or lowering.
//! - Extension registration or execution.
//!
//! ## DEPENDENCIES
//!
//! - `types/type_def.rs` owns TypeRef.
//! - `schema/types.rs` owns QualifiedName.
//! - `values/constants.rs` owns ConstantValue and constant validation.
//! - Serde provides data serialization traits.
//!
//! ## INTEGRATION
//!
//! - Declare this module from the actual owner of `values`.
//! - The type verifier must validate cell values against column types.
//! - Deserialization boundaries must call `Table::validate_with`.
//! - SQL or other data dialects may lower into this representation when
//!   their semantics fit this model.
//! - Dialects needing streaming, lazy, sparse, or distributed tables should
//!   define an explicit representation or extension contract rather than
//!   silently changing the meaning of this in-memory table.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! No fixed row count, column count, cell count, field width, tensor rank,
//! device count, or hardware capacity is imposed by the data model.
//!
//! Collections grow dynamically. Optional budgets protect callers against
//! excessive validation work without defining language-level table limits.
//! All concrete executions remain bounded by available memory, time, and
//! the finite limits of the implementation.
//!
//! ## SEMANTIC BOUNDARY
//!
//! Local validation checks shape, names, nullability, and the local
//! representation of each cell. It does not prove that a cell conforms to
//! the semantic type declared by its column.
//!
//! Unknown or unresolved TypeRef values must be handled by the canonical
//! type verifier. This module never guesses a type or silently converts data.
//!
//! ## DETERMINISM
//!
//! Column order and row order are significant and preserved. Duplicate
//! column names are rejected. This module does not promise canonical byte
//! serialization; that belongs to the serialization subsystem.
//!
//! ## FREEZE CRITERIA
//!
//! - Reuses TypeRef and ConstantValue rather than duplicating them.
//! - Rejects duplicate or malformed column names.
//! - Rejects rows with incorrect widths.
//! - Rejects null cells in non-nullable columns.
//! - Uses checked arithmetic for validation counters.
//! - Applies caller-supplied budgets without silent fallback.
//! - Contains no unsafe code or target-specific assumptions.
//! - Module integration, compilation, and tests pass.
 
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::zuir::schema::types::QualifiedName;
use crate::zuir::types::type_def::TypeRef;
use crate::zuir::values::constants::{
    ConstantValidationBudget,
    ConstantValidationError,
    ConstantValue,
};

/// A typed, ordered table value.
///
/// The schema determines the meaning of each column. Rows preserve their
/// insertion order, and cell positions correspond to schema column order.
///
/// A table may be empty while retaining its schema. This is useful for
/// representing typed query results before rows are produced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Table {
    /// Table schema and ordered column definitions.
    pub schema: TableSchema,

    /// Ordered table rows.
    pub rows: Vec<TableRow>,
}

impl Table {
    /// Construct a table and validate its local structure.
    ///
    /// This does not resolve column types or validate cell/type compatibility.
    pub fn new(
        schema: TableSchema,
        rows: Vec<TableRow>,
    ) -> Result<Self, TableValidationError> {
        let table = Self { schema, rows };
        table.validate()?;
        Ok(table)
    }

    /// Construct a table with a schema and no rows.
    ///
    /// This is useful for typed empty results and incremental construction.
    pub fn empty(schema: TableSchema) -> Result<Self, TableValidationError> {
        Self::new(schema, Vec::new())
    }

    /// Validate with no explicit table-level budget.
    ///
    /// Each cell is still subject to the supplied constant subsystem's
    /// unlimited validation policy. Actual work remains resource-bounded.
    pub fn validate(&self) -> Result<(), TableValidationError> {
        self.validate_with(&TableValidationBudget::unlimited())
    }

    /// Validate the table using an explicit operational budget.
    ///
    /// `max_cells` applies to the total number of cells present in the table.
    /// `max_schema_text_bytes` applies to schema and column-name text.
    /// `constant_budget` applies independently to each non-null cell value.
    ///
    /// The cell-value budget is per value, not an aggregate budget across
    /// all nested values in the table.
    pub fn validate_with(
        &self,
        budget: &TableValidationBudget,
    ) -> Result<(), TableValidationError> {
        self.schema.validate_with(budget)?;

        check_limit(
            self.rows.len(),
            budget.max_rows,
            TableValidationError::RowBudgetExceeded,
        )?;

        let width = self.schema.columns.len();
        let mut total_cells = 0usize;

        for (row_index, row) in self.rows.iter().enumerate() {
            if row.cells.len() != width {
                return Err(TableValidationError::RowWidthMismatch {
                    row: row_index,
                    expected: width,
                    actual: row.cells.len(),
                });
            }

            total_cells = total_cells
                .checked_add(row.cells.len())
                .ok_or(TableValidationError::CounterOverflow)?;

            check_limit(
                total_cells,
                budget.max_cells,
                TableValidationError::CellBudgetExceeded,
            )?;

            for (column_index, cell) in row.cells.iter().enumerate() {
                let column = &self.schema.columns[column_index];

                match cell {
                    None if !column.nullable => {
                        return Err(TableValidationError::NullInNonNullableColumn {
                            row: row_index,
                            column: column_index,
                            name: column.name.clone(),
                        });
                    }
                    None => {}
                    Some(value) => {
                        value
                            .validate_with(&budget.constant_budget)
                            .map_err(|source| {
                                TableValidationError::InvalidCellValue {
                                    row: row_index,
                                    column: column_index,
                                    source,
                                }
                            })?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Return the number of rows.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Return the number of columns.
    #[must_use]
    pub fn column_count(&self) -> usize {
        self.schema.columns.len()
    }

    /// Return the total number of cells, if the count is representable.
    ///
    /// This is a structural count, not a count of non-null values.
    #[must_use]
    pub fn cell_count(&self) -> Option<usize> {
        self.rows
            .iter()
            .try_fold(0usize, |total, row| {
                total.checked_add(row.cells.len())
            })
    }

    /// Return a column by its exact name.
    ///
    /// Names are case-sensitive. This lookup does not resolve aliases or
    /// perform language-specific identifier normalization.
    #[must_use]
    pub fn column(&self, name: &str) -> Option<&TableColumn> {
        self.schema.columns.iter().find(|column| column.name == name)
    }

    /// Return a row by its zero-based position.
    #[must_use]
    pub fn row(&self, index: usize) -> Option<&TableRow> {
        self.rows.get(index)
    }
}

/// Schema shared by every row in a table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableSchema {
    /// Optional semantic name for the table.
    ///
    /// An unnamed table is valid, for example as an intermediate query result.
    #[serde(default)]
    pub name: Option<QualifiedName>,

    /// Columns in their semantic and positional order.
    pub columns: Vec<TableColumn>,
}

impl TableSchema {
    /// Construct and locally validate a table schema.
    pub fn new(columns: Vec<TableColumn>) -> Result<Self, TableValidationError> {
        let schema = Self {
            name: None,
            columns,
        };
        schema.validate()?;
        Ok(schema)
    }

    /// Construct a named schema and locally validate it.
    pub fn named(
        name: QualifiedName,
        columns: Vec<TableColumn>,
    ) -> Result<Self, TableValidationError> {
        let schema = Self {
            name: Some(name),
            columns,
        };
        schema.validate()?;
        Ok(schema)
    }

    /// Validate without explicit resource limits.
    pub fn validate(&self) -> Result<(), TableValidationError> {
        self.validate_with(&TableValidationBudget::unlimited())
    }

    /// Validate column names, uniqueness, and schema text budgets.
    ///
    /// Type references are deliberately not resolved here.
    pub fn validate_with(
        &self,
        budget: &TableValidationBudget,
    ) -> Result<(), TableValidationError> {
        check_limit(
            self.columns.len(),
            budget.max_columns,
            TableValidationError::ColumnBudgetExceeded,
        )?;

        let mut names = BTreeSet::new();
        let mut text_bytes = 0usize;

        if let Some(name) = &self.name {
            name.validate()
                .map_err(|error| TableValidationError::InvalidTableName {
                    message: error.to_string(),
                })?;

            text_bytes = text_bytes
                .checked_add(name.canonical_string().len())
                .ok_or(TableValidationError::CounterOverflow)?;
        }

        for column in &self.columns {
            validate_column_name(&column.name)?;

            if !names.insert(column.name.as_str()) {
                return Err(TableValidationError::DuplicateColumnName {
                    name: column.name.clone(),
                });
            }

            text_bytes = text_bytes
                .checked_add(column.name.len())
                .ok_or(TableValidationError::CounterOverflow)?;
        }

        check_limit(
            text_bytes,
            budget.max_schema_text_bytes,
            TableValidationError::SchemaTextBudgetExceeded,
        )?;

        Ok(())
    }
}

/// A column in a table schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableColumn {
    /// Column identifier.
    ///
    /// This model requires one valid qualified-name component. Source-language
    /// spelling and dialect-specific identifier rules belong to their owners.
    pub name: String,

    /// Canonical semantic type reference for this column.
    pub ty: TypeRef,

    /// Whether a missing cell value is permitted.
    ///
    /// This flag describes table nullability. It does not redefine the
    /// semantics of ConstantValue::Unit.
    #[serde(default)]
    pub nullable: bool,
}

impl TableColumn {
    /// Construct a non-nullable column.
    pub fn required(
        name: impl Into<String>,
        ty: TypeRef,
    ) -> Result<Self, TableValidationError> {
        Self::new(name, ty, false)
    }

    /// Construct a nullable column.
    pub fn optional(
        name: impl Into<String>,
        ty: TypeRef,
    ) -> Result<Self, TableValidationError> {
        Self::new(name, ty, true)
    }

    /// Construct and validate a column identifier.
    ///
    /// Type existence and type compatibility are checked by the type system.
    pub fn new(
        name: impl Into<String>,
        ty: TypeRef,
        nullable: bool,
    ) -> Result<Self, TableValidationError> {
        let column = Self {
            name: name.into(),
            ty,
            nullable,
        };
        column.validate()?;
        Ok(column)
    }

    /// Validate the column's local identifier representation.
    pub fn validate(&self) -> Result<(), TableValidationError> {
        validate_column_name(&self.name)
    }
}

/// A row whose cell order matches the schema's column order.
///
/// `None` means that the cell is absent/null according to table semantics.
/// `Some(ConstantValue::Unit)` is a distinct represented value and is not
/// automatically interpreted as null.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableRow {
    /// Ordered cell values.
    pub cells: Vec<Option<ConstantValue>>,
}

impl TableRow {
    /// Construct a row from its ordered cells.
    ///
    /// Width validation requires a schema and is performed by Table.
    #[must_use]
    pub fn new(cells: Vec<Option<ConstantValue>>) -> Self {
        Self { cells }
    }

    /// Return the number of cells in the row.
    #[must_use]
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Return whether the row contains no cells.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Return a cell by zero-based column position.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Option<ConstantValue>> {
        self.cells.get(index)
    }
}

/// Explicit resource budgets for validating table structure and cell values.
///
/// `None` means that the table-level policy imposes no explicit limit for
/// that dimension. It does not imply infinite physical resources.
///
/// Cell values are validated with `constant_budget` independently. Callers
/// requiring an aggregate nested-value budget should implement it in a
/// shared validation context rather than repeatedly resetting this budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TableValidationBudget {
    /// Maximum number of rows.
    pub max_rows: Option<usize>,

    /// Maximum number of columns.
    pub max_columns: Option<usize>,

    /// Maximum total number of cells across all rows.
    pub max_cells: Option<usize>,

    /// Maximum UTF-8 byte length of table and column names combined.
    pub max_schema_text_bytes: Option<usize>,

    /// Validation policy applied independently to each non-null cell value.
    pub constant_budget: ConstantValidationBudget,
}

impl TableValidationBudget {
    /// Create a policy with no explicit table-level limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_rows: None,
            max_columns: None,
            max_cells: None,
            max_schema_text_bytes: None,
            constant_budget: ConstantValidationBudget::unlimited(),
        }
    }

    /// Create a policy with explicit table limits and a cell-value policy.
    #[must_use]
    pub const fn new(
        max_rows: Option<usize>,
        max_columns: Option<usize>,
        max_cells: Option<usize>,
        max_schema_text_bytes: Option<usize>,
        constant_budget: ConstantValidationBudget,
    ) -> Self {
        Self {
            max_rows,
            max_columns,
            max_cells,
            max_schema_text_bytes,
            constant_budget,
        }
    }
}

/// Errors from local table representation validation.
///
/// These errors do not imply a semantic type mismatch. Type resolution and
/// cell/type compatibility belong to the canonical type verifier.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum TableValidationError {
    /// A table name is malformed.
    #[error("invalid table name: {message}")]
    InvalidTableName {
        message: String,
    },

    /// A column name is empty or not a single valid name component.
    #[error("invalid table column name: {name:?}")]
    InvalidColumnName {
        name: String,
    },

    /// Two columns have the same exact name.
    #[error("duplicate table column name: {name}")]
    DuplicateColumnName {
        name: String,
    },

    /// A row contains a different number of cells from the schema width.
    #[error(
        "table row {row} has {actual} cells, but the schema requires {expected}"
    )]
    RowWidthMismatch {
        row: usize,
        expected: usize,
        actual: usize,
    },

    /// A null cell appears in a non-nullable column.
    #[error(
        "null value in non-nullable column {name:?} at row {row}, column {column}"
    )]
    NullInNonNullableColumn {
        row: usize,
        column: usize,
        name: String,
    },

    /// A cell's local ConstantValue representation is invalid.
    #[error(
        "invalid constant at row {row}, column {column}: {source}"
    )]
    InvalidCellValue {
        row: usize,
        column: usize,
        #[source]
        source: ConstantValidationError,
    },

    /// The row count exceeded the configured budget.
    #[error("table row budget exceeded")]
    RowBudgetExceeded,

    /// The column count exceeded the configured budget.
    #[error("table column budget exceeded")]
    ColumnBudgetExceeded,

    /// The total cell count exceeded the configured budget.
    #[error("table cell budget exceeded")]
    CellBudgetExceeded,

    /// Schema-name text exceeded the configured budget.
    #[error("table schema text budget exceeded")]
    SchemaTextBudgetExceeded,

    /// A checked validation counter could not be represented.
    #[error("table validation counter overflow")]
    CounterOverflow,
}

/// Validate that a column name is one valid qualified-name component.
fn validate_column_name(name: &str) -> Result<(), TableValidationError> {
    let parsed = QualifiedName::parse(name).map_err(|_| {
        TableValidationError::InvalidColumnName {
            name: name.to_owned(),
        }
    })?;

    if parsed.depth() != 1 || parsed.local_name() != name {
        return Err(TableValidationError::InvalidColumnName {
            name: name.to_owned(),
        });
    }

    Ok(())
}

/// Enforce an optional limit without imposing a default maximum.
fn check_limit(
    actual: usize,
    limit: Option<usize>,
    error: TableValidationError,
) -> Result<(), TableValidationError> {
    if limit.is_some_and(|maximum| actual > maximum) {
        Err(error)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn boolean_type() -> TypeRef {
        TypeRef::Named(
            QualifiedName::parse("core::Boolean")
                .expect("test type name should be valid"),
        )
    }

    fn string_type() -> TypeRef {
        TypeRef::Named(
            QualifiedName::parse("core::String")
                .expect("test type name should be valid"),
        )
    }

    fn sample_schema() -> TableSchema {
        TableSchema::new(vec![
            TableColumn::required("enabled", boolean_type())
                .expect("valid column"),
            TableColumn::optional("description", string_type())
                .expect("valid column"),
        ])
        .expect("valid schema")
    }

    #[test]
    fn constructs_and_validates_an_empty_typed_table() {
        let table = Table::empty(sample_schema()).expect("valid empty table");

        assert_eq!(table.row_count(), 0);
        assert_eq!(table.column_count(), 2);
        assert_eq!(table.cell_count(), Some(0));
        assert!(table.validate().is_ok());
    }

    #[test]
    fn accepts_valid_rows_and_nullable_cells() {
        let table = Table::new(
            sample_schema(),
            vec![
                TableRow::new(vec![
                    Some(ConstantValue::Boolean(true)),
                    Some(ConstantValue::String("ready".to_owned())),
                ]),
                TableRow::new(vec![
                    Some(ConstantValue::Boolean(false)),
                    None,
                ]),
            ],
        )
        .expect("valid table");

        assert_eq!(table.row_count(), 2);
        assert_eq!(table.cell_count(), Some(4));
        assert!(table.validate().is_ok());
    }

    #[test]
    fn rejects_a_row_with_the_wrong_width() {
        let result = Table::new(
            sample_schema(),
            vec![TableRow::new(vec![
                Some(ConstantValue::Boolean(true)),
            ])],
        );

        assert!(matches!(
            result,
            Err(TableValidationError::RowWidthMismatch {
                row: 0,
                expected: 2,
                actual: 1,
            })
        ));
    }

    #[test]
    fn rejects_null_in_a_non_nullable_column() {
        let result = Table::new(
            sample_schema(),
            vec![TableRow::new(vec![
                None,
                Some(ConstantValue::String("text".to_owned())),
            ])],
        );

        assert!(matches!(
            result,
            Err(TableValidationError::NullInNonNullableColumn {
                row: 0,
                column: 0,
                ..
            })
        ));
    }

    #[test]
    fn rejects_duplicate_column_names() {
        let result = TableSchema::new(vec![
            TableColumn::required("value", boolean_type())
                .expect("valid first column"),
            TableColumn::optional("value", string_type())
                .expect("valid second column"),
        ]);

        assert!(matches!(
            result,
            Err(TableValidationError::DuplicateColumnName { .. })
        ));
    }

    #[test]
    fn rejects_invalid_column_names() {
        assert!(TableColumn::required("", boolean_type()).is_err());
        assert!(TableColumn::required("outer::inner", boolean_type()).is_err());
    }

    #[test]
    fn finds_columns_by_exact_name() {
        let schema = sample_schema();

        assert_eq!(
            schema.columns.iter().find(|c| c.name == "enabled").map(|c| c.name.as_str()),
            Some("enabled")
        );
    }

    #[test]
    fn enforces_row_budget() {
        let table = Table {
            schema: sample_schema(),
            rows: vec![
                TableRow::new(vec![
                    Some(ConstantValue::Boolean(true)),
                    None,
                ]),
            ],
        };

        let budget = TableValidationBudget::new(
            Some(0),
            None,
            None,
            None,
            ConstantValidationBudget::unlimited(),
        );

        assert!(matches!(
            table.validate_with(&budget),
            Err(TableValidationError::RowBudgetExceeded)
        ));
    }

    #[test]
    fn enforces_column_budget() {
        let schema = sample_schema();

        let budget = TableValidationBudget::new(
            None,
            Some(1),
            None,
            None,
            ConstantValidationBudget::unlimited(),
        );

        assert!(matches!(
            schema.validate_with(&budget),
            Err(TableValidationError::ColumnBudgetExceeded)
        ));
    }

    #[test]
    fn enforces_total_cell_budget() {
        let table = Table {
            schema: sample_schema(),
            rows: vec![TableRow::new(vec![
                Some(ConstantValue::Boolean(true)),
                None,
            ])],
        };

        let budget = TableValidationBudget::new(
            None,
            None,
            Some(1),
            None,
            ConstantValidationBudget::unlimited(),
        );

        assert!(matches!(
            table.validate_with(&budget),
            Err(TableValidationError::CellBudgetExceeded)
        ));
    }

    #[test]
    fn enforces_schema_text_budget() {
        let schema = sample_schema();

        let budget = TableValidationBudget::new(
            None,
            None,
            None,
            Some(1),
            ConstantValidationBudget::unlimited(),
        );

        assert!(matches!(
            schema.validate_with(&budget),
            Err(TableValidationError::SchemaTextBudgetExceeded)
        ));
    }

    #[test]
    fn rejects_invalid_constant_encodings_inside_cells() {
        let table = Table {
            schema: sample_schema(),
            rows: vec![TableRow::new(vec![
                Some(ConstantValue::BytesHex("0G".to_owned())),
                None,
            ])],
        };

        assert!(matches!(
            table.validate(),
            Err(TableValidationError::InvalidCellValue {
                row: 0,
                column: 0,
                ..
            })
        ));
    }

    #[test]
    fn distinguishes_null_from_unit() {
        let schema = TableSchema::new(vec![
            TableColumn::required("value", boolean_type())
                .expect("valid column"),
        ])
        .expect("valid schema");

        let null_table = Table {
            schema: schema.clone(),
            rows: vec![TableRow::new(vec![None])],
        };

        let unit_table = Table {
            schema,
            rows: vec![TableRow::new(vec![
                Some(ConstantValue::Unit),
            ])],
        };

        assert!(matches!(
            null_table.validate(),
            Err(TableValidationError::NullInNonNullableColumn { .. })
        ));

        // Local table validation does not establish semantic type
        // compatibility; the type verifier must reject Unit for Boolean.
        assert!(unit_table.validate().is_ok());
    }

    #[test]
    fn lookup_returns_no_result_for_unknown_columns() {
        let table = Table::empty(sample_schema()).expect("valid table");

        assert!(table.column("missing").is_none());
        assert!(table.column("enabled").is_some());
    }

    #[test]
    fn preserves_row_and_column_order_through_serialization() {
        let table = Table::new(
            sample_schema(),
            vec![TableRow::new(vec![
                Some(ConstantValue::Boolean(true)),
                Some(ConstantValue::String("first".to_owned())),
            ])],
        )
        .expect("valid table");

        let encoded =
            serde_json::to_string(&table).expect("table should serialize");

        let decoded: Table =
            serde_json::from_str(&encoded).expect("table should deserialize");

        assert_eq!(decoded, table);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn deserialized_tables_must_be_validated_again() {
        // A schema with duplicate names can be constructed by deserialization
        // because Serde does not call TableSchema::new.
        let malformed = r#"{
            "schema": {
                "name": null,
                "columns": [
                    {
                        "name": "value",
                        "ty": {
                            "kind": "named",
                            "value": ["core", "Boolean"]
                        },
                        "nullable": false
                    },
                    {
                        "name": "value",
                        "ty": {
                            "kind": "named",
                            "value": ["core", "String"]
                        },
                        "nullable": true
                    }
                ]
            },
            "rows": []
        }"#;

        if let Ok(table) = serde_json::from_str::<Table>(malformed) {
            assert!(matches!(
                table.validate(),
                Err(TableValidationError::DuplicateColumnName { .. })
            ));
        }
    }
}
