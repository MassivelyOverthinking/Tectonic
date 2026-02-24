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
    InvalidParamaterError { param: &'static str, issue: &'static str},
    RequiredFieldError { field: &'static str },
    CacheLimitError { size: usize, limit: usize },
    ArenaError { message: &'static str }
}


impl fmt::Display for TectonicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TectonicError::InvalidInputError { what, got} => 
                write!(f, "Invalid Input format: Expected value {} - Recieved value {}", what, got),
            TectonicError::InvalidParamaterError { param, issue } =>
                write!(f, "Invalid Paramater: The paramater {}, must {}", param, issue),
            TectonicError::RequiredFieldError { field } => 
                write!(f, "Required Field: The parameter {} is required field and must be filled!", field),
            TectonicError::CacheLimitError { size, limit } =>
                write!(f, "Cache Limit Exceeded: Current size {} > Max entries {}", size, limit),
            TectonicError::ArenaError { message } => 
                write!(f, "Arena Storage Error: {}", message)
        }
    }
}

// Provide Error trait functionality for TectonicError (dyn)
impl Error for TectonicError {}