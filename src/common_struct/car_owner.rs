use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code, unused)]
pub struct CarOwner {
    pub id: u64,
    pub owner: String,
    pub phone: String,
    pub address: String,
    pub license_plate: String,
}

impl CarOwner {
    pub fn new(
        id: u64,
        owner: String,
        phone: String,
        address: String,
        license_plate: String,
    ) -> Self {
        CarOwner {
            id,
            owner,
            phone,
            address,
            license_plate,
        }
    }
}

impl Display for CarOwner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
