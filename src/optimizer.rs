// ... (rest of src/optimizer.rs stays same) ...
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizerError {
    pub message: String,
    pub span: Span,
}
// ...
