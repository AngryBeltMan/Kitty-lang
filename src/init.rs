use std::process::Command;
pub fn init_build() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C","mkdir build && cd build && cargo new --bin build_file"])
            .output().expect("could not create build");
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("mkdir build && cd build && cargo new --bin build_file")
            .output().expect("could not creat build");
    }
}
