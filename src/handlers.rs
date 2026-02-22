use crate::{error::AppError, models::*, services::scraper::ScraperService};
use axum::{
    Json,
    extract::{Query, State},
};
use std::sync::Arc;

#[utoipa::path(get, path = "/titles/search", tag = "Titles", params(TitleSearchQuery), responses((status = 200, body = Vec<TitleSearchResult>)))]
pub async fn title_search_handler(
    State(s): State<Arc<ScraperService>>,
    Query(q): Query<TitleSearchQuery>,
) -> Result<Json<Vec<TitleSearchResult>>, AppError> {
    Ok(Json(s.search_titles(q).await?))
}

#[utoipa::path(get, path = "/people/search", tag = "People", params(PeopleSearchQuery), responses((status = 200, body = Vec<PeopleSearchResult>)))]
pub async fn people_search_handler(
    State(s): State<Arc<ScraperService>>,
    Query(q): Query<PeopleSearchQuery>,
) -> Result<Json<Vec<PeopleSearchResult>>, AppError> {
    Ok(Json(s.search_people(q).await?))
}

#[utoipa::path(get, path = "/articles/search", tag = "Articles", params(ArticleSearchQuery), responses((status = 200, body = Vec<ArticleSearchResult>)))]
pub async fn article_search_handler(
    State(s): State<Arc<ScraperService>>,
    Query(q): Query<ArticleSearchQuery>,
) -> Result<Json<Vec<ArticleSearchResult>>, AppError> {
    Ok(Json(s.search_articles(q).await?))
}
