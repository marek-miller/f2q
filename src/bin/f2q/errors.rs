use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    CmdArgs { msg: String },
    File { msg: String },
    Serde { msg: String },
    F2Q(f2q::Error),
}

impl Display for Error {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Error::CmdArgs {
                msg,
            } => write!(f, "[command line] {msg}"),
            Error::File {
                msg,
            } => write!(f, "[file] {msg}"),
            Error::F2Q(e) => write!(f, "f2q: {e}"),
            Error::Serde {
                msg,
            } => write!(f, "[serde] {msg}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<&Error> for u8 {
    fn from(value: &Error) -> Self {
        match value {
            Error::CmdArgs {
                ..
            } => 1,
            Error::File {
                ..
            } => 2,
            Error::F2Q(_) => 3,
            Error::Serde {
                ..
            } => 11,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::File {
            msg: format!("{value}"),
        }
    }
}

impl From<f2q::Error> for Error {
    fn from(value: f2q::Error) -> Self {
        Self::F2Q(value)
    }
}

macro_rules! impl_serde_error {
    ($($Typ:ty)* ) => {
        $(
            impl From<$Typ> for Error {
                fn from(value: $Typ) -> Self {
                    Self::Serde { msg: format!("{value}") }
                }
            }
        )*
    };
}

impl_serde_error!(serde_json::Error);
impl_serde_error!(serde_yaml::Error);
impl_serde_error!(toml::ser::Error toml::de::Error);
