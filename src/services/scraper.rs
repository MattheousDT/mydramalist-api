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

    /// Internal wrapper to handle the logging and caching lifecycle for any search type.
    async fn perform_search<T, F>(&self, label: &str, url: String, parser: F) -> Result<Vec<T>>
    where
        F: FnOnce(&str) -> Vec<T>,
        T: Clone + std::fmt::Debug,
    {
        info!(target: "scraper", "[{}] Initiating search: {}", label, url);

        // 1. Check Cache
        if let Some(cached_html) = self.html_cache.get(&url).await {
            debug!(target: "scraper", "[{}] Cache Hit", label);
            let results = parser(&cached_html);
            info!(target: "scraper", "[{}] Parsed {} items from cache", label, results.len());
            return Ok(results);
        }

        // 2. Fetch Fresh HTML
        debug!(target: "scraper", "[{}] Cache Miss. Fetching from MDL...", label);
        let response = self.client.get(&url).send().await?;
        let status = response.status();
        let html = response.text().await?;

        debug!(target: "scraper", "[{}] Received response (Status: {})", label, status);

        // 3. Parse and Cache
        let results = parser(&html);
        self.html_cache.insert(url, html.to_string()).await;

        info!(target: "scraper", "[{}] Parsed {} items from fresh HTML", label, results.len());
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
