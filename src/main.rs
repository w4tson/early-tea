extern crate qml;

use qml::*;
use std::fs;
use std::io;
use std::path::Path;

Q_LISTMODEL_ITEM!{
    pub QDirModel<FileItem> {
        file_name: String,
        is_dir: bool,
    }
}

fn main() {
    // Create a new QML Engine.
    let mut engine = QmlEngine::new();

    // Create a model with files.
    let dir_str = ".";
    let current_dir = fs::canonicalize(dir_str)
        .expect(&format!("Could not canonicalize {}.", dir_str));
    let mut dir_model = QDirModel::new();
    list_dir(&current_dir, &mut dir_model).expect("Could not read directory.");
    engine.set_and_store_property("dirModel", &dir_model);

    // Load some QML
    #[cfg(debug_assertions)]
    engine.load_file("src/early-tea.qml");
    #[cfg(not(debug_assertions))]
    engine.load_data(include_str!("early-tea.qml"));
    engine.exec();
}

fn list_dir(dir: &Path, model: &mut QDirModel) -> io::Result<()> {
    // get iterator over readable entries
    let entry_iter = fs::read_dir(dir)?.filter_map(|e| e.ok());

    model.clear();
    for entry in entry_iter {
        if let Ok(metadata) = entry.metadata() {
            if let Ok(file_name) = entry.file_name().into_string() {
                model.append_item(FileItem {
                    file_name: file_name,
                    is_dir: metadata.is_dir(),
                });
            }
        }
    }
    Ok(())
}