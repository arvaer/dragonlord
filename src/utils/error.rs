use crate::crypt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error{
    DateTimeParseError(String),
    FailToB64uDecode,
    Crypt(crypt::Error)
}

impl core::fmt::Display for Error {
    fn fmt ( &self, f: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl From<crypt::Error> for Error {
    fn from(val: crypt::Error) -> Self {
        Self::Crypt(val)
    }
}

impl std::error::Error for Error {}
