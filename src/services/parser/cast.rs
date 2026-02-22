use crate::models::{CastMember, CrewMember, TitleCast};
use scraper::{ElementRef, Html, Selector};

use super::utils::{clean_text, extract_img};

pub fn parse_title_cast(html: &str) -> TitleCast {
    let doc = Html::parse_document(html);
    let box_body_sel = Selector::parse(".box.cast-credits .box-body").unwrap();
    let header_sel = Selector::parse("h3.header").unwrap();
    let list_sel = Selector::parse("ul.list").unwrap();
    let list_item_sel = Selector::parse("li.list-item").unwrap();

    let mut cast = Vec::new();
    let mut crew = Vec::new();

    if let Some(box_body) = doc.select(&box_body_sel).next() {
        let mut current_category = String::new();

        for child in box_body.children() {
            if let Some(el) = ElementRef::wrap(child) {
                if el.value().name() == "h3" && header_sel.matches(&el) {
                    current_category = clean_text(&el.text().collect::<String>());
                } else if el.value().name() == "ul" && list_sel.matches(&el) {
                    for li in el.select(&list_item_sel) {
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
    let name_el = el
        .select(&Selector::parse("a.text-primary").unwrap())
        .next()?;
    let name = clean_text(&name_el.text().collect::<String>());
    let person_id = name_el
        .value()
        .attr("href")?
        .rsplit('/')
        .next()?
        .to_string();

    let image = extract_img(el);

    let mut character = None;
    let mut character_id = None;

    // Find the character small tag (any small tag that isn't the role/text-muted)
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
        .map(|e| clean_text(&e.text().collect::<String>()))
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
    let name_el = el
        .select(&Selector::parse("a.text-primary").unwrap())
        .next()?;
    let name = clean_text(&name_el.text().collect::<String>());
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
