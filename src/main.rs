extern crate qml;

use qml::*;

fn main() {
    // Create a new QML Engine.
    let mut engine = QmlEngine::new();

    // Bind a message to the QML enviroment.
    let message = QVariant::from("Hello, world!");
    engine.set_property("message", &message);

    // Load some QML
    engine.load_data("
        import QtQuick 2.0
        import QtQuick.Controls 1.0

        ApplicationWindow {
            visible: true
            Text {
                anchors.fill: parent
                text: message
            }
        }
    ");
    engine.exec();
}