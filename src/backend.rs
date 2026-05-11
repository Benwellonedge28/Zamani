// ... (rest of src/backend.rs stays same) ...
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendError {
    pub message: String,
    pub span: Span,
    pub target: String,
}
// ...
