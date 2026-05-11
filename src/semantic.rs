// ... (rest of src/semantic.rs stays same) ...
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticError {
    pub message: String,
    pub span: Span,
}
// ...
