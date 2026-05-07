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

#[derive(Debug, Clone, PartialEq)]
pub enum TectonicError {          // Simple Error with custom messaging
    InvalidInput {
        context: &'static str,
        what: &'static str, 
        got: String},

    InvalidParameter {
        param: &'static str,
        issue: &'static str
    },

    RequiredField { 
        field: &'static str 
    },

    CapacityExceeded {
        component: &'static str, 
        size: usize, 
        limit: usize 
    },

    NotFound {
        component: &'static str,
        key: &'static str,
    },

    Arena { 
        message: &'static str 
    },

    Repository { 
        message: &'static str 
    },

    Centroid { 
        message: &'static str 
    },

    Location { 
        message: &'static str 
    },

    Quantization { 
        message: &'static str 
    },

    Search { 
        message: &'static str 
    },

    Admission { 
        message: &'static str 
    },

    Eviction { 
        message: &'static str 
    },

    InconsistentState { 
        message: &'static str 
    },
}

impl TectonicError {
    #[inline]
    pub fn kind(&self) -> TectonicErrorKind {
        match self {
            TectonicError::InvalidInput { .. } => TectonicErrorKind::InvalidInput,
            TectonicError::InvalidParameter { .. } => TectonicErrorKind::InvalidParameter,
            TectonicError::RequiredField { .. } => TectonicErrorKind::RequiredField,
            TectonicError::CapacityExceeded { .. } => TectonicErrorKind::CapacityExceeded,
            TectonicError::NotFound { .. } => TectonicErrorKind::NotFound,
            TectonicError::Arena { .. } => TectonicErrorKind::Arena,
            TectonicError::Repository { .. } => TectonicErrorKind::Repository,
            TectonicError::Centroid { .. } => TectonicErrorKind::Centroid,
            TectonicError::Location { .. } => TectonicErrorKind::Location,
            TectonicError::Quantization { .. } => TectonicErrorKind::Quantization,
            TectonicError::Search { .. } => TectonicErrorKind::Search,
            TectonicError::Admission { .. } => TectonicErrorKind::Admission,
            TectonicError::Eviction { .. } => TectonicErrorKind::Eviction,
            TectonicError::InconsistentState { .. } => TectonicErrorKind::InconsistentState,
        }
    }

    #[inline]
    pub fn invalid_input(context: &'static str, what: &'static str, got: String) -> Self {
        TectonicError::InvalidInput { context, what, got }
    }

    #[inline]
    pub fn invalid_parameter(param: &'static str, issue: &'static str) -> Self {
        TectonicError::InvalidParameter { param, issue }
    }

    #[inline]
    pub fn required_field(field: &'static str) -> Self {
        TectonicError::RequiredField { field }
    }

    #[inline]
    pub fn capacity_exceeded(component: &'static str, size: usize, limit: usize) -> Self {
        TectonicError::CapacityExceeded { component, size, limit }
    }

    #[inline]
    pub fn not_found(component: &'static str, key: &'static str) -> Self {
        TectonicError::NotFound { component, key }
    }

    #[inline]
    pub fn arena(message: &'static str) -> Self {
        TectonicError::Arena { message }
    }

    #[inline]
    pub fn repository(message: &'static str) -> Self {
        TectonicError::Repository { message }
    }

    #[inline]
    pub fn centroid(message: &'static str) -> Self {
        TectonicError::Centroid { message }
    }

    #[inline]
    pub fn location(message: &'static str) -> Self {
        TectonicError::Location { message }
    }

    #[inline]
    pub fn quantization(message: &'static str) -> Self {
        TectonicError::Quantization { message }
    }

    #[inline]
    pub fn search(message: &'static str) -> Self {
        TectonicError::Search { message }
    }

    #[inline]
    pub fn admission(message: &'static str) -> Self {
        TectonicError::Admission { message }
    }

    #[inline]
    pub fn eviction(message: &'static str) -> Self {
        TectonicError::Eviction { message }
    }

    #[inline]
    pub fn inconsistent_state(message: &'static str) -> Self {
        TectonicError::InconsistentState { message }
    }
}


impl fmt::Display for TectonicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput { context, what, got } => 
            write!(f, "Invalid input in {}: expected {}, got {}", context, what, got),
            Self::InvalidParameter { param, issue } => 
            write!(f, "Invalid parameter {}: {}", param, issue),
            Self::RequiredField { field } => 
            write!(f, "Required field {}: must be provided", field),
            Self::CapacityExceeded { component, size, limit } => 
            write!(f, "Capacity exceeded for {}: size {} > limit {}", component, size, limit),
            Self::NotFound { component, key } => 
            write!(f, "Not found: {} with key {}", component, key),
            Self::Arena { message } => 
            write!(f, "Arena error: {}", message),
            Self::Repository { message } => 
            write!(f, "Repository error: {}", message),
            Self::Centroid { message } => 
            write!(f, "Centroid error: {}", message),
            Self::Location { message } => 
            write!(f, "Location error: {}", message),
            Self::Quantization { message } => 
            write!(f, "Quantization error: {}", message),
            Self::Search { message } => 
            write!(f, "Search error: {}", message),
            Self::Admission { message } => 
            write!(f, "Admission error: {}", message),
            Self::Eviction { message } => 
            write!(f, "Eviction error: {}", message),
            Self::InconsistentState { message } => 
            write!(f, "Inconsistent state error: {}", message),
        }
    }
}

// Provide Error trait functionality for TectonicError (dyn)
impl Error for TectonicError {}