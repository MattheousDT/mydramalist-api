use crate::models::TitlePhotos;
use crate::services::parser::utils::extract_img;
use scraper::{Html, Selector};

pub fn parse_title_photos(html: &str) -> TitlePhotos {
    let doc = Html::parse_document(html);

    let photos = doc
        .select(&Selector::parse(".box-body img").unwrap())
        .filter_map(|el| extract_img(&el))
        .collect();

    let pagination_sel = Selector::parse(".pagination .page-item").unwrap();
    let mut current_page = 1;
    let mut max_page = 1;

    for li in doc.select(&pagination_sel) {
        let is_active = li.value().classes().any(|c| c == "active");
        let text = li.text().collect::<String>().trim().to_string();

        if let Ok(num) = text.parse::<i32>() {
            if is_active {
                current_page = num;
            }
            if num > max_page {
                max_page = num;
            }
        }
    }

    TitlePhotos {
        photos,
        page: current_page,
        total_pages: max_page,
    }
}
