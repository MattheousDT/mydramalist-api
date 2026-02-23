use crate::models::*;
use crate::services::mdl_url::MdlUrlBuilder;
use crate::services::parser::MdlParser;
use anyhow::Result;
use moka::future::Cache;
use reqwest::Client;
use std::time::Duration;
use tracing::instrument;

pub struct ScraperService {
    client: Client,
    html_cache: Cache<String, String>,
}

impl ScraperService {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
                .build()
                .unwrap(),
            html_cache: Cache::builder().time_to_live(Duration::from_secs(1800)).build(),
        }
    }

    #[instrument(skip(self), fields(search_type = "title_details"))]
    pub async fn get_title_details(&self, id: String) -> Result<Option<TitleDetails>> {
        let url = format!("https://mydramalist.com/{}", id);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_title_details(&cached_html, &id));
        }

        let response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            return Ok(None);
        }

        let html = response.text().await?;
        let results = MdlParser::parse_title_details(&html, &id);
        self.html_cache.insert(url, html).await;

        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "title_episodes"))]
    pub async fn get_title_episodes(&self, id: String) -> Result<Vec<Episode>> {
        let url = format!("https://mydramalist.com/{}/episodes", id);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_title_episodes(&cached_html));
        }

        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let results = MdlParser::parse_title_episodes(&html);
        self.html_cache.insert(url, html).await;
        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "title_cast"))]
    pub async fn get_title_cast(&self, id: String) -> Result<TitleCast> {
        let url = format!("https://mydramalist.com/{}/cast", id);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_title_cast(&cached_html));
        }

        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let results = MdlParser::parse_title_cast(&html);
        self.html_cache.insert(url, html).await;
        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "title_photos"))]
    pub async fn get_title_photos(&self, id: String, page: i32) -> Result<PaginatedPhotos> {
        let url = format!("https://mydramalist.com/{}/photos?page={}", id, page);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_title_photos(&cached_html));
        }

        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let results = MdlParser::parse_title_photos(&html);
        self.html_cache.insert(url, html).await;
        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "title_reviews"))]
    pub async fn get_title_reviews(
        &self,
        id: String,
        query: ReviewSearchQuery,
    ) -> Result<PaginatedReviews> {
        let url = MdlUrlBuilder::title_reviews(&id, &query);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_title_reviews(&cached_html));
        }

        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let results = MdlParser::parse_title_reviews(&html);
        self.html_cache.insert(url, html).await;
        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "title_recommendations"))]
    pub async fn get_title_recommendations(
        &self,
        id: String,
        page: i32,
    ) -> Result<PaginatedRecommendations> {
        let url = format!("https://mydramalist.com/{}/recs?page={}", id, page);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_title_recommendations(&cached_html));
        }

        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let results = MdlParser::parse_title_recommendations(&html);
        self.html_cache.insert(url, html).await;
        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "titles"))]
    pub async fn search_titles(&self, query: TitleSearchQuery) -> Result<Vec<TitleSearchResult>> {
        let url = MdlUrlBuilder::search_titles(&query);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_title_search(&cached_html));
        }

        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let results = MdlParser::parse_title_search(&html);
        self.html_cache.insert(url, html).await;
        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "people"))]
    pub async fn search_people(&self, query: PeopleSearchQuery) -> Result<Vec<PeopleSearchResult>> {
        let url = MdlUrlBuilder::search_people(&query);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_people_search(&cached_html));
        }

        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let results = MdlParser::parse_people_search(&html);
        self.html_cache.insert(url, html).await;
        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "articles"))]
    pub async fn search_articles(
        &self,
        query: ArticleSearchQuery,
    ) -> Result<Vec<ArticleSearchResult>> {
        let url = MdlUrlBuilder::search_articles(&query);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_article_search(&cached_html));
        }

        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let results = MdlParser::parse_article_search(&html);
        self.html_cache.insert(url, html).await;
        Ok(results)
    }
}
