use crate::models::Image;
use serde::{Deserialize, Serialize};
use strum::Display;
use utoipa::ToSchema;

/// The watch status of the reviewer for the specific title.
#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "lowercase")]
pub enum ReviewStatus {
    /// The user has finished watching the title.
    #[serde(alias = "Completed")]
    Completed,
    /// The user is currently watching the title.
    #[serde(alias = "Ongoing")]
    Ongoing,
    /// The user has stopped watching the title before finishing.
    #[serde(alias = "Dropped")]
    Dropped,
}

/// Available sorting options for title reviews.
#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "lowercase")]
pub enum ReviewSort {
    /// Sort by the number of helpful votes.
    Helpful,
    /// Sort by the most recently published reviews.
    Recent,
}

/// A paginated collection of user reviews for a title.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TitleReviews {
    /// List of review details.
    pub reviews: Vec<Review>,

    /// The current page number.
    #[schema(example = 1)]
    pub page: i32,

    /// The total number of pages available.
    #[schema(example = 27)]
    pub total_pages: i32,
}

/// Detailed information about a single user review.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Review {
    /// Unique identifier for the review.
    #[schema(example = "143143")]
    pub id: String,

    /// Information about the user who authored the review.
    pub author: ReviewAuthor,

    /// Date the review was posted.
    #[schema(example = "May 31, 2021")]
    pub date: Option<String>,

    /// User's watch status at the time of the review.
    pub status: Option<ReviewStatus>,

    /// Number of episodes the user had watched when writing the review.
    #[schema(example = 10)]
    pub episodes_seen: Option<i32>,

    /// Breakdown of scores provided by the reviewer.
    pub scores: ReviewScores,

    /// Count of users who found this review helpful.
    #[schema(example = 260)]
    pub helpful_count: i32,

    /// Total number of comments on this review.
    #[schema(example = 12)]
    pub comments_count: i32,

    /// Indicates if the review content contains spoilers.
    pub spoilers: bool,

    /// The textual content of the review (HTML).
    pub content: String,
}

/// Public profile information of the review author.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct ReviewAuthor {
    /// The user's display name.
    #[schema(example = "CherryBunny")]
    pub name: String,

    /// The user's unique identifier or slug.
    #[schema(example = "CherryBunny")]
    pub id: String,

    /// The user's profile avatar.
    pub avatar: Option<Image>,
}

/// Numerical ratings for different aspects of the title.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct ReviewScores {
    /// The overall score given by the user (out of 10).
    #[schema(example = 9.0)]
    pub overall: Option<f32>,

    /// Rating for the script and plot.
    #[schema(example = 9.0)]
    pub story: Option<f32>,

    /// Rating for the performance of the actors.
    #[schema(example = 10.0)]
    pub acting_cast: Option<f32>,

    /// Rating for the soundtrack and background music.
    #[schema(example = 9.0)]
    pub music: Option<f32>,

    /// Rating indicating how likely the user would watch it again.
    #[schema(example = 10.0)]
    pub rewatch_value: Option<f32>,
}
