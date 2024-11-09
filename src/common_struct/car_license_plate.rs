use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[allow(unused, dead_code)]
pub struct CarLicensePlate {
    /// `GREEN` used for New Energy cars. Like: EV
    /// `YELLOW` is used for heavy car. Like: truck
    /// `BLUE` is used for light car. Like: private car
    /// `WHITE` is used for government's car. Like: army
    /// `BLACK` is used for consulate's cars.
    pub color: String,

    /// e.g: 京A-76B33（蓝）
    pub number: String,
}

///
impl Display for CarLicensePlate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "车牌：{} ({})", self.number, self.color)
    }
}

#[cfg(test)]
#[allow(dead_code, unused)]
pub mod car_desc_tests {
    use crate::common_struct::car_license_plate::CarLicensePlate;

    #[test]
    pub fn test_new_car_license_plate() {
        let car_license_plate = CarLicensePlate {
            color: "黑色".to_string(),
            number: "京A-76B33".to_string(),
        };
        println!("{}", car_license_plate);
    }
}