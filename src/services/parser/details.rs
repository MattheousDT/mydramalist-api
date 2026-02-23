use crate::models::*;
use regex::Regex;
use scraper::{Html, Selector};
use std::sync::LazyLock;

use super::utils::{clean_text, extract_img, from_scraped_str};

static TAG_ID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"th=(\d+)").expect("Invalid Regex"));
static VIEW_ALL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\((\d+)\)").expect("Invalid Regex"));
static NUMBER_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^\d.]").expect("Invalid Regex"));

pub fn parse_title_details(html: &str, title_id: &str) -> Option<TitleDetails> {
    let doc = Html::parse_document(html);

    let title = doc
        .select(&Selector::parse(".film-title").unwrap())
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let year = doc
        .select(&Selector::parse(".film-subtitle span").unwrap())
        .next()
        .and_then(|e| {
            e.text()
                .collect::<String>()
                .split('‧')
                .last()
                .map(|s| s.trim().parse::<i32>().unwrap_or(0))
        })
        .filter(|&y| y > 0);

    let poster = doc
        .select(&Selector::parse(".film-cover").unwrap())
        .next()
        .and_then(|el| extract_img(&el));

    let synopsis = doc
        .select(&Selector::parse(".show-synopsis span").unwrap())
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string());

    let mut native_title = None;
    let mut aka = Vec::new();
    let mut country = None;
    let mut r#type = None;
    let mut episodes = None;
    let mut aired = None;
    let mut aired_on = None;
    let mut duration = None;
    let mut content_rating = None;
    let mut score = None;
    let mut votes = None;
    let mut watchers = None;
    let mut rank = None;
    let mut popularity = None;
    let mut favorites = None;
    let mut original_network = Vec::new();
    let mut directors = Vec::new();
    let mut screenwriters = Vec::new();
    let mut genres = Vec::new();
    let mut tags = Vec::new();

    for li in doc.select(&Selector::parse(".show-detailsxss ul.list li.list-item").unwrap()) {
        let label_el = li.select(&Selector::parse("b.inline").unwrap()).next();
        if let Some(label) = label_el {
            let label_text = label.text().collect::<String>().trim().to_string();
            let full_text = li.text().collect::<String>();
            let value_text = full_text.replace(&label_text, "").trim().to_string();

            match label_text.as_str() {
                "Native Title:" => {
                    native_title = li
                        .select(&Selector::parse("a").unwrap())
                        .next()
                        .map(|e| e.text().collect::<String>().trim().to_string());
                }
                "Also Known As:" => {
                    if let Some(aka_el) = li
                        .select(&Selector::parse(".mdl-aka-titles").unwrap())
                        .next()
                    {
                        aka = aka_el
                            .text()
                            .collect::<String>()
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                }
                "Country:" => country = from_scraped_str::<Country>(&value_text),
                "Type:" => r#type = from_scraped_str::<Type>(&value_text),
                "Episodes:" => episodes = value_text.parse().ok(),
                "Aired:" => aired = Some(value_text),
                "Aired On:" => aired_on = Some(value_text),
                "Duration:" => duration = Some(value_text),
                "Content Rating:" => content_rating = Some(value_text),
                "Score:" => {
                    score = value_text
                        .split_whitespace()
                        .next()
                        .and_then(|s| s.parse().ok());
                    votes = li
                        .select(&Selector::parse(".hft a").unwrap())
                        .next()
                        .and_then(|e| {
                            NUMBER_RE
                                .replace_all(&e.text().collect::<String>(), "")
                                .parse()
                                .ok()
                        });
                }
                "Ranked:" => rank = NUMBER_RE.replace_all(&value_text, "").parse().ok(),
                "Popularity:" => popularity = NUMBER_RE.replace_all(&value_text, "").parse().ok(),
                "Watchers:" => watchers = NUMBER_RE.replace_all(&value_text, "").parse().ok(),
                "Favorites:" => favorites = NUMBER_RE.replace_all(&value_text, "").parse().ok(),
                "Original Network:" => {
                    original_network = li
                        .select(&Selector::parse("a").unwrap())
                        .map(|e| e.text().collect::<String>().trim().to_string())
                        .collect();
                }
                "Director:" => {
                    directors = li
                        .select(&Selector::parse("a").unwrap())
                        .filter_map(|e| {
                            let href = e.value().attr("href")?;
                            Some(CrewMember {
                                id: href.rsplit('/').next()?.to_string(),
                                name: e.text().collect::<String>().trim().to_string(),
                                job: Some("Director".to_string()),
                                image: None,
                            })
                        })
                        .collect();
                }
                "Screenwriter:" => {
                    screenwriters = li
                        .select(&Selector::parse("a").unwrap())
                        .filter_map(|e| {
                            let href = e.value().attr("href")?;
                            Some(CrewMember {
                                id: href.rsplit('/').next()?.to_string(),
                                name: e.text().collect::<String>().trim().to_string(),
                                job: Some("Screenwriter".to_string()),
                                image: None,
                            })
                        })
                        .collect();
                }
                "Genres:" => {
                    genres = li
                        .select(&Selector::parse("a").unwrap())
                        .filter_map(|e| from_scraped_str::<Genre>(&e.text().collect::<String>()))
                        .collect();
                }
                "Tags:" => {
                    tags = li
                        .select(&Selector::parse("a").unwrap())
                        .filter_map(|e| {
                            let href = e.value().attr("href")?;
                            let id_caps = TAG_ID_RE.captures(href)?;
                            Some(Tag {
                                id: id_caps.get(1)?.as_str().parse().ok()?,
                                name: e
                                    .text()
                                    .collect::<String>()
                                    .trim_matches(',')
                                    .trim()
                                    .to_string(),
                            })
                        })
                        .collect();
                }
                _ => {}
            }
        }
    }

    let mut reviews_count = None;
    for hfs in doc.select(&Selector::parse(".hfs").unwrap()) {
        let text = hfs.text().collect::<String>();
        if text.contains("Reviews:") {
            reviews_count = hfs
                .select(&Selector::parse("a").unwrap())
                .next()
                .and_then(|e| {
                    NUMBER_RE
                        .replace_all(&e.text().collect::<String>(), "")
                        .parse()
                        .ok()
                });
        } else if watchers.is_none() && text.contains("# of Watchers:") {
            watchers = hfs
                .select(&Selector::parse("b").unwrap())
                .next()
                .and_then(|e| {
                    NUMBER_RE
                        .replace_all(&e.text().collect::<String>(), "")
                        .parse()
                        .ok()
                });
        }
    }

    let statistics = Statistics {
        score,
        votes,
        watchers,
        reviews_count,
        rank,
        popularity,
        favorites,
    };

    let where_to_watch = doc
        .select(&Selector::parse(".box-body.wts .row.no-gutter").unwrap())
        .filter_map(|el| {
            let a_el = el
                .select(&Selector::parse(".col-xs-10 a").unwrap())
                .next()?;
            let link = a_el.value().attr("href")?;
            let name = a_el.text().collect::<String>().trim().to_string();
            let service_type = el
                .select(&Selector::parse(".col-xs-10 div:nth-child(2)").unwrap())
                .next()
                .map(|e| e.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let full_link = if link.starts_with('/') {
                format!("https://mydramalist.com{}", link)
            } else {
                link.to_string()
            };

            Some(WhereToWatch {
                name,
                service_type,
                link: full_link,
            })
        })
        .collect();

    let cast = doc
        .select(&Selector::parse(".credits .list-item").unwrap())
        .filter_map(|el| {
            let a_el = el
                .select(&Selector::parse(".credits-left a").unwrap())
                .next()?;
            let person_id = a_el.value().attr("href")?.rsplit('/').next()?.to_string();
            let image = extract_img(&el);
            let name = el
                .select(&Selector::parse("b[itempropx=\"name\"]").unwrap())
                .next()?
                .text()
                .collect::<String>()
                .trim()
                .to_string();

            let mut character = None;
            let mut character_id = None;

            // Extract Character Info (Same logic as cast parser)
            for small in el.select(&Selector::parse("small").unwrap()) {
                if !small.value().classes().any(|c| c == "text-muted") {
                    character = Some(clean_text(&small.text().collect::<String>()));

                    if let Some(a) = small.select(&Selector::parse("a").unwrap()).next() {
                        if let Some(href) = a.value().attr("href") {
                            character_id = Some(href.trim_start_matches("/character/").to_string());
                        }
                    }
                    break;
                }
            }

            let role = el
                .select(&Selector::parse("small.text-muted").unwrap())
                .next()
                .map(|e| e.text().collect::<String>().trim().to_string());

            Some(CastMember {
                person_id,
                name,
                character,
                character_id,
                role,
                image,
            })
        })
        .collect();

    let cast_count = doc
        .select(&Selector::parse(".box-footer a[href$=\"/cast\"]").unwrap())
        .next()
        .and_then(|e| {
            let text = e.text().collect::<String>();
            VIEW_ALL_RE
                .captures(&text)
                .and_then(|c| c.get(1))
                .and_then(|m| m.as_str().parse().ok())
        });

    let photos = doc
        .select(&Selector::parse(".box-body.photos img").unwrap())
        .filter_map(|img| {
            let img_url = img.value().attr("data-src").or(img.value().attr("src"))?;
            Some(Image::from(img_url.to_string()))
        })
        .collect();

    let photos_count = doc
        .select(&Selector::parse(".box-tool a[href$=\"/photos\"]").unwrap())
        .next()
        .and_then(|e| {
            let text = e.text().collect::<String>();
            VIEW_ALL_RE
                .captures(&text)
                .and_then(|c| c.get(1))
                .and_then(|m| m.as_str().parse().ok())
        });

    let recommendations = doc
        .select(&Selector::parse(".details-recommendations .rec-item").unwrap())
        .filter_map(|el| {
            let a_el = el.select(&Selector::parse("a").unwrap()).next()?;
            let id = a_el.value().attr("href")?.rsplit('/').next()?.to_string();
            let title = a_el.value().attr("title")?.to_string();
            let poster = extract_img(&el);
            Some(RecommendationPreview { id, title, poster })
        })
        .collect();

    Some(TitleDetails {
        id: title_id.to_string(),
        title,
        native_title,
        aka,
        year,
        poster,
        synopsis,
        statistics,
        country,
        r#type,
        episodes,
        aired,
        aired_on,
        original_network,
        duration,
        content_rating,
        directors,
        screenwriters,
        genres,
        tags,
        where_to_watch,
        cast,
        cast_count,
        photos,
        photos_count,
        recommendations,
    })
}
