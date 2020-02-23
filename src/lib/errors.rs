use std::io;

#[derive(Debug)]
pub enum ApplicationError {
    Error(String),
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::Error(error) => write!(f, "{}", error),
        }
    }
}

impl std::error::Error for ApplicationError {}

impl From<io::Error> for ApplicationError {
    fn from(error: io::Error) -> Self {
        ApplicationError::Error(format!("{}", error))
    }
}

impl From<String> for ApplicationError {
    fn from(error: String) -> Self {
        ApplicationError::Error(error)
    }
}

pub trait Flattenable<S, E, I: Into<E>, O: Into<E>> {
    fn flatten(self) -> Result<S, E>;
}

impl<S, I: Into<ApplicationError>, O: Into<ApplicationError>> Flattenable<S, ApplicationError, I, O>
    for Result<Result<S, I>, O>
{
    fn flatten(self) -> Result<S, ApplicationError> {
        match self {
            Ok(Ok(r)) => Ok(r),
            Ok(Err(r)) => Err(r.into()),
            Err(err) => Err(err.into()),
        }
    }
}

// When I try to make Flattenable's implementation fully generic,
// I lose type inference at the call site for the error. Why?
// impl<S, E, I: Into<E>, O: Into<E>>
//     Flattenable<S, E, I, O>
//     for Result<Result<S, I>, O>
// {
//     fn flatten(self) -> Result<S, E> {
//         match self {
//             Ok(Ok(r)) => Ok(r),
//             Ok(Err(r)) => Err(r.into()),
//             Err(err) => Err(err.into()),
//         }
//     }
// }

pub trait Fallible<S> {
    fn fail_to(self, message: &str) -> Result<S, ApplicationError>;
}

impl<S> Fallible<S> for Option<S> {
    fn fail_to(self, message: &str) -> Result<S, ApplicationError> {
        match self {
            Some(v) => Ok(v),
            None => Err(ApplicationError::Error(message.into())),
        }
    }
}
