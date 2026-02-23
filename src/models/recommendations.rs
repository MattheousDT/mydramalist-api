use crate::models::Image;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Detailed information about a single user recommendation.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Recommendation {
    /// Unique identifier for the recommended title.
    #[schema(example = "705665-tell-me-your-wish")]
    pub id: String,

    /// Primary English title of the recommended title.
    #[schema(example = "If You Wish Upon Me (2022)")]
    pub title: String,

    /// Poster images for the recommended title.
    pub poster: Option<Image>,

    /// Average user score for this title (out of 10).
    #[schema(example = 8.4)]
    pub score: Option<f32>,

    /// The HTML content of the recommendation.
    pub content: String,

    /// The user's display name who made the recommendation.
    #[schema(example = "Tee")]
    pub author: String,

    /// The user's unique identifier.
    #[schema(example = "messijoahae")]
    pub author_id: String,

    /// Number of likes this recommendation received.
    #[schema(example = 1)]
    pub likes: i32,

    /// Count of other users who also recommended this title.
    #[schema(example = 2)]
    pub read_more_count: i32,
}
