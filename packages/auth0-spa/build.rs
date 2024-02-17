fn main() {
    std::process::Command::new("bun")
        .args(&["install", "--frozen-lockfile"])
        .status()
        .expect("Failed to install the project");

    std::process::Command::new("bun")
        .args(&["run", "build"])
        .status()
        .expect("Failed to build the project");
}
