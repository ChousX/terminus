#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    QuerySingleError(#[from] bevy::ecs::query::QuerySingleError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<X> = std::result::Result<X, Error>;
