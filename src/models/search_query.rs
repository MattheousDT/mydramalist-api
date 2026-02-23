use crate::models::*;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
/// Advanced search parameters for finding Dramas, Movies, and TV Shows.
pub struct TitleSearchQuery {
    /// The page number to retrieve.
    #[param(example = 1)]
    pub page: Option<i32>,

    /// Keyword search.
    #[param(example = "The Glory")]
    pub q: Option<String>,

    /// Filter by specific type.
    #[param(example = "drama")]
    pub r#type: Option<Type>,

    /// Filter by production country.
    #[param(example = "south_korea")]
    pub country: Option<Country>,

    /// Filter by specific format.
    #[param(example = "standard_series")]
    pub format: Option<Format>,

    /// Main genre to filter by.
    pub genre: Option<Genre>,

    /// Internal MDL Tag ID.
    pub tag: Option<i32>,

    /// Internal MDL Network ID.
    pub network: Option<i32>,

    /// Internal MDL Service ID.
    pub service: Option<i32>,

    /// Minimum release year.
    #[param(example = 2022)]
    pub release_year_min: Option<i32>,

    /// Maximum release year.
    #[param(example = 2023)]
    pub release_year_max: Option<i32>,

    /// Minimum rating score (0-10).
    #[param(example = 8)]
    pub rating_min: Option<i32>,

    /// Maximum rating score (0-10).
    #[param(example = 10)]
    pub rating_max: Option<i32>,

    /// Minimum total episodes.
    #[param(example = 8)]
    pub episodes_min: Option<i32>,

    /// Maximum total episodes.
    #[param(example = 16)]
    pub episodes_max: Option<i32>,

    /// Minimum episode runtime in minutes.
    #[param(example = 45)]
    pub runtime_min: Option<i32>,

    /// Maximum episode runtime in minutes.
    #[param(example = 60)]
    pub runtime_max: Option<i32>,

    /// Production status.
    #[param(example = "completed")]
    pub status: Option<Status>,

    /// Ordering of results.
    #[param(example = "top_rated")]
    pub sort: Option<TitleSort>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
/// Parameters for filtering and sorting user reviews for a title.
pub struct ReviewSearchQuery {
    /// The page number to retrieve.
    #[param(example = 1)]
    pub page: Option<i32>,

    /// Filter reviews by the author's watch status.
    #[param(example = "completed")]
    pub status: Option<ReviewStatus>,

    /// Sort order for the reviews.
    #[param(example = "helpful")]
    pub sort: Option<ReviewSort>,

    /// Whether to filter out reviews containing spoilers.
    #[param(example = true)]
    pub hide_spoiler: Option<bool>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
/// Search parameters for finding Actors, Actresses, and Production Staff.
pub struct PeopleSearchQuery {
    /// The page number to retrieve.
    #[param(example = 1)]
    pub page: Option<i32>,

    /// Name of the person.
    #[param(example = "Park Eun-Bin")]
    pub q: Option<String>,

    /// Nationality of the person.
    #[param(example = "korean")]
    pub nationality: Option<Nationality>,

    /// Gender of the person.
    #[param(example = "female")]
    pub gender: Option<Gender>,

    /// Ordering of results.
    #[param(example = "most_popular")]
    pub sort: Option<PeopleSort>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
/// Search parameters for MDL Articles, Editorials, and News.
pub struct ArticleSearchQuery {
    /// The page number to retrieve.
    #[param(example = 1)]
    pub page: Option<i32>,

    /// Keywords for the article content.
    #[param(example = "The Glory")]
    pub q: Option<String>,

    /// Filter by article category.
    #[param(example = "news")]
    pub category: Option<ArticleCategory>,

    /// Ordering of results.
    #[param(example = "publish_date")]
    pub sort: Option<ArticleSort>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
/// Standard pagination query parameters.
pub struct PaginationQuery {
    /// The page number to retrieve.
    #[param(example = 1)]
    pub page: Option<i32>,
}
