// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::{error::Error, fmt};

// ============================================================
// CUSTOM ERROS (TECTONIC-ERROR)
// ============================================================

#[derive(Debug)]
pub enum TectonicError {          // Simple Error with custom messaging
    InvalidInputError { what: &'static str, got: String},
    CacheLimitError { size: usize, limit: usize }
}


impl fmt::Display for TectonicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TectonicError::InvalidInputError { what, got} => 
                write!(f, "Invalid Input format: Expected value {} - Recieved value {}", what, got),
            TectonicError::CacheLimitError { size, limit } =>
                write!(f, "Cache Limit Exceeded: Current size {} > Max entries {}", size, limit)
        }
    }
}

// Provide Error trait functionality for TectonicError (dyn)
impl Error for TectonicError {}