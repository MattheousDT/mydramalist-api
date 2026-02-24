use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use utoipa::ToSchema;

#[derive(
    Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display, EnumString, PartialEq, Eq,
)]
#[serde(rename_all = "snake_case")]
#[strum(ascii_case_insensitive)]
#[repr(i32)]
pub enum Type {
    #[strum(serialize = "drama", serialize = "special")]
    Drama = 68,
    Movie = 77,
    #[serde(rename = "tv_show")]
    #[strum(serialize = "tv show", serialize = "tv_show")]
    TVShow = 86,
}

#[derive(
    Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display, EnumString, PartialEq, Eq,
)]
#[serde(rename_all = "snake_case")]
#[strum(ascii_case_insensitive)]
#[repr(i32)]
pub enum Country {
    #[strum(serialize = "japan", serialize = "japanese")]
    Japan = 1,
    #[strum(serialize = "china", serialize = "chinese")]
    China = 2,
    #[strum(
        serialize = "south korea",
        serialize = "south korean",
        serialize = "korea",
        serialize = "korean"
    )]
    SouthKorea = 3,
    #[strum(serialize = "hong kong")]
    HongKong = 4,
    #[strum(serialize = "taiwan", serialize = "taiwanese")]
    Taiwan = 5,
    #[strum(serialize = "thailand", serialize = "thai")]
    Thailand = 6,
    #[strum(serialize = "philippines", serialize = "filipino")]
    Philippines = 140,
    #[strum(serialize = "singapore", serialize = "singaporean")]
    Singapore = 157,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum Status {
    Ongoing = 1,
    Upcoming = 2,
    Completed = 3,
}

#[derive(
    Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display, EnumString, PartialEq, Eq,
)]
#[serde(rename_all = "snake_case")]
#[strum(ascii_case_insensitive)]
#[repr(i32)]
pub enum Format {
    // Drama Formats
    StandardSeries = 7,
    WebSeries = 8,
    VerticalSeries = 10,
    DramaSpecial = 14,
    // Movie Formats
    FeatureFilm = 24,
    ShortFilm = 26,
    IndependentFilm = 28,
    MadeForTVMovie = 30,
    Documentary = 32,
    AnimatedFilm = 34,
    Other = 36,
    // TV Show Formats
    TalkShow = 5,
    VarietyShow = 6,
    DocumentaryProgram = 16,
    MusicProgram = 18,
    RealityProgram = 20,
    OtherProgram = 22,
    AnimatedProgram = 38,
}

#[derive(
    Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display, EnumString, PartialEq, Eq,
)]
#[serde(rename_all = "snake_case")]
#[strum(ascii_case_insensitive)]
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
    #[strum(serialize = "martial arts")]
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
    #[strum(serialize = "sci-fi")]
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

#[derive(
    Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display, EnumString, PartialEq, Eq,
)]
#[serde(rename_all = "snake_case")]
#[strum(ascii_case_insensitive)]
#[repr(i32)]
pub enum Nationality {
    #[strum(serialize = "japan", serialize = "japanese")]
    Japanese = 1,
    #[strum(serialize = "china", serialize = "chinese")]
    Chinese = 2,
    #[strum(
        serialize = "south korea",
        serialize = "south korean",
        serialize = "korean"
    )]
    Korean = 3,
    #[strum(serialize = "hong kong", serialize = "hong konger")]
    HongKonger = 4,
    #[strum(serialize = "taiwan", serialize = "taiwanese")]
    Taiwanese = 5,
    #[strum(serialize = "thailand", serialize = "thai")]
    Thai = 6,
    #[strum(serialize = "philippines", serialize = "filipino")]
    Filipino = 140,
    #[strum(serialize = "singapore", serialize = "singaporean")]
    Singaporean = 157,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum Gender {
    Male = 77,
    Female = 70,
}

#[derive(
    Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display, EnumString, PartialEq, Eq,
)]
#[serde(rename_all = "snake_case")]
#[strum(ascii_case_insensitive)]
#[repr(i32)]
pub enum ArticleCategory {
    Interviews = 1,
    Editorials = 2,
    MDL = 3,
    News = 4,
    Recaps = 5,
    Announcement = 7,
    Articles = 9,
    #[strum(serialize = "k-pop")]
    KPop = 10,
    Celebrity = 12,
    #[strum(serialize = "k-drama")]
    KDrama = 14,
    Trailer = 16,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "lowercase")]
pub enum PeopleSort {
    Relevance,
    #[strum(to_string = "popular")]
    MostPopular,
    #[strum(to_string = "recently")]
    RecentlyAdded,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Copy, Clone, Display, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "lowercase")]
pub enum ArticleSort {
    Relevance,
    #[strum(to_string = "popular")]
    MostPopular,
    #[strum(to_string = "publish")]
    PublishDate,
}
