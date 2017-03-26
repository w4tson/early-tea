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

    // Load some QML
    engine.load_file("src/early-tea.qml");

    engine.exec();
}
