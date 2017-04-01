import QtQuick 2.2
import QtQuick.Controls 1.0

ListView {
    width: 180; height: 200

    Component {
            id: contactsDelegate
            Rectangle {
                id: wrapper
                width: 180
                height: contactInfo.height
                color: ListView.isCurrentItem ? "black" : "red"
                Text {
                    id: contactInfo
                    text: name + ": " + number
                    color: wrapper.ListView.isCurrentItem ? "red" : "black"
                }
            }
        }

    model: contactsModel
    delegate: contactsDelegate
    highlight: Rectangle { color: "lightsteelblue"; radius: 5 }
    focus: true
}
