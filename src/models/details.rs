use crate::models::{Country, Genre, Image, Type};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Comprehensive details about a specific Drama, Movie, or TV Show.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TitleDetails {
    /// Unique identifier or slug for the title.
    #[schema(example = "735043-life")]
    pub id: String,

    /// Primary English title.
    #[schema(example = "When Life Gives You Tangerines")]
    pub title: String,

    /// Title in its original language.
    #[schema(example = "폭싹 속았수다")]
    pub native_title: Option<String>,

    /// Alternative names or translations for the title.
    pub aka: Vec<String>,

    /// Release year.
    #[schema(example = 2025)]
    pub year: Option<i32>,

    /// Poster images in various sizes.
    pub poster: Option<Image>,

    /// Description or summary of the plot.
    #[schema(
        example = "It is a story that resembles a tribute to our parents' tender and still youthful seasons..."
    )]
    pub synopsis: Option<String>,

    /// MyDramaList specific statistics for the title.
    pub statistics: Statistics,

    /// Country of origin.
    #[schema(example = "south_korea")]
    pub country: Option<Country>,

    /// Content type (e.g., Drama, Movie, TV Show).
    #[schema(example = "drama")]
    pub r#type: Option<Type>,

    /// Total number of episodes.
    #[schema(example = 16)]
    pub episodes: Option<i32>,

    /// Formatted string of the airing dates.
    #[schema(example = "Mar  7, 2025 - Mar 28, 2025")]
    pub aired: Option<String>,

    /// The day(s) of the week the title airs.
    #[schema(example = "Friday")]
    pub aired_on: Option<String>,

    /// The network(s) that originally broadcast the title.
    pub original_network: Vec<String>,

    /// Runtime duration per episode or total runtime for movies.
    #[schema(example = "1 hr. 2 min.")]
    pub duration: Option<String>,

    /// Age rating for the content.
    #[schema(example = "13+ - Teens 13 or older")]
    pub content_rating: Option<String>,

    /// List of directors.
    pub directors: Vec<CrewMember>,

    /// List of screenwriters.
    pub screenwriters: Vec<CrewMember>,

    /// Categorized genres.
    pub genres: Vec<Genre>,

    /// User-voted descriptive tags.
    pub tags: Vec<Tag>,

    /// Available streaming platforms or services.
    pub where_to_watch: Vec<WhereToWatch>,

    /// Highlighted cast members (usually the main cast and top supporting).
    pub cast: Vec<CastMember>,

    /// Total number of cast members credited.
    #[schema(example = 129)]
    pub cast_count: Option<i32>,

    /// Highlighted promotional photos or stills.
    pub photos: Vec<Image>,

    /// Total number of photos available.
    #[schema(example = 84)]
    pub photos_count: Option<i32>,

    /// Similar titles recommended by users.
    pub recommendations: Vec<RecommendationPreview>,
}

/// Statistics and rankings from MyDramaList users.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Statistics {
    /// Average user score (out of 10).
    #[schema(example = 9.3)]
    pub score: Option<f32>,

    /// Total number of users who rated the title.
    #[schema(example = 61286)]
    pub votes: Option<i32>,

    /// Total number of users who have added this title to their lists.
    #[schema(example = 132894)]
    pub watchers: Option<i32>,

    /// Total number of user reviews.
    #[schema(example = 463)]
    pub reviews_count: Option<i32>,

    /// Global rank among all titles on MDL.
    #[schema(example = 8)]
    pub rank: Option<i32>,

    /// Global popularity rank based on watchers.
    #[schema(example = 64)]
    pub popularity: Option<i32>,

    /// Number of users who favorited the title.
    #[schema(example = 0)]
    pub favorites: Option<i32>,
}

/// A person credited in the crew (e.g., Director, Screenwriter).
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct CrewMember {
    /// Unique identifier for the person.
    #[schema(example = "15891-kim-won-suk")]
    pub id: String,

    /// Full name of the crew member.
    #[schema(example = "Kim Won Suk")]
    pub name: String,

    /// The role or job of the crew member (e.g., Director, Screenwriter).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = "Director")]
    pub job: Option<String>,

    /// Profile image of the crew member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
}

/// A descriptive tag applied to the title.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Tag {
    /// Internal MyDramaList tag ID.
    #[schema(example = 1470)]
    pub id: i32,

    /// Name of the tag (e.g., "Family Relationship").
    #[schema(example = "Family Relationship")]
    pub name: String,
}

/// Information on where a title can be watched legally.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct WhereToWatch {
    /// Name of the streaming service or store.
    #[schema(example = "Netflix")]
    pub name: String,

    /// Type of availability (e.g., Subscription, Free, Rent/Buy).
    #[schema(example = "Subscription (sub)")]
    pub service_type: String,

    /// Direct link to the title on the service.
    #[schema(example = "https://www.netflix.com/title/81681535")]
    pub link: String,
}

/// A person credited in the cast.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct CastMember {
    /// Unique identifier for the person.
    #[schema(example = "426-iu")]
    pub person_id: String,

    /// Full name of the actor/actress.
    #[schema(example = "IU")]
    pub name: String,

    /// Name of the character played.
    #[schema(example = "O Ae Sun | Yang Geum Myeong")]
    pub character: Option<String>,

    /// Unique identifier for the character (if available).
    #[schema(example = "cho-sang-gu")]
    pub character_id: Option<String>,

    /// Role classification (e.g., Main Role, Support Role).
    #[schema(example = "Main Role")]
    pub role: Option<String>,

    /// Portrait images of the cast member.
    pub image: Option<Image>,
}

/// A user-recommended title similar to the current one.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct RecommendationPreview {
    /// Unique identifier for the recommended title.
    #[schema(example = "13544-reply-1988")]
    pub id: String,

    /// Primary English title of the recommended title.
    #[schema(example = "Reply 1988")]
    pub title: String,

    /// Poster images for the recommended title.
    pub poster: Option<Image>,
}
