///
pub fn recognize() {
    println!("")
}

///
#[cfg(test)]
pub mod ocrs_recognizer_tests{
    use std::env;
    use ocrs::*;

    ///
    #[test]
    pub fn test_ocrs_recognizer(){
        let path = env::current_dir()?;
        println!("Current dir: {}", path);
    }
}