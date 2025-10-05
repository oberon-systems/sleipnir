use std::env;

pub struct EnvSetter {
    keys: Vec<String>,
}

// Safe env setter with auto-cleanup
#[allow(dead_code)]
impl EnvSetter {
    // create new instance
    pub fn new() -> Self {
        EnvSetter { keys: Vec::new() }
    }
    // setter
    pub fn set(&mut self, key: &str, value: &str) {
        unsafe {
            env::set_var(key, value);
        }
        self.keys.push(key.to_string());
    }
    // remover
    pub fn del(&mut self, key: &str) {
        unsafe {
            env::remove_var(key);
        }
        // remove key from vector if exists
        self.keys.retain(|k| k != key);
    }
}

// Resource Acquisition Is Initialization
impl Drop for EnvSetter {
    fn drop(&mut self) {
        for key in &self.keys {
            unsafe {
                env::remove_var(key);
            }
        }
    }
}
