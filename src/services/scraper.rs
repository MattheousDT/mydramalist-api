use crate::models::*;
use crate::services::mdl_url::MdlUrlBuilder;
use crate::services::parser::MdlParser;
use anyhow::Result;
use moka::future::Cache;
use reqwest::Client;
use std::collections::BTreeMap;
use std::time::Duration;
use tracing::{info, instrument};

#[derive(serde::Deserialize)]
struct RawAgeStat {
    age: String,
    percentage: f32,
    rating: f32,
}

#[derive(serde::Deserialize)]
struct AgeGroupResponse {
    data: Vec<RawAgeStat>,
}

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
        info!("Fetching title details: {}", url);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_title_details(&cached_html, &id));
        }

        let response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
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
        info!("Fetching title episodes: {}", url);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_title_episodes(&cached_html));
        }

        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let results = MdlParser::parse_title_episodes(&html);
        self.html_cache.insert(url, html.to_string()).await;
        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "title_cast"))]
    pub async fn get_title_cast(&self, id: String) -> Result<TitleCast> {
        let url = format!("https://mydramalist.com/{}/cast", id);
        info!("Fetching title cast: {}", url);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_title_cast(&cached_html));
        }

        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let results = MdlParser::parse_title_cast(&html);
        self.html_cache.insert(url, html.to_string()).await;
        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "title_photos"))]
    pub async fn get_title_photos(&self, id: String, page: i32) -> Result<PaginatedPhotos> {
        let url = format!("https://mydramalist.com/{}/photos?page={}", id, page);
        info!("Fetching title photos: {}", url);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_title_photos(&cached_html));
        }

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
    ) -> Result<PaginatedReviews> {
        let url = MdlUrlBuilder::title_reviews(&id, &query);
        info!("Fetching title reviews: {}", url);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_title_reviews(&cached_html));
        }

        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let results = MdlParser::parse_title_reviews(&html);
        self.html_cache.insert(url, html.to_string()).await;
        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "title_recommendations"))]
    pub async fn get_title_recommendations(
        &self,
        id: String,
        page: i32,
    ) -> Result<PaginatedRecommendations> {
        let url = format!("https://mydramalist.com/{}/recs?page={}", id, page);
        info!("Fetching title recommendations: {}", url);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_title_recommendations(&cached_html));
        }

        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let results = MdlParser::parse_title_recommendations(&html);
        self.html_cache.insert(url, html.to_string()).await;
        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "title_statistics"))]
    pub async fn get_title_statistics(&self, id: String) -> Result<TitleStatistics> {
        let url = format!("https://mydramalist.com/{}/statistics", id);
        info!("Fetching title statistics: {}", url);

        let html = if let Some(cached_html) = self.html_cache.get(&url).await {
            cached_html
        } else {
            let response = self.client.get(&url).send().await?;
            let text = response.text().await?;
            self.html_cache.insert(url, text.clone()).await;
            text
        };

        let mut stats = MdlParser::parse_title_statistics(&html, &id);

        if let Some(num_id) = id.split('-').next() {
            let age_url = format!("https://mydramalist.com/v1/titles/{}/agegroup", num_id);
            if let Ok(age_resp) = self.client.get(&age_url).send().await {
                if let Ok(age_text) = age_resp.text().await {
                    if let Ok(age_resp_data) = serde_json::from_str::<AgeGroupResponse>(&age_text) {
                        let mut age_map = BTreeMap::new();
                        for item in age_resp_data.data {
                            age_map.insert(
                                item.age,
                                AgeStat {
                                    percentage: item.percentage,
                                    rating: item.rating,
                                },
                            );
                        }
                        stats.age = age_map;
                    }
                }
            }
        }

        Ok(stats)
    }

    #[instrument(skip(self), fields(search_type = "titles"))]
    pub async fn search_titles(&self, query: TitleSearchQuery) -> Result<Vec<TitleSearchResult>> {
        let url = MdlUrlBuilder::search_titles(&query);
        info!("Searching titles: {}", url);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_title_search(&cached_html));
        }

        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let results = MdlParser::parse_title_search(&html);
        self.html_cache.insert(url, html.to_string()).await;
        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "people"))]
    pub async fn search_people(&self, query: PeopleSearchQuery) -> Result<Vec<PeopleSearchResult>> {
        let url = MdlUrlBuilder::search_people(&query);
        info!("Searching people: {}", url);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_people_search(&cached_html));
        }

        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let results = MdlParser::parse_people_search(&html);
        self.html_cache.insert(url, html.to_string()).await;
        Ok(results)
    }

    #[instrument(skip(self), fields(search_type = "articles"))]
    pub async fn search_articles(
        &self,
        query: ArticleSearchQuery,
    ) -> Result<Vec<ArticleSearchResult>> {
        let url = MdlUrlBuilder::search_articles(&query);
        info!("Searching articles: {}", url);

        if let Some(cached_html) = self.html_cache.get(&url).await {
            return Ok(MdlParser::parse_article_search(&cached_html));
        }

        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let results = MdlParser::parse_article_search(&html);
        self.html_cache.insert(url, html.to_string()).await;
        Ok(results)
    }
}
