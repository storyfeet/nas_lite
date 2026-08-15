use axum::http::StatusCode;
use axum::response::IntoResponse;
use err_tools::traceable::*;

pub struct NasRes<T: IntoResponse>(Result<T, TraceError>);

impl<T: IntoResponse> IntoResponse for NasRes<T> {
    fn into_response(self) -> axum::response::Response {
        match self.0 {
            Ok(t) => t.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}

impl<T: IntoResponse> NasRes<T> {
    pub fn ok(t: T) -> NasRes<T> {
        NasRes::<T>(Ok(t))
    }

    pub fn err(e: TraceError) -> NasRes<T> {
        NasRes::<T>(Err(e))
    }
}

impl<T: IntoResponse> From<TraceError> for NasRes<T> {
    fn from(value: TraceError) -> Self {
        Self::err(value)
    }
}

impl<T: IntoResponse> From<Result<T, TraceError>> for NasRes<T> {
    fn from(value: Result<T, TraceError>) -> Self {
        NasRes(value)
    }
}
