#[macro_use]
extern crate qml;
mod foostate;

use qml::*;
use foostate::FooState;
use foostate::QFooState;

fn main() {
    // Create a new QML Engine.
    let mut engine = QmlEngine::new();

    let foo_state = QFooState::new(FooState, "My name".into());
    foo_state.simple_signal("Hi from Rust!".into());
    engine.set_and_store_property("foo", foo_state.get_qobj());

    let mut qalm = QListModel::new(&["name", "number"]);
    qalm.append_row(qvarlist!["John", 1].into_iter());
    qalm.append_row(qvarlist!["Paul", 2].into_iter());
    qalm.append_row(qvarlist!["George", 3].into_iter());
    qalm.append_row(qvarlist!["Ringo", 4].into_iter());
    engine.set_property("contactsModel", &qalm.get_qvar());

    // Load some QML
    engine.load_file("src/early-tea.qml");

    engine.exec();
}

