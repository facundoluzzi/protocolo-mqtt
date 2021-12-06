use super::wildcard_handler::Wildcard;

pub enum WildcardResult {
    HasWildcard(Wildcard),
    HasNoWildcard,
    InvalidWildcard,
}