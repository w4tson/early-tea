pub struct FooState;

use qml::*;
use std::process::Command;


impl FooState {
    pub fn simple_receiver(&mut self) -> Option<&QVariant> {
        println!("Slot called");

        // This is a function that also will be a slot
        None
    }

    fn is_git_repo(&mut self, file_path: &String) -> bool {

        let normalized_file_path = file_path.replace("file://", "");

        let output = Command::new("git")
            .current_dir(normalized_file_path)
            .arg("rev-parse")
            .status()
            .unwrap_or_else(|e| panic!("failed validating git repo: {}", e));

        output.success()
    }

    pub fn open_repo(&mut self, repo: String) -> Option<&QVariant> {
        let msg = match self.is_git_repo(&repo) {
            true => format!("File [{}] is a git repo", repo),
            _    => format!("File [{}] is not a git repo", repo)
        };
        println!("{}", msg);

        None
    }
}

Q_OBJECT!(
pub FooState as QFooState{
    signals:
        fn simple_signal(s: String);
    slots:
        fn simple_receiver();
        fn open_repo(repo: String);
    properties:
        name: String; read: get_name, write: set_name, notify: name_changed;
});
