use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, From)]
pub enum Error {
	DatabaseConnectionFailed,
	DbInitFailed(String),
	#[from]
	IcedError(iced::Error),
	#[from]
	IoError(std::io::Error),
	#[from]
	DuckdbError(duckdb::Error),
}

impl std::fmt::Display for Error {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		match self {
			Self::DatabaseConnectionFailed => {
				write!(f, "Database connection failed")
			}
			Self::DbInitFailed(err) => {
				write!(f, "Database initialized failed error: {err:?}")
			}
			Self::IcedError(err) => write!(f, "Iced error: {err:?}"),
			Self::IoError(err) => write!(f, "IO Erro: {err:?}"),
			Self::DuckdbError(err) => write!(f, "Duckdb error: {err:?}"),
		}
	}
}

impl std::error::Error for Error {}
