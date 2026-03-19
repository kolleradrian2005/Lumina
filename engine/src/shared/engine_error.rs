use std::fmt::Display;

#[derive(Debug)]
pub enum EngineError {
    Generic(String),
    FileNotFound(String),
    ShaderCompilation(String, String), // (shader path, error log)
}

impl Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::Generic(msg) => write!(f, "{}", msg),
            EngineError::FileNotFound(path) => write!(f, "File not found: {}", path),
            EngineError::ShaderCompilation(path, err) => {
                write!(f, "Shader compilation failed: '{}': {}", path, err)
            }
        }
    }
}
