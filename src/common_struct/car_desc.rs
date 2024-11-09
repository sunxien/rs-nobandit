use std::fmt::{Display, Formatter};

use crate::common_struct::car_license_plate::CarLicensePlate;

///
#[derive(Debug)]
#[allow(unused, dead_code)]
pub struct CarDesc {
    ///
    pub color: String,

    /// e.g: CAR, SUV, MPV and etc
    pub style: String,

    ///
    pub car_license_plate: Option<CarLicensePlate>,
}

///
impl Display for CarDesc {
    /// e.g: 黑色轿车，车牌：京A-76B33（蓝）
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let car_license_plate_str = match &self.car_license_plate {
            Some(v) => v.to_string(),
            None => "未识别".to_string(),
        };
        write!(f, "{}汽车，车牌：{}", self.color, car_license_plate_str)
    }
}


#[cfg(test)]
#[allow(dead_code, unused)]
pub mod car_desc_tests {
    use crate::common_struct::car_desc::CarDesc;
    use crate::common_struct::car_license_plate::CarLicensePlate;

    #[test]
    pub fn test_new_car_desc_with_unknown_license_plate() {
        let car_desc = CarDesc {
            color: "黑色".to_string(),
            style: "MPV".to_string(),
            car_license_plate: Option::None,
        };
        println!("{}", car_desc);
    }

    ///
    #[test]
    pub fn test_new_car_desc_with_license_plate() {
        let car_license_plate = CarLicensePlate {
            color: "黑色".to_string(),
            number: "京A-76B33".to_string(),
        };

        let car_desc = CarDesc {
            color: "黑色".to_string(),
            style: "MPV".to_string(),
            car_license_plate: Option::Some(car_license_plate),
        };
        println!("{}", car_desc);
    }
}