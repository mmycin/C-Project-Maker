use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Download error: {0}")]
    Download(#[from] reqwest::Error),

    #[error("Zip extraction error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[allow(dead_code)]  // Suppress warning for unused variant
    #[error("UI error: {0}")]
    Ui(String),

    #[allow(dead_code)]  // Suppress warning for unused variant
    #[error("Invalid project configuration: {0}")]
    Config(String),

    #[allow(dead_code)]  // Suppress warning for unused variant
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type AppResult<T> = Result<T, AppError>;
