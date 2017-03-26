#[macro_use]
extern crate qml;

use qml::*;

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

pub struct FooState;

impl FooState {
    pub fn simple_receiver(&mut self) -> Option<&QVariant> {
        println!("Slot called");

        // This is a function that also will be a slot
        None
    }
}

Q_OBJECT!(
pub FooState as QFooState{
    signals:
        fn simple_signal(s: String);
    slots:
        fn simple_receiver();
    properties:
        name: String; read: get_name, write: set_name, notify: name_changed;
});
