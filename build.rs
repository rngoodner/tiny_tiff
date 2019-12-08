use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

// clone and install TinyTIFF
// will prompt for sudo to run make install
fn main() {
    // only put new files in out_dir
    let out_dir = env::var("OUT_DIR").expect("could not get OUT_DIR");
    env::set_current_dir(out_dir).expect("could not set OUT_DIR");

    // if on linux or macos
    if env::consts::OS == "linux" || env::consts::OS == "macos" {
        // check for git
        Command::new("git")
            .arg("--version")
            .status()
            .expect("git not found");
        // check for cmake
        Command::new("cmake")
            .arg("--version")
            .status()
            .expect("cmake not found");

        // if the path for TinyTIFF does not exist
        if !Path::new("./TinyTIFF").exists() {
            // clone git repo for TinyTIFF
            Command::new("git")
                .arg("clone")
                .arg("https://github.com/ryn1x/TinyTIFF.git")
                .status()
                .expect("could not clone git repo");

            // make build dir and switch to it
            let build_dir = "./TinyTIFF/build";
            fs::create_dir(build_dir).expect("could not create build dir");
            env::set_current_dir(build_dir).expect("could not set build dir");

            // build and install with cmake
            Command::new("cmake")
                .arg("..")
                .status()
                .expect("could not run cmake");
            Command::new("make").status().expect("could not run make");
            Command::new("sudo")
                .arg("make")
                .arg("install")
                .status()
                .expect("could not run make install");
        }
    }
}
