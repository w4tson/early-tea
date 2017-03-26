use std::env;
use std::env::consts;


fn main() {

    // We use the QTDIR or QTDIR64 env variables to find the location of
    // Qt5. If these are not set, we use the default homebrew install
    // location.
    let qtdir_variable = match consts::ARCH {
        "x86_64" => "QTDIR64",
        _ => "QTDIR",
    };
    let qt5_location = env::var(qtdir_variable).unwrap_or(String::from("/usr/local/opt/qt5"));

    println!("cargo:rustc-link-search=framework={}/Frameworks", qt5_location);
    println!("cargo:rustc-link-lib=c++");
}
