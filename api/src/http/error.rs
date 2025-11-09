use axum::response::{IntoResponse, Response};
use shared::Error;

pub struct HttpError(Error);

impl From<Error> for HttpError {
    fn from(error: Error) -> Self {
        HttpError(error)
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        match self.0 {
            Error::Application(application_error) => todo!(),
            Error::Domain(domain_error) => todo!(),
            Error::Infrastructure(infrastructure_error) => todo!(),
            Error::InternalServerError(_) => todo!(),
        }
    }
}
