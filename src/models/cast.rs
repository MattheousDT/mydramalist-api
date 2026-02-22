use crate::models::{CastMember, CrewMember};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Full cast and crew credits for a title.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TitleCast {
    /// List of actors and their roles.
    pub cast: Vec<CastMember>,

    /// List of production crew members and their jobs.
    pub crew: Vec<CrewMember>,
}
