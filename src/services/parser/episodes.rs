use super::utils::extract_img;
use crate::models::Episode;
use regex::Regex;
use scraper::{Html, Selector};
use std::sync::LazyLock;

static USERS_SCORED_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"from ([\d,]+) users").expect("Invalid Regex"));

static EP_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".episodes .episode").unwrap());
static TITLE_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".title a").unwrap());
static RATING_VAL_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".rating-panel b").unwrap());
static RATING_TEXT_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".rating-panel div").unwrap());
static DATE_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".air-date").unwrap());
static SUM_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".summary").unwrap());

pub fn parse_title_episodes(html: &str) -> Vec<Episode> {
    let doc = Html::parse_document(html);

    doc.select(&*EP_SEL)
        .filter_map(|el| {
            let a_el = el.select(&*TITLE_SEL).next()?;
            let title = a_el.text().collect::<String>().trim().to_owned();
            let href = a_el.value().attr("href")?;

            let episode_number = href.rsplit('/').next()?.parse::<i32>().ok()?;

            let cover = extract_img(&el);

            let score = el
                .select(&*RATING_VAL_SEL)
                .next()
                .and_then(|e| e.text().collect::<String>().parse::<f32>().ok());

            let votes = el.select(&*RATING_TEXT_SEL).next().and_then(|e| {
                let text = e.text().collect::<String>();
                USERS_SCORED_RE
                    .captures(&text)
                    .and_then(|c| c.get(1))
                    .and_then(|m| m.as_str().replace(',', "").parse::<i32>().ok())
            });

            let air_date = el
                .select(&*DATE_SEL)
                .next()
                .map(|e| e.text().collect::<String>().trim().to_owned());

            let summary = el
                .select(&*SUM_SEL)
                .next()
                .map(|e| e.text().collect::<String>().trim().to_owned());

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
