use crate::models::*;
use crate::services::mdl_url::MdlUrlBuilder;
use crate::services::parser::MdlParser;
use anyhow::Result;
use moka::future::Cache;
use reqwest::Client;
use std::time::Duration;
use tracing::{debug, info, instrument};

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

    async fn perform_search<T, F>(&self, label: &str, url: String, parser: F) -> Result<Vec<T>>
    where
        F: FnOnce(&str) -> Vec<T>,
        T: Clone + std::fmt::Debug,
    {
        info!(target: "scraper", "[{}] Initiating search: {}", label, url);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            debug!(target: "scraper", "[{}] Cache Hit", label);
            let results = parser(&cached_html);
            info!(target: "scraper", "[{}] Parsed {} items from cache", label, results.len());
            return Ok(results);
        }

        debug!(target: "scraper", "[{}] Cache Miss. Fetching from MDL...", label);
        let response = self.client.get(&url).send().await?;
        let status = response.status();
        let html = response.text().await?;

        debug!(target: "scraper", "[{}] Received response (Status: {})", label, status);

        let results = parser(&html);
        self.html_cache.insert(url, html.to_string()).await;

        info!(target: "scraper", "[{}] Parsed {} items from fresh HTML", label, results.len());
        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "title_details"))]
    pub async fn get_title_details(&self, id: String) -> Result<Option<TitleDetails>> {
        let url = format!("https://mydramalist.com/{}", id);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_title_details(&cached_html, &id));
        }

        let response = self.client.get(&url).send().await?;
        let status = response.status();

        if !status.is_success() {
            return Ok(None);
        }

        let html = response.text().await?;
        let results = MdlParser::parse_title_details(&html, &id);
        self.html_cache.insert(url, html.to_string()).await;

        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "title_episodes"))]
    pub async fn get_title_episodes(&self, id: String) -> Result<Vec<Episode>> {
        let url = format!("https://mydramalist.com/{}/episodes", id);
        self.perform_search("Episodes", url, MdlParser::parse_title_episodes)
            .await
    }

    #[instrument(skip(self), fields(search_type = "title_cast"))]
    pub async fn get_title_cast(&self, id: String) -> Result<TitleCast> {
        let url = format!("https://mydramalist.com/{}/cast", id);

        let label = "Cast";
        info!(target: "scraper", "[{}] Initiating search: {}", label, url);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            debug!(target: "scraper", "[{}] Cache Hit", label);
            return Ok(MdlParser::parse_title_cast(&cached_html));
        }

        debug!(target: "scraper", "[{}] Cache Miss. Fetching from MDL...", label);
        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;

        let results = MdlParser::parse_title_cast(&html);
        self.html_cache.insert(url, html.to_string()).await;

        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "title_photos"))]
    pub async fn get_title_photos(&self, id: String, page: i32) -> Result<TitlePhotos> {
        let url = format!("https://mydramalist.com/{}/photos?page={}", id, page);

        let label = "Photos";
        info!(target: "scraper", "[{}] Initiating search: {}", label, url);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            debug!(target: "scraper", "[{}] Cache Hit", label);
            return Ok(MdlParser::parse_title_photos(&cached_html));
        }

        debug!(target: "scraper", "[{}] Cache Miss. Fetching from MDL...", label);
        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;

        let results = MdlParser::parse_title_photos(&html);
        self.html_cache.insert(url, html.to_string()).await;

        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "title_reviews"))]
    pub async fn get_title_reviews(
        &self,
        id: String,
        query: ReviewSearchQuery,
    ) -> Result<TitleReviews> {
        let url = MdlUrlBuilder::title_reviews(&id, &query);

        let label = "Reviews";
        info!(target: "scraper", "[{}] Initiating search: {}", label, url);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            debug!(target: "scraper", "[{}] Cache Hit", label);
            return Ok(MdlParser::parse_title_reviews(&cached_html));
        }

        debug!(target: "scraper", "[{}] Cache Miss. Fetching from MDL...", label);
        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;

        let results = MdlParser::parse_title_reviews(&html);
        self.html_cache.insert(url, html.to_string()).await;

        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "titles"))]
    pub async fn search_titles(&self, query: TitleSearchQuery) -> Result<Vec<TitleSearchResult>> {
        let url = MdlUrlBuilder::search_titles(&query);
        self.perform_search("Titles", url, MdlParser::parse_title_search)
            .await
    }

    #[instrument(skip(self), fields(search_type = "people"))]
    pub async fn search_people(&self, query: PeopleSearchQuery) -> Result<Vec<PeopleSearchResult>> {
        let url = MdlUrlBuilder::search_people(&query);
        self.perform_search("People", url, MdlParser::parse_people_search)
            .await
    }

    #[instrument(skip(self), fields(search_type = "articles"))]
    pub async fn search_articles(
        &self,
        query: ArticleSearchQuery,
    ) -> Result<Vec<ArticleSearchResult>> {
        let url = MdlUrlBuilder::search_articles(&query);
        self.perform_search("Articles", url, MdlParser::parse_article_search)
            .await
    }
}
