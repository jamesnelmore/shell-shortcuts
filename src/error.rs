use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Provided shortcut was invalid")]
    InvalidShortcut,
    #[error("Provided command was invalid")]
    InvalidCommand,
    #[error("Provided string was invalid")]
    InvalidString,
    #[error("IO issue")]
    IOError(#[from] std::io::Error),
    #[error("Other issue")]
    General(String),
}

// TODO test for failing parses
