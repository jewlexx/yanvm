use std::error::Error;

trait ToError<T, E: Error> {
    fn to_error(self) -> Result<T, E>;
}

impl<T, E: Error> ToError<T, E> for Option<T> {
    fn to_error(self) -> Result<T, E> {
        match self {
            Some(t) => Ok(t),
            None => Err(E::NoneError("option is empty")),
        }
    }
}
