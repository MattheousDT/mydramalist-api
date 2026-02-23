use crate::models::{PaginatedRecommendations, Recommendation};
use crate::services::parser::utils::{clean_text, extract_img, parse_pagination};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::sync::LazyLock;

static READ_MORE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Read recommendations by (\d+) more users?").expect("Invalid Regex")
});

pub fn parse_title_recommendations(html: &str) -> PaginatedRecommendations {
    let doc = Html::parse_document(html);

    let box_body_sel = Selector::parse(".box-body.b-t[id^='rec_']").unwrap();
    let title_sel = Selector::parse("b a").unwrap();
    let rating_sel = Selector::parse(".score").unwrap();
    let content_sel = Selector::parse(".recs-body").unwrap();
    let author_sel = Selector::parse(".recs-author a").unwrap();
    let like_cnt_sel = Selector::parse(".like-cnt").unwrap();
    let more_recs_sel = Selector::parse(".more-recs-container a").unwrap();

    let items = doc
        .select(&box_body_sel)
        .filter_map(|el| {
            let title_el = el.select(&title_sel).next()?;
            let title = clean_text(&title_el.text().collect::<String>());
            let id = title_el
                .value()
                .attr("href")?
                .rsplit('/')
                .next()?
                .to_string();

            let poster = extract_img(&el);

            let score = el
                .select(&rating_sel)
                .next()
                .and_then(|e| e.text().collect::<String>().parse::<f32>().ok());

            let content = if let Some(body) = el.select(&content_sel).next() {
                let mut content_parts = Vec::new();
                for child in body.children() {
                    if let Some(element) = ElementRef::wrap(child) {
                        if element.value().classes().any(|c| c == "recs-by") {
                            continue;
                        }
                        content_parts.push(element.html());
                    } else if let Some(text) = child.value().as_text() {
                        let t = text.trim();
                        if !t.is_empty() {
                            content_parts.push(t.to_string());
                        }
                    }
                }
                clean_text(&content_parts.join(" "))
            } else {
                String::new()
            };

            let author_el = el.select(&author_sel).next()?;
            let author = clean_text(&author_el.text().collect::<String>());
            let author_id = author_el
                .value()
                .attr("href")?
                .rsplit('/')
                .next()?
                .to_string();

            let likes = el
                .select(&like_cnt_sel)
                .next()
                .and_then(|e| e.text().collect::<String>().parse::<i32>().ok())
                .unwrap_or(0);

            let read_more_count = el
                .select(&more_recs_sel)
                .next()
                .and_then(|e| {
                    let text = e.text().collect::<String>();
                    READ_MORE_RE
                        .captures(&text)
                        .and_then(|c| c.get(1))
                        .and_then(|m| m.as_str().parse::<i32>().ok())
                })
                .unwrap_or(0);

            Some(Recommendation {
                id,
                title,
                poster,
                score,
                content,
                author,
                author_id,
                likes,
                read_more_count,
            })
        })
        .collect();

    let (page, total_pages) = parse_pagination(&doc);

    PaginatedRecommendations {
        items,
        page,
        total_pages,
    }
}
