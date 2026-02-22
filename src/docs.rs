use crate::{handlers::*, models::*};
use axum::{Json, response::Html};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(title_search_handler, people_search_handler, article_search_handler,),
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
    )),
    info(title = "MDL API", version = "1.2.0")
)]
pub struct ApiDoc;

pub async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

pub async fn scalar_docs() -> Html<&'static str> {
    Html(
        r#"<!doctype html><html><head><title>API Docs</title></head><body>
    <script id="api-reference" data-url="/api-docs/openapi.json" src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
    </body></html>"#,
    )
}
