use crate::models::{TitleStatistics, WatchStatusStats};
use regex::Regex;
use std::collections::BTreeMap;
use std::sync::LazyLock;

static CATEGORIES_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"categories:\s*\[(.*?)\]").expect("Invalid Regex"));

static DATA_ARRAY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"data:\s*\[([\d\.,\s]+)\]").expect("Invalid Regex"));

static PIE_ITEM_ARRAY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\['(.*?)',\s*(\d+)\]").expect("Invalid Regex"));

static PIE_CURRENT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"name:\s*'Currently watching',\s*y:\s*(\d+)").expect("Invalid Regex")
});

static SCORE_DATA_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"name:\s*'Votes',\s*data:\s*(\[.*?\])\s*\}").expect("Invalid Regex")
});

#[derive(serde::Deserialize)]
struct RawScoreItem {
    v: f32,
}

pub fn parse_title_statistics(html: &str, id: &str) -> TitleStatistics {
    TitleStatistics {
        id: id.to_string(),
        activity: extract_kv_chart::<i32>(html, "#chartline"),
        rating: extract_kv_chart::<f32>(html, "#overall_ratings"),
        watch_status: extract_watch_status(html).unwrap_or(WatchStatusStats {
            current: 0,
            completed: 0,
            plan_to_watch: 0,
            on_hold: 0,
            dropped: 0,
        }),
        score: extract_scores(html),
        age: BTreeMap::new(),
    }
}

fn extract_kv_chart<T>(html: &str, chart_id: &str) -> BTreeMap<String, T>
where
    T: std::str::FromStr + Copy,
{
    let mut map = BTreeMap::new();
    let pattern = format!(r"\$\('{chart_id}'\)\.highcharts\(([\s\S]*?)\);");
    let chart_block_re = Regex::new(&pattern).ok();
    let block = chart_block_re
        .and_then(|re| re.captures(html))
        .and_then(|c| c.get(1))
        .map(|m| m.as_str());

    if let Some(b) = block {
        let cats: Vec<String> = CATEGORIES_RE
            .captures(b)
            .and_then(|c| c.get(1))
            .map(|m| {
                m.as_str()
                    .split(',')
                    .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string())
                    .collect()
            })
            .unwrap_or_default();

        let vals: Vec<T> = DATA_ARRAY_RE
            .captures(b)
            .and_then(|c| c.get(1))
            .map(|m| {
                m.as_str()
                    .split(',')
                    .filter_map(|s| s.trim().parse::<T>().ok())
                    .collect()
            })
            .unwrap_or_default();

        for (k, v) in cats.into_iter().zip(vals) {
            map.insert(k, v);
        }
    }

    map
}

fn extract_watch_status(html: &str) -> Option<WatchStatusStats> {
    let chart_block_re = Regex::new(r"\$\('#chartpie'\)\.highcharts\(([\s\S]*?)\);").ok()?;
    let block = chart_block_re.captures(html)?.get(1)?.as_str();

    let current = PIE_CURRENT_RE
        .captures(block)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<i32>().ok())
        .unwrap_or(0);

    let mut completed = 0;
    let mut plan_to_watch = 0;
    let mut on_hold = 0;
    let mut dropped = 0;

    for caps in PIE_ITEM_ARRAY_RE.captures_iter(block) {
        let name = caps.get(1)?.as_str();
        let count = caps.get(2)?.as_str().parse::<i32>().unwrap_or(0);
        match name {
            "Completed" => completed = count,
            "Plan to watch" => plan_to_watch = count,
            "On-hold" => on_hold = count,
            "Dropped" => dropped = count,
            _ => {}
        }
    }

    Some(WatchStatusStats {
        current,
        completed,
        plan_to_watch,
        on_hold,
        dropped,
    })
}

fn extract_scores(html: &str) -> BTreeMap<String, f32> {
    let mut map = BTreeMap::new();
    let chart_block_re = Regex::new(r"\$\('#chart'\)\.highcharts\(([\s\S]*?)\);").ok();
    let block = chart_block_re
        .and_then(|re| re.captures(html))
        .and_then(|c| c.get(1))
        .map(|m| m.as_str());

    if let Some(b) = block {
        if let Some(data_match) = SCORE_DATA_RE.captures(b) {
            if let Ok(raw_items) =
                serde_json::from_str::<Vec<RawScoreItem>>(data_match.get(1).unwrap().as_str())
            {
                let mut current_score = 1.0;
                for item in raw_items {
                    map.insert(format!("{:.1}", current_score), item.v);
                    current_score += 0.5;
                }
            }
        }
    }

    map
}
