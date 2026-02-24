use crate::models::*;
use regex::Regex;
use scraper::{Html, Selector};
use std::str::FromStr;
use std::sync::LazyLock;

use super::utils::{extract_img, parse_pagination};

static MDL_INFO_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"([A-z\s]+) (Movie|TV Show|Drama|Special) - (\d{4}|TBA)(?:, (\d+) episodes)?")
        .expect("Invalid Regex")
});

static BOX_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".box").unwrap());
static H6_A_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("h6 a").unwrap());
static SCORE_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".score").unwrap());
static RANKING_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".ranking").unwrap());
static TEXT_MUTED_P_P_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".text-muted + p + p").unwrap());
static TEXT_MUTED_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".text-muted").unwrap());
static P_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("p").unwrap());
static LIKE_CNTB_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".like-cntb").unwrap());
static H6_PLUS_P_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("h6 + p").unwrap());
static CATEGORY_NAME_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".category-name").unwrap());
static PUB_DATE_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".pub-date").unwrap());
static LIKES_CNT_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".likes-cnt").unwrap());
static COMMENTS_CNT_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".comments-cnt").unwrap());

pub fn parse_title_search(html: &str) -> PaginatedTitleResults {
    let doc = Html::parse_document(html);

    let items = doc
        .select(&*BOX_SEL)
        .filter_map(|el| {
            let link = el.select(&*H6_A_SEL).next()?;
            let href = link.value().attr("href")?;
            if href == "/search" || !href.contains('/') {
                return None;
            }

            let id = href.rsplit('/').next().unwrap().to_string();
            let title = link.text().collect();

            let rating = el
                .select(&*SCORE_SEL)
                .next()
                .and_then(|e| e.text().collect::<String>().parse().ok());

            let ranking = el
                .select(&*RANKING_SEL)
                .next()
                .and_then(|e| e.text().collect::<String>().parse().ok());

            let poster = extract_img(&el);

            let description = el
                .select(&*TEXT_MUTED_P_P_SEL)
                .next()
                .map(|e| e.text().collect::<String>().trim().to_owned());

            let text = el
                .select(&*TEXT_MUTED_SEL)
                .next()
                .map(|e| e.text().collect::<String>().trim().to_owned())
                .unwrap();

            let caps = MDL_INFO_RE.captures(&text)?;

            let country = caps
                .get(1)
                .and_then(|m| Country::from_str(m.as_str()).ok())
                .unwrap();

            let type_str = caps.get(2).unwrap().as_str();
            let r#type = Type::from_str(type_str).unwrap();
            let format = if type_str == "Special" {
                Some(Format::Drama(DramaFormat::DramaSpecial))
            } else {
                None
            };

            let year = caps
                .get(3)
                .map(|m| m.as_str())
                .filter(|&s| s != "TBA")
                .and_then(|s| s.parse().ok());

            let episodes = caps.get(4).and_then(|m| m.as_str().parse().ok());

            Some(TitleSearchResult {
                id,
                r#type,
                format,
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
        .collect();

    let (page, total_pages) = parse_pagination(&doc);

    PaginatedTitleResults {
        items,
        page,
        total_pages,
    }
}

pub fn parse_people_search(html: &str) -> PaginatedPeopleResults {
    let doc = Html::parse_document(html);

    let items = doc
        .select(&*BOX_SEL)
        .filter_map(|el| {
            let link = el.select(&*H6_A_SEL).next()?;
            let href = link.value().attr("href")?;

            let id = href.rsplit('/').next().unwrap().to_string();
            let name = link.text().collect();

            let nationality = el
                .select(&*TEXT_MUTED_SEL)
                .next()
                .map(|e| e.text().collect::<String>().trim().to_owned());

            let bio = el
                .select(&*P_SEL)
                .next()
                .map(|e| e.text().collect::<String>().trim().to_owned());

            let hearts = el
                .select(&*LIKE_CNTB_SEL)
                .next()
                .map(|e| {
                    e.text()
                        .collect::<String>()
                        .trim()
                        .to_owned()
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
        .collect();

    let (page, total_pages) = parse_pagination(&doc);

    PaginatedPeopleResults {
        items,
        page,
        total_pages,
    }
}

pub fn parse_article_search(html: &str) -> PaginatedArticleResults {
    let doc = Html::parse_document(html);

    let items = doc
        .select(&*BOX_SEL)
        .filter_map(|el| {
            let link = el.select(&*H6_A_SEL).next()?;
            let href = link.value().attr("href")?;

            let id = href.rsplit('/').next().unwrap().to_string();
            let title = link.text().collect();

            let r#abstract = el
                .select(&*H6_PLUS_P_SEL)
                .next()
                .map(|e| e.text().collect::<String>().trim().to_owned())
                .unwrap();

            let category = el
                .select(&*CATEGORY_NAME_SEL)
                .next()
                .map(|e| e.text().collect::<String>().trim().to_owned())
                .and_then(|m| ArticleCategory::from_str(&m).ok())
                .unwrap();

            let date = el
                .select(&*PUB_DATE_SEL)
                .next()
                .map(|e| e.text().collect::<String>().trim().to_owned())
                .unwrap();

            let likes: i32 = el
                .select(&*LIKES_CNT_SEL)
                .next()
                .map(|e| {
                    e.text()
                        .collect::<String>()
                        .trim()
                        .to_owned()
                        .parse()
                        .unwrap()
                })
                .unwrap();

            let comments = el
                .select(&*COMMENTS_CNT_SEL)
                .next()
                .map(|e| {
                    e.text()
                        .collect::<String>()
                        .trim()
                        .to_owned()
                        .parse()
                        .unwrap()
                })
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
        .collect();

    let (page, total_pages) = parse_pagination(&doc);

    PaginatedArticleResults {
        items,
        page,
        total_pages,
    }
}
