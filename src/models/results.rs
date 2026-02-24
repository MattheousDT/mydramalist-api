use std::sync::LazyLock;

use crate::models::{ArticleCategory, Country, Format, Type};
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

static MDL_IMAGE_URL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\/[A-z0-9_]+)([a-z])(\.)").expect("Invalid Regex"));

/// All the different image sizing that MDL offers.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq, Eq)]
pub struct Image {
    /// Approx. w150px - Used primarily on search results
    #[schema(example = "https://i.mydramalist.com/4v6zJ_4s.jpg")]
    pub small: String,
    /// Approx. w-300px - Used primarily on details page
    #[schema(example = "https://i.mydramalist.com/4v6zJ_4c.jpg")]
    pub cover: String,
    /// Full size image
    #[schema(example = "https://i.mydramalist.com/4v6zJ_4f.jpg")]
    pub full: String,
}

impl From<String> for Image {
    fn from(s: String) -> Self {
        Self {
            small: MDL_IMAGE_URL_RE.replace(&s, "${1}s${3}").to_string(),
            cover: MDL_IMAGE_URL_RE.replace(&s, "${1}c${3}").to_string(),
            full: MDL_IMAGE_URL_RE.replace(&s, "${1}f${3}").to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
/// A concise summary of a Drama, Movie, or TV Show found in search results.
pub struct TitleSearchResult {
    /// Unique identifier or slug
    #[schema(example = "685237-untitled-kim-eun-sook-project")]
    pub id: String,

    /// Primary title of the content.
    #[schema(example = "The Glory")]
    pub title: String,

    /// The type of title.
    #[schema(example = "drama")]
    pub r#type: Type,

    /// Format of the title.
    #[schema(example = "drama_special")]
    pub format: Option<Format>,

    /// The country of origin.
    #[schema(example = "south_korea")]
    pub country: Country,

    /// Release year.
    #[schema(example = 2022)]
    pub year: Option<i32>,

    /// Total number of episodes.
    #[schema(example = 8)]
    pub episodes: Option<i32>,

    /// A short snippet or synopsis of the title.
    #[schema(
        example = "A high school student dreams of becoming an architect. However, she had to drop out of school after suffering from brutal school violence. Years later, the perpetrator gets married and has a kid. Once the kid is in elementary…"
    )]
    pub description: Option<String>,

    /// User rating score (0.0 to 10.0).
    #[schema(example = 8.9)]
    pub rating: Option<f32>,

    /// Global rank position on MyDramaList.
    #[schema(example = 55)]
    pub ranking: Option<i32>,

    /// URLs for the poster image.
    pub poster: Option<Image>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq, Eq)]
/// A summary of an actor, actress, or crew member.
pub struct PeopleSearchResult {
    /// Unique identifier or slug
    #[schema(example = "1897-park-eun-bin")]
    pub id: String,

    /// Full name of the person.
    #[schema(example = "Park Eun Bin")]
    pub name: String,

    /// Citizenship or background
    #[schema(example = "South Korean")]
    pub nationality: Option<String>,

    /// Short biographical snippet.
    #[schema(
        example = "Park Eun Bin is a South Korean actress born in Seoul. She is managed by Namoo Actors. She debuted at the age of five and has acted in numerous television series as a child actress and younger versions of various characters. She played her first leading role in the time-traveling romance \"Operation Proposal\".…"
    )]
    pub bio: Option<String>,

    /// Total number of 'hearts' or fans on MyDramaList.
    #[schema(example = 7202)]
    pub hearts: Option<i32>,

    /// URLs for the person's profile picture.
    pub portrait: Option<Image>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq, Eq)]
/// Metadata for editorials, news, or recaps posted on MyDramaList.
pub struct ArticleSearchResult {
    /// Unique identifier for the article.
    #[schema(example = "awesome-kdrama-rom-coms-to-watch-on-any-day-part-1")]
    pub id: String,

    /// The headline of the article.
    #[schema(example = "Awesome Kdrama Rom-Coms to Watch On Any Day (Part 1)")]
    pub title: String,

    /// A brief summary or introduction of the article content.
    #[schema(
        example = "We are back with a selection of Kdrama rom-coms for you to choose from. Try to keep your Plan to Watch list under control!"
    )]
    pub r#abstract: String,

    /// The editorial category.
    #[schema(example = "editorials")]
    pub category: ArticleCategory,

    /// The publish date as formatted on MyDramaList.
    #[schema(example = "Jul 2, 2021")]
    pub date: String,

    /// Total likes or reactions received.
    #[schema(example = 712)]
    pub likes: i32,

    /// Number of user comments.
    #[schema(example = 224)]
    pub comments: i32,

    /// Image URLs for the article.
    pub image: Option<Image>,
}
