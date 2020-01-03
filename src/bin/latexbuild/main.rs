use std::fs::metadata;
use std::path::PathBuf;
use latexbuild::*;

fn main() {
    let entry = "test_project/index.tex";
    let pdf = "test_project/bin/index.pdf";

    let pdf_metadata = metadata(pdf);

    let build = match pdf_metadata {
        Ok(metadata) => {
            let modified = metadata.modified().unwrap();
            let items = vec![
                PathBuf::from("test_project/index.tex"),
                PathBuf::from("test_project/include.tex"),
            ];

            needs_rebuild(&modified, &items).unwrap()
        }
        Err(_) => true,
    };

    if build {
        println!("perform build on {}", entry);
    }
}
