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

    // Load some QML
    engine.load_file("src/early-tea.qml");

    engine.exec();
}

