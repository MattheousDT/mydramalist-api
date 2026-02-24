use crate::models::{Format, Gender, Image, Nationality, Type};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Comprehensive details about a specific actor, actress, or crew member.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct PersonDetails {
    /// Unique identifier or slug for the person.
    #[schema(example = "1897-park-eun-bin")]
    pub id: String,

    /// Full name of the person.
    #[schema(example = "Park Eun Bin")]
    pub name: String,

    /// The person's first or given name.
    #[schema(example = "Eun Bin")]
    pub given_name: Option<String>,

    /// The person's last or family name.
    #[schema(example = "Park")]
    pub family_name: Option<String>,

    /// The person's name in their native language.
    #[schema(example = "박은빈")]
    pub native_name: Option<String>,

    /// Alternative names or romanizations.
    pub aka: Vec<String>,

    /// The person's nationality.
    #[schema(example = "korean")]
    pub nationality: Option<Nationality>,

    /// The person's gender.
    #[schema(example = "female")]
    pub gender: Option<Gender>,

    /// The date the person was born.
    #[schema(example = "September 4, 1992")]
    pub born: Option<String>,

    /// The current age of the person.
    #[schema(example = 33)]
    pub age: Option<i32>,

    /// The biography of the person.
    #[schema(
        example = "Park Eun Bin is a South Korean actress born in Seoul. She is managed by Namoo Actors..."
    )]
    pub biography: Option<String>,

    /// Total number of users who follow this person.
    #[schema(example = 0)]
    pub followers: i32,

    /// Total number of users who favourited (hearted) this person.
    #[schema(example = 7264)]
    pub hearts: i32,

    /// Portrait images of the person.
    pub portrait: Option<Image>,

    /// A list of titles the person has worked on.
    pub works: Vec<PersonWork>,
}

/// A specific title (Drama, Movie, Special, or TV Show) the person has worked on.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct PersonWork {
    /// Unique identifier for the title.
    #[schema(example = "760649-hyper-knife")]
    pub id: String,

    /// Primary title name.
    #[schema(example = "Hyper Knife")]
    pub title: String,

    /// Type of the title.
    #[schema(example = "drama")]
    pub r#type: Type,

    /// Specific sub-format (if applicable).
    pub format: Option<Format>,

    /// Release year of the title.
    #[schema(example = 2025)]
    pub year: Option<i32>,

    /// Total episodes (may be omitted for movies).
    #[schema(example = 8)]
    pub episodes: Option<i32>,

    /// The name of the role played or job title.
    #[schema(example = "Jung Se Ok")]
    pub role: Option<String>,

    /// The classification of the role (e.g., Main Role, Support Role, Guest).
    #[schema(example = "Main Role")]
    pub role_type: Option<String>,

    /// The user rating of the title on MDL.
    #[schema(example = 7.9)]
    pub rating: Option<f32>,
}
