use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common_struct::app_error::AppError;
use crate::common_struct::config::Config;

///
const SELECT_CONFIG_SQL: &str = r#"
    select
        `id`,
        `garage_name`,
        `garage_type`,
        `free_time`,
        `parking_price`,
        `max_stay_time`
    from `config` where id=:id
"#;
pub fn load_config(conn: &mut PooledConn) -> Result<Config, AppError> {
    // TODO Refactor to use `PreparedStatement`
    let result = conn
        .exec_first(
            SELECT_CONFIG_SQL,
            params! {
                "id" => 1
            },
        )
        .map(|row| {
            row.map(
                |(id, garage_name, garage_type, free_time, parking_price, max_stay_time)| Config {
                    id,
                    garage_name,
                    garage_type,
                    free_time,
                    parking_price,
                    max_stay_time,
                },
            )
        });
    if result.is_err() {
        return Result::Err(AppError::new(500, result.unwrap_err().to_string()));
    }
    match result.unwrap() {
        Some(c) => Result::Ok(c),
        None => return Result::Err(AppError::new(400, String::from("Config not found"))),
    }
}

///
#[cfg(test)]
pub mod config_operation_tests {
    use std::time::Duration;

    use crate::common_struct::database_connect::DatabaseConnect;
    use crate::database_operation::config_operation::load_config;

    #[test]
    pub fn test_load_config() {
        let database_connect = DatabaseConnect::new(
            String::from("root"),
            String::from("root"),
            String::from("localhost"),
            String::from("nobandit"),
        );
        let mut conn = database_connect
            .new_connection(Duration::from_millis(50))
            .unwrap();
        let result = load_config(&mut conn);
        println!("Load Config: {}", result.unwrap());
    }
}
