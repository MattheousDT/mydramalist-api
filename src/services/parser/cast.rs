use super::utils::{extract_character_info, extract_img};
use crate::models::{CastMember, CrewMember, TitleCast};
use scraper::{ElementRef, Html, Selector};
use std::sync::LazyLock;

static BOX_BODY_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".box.cast-credits .box-body").unwrap());
static HEADER_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("h3.header").unwrap());
static LIST_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse("ul.list").unwrap());
static LIST_ITEM_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("li.list-item").unwrap());
static TEXT_PRIMARY_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("a.text-primary").unwrap());
static TEXT_MUTED_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("small.text-muted").unwrap());

pub fn parse_title_cast(html: &str) -> TitleCast {
    let doc = Html::parse_document(html);
    let mut cast = Vec::new();
    let mut crew = Vec::new();

    if let Some(box_body) = doc.select(&*BOX_BODY_SEL).next() {
        let mut current_category = String::new();

        for child in box_body.children() {
            if let Some(el) = ElementRef::wrap(child) {
                if el.value().name() == "h3" && HEADER_SEL.matches(&el) {
                    current_category = el.text().collect::<String>().trim().to_owned();
                } else if el.value().name() == "ul" && LIST_SEL.matches(&el) {
                    for li in el.select(&*LIST_ITEM_SEL) {
                        let is_crew_category = [
                            "Cinematography",
                            "Composer",
                            "Director",
                            "Screenwriter",
                            "Producer",
                            "Editor",
                            "Art Director",
                        ]
                        .contains(&current_category.as_str());

                        if is_crew_category {
                            if let Some(member) = parse_crew_member(&li, &current_category) {
                                crew.push(member);
                            }
                        } else {
                            if let Some(member) = parse_cast_member(&li, &current_category) {
                                cast.push(member);
                            }
                        }
                    }
                }
            }
        }
    }

    TitleCast { cast, crew }
}

fn parse_cast_member(el: &ElementRef, category: &str) -> Option<CastMember> {
    let name_el = el.select(&*TEXT_PRIMARY_SEL).next()?;
    let name = name_el.text().collect::<String>().trim().to_owned();
    let person_id = name_el
        .value()
        .attr("href")?
        .rsplit('/')
        .next()?
        .to_string();

    let image = extract_img(el);
    let (character, character_id) = extract_character_info(el);

    let role = el
        .select(&*TEXT_MUTED_SEL)
        .next()
        .map(|e| e.text().collect::<String>().trim().to_owned())
        .unwrap_or_else(|| category.to_string());

    Some(CastMember {
        person_id,
        name,
        character,
        character_id,
        role: Some(role),
        image,
    })
}

fn parse_crew_member(el: &ElementRef, job: &str) -> Option<CrewMember> {
    let name_el = el.select(&*TEXT_PRIMARY_SEL).next()?;
    let name = name_el.text().collect::<String>().trim().to_owned();
    let id = name_el
        .value()
        .attr("href")?
        .rsplit('/')
        .next()?
        .to_string();

    let image = extract_img(el);

    Some(CrewMember {
        id,
        name,
        job: Some(job.to_string()),
        image,
    })
}
