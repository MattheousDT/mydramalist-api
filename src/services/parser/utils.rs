use crate::models::Image;
use scraper::{ElementRef, Html, Selector};

pub fn extract_img(el: &ElementRef) -> Option<Image> {
    let img_node = if el.value().name() == "img" {
        Some(*el)
    } else {
        el.select(&Selector::parse("img").unwrap()).next()
    };

    img_node.and_then(|img| {
        img.value()
            .attr("data-src")
            .or(img.value().attr("src"))
            .map(|s| Image::from(s.to_string()))
    })
}

pub fn clean_text(s: &str) -> String {
    s.trim().to_string()
}

pub fn from_scraped_str<T>(s: &str) -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    let cleaned = s.trim();
    let json_compat = format!("\"{}\"", cleaned);

    if let Ok(val) = serde_json::from_str::<T>(&json_compat) {
        return Some(val);
    }

    let snake_case = cleaned.to_lowercase().replace([' ', '-'], "_");
    let json_snake_compat = format!("\"{}\"", snake_case);

    if let Ok(val) = serde_json::from_str::<T>(&json_snake_compat) {
        return Some(val);
    }

    tracing::warn!("Failed to map scraped string to Enum: [{}]", cleaned);
    None
}

pub fn parse_pagination(doc: &Html) -> (i32, i32) {
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

    (current_page, max_page)
}
