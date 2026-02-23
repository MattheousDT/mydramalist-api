// Semi-zesty macro because utoipa macros do not play ball with generic structs.
macro_rules! define_pagination {
    ($name:ident, $item:ty, $total_example:expr) => {
        #[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone)]
        pub struct $name {
            pub items: Vec<$item>,

            /// The current page number.
            #[schema(example = 1)]
            pub page: i32,

            /// The total number of pages available.
            #[schema(example = $total_example)]
            pub total_pages: i32,
        }
    };
}

use crate::models::{Image, Recommendation, Review};

define_pagination!(PaginatedPhotos, Image, 3);
define_pagination!(PaginatedReviews, Review, 27);
define_pagination!(PaginatedRecommendations, Recommendation, 6);
