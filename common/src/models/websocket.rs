use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FoodieMessageType {
    FriendRequest,
    RecipeDelete,
    RecipeCreate,
}

impl FromStr for FoodieMessageType {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
