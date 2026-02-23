pub mod cast;
pub mod details;
pub mod episodes;
pub mod photos;
pub mod recommendations;
pub mod reviews;
pub mod search;
pub mod statistics;
pub mod utils;

pub struct MdlParser;

impl MdlParser {
    pub fn parse_title_search(html: &str) -> Vec<crate::models::TitleSearchResult> {
        search::parse_title_search(html)
    }

    pub fn parse_people_search(html: &str) -> Vec<crate::models::PeopleSearchResult> {
        search::parse_people_search(html)
    }

    pub fn parse_article_search(html: &str) -> Vec<crate::models::ArticleSearchResult> {
        search::parse_article_search(html)
    }

    pub fn parse_title_details(html: &str, title_id: &str) -> Option<crate::models::TitleDetails> {
        details::parse_title_details(html, title_id)
    }

    pub fn parse_title_episodes(html: &str) -> Vec<crate::models::Episode> {
        episodes::parse_title_episodes(html)
    }

    pub fn parse_title_cast(html: &str) -> crate::models::TitleCast {
        cast::parse_title_cast(html)
    }

    pub fn parse_title_photos(html: &str) -> crate::models::PaginatedPhotos {
        photos::parse_title_photos(html)
    }

    pub fn parse_title_reviews(html: &str) -> crate::models::PaginatedReviews {
        reviews::parse_title_reviews(html)
    }

    pub fn parse_title_recommendations(html: &str) -> crate::models::PaginatedRecommendations {
        recommendations::parse_title_recommendations(html)
    }

    pub fn parse_title_statistics(html: &str, id: &str) -> crate::models::TitleStatistics {
        statistics::parse_title_statistics(html, id)
    }
}
