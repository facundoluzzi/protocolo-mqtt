use crate::wildcard::wildcard_handler::Wildcard;

/// Enum de structs relacionado con la obtencion de las wildcards.
pub enum WildcardResult {
    HasWildcard(Wildcard),
    HasNoWildcard,
    InvalidWildcard,
}
