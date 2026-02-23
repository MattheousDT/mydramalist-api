use crate::{handlers::*, models::*};
use axum::{Json, response::Html};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        article_search_handler,
        people_search_handler,
        title_cast_handler,
        title_details_handler,
        title_episodes_handler,
        title_photos_handler,
        title_recommendations_handler,
        title_reviews_handler,
        title_search_handler,
    ),
    components(schemas(
        ArticleCategory,
        ArticleSearchQuery,
        ArticleSearchResult,
        ArticleSort,
        CastMember,
        Country,
        CrewMember,
        DramaFormat,
        Episode,
        Format,
        Gender,
        Genre,
        Image,
        MovieFormat,
        Nationality,
        PaginatedPhotos,
        PaginatedRecommendations,
        PaginatedReviews,
        PaginationQuery,
        PeopleSearchQuery,
        PeopleSearchResult,
        PeopleSort,
        RecommendationPreview,
        RecommendationPreview,
        Review,
        ReviewAuthor,
        ReviewScores,
        ReviewSearchQuery,
        ReviewSort,
        ReviewStatus,
        Statistics,
        Status,
        Tag,
        TitleCast,
        TitleDetails,
        TitleSearchQuery,
        TitleSearchResult,
        TitleSort,
        TVShowFormat,
        Type,
        WhereToWatch,
    )),
    tags(
        (name = "Articles", description = "Endpoints for retrieving Editorials, News, Interviews, and Recaps published on MyDramaList."),
        (name = "People", description = "Endpoints for searching Actors, Actresses, and Crew Members."),
        (name = "Titles", description = "Endpoints for finding and retrieving details about Dramas, Movies, and TV Shows."),
    ),
    info(title = "MDL API", version = "1.2.0", description = "Unofficial API for MyDramaList")
)]
pub struct ApiDoc;

pub async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

pub async fn scalar_docs() -> Html<&'static str> {
    Html(
        r#"<!doctype html>
        <html>
          <head>
            <title>MDL API Docs</title>
            <meta charset="utf-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />
          </head>
          <body>
            <script
                id="api-reference"
                data-url="/api-docs/openapi.json"
                data-configuration='{"theme": "kepler", "layout": "modern", "defaultOpenAllTags": true, "showSidebar": true}'
                src="https://cdn.jsdelivr.net/npm/@scalar/api-reference">
            </script>
          </body>
        </html>"#,
    )
}
