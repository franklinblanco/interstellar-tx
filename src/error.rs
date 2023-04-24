use std::fmt::Display;


#[derive(Debug)]
pub struct NotFoundError;

impl Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Itx not found in TX queue.")
    }
}
impl std::error::Error for NotFoundError{}