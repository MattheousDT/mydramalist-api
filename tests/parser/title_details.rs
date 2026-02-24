use mydramalist_api::models::{Country, Format, Genre, Type};
use mydramalist_api::services::parser::MdlParser;

use crate::read_fixture;

#[test]
fn test_parse_title_details() {
    let html = read_fixture("details.html");
    let id = "685237-untitled-kim-eun-sook-project";
    let details = MdlParser::parse_title_details(&html, id).expect("Failed to parse title details");

    assert_eq!(details.id, id);
    assert_eq!(details.title, "The Glory");
    assert_eq!(details.native_title.as_deref(), Some("더 글로리"));
    assert_eq!(details.year, Some(2022));
    assert_eq!(details.episodes, Some(8));
    assert_eq!(details.aired.as_deref(), Some("Dec 30, 2022"));
    assert_eq!(details.aired_on.as_deref(), Some("Friday"));
    assert_eq!(details.duration.as_deref(), Some("50 min."));
    assert_eq!(
        details.content_rating.as_deref(),
        Some("18+ Restricted (violence & profanity)")
    );
    assert_eq!(details.country, Some(Country::SouthKorea));
    assert_eq!(details.r#type, Some(Type::Drama));
    assert_eq!(details.format, Some(Format::StandardSeries));

    assert_eq!(details.aka.len(), 17);
    assert!(details.aka.contains(&"The Glory Part 1".to_string()));
    assert!(
        details
            .aka
            .contains(&"Untitled Kim Eun Sook Project".to_string())
    );

    let poster = details.poster.expect("Poster should exist");
    assert!(poster.small.contains("4v6zJ_4s.jpg"));
    assert!(poster.cover.contains("4v6zJ_4c.jpg"));
    assert!(poster.full.contains("4v6zJ_4f.jpg"));

    assert!(
        details
            .synopsis
            .as_ref()
            .unwrap()
            .contains("A high school student dreams of becoming an architect.")
    );

    let stats = details.statistics;
    assert_eq!(stats.score, Some(8.9));
    assert_eq!(stats.votes, Some(107326));
    assert_eq!(stats.watchers, Some(166124));
    assert_eq!(stats.reviews_count, Some(225));
    assert_eq!(stats.rank, Some(106));
    assert_eq!(stats.popularity, Some(35));

    assert_eq!(details.original_network, vec!["Netflix".to_string()]);

    assert_eq!(details.directors.len(), 1);
    assert_eq!(details.directors[0].id, "16031-ahn-gil-ho");
    assert_eq!(details.directors[0].name, "Ahn Gil Ho");

    assert_eq!(details.screenwriters.len(), 1);
    assert_eq!(details.screenwriters[0].id, "15905-kim-eun-sook");
    assert_eq!(details.screenwriters[0].name, "Kim Eun Sook");

    assert_eq!(
        details.genres,
        vec![Genre::Thriller, Genre::Drama, Genre::Melodrama]
    );

    assert_eq!(details.tags.len(), 10);
    assert_eq!(details.tags[0].id, 370);
    assert_eq!(details.tags[0].name, "Revenge");

    assert_eq!(details.where_to_watch.len(), 1);
    assert_eq!(details.where_to_watch[0].name, "Netflix");
    assert_eq!(details.where_to_watch[0].service_type, "Subscription (sub)");

    assert_eq!(details.cast.len(), 6);
    let cast1 = &details.cast[0];
    assert_eq!(cast1.person_id, "294-song-hye-kyo");
    assert_eq!(cast1.name, "Song Hye Kyo");
    assert_eq!(cast1.character.as_deref(), Some("Moon Dong Eun"));
    assert_eq!(cast1.role.as_deref(), Some("Main Role"));
    assert!(cast1.image.as_ref().unwrap().full.contains("KpY8K8_5f.jpg"));

    assert_eq!(details.cast_count, Some(79));

    assert_eq!(details.photos.len(), 6);
    assert_eq!(details.photos_count, Some(104));

    assert_eq!(details.recommendations.len(), 6);
    assert_eq!(details.recommendations[0].id, "702267-weak-hero");
    assert_eq!(details.recommendations[0].title, "Weak Hero Class 1");
}
