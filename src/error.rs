use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("The destination hasn't been set.")]
    NoDestinationRect,
}
