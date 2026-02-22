pub mod details;
pub mod search;
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
}
