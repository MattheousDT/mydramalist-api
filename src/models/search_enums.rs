use serde::{Deserialize, Serialize};
use strum::Display;
use utoipa::ToSchema;

// The aliases here are for parsing search results

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum Type {
    #[serde(alias = "Drama")]
    Drama = 68,
    #[serde(alias = "Movie")]
    Movie = 77,
    #[serde(rename = "tv_show", alias = "TV Show")]
    TVShow = 86,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum Country {
    #[serde(alias = "Japanese")]
    Japan = 1,
    #[serde(alias = "Chinese")]
    China = 2,
    #[serde(alias = "South Korean", alias = "Korean")]
    SouthKorea = 3,
    #[serde(alias = "Taiwanese")]
    Taiwan = 4,
    #[serde(alias = "Hong Kong")]
    HongKong = 5,
    #[serde(alias = "Thai")]
    Thailand = 6,
    #[serde(alias = "Filipino")]
    Philippines = 7,
    #[serde(alias = "Singaporean")]
    Singapore = 8,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum Status {
    Ongoing = 1,
    Upcoming = 2,
    Completed = 3,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum DramaFormat {
    StandardSeries = 7,
    WebSeries = 8,
    VerticalSeries = 10,
    DramaSpecial = 14,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum MovieFormat {
    FeatureFilm = 24,
    ShortFilm = 26,
    IndependentFilm = 28,
    MadeForTVMovie = 30,
    Documentary = 32,
    AnimatedFilm = 34,
    Other = 36,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum TVShowFormat {
    TalkShow = 5,
    VarietyShow = 6,
    DocumentaryProgram = 16,
    MusicProgram = 18,
    RealityProgram = 20,
    OtherProgram = 22,
    AnimatedProgram = 38,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Format {
    Drama(DramaFormat),
    Movie(MovieFormat),
    TV(TVShowFormat),
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum Genre {
    Action = 1,
    Adventure = 5,
    Animals = 9,
    Business = 13,
    Comedy = 17,
    Crime = 21,
    Detective = 38,
    Documentary = 35,
    Drama = 25,
    Family = 29,
    Fantasy = 32,
    Food = 2,
    Friendship = 6,
    Historical = 10,
    Horror = 14,
    Investigation = 42,
    Law = 18,
    Life = 22,
    Manga = 44,
    MartialArts = 26,
    Mature = 40,
    Medical = 30,
    Melodrama = 33,
    Military = 3,
    Music = 7,
    Mystery = 11,
    Political = 41,
    Psychological = 15,
    Romance = 19,
    School = 23,
    SciFi = 27,
    Sitcom = 36,
    Sports = 31,
    Supernatural = 34,
    Suspense = 4,
    Thriller = 8,
    Tokusatsu = 12,
    Tragedy = 39,
    Vampire = 16,
    War = 37,
    Western = 45,
    Wuxia = 20,
    Youth = 24,
    Zombies = 28,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum Nationality {
    Japanese = 1,
    Chinese = 2,
    Korean = 3,
    HongKonger = 4,
    Taiwanese = 5,
    Thai = 6,
    Filipino = 140,
    Singaporean = 157,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum Gender {
    Male = 77,
    Female = 70,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display)]
#[serde(rename_all = "lowercase")]
#[repr(i32)]
pub enum ArticleCategory {
    #[serde(alias = "Interviews")]
    Interviews = 1,
    #[serde(alias = "Editorials")]
    Editorials = 2,
    #[serde(alias = "MDL")]
    MDL = 3,
    #[serde(alias = "News")]
    News = 4,
    #[serde(alias = "Recaps")]
    Recaps = 5,
    #[serde(alias = "Announcement")]
    Announcement = 7,
    #[serde(alias = "Articles")]
    Articles = 9,
    #[serde(alias = "K-Pop")]
    KPop = 10,
    #[serde(alias = "Celebrity")]
    Celebrity = 12,
    #[serde(alias = "K-Drama")]
    KDrama = 14,
    #[serde(alias = "Trailer")]
    Trailer = 16,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "lowercase")]
pub enum TitleSort {
    Relevance,
    #[strum(to_string = "popular")]
    MostPopular,
    #[strum(to_string = "top")]
    TopRanked,
    #[strum(to_string = "rated")]
    TopRated,
    #[strum(to_string = "newest")]
    Newest,
    #[strum(to_string = "date")]
    ReleaseDate,
    #[strum(to_string = "recently")]
    RecentlyAdded,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "lowercase")]
pub enum PeopleSort {
    Relevance,
    #[strum(to_string = "popular")]
    MostPopular,
    #[strum(to_string = "recently")]
    RecentlyAdded,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "lowercase")]
pub enum ArticleSort {
    Relevance,
    #[strum(to_string = "popular")]
    MostPopular,
    #[strum(to_string = "publish")]
    PublishDate,
}
