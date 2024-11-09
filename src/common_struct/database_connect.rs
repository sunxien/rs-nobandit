use std::fmt::{Display, Formatter};
use std::io::Write;

use md5::{Digest, Md5};
use md5::digest::DynDigest;
use mysql::{Pool, PooledConn};

/// e.g:
/// let url = "mysql://root:password@localhost:3306/MYDB";
//  let pool = Pool::new(url).unwrap();
//  let mut conn = pool.get_conn().unwrap();
#[derive(Debug)]
#[allow(dead_code, unused, unused_imports)]
pub struct DatabaseConnect {
    user: String,
    password: String,
    hostname: String,
    port: u16,
    schema: String,
}

///
impl Display for DatabaseConnect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "mysql://{}:{}@{}:{}/{}", self.user,
               md5_str(self.password.as_str()),
               self.hostname,
               self.port,
               self.schema)
    }
}

///
impl DatabaseConnect {
    ///
    pub fn new(user: String, password: String, hostname: String, schema: String) -> Self {
        DatabaseConnect {
            user,
            password,
            hostname,
            port: 3306, // Default 3306
            schema,
        }
    }

    ///
    pub fn format_url(&self) -> String {
        format!("mysql://{user}:{password}@{hostname}:{port}/{schema}",
                user = self.user,
                password = self.password.as_str(),
                hostname = self.hostname,
                port = self.port,
                schema = self.schema
        )
    }

    ///
    pub fn new_connection_pool(&self) -> Pool {
        let url = self.format_url();
        Pool::new(url.as_str()).unwrap()
    }

    ///
    pub fn new_connection(&self, timeout: core::time::Duration) -> mysql::Result<PooledConn> {
        self.new_connection_pool().try_get_conn(timeout)
    }
}

///
pub fn md5_str(input: &str) -> String {
    let mut md5 = Md5::new();
    md5.write(input.as_bytes());
    format!("{:X}", md5.finalize())
}

///
#[allow(dead_code, unused, unused_imports)]
#[cfg(test)]
pub mod database_connect {
    use std::io::Write;
    use std::time::Duration;

    use md5::{Digest, Md5};
    use mysql::prelude::Queryable;

    use crate::common_struct::database_connect::{DatabaseConnect, md5_str};

    #[test]
    pub fn test_new_DatabaseConnect() {
        let database_connect = DatabaseConnect {
            user: String::from("root"),
            password: String::from("root"),
            hostname: String::from("localhost"),
            port: 3306,
            schema: String::from("nobandit"),
        };
        println!("{}", database_connect);
    }

    ///
    const TEST_SQL: &str =  r#"
        select 1 from dual
    "#;
    #[test]
    pub fn test_new_PooledConnection() {
        let database_connect = DatabaseConnect::new(
            String::from("root"),
            String::from("root"),
            String::from("localhost"),
            String::from("nobandit"),
        );
        let mut conn = database_connect.new_connection(Duration::from_millis(50)).unwrap();
        let r: Option<u8> = conn.query_first(TEST_SQL).expect("Query 'select 1 from dual failed'");
        match r {
            Some(v) => println!("Result: {}", v),
            None => println!("Result: None !!!")
        }
    }

    #[test]
    pub fn test_md5() {
        let password = "123456";
        let mut md5 = Md5::new();
        md5.write(password.as_bytes());
        println!("MD5 Digest: {:X}", md5.finalize());
        println!("MD5 Digest: {:?}", md5_str(password));
    }
}