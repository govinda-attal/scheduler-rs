use tonic::Code;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Generic: {0}")]
    Generic(String),

    #[error("BadRequest: {0}")]
    BadRquest(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    SqlxMigrateError(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    TonicTransportError(#[from] tonic::transport::Error),
}

impl From<Error> for tonic::Status {
    fn from(e: Error) -> Self {
        use tonic::{Code, Status};
        match e {
            Error::BadRquest(str) => Status::new(Code::InvalidArgument, str),
            Error::Generic(str) => Status::new(Code::Internal, str),
            _ => Status::new(Code::Internal, e.to_string()),
        }
    }
}
