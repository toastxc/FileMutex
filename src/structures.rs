use serde::{Deserialize, Serialize};

/// Mutex Configuration, stores all fields needed to manipulate Mutex
/// Contains most methods for using mutex in functions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mutex {
    pub state: MutexState,
    pub path: String,
    pub timeout: u64,
}

/// MutexState Can be Locked or Open, this data is stored in file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MutexState {
    Locked,
    Open,
}


#[derive(Debug)]
pub enum MutexError {
    /// file system errors
    File(std::io::Error),
    /// mutex state specific errors
    State(MutexExceptions),
    /// generic (stuff like utf8 or unhandled errors
    Generic(String),
}
#[derive(Debug)]
pub enum MutexExceptions {
    /// tried to lock mutex, but mutex was already locked
    AlreadyLocked,
    /// Could not read mutex state (invalid file)
    InvalidState,
}
