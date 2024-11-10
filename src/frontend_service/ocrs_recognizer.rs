use std::env;
use std::io::Error;
use std::path::PathBuf;

use image::GenericImageView;
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
use walkdir::WalkDir;

/// cargo install ocrs-cli && ocrs xxx.jpg
/// Rust Doc: https://docs.rs/ocrs/latest/ocrs/struct.OcrEngineParams.html
/// Examples: https://github.com/robertknight/ocrs/blob/main/ocrs/examples/hello_ocr.rs
/// Download Models: https://github.com/robertknight/ocrs-models/blob/main/README.md
pub fn recognize_with_ocrs(filename: &str) -> String {
    let detection_model_path = file_path("models/text-detection.rten");
    // println!("{:?}", detection_model_path);

    let recognition_model_path = file_path("models/text-recognition.rten");
    // println!("{:?}", recognition_model_path);

    let detection_model = Model::load_file(detection_model_path).unwrap();
    let recognition_model = Model::load_file(recognition_model_path).unwrap();

    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    }).unwrap();

    let rgb = image::open(filename).unwrap().into_rgb8();
    let source = ImageSource::from_bytes(rgb.as_raw(), rgb.dimensions()).unwrap();
    let input = engine.prepare_input(source).unwrap();

    let word_rects = engine.detect_words(&input).unwrap();
    let line_rects = engine.find_text_lines(&input, &word_rects);
    let line_texts = engine.recognize_text(&input, &line_rects).unwrap();

    let mut license_plate: String = String::new();
    for line in line_texts.iter().flatten()
        .filter(|l| {
            let str = l.to_string();
            str.len() > 6 && str.len() < 10
        }) {
        license_plate.push_str(line.to_string().as_str());
    }
    license_plate
}

/// https://tesseract-ocr.github.io/tessdoc/Installation.html
/// https://github.com/tesseract-ocr/tesseract?tab=readme-ov-file#installing-tesseract
pub fn recoginize_with_tesseract(){}

/// Given a file path relative to the crate root, return the absolute path.
fn file_path(path: &str) -> PathBuf {
    let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    abs_path.push(path);
    abs_path
}

///
pub fn load_images() -> Result<Vec<PathBuf>, Error> {
    let result = env::current_dir()?;
    let mut images = Vec::new();
    for dir_entry in WalkDir::new(result) {
        let dir_entry = dir_entry?;
        let metadata = dir_entry.metadata()?;
        if metadata.len() > 0u64 && metadata.is_file() && !metadata.is_symlink() {
            match dir_entry.file_name().to_str() {
                Some(v) => {
                    if v.ends_with(".jpg") || v.ends_with(".jpeg") || v.ends_with(".png") {
                        images.push(dir_entry.path().to_path_buf());
                    }
                }
                None => {
                    // do nothing
                }
            }
        }
    }
    Result::Ok(images)
}

///
#[cfg(test)]
pub mod ocrs_recognizer_tests {
    use std::ops::Sub;
    use std::time::Instant;

    use crate::frontend_service::ocrs_recognizer::{load_images, recognize_with_ocrs};

    ///
    #[test]
    pub fn test_ocrs_recognizer() {
        let images = load_images().unwrap();
        for path in images {
            // println!("Recognizing {:?}", path);
            let begin = Instant::now();
            let text = recognize_with_ocrs(path.to_str().unwrap());
            let elapsed = Instant::now().sub(begin).as_millis();
            println!("Recognized {:?} ===> {:?}  Elapsed: {:?} ms", path, text, elapsed);
        }
    }
}
