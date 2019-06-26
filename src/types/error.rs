#[cfg(unix)]
use errno::errno;

#[cfg(unix)]
pub struct ErrnoError(String);

#[cfg(unix)]
impl ErrnoError {
    pub fn new() -> ErrnoError {
        let e = errno();

        ErrnoError(format!("{}", e))
    }
}

#[cfg(unix)]
impl std::error::Error for ErrnoError {
    fn description(&self) -> &str {
        &self.0
    }
}

#[cfg(unix)]
impl std::fmt::Display for ErrnoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(unix)]
impl std::fmt::Debug for ErrnoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Error(Box<dyn std::error::Error + Send + Sync>);

impl Error {
    pub fn new<T>(error: T) -> Error
    where
        T: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Error(error.into())
    }

    #[cfg(unix)]
    pub fn from_errno() -> Error {
        Self::new(ErrnoError::new())
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.0.description()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
