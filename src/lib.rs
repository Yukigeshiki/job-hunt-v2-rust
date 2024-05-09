use thiserror::Error;

pub mod repository;
pub mod scraper;
pub mod site;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Error retrieving selector group. {0}")]
    Selector(String),

    #[error("Error making request to '{0}'. {1}")]
    Request(String, String),

    #[error("Error decoding HTML. {0}")]
    Decode(String),

    #[error("Error connecting to DB. {0}")]
    SqliteConnection(String),

    #[error("Error querying DB. {0}")]
    SqliteQuery(String),

    #[error("Failed to serialise/deserialise tags array: {0}")]
    Serialisation(String),
}
