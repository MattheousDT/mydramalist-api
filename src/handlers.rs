use crate::{error::AppError, extractors::Query, models::*, services::scraper::ScraperService};
use axum::{
    Json,
    extract::{Path, State},
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
    match s.get_title_details(id.clone()).await? {
        Some(d) => Ok(Json(d)),
        None => Err(AppError::NotFound(format!("Title '{}' not found", id))),
    }
}

#[utoipa::path(
    get,
    path = "/titles/{id}/statistics",
    tag = "Titles",
    summary = "Get Title Statistics",
    description = "Retrieves statistical data for a title, including rating trends, user activity, and status distribution.",
    params(
        ("id" = String, Path, description = "MyDramaList Title ID", example = "49231-move-to-heaven")
    ),
    responses(
        (status = 200, description = "Statistics successfully retrieved", body = TitleStatistics)
    )
)]
pub async fn title_statistics_handler(
    State(s): State<Arc<ScraperService>>,
    Path(id): Path<String>,
) -> Result<Json<TitleStatistics>, AppError> {
    match s.get_title_statistics(id.clone()).await? {
        Some(stats) => Ok(Json(stats)),
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
    match s.get_title_episodes(id.clone()).await? {
        Some(eps) => Ok(Json(eps)),
        None => Err(AppError::NotFound(format!("Title '{}' not found", id))),
    }
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
    match s.get_title_cast(id.clone()).await? {
        Some(cast) => Ok(Json(cast)),
        None => Err(AppError::NotFound(format!("Title '{}' not found", id))),
    }
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
        (status = 200, description = "Photos successfully retrieved", body = PaginatedPhotos)
    )
)]
pub async fn title_photos_handler(
    State(s): State<Arc<ScraperService>>,
    Path(id): Path<String>,
    Query(pq): Query<PaginationQuery>,
) -> Result<Json<PaginatedPhotos>, AppError> {
    let page = pq.page.unwrap_or(1);
    match s.get_title_photos(id.clone(), page).await? {
        Some(photos) => Ok(Json(photos)),
        None => Err(AppError::NotFound(format!("Title '{}' not found", id))),
    }
}

#[utoipa::path(
    get,
    path = "/titles/{id}/reviews",
    tag = "Titles",
    summary = "Get Title Reviews",
    description = "Retrieves user reviews for a specific title using its MyDramaList ID. Supports advanced filtering and sorting.",
    params(
        ("id" = String, Path, description = "MyDramaList Title ID", example = "49231-move-to-heaven"),
        ReviewSearchQuery
    ),
    responses(
        (status = 200, description = "Reviews successfully retrieved", body = PaginatedReviews)
    )
)]
pub async fn title_reviews_handler(
    State(s): State<Arc<ScraperService>>,
    Path(id): Path<String>,
    Query(q): Query<ReviewSearchQuery>,
) -> Result<Json<PaginatedReviews>, AppError> {
    match s.get_title_reviews(id.clone(), q).await? {
        Some(reviews) => Ok(Json(reviews)),
        None => Err(AppError::NotFound(format!("Title '{}' not found", id))),
    }
}

#[utoipa::path(
    get,
    path = "/titles/{id}/recommendations",
    tag = "Titles",
    summary = "Get Title Recommendations",
    description = "Retrieves recommendations for a specific title using its MyDramaList ID. Supports pagination.",
    params(
        ("id" = String, Path, description = "MyDramaList Title ID", example = "49231-move-to-heaven"),
        PaginationQuery
    ),
    responses(
        (status = 200, description = "Recommendations successfully retrieved", body = PaginatedRecommendations)
    )
)]
pub async fn title_recommendations_handler(
    State(s): State<Arc<ScraperService>>,
    Path(id): Path<String>,
    Query(pq): Query<PaginationQuery>,
) -> Result<Json<PaginatedRecommendations>, AppError> {
    let page = pq.page.unwrap_or(1);
    match s.get_title_recommendations(id.clone(), page).await? {
        Some(recs) => Ok(Json(recs)),
        None => Err(AppError::NotFound(format!("Title '{}' not found", id))),
    }
}

#[utoipa::path(
    get,
    path = "/people/{id}",
    tag = "People",
    summary = "Get Person Details",
    description = "Retrieves comprehensive details about a specific actor, actress, or crew member using their MyDramaList ID.",
    params(
        ("id" = String, Path, description = "MyDramaList Person ID", example = "1897-park-eun-bin")
    ),
    responses(
        (status = 200, description = "Person details successfully retrieved", body = PersonDetails),
        (status = 404, description = "Person not found")
    )
)]
pub async fn person_details_handler(
    State(s): State<Arc<ScraperService>>,
    Path(id): Path<String>,
) -> Result<Json<PersonDetails>, AppError> {
    match s.get_person_details(id.clone()).await? {
        Some(d) => Ok(Json(d)),
        None => Err(AppError::NotFound(format!("Person '{}' not found", id))),
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
        (status = 200, description = "A list of matching titles", body = PaginatedTitleResults)
    )
)]
pub async fn title_search_handler(
    State(s): State<Arc<ScraperService>>,
    Query(q): Query<TitleSearchQuery>,
) -> Result<Json<PaginatedTitleResults>, AppError> {
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
        (status = 200, description = "A list of matching people", body = PaginatedPeopleResults)
    )
)]
pub async fn people_search_handler(
    State(s): State<Arc<ScraperService>>,
    Query(q): Query<PeopleSearchQuery>,
) -> Result<Json<PaginatedPeopleResults>, AppError> {
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
        (status = 200, description = "A list of matching articles", body = PaginatedArticleResults)
    )
)]
pub async fn article_search_handler(
    State(s): State<Arc<ScraperService>>,
    Query(q): Query<ArticleSearchQuery>,
) -> Result<Json<PaginatedArticleResults>, AppError> {
    Ok(Json(s.search_articles(q).await?))
}
