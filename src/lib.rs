pub mod error;
pub mod patterns;
pub mod legend;
pub mod anonymizer;

pub use error::{AnonymizationError, PatternError, LegendError};
pub use legend::{AnonymizationMap, format_legend};
pub use anonymizer::{Options, anonymize_text};