use std::fmt;

#[derive(Debug)]
pub enum AnonymizationError {
    Pattern(PatternError),
    Io(std::io::Error),
    Legend(LegendError),
}

#[derive(Debug)]
pub enum PatternError {
    InvalidRegex(String),
    ProcessingFailed(String),
}

#[derive(Debug)]
pub enum LegendError {
    FormatError(String),
}

impl fmt::Display for AnonymizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnonymizationError::Pattern(err) => write!(f, "Pattern error: {}", err),
            AnonymizationError::Io(err) => write!(f, "IO error: {}", err),
            AnonymizationError::Legend(err) => write!(f, "Legend error: {}", err),
        }
    }
}

impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatternError::InvalidRegex(msg) => write!(f, "Invalid regex: {}", msg),
            PatternError::ProcessingFailed(msg) => write!(f, "Processing failed: {}", msg),
        }
    }
}

impl fmt::Display for LegendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LegendError::FormatError(msg) => write!(f, "Format error: {}", msg),
        }
    }
}

impl std::error::Error for AnonymizationError {}
impl std::error::Error for PatternError {}
impl std::error::Error for LegendError {}

impl From<PatternError> for AnonymizationError {
    fn from(err: PatternError) -> Self {
        AnonymizationError::Pattern(err)
    }
}

impl From<std::io::Error> for AnonymizationError {
    fn from(err: std::io::Error) -> Self {
        AnonymizationError::Io(err)
    }
}

impl From<LegendError> for AnonymizationError {
    fn from(err: LegendError) -> Self {
        AnonymizationError::Legend(err)
    }
}

impl From<regex::Error> for PatternError {
    fn from(err: regex::Error) -> Self {
        PatternError::InvalidRegex(err.to_string())
    }
}