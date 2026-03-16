#[derive(Debug)]
pub enum AstoError {
    SyntaxError(String),
    ParamsError(String),   
    KeywordError(String),
}


impl std::fmt::Display for AstoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AstoError::KeywordError(e) => write!(f, "Asto : KeywordError - {}", e),
            AstoError::ParamsError(e) => write!(f, "Asto : ParamsError - {}", e),
            AstoError::SyntaxError(e) => write!(f, "Asto : SyntaxError - {}", e),
        }
    }
}


impl std::error::Error for AstoError {}