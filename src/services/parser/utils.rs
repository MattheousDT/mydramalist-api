use crate::models::Image;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::sync::LazyLock;

static SMALL_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("small").unwrap());
static A_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("a").unwrap());
static IMG_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("img").unwrap());
static PAGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[?&]page=(\d+)").expect("Invalid Regex"));

pub fn extract_img(el: &ElementRef) -> Option<Image> {
    let img_node = if el.value().name() == "img" {
        Some(*el)
    } else {
        el.select(&*IMG_SEL).next()
    };

    img_node.and_then(|img| {
        img.value()
            .attr("data-src")
            .or(img.value().attr("src"))
            .map(|s| Image::from(s.to_string()))
    })
}

pub fn extract_character_info(el: &ElementRef) -> (Option<String>, Option<String>) {
    let mut character = None;
    let mut character_id = None;

    for small in el.select(&*SMALL_SEL) {
        if !small.value().classes().any(|c| c == "text-muted") {
            character = Some(small.text().collect::<String>().trim().to_owned());

            if let Some(a) = small.select(&*A_SEL).next() {
                if let Some(href) = a.value().attr("href") {
                    character_id = Some(href.trim_start_matches("/character/").to_string());
                }
            }
            break;
        }
    }

    (character, character_id)
}

pub fn parse_pagination(doc: &Html) -> (i32, i32) {
    static PAGINATION_SEL: LazyLock<Selector> =
        LazyLock::new(|| Selector::parse(".pagination .page-item").unwrap());

    let mut current_page = 1;
    let mut max_page = 1;

    for li in doc.select(&*PAGINATION_SEL) {
        let is_active = li.value().classes().any(|c| c == "active");

        // 1. Parse the page number from the text (used to find the active/current page)
        let text = li.text().collect::<String>().trim().to_owned();
        if let Ok(num) = text.parse::<i32>() {
            if is_active {
                current_page = num;
            }
            if num > max_page {
                max_page = num;
            }
        }

        // 2. Extract the page number from the `href` (captures the .last and .next links)
        if let Some(a) = li.select(&*A_SEL).next() {
            if let Some(href) = a.value().attr("href") {
                if let Some(caps) = PAGE_RE.captures(href) {
                    if let Some(m) = caps.get(1) {
                        if let Ok(num) = m.as_str().parse::<i32>() {
                            if num > max_page {
                                max_page = num;
                            }
                        }
                    }
                }
            }
        }
    }

    (current_page, max_page)
}
