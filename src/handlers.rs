use crate::{error::AppError, models::*, services::scraper::ScraperService};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/titles/{id}",
    tag = "Titles",
    summary = "Get Title Details",
    description = "Retrieves comprehensive details about a specific drama, movie, or TV show using its MyDramaList ID.",
    params(
        ("id" = String, Path, description = "MyDramaList Title ID", example = "735043-life")
    ),
    responses(
        (status = 200, description = "Title details successfully retrieved", body = TitleDetails),
        (status = 404, description = "Title not found")
    )
)]
pub async fn title_details_handler(
    State(s): State<Arc<ScraperService>>,
    Path(id): Path<String>,
) -> Result<Json<TitleDetails>, AppError> {
    let details = s.get_title_details(id.clone()).await?;
    match details {
        Some(d) => Ok(Json(d)),
        None => Err(AppError::NotFound(format!("Title '{}' not found", id))),
    }
}

#[utoipa::path(
    get,
    path = "/titles/search",
    tag = "Titles",
    summary = "Search Titles",
    description = "Search for dramas, movies, and TV shows using various advanced filters including genres, ratings, status, and release dates.",
    params(TitleSearchQuery),
    responses(
        (status = 200, description = "A list of matching titles", body = Vec<TitleSearchResult>)
    )
)]
pub async fn title_search_handler(
    State(s): State<Arc<ScraperService>>,
    Query(q): Query<TitleSearchQuery>,
) -> Result<Json<Vec<TitleSearchResult>>, AppError> {
    Ok(Json(s.search_titles(q).await?))
}

#[utoipa::path(
    get,
    path = "/people/search",
    tag = "People",
    summary = "Search People",
    description = "Search for actors, actresses, and production staff by name, nationality, and gender.",
    params(PeopleSearchQuery),
    responses(
        (status = 200, description = "A list of matching people", body = Vec<PeopleSearchResult>)
    )
)]
pub async fn people_search_handler(
    State(s): State<Arc<ScraperService>>,
    Query(q): Query<PeopleSearchQuery>,
) -> Result<Json<Vec<PeopleSearchResult>>, AppError> {
    Ok(Json(s.search_people(q).await?))
}

#[utoipa::path(
    get,
    path = "/articles/search",
    tag = "Articles",
    summary = "Search Articles",
    description = "Search for editorials, news, interviews, and recaps published on MyDramaList.",
    params(ArticleSearchQuery),
    responses(
        (status = 200, description = "A list of matching articles", body = Vec<ArticleSearchResult>)
    )
)]
pub async fn article_search_handler(
    State(s): State<Arc<ScraperService>>,
    Query(q): Query<ArticleSearchQuery>,
) -> Result<Json<Vec<ArticleSearchResult>>, AppError> {
    Ok(Json(s.search_articles(q).await?))
}
