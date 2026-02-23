use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use utoipa::ToSchema;

/// Comprehensive statistics for a title including trends, watch status, and demographics.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TitleStatistics {
    /// Unique identifier for the title.
    #[schema(example = "49231-move-to-heaven")]
    pub id: String,

    /// Activity trend over time.
    #[schema(example = json!({"Oct 26": 672, "Oct 27": 755}))]
    pub activity: BTreeMap<String, i32>,

    /// Rating trend over time.
    #[schema(example = json!({"Feb 11": 9.1, "Feb 12": 9.1}))]
    pub rating: BTreeMap<String, f32>,

    /// Breakdown of user watch statuses.
    pub watch_status: WatchStatusStats,

    /// Distribution of user scores as percentages.
    #[schema(example = json!({"10.0": 42.0, "9.5": 17.0}))]
    pub score: BTreeMap<String, f32>,

    /// Age group distribution.
    #[schema(example = json!({"13-17": {"percentage": 9.2, "rating": 9.2}}))]
    pub age: BTreeMap<String, AgeStat>,
}

/// Numerical counts of users in various stages of watching the title.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct WatchStatusStats {
    /// Total users currently watching.
    #[schema(example = 7170)]
    pub current: i32,
    /// Total users who have completed the title.
    #[schema(example = 187025)]
    pub completed: i32,
    /// Total users who have the title in their plan-to-watch list.
    #[schema(example = 44880)]
    pub plan_to_watch: i32,
    /// Total users who have put the title on hold.
    #[schema(example = 3377)]
    pub on_hold: i32,
    /// Total users who have dropped the title.
    #[schema(example = 2071)]
    pub dropped: i32,
}

/// Demographic statistics for a specific age group.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct AgeStat {
    /// The percentage of total watchers in this age group.
    #[schema(example = 9.2)]
    pub percentage: f32,
    /// The average rating given by users in this age group.
    #[schema(example = 9.2)]
    pub rating: f32,
}
