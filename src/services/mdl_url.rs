use crate::models::*;
use url::form_urlencoded::Serializer;

pub struct MdlUrlBuilder;

impl MdlUrlBuilder {
    pub fn title_reviews(id: &str, query: &ReviewSearchQuery) -> String {
        let mut serializer = Serializer::new(String::from(format!(
            "https://mydramalist.com/{}/reviews?",
            id
        )));
        serializer.append_pair("page", &query.page.unwrap_or(1).to_string());

        if let Some(v) = query.sort {
            serializer.append_pair("sort", &v.to_string());
        }
        if let Some(v) = query.status {
            serializer.append_pair("status", &v.to_string());
        }
        if let Some(true) = query.hide_spoiler {
            serializer.append_pair("spoiler", "0");
        }

        serializer.finish()
    }

    pub fn search_titles(query: &TitleSearchQuery) -> String {
        let mut serializer =
            Serializer::new(String::from("https://mydramalist.com/search?adv=titles&"));

        serializer.append_pair("page", &query.page.unwrap_or(1).to_string());

        if let Some(q) = &query.q {
            serializer.append_pair("q", q);
        }
        if let Some(v) = query.r#type {
            serializer.append_pair("ty", &(v as i32).to_string());
        }
        if let Some(v) = query.country {
            serializer.append_pair("co", &(v as i32).to_string());
        }
        if let Some(v) = query.genre {
            serializer.append_pair("ge", &(v as i32).to_string());
        }
        if let Some(v) = query.tag {
            serializer.append_pair("th", &v.to_string());
        }
        if let Some(v) = query.network {
            serializer.append_pair("nt", &v.to_string());
        }
        if let Some(v) = query.service {
            serializer.append_pair("sr", &v.to_string());
        }

        // Reconstruct the tuples using sensible defaults for the missing half
        if query.release_year_min.is_some() || query.release_year_max.is_some() {
            let min = query.release_year_min.unwrap_or(1900);
            let max = query.release_year_max.unwrap_or(2050);
            serializer.append_pair("re", &format!("{},{}", min, max));
        }

        if query.rating_min.is_some() || query.rating_max.is_some() {
            let min = query.rating_min.unwrap_or(0);
            let max = query.rating_max.unwrap_or(10);
            serializer.append_pair("rt", &format!("{},{}", min, max));
        }

        if query.episodes_min.is_some() || query.episodes_max.is_some() {
            let min = query.episodes_min.unwrap_or(0);
            let max = query.episodes_max.unwrap_or(999);
            serializer.append_pair("ep", &format!("{},{}", min, max));
        }

        if query.runtime_min.is_some() || query.runtime_max.is_some() {
            let min = query.runtime_min.unwrap_or(0);
            let max = query.runtime_max.unwrap_or(999);
            serializer.append_pair("ru", &format!("{},{}", min, max));
        }

        if let Some(v) = query.status {
            serializer.append_pair("st", &(v as i32).to_string());
        }
        if let Some(v) = query.format {
            let fo = match v {
                Format::Drama(f) => f as i32,
                Format::Movie(f) => f as i32,
                Format::TV(f) => f as i32,
            };
            serializer.append_pair("fo", &fo.to_string());
        }
        if let Some(v) = query.sort {
            serializer.append_pair("so", &v.to_string());
        }

        serializer.finish()
    }

    pub fn search_people(query: &PeopleSearchQuery) -> String {
        let mut serializer =
            Serializer::new(String::from("https://mydramalist.com/search?adv=people&"));

        serializer.append_pair("page", &query.page.unwrap_or(1).to_string());

        if let Some(q) = &query.q {
            serializer.append_pair("q", q);
        }
        if let Some(v) = query.nationality {
            serializer.append_pair("na", &(v as i32).to_string());
        }
        if let Some(v) = query.gender {
            serializer.append_pair("gd", &(v as i32).to_string());
        }
        if let Some(v) = query.sort {
            serializer.append_pair("so", &v.to_string());
        }

        serializer.finish()
    }

    pub fn search_articles(query: &ArticleSearchQuery) -> String {
        let mut serializer =
            Serializer::new(String::from("https://mydramalist.com/search?adv=articles&"));

        serializer.append_pair("page", &query.page.unwrap_or(1).to_string());

        if let Some(q) = &query.q {
            serializer.append_pair("q", q);
        }
        if let Some(v) = query.category {
            serializer.append_pair("aca", &(v as i32).to_string());
        }
        if let Some(v) = query.sort {
            serializer.append_pair("so", &v.to_string());
        }

        serializer.finish()
    }
}
