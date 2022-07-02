#[derive(Debug, thiserror::Error)]
pub enum NoneError {
    #[error("Option resolved to none")]
    NoneError,
}

trait ToError<T> {
    fn to_error(self) -> Result<T, NoneError>;
}

impl<T> ToError<T> for Option<T> {
    fn to_error(self) -> Result<T, NoneError> {
        match self {
            Some(t) => Ok(t),
            None => Err(NoneError::NoneError),
        }
    }
}
