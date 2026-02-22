use crate::models::Episode;
use regex::Regex;
use scraper::{Html, Selector};
use std::sync::LazyLock;

use super::utils::{clean_text, extract_img};

static USERS_SCORED_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"from ([\d,]+) users").expect("Invalid Regex"));

pub fn parse_title_episodes(html: &str) -> Vec<Episode> {
    let doc = Html::parse_document(html);
    let ep_sel = Selector::parse(".episodes .episode").unwrap();
    let title_sel = Selector::parse(".title a").unwrap();
    let rating_val_sel = Selector::parse(".rating-panel b").unwrap();
    let rating_text_sel = Selector::parse(".rating-panel div").unwrap();
    let date_sel = Selector::parse(".air-date").unwrap();
    let sum_sel = Selector::parse(".summary").unwrap();

    doc.select(&ep_sel)
        .filter_map(|el| {
            let a_el = el.select(&title_sel).next()?;
            let title = clean_text(&a_el.text().collect::<String>());
            let href = a_el.value().attr("href")?;

            // Extract episode number from url, e.g., /49231-move-to-heaven/episode/1
            let episode_number = href.rsplit('/').next()?.parse::<i32>().ok()?;

            let cover = extract_img(&el);

            let score = el
                .select(&rating_val_sel)
                .next()
                .and_then(|e| e.text().collect::<String>().parse::<f32>().ok());

            let votes = el.select(&rating_text_sel).next().and_then(|e| {
                let text = e.text().collect::<String>();
                USERS_SCORED_RE
                    .captures(&text)
                    .and_then(|c| c.get(1))
                    .and_then(|m| m.as_str().replace(',', "").parse::<i32>().ok())
            });

            let air_date = el
                .select(&date_sel)
                .next()
                .map(|e| clean_text(&e.text().collect::<String>()));

            let summary = el
                .select(&sum_sel)
                .next()
                .map(|e| clean_text(&e.text().collect::<String>()));

            Some(Episode {
                episode_number,
                title,
                score,
                votes,
                air_date,
                summary,
                cover,
            })
        })
        .collect()
}
