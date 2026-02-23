use crate::models::PaginatedPhotos;
use crate::services::parser::utils::{extract_img, parse_pagination};
use scraper::{Html, Selector};

pub fn parse_title_photos(html: &str) -> PaginatedPhotos {
    let doc = Html::parse_document(html);

    let items = doc
        .select(&Selector::parse(".box-body img").unwrap())
        .filter_map(|el| extract_img(&el))
        .collect();

    let (page, total_pages) = parse_pagination(&doc);

    PaginatedPhotos {
        items,
        page,
        total_pages,
    }
}
