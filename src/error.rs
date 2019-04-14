/************************************************************************************************/

#[derive(Debug)]
pub struct Error {
    reason: String,
}

/************************************************************************************************/

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}
/************************************************************************************************/

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.reason.as_str()
    }
}

/************************************************************************************************/

impl Error {
    pub fn with_reason(reason: &str) -> Error {
        Error {
            reason: String::from(reason),
        }
    }
}

/************************************************************************************************/