use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

///
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code, unused)]
pub struct Config {
    // primary key
    pub id: u64,

    // garage name
    pub garage_name: String,

    /// garage type
    pub garage_type: u8,

    ///
    pub free_time: u8,

    ///
    pub parking_price: u8,

    ///
    pub max_stay_time: u8,

}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[cfg(test)]
pub mod config_tests {

    #[test]
    pub fn test_display_config(){

    }
}