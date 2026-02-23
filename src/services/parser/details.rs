use super::utils::{extract_character_info, extract_img};
use crate::models::*;
use regex::Regex;
use scraper::{Html, Selector};
use std::str::FromStr;
use std::sync::LazyLock;

static TAG_ID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"th=(\d+)").expect("Invalid Regex"));
static VIEW_ALL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\((\d+)\)").expect("Invalid Regex"));
static NUMBER_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^\d.]").expect("Invalid Regex"));

static FILM_TITLE_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".film-title").unwrap());
static FILM_SUBTITLE_SPAN_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".film-subtitle span").unwrap());
static FILM_COVER_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".film-cover").unwrap());
static SHOW_SYNOPSIS_SPAN_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".show-synopsis span").unwrap());
static SHOW_DETAILS_LI_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".show-detailsxss ul.list li.list-item").unwrap());
static B_INLINE_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("b.inline").unwrap());
static A_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("a").unwrap());
static MDL_AKA_TITLES_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".mdl-aka-titles").unwrap());
static HFT_A_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".hft a").unwrap());
static HFS_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".hfs").unwrap());
static B_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("b").unwrap());
static WTS_ROW_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".box-body.wts .row.no-gutter").unwrap());
static COL_XS_10_A_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".col-xs-10 a").unwrap());
static COL_XS_10_DIV2_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".col-xs-10 div:nth-child(2)").unwrap());
static CREDITS_LIST_ITEM_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".credits .list-item").unwrap());
static CREDITS_LEFT_A_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".credits-left a").unwrap());
static B_ITEMPROP_NAME_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("b[itempropx=\"name\"]").unwrap());
static SMALL_TEXT_MUTED_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("small.text-muted").unwrap());
static BOX_FOOTER_CAST_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".box-footer a[href$=\"/cast\"]").unwrap());
static BOX_BODY_PHOTOS_IMG_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".box-body.photos img").unwrap());
static BOX_TOOL_PHOTOS_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".box-tool a[href$=\"/photos\"]").unwrap());
static RECS_ITEM_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".details-recommendations .rec-item").unwrap());

pub fn parse_title_details(html: &str, title_id: &str) -> Option<TitleDetails> {
    let doc = Html::parse_document(html);

    let title = doc
        .select(&*FILM_TITLE_SEL)
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let year = doc
        .select(&*FILM_SUBTITLE_SPAN_SEL)
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
        .select(&*FILM_COVER_SEL)
        .next()
        .and_then(|el| extract_img(&el));

    let synopsis = doc
        .select(&*SHOW_SYNOPSIS_SPAN_SEL)
        .next()
        .map(|e| e.text().collect::<String>().trim().to_owned());

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

    for li in doc.select(&*SHOW_DETAILS_LI_SEL) {
        let label_el = li.select(&*B_INLINE_SEL).next();
        if let Some(label) = label_el {
            let label_text = label.text().collect::<String>().trim().to_owned();
            let full_text = li.text().collect::<String>();
            let value_text = full_text.replace(&label_text, "").trim().to_owned();

            match label_text.as_str() {
                "Native Title:" => {
                    native_title = li
                        .select(&*A_SEL)
                        .next()
                        .map(|e| e.text().collect::<String>().trim().to_owned());
                }
                "Also Known As:" => {
                    if let Some(aka_el) = li.select(&*MDL_AKA_TITLES_SEL).next() {
                        aka = aka_el
                            .text()
                            .collect::<String>()
                            .split(',')
                            .map(|s| s.trim().to_owned())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                }
                "Country:" => country = Country::from_str(&value_text).ok(),
                "Type:" => r#type = Type::from_str(&value_text).ok(),
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
                    votes = li.select(&*HFT_A_SEL).next().and_then(|e| {
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
                        .select(&*A_SEL)
                        .map(|e| e.text().collect::<String>().trim().to_owned())
                        .collect();
                }
                "Director:" => {
                    directors = li
                        .select(&*A_SEL)
                        .filter_map(|e| {
                            let href = e.value().attr("href")?;
                            Some(CrewMember {
                                id: href.rsplit('/').next()?.to_string(),
                                name: e.text().collect::<String>().trim().to_owned(),
                                job: Some("Director".to_string()),
                                image: None,
                            })
                        })
                        .collect();
                }
                "Screenwriter:" => {
                    screenwriters = li
                        .select(&*A_SEL)
                        .filter_map(|e| {
                            let href = e.value().attr("href")?;
                            Some(CrewMember {
                                id: href.rsplit('/').next()?.to_string(),
                                name: e.text().collect::<String>().trim().to_owned(),
                                job: Some("Screenwriter".to_string()),
                                image: None,
                            })
                        })
                        .collect();
                }
                "Genres:" => {
                    genres = li
                        .select(&*A_SEL)
                        .filter_map(|e| Genre::from_str(&e.text().collect::<String>().trim()).ok())
                        .collect();
                }
                "Tags:" => {
                    tags = li
                        .select(&*A_SEL)
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
    for hfs in doc.select(&*HFS_SEL) {
        let text = hfs.text().collect::<String>();
        if text.contains("Reviews:") {
            reviews_count = hfs.select(&*A_SEL).next().and_then(|e| {
                NUMBER_RE
                    .replace_all(&e.text().collect::<String>(), "")
                    .parse()
                    .ok()
            });
        } else if watchers.is_none() && text.contains("# of Watchers:") {
            watchers = hfs.select(&*B_SEL).next().and_then(|e| {
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
        .select(&*WTS_ROW_SEL)
        .filter_map(|el| {
            let a_el = el.select(&*COL_XS_10_A_SEL).next()?;
            let link = a_el.value().attr("href")?;
            let name = a_el.text().collect::<String>().trim().to_owned();
            let service_type = el
                .select(&*COL_XS_10_DIV2_SEL)
                .next()
                .map(|e| e.text().collect::<String>().trim().to_owned())
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
        .select(&*CREDITS_LIST_ITEM_SEL)
        .filter_map(|el| {
            let a_el = el.select(&*CREDITS_LEFT_A_SEL).next()?;
            let person_id = a_el.value().attr("href")?.rsplit('/').next()?.to_string();
            let image = extract_img(&el);
            let name = el
                .select(&*B_ITEMPROP_NAME_SEL)
                .next()?
                .text()
                .collect::<String>()
                .trim()
                .to_owned();

            let (character, character_id) = extract_character_info(&el);

            let role = el
                .select(&*SMALL_TEXT_MUTED_SEL)
                .next()
                .map(|e| e.text().collect::<String>().trim().to_owned());

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

    let cast_count = doc.select(&*BOX_FOOTER_CAST_SEL).next().and_then(|e| {
        let text = e.text().collect::<String>();
        VIEW_ALL_RE
            .captures(&text)
            .and_then(|c| c.get(1))
            .and_then(|m| m.as_str().parse().ok())
    });

    let photos = doc
        .select(&*BOX_BODY_PHOTOS_IMG_SEL)
        .filter_map(|img| {
            let img_url = img.value().attr("data-src").or(img.value().attr("src"))?;
            Some(Image::from(img_url.to_string()))
        })
        .collect();

    let photos_count = doc.select(&*BOX_TOOL_PHOTOS_SEL).next().and_then(|e| {
        let text = e.text().collect::<String>();
        VIEW_ALL_RE
            .captures(&text)
            .and_then(|c| c.get(1))
            .and_then(|m| m.as_str().parse().ok())
    });

    let recommendations = doc
        .select(&*RECS_ITEM_SEL)
        .filter_map(|el| {
            let a_el = el.select(&*A_SEL).next()?;
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
