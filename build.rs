use std::process::Command;

fn unwrap_nested_result<T, X, Y>(input: Result<Result<T, X>, Y>, default: T) -> T {
    match input {
        Ok(Ok(value)) => value,
        _ => default,
    }
}

fn main() {
    let git_hash = unwrap_nested_result(
        Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()
            .map(|output| String::from_utf8(output.stdout)),
        "UNKNOWN_GIT_COMMIT".to_string(),
    );
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    // TODO add latest tag to variables
}
