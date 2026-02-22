use crate::models::*;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::sync::LazyLock;

static MDL_INFO_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"([A-z\s]+) (Movie|TV Show|Drama) - (\d{4}|TBA)(?:, (\d+) episodes)?")
        .expect("Invalid Regex")
});

pub struct MdlParser;

impl MdlParser {
    pub fn parse_title_search(html: &str) -> Vec<TitleSearchResult> {
        let doc = Html::parse_document(html);
        let box_sel = Selector::parse(".box").unwrap();

        doc.select(&box_sel)
            .filter_map(|el| {
                let link = el.select(&Selector::parse("h6 a").unwrap()).next()?;
                let href = link.value().attr("href")?;
                if href == "/search" || !href.contains('/') {
                    return None;
                }

                let id = href.rsplit('/').next().unwrap().to_string();
                let title = link.text().collect();

                let rating = el
                    .select(&Selector::parse(".score").unwrap())
                    .next()
                    .and_then(|e| e.text().collect::<String>().parse().ok());

                let ranking = el
                    .select(&Selector::parse(".ranking").unwrap())
                    .next()
                    .and_then(|e| e.text().collect::<String>().parse().ok());

                let poster = extract_img(&el).map(|i| Image::from(i));

                let description = el
                    .select(&Selector::parse(".text-muted + p + p").unwrap())
                    .next()
                    .map(|e| clean_text(&e.text().collect::<String>()));

                // Now the fun begins...
                let text = el
                    .select(&Selector::parse(".text-muted").unwrap())
                    .next()
                    .map(|e| clean_text(&e.text().collect::<String>()))
                    .unwrap();
                let caps = MDL_INFO_RE.captures(&text)?;

                // 1. Country (Group 1)
                let country = caps
                    .get(1)
                    .and_then(|m| from_scraped_str(m.as_str()))
                    .unwrap();

                // 2. Type (Group 2)
                let r#type = caps
                    .get(2)
                    .and_then(|m| from_scraped_str(m.as_str()))
                    .unwrap();

                // 3. Year (Group 3) - Handle "TBA"
                let year = caps
                    .get(3)
                    .map(|m| m.as_str())
                    .filter(|&s| s != "TBA")
                    .and_then(|s| s.parse().ok());

                // 4. Episodes (Group 4) - Optional
                let episodes = caps.get(4).and_then(|m| m.as_str().parse().ok());

                Some(TitleSearchResult {
                    id,
                    r#type,
                    title,
                    year,
                    rating,
                    country,
                    description,
                    episodes,
                    ranking,
                    poster,
                })
            })
            .collect()
    }

    pub fn parse_people_search(html: &str) -> Vec<PeopleSearchResult> {
        let doc = Html::parse_document(html);
        let box_sel = Selector::parse(".box").unwrap();

        doc.select(&box_sel)
            .filter_map(|el| {
                let link = el.select(&Selector::parse("h6 a").unwrap()).next()?;
                let href = link.value().attr("href")?;

                let id = href.rsplit('/').next().unwrap().to_string();
                let name = link.text().collect();

                let nationality = el
                    .select(&Selector::parse(".text-muted").unwrap())
                    .next()
                    .map(|e| clean_text(&e.text().collect::<String>()));

                let bio = el
                    .select(&Selector::parse("p").unwrap())
                    .next()
                    .map(|e| clean_text(&e.text().collect::<String>()));

                let hearts = el
                    .select(&Selector::parse(".like-cntb").unwrap())
                    .next()
                    .map(|e| {
                        clean_text(&e.text().collect::<String>())
                            .replace(",", "")
                            .parse()
                            .ok()
                    })
                    .flatten();

                let portrait = extract_img(&el).map(|i| Image::from(i));

                Some(PeopleSearchResult {
                    id,
                    name,
                    nationality,
                    bio,
                    hearts,
                    portrait,
                })
            })
            .collect()
    }

    pub fn parse_article_search(html: &str) -> Vec<ArticleSearchResult> {
        let doc = Html::parse_document(html);
        let box_sel = Selector::parse(".box").unwrap();

        doc.select(&box_sel)
            .filter_map(|el| {
                let link = el.select(&Selector::parse("h6 a").unwrap()).next()?;
                let href = link.value().attr("href")?;

                let id = href.rsplit('/').next().unwrap().to_string();
                let title = link.text().collect();

                let r#abstract = el
                    .select(&Selector::parse("h6 + p").unwrap())
                    .next()
                    .map(|e| clean_text(&e.text().collect::<String>()))
                    .unwrap();

                let category = el
                    .select(&Selector::parse(".category-name").unwrap())
                    .next()
                    .map(|e| clean_text(&e.text().collect::<String>()))
                    .and_then(|m| from_scraped_str(m.as_str()))
                    .unwrap();

                let date = el
                    .select(&Selector::parse(".pub-date").unwrap())
                    .next()
                    .map(|e| clean_text(&e.text().collect::<String>()))
                    .unwrap();

                let likes: i32 = el
                    .select(&Selector::parse(".likes-cnt").unwrap())
                    .next()
                    .map(|e| clean_text(&e.text().collect::<String>()).parse().unwrap())
                    .unwrap();

                let comments = el
                    .select(&Selector::parse(".comments-cnt").unwrap())
                    .next()
                    .map(|e| clean_text(&e.text().collect::<String>()).parse().unwrap())
                    .unwrap();

                let image = extract_img(&el).map(|i| Image::from(i));

                Some(ArticleSearchResult {
                    id,
                    title,
                    r#abstract,
                    category,
                    date,
                    likes,
                    comments,
                    image,
                })
            })
            .collect()
    }
}

// --- Internal Helpers ---

fn extract_img(el: &ElementRef) -> Option<String> {
    el.select(&Selector::parse("img").unwrap())
        .next()
        .and_then(|img| {
            img.value()
                .attr("data-src")
                .or(img.value().attr("src"))
                .map(|s| s.to_string())
        })
}

/// Helper function to trim and clean strings
fn clean_text(s: &str) -> String {
    s.trim().to_string()
}

fn from_scraped_str<T>(s: &str) -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    // Trim the string before wrapping in quotes to ensure clean matching
    let cleaned = s.trim();
    let json_compat = format!("\"{}\"", cleaned);

    match serde_json::from_str::<T>(&json_compat) {
        Ok(val) => Some(val),
        Err(_) => {
            // Log exactly what failed so you can add a new alias to the enum
            tracing::warn!("Failed to map scraped string to Enum: [{}]", cleaned);
            None
        }
    }
}
