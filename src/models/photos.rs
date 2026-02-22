use crate::models::Image;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A collection of photos for a specific title, including pagination information.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TitlePhotos {
    /// List of photos.
    pub photos: Vec<Image>,

    /// The current page number.
    #[schema(example = 1)]
    pub page: i32,

    /// The total number of pages available.
    #[schema(example = 3)]
    pub total_pages: i32,
}
