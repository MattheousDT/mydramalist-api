use crate::models::*;
use regex::Regex;
use scraper::{Html, Selector};
use std::sync::LazyLock;

use super::utils::{clean_text, extract_img, from_scraped_str};

static MDL_INFO_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"([A-z\s]+) (Movie|TV Show|Drama) - (\d{4}|TBA)(?:, (\d+) episodes)?")
        .expect("Invalid Regex")
});

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

            let poster = extract_img(&el);

            let description = el
                .select(&Selector::parse(".text-muted + p + p").unwrap())
                .next()
                .map(|e| clean_text(&e.text().collect::<String>()));

            let text = el
                .select(&Selector::parse(".text-muted").unwrap())
                .next()
                .map(|e| clean_text(&e.text().collect::<String>()))
                .unwrap();

            let caps = MDL_INFO_RE.captures(&text)?;

            let country = caps
                .get(1)
                .and_then(|m| from_scraped_str::<Country>(m.as_str()))
                .unwrap();

            let r#type = caps
                .get(2)
                .and_then(|m| from_scraped_str::<Type>(m.as_str()))
                .unwrap();

            let year = caps
                .get(3)
                .map(|m| m.as_str())
                .filter(|&s| s != "TBA")
                .and_then(|s| s.parse().ok());

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

            let portrait = extract_img(&el);

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
                .and_then(|m| from_scraped_str::<ArticleCategory>(&m))
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

            let image = extract_img(&el);

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
