use std::num::NonZeroU32;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resolution {
    pub x: NonZeroU32,
    pub y: NonZeroU32,
}
