use mysql::prelude::Queryable;
use mysql::{params, PooledConn, TxOpts};

use crate::common_struct::app_error::AppError;
use crate::common_struct::car_owner::CarOwner;

///
const REGISTER_SQL: &str = r#"
    insert into `registered` (`owner`,`phone`,`address`,`license_plate`)
    values (:owner, :phone, :address, :license_plate);
"#;
pub fn register(conn: &mut PooledConn, car_owner: CarOwner) -> Result<u64, AppError> {
    // TODO Refactor to use `PreparedStatement`
    let mut trx = conn.start_transaction(TxOpts::default()).unwrap();
    let executed = trx.exec_drop(
        REGISTER_SQL,
        params! {
            "owner" => car_owner.owner,
            "phone" => car_owner.phone,
            "address" => car_owner.address,
            "license_plate" => car_owner.license_plate,
        },
    );
    if executed.is_err() {
        return Result::Err(AppError::new(500, executed.unwrap_err().to_string()));
    }
    let affect_rows = trx.affected_rows();
    let commited = trx.commit(); // `trx` moved here
    if commited.is_err() {
        return Result::Err(AppError::new(500, commited.unwrap_err().to_string()));
    } else {
        return Result::Ok(affect_rows);
    }
}

///
#[cfg(test)]
pub mod registered_operation_tests {
    use std::time::Duration;

    use crate::common_struct::car_owner::CarOwner;
    use crate::common_struct::database_connect::DatabaseConnect;
    use crate::database_operation::registered_operation::register;

    #[test]
    pub fn test_load_config() {
        let car_owner = CarOwner::new(
            2,
            String::from("周群"),
            String::from("010-45443720"),
            String::from("16幢-1单元-1101室内"),
            String::from("京F-88531"),
        );

        let database_connect = DatabaseConnect::new(
            String::from("root"),
            String::from("root"),
            String::from("localhost"),
            String::from("nobandit"),
        );
        let mut conn = database_connect
            .new_connection(Duration::from_millis(50))
            .unwrap();
        let result = register(&mut conn, car_owner);
        println!("Register Car Owner: {}", result.unwrap());
    }
}
