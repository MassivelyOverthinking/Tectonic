// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::{error::Error, fmt::{self}};

// ============================================================
// TECTONIC ERROR
// ============================================================
// Error definitions for complete Tectonic project.
// ---
// This module provides a single project-wide error type used consitently throughout the codebase 
// by configuration, arena storage, routing, quantization, similarity search, eviction, admission,
// and location tracking.
// ---
// Design goals:
// - keep hot-path errors cheap to construct.
// - provide structured context for debugging.
// - avoid stringly-typed error categories spread across the codebase.
// - make invalid internal state explicit and easy to detect.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TectonicErrorKind {
    InvalidInput,
    InvalidParameter,
    RequiredField,
    CapacityExceeded,
    NotFound,
    Arena,
    Repository,
    Centroid,
    Location,
    Quantization,
    Search,
    Admission,
    Eviction,
    InconsistentState,
}

#[derive(Debug)]
pub enum TectonicError {          // Simple Error with custom messaging
    InvalidInputError { what: &'static str, got: String},
    InvalidParamaterError { param: &'static str, issue: &'static str},
    RequiredFieldError { field: &'static str },
    CacheLimitError { size: usize, limit: usize },
    ArenaError { message: &'static str },
    RepoError { message: &'static str },
    CentroidError { message: &'static str },
    QuantizationError { message: &'static str },
    InvalidVectorError { index: usize },
    InconsistenStateError { message: &'static str },
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
                write!(f, "Arena Storage Error: {}", message),
            TectonicError::RepoError { message } =>
                write!(f, "Repository Storage Error: {}", message),
            TectonicError::CentroidError { message } => 
                write!(f, "Centroid Error: {}", message),
            TectonicError::QuantizationError { message } =>
                write!(f, "Quantization Error: {}", message),
            TectonicError::InvalidVectorError { index } =>
                write!(f, "Invalid Vector Error: Value at index {} is invalid", index),
            TectonicError::InconsistenStateError { message } =>
                write!(f, "Inconsistent State: {}", message)
        }
    }
}

// Provide Error trait functionality for TectonicError (dyn)
impl Error for TectonicError {}