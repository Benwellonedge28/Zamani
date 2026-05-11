// ... (rest of src/parser.rs stays same) ...
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    pub message: String,
    pub span: Span,
}
// ...
