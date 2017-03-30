import QtQuick 2.3
import QtQuick.Controls 1.4
import QtQuick.Controls.Styles 1.3

Rectangle {
    id:root
    color: "green"

    height: 120;
    width: 175;

    signal submit(string text)
    
    function clear() {
        textArea.cursorPosition = 0;
        textArea.text="";
    }

    TextArea {
        id: textArea

        anchors.fill: parent
        anchors.margins: 1

        Constants {
            id: constants;
        }

        Keys.onPressed: {
            if ((event.key == Qt.Key_Return) && (event.modifiers & Qt.ControlModifier)) {
                console.log("Enter and control");
                root.submit(textArea.text);
                event.accepted = true;
            }
        }

    }
}