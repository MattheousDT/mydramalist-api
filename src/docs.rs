use crate::{handlers::*, models::*};
use axum::{Json, response::Html};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(title_search_handler, people_search_handler, article_search_handler, title_details_handler),
    components(schemas(
        ArticleCategory,
        ArticleSearchQuery,
        ArticleSearchResult,
        ArticleSort,
        Country,
        DramaFormat,
        Format,
        Gender,
        Genre,
        Image,
        MovieFormat,
        Nationality,
        PeopleSearchQuery,
        PeopleSearchResult,
        PeopleSort,
        Status,
        TVShowFormat,
        TitleSearchQuery,
        TitleSearchResult,
        TitleSort,
        Type,
        TitleDetails,
        Statistics,
        CrewMember,
        Tag,
        WhereToWatch,
        CastMember,
        Recommendation,
    )),
    tags(
        (name = "Titles", description = "Endpoints for finding and retrieving details about Dramas, Movies, and TV Shows."),
        (name = "People", description = "Endpoints for searching Actors, Actresses, and Crew Members."),
        (name = "Articles", description = "Endpoints for retrieving Editorials, News, Interviews, and Recaps published on MyDramaList.")
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
