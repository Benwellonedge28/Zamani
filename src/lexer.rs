// ... (rest of src/lexer.rs stays same) ...
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    pub message: String,
    pub span: Span,
}
// ...
