use crate::structures::{Mutex, MutexError, MutexExceptions, MutexState};

impl Mutex {
    /// creates a mutex based on settings
    pub async fn gen(&self) -> Result<Self, MutexError> {
        let mut watchdog = 0;
        loop {
            match MutexState::from_path(&self.path) {
                // if unlocked, leave loop
                Ok(read_state) => {
                    if !read_state.is_locked() {
                        break;
                    }
                }
                // if locked, wait
                Err(MutexError::State(MutexExceptions::AlreadyLocked)) => {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }

                // otherwise throw error
                Err(other_error) => return Err(other_error),
            }
            watchdog += 1;
            if watchdog > self.timeout {
                // timeout waiting, exceeded limit for mutexs
                return Err(MutexError::State(MutexExceptions::AlreadyLocked));
            }
        }

        Ok(self.to_owned())
    }

    /// gen alias
    pub async fn generate(&self) -> Result<Self, MutexError> {
        self.gen().await
    }

    /// alias for Default
    pub fn new() -> Self {
        // defaults
        Self {
            ..Default::default()
        }
    }
    /// specify timeout duration for reading mutex in seconds
    pub fn set_timeout(&mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self.to_owned()
    }
    /// specify file path for mutex file
    pub fn set_path(&mut self, path: &str) -> Self {
        self.path = String::from(path);
        self.to_owned()
    }

    /// set mutex to open - rewrites to file
    pub fn open(&mut self) -> Result<Self, MutexError> {
        self.state = MutexState::Open;
        self.state.sync(&self.path)?;
        Ok(self.to_owned())
    }
    /// set mutex to locked - rewrites to file
    pub fn lock(&mut self) -> Result<Self, MutexError> {
        self.state = MutexState::Locked;
        self.state.sync(&self.path)?;
        Ok(self.to_owned())
    }

    /// Opens mutex without modifying file
    pub fn local_lock(&mut self) -> Result<Self, MutexError> {
        self.state = MutexState::Locked;
        Ok(self.to_owned())
    }
    /// Locks mutex without modifying file
    pub fn lock_open(&mut self) -> Result<Self, MutexError> {
        self.state = MutexState::Open;
        Ok(self.to_owned())
    }

    // Write changes to file
    pub fn sync(&mut self) -> Result<Self, MutexError> {
        self.state.sync(&self.path)?;
        Ok(self.to_owned())
    }
}

/// Default options for Mutex
impl Default for Mutex {
    fn default() -> Self {
        Self {
            state: MutexState::Locked,
            path: String::from(".mutex.lock"),
            timeout: 10,
        }
    }
}
