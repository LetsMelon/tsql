use std::process::Command;
fn main() {
    let git_hash = match Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .map(|output| String::from_utf8(output.stdout))
    {
        Ok(Ok(hash)) => hash,
        _ => "UNKNOWN_GIT_HASH".to_string(),
    };

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}
