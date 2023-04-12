use thiserror::Error;

#[derive(Debug, Error)]
pub enum GrepError {
    GlobPatternError(#[from] glob::PatternError),
    
}
