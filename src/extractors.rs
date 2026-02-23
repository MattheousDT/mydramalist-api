use crate::error::AppError;
use axum::{extract::FromRequestParts, http::request::Parts};
use serde::de::DeserializeOwned;

/// A wrapper around axum::extract::Query that converts deserialization errors
/// into a JSON-formatted AppError::BadRequest instead of plaintext.
pub struct Query<T>(pub T);

impl<T, S> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Query::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Query(value.0)),
            Err(rejection) => Err(AppError::BadRequest(rejection.to_string())),
        }
    }
}
