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
    path = "/titles/{id}/episodes",
    tag = "Titles",
    summary = "Get Title Episodes",
    description = "Retrieves a list of episodes for a specific Drama or TV Show using its MyDramaList ID.",
    params(
        ("id" = String, Path, description = "MyDramaList Title ID", example = "49231-move-to-heaven")
    ),
    responses(
        (status = 200, description = "List of episodes successfully retrieved", body = Vec<Episode>)
    )
)]
pub async fn title_episodes_handler(
    State(s): State<Arc<ScraperService>>,
    Path(id): Path<String>,
) -> Result<Json<Vec<Episode>>, AppError> {
    Ok(Json(s.get_title_episodes(id).await?))
}

#[utoipa::path(
    get,
    path = "/titles/{id}/cast",
    tag = "Titles",
    summary = "Get Title Cast & Crew",
    description = "Retrieves the full cast and crew credits for a specific title using its MyDramaList ID.",
    params(
        ("id" = String, Path, description = "MyDramaList Title ID", example = "49231-move-to-heaven")
    ),
    responses(
        (status = 200, description = "Cast and crew credits successfully retrieved", body = TitleCast)
    )
)]
pub async fn title_cast_handler(
    State(s): State<Arc<ScraperService>>,
    Path(id): Path<String>,
) -> Result<Json<TitleCast>, AppError> {
    Ok(Json(s.get_title_cast(id).await?))
}

#[utoipa::path(
    get,
    path = "/titles/{id}/photos",
    tag = "Titles",
    summary = "Get Title Photos",
    description = "Retrieves photos for a specific title using its MyDramaList ID. Supports pagination.",
    params(
        ("id" = String, Path, description = "MyDramaList Title ID", example = "49231-move-to-heaven"),
        PaginationQuery
    ),
    responses(
        (status = 200, description = "Photos successfully retrieved", body = TitlePhotos)
    )
)]
pub async fn title_photos_handler(
    State(s): State<Arc<ScraperService>>,
    Path(id): Path<String>,
    Query(pq): Query<PaginationQuery>,
) -> Result<Json<TitlePhotos>, AppError> {
    let page = pq.page.unwrap_or(1);
    Ok(Json(s.get_title_photos(id, page).await?))
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
