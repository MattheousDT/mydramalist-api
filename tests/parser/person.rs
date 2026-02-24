use mydramalist_api::models::{Format, Gender, Nationality, Type};
use mydramalist_api::services::parser::MdlParser;

use crate::read_fixture;

#[test]
fn test_parse_person_details() {
    let html = read_fixture("person.html");
    let id = "1897-park-eun-bin";
    let details =
        MdlParser::parse_person_details(&html, id).expect("Failed to parse person details");

    assert_eq!(details.id, id);
    assert_eq!(details.name, "Park Eun Bin");
    assert_eq!(details.given_name.as_deref(), Some("Eun Bin"));
    assert_eq!(details.family_name.as_deref(), Some("Park"));
    assert_eq!(details.native_name.as_deref(), Some("박은빈"));
    assert_eq!(details.aka, vec!["Park Eunbin".to_string()]);
    assert_eq!(details.nationality, Some(Nationality::Korean));
    assert_eq!(details.gender, Some(Gender::Female));
    assert_eq!(details.born.as_deref(), Some("September 4, 1992"));
    assert_eq!(details.age, Some(33));

    assert_eq!(details.followers, 0);
    assert_eq!(details.hearts, 7291);

    let portrait = details.portrait.expect("Portrait should exist");
    assert!(portrait.small.contains("v7Evg_5s.jpg"));
    assert!(portrait.cover.contains("v7Evg_5c.jpg"));
    assert!(portrait.full.contains("v7Evg_5f.jpg"));

    let biography = details.biography.expect("Biography should exist");
    assert!(biography.contains("Park Eun Bin is a South Korean actress born in Seoul."));
    assert!(biography.contains("She played her first leading role in the time-traveling romance"));

    assert!(!details.works.is_empty());

    let woo_s2 = details
        .works
        .iter()
        .find(|w| w.id == "736027-extraordinary-attorney-woo-season-2")
        .expect("Extraordinary Attorney Woo Season 2 should be parsed");
    assert_eq!(woo_s2.title, "Extraordinary Attorney Woo Season 2");
    assert_eq!(woo_s2.r#type, Type::Drama);
    assert_eq!(woo_s2.format, None);
    assert_eq!(woo_s2.year, None);
    assert_eq!(woo_s2.episodes, Some(0));
    assert_eq!(woo_s2.role.as_deref(), Some("Woo Yeong U"));
    assert_eq!(woo_s2.role_type.as_deref(), Some("Main Role"));
    assert_eq!(woo_s2.rating, Some(0.0));

    let special = details
        .works
        .iter()
        .find(|w| w.id == "12677-kitchen-maid")
        .expect("Kitchen Maid should be parsed");
    assert_eq!(special.title, "Kitchen Maid");
    assert_eq!(special.r#type, Type::Drama);
    assert_eq!(special.format, Some(Format::DramaSpecial));
    assert_eq!(special.year, Some(2002));
    assert_eq!(special.episodes, Some(2));
    assert_eq!(special.role_type.as_deref(), Some("Support Role"));

    let movie = details
        .works
        .iter()
        .find(|w| w.id == "32645-boston-1947")
        .expect("Road to Boston should be parsed");
    assert_eq!(movie.title, "Road to Boston");
    assert_eq!(movie.r#type, Type::Movie);
    assert_eq!(woo_s2.format, None);
    assert_eq!(movie.year, Some(2023));
    assert_eq!(movie.episodes, None);
    assert_eq!(movie.role.as_deref(), Some("Seo Yoon Bbok's girlfriend"));
    assert_eq!(movie.role_type.as_deref(), Some("Guest Role"));

    let tv_show = details
        .works
        .iter()
        .find(|w| w.id == "739999-yoo-quiz-on-the-block-season-4")
        .expect("Yoo Quiz on the Block Season 4 should be parsed");
    assert_eq!(tv_show.title, "Yoo Quiz on the Block Season 4");
    assert_eq!(tv_show.r#type, Type::TVShow);
    assert_eq!(tv_show.format, None);
    assert_eq!(tv_show.year, Some(2022));
    assert_eq!(tv_show.episodes, Some(175));
    assert_eq!(tv_show.role.as_deref(), Some("(Ep. 163)"));
    assert_eq!(tv_show.role_type.as_deref(), Some("Guest"));
    assert_eq!(tv_show.rating, Some(7.6));
}
