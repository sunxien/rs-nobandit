use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct AppError {
    pub code: u32,
    pub message: String,
}

impl AppError {
    pub fn new(code: u32, message: String) -> Self {
        AppError { code, message }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error Code: {}\nError Message: {}",
            self.code, self.message
        )
    }
}

impl Error for AppError {}

#[cfg(test)]
pub mod app_error_tests {
    use crate::common_struct::app_error::AppError;

    #[test]
    pub fn test_new_app_error() {
        let app_error = AppError::new(400, String::from("Image not found"));
        println!("{}", app_error);
    }
}
