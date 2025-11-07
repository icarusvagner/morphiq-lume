pub fn b32hexu_encode(content: &str) -> String {
	data_encoding::BASE32HEX_NOPAD.encode(content.as_bytes())
}

pub fn b32hexu_decode(b32hexu: &str) -> Result<String> {
	data_encoding::BASE32HEX_NOPAD
		.decode(b32hexu.as_bytes())
		.ok()
		.and_then(|v| String::from_utf8(v).ok())
		.ok_or(Error::FailedToDecodeHex32u)
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	FailedToDecodeHex32u,
}

impl std::fmt::Display for Error {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(f, "{self:?}")
	}
}

impl std::error::Error for Error {}
