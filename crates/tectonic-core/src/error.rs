// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::fmt;

// ============================================================
// CUSTOM ERROS (TECTONIC-ERROR)
// ============================================================

#[derive(Debug)]
pub struct TectonicError {          // Simple Error with custom messaging
    message: String,
}

// Add .into() type conversion to TectonicError to ensure String messaging
impl TectonicError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}


impl fmt::Display for TectonicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Provide Error trait functionality for TectonicError (dyn)
impl std::error::Error for TectonicError {}