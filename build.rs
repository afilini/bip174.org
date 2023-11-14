use std::process::Command;

// Inspired by https://stackoverflow.com/questions/43753491/include-git-commit-hash-as-string-into-rust-program
fn main() {
    let git_status = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .zip(
            Command::new("git")
                .args(["diff-files", "--quiet"])
                .status()
                .ok()
                .map(|status| if status.success() { "" } else { "-dirty" }),
        )
        .map(|(hash, dirty)| format!("{}{}", &hash[..8], dirty))
        .unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=GIT_STATUS={}", git_status);
}
