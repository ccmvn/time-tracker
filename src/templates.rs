use std::path::{Path, PathBuf};

use actix_web::web;
use actix_web::web::Data;
use log::warn;
use tera::{Error, Tera};
use walkdir::WalkDir;

fn get_base_path() -> PathBuf {
    PathBuf::from("./templates")
}

pub fn load_templates() -> Result<Tera, Error> {
    let mut tera = Tera::default();

    let base_path = get_base_path();
    let exclude_dirs = ["assets", "css", "node_modules", "js"];

    // Define the paths in the order you want them to be loaded
    let template_paths = [
        "app.html",
        "navbar.html",
        "modal/entry.html",
        "modal/edit_entry.html",
        "modal/absence.html",
        "modal/edit_absence.html",
    ];

    for relative_path_str in &template_paths {
        let path = base_path.join(relative_path_str);
        let template_name = Path::new(relative_path_str)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();

        tera.add_template_file(path, Some(template_name)).unwrap();
    }

    // Walk through directories and subdirectories
    for entry in WalkDir::new(&base_path) {
        let entry = entry.unwrap();
        if entry.path().is_dir() || exclude_dirs.iter().any(|dir| entry.path().to_string_lossy().contains(dir)) {
            continue;
        }
        if let Some(ext) = entry.path().extension() {
            if ext == "html" {
                let template_name = entry.path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap();

                tera.add_template_file(entry.path(), Some(&template_name)).unwrap();
            }
        }
    }

    Ok(tera)
}

pub fn tera_config(config: &mut web::ServiceConfig) {
    // Load the Tera templates
    let tera = match load_templates() {
        Ok(templates) => Data::new(templates),
        Err(err) => {
            warn!("Failed to load Tera templates: {}", err);
            return;
        }
    };

    config.app_data(tera.clone());
}
