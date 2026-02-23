use crate::models::{PaginatedRecommendations, Recommendation};
use crate::services::parser::utils::{extract_img, parse_pagination};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::sync::LazyLock;

static READ_MORE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Read recommendations by (\d+) more users?").expect("Invalid Regex")
});

static BOX_BODY_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".box-body.b-t[id^='rec_']").unwrap());
static TITLE_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("b a").unwrap());
static RATING_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".score").unwrap());
static CONTENT_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".recs-body").unwrap());
static AUTHOR_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".recs-author a").unwrap());
static LIKE_CNT_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".like-cnt").unwrap());
static MORE_RECS_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".more-recs-container a").unwrap());

pub fn parse_title_recommendations(html: &str) -> PaginatedRecommendations {
    let doc = Html::parse_document(html);

    let items = doc
        .select(&*BOX_BODY_SEL)
        .filter_map(|el| {
            let title_el = el.select(&*TITLE_SEL).next()?;
            let title = title_el.text().collect::<String>().trim().to_owned();
            let id = title_el
                .value()
                .attr("href")?
                .rsplit('/')
                .next()?
                .to_string();

            let poster = extract_img(&el);

            let score = el
                .select(&*RATING_SEL)
                .next()
                .and_then(|e| e.text().collect::<String>().parse::<f32>().ok());

            let content = if let Some(body) = el.select(&*CONTENT_SEL).next() {
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
                content_parts.join(" ").trim().to_owned()
            } else {
                String::new()
            };

            let author_el = el.select(&*AUTHOR_SEL).next()?;
            let author = author_el.text().collect::<String>().trim().to_owned();
            let author_id = author_el
                .value()
                .attr("href")?
                .rsplit('/')
                .next()?
                .to_string();

            let likes = el
                .select(&*LIKE_CNT_SEL)
                .next()
                .and_then(|e| e.text().collect::<String>().parse::<i32>().ok())
                .unwrap_or(0);

            let read_more_count = el
                .select(&*MORE_RECS_SEL)
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
