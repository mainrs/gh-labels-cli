use hubcaps::Error;

pub trait LabelAlreadyExistsExt {
    fn is_label_already_exists_error(&self) -> bool;
}

impl LabelAlreadyExistsExt for Error {
    fn is_label_already_exists_error(&self) -> bool {
        if let Self::Fault { code, error } = self {
            if *code == 422 && error.message == "Validation Failed" {
                return match &error.errors {
                    Some(v) => v
                        .iter()
                        .any(|v| v.field.as_deref() == Some("name") && v.code == "already_exists"),
                    None => false,
                };
            };
        }

        false
    }
}
