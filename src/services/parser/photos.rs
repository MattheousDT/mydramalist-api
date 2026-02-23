use crate::models::PaginatedPhotos;
use crate::services::parser::utils::{extract_img, parse_pagination};
use scraper::{Html, Selector};
use std::sync::LazyLock;

static IMG_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".box-body img").unwrap());

pub fn parse_title_photos(html: &str) -> PaginatedPhotos {
    let doc = Html::parse_document(html);

    let items = doc
        .select(&*IMG_SEL)
        .filter_map(|el| extract_img(&el))
        .collect();

    let (page, total_pages) = parse_pagination(&doc);

    PaginatedPhotos {
        items,
        page,
        total_pages,
    }
}
