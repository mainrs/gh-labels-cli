use hubcaps::Error;

pub trait RepoNotFoundExt {
    fn is_repo_not_found_error(&self) -> bool;
}

impl RepoNotFoundExt for Error {
    fn is_repo_not_found_error(&self) -> bool {
        match self {
            Self::Fault { code, error } => {
                *code == 404 && error.message == "Not Found" && error.errors == None
            }
            _ => false,
        }
    }
}
