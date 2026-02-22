use crate::models::Image;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Details about a specific episode of a Drama or TV Show.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Episode {
    /// The episode number.
    #[schema(example = 1)]
    pub episode_number: i32,

    /// The title of the episode.
    #[schema(example = "Move to Heaven Episode 1")]
    pub title: String,

    /// Average user score for this episode (out of 10).
    #[schema(example = 8.7)]
    pub score: Option<f32>,

    /// Total number of users who rated this episode.
    #[schema(example = 71)]
    pub votes: Option<i32>,

    /// The date the episode originally aired.
    #[schema(example = "May 14, 2021")]
    pub air_date: Option<String>,

    /// A brief summary or synopsis of the episode.
    #[schema(
        example = "A factory intern’s fatal injury brings Han Jeong-u and Han Geu-ru to his room, where his goals for the future and love for his parents come into view."
    )]
    pub summary: Option<String>,

    /// The thumbnail or cover image for the episode.
    pub cover: Option<Image>,
}
