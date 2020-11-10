#[derive(Debug)]
pub enum Error {
    InvalidCString,
    LoadLibararyFail,
    GetProcAddressFail,
    NtAllocVmErr(i32),
    NtProtectVmErr(i32),
    InvalidUtf8String,
    MismatchedArch,
    MismatchedLoader,
    NoEntryPoint,
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::convert::From<std::str::Utf8Error> for Error {
    fn from(_: std::str::Utf8Error) -> Self {
        Error::InvalidUtf8String
    }
}
