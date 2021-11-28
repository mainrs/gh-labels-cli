use hubcaps::Error;

pub trait UnauthorizedExt {
    fn is_user_unauthorized(&self) -> bool;
}

impl UnauthorizedExt for Error {
    fn is_user_unauthorized(&self) -> bool {
        match self {
            Self::Fault { code, error } => *code == 401 && error.message == "Bad credentials",
            _ => false,
        }
    }
}
