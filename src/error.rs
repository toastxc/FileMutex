use std::fmt;
use crate::structures::MutexError;

/// Standard display for MutexError
/// Handles File, State and Generic errors
impl fmt::Display for MutexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MutexError::File(fs_error) => write!(
                f,
                "Error occurred while manipulating mutex file: {}",
                fs_error
            ),
            MutexError::State(muterror) => {
                write!(f, "Error occurred with runtime mutex: {:#?}", muterror)
            }

            MutexError::Generic(error) => {
                write!(f, "{}", error)
            }
        }
    }
}
