use crate::models::*;

pub struct MdlUrlBuilder;

impl MdlUrlBuilder {
    pub fn search_titles(query: &TitleSearchQuery) -> String {
        let mut p = vec!["adv=titles".to_string()];
        if let Some(q) = &query.q {
            p.push(format!("q={}", urlencoding::encode(q)));
        }
        if let Some(v) = query.r#type {
            p.push(format!("ty={}", v as i32));
        }
        if let Some(v) = query.country {
            p.push(format!("co={}", v as i32));
        }
        if let Some(v) = query.genre {
            p.push(format!("ge={}", v as i32));
        }
        if let Some(v) = query.tag {
            p.push(format!("th={}", v));
        }
        if let Some(v) = query.network {
            p.push(format!("nt={}", v));
        }
        if let Some(v) = query.service {
            p.push(format!("sr={}", v));
        }
        if let Some(v) = query.release_date {
            p.push(format!("re={},{}", v.0, v.1));
        }
        if let Some(v) = query.rating {
            p.push(format!("rt={},{}", v.0, v.1));
        }
        if let Some(v) = query.episodes {
            p.push(format!("ep={},{}", v.0, v.1));
        }
        if let Some(v) = query.runtime {
            p.push(format!("ru={},{}", v.0, v.1));
        }
        if let Some(v) = query.status {
            p.push(format!("st={}", v as i32));
        }
        if let Some(v) = query.format {
            p.push(format!(
                "fo={}",
                match v {
                    Format::Drama(f) => f as i32,
                    Format::Movie(f) => f as i32,
                    Format::TV(f) => f as i32,
                }
            ));
        }
        if let Some(v) = query.sort {
            p.push(format!("so={}", v));
        }
        format!("https://mydramalist.com/search?{}", p.join("&"))
    }

    pub fn search_people(query: &PeopleSearchQuery) -> String {
        let mut p = vec!["adv=people".to_string()];
        if let Some(q) = &query.q {
            p.push(format!("q={}", urlencoding::encode(q)));
        }
        if let Some(v) = query.nationality {
            p.push(format!("na={}", v as i32));
        }
        if let Some(v) = query.gender {
            p.push(format!("gd={}", v as i32));
        }
        if let Some(v) = query.sort {
            p.push(format!("so={}", v));
        }
        format!("https://mydramalist.com/search?{}", p.join("&"))
    }

    pub fn search_articles(query: &ArticleSearchQuery) -> String {
        let mut p = vec!["adv=articles".to_string()];
        if let Some(q) = &query.q {
            p.push(format!("q={}", urlencoding::encode(q)));
        }
        if let Some(v) = query.category {
            p.push(format!("aca={}", v as i32));
        }
        if let Some(v) = query.sort {
            p.push(format!("so={}", v));
        }
        format!("https://mydramalist.com/search?{}", p.join("&"))
    }
}
