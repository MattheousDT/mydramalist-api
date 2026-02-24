use crate::models::{Format, Gender, Nationality, PersonDetails, PersonWork, Type};
use crate::services::parser::utils::extract_img;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::str::FromStr;
use std::sync::LazyLock;

static WHITESPACE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\s+").expect("Invalid Regex"));

static FILM_TITLE_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".film-title").unwrap());
static FOLLOWERS_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".stats-followers .stats-num").unwrap());
static HEARTS_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".stats-points .stats-num").unwrap());
static PORTRAIT_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".content-side .box-body img.img-responsive").unwrap());

static DETAILS_LI_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".box-body.light-b ul.list li.list-item").unwrap());
static B_INLINE_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("b.inline").unwrap());

static BIO_CONTAINER_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".col-sm-8.col-lg-12.col-md-12").unwrap());

static BOX_BODY_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".box-body").unwrap());
static HEADER_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("h5.header").unwrap());
static FILM_LIST_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("table.film-list").unwrap());
static FILM_LIST_TR_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("tbody tr").unwrap());

static TR_YEAR_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("td.year").unwrap());
static TR_TITLE_A_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("td.title b a").unwrap());
static TR_EPISODES_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("td.episodes").unwrap());
static TR_ROLE_NAME_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("td.role .name").unwrap());
static TR_ROLE_ID_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("td.role .roleid").unwrap());
static TR_RATING_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("td.text-center .text-sm").unwrap());

pub fn parse_person_details(html: &str, person_id: &str) -> Option<PersonDetails> {
    let doc = Html::parse_document(html);

    let name = doc
        .select(&*FILM_TITLE_SEL)
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let followers = doc
        .select(&*FOLLOWERS_SEL)
        .next()
        .and_then(|e| e.text().collect::<String>().replace(',', "").parse().ok())
        .unwrap_or(0);

    let hearts = doc
        .select(&*HEARTS_SEL)
        .next()
        .and_then(|e| e.text().collect::<String>().replace(',', "").parse().ok())
        .unwrap_or(0);

    let portrait = doc
        .select(&*PORTRAIT_SEL)
        .next()
        .and_then(|el| extract_img(&el));

    let mut given_name = None;
    let mut family_name = None;
    let mut native_name = None;
    let mut aka = Vec::new();
    let mut nationality = None;
    let mut gender = None;
    let mut born = None;
    let mut age = None;

    for li in doc.select(&*DETAILS_LI_SEL) {
        let label = li
            .select(&*B_INLINE_SEL)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_owned())
            .unwrap_or_default();
        let full_text = li.text().collect::<String>();
        let value = full_text.replace(&label, "").trim().to_owned();

        match label.as_str() {
            "First Name:" => given_name = Some(value),
            "Family Name:" => family_name = Some(value),
            "Native name:" => native_name = Some(value),
            "Also Known as:" => {
                aka = value
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            }
            "Nationality:" => nationality = Nationality::from_str(&value).ok(),
            "Gender:" => {
                gender = match value.to_lowercase().as_str() {
                    "male" => Some(Gender::Male),
                    "female" => Some(Gender::Female),
                    _ => None,
                }
            }
            "Born:" => born = Some(value),
            "Age:" => age = value.parse().ok(),
            _ => {}
        }
    }

    let mut biography = String::new();
    if let Some(col) = doc.select(&*BIO_CONTAINER_SEL).next() {
        for child in col.children() {
            if let Some(el) = ElementRef::wrap(child) {
                let node_name = el.value().name();
                if el.value().classes().any(|c| c == "hidden-md-up") {
                    continue;
                }
                if node_name == "a" && el.text().collect::<String>().contains("Edit Biography") {
                    continue;
                }
                if node_name == "br" {
                    biography.push('\n');
                } else {
                    let text = el.text().collect::<String>();
                    biography.push_str(&WHITESPACE_RE.replace_all(&text, " "));
                }
            } else if let Some(text) = child.value().as_text() {
                let text = text.to_string();
                biography.push_str(&WHITESPACE_RE.replace_all(&text, " "));
            }
        }
    }

    let biography = biography.replace(" \n", "\n").replace("\n ", "\n");
    let biography = biography.trim().to_string();
    let biography = if biography.is_empty() {
        None
    } else {
        Some(biography)
    };

    let mut works = Vec::new();
    for box_body in doc.select(&*BOX_BODY_SEL) {
        let mut r#type = None;
        let mut format = None;

        for child in box_body.children() {
            if let Some(el) = ElementRef::wrap(child) {
                if el.value().name() == "h5" && HEADER_SEL.matches(&el) {
                    let type_text = el.text().collect::<String>().trim().to_owned();

                    r#type = Type::from_str(&type_text).ok();
                    format = if type_text == "Special" {
                        Some(Format::DramaSpecial)
                    } else {
                        None
                    };
                } else if el.value().name() == "table" && FILM_LIST_SEL.matches(&el) {
                    let r#type = match r#type {
                        Some(c) => c,
                        None => continue,
                    };

                    for tr in el.select(&*FILM_LIST_TR_SEL) {
                        let year_text = tr
                            .select(&*TR_YEAR_SEL)
                            .next()
                            .map(|e| e.text().collect::<String>().trim().to_owned())
                            .unwrap_or_default();
                        let year = year_text.parse::<i32>().ok();

                        let a_el = match tr.select(&*TR_TITLE_A_SEL).next() {
                            Some(a) => a,
                            None => continue,
                        };
                        let title = a_el.text().collect::<String>().trim().to_owned();
                        let work_id = a_el
                            .value()
                            .attr("href")
                            .and_then(|h| h.rsplit('/').next())
                            .unwrap_or_default()
                            .to_string();

                        let episodes = tr
                            .select(&*TR_EPISODES_SEL)
                            .next()
                            .and_then(|e| e.text().collect::<String>().trim().parse::<i32>().ok());

                        let role = tr
                            .select(&*TR_ROLE_NAME_SEL)
                            .next()
                            .map(|e| e.text().collect::<String>().trim().to_owned())
                            .filter(|s| !s.is_empty());

                        let role_type = tr
                            .select(&*TR_ROLE_ID_SEL)
                            .next()
                            .map(|e| e.text().collect::<String>().trim().to_owned())
                            .filter(|s| !s.is_empty());

                        let rating = tr
                            .select(&*TR_RATING_SEL)
                            .next()
                            .and_then(|e| e.text().collect::<String>().trim().parse::<f32>().ok());

                        works.push(PersonWork {
                            id: work_id,
                            title,
                            r#type,
                            format,
                            year,
                            episodes,
                            role,
                            role_type,
                            rating,
                        });
                    }
                }
            }
        }
    }

    Some(PersonDetails {
        id: person_id.to_string(),
        name,
        given_name,
        family_name,
        native_name,
        aka,
        nationality,
        gender,
        born,
        age,
        biography,
        followers,
        hearts,
        portrait,
        works,
    })
}
