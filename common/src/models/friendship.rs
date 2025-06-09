use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FriendshipAnswer {
    pub user_id: i32,
    pub status: FriendshipStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, Eq, PartialEq, EnumIter, Display)]
pub enum FriendshipStatus {
    Pending,
    Accepted,
    Rejected,
    Blocked,
}
