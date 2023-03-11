use std::process::Command;

// Ensures the client project is built to be served by this backend as server side rendered (ssr).
fn main() {
    println!("cargo:rerun-if-changed=../client/Cargo.toml");
    println!("cargo:rerun-if-changed=../client/index.html");
    println!("cargo:rerun-if-changed=../client/src");
    println!("cargo:rerun-if-changed=../client/locales");
    println!("cargo:rerun-if-changed=../client/assets");

    run_trunk();
}

fn run_trunk() {
    let client_public_path_prefix =
        option_env!("ONION2_BUILD_CLIENT_PUBLIC_PATH_PREFIX").unwrap_or("/");
    let mut args = vec![
        "build",
        "--public-url",
        client_public_path_prefix,
        "--features",
        "hydration",
    ];
    if Ok("release".to_owned()) == std::env::var("PROFILE") {
        args.push("--release");
    }
    let output = Command::new("trunk")
        .args(&args)
        .current_dir(std::fs::canonicalize("../client").unwrap())
        .status()
        .unwrap();
    assert!(output.success());
}
