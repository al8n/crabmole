/// Pipe
#[cfg(feature = "io")]
#[cfg_attr(docsrs, doc(cfg(feature = "io")))]
pub mod pipe;

/// Pipe
#[cfg(feature = "async-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-io")))]
pub mod async_pipe;

/// Error for pipe
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PipeError {
    /// Read/Write on closed pipe
    Closed,
    /// EOF
    Eof,
}

impl PipeError {
    /// Returns true if the error is [`PipeError::Eof`]
    #[inline]
    pub const fn is_eof(&self) -> bool {
        matches!(self, Self::Eof)
    }

    /// Returns true if the error is [`PipeError::Closed`]
    #[inline]
    pub const fn is_closed(&self) -> bool {
        matches!(self, Self::Closed)
    }
}

impl core::fmt::Display for PipeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PipeError::Closed => write!(f, "Pipe: read/write on closed pipe"),
            PipeError::Eof => write!(f, "Pipe: EOF"),
        }
    }
}

impl std::error::Error for PipeError {}
