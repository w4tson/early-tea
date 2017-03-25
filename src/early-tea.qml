import QtQuick 2.0
import QtQuick.Controls 1.0

ApplicationWindow {
    visible: true

    ListView {
        anchors.fill: parent
        model: dirModel
        delegate: Text {
            text: file_name
            font.italic: is_dir
        }
    }
}