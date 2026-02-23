use crate::models::{PaginatedReviews, Review, ReviewAuthor, ReviewScores, ReviewStatus};
use crate::services::parser::utils::{extract_img, parse_pagination};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::str::FromStr;
use std::sync::LazyLock;

static HELPFUL_COUNT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\d+) people found this review helpful").expect("Invalid Regex"));
static EPISODES_SEEN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\d+) of \d+ episodes seen").expect("Invalid Regex"));
static NUMBER_ONLY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\d+").expect("Invalid Regex"));

static REVIEW_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".review").unwrap());
static AVATAR_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".avatar").unwrap());
static AUTHOR_LINK_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".box-body .text-primary").unwrap());
static DATETIME_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".datetime").unwrap());
static EPISODES_SEEN_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".episodes-seen").unwrap());
static USER_STATS_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".user-stats small").unwrap());
static REVIEW_BODY_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".review-body").unwrap());
static RATING_ROW_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".list-group.review-rating div").unwrap());
static OVERALL_SCORE_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".rating-overall .score").unwrap());
static REVIEW_TAG_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".review-tag").unwrap());
static COMMENTS_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".fa-comments-alt").unwrap());
static SPOILER_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".review-spoiler").unwrap());

pub fn parse_title_reviews(html: &str) -> PaginatedReviews {
    let doc = Html::parse_document(html);

    let items =
        doc.select(&*REVIEW_SEL)
            .filter_map(|el| {
                let id = el
                    .value()
                    .attr("id")
                    .map(|s| s.replace("review-", ""))
                    .unwrap_or_default();

                let author_link = el.select(&*AUTHOR_LINK_SEL).next()?;
                let author_name = author_link.text().collect::<String>().trim().to_owned();
                let author_id = author_link
                    .value()
                    .attr("href")?
                    .rsplit('/')
                    .next()?
                    .to_string();

                let avatar = el.select(&*AVATAR_SEL).next().and_then(|a| extract_img(&a));

                let author = ReviewAuthor {
                    name: author_name,
                    id: author_id,
                    avatar,
                };

                let date = el
                    .select(&*DATETIME_SEL)
                    .next()
                    .map(|e| e.text().collect::<String>().trim().to_owned());

                let status = el.select(&*REVIEW_TAG_SEL).next().and_then(|e| {
                    ReviewStatus::from_str(&e.text().collect::<String>().trim()).ok()
                });

                let episodes_seen = el.select(&*EPISODES_SEEN_SEL).next().and_then(|e| {
                    let text = e.text().collect::<String>();
                    EPISODES_SEEN_RE
                        .captures(&text)
                        .and_then(|c| c.get(1))
                        .and_then(|m| m.as_str().parse::<i32>().ok())
                });

                let helpful_count = el
                    .select(&*USER_STATS_SEL)
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
                    .select(&*COMMENTS_SEL)
                    .next()
                    .and_then(|icon| {
                        let parent = ElementRef::wrap(icon.parent()?)?;
                        let text = parent.text().collect::<String>();
                        NUMBER_ONLY_RE
                            .find(&text)
                            .and_then(|m| m.as_str().parse::<i32>().ok())
                    })
                    .unwrap_or(0);

                let spoilers = el.select(&*SPOILER_SEL).next().is_some();

                let overall = el.select(&*OVERALL_SCORE_SEL).next().and_then(|e| {
                    let s = e.text().collect::<String>();
                    s.trim().parse::<f32>().ok()
                });

                let mut story = None;
                let mut acting_cast = None;
                let mut music = None;
                let mut rewatch_value = None;

                for row in el.select(&*RATING_ROW_SEL) {
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

                let content = if let Some(body) = el.select(&*REVIEW_BODY_SEL).next() {
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
                    content: content.trim().to_owned(),
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
