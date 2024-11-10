use mysql::{params, PooledConn, TxOpts};
use mysql::prelude::Queryable;

use crate::common_struct::app_error::AppError;
use crate::common_struct::history::History;

///
const SELECT_HISTORY_SQL: &str = r#"
    select
     `id`,
     `license_plate`,
     `enter_time`,
     `exit_time`,
     `status` from history where `license_plate`=:license_plate
"#;
pub fn load_history(conn: &mut PooledConn, license_plate: &str) -> Result<History, AppError> {
    // TODO Refactor to use `PreparedStatement`
    let result = conn.exec_first(SELECT_HISTORY_SQL,
                                 params! {
            "license_plate" => license_plate
        },
    ).map(|row| {
        row.map(|(id, license_plate, enter_time, exit_time, status)| History {
            id,
            license_plate,
            enter_time,
            exit_time,
            status,
        })
    });
    if result.is_err() {
        return Result::Err(AppError::new(500, result.unwrap_err().to_string()));
    }
    match result.unwrap() {
        Some(c) => Result::Ok(c),
        None => return Result::Err(AppError::new(400, String::from("History not found")))
    }
}


///
// const CHECK_SQL: &str = r#"
//
// ";
const ENTER_SQL: &str = r#"
    insert into `history` (`license_plate`) values (:license_plate);
"#;
pub fn enter(conn: &mut PooledConn, license_plate: &str) -> Result<u64, AppError> {
    // TODO Refactor to use `PreparedStatement`
    let mut trx = conn.start_transaction(TxOpts::default()).unwrap();
    let executed = trx.exec_drop(ENTER_SQL,
                                 params! {
        "license_plate" => license_plate,
    });
    if executed.is_err() {
        return Result::Err(AppError::new(500, executed.unwrap_err().to_string()));
    }
    let affect_rows = trx.affected_rows();
    let commited = trx.commit(); // Be care of `trx` moved here
    if commited.is_err() {
        return Result::Err(AppError::new(500, commited.unwrap_err().to_string()));
    } else {
        return Result::Ok(affect_rows);
    }
}


///
const STAY_TIME_SQL: &str = r#"
    select timestampdiff(SECOND,`enter_time`,`exit_time`) as stay_time from `history`
    where `license_plate`=:license_plate;
"#;
pub fn stay_time(conn: &mut PooledConn, license_plate: &str) -> Result<u32, AppError> {
    // TODO Refactor to use `PreparedStatement`
    let result = conn.exec_first(STAY_TIME_SQL,
                                 params! {
            "license_plate" => license_plate
        },
    );
    if result.is_err() {
        return Result::Err(AppError::new(500, result.unwrap_err().to_string()));
    }
    match result.unwrap() {
        Some(c) => Result::Ok(c),
        None => return Result::Err(AppError::new(400, String::from("Config not found")))
    }
}


///
const EXIT_SQL: &str = r#"
    update `history` set `exit_time`=current_timestamp,`status`=4
    where `license_plate`=:license_plate
"#;
pub fn exit(conn: &mut PooledConn, license_plate: &str) {
    // TODO Refactor to use `PreparedStatement`
    // 防止：已支付，未出场；下次出场，检查超时未出场，从上次的出场时间重新计费；
}


///
#[cfg(test)]
pub mod history_operation_tests {
    use std::time::Duration;

    use crate::common_struct::database_connect::DatabaseConnect;
    use crate::database_operation::history_operation::{enter, stay_time};

    ///
    #[test]
    pub fn test_enter() {
        let license_plate = String::from("京F-88531");

        let database_connect = DatabaseConnect::new(
            String::from("root"),
            String::from("root"),
            String::from("localhost"),
            String::from("nobandit"),
        );
        let mut conn = database_connect.new_connection(Duration::from_millis(50)).unwrap();
        let result = enter(&mut conn, &license_plate);
        println!("Car: {} is enter {} times", license_plate, result.unwrap());
    }

    ///
    #[test]
    pub fn test_stay_time() {
        let license_plate = String::from("京F-88531");

        let database_connect = DatabaseConnect::new(
            String::from("root"),
            String::from("root"),
            String::from("localhost"),
            String::from("nobandit"),
        );
        let mut conn = database_connect.new_connection(Duration::from_millis(50)).unwrap();
        let result = stay_time(&mut conn, &license_plate);
        println!("Car: {} has stayed {} seconds", license_plate, result.unwrap());
    }

    ///
    #[test]
    pub fn test_exit() {}
}
