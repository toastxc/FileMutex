use crate::structures::{MutexError, MutexExceptions, MutexState};
use core::fmt;
use regex::Regex;

/// The state of a mutex, this can be open or locked
impl MutexState {
    pub fn from_path(path: &str) -> Result<Self, MutexError> {
        match std::fs::read(path) {
            Err(fs_error) => Err(MutexError::File(fs_error)),
            Ok(file) => {
                let string = match String::from_utf8(file) {
                    Err(fs_error) => return Err(MutexError::Generic(fs_error.to_string())),
                    Ok(file) => file,
                };
                Ok(MutexState::from_string(&string)?)
            }
        }
    }
    /// returns bool based on current state of mutex
    pub fn is_locked(&self) -> bool {
        match self {
            MutexState::Locked => true,
            MutexState::Open => false,
        }
    }

    // inverse is_locked
    pub fn is_open(&self) -> bool {
        !self.is_locked()
    }

    /// updates mutex file with current state
    pub fn sync(&self, path: &str) -> Result<(), MutexError> {
        std::fs::write(path, self.to_string()).unwrap();

        if let Err(fs_error) = std::fs::write(path, self.to_string()) {
            return Err(MutexError::File(fs_error));
        };
        Ok(())
    }

    /// Imports mutex from string, removes strange characters with Regex
    pub fn from_string(input: &str) -> Result<Self, MutexError> {
        // Remove all non-alphanumeric characters using regex
        let regex = Regex::new(r#"[^a-zA-Z0-9]+"#).unwrap();
        let input: String = regex.replace_all(input, "").into();
        match input.as_str() {
            "Locked" => Ok(MutexState::Locked),
            "Open" => Ok(MutexState::Open),
            _ => Err(MutexError::State(MutexExceptions::InvalidState)),
        }
    }
}

/// Print function for Mutex
impl fmt::Display for MutexState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state = match self {
            MutexState::Locked => r#"Locked"#,
            MutexState::Open => r#"Open"#,
        };
        write!(f, "{state}")
    }
}
