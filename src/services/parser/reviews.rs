use crate::models::{PaginatedReviews, Review, ReviewAuthor, ReviewScores, ReviewStatus};
use crate::services::parser::utils::{clean_text, extract_img, from_scraped_str, parse_pagination};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::sync::LazyLock;

static HELPFUL_COUNT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\d+) people found this review helpful").expect("Invalid Regex"));
static EPISODES_SEEN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\d+) of \d+ episodes seen").expect("Invalid Regex"));
static NUMBER_ONLY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\d+").expect("Invalid Regex"));

pub fn parse_title_reviews(html: &str) -> PaginatedReviews {
    let doc = Html::parse_document(html);

    let review_sel = Selector::parse(".review").unwrap();
    let avatar_sel = Selector::parse(".avatar").unwrap();
    let author_link_sel = Selector::parse(".box-body .text-primary").unwrap();
    let datetime_sel = Selector::parse(".datetime").unwrap();
    let episodes_seen_sel = Selector::parse(".episodes-seen").unwrap();
    let user_stats_sel = Selector::parse(".user-stats small").unwrap();
    let review_body_sel = Selector::parse(".review-body").unwrap();
    let rating_row_sel = Selector::parse(".list-group.review-rating div").unwrap();
    let overall_score_sel = Selector::parse(".rating-overall .score").unwrap();
    let review_tag_sel = Selector::parse(".review-tag").unwrap();
    let comments_sel = Selector::parse(".fa-comments-alt").unwrap();
    let spoiler_sel = Selector::parse(".review-spoiler").unwrap();

    let items = doc
        .select(&review_sel)
        .filter_map(|el| {
            let id = el
                .value()
                .attr("id")
                .map(|s| s.replace("review-", ""))
                .unwrap_or_default();

            let author_link = el.select(&author_link_sel).next()?;
            let author_name = clean_text(&author_link.text().collect::<String>());
            let author_id = author_link
                .value()
                .attr("href")?
                .rsplit('/')
                .next()?
                .to_string();

            let avatar = el.select(&avatar_sel).next().and_then(|a| extract_img(&a));

            let author = ReviewAuthor {
                name: author_name,
                id: author_id,
                avatar,
            };

            let date = el
                .select(&datetime_sel)
                .next()
                .map(|e| clean_text(&e.text().collect::<String>()));

            let status = el
                .select(&review_tag_sel)
                .next()
                .and_then(|e| from_scraped_str::<ReviewStatus>(&e.text().collect::<String>()));

            let episodes_seen = el.select(&episodes_seen_sel).next().and_then(|e| {
                let text = e.text().collect::<String>();
                EPISODES_SEEN_RE
                    .captures(&text)
                    .and_then(|c| c.get(1))
                    .and_then(|m| m.as_str().parse::<i32>().ok())
            });

            let helpful_count = el
                .select(&user_stats_sel)
                .next()
                .map(|e| e.text().collect::<String>())
                .and_then(|text| {
                    HELPFUL_COUNT_RE
                        .captures(&text)
                        .and_then(|c| c.get(1))
                        .and_then(|m| m.as_str().parse::<i32>().ok())
                })
                .unwrap_or(0);

            let comments_count = el
                .select(&comments_sel)
                .next()
                .and_then(|icon| {
                    let parent = ElementRef::wrap(icon.parent()?)?;
                    let text = parent.text().collect::<String>();
                    NUMBER_ONLY_RE
                        .find(&text)
                        .and_then(|m| m.as_str().parse::<i32>().ok())
                })
                .unwrap_or(0);

            let spoilers = el.select(&spoiler_sel).next().is_some();

            let overall = el.select(&overall_score_sel).next().and_then(|e| {
                let s = e.text().collect::<String>();
                s.trim().parse::<f32>().ok()
            });

            let mut story = None;
            let mut acting_cast = None;
            let mut music = None;
            let mut rewatch_value = None;

            for row in el.select(&rating_row_sel) {
                let text = row.text().collect::<String>();
                let score = text
                    .split_whitespace()
                    .next_back()
                    .and_then(|s: &str| s.parse::<f32>().ok());

                if text.starts_with("Story") {
                    story = score;
                } else if text.starts_with("Acting/Cast") {
                    acting_cast = score;
                } else if text.starts_with("Music") {
                    music = score;
                } else if text.starts_with("Rewatch Value") {
                    rewatch_value = score;
                }
            }

            let scores = ReviewScores {
                overall,
                story,
                acting_cast,
                music,
                rewatch_value,
            };

            let content = if let Some(body) = el.select(&review_body_sel).next() {
                let mut content_parts = Vec::new();
                for child in body.children() {
                    if let Some(element) = ElementRef::wrap(child) {
                        let class = element.value().classes().collect::<Vec<_>>();
                        if class.contains(&"box")
                            || class.contains(&"read-more")
                            || class.contains(&"review-helpful")
                            || class.contains(&"review-spoiler")
                        {
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
                content_parts.join(" ")
            } else {
                String::new()
            };

            Some(Review {
                id,
                author,
                date,
                status,
                episodes_seen,
                scores,
                helpful_count,
                comments_count,
                spoilers,
                content: clean_text(&content),
            })
        })
        .collect();

    let (page, total_pages) = parse_pagination(&doc);

    PaginatedReviews {
        items,
        page,
        total_pages,
    }
}
