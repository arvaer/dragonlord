use serde::Serialize;
use crate::model::store;
use serde_with::{serde_as, DisplayFromStr};


pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    EntityNotFound { entity: &'static str, id: i64 },
    // Modules
    Store(store::Error),
    // Extenral
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),


}

impl From<store::Error> for Error{
    fn from(error: store::Error) -> Self {
        Self::Store(error)
    }
}

impl From<sqlx::Error> for Error{
    fn from(error: sqlx::Error) -> Self{
        Self::Sqlx(error)
    }
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}


impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
