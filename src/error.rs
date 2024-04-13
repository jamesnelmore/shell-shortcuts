use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Invalid shortcut: {0}")]
    InvalidShortcut(String),

    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Could not parse {0} into Alias")]
    ParseFailed(String),

    #[error("Could not find {0}")]
    ShortcutNotFound(String),

    #[error("IO issue")]
    IOError(#[from] std::io::Error),

    #[error("Other issue")]
    General(String),
}
