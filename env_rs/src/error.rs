use std::env::VarError;
use std::str::FromStr;

pub enum Error<T> {
    EnvError(VarError),
    ParseError(Box<dyn FromStr>),
}

impl<T> From<VarError> for Error<T> {
    fn from(err: VarError) -> Error<T> {
        Error::EnvError(err)
    }
}

impl<E> From<E> for Error<E::Err>
    where E: FromStr
{
    fn from(err: E::Err) -> Error<E> {
        Error::ParseError(err)
    }
}
