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

    async fn fetch_and_parse<T, F>(&self, url: &str, parser: F) -> Result<Option<T>>
    where
        F: FnOnce(&str) -> T,
        T: Clone + Send + Sync + 'static,
    {
        if let Some(cached_html) = self.html_cache.get(url).await {
            return Ok(Some(parser(&cached_html)));
        }

        let response = self.client.get(url).send().await?;
        if !response.status().is_success() {
            return Ok(None);
        }

        let html = response.text().await?;
        let result = parser(&html);
        self.html_cache
            .insert(url.to_string(), html.to_string())
            .await;

        Ok(Some(result))
    }

    #[instrument(skip(self), fields(search_type = "title_details"))]
    pub async fn get_title_details(&self, id: String) -> Result<Option<TitleDetails>> {
        let url = format!("https://mydramalist.com/{}", id);
        info!("Fetching title details: {}", url);
        self.fetch_and_parse(&url, |html| MdlParser::parse_title_details(html, &id))
            .await
            .map(|opt| opt.flatten())
    }

    #[instrument(skip(self), fields(search_type = "person_details"))]
    pub async fn get_person_details(&self, id: String) -> Result<Option<PersonDetails>> {
        let url = format!("https://mydramalist.com/people/{}", id);
        info!("Fetching person details: {}", url);
        self.fetch_and_parse(&url, |html| MdlParser::parse_person_details(html, &id))
            .await
            .map(|opt| opt.flatten())
    }

    #[instrument(skip(self), fields(search_type = "title_episodes"))]
    pub async fn get_title_episodes(&self, id: String) -> Result<Option<Vec<Episode>>> {
        let url = format!("https://mydramalist.com/{}/episodes", id);
        info!("Fetching title episodes: {}", url);
        self.fetch_and_parse(&url, |html| MdlParser::parse_title_episodes(html))
            .await
    }

    #[instrument(skip(self), fields(search_type = "title_cast"))]
    pub async fn get_title_cast(&self, id: String) -> Result<Option<TitleCast>> {
        let url = format!("https://mydramalist.com/{}/cast", id);
        info!("Fetching title cast: {}", url);
        self.fetch_and_parse(&url, |html| MdlParser::parse_title_cast(html))
            .await
    }

    #[instrument(skip(self), fields(search_type = "title_photos"))]
    pub async fn get_title_photos(&self, id: String, page: i32) -> Result<Option<PaginatedPhotos>> {
        let url = format!("https://mydramalist.com/{}/photos?page={}", id, page);
        info!("Fetching title photos: {}", url);
        self.fetch_and_parse(&url, |html| MdlParser::parse_title_photos(html))
            .await
    }

    #[instrument(skip(self), fields(search_type = "title_reviews"))]
    pub async fn get_title_reviews(
        &self,
        id: String,
        query: ReviewSearchQuery,
    ) -> Result<Option<PaginatedReviews>> {
        let url = MdlUrlBuilder::title_reviews(&id, &query);
        info!("Fetching title reviews: {}", url);
        self.fetch_and_parse(&url, |html| MdlParser::parse_title_reviews(html))
            .await
    }

    #[instrument(skip(self), fields(search_type = "title_recommendations"))]
    pub async fn get_title_recommendations(
        &self,
        id: String,
        page: i32,
    ) -> Result<Option<PaginatedRecommendations>> {
        let url = format!("https://mydramalist.com/{}/recs?page={}", id, page);
        info!("Fetching title recommendations: {}", url);
        self.fetch_and_parse(&url, |html| MdlParser::parse_title_recommendations(html))
            .await
    }

    #[instrument(skip(self), fields(search_type = "title_statistics"))]
    pub async fn get_title_statistics(&self, id: String) -> Result<Option<TitleStatistics>> {
        let url = format!("https://mydramalist.com/{}/statistics", id);
        info!("Fetching title statistics: {}", url);

        let mut stats = if let Some(cached_html) = self.html_cache.get(&url).await {
            MdlParser::parse_title_statistics(&cached_html, &id)
        } else {
            let response = self.client.get(&url).send().await?;
            if !response.status().is_success() {
                return Ok(None);
            }
            let text = response.text().await?;
            self.html_cache.insert(url.clone(), text.clone()).await;
            MdlParser::parse_title_statistics(&text, &id)
        };

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

        Ok(Some(stats))
    }

    #[instrument(skip(self), fields(search_type = "titles"))]
    pub async fn search_titles(&self, query: TitleSearchQuery) -> Result<PaginatedTitleResults> {
        let url = MdlUrlBuilder::search_titles(&query);
        info!("Searching titles: {}", url);

        let res = self
            .fetch_and_parse(&url, |html| MdlParser::parse_title_search(html))
            .await?;

        Ok(res.unwrap_or_else(|| PaginatedTitleResults {
            items: vec![],
            page: query.page.unwrap_or(1),
            total_pages: 1,
        }))
    }

    #[instrument(skip(self), fields(search_type = "people"))]
    pub async fn search_people(&self, query: PeopleSearchQuery) -> Result<PaginatedPeopleResults> {
        let url = MdlUrlBuilder::search_people(&query);
        info!("Searching people: {}", url);

        let res = self
            .fetch_and_parse(&url, |html| MdlParser::parse_people_search(html))
            .await?;

        Ok(res.unwrap_or_else(|| PaginatedPeopleResults {
            items: vec![],
            page: query.page.unwrap_or(1),
            total_pages: 1,
        }))
    }

    #[instrument(skip(self), fields(search_type = "articles"))]
    pub async fn search_articles(
        &self,
        query: ArticleSearchQuery,
    ) -> Result<PaginatedArticleResults> {
        let url = MdlUrlBuilder::search_articles(&query);
        info!("Searching articles: {}", url);

        let res = self
            .fetch_and_parse(&url, |html| MdlParser::parse_article_search(html))
            .await?;

        Ok(res.unwrap_or_else(|| PaginatedArticleResults {
            items: vec![],
            page: query.page.unwrap_or(1),
            total_pages: 1,
        }))
    }
}
