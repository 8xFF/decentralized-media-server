use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerError {
    pub code: String,
    pub message: String,
}

impl ServerError {
    pub fn build<T1: ToString, T2: ToString>(code: T1, message: T2) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
        }
    }
}

trait ErrorDebugger {
    fn log_error(&self, msg: &str);
}

impl<D, E: Debug> ErrorDebugger for Result<D, E> {
    fn log_error(&self, msg: &str) {
        if let Err(e) = self {
            log::error!("{}: {:?}", msg, e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_error_build() {
        let error = ServerError::build("404", "Not Found");
        assert_eq!(error.code, "404");
        assert_eq!(error.message, "Not Found");
    }
}
